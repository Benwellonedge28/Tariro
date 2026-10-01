//! TUMC AST - Abstract Syntax Tree
//!
//! This module defines the abstract syntax tree representation for
//! the TUMC compiler intermediate representation.
//!
//! # Node Types
//!
//! - [`Program`] - Root node containing all top-level items
//! - [`Item`] - Top-level declarations (functions, structs, etc.)
//! - [`Stmt`] - Statements (let, if, while, etc.)
//! - [`Expr`] - Expressions (literals, binary ops, etc.)
//! - [`Pattern`] - Pattern matching for destructuring

use crate::tumc_compiler::{SourceLocation, Type};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Program
// ============================================================================

/// Root node of the AST
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    /// Items (top-level declarations)
    pub items: Vec<Item>,

    /// Source spans for each item
    pub spans: HashMap<usize, SourceSpan>,
}

/// Source span
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl Program {
    /// Create a new program
    pub fn new(items: Vec<Item>) -> Self {
        Self {
            items,
            spans: HashMap::new(),
        }
    }

    /// Count total nodes in AST
    pub fn node_count(&self) -> usize {
        self.items.iter().map(|i| i.node_count()).sum()
    }
}

// ============================================================================
// Items (Top-Level Declarations)
// ============================================================================

/// Top-level item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Item {
    /// Function declaration
    Function(Function),

    /// Struct declaration
    Struct(Struct),

    /// Enum declaration
    Enum(Enum),

    /// Trait declaration
    Trait(Trait),

    /// Impl block
    Impl(Impl),

    /// Type alias
    TypeAlias(TypeAlias),

    /// Constant declaration
    Constant(Constant),

    /// Module declaration
    Module(Module),
}

impl Item {
    /// Get item name
    pub fn name(&self) -> &str {
        match self {
            Item::Function(f) => &f.name,
            Item::Struct(s) => &s.name,
            Item::Enum(e) => &e.name,
            Item::Trait(t) => &t.name,
            Item::Impl(i) => &i.name.as_deref().unwrap_or("impl"),
            Item::TypeAlias(t) => &t.name,
            Item::Constant(c) => &c.name,
            Item::Module(m) => &m.name,
        }
    }

    /// Count nodes in this item
    pub fn node_count(&self) -> usize {
        match self {
            Item::Function(f) => f.node_count(),
            Item::Struct(s) => s.node_count(),
            Item::Enum(e) => e.node_count(),
            Item::Trait(t) => t.node_count(),
            Item::Impl(i) => i.node_count(),
            Item::TypeAlias(t) => t.node_count(),
            Item::Constant(c) => c.node_count(),
            Item::Module(m) => m.node_count(),
        }
    }
}

/// Function declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Function name
    pub name: String,

    /// Generic parameters
    pub generics: Vec<GenericParam>,

    /// Parameters
    pub parameters: Vec<Param>,

    /// Return type
    pub return_type: Option<Type>,

    /// Function body
    pub body: Block,

    /// Visibility
    pub visibility: Visibility,

    /// Is async
    pub is_async: bool,

    /// Is external (no body)
    pub is_external: bool,

    /// Location
    pub location: SourceSpan,
}

impl Function {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len() + self.parameters.iter().map(|p| p.node_count()).sum::<usize>()
            + self.body.node_count()
    }
}

/// Generic parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<TypeBound>,
}

/// Type bound
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeBound {
    /// Implements a trait
    Trait(String),
    /// Lifetime parameter
    Lifetime(String),
}

/// Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub param_type: Type,
    pub is_mutable: bool,
    pub is_reference: bool,
    pub location: SourceSpan,
}

impl Param {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.param_type.node_count()
    }
}

/// Visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Crate,
}

/// Struct declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<Field>,
    pub location: SourceSpan,
}

impl Struct {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len() + self.fields.iter().map(|f| f.node_count()).sum::<usize>()
    }
}

/// Field declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: Type,
    pub visibility: Visibility,
    pub location: SourceSpan,
}

impl Field {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.field_type.node_count()
    }
}

/// Enum declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub location: SourceSpan,
}

