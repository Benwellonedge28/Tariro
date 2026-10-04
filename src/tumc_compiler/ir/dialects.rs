// IR Dialects Module - Domain-specific IR levels

pub mod typed_ptr;
pub mod linalg;
pub mod gpu;
pub mod quantum;

pub use typed_ptr::*;
pub use linalg::*;
pub use gpu::*;
pub use quantum::*;

// ============================================================================
// Dialect Infrastructure
// ============================================================================

/// Base trait for all IR dialects
pub trait Dialect {
    /// Dialect name/namespace
    fn name(&self) -> &'static str;

    /// Convert from standard IR
    fn from_standard(&mut self, module: &Module) -> Result<(), DialectError>;

    /// Convert to standard IR
    fn to_standard(&self) -> Result<Module, DialectError>;
}

/// Dialect conversion error
#[derive(Debug)]
pub enum DialectError {
    UnsupportedOperation(String),
    TypeMismatch { expected: String, found: String },
    ConversionFailed(String),
}

impl std::fmt::Display for DialectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DialectError::UnsupportedOperation(op) => {
                write!(f, "Unsupported operation: {}", op)
            }
            DialectError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            DialectError::ConversionFailed(msg) => {
                write!(f, "Conversion failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for DialectError {}

// ============================================================================
// MLIR-style Dialect Infrastructure
// ============================================================================

/// Dialect registry
pub struct DialectRegistry {
    dialects: HashMap<String, Box<dyn Dialect>>,
}

impl DialectRegistry {
    pub fn new() -> Self {
        Self {
            dialects: HashMap::new(),
        }
    }

    pub fn register<D: Dialect + 'static>(&mut self, dialect: D) {
        let name = dialect.name().to_string();
        self.dialects.insert(name, Box::new(dialect));
    }

    pub fn get(&self, name: &str) -> Option<&dyn Dialect> {
        self.dialects.get(name).map(|b| b.as_ref())
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.dialects.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for DialectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Attribute in MLIR-style dialects
#[derive(Debug, Clone)]
pub enum Attribute {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Type(IrType),
    DenseElements(Vec<i64>),
    SparseElements(Vec<(usize, i64)>),
    AffineMap(AffineMap),
    SymbolRef(String),
    Array(Vec<Attribute>),
    Dict(HashMap<String, Attribute>),
}

/// Affine expression
#[derive(Debug, Clone)]
pub enum AffineExpr {
    Dim(usize),
    Symbol(usize),
    Add(Box<AffineExpr>, Box<AffineExpr>),
    Mul(Box<AffineExpr>, Box<AffineExpr>),
    FloorDiv(Box<AffineExpr>, Box<AffineExpr>),
    CeilDiv(Box<AffineExpr>, Box<AffineExpr>),
    Mod(Box<AffineExpr>, Box<AffineExpr>),
    Constant(i64),
    Neg(Box<AffineExpr>),
}

impl AffineExpr {
    pub fn dim(idx: usize) -> Self {
        AffineExpr::Dim(idx)
    }

    pub fn sym(idx: usize) -> Self {
        AffineExpr::Symbol(idx)
    }

    pub fn add(self, other: Self) -> Self {
        AffineExpr::Add(Box::new(self), Box::new(other))
    }

    pub fn mul(self, other: Self) -> Self {
        AffineExpr::Mul(Box::new(self), Box::new(other))
    }
}

/// Affine map (multi-dimensional affine expression)
#[derive(Debug, Clone)]
pub struct AffineMap {
    pub dims: usize,
    pub symbols: usize,
    pub results: Vec<AffineExpr>,
}

impl AffineMap {
    pub fn new(dims: usize, symbols: usize, results: Vec<AffineExpr>) -> Self {
        Self { dims, symbols, results }
    }

    /// Create a simple identity map
    pub fn identity(n: usize) -> Self {
        Self {
            dims: n,
            symbols: 0,
            results: (0..n).map(AffineExpr::Dim).collect(),
        }
    }

    /// Create a map with one result
    pub fn single(dims: usize, symbols: usize, expr: AffineExpr) -> Self {
        Self {
            dims,
            symbols,
            results: vec![expr],
        }
    }

    pub fn eval(&self, dim_values: &[i64], sym_values: &[i64]) -> Vec<i64> {
        fn eval_expr(expr: &AffineExpr, dims: &[i64], syms: &[i64]) -> i64 {
            match expr {
                AffineExpr::Dim(idx) => dims.get(*idx).copied().unwrap_or(0),
                AffineExpr::Symbol(idx) => syms.get(*idx).copied().unwrap_or(0),
                AffineExpr::Constant(v) => *v,
                AffineExpr::Neg(e) => -eval_expr(e, dims, syms),
                AffineExpr::Add(a, b) => eval_expr(a, dims, syms) + eval_expr(b, dims, syms),
                AffineExpr::Mul(a, b) => eval_expr(a, dims, syms) * eval_expr(b, dims, syms),
                AffineExpr::FloorDiv(a, b) => {
                    let (div, rem) = div_rem(eval_expr(a, dims, syms), eval_expr(b, dims, syms));
                    div
                }
                AffineExpr::CeilDiv(a, b) => {
                    let (div, rem) = div_rem(eval_expr(a, dims, syms), eval_expr(b, dims, syms));
                    div + if rem != 0 { 1 } else { 0 }
                }
                AffineExpr::Mod(a, b) => {
                    let b_val = eval_expr(b, dims, syms);
                    if b_val == 0 { 0 } else { eval_expr(a, dims, syms) % b_val }
                }
            }
        }

        fn div_rem(a: i64, b: i64) -> (i64, i64) {
            if b == 0 { (0, 0) } else { (a / b, a % b) }
        }

        self.results.iter()
            .map(|r| eval_expr(r, dim_values, sym_values))
            .collect()
    }
}

use std::collections::HashMap;
