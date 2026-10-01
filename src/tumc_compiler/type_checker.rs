//! Type checking and inference for TUMC
//! Implements Hindley-Milner type inference with constraint solving

use crate::prelude::*;
use super::ast::{self, *};
use super::parser::SourceLocation;
use std::collections::{HashMap, HashSet};

/// Represents a type variable for inference
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeVar(pub usize);

/// A type in the TUMC type system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Primitive types
    Unit,
    Bool,
    Int,
    Float,
    Char,
    String,

    /// Named types (user-defined)
    Named(String),

    /// Generic type parameter
    TypeVar(TypeVar),

    /// Function types: fn(Arg1, Arg2, ...) -> ReturnType
    Function(Vec<Type>, Box<Type>),

    /// Tuple types
    Tuple(Vec<Type>),

    /// Array/Slice types
    Array(Box<Type>),

    /// Dictionary/Map types
    Dict(Box<Type>, Box<Type>),

    /// Optional types
    Optional(Box<Type>),

    /// Result types for error handling
    Result(Box<Type>, Box<Type>),

    /// Struct types with named fields
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },

    /// Enum types with variants
    Enum {
        name: String,
        variants: Vec<(String, Option<Type>)>,
    },

    /// Trait object types
    TraitObject {
        traits: Vec<String>,
    },

    /// Reference types
    Ref(Box<Type>),
    MutRef(Box<Type>),

    /// Pointer types
    Ptr(Box<Type>),

    /// Unknown/Error type placeholder
    Error,
}

impl Type {
    /// Check if type is primitive
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Unit | Type::Bool | Type::Int | Type::Float | Type::Char | Type::String
        )
    }

    /// Check if type is copyable (Clone + Copy)
    pub fn is_copyable(&self) -> bool {
        self.is_primitive()
    }

    /// Check if type is sendable across threads
    pub fn is_sendable(&self) -> bool {
        match self {
            Type::Function(_, _) => false, // Closures may capture non-sendable state
            _ => true,
        }
    }

    /// Unify two types, returning constraints
    pub fn unify(&self, other: &Type) -> Result<Vec<TypeConstraint>, TypeError> {
        if self == other {
            return Ok(vec![]);
        }

        match (self, other) {
            // Type variable can unify with anything
            (Type::TypeVar(_), _) | (_, Type::TypeVar(_)) => {
                Ok(vec![TypeConstraint::Equal(self.clone(), other.clone())])
            }

            // Function type unification
            (Type::Function(args1, ret1), Type::Function(args2, ret2)) => {
                if args1.len() != args2.len() {
                    return Err(TypeError::ArgCountMismatch {
                        expected: args1.len(),
                        found: args2.len(),
                    });
                }

                let mut constraints = Vec::new();
                for (a1, a2) in args1.iter().zip(args2.iter()) {
                    constraints.extend(a1.unify(a2)?);
                }
                constraints.extend(ret1.unify(ret2)?);
                Ok(constraints)
            }

            // Tuple unification
            (Type::Tuple(elems1), Type::Tuple(elems2)) => {
                if elems1.len() != elems2.len() {
                    return Err(TypeError::TupleLengthMismatch {
                        expected: elems1.len(),
                        found: elems2.len(),
                    });
                }

                let mut constraints = Vec::new();
                for (e1, e2) in elems1.iter().zip(elems2.iter()) {
                    constraints.extend(e1.unify(e2)?);
                }
                Ok(constraints)
            }

            // Optional unification
            (Type::Optional(inner1), Type::Optional(inner2)) => {
                inner1.unify(inner2)
            }

            // Result unification
            (Type::Result(ok1, err1), Type::Result(ok2, err2)) => {
                let mut constraints = ok1.unify(ok2)?;
                constraints.extend(err1.unify(err2)?);
                Ok(constraints)
            }

            // Reference unification
            (Type::Ref(inner1), Type::Ref(inner2)) => inner1.unify(inner2),
            (Type::MutRef(inner1), Type::MutRef(inner2)) => inner1.unify(inner2),

            // Struct unification by name
            (Type::Struct { name: n1, .. }, Type::Struct { name: n2, .. }) if n1 == n2 => {
                Ok(vec![])
            }

            // Enum unification by name
            (Type::Enum { name: n1, .. }, Type::Enum { name: n2, .. }) if n1 == n2 => {
                Ok(vec![])
            }

            // Named types
            (Type::Named(n1), Type::Named(n2)) if n1 == n2 => Ok(vec![]),

            // Error type unifies with anything
            (Type::Error, _) | (_, Type::Error) => Ok(vec![]),

            // Mismatched types
            _ => Err(TypeError::TypeMismatch {
                expected: self.clone(),
                found: other.clone(),
            }),
        }
    }

    /// Get display name for type
    pub fn display_name(&self) -> String {
        match self {
            Type::Unit => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "string".to_string(),
            Type::Named(name) => name.clone(),
            Type::TypeVar(tv) => format!("T{}", tv.0),
            Type::Function(args, ret) => {
                let args_str = args.iter().map(|a| a.display_name()).collect::<Vec<_>>().join(", ");
                format!("fn({}) -> {}", args_str, ret.display_name())
            }
            Type::Tuple(elems) => {
                let elems_str = elems.iter().map(|e| e.display_name()).collect::<Vec<_>>().join(", ");
                format!("({})", elems_str)
            }
            Type::Array(inner) => format!("[{}]", inner.display_name()),
            Type::Dict(key, val) => format!("[{}: {}]", key.display_name(), val.display_name()),
            Type::Optional(inner) => format!("{}?", inner.display_name()),
            Type::Result(ok, err) => format!("Result<{}, {}>", ok.display_name(), err.display_name()),
            Type::Struct { name, .. } => name.clone(),
            Type::Enum { name, .. } => name.clone(),
            Type::TraitObject { traits } => traits.join(" + "),
            Type::Ref(inner) => format!("&{}", inner.display_name()),
            Type::MutRef(inner) => format!("&mut {}", inner.display_name()),
            Type::Ptr(inner) => format!("*{}", inner.display_name()),
            Type::Error => "Error".to_string(),
        }
    }
}

