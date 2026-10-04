// Type System Module

pub mod inference;
pub mod gradual;
pub mod dependent;

pub use inference::*;
pub use gradual::*;
pub use dependent::*;

// ============================================================================
// Type System Core
// ============================================================================

use crate::tumc_compiler::ir::*;

/// Type schema with type parameters
#[derive(Debug, Clone)]
pub struct TypeSchema {
    pub params: Vec<TypeVar>,
    pub body: MonoType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeVar(pub usize);

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum IntSize {
    I8, I16, I32, I64, I128, Isize,
    U8, U16, U32, U64, U128, Usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum FloatSize {
    F16, F32, F64,
}

#[derive(Debug, Clone)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, MonoType)>,
}

#[derive(Debug, Clone)]
pub struct EnumType {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub data: Vec<MonoType>,
}

#[derive(Debug, Clone)]
pub struct TraitType {
    pub name: String,
    pub methods: Vec<String>,
    pub super_traits: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub params: Vec<MonoType>,
    pub ret: Box<MonoType>,
    pub safety: Safety,
    pub abi: Option<String>,
}

// ============================================================================
// Type Unification
// ============================================================================

/// Type unification result
#[derive(Debug, Clone)]
pub enum UnificationResult {
    Success,
    Failure(TypeMismatch),
}

/// Type mismatch error
#[derive(Debug, Clone)]
pub struct TypeMismatch {
    pub expected: MonoType,
    pub found: MonoType,
    pub context: String,
}

/// Unification context
pub struct UnificationContext {
    pub substitutions: HashMap<TypeVar, MonoType>,
    pub constraints: Vec<TypeConstraint>,
}

impl UnificationContext {
    pub fn new() -> Self {
        Self {
            substitutions: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    pub fn unify(&mut self, a: &MonoType, b: &MonoType) -> UnificationResult {
        match (a, b) {
            (MonoType::Wildcard, _) => UnificationResult::Success,
            (_, MonoType::Wildcard) => UnificationResult::Success,
            (MonoType::Unit, MonoType::Unit) => UnificationResult::Success,
            (MonoType::Bool, MonoType::Bool) => UnificationResult::Success,
            (MonoType::Int(a_size), MonoType::Int(b_size)) if a_size == b_size => UnificationResult::Success,
            (MonoType::Float(a_size), MonoType::Float(b_size)) if a_size == b_size => UnificationResult::Success,
            (MonoType::Char, MonoType::Char) => UnificationResult::Success,
            (MonoType::String, MonoType::String) => UnificationResult::Success,
            (MonoType::Never, MonoType::Never) => UnificationResult::Success,
            (MonoType::Array(a_elem), MonoType::Array(b_elem)) => {
                self.unify(a_elem, b_elem)
            }
            (MonoType::Slice(a_elem), MonoType::Slice(b_elem)) => {
                self.unify(a_elem, b_elem)
            }
            (MonoType::Tuple(a_fields), MonoType::Tuple(b_fields)) if a_fields.len() == b_fields.len() => {
                for (a, b) in a_fields.iter().zip(b_fields.iter()) {
                    if let UnificationResult::Failure(e) = self.unify(a, b) {
                        return UnificationResult::Failure(e);
                    }
                }
                UnificationResult::Success
            }
            (MonoType::Function(a_func), MonoType::Function(b_func)) => {
                if a_func.params.len() != b_func.params.len() {
                    return UnificationResult::Failure(TypeMismatch {
                        expected: a.clone(),
                        found: b.clone(),
                        context: "function parameter count mismatch".to_string(),
                    });
                }
                for (a, b) in a_func.params.iter().zip(b_func.params.iter()) {
                    if let UnificationResult::Failure(e) = self.unify(a, b) {
                        return UnificationResult::Failure(e);
                    }
                }
                self.unify(&a_func.ret, &b_func.ret)
            }
            _ => UnificationResult::Failure(TypeMismatch {
                expected: a.clone(),
                found: b.clone(),
                context: "type mismatch".to_string(),
            }),
        }
    }

    pub fn substitute(&self, var: &TypeVar) -> Option<&MonoType> {
        self.substitutions.get(var).map(|t| t)
    }

    pub fn apply_substitution(&self, ty: &MonoType) -> MonoType {
        ty.clone() // Simplified
    }
}

impl Default for UnificationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Type constraint
#[derive(Debug, Clone)]
pub enum TypeConstraint {
    Implements { type_: MonoType, trait_: String },
    Subtype { sub: MonoType, sup: MonoType },
    Equal { left: MonoType, right: MonoType },
}

// ============================================================================
// Trait Resolution
// ============================================================================

/// Trait resolution context
pub struct TraitContext {
    pub impls: HashMap<String, Vec<TraitImpl>>,
    pub where_clauses: Vec<WhereClause>,
}

impl TraitContext {
    pub fn new() -> Self {
        Self {
            impls: HashMap::new(),
            where_clauses: Vec::new(),
        }
    }

    pub fn add_impl(&mut self, impl_: TraitImpl) {
        self.impls.entry(impl_.trait_name.clone()).or_default().push(impl_);
    }

    pub fn resolve_trait(&self, type_: &MonoType, trait_name: &str) -> Option<&TraitImpl> {
        self.impls.get(trait_name)?.iter().find(|i| self.types_equal(&i.self_type, type_))
    }

    fn types_equal(&self, a: &MonoType, b: &MonoType) -> bool {
        a == b
    }
}

impl Default for TraitContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct TraitImpl {
    pub trait_name: String,
    pub self_type: MonoType,
    pub methods: HashMap<String, MonoType>,
}

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub predicates: Vec<WherePredicate>,
}

#[derive(Debug, Clone)]
pub enum WherePredicate {
    TraitBound { ty: MonoType, trait_name: String },
    Equality { left: MonoType, right: MonoType },
    LifetimeBound { lifetime: String, bound: LifetimeBound },
}

#[derive(Debug, Clone)]
pub enum LifetimeBound {
    Lifetime(String),
    Trait { name: String },
}

use std::collections::HashMap;