impl Enum {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len() + self.variants.len()
    }
}

/// Enum variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Type>,
    pub location: SourceSpan,
}

/// Trait declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub items: Vec<TraitItem>,
    pub location: SourceSpan,
}

impl Trait {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len() + self.items.len()
    }
}

/// Trait item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraitItem {
    Function { name: String, sig: FunctionSig },
    Type { name: String },
}

/// Function signature (without body)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSig {
    pub generics: Vec<GenericParam>,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
}

/// Impl block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Impl {
    pub name: Option<String>,
    pub generics: Vec<GenericParam>,
    pub trait_name: Option<String>,
    pub for_type: Type,
    pub items: Vec<ImplItem>,
    pub location: SourceSpan,
}

impl Impl {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len() + self.items.iter().map(|i| i.node_count()).sum::<usize>()
    }
}

/// Impl item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplItem {
    Function(Function),
    Constant(Constant),
}

/// Type alias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub aliased_type: Type,
    pub location: SourceSpan,
}

impl TypeAlias {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len() + self.aliased_type.node_count()
    }
}

/// Constant declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    pub const_type: Type,
    pub value: Expr,
    pub location: SourceSpan,
}

impl Constant {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        2 + self.value.node_count()
    }
}

/// Module declaration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub items: Vec<Item>,
    pub location: SourceSpan,
}

impl Module {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.items.iter().map(|i| i.node_count()).sum::<usize>()
    }
}

// ============================================================================
// Statements
// ============================================================================

/// Statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    /// Let binding
    Let {
        pattern: Pattern,
        let_type: Option<Type>,
        initializer: Option<Expr>,
        location: SourceSpan,
    },

    /// Expression statement
    Expr {
        expr: Expr,
        location: SourceSpan,
    },

    /// Assignment
    Assign {
        target: Expr,
        value: Expr,
        location: SourceSpan,
    },

    /// Compound assignment (+=, -=, etc.)
    CompoundAssign {
        target: Expr,
        op: BinOp,
        value: Expr,
        location: SourceSpan,
    },

    /// Return statement
    Return {
        value: Option<Expr>,
        location: SourceSpan,
    },

    /// Break statement
    Break {
        label: Option<String>,
        location: SourceSpan,
    },

    /// Continue statement
    Continue {
        label: Option<String>,
        location: SourceSpan,
    },

    /// If statement
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Stmt>,
        location: SourceSpan,
    },

    /// While loop
    While {
        condition: Expr,
        body: Block,
        label: Option<String>,
        location: SourceSpan,
    },

    /// For loop
    For {
        pattern: Pattern,
        iterator: Expr,
        body: Block,
        label: Option<String>,
        location: SourceSpan,
    },

    /// Loop (infinite loop)
    Loop {
        body: Block,
        label: Option<String>,
        location: SourceSpan,
    },

    /// Match expression
    Match {
        value: Expr,
        arms: Vec<MatchArm>,
        location: SourceSpan,
    },
}

impl Stmt {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        match self {
            Stmt::Let { pattern, initializer, .. } => {
                1 + pattern.node_count() + initializer.as_ref().map(|e| e.node_count()).unwrap_or(0)
            }
            Stmt::Expr { expr, .. } => 1 + expr.node_count(),
            Stmt::Assign { target, value, .. } => 1 + target.node_count() + value.node_count(),
            Stmt::CompoundAssign { target, value, .. } => 1 + target.node_count() + value.node_count(),
            Stmt::Return { value, .. } => 1 + value.as_ref().map(|e| e.node_count()).unwrap_or(0),
            Stmt::Break { .. } | Stmt::Continue { .. } => 1,
            Stmt::If { condition, then_block, else_block, .. } => {
                1 + condition.node_count() + then_block.node_count()
                    + else_block.as_ref().map(|e| e.node_count()).unwrap_or(0)
            }
            Stmt::While { condition, body, .. } => 1 + condition.node_count() + body.node_count(),
            Stmt::For { pattern, iterator, body, .. } => {
                1 + pattern.node_count() + iterator.node_count() + body.node_count()
            }
            Stmt::Loop { body, .. } => 1 + body.node_count(),
            Stmt::Match { value, arms, .. } => {
                1 + value.node_count() + arms.iter().map(|a| a.node_count()).sum::<usize>()
            }
        }
    }
}