/// Type constraint for constraint solving
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeConstraint {
    Equal(Type, Type),
    Implements { type_: Type, trait_name: String },
    Subtype { sub: Type, sup: Type },
}

impl TypeConstraint {
    /// Solve a constraint, updating the type environment
    pub fn solve(&self, env: &mut TypeEnv) -> Result<(), TypeError> {
        match self {
            TypeConstraint::Equal(t1, t2) => {
                env.unify(t1, t2)
            }
            TypeConstraint::Implements { type_, trait_name } => {
                env.add_trait_bound(type_, trait_name)
            }
            TypeConstraint::Subtype { sub, sup } => {
                // Check if sub is subtype of sup
                if env.is_subtype(sub, sup) {
                    Ok(())
                } else {
                    Err(TypeError::SubtypeViolation {
                        sub: sub.clone(),
                        sup: sup.clone(),
                    })
                }
            }
        }
    }
}

/// Type environment holding type variables and constraints
pub struct TypeEnv {
    /// Next type variable ID
    next_var: usize,

    /// Type variable substitutions
    substitutions: HashMap<TypeVar, Type>,

    /// Named types (structs, enums, aliases)
    types: HashMap<String, Type>,

    /// Trait implementations
    trait_impls: HashMap<String, Vec<Type>>,

    /// Local bindings (function parameters, local variables)
    bindings: HashMap<String, Type>,

    /// Trait bounds on type variables
    bounds: HashMap<TypeVar, HashSet<String>>,
}

impl TypeEnv {
    /// Create new empty type environment
    pub fn new() -> Self {
        TypeEnv {
            next_var: 0,
            substitutions: HashMap::new(),
            types: Self::builtin_types(),
            trait_impls: HashMap::new(),
            bindings: HashMap::new(),
            bounds: HashMap::new(),
        }
    }

    /// Create builtin types
    fn builtin_types() -> HashMap<String, Type> {
        let mut types = HashMap::new();
        types.insert("i8".to_string(), Type::Int);
        types.insert("i16".to_string(), Type::Int);
        types.insert("i32".to_string(), Type::Int);
        types.insert("i64".to_string(), Type::Int);
        types.insert("u8".to_string(), Type::Int);
        types.insert("u16".to_string(), Type::Int);
        types.insert("u32".to_string(), Type::Int);
        types.insert("u64".to_string(), Type::Int);
        types.insert("f32".to_string(), Type::Float);
        types.insert("f64".to_string(), Type::Float);
        types.insert("bool".to_string(), Type::Bool);
        types.insert("char".to_string(), Type::Char);
        types.insert("str".to_string(), Type::String);
        types.insert("String".to_string(), Type::String);
        types
    }

    /// Generate fresh type variable
    pub fn fresh_var(&mut self) -> Type {
        let var = TypeVar(self.next_var);
        self.next_var += 1;
        Type::TypeVar(var)
    }

    /// Add a named type to the environment
    pub fn add_type(&mut self, name: String, type_: Type) {
        self.types.insert(name, type_);
    }

    /// Look up a named type
    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    /// Add a binding (variable to type)
    pub fn add_binding(&mut self, name: String, type_: Type) {
        self.bindings.insert(name, type_);
    }

