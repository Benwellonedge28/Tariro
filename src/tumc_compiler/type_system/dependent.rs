// Dependent Types

use super::*;

/// Dependent type expression
#[derive(Debug, Clone)]
pub enum DepTypeExpr {
    Var(String),
    Const(Const),
    App(Box<DepTypeExpr>, Box<DepTypeExpr>),
    Lambda(String, Box<DepTypeExpr>, Box<DepTypeExpr>),
    Pi(String, Box<DepTypeExpr>, Box<DepTypeExpr>),
    Sigma(String, Box<DepTypeExpr>, Box<DepTypeExpr>),
    Eq(Box<DepTypeExpr>, Box<DepTypeExpr>),
    Nat,
    NatLit(u64),
    Add(Box<DepTypeExpr>, Box<DepTypeExpr>),
    Sub(Box<DepTypeExpr>, Box<DepTypeExpr>),
    Mul(Box<DepTypeExpr>, Box<DepTypeExpr>),
}

#[derive(Debug, Clone)]
pub enum Const {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

/// Dependent type checker
pub struct DependentTypeChecker {
    pub context: DepContext,
}

#[derive(Debug, Clone, Default)]
pub struct DepContext {
    pub vars: Vec<(String, DepTypeExpr)>,
    pub definitions: HashMap<String, DepTypeExpr>,
}

impl DepContext {
    pub fn extend(&self, name: String, ty: DepTypeExpr) -> Self {
        let mut ctx = self.clone();
        ctx.vars.push((name, ty));
        ctx
    }

    pub fn lookup(&self, name: &str) -> Option<&DepTypeExpr> {
        self.vars.iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, t)| t)
    }
}

impl DependentTypeChecker {
    pub fn new() -> Self {
        Self {
            context: DepContext::default(),
        }
    }