/// Match arm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub location: SourceSpan,
}

impl MatchArm {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.pattern.node_count()
            + self.guard.as_ref().map(|e| e.node_count()).unwrap_or(0)
            + self.body.node_count()
    }
}

/// Pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    /// Wildcard pattern (_)
    Wildcard(SourceSpan),

    /// Variable pattern
    Variable {
        name: String,
        mutable: bool,
        location: SourceSpan,
    },

    /// Literal pattern
    Literal(Literal, SourceSpan),

    /// Tuple pattern
    Tuple(Vec<Pattern>, SourceSpan),

    /// Array pattern
    Array(Vec<Pattern>, SourceSpan),

    /// Struct pattern
    Struct {
        name: String,
        fields: Vec<(String, Option<Pattern>)>,
        rest: bool,
        location: SourceSpan,
    },

    /// Enum variant pattern
    Variant {
        enum_name: String,
        variant_name: String,
        fields: Vec<Pattern>,
        location: SourceSpan,
    },

    /// Range pattern
    Range {
        start: Literal,
        end: Literal,
        location: SourceSpan,
    },

    /// Or pattern
    Or(Vec<Pattern>, SourceSpan),

    /// Reference pattern
    Ref(Box<Pattern>, SourceSpan),

    /// Mut reference pattern
    MutRef(Box<Pattern>, SourceSpan),
}

impl Pattern {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        match self {
            Pattern::Wildcard(_) => 1,
            Pattern::Variable { .. } => 1,
            Pattern::Literal(_, _) => 1,
            Pattern::Tuple(pats, _) | Pattern::Or(pats, _) => {
                1 + pats.iter().map(|p| p.node_count()).sum::<usize>()
            }
            Pattern::Array(pats, _) => 1 + pats.iter().map(|p| p.node_count()).sum::<usize>(),
            Pattern::Struct { fields, rest, .. } => {
                1 + fields.len() + if *rest { 1 } else { 0 }
            }
            Pattern::Variant { fields, .. } => {
                1 + fields.iter().map(|p| p.node_count()).sum::<usize>()
            }
            Pattern::Range { .. } => 1,
            Pattern::Ref(p, _) | Pattern::MutRef(p, _) => 1 + p.node_count(),
        }
    }
}

// ============================================================================
// Expressions
// ============================================================================

/// Expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expr {
    /// Expression kind
    pub kind: ExprKind,

    /// Type (filled by type checker)
    pub expr_type: Option<Type>,

    /// Location
    pub location: SourceSpan,
}

impl Expr {
    /// Create a new expression
    pub fn new(kind: ExprKind, location: SourceSpan) -> Self {
        Self {
            kind,
            expr_type: None,
            location,
        }
    }

    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.kind.node_count()
    }
}