    /// Look up a binding
    pub fn get_binding(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name)
    }

    /// Remove a binding
    pub fn remove_binding(&mut self, name: &str) {
        self.bindings.remove(name);
    }

    /// Unify two types
    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<(), TypeError> {
        // Apply substitutions first
        let t1 = self.apply(t1);
        let t2 = self.apply(t2);

        // Handle type variable cases
        match (&t1, &t2) {
            (Type::TypeVar(v1), Type::TypeVar(v2)) if v1 == v2 => Ok(()),
            (Type::TypeVar(v), t) | (t, Type::TypeVar(v)) => {
                if self.occurs_in(v, t) {
                    return Err(TypeError::InfiniteType {
                        var: v.clone(),
                        type_: t.clone(),
                    });
                }
                self.substitutions.insert(v.clone(), t.clone());
                Ok(())
            }
            _ => {
                // Use Type's unify method
                let constraints = t1.unify(&t2)?;
                for constraint in constraints {
                    constraint.solve(self)?;
                }
                Ok(())
            }
        }
    }

    /// Check if type variable occurs in type (for occurs check)
    fn occurs_in(&self, var: &TypeVar, type_: &Type) -> bool {
        match type_ {
            Type::TypeVar(v) => v == var,
            Type::Function(args, ret) => {
                args.iter().any(|a| self.occurs_in(var, a)) || self.occurs_in(var, ret)
            }
            Type::Tuple(elems) => elems.iter().any(|e| self.occurs_in(var, e)),
            Type::Array(inner) => self.occurs_in(var, inner),
            Type::Dict(k, v) => self.occurs_in(var, k) || self.occurs_in(var, v),
            Type::Optional(inner) => self.occurs_in(var, inner),
            Type::Result(ok, err) => self.occurs_in(var, ok) || self.occurs_in(var, err),
            Type::Struct { fields, .. } => fields.iter().any(|(_, t)| self.occurs_in(var, t)),
            Type::Enum { variants, .. } => {
                variants.iter().any(|(_, t)| t.as_ref().map_or(false, |t| self.occurs_in(var, t)))
            }
            Type::TraitObject { traits: _ } => false,
            Type::Ref(inner) | Type::MutRef(inner) | Type::Ptr(inner) => {
                self.occurs_in(var, inner)
            }
            _ => false,
        }
    }

    /// Apply substitutions to a type
    pub fn apply(&self, type_: &Type) -> Type {
        match type_ {
            Type::TypeVar(v) => {
                self.substitutions.get(v).cloned().map(|t| self.apply(&t)).unwrap_or_else(|| type_.clone())
            }
            Type::Function(args, ret) => {
                Type::Function(
                    args.iter().map(|a| self.apply(a)).collect(),
                    Box::new(self.apply(ret)),
                )
            }
            Type::Tuple(elems) => {
                Type::Tuple(elems.iter().map(|e| self.apply(e)).collect())
            }
            Type::Array(inner) => Type::Array(Box::new(self.apply(inner))),
            Type::Dict(k, v) => Type::Dict(Box::new(self.apply(k)), Box::new(self.apply(v))),
            Type::Optional(inner) => Type::Optional(Box::new(self.apply(inner))),
            Type::Result(ok, err) => {
                Type::Result(Box::new(self.apply(ok)), Box::new(self.apply(err)))
            }
            Type::Ref(inner) => Type::Ref(Box::new(self.apply(inner))),
            Type::MutRef(inner) => Type::MutRef(Box::new(self.apply(inner))),
            Type::Ptr(inner) => Type::Ptr(Box::new(self.apply(inner))),
            _ => type_.clone(),
        }
    }

    /// Add trait bound to type variable
    pub fn add_trait_bound(&mut self, type_: &Type, trait_name: &str) -> Result<(), TypeError> {
        match type_ {
            Type::TypeVar(v) => {
                self.bounds.entry(v.clone()).or_default().insert(trait_name.to_string());
                Ok(())
            }
            _ => {
                // Check if type actually implements trait
                let impls = self.trait_impls.entry(trait_name.to_string()).or_default();
                impls.push(type_.clone());
                Ok(())
            }
        }
    }

    /// Check if t1 is subtype of t2
    pub fn is_subtype(&self, sub: &Type, sup: &Type) -> bool {
        let sub = self.apply(sub);
        let sup = self.apply(sup);

        // Same types are subtypes
        if sub == sup {
            return true;
        }

        // Type variable is subtype if it has the right bounds
        if let Type::TypeVar(v) = &sup {
            if let Some(bounds) = self.bounds.get(v) {
                return bounds.contains("Subtype");
            }
        }

        match (&sub, &sup) {
            // Primitive subtypes
            (Type::Int, Type::Int) => true,
            (Type::Float, Type::Float) => true,
            (Type::Float, Type::Int) => true, // Float widens to int (lossy but allowed)

            // Struct subtyping (width subtyping)
            (Type::Struct { name: n1, fields: f1 }, Type::Struct { name: n2, fields: f2 }) if n1 == n2 => {
                f2.iter().all(|(name, ty2)| {
                    f1.iter().any(|(name1, ty1)| name1 == name && self.is_subtype(ty1, ty2))
                })
            }

            _ => false,
        }
    }
}

/// Type error types
#[derive(Debug, Clone)]
pub enum TypeError {
    UndefinedType { name: String },
    UndefinedVariable { name: String },
    TypeMismatch { expected: Type, found: Type },
    ArgCountMismatch { expected: usize, found: usize },
    TupleLengthMismatch { expected: usize, found: usize },
    InfiniteType { var: TypeVar, type_: Type },
    SubtypeViolation { sub: Type, sup: Type },
    TraitNotImplemented { type_: Type, trait_name: String },
    RecursiveType { name: String },
    InvalidFieldAccess { type_: Type, field: String },
    InvalidMethodCall { type_: Type, method: String },
    CyclicDependency { types: Vec<String> },
    CannotInfer { context: String },
    InvalidPattern { pattern: String, type_: Type },
    UnhandledCase { discriminant: String },
    Divergence { type_: Type }, // Type represents non-termination
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::UndefinedType { name } => {
                write!(f, "Undefined type: '{}'", name)
            }
            TypeError::UndefinedVariable { name } => {
                write!(f, "Undefined variable: '{}'", name)
            }
            TypeError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected '{}', found '{}'", expected.display_name(), found.display_name())
            }
            TypeError::ArgCountMismatch { expected, found } => {
                write!(f, "Argument count mismatch: expected {}, found {}", expected, found)
            }
            TypeError::TupleLengthMismatch { expected, found } => {
                write!(f, "Tuple length mismatch: expected {}, found {}", expected, found)
            }
            TypeError::InfiniteType { var, type_ } => {
                write!(f, "Infinite type: T{} = {}", var.0, type_.display_name())
            }
            TypeError::SubtypeViolation { sub, sup } => {
                write!(f, "Subtype violation: '{}' is not a subtype of '{}'", sub.display_name(), sup.display_name())
            }
            TypeError::TraitNotImplemented { type_, trait_name } => {
                write!(f, "Type '{}' does not implement trait '{}'", type_.display_name(), trait_name)
            }
            TypeError::RecursiveType { name } => {
                write!(f, "Recursive type definition: '{}'", name)
            }
            TypeError::InvalidFieldAccess { type_, field } => {
                write!(f, "Invalid field access: type '{}' has no field '{}'", type_.display_name(), field)
            }
            TypeError::InvalidMethodCall { type_, method } => {
                write!(f, "Invalid method call: type '{}' has no method '{}'", type_.display_name(), method)
            }
            TypeError::CyclicDependency { types } => {
                write!(f, "Cyclic type dependency: {}", types.join(" -> "))
            }
            TypeError::CannotInfer { context } => {
                write!(f, "Cannot infer type in context: {}", context)
            }
            TypeError::InvalidPattern { pattern, type_ } => {
                write!(f, "Invalid pattern '{}' for type '{}'", pattern, type_.display_name())
            }
            TypeError::UnhandledCase { discriminant } => {
                write!(f, "Unhandled case in pattern matching: '{}'", discriminant)
            }
            TypeError::Divergence { type_ } => {
                write!(f, "Diverging expression of type '{}'", type_.display_name())
            }
        }
    }
}