    /// Check if a type is well-formed
    pub fn check_type(&self, ty: &DepTypeExpr) -> Result<(), TypeError> {
        match ty {
            DepTypeExpr::Nat | DepTypeExpr::NatLit(_) => Ok(()),
            DepTypeExpr::Var(name) => {
                if self.context.lookup(name).is_some() {
                    Ok(())
                } else {
                    Err(TypeError::UndefinedVariable(name.clone()))
                }
            }
            DepTypeExpr::Pi(_, a, b) => {
                self.check_type(a)?;
                self.check_type(b)?;
                Ok(())
            }
            DepTypeExpr::Sigma(_, a, b) => {
                self.check_type(a)?;
                self.check_type(b)?;
                Ok(())
            }
            DepTypeExpr::Eq(a, b) => {
                self.check_type(a)?;
                self.check_type(b)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Convert dependent type to MonoType (erasing value dependencies)
    pub fn erase(&self, ty: &DepTypeExpr) -> MonoType {
        match ty {
            DepTypeExpr::Nat => MonoType::Int(IntSize::Usize),
            DepTypeExpr::NatLit(_) => MonoType::Int(IntSize::Usize),
            DepTypeExpr::Var(_) => MonoType::Wildcard,
            DepTypeExpr::App(f, _) => self.erase(f),
            DepTypeExpr::Add(a, b) => {
                // Both must be Nat
                self.erase(a)
            }
            _ => MonoType::Wildcard,
        }
    }

    /// Normalize a dependent type expression
    pub fn normalize(&self, expr: &DepTypeExpr) -> DepTypeExpr {
        match expr {
            DepTypeExpr::Var(name) => {
                if let Some(def) = self.context.lookup(name) {
                    self.normalize(def)
                } else {
                    expr.clone()
                }
            }
            DepTypeExpr::App(f, arg) => {
                let f_norm = self.normalize(f);
                let arg_norm = self.normalize(arg);
                self.beta_reduce(&f_norm, &arg_norm)
            }
            DepTypeExpr::Add(a, b) => {
                match (self.normalize(a), self.normalize(b)) {
                    (DepTypeExpr::NatLit(x), DepTypeExpr::NatLit(y)) => {
                        DepTypeExpr::NatLit(x + y)
                    }
                    (a, b) => DepTypeExpr::Add(Box::new(a), Box::new(b)),
                }
            }
            DepTypeExpr::Sub(a, b) => {
                match (self.normalize(a), self.normalize(b)) {
                    (DepTypeExpr::NatLit(x), DepTypeExpr::NatLit(y)) => {
                        DepTypeExpr::NatLit(x.saturating_sub(y))
                    }
                    (a, b) => DepTypeExpr::Sub(Box::new(a), Box::new(b)),
                }
            }
            _ => expr.clone(),
        }
    }

    /// Beta reduction
    fn beta_reduce(&self, f: &DepTypeExpr, arg: &DepTypeExpr) -> DepTypeExpr {
        match f {
            DepTypeExpr::Lambda(x, _, body) => {
                self.subst(body, x, arg)
            }
            _ => DepTypeExpr::App(Box::new(f.clone()), Box::new(arg.clone())),
        }
    }

    /// Substitution
    fn subst(&self, expr: &DepTypeExpr, var: &str, replacement: &DepTypeExpr) -> DepTypeExpr {
        match expr {
            DepTypeExpr::Var(name) if name == var => replacement.clone(),
            DepTypeExpr::Var(_) => expr.clone(),
            DepTypeExpr::Lambda(x, ty, body) if x != var => {
                DepTypeExpr::Lambda(
                    x.clone(),
                    Box::new(self.subst(ty, var, replacement)),
                    Box::new(self.subst(body, var, replacement)),
                )
            }
            DepTypeExpr::App(func, arg) => {
                DepTypeExpr::App(
                    Box::new(self.subst(func, var, replacement)),
                    Box::new(self.subst(arg, var, replacement)),
                )
            }
            _ => expr.clone(),
        }
    }
}

impl Default for DependentTypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Refinement type (subset of dependent type)
#[derive(Debug, Clone)]
pub struct RefinementType {
    pub base: MonoType,
    pub predicate: Predicate,
}

#[derive(Debug, Clone)]
pub enum Predicate {
    True,
    And(Box<Predicate>, Box<Predicate>),
    Or(Box<Predicate>, Box<Predicate>),
    Not(Box<Predicate>),
    Var(String),
    App(String, Vec<DepTypeExpr>),
    Lt(DepTypeExpr, DepTypeExpr),
    Le(DepTypeExpr, DepTypeExpr),
    Gt(DepTypeExpr, DepTypeExpr),
    Ge(DepTypeExpr, DepTypeExpr),
    Eq(DepTypeExpr, DepTypeExpr),
}

impl Predicate {
    /// Evaluate predicate at runtime (for runtime checks)
    pub fn evaluate(&self, env: &HashMap<String, Const>) -> Option<bool> {
        match self {
            Predicate::True => Some(true),
            Predicate::Var(name) => env.get(name).map(|_| true),
            Predicate::Lt(a, b) => {
                let a_val = self.eval_expr(a, env)?;
                let b_val = self.eval_expr(b, env)?;
                Some(a_val < b_val)
            }
            Predicate::Le(a, b) => {
                let a_val = self.eval_expr(a, env)?;
                let b_val = self.eval_expr(b, env)?;
                Some(a_val <= b_val)
            }
            Predicate::Eq(a, b) => {
                let a_val = self.eval_expr(a, env)?;
                let b_val = self.eval_expr(b, env)?;
                Some(a_val == b_val)
            }
            _ => None,
        }
    }

    fn eval_expr(&self, expr: &DepTypeExpr, env: &HashMap<String, Const>) -> Option<Const> {
        match expr {
            DepTypeExpr::NatLit(n) => Some(Const::Int(*n as i64)),
            DepTypeExpr::Var(name) => env.get(name).cloned(),
            _ => None,
        }
    }
}

/// Indexed types (naturals at type level)
#[derive(Debug, Clone)]
pub struct IndexedType {
    pub index: DepTypeExpr,
}

impl IndexedType {
    pub fn new(index: DepTypeExpr) -> Self {
        Self { index }
    }

    pub fn array_len(&self) -> Option<u64> {
        if let DepTypeExpr::NatLit(n) = self.index {
            Some(n)
        } else {
            None
        }
    }
}

/// Vector type with length at type level
pub struct VecType {
    pub element: MonoType,
    pub length: DepTypeExpr,
}

impl VecType {
    pub fn new(element: MonoType, length: DepTypeExpr) -> Self {
        Self { element, length }
    }

    pub fn of(element: MonoType, n: u64) -> Self {
        Self {
            element,
            length: DepTypeExpr::NatLit(n),
        }
    }
}

use std::collections::HashMap;
