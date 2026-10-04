// Type Inference - Hindley-Milner

use super::*;

/// Type inference engine
pub struct TypeInference {
    pub context: TypeContext,
    pub unification: UnificationContext,
    pub constraints: Vec<Constraint>,
    pub next_var: usize,
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Equal(MonoType, MonoType),
    Subtype(MonoType, MonoType),
    Implements(MonoType, String),
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            context: TypeContext::new(),
            unification: UnificationContext::new(),
            constraints: Vec::new(),
            next_var: 0,
        }
    }

    /// Generate a fresh type variable
    pub fn fresh_var(&mut self) -> MonoType {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        MonoType::Var(var)
    }

    /// Infer type of an expression
    pub fn infer_expr(&mut self, expr: &Expr) -> Result<MonoType, TypeError> {
        match &expr.kind {
            ExprKind::Unit => Ok(MonoType::Unit),
            ExprKind::Bool(_) => Ok(MonoType::Bool),
            ExprKind::Char(_) => Ok(MonoType::Char),
            ExprKind::Int(_) => Ok(MonoType::Int(IntSize::I64)),
            ExprKind::Float(_) => Ok(MonoType::Float(FloatSize::F64)),
            ExprKind::String(_) => Ok(MonoType::String),
            ExprKind::Ident(name) => {
                self.context.get(name).cloned()
                    .ok_or_else(|| TypeError::UndefinedVariable(name.clone()))
            }
            ExprKind::Binary(op, lhs, rhs) => {
                let lhs_ty = self.infer_expr(lhs)?;
                let rhs_ty = self.infer_expr(rhs)?;
                let result_ty = self.fresh_var();

                self.constraints.push(Constraint::Equal(lhs_ty, rhs_ty));
                self.constraints.push(Constraint::Equal(result_ty.clone(), lhs_ty));

                Ok(result_ty)
            }
            ExprKind::Unary(_, operand) => {
                self.infer_expr(operand)
            }
            ExprKind::Call(callee, args) => {
                let callee_ty = self.infer_expr(callee)?;

                let param_tys: Vec<_> = args.iter()
                    .map(|a| self.infer_expr(a))
                    .collect::<Result<Vec<_>, _>>()?;

                let ret_ty = self.fresh_var();
                self.constraints.push(Constraint::Equal(
                    callee_ty,
                    MonoType::Function(Box::new(FunctionType {
                        params: param_tys,
                        ret: Box::new(ret_ty.clone()),
                        safety: Safety::Safe,
                        abi: None,
                    })),
                ));

                Ok(ret_ty)
            }
            ExprKind::If(cond, then, else_) => {
                let cond_ty = self.infer_expr(cond)?;
                self.constraints.push(Constraint::Equal(cond_ty, MonoType::Bool));

                let then_ty = self.infer_expr(then)?;
                let else_ty = self.infer_expr(else_)?;

                let result_ty = self.fresh_var();
                self.constraints.push(Constraint::Equal(then_ty, result_ty.clone()));
                self.constraints.push(Constraint::Equal(else_ty, result_ty.clone()));

                Ok(result_ty)
            }
            ExprKind::Lambda(params, body) => {
                let param_tys: Vec<_> = params.iter()
                    .map(|_| self.fresh_var())
                    .collect();

                for (param, ty) in params.iter().zip(param_tys.iter()) {
                    self.context.insert(param.clone(), ty.clone());
                }

                let body_ty = self.infer_expr(body)?;

                Ok(MonoType::Function(Box::new(FunctionType {
                    params: param_tys,
                    ret: Box::new(body_ty),
                    safety: Safety::Safe,
                    abi: None,
                })))
            }
            ExprKind::Let(name, ty, value, body) => {
                let value_ty = self.infer_expr(value)?;

                if let Some(expected_ty) = ty {
                    self.constraints.push(Constraint::Equal(value_ty, expected_ty.clone()));
                }

                self.context.insert(name.clone(), value_ty);

                self.infer_expr(body)
            }
            _ => Ok(MonoType::Wildcard),
        }
    }

    /// Solve constraints and produce substitution
    pub fn solve(&mut self) -> Result<Substitution, TypeError> {
        let mut substitution = Substitution::new();

        for constraint in &self.constraints {
            match constraint {
                Constraint::Equal(a, b) => {
                    match self.unification.unify(a, b) {
                        UnificationResult::Success => {}
                        UnificationResult::Failure(e) => {
                            return Err(TypeError::UnificationFailed(e));
                        }
                    }
                }
                Constraint::Subtype(a, b) => {
                    // Subtype constraint
                }
                Constraint::Implements(ty, trait_name) => {
                    // Trait constraint
                }
            }
        }

        Ok(substitution)
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}