/// Expression kind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExprKind {
    /// Literal value
    Literal(Literal),

    /// Identifier
    Identifier(String),

    /// Path (a::b::c)
    Path(Vec<String>),

    /// Block expression
    Block(Block),

    /// Function call
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },

    /// Method call
    MethodCall {
        receiver: Box<Expr>,
        method: String,
        arguments: Vec<Expr>,
    },

    /// Binary operation
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    /// Unary operation
    Unary {
        op: UnOp,
        operand: Box<Expr>,
    },

    /// If expression
    If {
        condition: Box<Expr>,
        then_block: Block,
        else_expr: Option<Box<Expr>>,
    },

    /// Match expression
    Match {
        value: Box<Expr>,
        arms: Vec<MatchArm>,
    },

    /// Loop expression
    Loop(Box<Block>),

    /// While expression
    While {
        condition: Box<Expr>,
        body: Box<Block>,
    },

    /// For expression
    For {
        pattern: Pattern,
        iterator: Box<Expr>,
        body: Box<Block>,
    },

    /// Break expression
    Break {
        label: Option<String>,
        value: Option<Box<Expr>>,
    },

    /// Continue expression
    Continue {
        label: Option<String>,
    },

    /// Return expression
    Return(Option<Box<Expr>>),

    /// Assignment
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
    },

    /// Compound assignment
    CompoundAssign {
        target: Box<Expr>,
        op: BinOp,
        value: Box<Expr>,
    },

    /// Array literal
    Array(Vec<Expr>),

    /// Tuple literal
    Tuple(Vec<Expr>),

    /// Struct literal
    Struct {
        name: String,
        fields: Vec<(String, Expr)>,
    },

    /// Field access
    Field {
        object: Box<Expr>,
        field: String,
    },

    /// Tuple field access
    TupleField {
        object: Box<Expr>,
        index: usize,
    },

    /// Index access
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    /// Range expression
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
    },

    /// Closure
    Closure {
        params: Vec<(String, Option<Type>)>,
        body: Box<Expr>,
    },

    /// Async block
    Async {
        body: Box<Expr>,
    },

    /// Await expression
    Await(Box<Expr>),

    /// Cast expression
    Cast {
        expr: Box<Expr>,
        target_type: Type,
    },

    /// Reference
    Ref {
        mutable: bool,
        expr: Box<Expr>,
    },

    /// Grouped expression
    Grouped(Box<Expr>),
}

impl ExprKind {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        match self {
            ExprKind::Literal(_) => 1,
            ExprKind::Identifier(_) | ExprKind::Path(_) => 1,
            ExprKind::Block(b) => b.node_count(),
            ExprKind::Call { function, arguments } => {
                1 + function.node_count() + arguments.iter().map(|e| e.node_count()).sum::<usize>()
            }
            ExprKind::MethodCall { receiver, arguments, .. } => {
                1 + receiver.node_count() + arguments.iter().map(|e| e.node_count()).sum::<usize>()
            }
            ExprKind::Binary { lhs, rhs, .. } => 1 + lhs.node_count() + rhs.node_count(),
            ExprKind::Unary { operand, .. } => 1 + operand.node_count(),
            ExprKind::If { condition, then_block, else_expr } => {
                1 + condition.node_count() + then_block.node_count()
                    + else_expr.as_ref().map(|e| e.node_count()).unwrap_or(0)
            }
            ExprKind::Match { value, arms } => {
                1 + value.node_count() + arms.iter().map(|a| a.node_count()).sum::<usize>()
            }
            ExprKind::Loop(b) | ExprKind::While { body: b, .. } => 1 + b.node_count(),
            ExprKind::For { iterator, body, .. } => {
                1 + iterator.node_count() + body.node_count()
            }
            ExprKind::Break { value, .. } => {
                1 + value.as_ref().map(|e| e.node_count()).unwrap_or(0)
            }
            ExprKind::Continue { .. } | ExprKind::Return(None) => 1,
            ExprKind::Return(Some(e)) => 1 + e.node_count(),
            ExprKind::Assign { target, value } | ExprKind::CompoundAssign { target, value, .. } => {
                1 + target.node_count() + value.node_count()
            }
            ExprKind::Array(exprs) | ExprKind::Tuple(exprs) => {
                1 + exprs.iter().map(|e| e.node_count()).sum::<usize>()
            }
            ExprKind::Struct { fields, .. } => {
                1 + fields.iter().map(|(_, e)| e.node_count()).sum::<usize>()
            }
            ExprKind::Field { object, .. }
            | ExprKind::TupleField { object, .. }
            | ExprKind::Index { object, .. }
            | ExprKind::Range { start: Some(object), .. }
            | ExprKind::Range { end: Some(object), .. } => 1 + object.node_count(),
            ExprKind::Range { .. } => 1,
            ExprKind::Closure { params, body } => {
                1 + params.len() + body.node_count()
            }
            ExprKind::Async { body } | ExprKind::Await(body) => 1 + body.node_count(),
            ExprKind::Cast { expr, .. } => 1 + expr.node_count(),
            ExprKind::Ref { expr, .. } | ExprKind::Grouped(expr) => 1 + expr.node_count(),
        }
    }
}

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    /// Power (not in Rust but in Python)
    Pow,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,

    // Logical
    And,
    Or,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnOp {
    Neg,  // -x
    Not,  // !x
    BitNot, // ~x
    Deref, // *x
}