/// Result of type checking
pub struct TypeCheckResult {
    pub type_: Type,
    pub constraints: Vec<TypeConstraint>,
}

impl Default for TypeCheckResult {
    fn default() -> Self {
        TypeCheckResult {
            type_: Type::Error,
            constraints: vec![],
        }
    }
}

/// Type checker implementation
pub struct TypeChecker {
    /// Type environment
    env: TypeEnv,

    /// Current function return type
    return_type: Option<Type>,

    /// Errors collected during checking
    errors: Vec<TypeError>,

    /// Warnings collected during checking
    warnings: Vec<String>,
}

impl TypeChecker {
    /// Create new type checker
    pub fn new() -> Self {
        TypeChecker {
            env: TypeEnv::new(),
            return_type: None,
            errors: vec![],
            warnings: vec![],
        }
    }

    /// Type check a program
    pub fn check_program(&mut self, program: &Program) -> Result<Type, Vec<TypeError>> {
        println!("TUMC: TypeChecker - Checking program with {} items", program.items.len());

        // First pass: collect all type definitions
        for item in &program.items {
            self.collect_type_def(item)?;
        }

        // Second pass: check all items
        for item in &program.items {
            self.check_item(item)?;
        }

        if self.errors.is_empty() {
            Ok(Type::Unit)
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Collect type definitions from item
    fn collect_type_def(&mut self, item: &Item) -> Result<(), TypeError> {
        match item {
            Item::Struct(def) => {
                let fields: Vec<(String, Type)> = def.fields.iter().map(|f| {
                    (f.name.clone(), self.convert_ann_type(&f.ann_type))
                }).collect();

                self.env.add_type(def.name.clone(), Type::Struct {
                    name: def.name.clone(),
                    fields,
                });
            }
            Item::Enum(def) => {
                let variants: Vec<(String, Option<Type>)> = def.variants.iter().map(|v| {
                    (v.name.clone(), v.fields.as_ref().map(|f| self.convert_ann_type(f)))
                }).collect();

                self.env.add_type(def.name.clone(), Type::Enum {
                    name: def.name.clone(),
                    variants,
                });
            }
            Item::TypeAlias(def) => {
                let type_ = self.convert_ann_type(&def.ann_type);
                self.env.add_type(def.name.clone(), type_);
            }
            Item::Trait(def) => {
                // Register trait name
                self.env.add_type(def.name.clone(), Type::TraitObject {
                    traits: vec![def.name.clone()],
                });
            }
            _ => {}
        }

        Ok(())
    }

    /// Check an item
    fn check_item(&mut self, item: &Item) -> Result<Type, TypeError> {
        match item {
            Item::Function(def) => self.check_function(def),
            Item::Struct(_) | Item::Enum(_) | Item::Trait(_) | Item::Impl(_) => {
                // Already handled in collect_type_def
                Ok(Type::Unit)
            }
            Item::TypeAlias(def) => {
                Ok(self.env.get_type(&def.name).cloned().unwrap_or(Type::Error))
            }
            Item::Constant(def) => {
                let type_ = self.convert_ann_type(&def.ann_type);
                let expr_type = self.check_expr(&def.value)?;
                self.env.unify(&type_, &expr_type)?;
                Ok(type_)
            }
            Item::Module(_) => {
                // Module handling would recursively check module items
                Ok(Type::Unit)
            }
        }
    }

    /// Check a function definition
    fn check_function(&mut self, def: &Function) -> Result<Type, TypeError> {
        println!("TUMC: TypeChecker - Checking function '{}'", def.name);

        // Save current return type
        let old_return = self.return_type.take();

        // Set new return type
        let ret_type = self.convert_ann_type(&def.return_type);
        self.return_type = Some(ret_type.clone());

        // Add parameters to environment
        for param in &def.params {
            let param_type = self.convert_ann_type(&param.ann_type);
            self.env.add_binding(param.name.clone(), param_type);
        }

        // Check function body
        for stmt in &def.body {
            self.check_stmt(stmt)?;
        }

        // Check return type
        if !def.is_ternary() && !def.body.is_empty() {
            if let Some(last) = def.body.last() {
                if !matches!(last, Stmt { kind: StmtKind::Return(_), .. }) {
                    // Unit return if no explicit return
                    self.env.unify(&Type::Unit, &ret_type)?;
                }
            }
        }

        // Restore return type
        self.return_type = old_return;

        // Remove parameters from environment
        for param in &def.params {
            self.env.remove_binding(&param.name);
        }

        let func_type = Type::Function(
            def.params.iter().map(|p| self.convert_ann_type(&p.ann_type)).collect(),
            Box::new(ret_type),
        );

        Ok(func_type)
    }

    /// Check a statement
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<Type, TypeError> {
        match &stmt.kind {
            StmtKind::Expr(expr) => self.check_expr(expr),

            StmtKind::Let(pattern, ann_type, value, body) => {
                let value_type = self.check_expr(value)?;
                let declared_type = match ann_type {
                    Some(t) => self.convert_ann_type(t),
                    None => value_type.clone(),
                };

                self.env.unify(&declared_type, &value_type)?;
                self.env.add_binding(pattern.to_string(), declared_type.clone());

                for s in body {
                    self.check_stmt(s)?;
                }

                self.env.remove_binding(&pattern.to_string());
                Ok(Type::Unit)
            }

            StmtKind::If(condition, then_block, else_block) => {
                let cond_type = self.check_expr(condition)?;
                self.env.unify(&Type::Bool, &cond_type)?;

                for stmt in then_block {
                    self.check_stmt(stmt)?;
                }

                if let Some((_, else_stmts)) = else_block {
                    for stmt in else_stmts {
                        self.check_stmt(stmt)?;
                    }
                }

                Ok(Type::Unit)
            }

            StmtKind::While(condition, body) => {
                let cond_type = self.check_expr(condition)?;
                self.env.unify(&Type::Bool, &cond_type)?;

                for stmt in body {
                    self.check_stmt(stmt)?;
                }

                Ok(Type::Unit)
            }

            StmtKind::For(variable, iterable, body) => {
                let iter_type = self.check_expr(iterable)?;

                // For now, assume iterator yields T
                let elem_type = self.env.fresh_var();
                self.env.unify(&Type::Array(Box::new(elem_type.clone())), &iter_type)?;

                self.env.add_binding(variable.clone(), elem_type);

                for stmt in body {
                    self.check_stmt(stmt)?;
                }

                self.env.remove_binding(variable);
                Ok(Type::Unit)
            }

            StmtKind::Loop(body) => {
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
                Ok(Type::Unit) // Could diverge
            }

            StmtKind::Break => Ok(Type::Error), // Break type is bottom/diverging
            StmtKind::Continue => Ok(Type::Error),

            StmtKind::Return(expr) => {
                let ret_type = self.return_type.clone().unwrap_or(Type::Unit);
                match expr {
                    Some(e) => {
                        let expr_type = self.check_expr(e)?;
                        self.env.unify(&ret_type, &expr_type)?;
                    }
                    None => {
                        self.env.unify(&Type::Unit, &ret_type)?;
                    }
                }
                Ok(Type::Error) // Return is diverging
            }

            StmtKind::Assert(expr) => {
                let expr_type = self.check_expr(expr)?;
                self.env.unify(&Type::Bool, &expr_type)?;
                Ok(Type::Unit)
            }

            StmtKind::Assignment(lhs, rhs) => {
                let lhs_type = self.check_expr(lhs)?;
                let rhs_type = self.check_expr(rhs)?;
                self.env.unify(&lhs_type, &rhs_type)?;
                Ok(Type::Unit)
            }

            StmtKind::CompoundAssignment(lhs, op, rhs) => {
                let lhs_type = self.check_expr(lhs)?;
                let rhs_type = self.check_expr(rhs)?;

                // Compound assignment requires compatible types
                match op {
                    BinOp::Add => {
                        if let Type::String = lhs_type {
                            self.env.unify(&Type::String, &rhs_type)?;
                        } else {
                            self.env.unify(&Type::Int, &lhs_type)?;
                            self.env.unify(&Type::Int, &rhs_type)?;
                        }
                    }
                    _ => {
                        self.env.unify(&Type::Int, &lhs_type)?;
                        self.env.unify(&Type::Int, &rhs_type)?;
                    }
                }

                Ok(lhs_type)
            }
        }
    }

    /// Check an expression
    fn check_expr(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        match &expr.kind {
            ExprKind::Literal(lit) => self.check_literal(lit),

            ExprKind::Identifier(name) => {
                self.env.get_binding(name).cloned()
                    .or_else(|| self.env.get_type(name).cloned())
                    .ok_or_else(|| TypeError::UndefinedVariable { name: name.clone() })
            }

            ExprKind::Binary { op, lhs, rhs } => self.check_binary_expr(op, lhs, rhs),

            ExprKind::Unary { op, operand } => self.check_unary_expr(op, operand),

            ExprKind::Call { function, arguments } => self.check_call(function, arguments),

            ExprKind::MethodCall { object, method, arguments } => {
                self.check_method_call(object, method, arguments)
            }

            ExprKind::If { condition, then_block, else_expr } => {
                self.check_expr(condition)?;
                for (pat, body) in then_block {
                    self.check_pattern(pat)?;
                    for stmt in body {
                        self.check_stmt(stmt)?;
                    }
                }
                if let Some((_, else_body)) = else_expr {
                    for stmt in else_body {
                        self.check_stmt(stmt)?;
                    }
                }
                Ok(Type::Unit) // If expression returns unit in all branches
            }

            ExprKind::Match { discriminant, arms } => {
                let disc_type = self.check_expr(discriminant)?;

                let mut result_type = self.env.fresh_var();
                for arm in arms {
                    self.check_pattern(&arm.pattern)?;
                    let arm_type = self.check_expr(&arm.body)?;
                    // Unify with accumulated result type
                    let _ = self.env.unify(&result_type, &arm_type);
                    result_type = self.env.apply(&result_type);
                }

                Ok(result_type)
            }

            ExprKind::Block(statements) => {
                let mut last_type = Type::Unit;
                for stmt in statements {
                    last_type = self.check_stmt(stmt)?;
                }
                Ok(last_type)
            }

            ExprKind::Tuple(elems) => {
                let elem_types: Result<Vec<_>, _> = elems.iter().map(|e| self.check_expr(e)).collect();
                Ok(Type::Tuple(elem_types?))
            }

            ExprKind::Array(elements) => {
                if elements.is_empty() {
                    return Ok(Type::Array(Box::new(self.env.fresh_var())));
                }
                let elem_types: Result<Vec<_>, _> = elements.iter().map(|e| self.check_expr(e)).collect();
                let elem_types = elem_types?;

                let base_type = elem_types[0].clone();
                for elem_type in &elem_types[1..] {
                    self.env.unify(&base_type, elem_type)?;
                }

                Ok(Type::Array(Box::new(self.env.apply(&base_type))))
            }

            ExprKind::Dict(pairs) => {
                if pairs.is_empty() {
                    let k = self.env.fresh_var();
                    let v = self.env.fresh_var();
                    return Ok(Type::Dict(Box::new(k), Box::new(v)));
                }

                let first = &pairs[0];
                let key_type = self.check_expr(&first.0)?;
                let val_type = self.check_expr(&first.1)?;

                for pair in &pairs[1..] {
                    let k = self.check_expr(&pair.0)?;
                    let v = self.check_expr(&pair.1)?;
                    self.env.unify(&key_type, &k)?;
                    self.env.unify(&val_type, &v)?;
                }

                Ok(Type::Dict(Box::new(key_type), Box::new(val_type)))
            }

            ExprKind::FieldAccess(base, field) => {
                let base_type = self.check_expr(base)?;

                match &self.env.apply(&base_type) {
                    Type::Struct { fields, .. } => {
                        fields.iter().find(|(n, _)| n == field)
                            .map(|(_, t)| Ok(t.clone()))
                            .unwrap_or_else(|| Err(TypeError::InvalidFieldAccess {
                                type_: base_type,
                                field: field.clone(),
                            }))
                    }
                    _ => Err(TypeError::InvalidFieldAccess {
                        type_: base_type,
                        field: field.clone(),
                    }),
                }
            }

            ExprKind::Index(base, index) => {
                let base_type = self.check_expr(base)?;
                let index_type = self.check_expr(index)?;
                self.env.unify(&Type::Int, &index_type)?;

                match &self.env.apply(&base_type) {
                    Type::Array(elem_type) => Ok((*elem_type).clone()),
                    Type::Dict(key_type, val_type) => {
                        self.env.unify(key_type, &index_type)?;
                        Ok((*val_type).clone())
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Array(Box::new(self.env.fresh_var())),
                        found: base_type,
                    }),
                }
            }

            ExprKind::Range(start, end, inclusive) => {
                let start_type = self.check_expr(start)?;
                let end_type = self.check_expr(end)?;
                self.env.unify(&Type::Int, &start_type)?;
                self.env.unify(&Type::Int, &end_type)?;
                Ok(Type::Array(Box::new(Type::Int)))
            }

            ExprKind::Closure(params, body, return_type) => {
                let old_return = self.return_type.take();

                let param_types: Vec<Type> = params.iter().map(|p| {
                    self.convert_ann_type(&p.ann_type)
                }).collect();

                for (i, param) in params.iter().enumerate() {
                    self.env.add_binding(param.name.clone(), param_types[i].clone());
                }

                if let Some(rt) = return_type {
                    self.return_type = Some(self.convert_ann_type(rt));
                } else {
                    self.return_type = Some(self.env.fresh_var());
                }

                for stmt in body {
                    self.check_stmt(stmt)?;
                }

                let ret_type = self.return_type.clone().unwrap_or(Type::Unit);
                self.return_type = old_return;

                for param in params {
                    self.env.remove_binding(&param.name);
                }

                Ok(Type::Function(param_types, Box::new(ret_type)))
            }

            ExprKind::Cast(expr, target_type) => {
                self.check_expr(expr)?;
                Ok(self.convert_ann_type(target_type))
            }

            ExprKind::Await(expr) => {
                self.check_expr(expr)
            }

            ExprKind::Try(expr) => {
                self.check_expr(expr)
            }

            ExprKind::Parenthesis(expr) => {
                self.check_expr(expr)
            }

            ExprKind::Path(_) => {
                Ok(self.env.fresh_var())
            }
        }
    }

    /// Check a literal
    fn check_literal(&self, lit: &Literal) -> Type {
        match lit {
            Literal::Unit => Type::Unit,
            Literal::Bool(_) => Type::Bool,
            Literal::Int(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::Char(_) => Type::Char,
            Literal::String(_) => Type::String,
            Literal::Byte(_) => Type::Int,
            Literal::RawString(_) => Type::String,
            Literal::ByteString(_) => Type::Array(Box::new(Type::Int)),
            Literal::Verbatim(_) => Type::String,
        }
    }

    /// Check binary expression
    fn check_binary_expr(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Result<Type, TypeError> {
        let lhs_type = self.check_expr(lhs)?;
        let rhs_type = self.check_expr(rhs)?;

        match op {
            // Arithmetic operators
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                match &lhs_type {
                    Type::Int => {
                        self.env.unify(&Type::Int, &rhs_type)?;
                        Ok(Type::Int)
                    }
                    Type::Float => {
                        self.env.unify(&Type::Float, &rhs_type)?;
                        Ok(Type::Float)
                    }
                    Type::String => {
                        if matches!(op, BinOp::Add) {
                            self.env.unify(&Type::String, &rhs_type)?;
                            Ok(Type::String)
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: Type::String,
                                found: rhs_type,
                            })
                        }
                    }
                    Type::Array(_) => {
                        // Array concatenation
                        self.env.unify(&lhs_type, &rhs_type)?;
                        Ok(lhs_type)
                    }
                    _ => {
                        self.env.unify(&lhs_type, &rhs_type)?;
                        Ok(lhs_type)
                    }
                }
            }

            // Bitwise operators
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor | BinOp::Shl | BinOp::Shr => {
                self.env.unify(&Type::Int, &lhs_type)?;
                self.env.unify(&Type::Int, &rhs_type)?;
                Ok(Type::Int)
            }

            // Comparison operators
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => {
                self.env.unify(&lhs_type, &rhs_type)?;
                Ok(Type::Bool)
            }

            // Logical operators
            BinOp::And | BinOp::Or => {
                self.env.unify(&Type::Bool, &lhs_type)?;
                self.env.unify(&Type::Bool, &rhs_type)?;
                Ok(Type::Bool)
            }

            // Assignment operators (shouldn't reach here directly)
            BinOp::Assign => {
                self.env.unify(&lhs_type, &rhs_type)?;
                Ok(Type::Unit)
            }

            BinOp::Range | BinOp::RangeInclusive => {
                self.env.unify(&Type::Int, &lhs_type)?;
                self.env.unify(&Type::Int, &rhs_type)?;
                Ok(Type::Array(Box::new(Type::Int)))
            }

            // Pipeline operator
            BinOp::Pipe => Ok(rhs_type),
        }
    }