/// Type context (symbol table for types)
#[derive(Debug, Clone, Default)]
pub struct TypeContext {
    pub bindings: HashMap<String, MonoType>,
    pub lifetimes: HashMap<String, Lifetime>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, ty: MonoType) {
        self.bindings.insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<&MonoType> {
        self.bindings.get(name)
    }

    pub fn extend(&self) -> Self {
        Self {
            bindings: self.bindings.clone(),
            lifetimes: self.lifetimes.clone(),
        }
    }
}

/// Lifetime representation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime(pub usize);

/// Substitution map
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    pub map: HashMap<TypeVar, MonoType>,
}

impl Substitution {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&self, ty: &MonoType) -> MonoType {
        match ty {
            MonoType::Var(var) => {
                self.map.get(var).cloned().unwrap_or_else(|| ty.clone())
            }
            MonoType::Array(elem) => {
                MonoType::Array(Box::new(self.apply(elem)))
            }
            MonoType::Slice(elem) => {
                MonoType::Slice(Box::new(self.apply(elem)))
            }
            MonoType::Tuple(elems) => {
                MonoType::Tuple(elems.iter().map(|e| self.apply(e)).collect())
            }
            MonoType::Function(func) => {
                MonoType::Function(Box::new(FunctionType {
                    params: func.params.iter().map(|p| self.apply(p)).collect(),
                    ret: Box::new(self.apply(&func.ret)),
                    safety: func.safety,
                    abi: func.abi.clone(),
                }))
            }
            _ => ty.clone(),
        }
    }

    pub fn compose(&mut self, other: &Substitution) {
        for (var, ty) in &other.map {
            let applied = self.apply(ty);
            self.map.insert(*var, applied);
        }
    }
}

/// MonoType with variables
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MonoType {
    Unit,
    Bool,
    Int(IntSize),
    Float(FloatSize),
    Char,
    String,
    Array(Box<MonoType>),
    Slice(Box<MonoType>),
    Tuple(Vec<MonoType>),
    Function(Box<FunctionType>),
    Struct(StructType),
    Enum(EnumType),
    Trait(TraitType),
    Pointer(Box<MonoType>),
    Reference(Box<MonoType>),
    Never,
    Wildcard,
    Var(TypeVar),
}

impl MonoType {
    pub fn from_ir_type(ir: &IrType) -> Self {
        match ir {
            IrType::Void => MonoType::Unit,
            IrType::Bool => MonoType::Bool,
            IrType::Int8 | IrType::Int16 | IrType::Int32 | IrType::Int64 => MonoType::Int(IntSize::I64),
            IrType::Float32 => MonoType::Float(FloatSize::F32),
            IrType::Float64 => MonoType::Float(FloatSize::F64),
            IrType::Char => MonoType::Char,
            IrType::Pointer(inner) => {
                MonoType::Pointer(Box::new(MonoType::from_ir_type(inner)))
            }
            IrType::Array(elem, _) => {
                MonoType::Array(Box::new(MonoType::from_ir_type(elem)))
            }
            _ => MonoType::Wildcard,
        }
    }

    pub fn to_ir_type(&self) -> IrType {
        match self {
            MonoType::Unit => IrType::Void,
            MonoType::Bool => IrType::Bool,
            MonoType::Int(_) => IrType::Int64,
            MonoType::Float(FloatSize::F32) => IrType::Float32,
            MonoType::Float(FloatSize::F64) => IrType::Float64,
            MonoType::Char => IrType::Char,
            MonoType::String => IrType::Pointer(Box::new(IrType::Int8)),
            MonoType::Pointer(inner) => IrType::Pointer(Box::new(inner.to_ir_type())),
            MonoType::Never => IrType::Never,
            _ => IrType::Void,
        }
    }
}

/// Type error
#[derive(Debug, Clone)]
pub enum TypeError {
    UnificationFailed(TypeMismatch),
    UndefinedVariable(String),
    InfiniteType(TypeVar),
    CannotInfer,
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnificationFailed(m) => {
                write!(f, "Type mismatch: expected {:?}, found {:?}", m.expected, m.found)
            }
            TypeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            TypeError::InfiniteType(var) => {
                write!(f, "Infinite type: {:?}", var)
            }
            TypeError::CannotInfer => {
                write!(f, "Cannot infer type")
            }
        }
    }
}

use std::collections::HashMap;