/// Literal value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// String
    String(String),
    /// Character
    Char(char),
    /// Boolean
    Bool(bool),
    /// Unit
    Unit,
}

// ============================================================================
// Block
// ============================================================================

/// Block expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub statements: Vec<Stmt>,
    pub tail_expr: Option<Box<Expr>>,
}

impl Block {
    /// Create a new block
    pub fn new(statements: Vec<Stmt>, tail_expr: Option<Expr>) -> Self {
        Self {
            statements,
            tail_expr: tail_expr.map(Box::new),
        }
    }

    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.statements.iter().map(|s| s.node_count()).sum::<usize>()
            + self.tail_expr.as_ref().map(|e| e.node_count()).unwrap_or(0)
    }
}

// ============================================================================
// Type Extensions
// ============================================================================

impl Type {
    /// Count nodes in type
    pub fn node_count(&self) -> usize {
        match self {
            Type::Function(f) => {
                1 + f.parameters.iter().map(|t| t.node_count()).sum::<usize>()
                    + f.return_type.node_count()
            }
            Type::Tuple(types) => 1 + types.iter().map(|t| t.node_count()).sum::<usize>(),
            Type::Array(t, _) => 1 + t.node_count(),
            Type::Ref(t) | Type::MutRef(t) | Type::Ptr(t) => 1 + t.node_count(),
            Type::Struct(s) => 1 + s.fields.iter().map(|(_, t)| t.node_count()).sum::<usize>(),
            Type::Enum(e) => {
                1 + e.variants.iter().map(|v| v.fields.iter().map(|t| t.node_count()).sum::<usize>()).sum::<usize>()
            }
            _ => 1,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_expr() {
        let expr = Expr::new(
            ExprKind::Literal(Literal::Int(42)),
            SourceSpan {
                start: SourceLocation { line: 1, column: 1, offset: 0 },
                end: SourceLocation { line: 1, column: 3, offset: 2 },
            },
        );
        assert_eq!(expr.node_count(), 1);
    }

    #[test]
    fn test_block_node_count() {
        let block = Block::new(
            vec![Stmt::Let {
                pattern: Pattern::Wildcard(SourceSpan {
                    start: SourceLocation { line: 1, column: 5, offset: 4 },
                    end: SourceLocation { line: 1, column: 6, offset: 5 },
                }),
                let_type: None,
                initializer: Some(Expr::new(
                    ExprKind::Literal(Literal::Int(42)),
                    SourceSpan {
                        start: SourceLocation { line: 1, column: 9, offset: 8 },
                        end: SourceLocation { line: 1, column: 11, offset: 10 },
                    },
                )),
                location: SourceSpan {
                    start: SourceLocation { line: 1, column: 1, offset: 0 },
                    end: SourceLocation { line: 1, column: 11, offset: 10 },
                },
            }],
            None,
        );
        assert_eq!(block.node_count(), 2);
    }

    #[test]
    fn test_pattern_node_count() {
        let tuple_pattern = Pattern::Tuple(
            vec![
                Pattern::Variable {
                    name: "a".to_string(),
                    mutable: false,
                    location: SourceSpan {
                        start: SourceLocation { line: 1, column: 2, offset: 1 },
                        end: SourceLocation { line: 1, column: 3, offset: 2 },
                    },
                },
                Pattern::Wildcard(SourceSpan {
                    start: SourceLocation { line: 1, column: 5, offset: 4 },
                    end: SourceLocation { line: 1, column: 6, offset: 5 },
                }),
            ],
            SourceSpan {
                start: SourceLocation { line: 1, column: 1, offset: 0 },
                end: SourceLocation { line: 1, column: 7, offset: 6 },
            },
        );
        assert_eq!(tuple_pattern.node_count(), 3);
    }
}