    /// Check unary expression
    fn check_unary_expr(&mut self, op: &UnOp, operand: &Expr) -> Result<Type, TypeError> {
        let operand_type = self.check_expr(operand)?;

        match op {
            UnOp::Neg => {
                match &operand_type {
                    Type::Int | Type::Float => Ok(operand_type),
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Int,
                        found: operand_type,
                    }),
                }
            }
            UnOp::Not => {
                self.env.unify(&Type::Bool, &operand_type)?;
                Ok(Type::Bool)
            }
            UnOp::BitNot => {
                self.env.unify(&Type::Int, &operand_type)?;
                Ok(Type::Int)
            }
            UnOp::Deref => {
                match &self.env.apply(&operand_type) {
                    Type::Ref(inner) | Type::MutRef(inner) => Ok((*inner).clone()),
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Ref(Box::new(self.env.fresh_var())),
                        found: operand_type,
                    }),
                }
            }
        }
    }

    /// Check function call
    fn check_call(&mut self, function: &Expr, arguments: &[Expr]) -> Result<Type, TypeError> {
        let func_type = self.check_expr(function)?;

        match &self.env.apply(&func_type) {
            Type::Function(params, ret) => {
                if params.len() != arguments.len() {
                    return Err(TypeError::ArgCountMismatch {
                        expected: params.len(),
                        found: arguments.len(),
                    });
                }

                for (param_type, arg) in params.iter().zip(arguments) {
                    let arg_type = self.check_expr(arg)?;
                    self.env.unify(param_type, &arg_type)?;
                }

                Ok((**ret).clone())
            }
            _ => {
                // Try to infer as a generic call
                let ret_type = self.env.fresh_var();
                let param_types: Vec<Type> = arguments.iter().map(|_| self.env.fresh_var()).collect();

                let inferred_func_type = Type::Function(param_types, Box::new(ret_type.clone()));
                self.env.unify(&func_type, &inferred_func_type)?;

                Ok(ret_type)
            }
        }
    }

    /// Check method call
    fn check_method_call(&mut self, object: &Expr, method: &str, arguments: &[Expr]) -> Result<Type, TypeError> {
        let obj_type = self.check_expr(object)?;

        // Look up method on type
        match &self.env.apply(&obj_type) {
            Type::Struct { name, fields } => {
                // Check if method is a field
                if let Some(field_type) = fields.iter().find(|(n, _)| n == method) {
                    if arguments.is_empty() {
                        return Ok(field_type.clone());
                    }
                }

                // Look up impl methods
                // For now, assume method returns some type
                Ok(self.env.fresh_var())
            }
            Type::Named(name) => {
                // Look for impl blocks
                Ok(self.env.fresh_var())
            }
            _ => Err(TypeError::InvalidMethodCall {
                type_: obj_type,
                method: method.to_string(),
            }),
        }
    }

    /// Check pattern
    fn check_pattern(&mut self, pattern: &Pattern) -> Result<Type, TypeError> {
        match pattern {
            Pattern::Wildcard => Ok(self.env.fresh_var()),
            Pattern::Literal(lit) => Ok(self.check_literal(lit)),
            Pattern::Range(start, end) => {
                self.env.unify(&Type::Int, &self.check_literal(start))?;
                self.env.unify(&Type::Int, &self.check_literal(end))?;
                Ok(Type::Int)
            }
            Pattern::Binding(name) => {
                let var_type = self.env.fresh_var();
                self.env.add_binding(name.clone(), var_type.clone());
                Ok(var_type)
            }
            Pattern::Constructor { name, args } => {
                let type_ = self.env.get_type(name)
                    .cloned()
                    .ok_or_else(|| TypeError::UndefinedType { name: name.clone() })?;

                match &type_ {
                    Type::Enum { variants, .. } => {
                        if let Some((_, Some(field_type))) = variants.iter().find(|(n, _)| n == name) {
                            for arg in args {
                                self.check_pattern(arg)?;
                            }
                            Ok(field_type.clone())
                        } else {
                            Ok(type_)
                        }
                    }
                    Type::Struct { fields, .. } => {
                        if args.len() != fields.len() {
                            return Err(TypeError::ArgCountMismatch {
                                expected: fields.len(),
                                found: args.len(),
                            });
                        }
                        for arg in args {
                            self.check_pattern(arg)?;
                        }
                        Ok(type_)
                    }
                    _ => Ok(type_),
                }
            }
            Pattern::Tuple(patterns) => {
                let elem_types: Result<Vec<_>, _> = patterns.iter().map(|p| self.check_pattern(p)).collect();
                Ok(Type::Tuple(elem_types?))
            }
            Pattern::Slice(patterns) => {
                let elem_type = self.env.fresh_var();
                for p in patterns {
                    let p_type = self.check_pattern(p)?;
                    self.env.unify(&elem_type, &p_type)?;
                }
                Ok(Type::Array(Box::new(self.env.apply(&elem_type))))
            }
            Pattern::Or(patterns) => {
                if patterns.is_empty() {
                    return Ok(self.env.fresh_var());
                }
                let first_type = self.check_pattern(&patterns[0])?;
                for p in &patterns[1..] {
                    let p_type = self.check_pattern(p)?;
                    self.env.unify(&first_type, &p_type)?;
                }
                Ok(self.env.apply(&first_type))
            }
            Pattern::As(pattern, name) => {
                let pattern_type = self.check_pattern(pattern)?;
                self.env.add_binding(name.clone(), pattern_type.clone());
                Ok(pattern_type)
            }
        }
    }

    /// Convert annotation type to internal type
    fn convert_ann_type(&self, ann_type: &AnnType) -> Type {
        match ann_type {
            AnnType::Unit => Type::Unit,
            AnnType::Bool => Type::Bool,
            AnnType::Int => Type::Int,
            AnnType::Float => Type::Float,
            AnnType::Char => Type::Char,
            AnnType::String => Type::String,
            AnnType::Named(name) => {
                self.env.get_type(name).cloned().unwrap_or_else(|| Type::Named(name.clone()))
            }
            AnnType::Ref(inner) => Type::Ref(Box::new(self.convert_ann_type(inner))),
            AnnType::MutRef(inner) => Type::MutRef(Box::new(self.convert_ann_type(inner))),
            AnnType::Ptr(inner) => Type::Ptr(Box::new(self.convert_ann_type(inner))),
            AnnType::Array(inner) => Type::Array(Box::new(self.convert_ann_type(inner))),
            AnnType::Func(params, ret) => Type::Function(
                params.iter().map(|p| self.convert_ann_type(p)).collect(),
                Box::new(self.convert_ann_type(ret)),
            ),
            AnnType::Tuple(elems) => Type::Tuple(elems.iter().map(|e| self.convert_ann_type(e)).collect()),
            AnnType::Optional(inner) => Type::Optional(Box::new(self.convert_ann_type(inner))),
            AnnType::Result(ok, err) => Type::Result(
                Box::new(self.convert_ann_type(ok)),
                Box::new(self.convert_ann_type(err)),
            ),
            AnnType::Error => Type::Error,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_unification() {
        let mut env = TypeEnv::new();

        let t1 = Type::TypeVar(TypeVar(0));
        let t2 = Type::Int;

        assert!(env.unify(&t1, &t2).is_ok());
        assert_eq!(env.apply(&t1), Type::Int);
    }

    #[test]
    fn test_function_type_unification() {
        let mut env = TypeEnv::new();

        let func1 = Type::Function(vec![Type::Int], Box::new(Type::Int));
        let func2 = Type::Function(vec![Type::Int], Box::new(Type::TypeVar(TypeVar(0))));

        assert!(env.unify(&func1, &func2).is_ok());
    }

    #[test]
    fn test_occurs_check() {
        let mut env = TypeEnv::new();

        let t1 = Type::TypeVar(TypeVar(0));
        let t2 = Type::Function(vec![], Box::new(t1.clone()));

        // Should fail: T0 = fn() -> T0 (infinite type)
        assert!(env.unify(&t1, &t2).is_err());
    }
}
