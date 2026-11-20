//! TUMC AST - Abstract Syntax Tree
//!
//! This module defines the abstract syntax tree representation for
//! the TUMC compiler intermediate representation.
//!
//! # Node Types
//!
//! - [`Program`] - Root node containing all top-level items
//! - [`Item`] - Top-level declarations (functions, structs, etc.)
//! - [`Stmt`] / [`StmtKind`] - Statements (let, if, while, etc.)
//! - [`Expr`] / [`ExprKind`] - Expressions (literals, binary ops, etc.)
//! - [`Pattern`] - Pattern matching for destructuring
//! - [`AnnType`] - Declared (annotated) types used by the type checker
//!
//! # Design
//!
//! The AST is the single source of truth shared by the parser, the type
//! checker, the optimizer and every code generator. It therefore uses a
//! *structural* representation:
//!
//! - Every expression is an [`Expr`] wrapper carrying an [`ExprKind`], a
//!   [`Span`] and a list of [`Annotation`]s.
//! - Every statement is a [`Stmt`] wrapper carrying a [`StmtKind`] and a
//!   [`Span`].
//! - Blocks are plain `Vec<Stmt>` (there is no separate `Block` node in the
//!   core pipeline).
//! - Declared types are expressed with [`AnnType`] (the "annotated type"
//!   lattice understood by [`crate::tumc_compiler::type_checker`]).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub use crate::tumc_compiler::{FunctionType, SourceLocation, Type};

/// Convenience alias for the source span type used throughout the AST.
pub type Span = SourceSpan;

// ============================================================================
// Program
// ============================================================================

/// Root node of the AST
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Program {
    /// Items (top-level declarations)
    pub items: Vec<Item>,

    /// Source spans for each item, keyed by item index
    pub spans: HashMap<usize, SourceSpan>,
}

/// Source span: the region of source code covered by a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

impl SourceSpan {
    /// Create a span covering a single location.
    pub fn new(start: SourceLocation, end: SourceLocation) -> Self {
        Self { start, end }
    }

    /// Create a zero-length span at the given location.
    pub fn at(loc: SourceLocation) -> Self {
        Self {
            start: loc,
            end: loc,
        }
    }

    /// Merge two spans into the smallest span covering both.
    pub fn merge(&self, other: &SourceSpan) -> SourceSpan {
        SourceSpan {
            start: self.start,
            end: other.end,
        }
    }
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    Impl(ImplBlock),

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
            Item::Impl(i) => &i.type_name,
            Item::TypeAlias(t) => &t.name,
            Item::Constant(c) => &c.name,
            Item::Module(m) => &m.name,
        }
    }

    /// Source span of the item
    pub fn span(&self) -> SourceSpan {
        match self {
            Item::Function(f) => f.span,
            Item::Struct(s) => s.span,
            Item::Enum(e) => e.span,
            Item::Trait(t) => t.span,
            Item::Impl(i) => i.span,
            Item::TypeAlias(t) => t.span,
            Item::Constant(c) => c.span,
            Item::Module(m) => m.span,
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
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Function {
    /// Function name
    pub name: String,

    /// Parameters
    pub params: Vec<FunctionParam>,

    /// Declared return type
    pub return_type: AnnType,

    /// Function body
    pub body: Vec<Stmt>,

    /// Location
    pub span: Span,

    /// Documentation comment attached to the declaration
    pub doc: Option<String>,

    /// Visibility
    pub visibility: Visibility,

    /// Is async
    pub is_async: bool,

    /// Is the function a ternary expression (its body is an `if` expression)
    /// rather than a statement block.
    pub ternary: bool,

    /// Generic parameters
    pub generics: Vec<GenericParam>,
}

impl Function {
    /// Is this function a ternary (expression-bodied) function?
    pub fn is_ternary(&self) -> bool {
        self.ternary
    }

    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len()
            + self.params.iter().map(|p| p.node_count()).sum::<usize>()
            + self.body.iter().map(|s| s.node_count()).sum::<usize>()
    }
}

/// Function parameter
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct FunctionParam {
    /// Parameter name
    pub name: String,

    /// Declared parameter type
    pub ann_type: AnnType,
}

impl FunctionParam {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1
    }
}

/// Generic parameter
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<TypeBound>,
}

/// Type bound
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeBound {
    /// Implements a trait
    Trait(String),
    /// Lifetime parameter
    Lifetime(String),
}

/// Visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Visibility {
    #[default]
    Public,
    Private,
    Crate,
}

/// Struct declaration
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Struct {
    pub name: String,
    pub ty_params: Vec<String>,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<Field>,
    pub span: Span,
    pub doc: Option<String>,
    pub visibility: Visibility,
}

impl Struct {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.ty_params.len()
            + self.generics.len()
            + self.fields.iter().map(|f| f.node_count()).sum::<usize>()
    }
}

/// Field declaration
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ann_type: AnnType,
    pub span: Span,
    pub doc: Option<String>,
    pub visibility: Visibility,
}

impl Field {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1
    }
}

/// Enum declaration
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub ty_params: Vec<String>,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
    pub doc: Option<String>,
    pub visibility: Visibility,
}

impl Enum {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.ty_params.len() + self.generics.len() + self.variants.len()
    }
}

/// Enum variant
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    /// Optional payload type of the variant
    pub fields: Option<AnnType>,
    pub span: Span,
    pub doc: Option<String>,
}

/// Trait declaration
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub items: Vec<TraitItem>,
    pub span: Span,
    pub doc: Option<String>,
}

impl Trait {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.generics.len() + self.items.len()
    }
}

/// Trait item
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum TraitItem {
    Function { name: String, sig: FunctionSig },
    Type { name: String },
}

/// Function signature (without body)
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct FunctionSig {
    pub generics: Vec<GenericParam>,
    pub parameters: Vec<(String, AnnType)>,
    pub return_type: Option<AnnType>,
}

/// Impl block
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImplBlock {
    /// Trait being implemented, if any
    pub trait_name: Option<String>,
    /// Type the impl block targets
    pub type_name: String,
    /// Generic parameters of the impl block
    pub ty_params: Vec<String>,
    /// Functions declared in the impl block
    pub functions: Vec<Function>,
    /// Constants declared in the impl block
    pub constants: Vec<Constant>,
    /// Location
    pub span: Span,
}

impl ImplBlock {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.ty_params.len()
            + self.functions.iter().map(|f| f.node_count()).sum::<usize>()
            + self.constants.iter().map(|c| c.node_count()).sum::<usize>()
    }
}

/// Impl item
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ImplItem {
    Function(Function),
    Constant(Constant),
}

/// Type alias
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub ty_params: Vec<String>,
    pub generics: Vec<GenericParam>,
    /// Aliased type
    pub ann_type: AnnType,
    pub span: Span,
    pub doc: Option<String>,
}

impl TypeAlias {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.ty_params.len() + self.generics.len()
    }
}

/// Constant declaration
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Constant {
    pub name: String,
    /// Declared type of the constant
    pub ann_type: AnnType,
    /// Initialiser expression
    pub value: Expr,
    pub span: Span,
    pub doc: Option<String>,
    pub visibility: Visibility,
}

impl Constant {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        2 + self.value.node_count()
    }
}

/// Module declaration
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub items: Vec<Item>,
    pub span: Span,
    pub doc: Option<String>,
    pub visibility: Visibility,
}

impl Module {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.items.iter().map(|i| i.node_count()).sum::<usize>()
    }
}

// ============================================================================
// Attributes
// ============================================================================

/// A raw attribute as captured by the parser.
///
/// Attributes are stored as a flat token stream; higher level consumers
/// (such as the derive system) interpret the tokens.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Attribute {
    /// Tokens making up the attribute
    pub tokens: Vec<Token>,
}

impl Attribute {
    /// Create an attribute from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    /// Collect the identifiers appearing in the attribute
    pub fn identifiers(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .filter_map(|t| match &t.kind {
                TokenKind::Identifier(name) => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }
}

/// A token captured inside an [`Attribute`].
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Token kinds understood inside attributes.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub enum TokenKind {
    #[default]
    EOF,
    Identifier(String),
    String(String),
    Integer(i64),
    Punct(char),
}

// ============================================================================
// Statements
// ============================================================================

/// Statement wrapper: a [`StmtKind`] together with its source span.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Stmt {
    /// Statement kind
    pub kind: StmtKind,
    /// Location
    pub span: Span,
}

impl Stmt {
    /// Create a new statement
    pub fn new(kind: StmtKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Create a statement at the default (unknown) location
    pub fn new_at(kind: StmtKind) -> Self {
        Self {
            kind,
            span: Span::default(),
        }
    }

    /// Create an expression statement
    pub fn new_expr(expr: Expr) -> Self {
        Self {
            kind: StmtKind::Expr(expr),
            span: Span::default(),
        }
    }

    /// Create a return statement
    pub fn new_return(value: Option<Expr>) -> Self {
        Self {
            kind: StmtKind::Return(value),
            span: Span::default(),
        }
    }

    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.kind.node_count()
    }
}

/// Statement kinds.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum StmtKind {
    /// Expression statement
    Expr(Expr),

    /// Let binding: `let <pattern>[: <type>] = <value>;` followed by `body`
    Let(Pattern, Option<AnnType>, Option<Expr>, Vec<Stmt>),

    /// If statement
    If(Box<Expr>, Vec<Stmt>, Option<(Pattern, Vec<Stmt>)>),

    /// While loop
    While(Box<Expr>, Vec<Stmt>),

    /// For loop: `for <var> in <iterable> { .. }`
    For(String, Box<Expr>, Vec<Stmt>),

    /// Loop (infinite loop)
    Loop(Vec<Stmt>),

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Return statement
    Return(Option<Expr>),

    /// Assert statement
    Assert(Expr),

    /// Assignment
    Assignment(Box<Expr>, Box<Expr>),

    /// Compound assignment (`+=`, `-=`, ...)
    CompoundAssignment(Box<Expr>, BinOp, Box<Expr>),

    /// Nested block
    Block(Vec<Stmt>),
}

impl Default for StmtKind {
    /// A statement defaults to an expression statement with no effect.
    fn default() -> Self {
        StmtKind::Expr(Expr::default())
    }
}

impl StmtKind {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        match self {
            StmtKind::Expr(e) => 1 + e.node_count(),
            StmtKind::Let(p, _, v, body) => {
                1 + p.node_count()
                    + v.as_ref().map(|e| e.node_count()).unwrap_or(0)
                    + body.iter().map(|s| s.node_count()).sum::<usize>()
            }
            StmtKind::If(c, then_b, else_b) => {
                1 + c.node_count()
                    + then_b.iter().map(|s| s.node_count()).sum::<usize>()
                    + else_b
                        .as_ref()
                        .map(|(_, b)| b.iter().map(|s| s.node_count()).sum::<usize>())
                        .unwrap_or(0)
            }
            StmtKind::While(c, body) => {
                1 + c.node_count() + body.iter().map(|s| s.node_count()).sum::<usize>()
            }
            StmtKind::For(_, it, body) => {
                1 + it.node_count() + body.iter().map(|s| s.node_count()).sum::<usize>()
            }
            StmtKind::Loop(body) | StmtKind::Block(body) => {
                1 + body.iter().map(|s| s.node_count()).sum::<usize>()
            }
            StmtKind::Break | StmtKind::Continue => 1,
            StmtKind::Return(v) => v.as_ref().map(|e| 1 + e.node_count()).unwrap_or(1),
            StmtKind::Assert(e) => 1 + e.node_count(),
            StmtKind::Assignment(l, r) => 1 + l.node_count() + r.node_count(),
            StmtKind::CompoundAssignment(l, _, r) => 1 + l.node_count() + r.node_count(),
        }
    }
}

// ============================================================================
// Patterns
// ============================================================================

/// Pattern matching node.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Pattern {
    /// Wildcard pattern (`_`)
    Wildcard,

    /// Literal pattern
    Literal(Literal),

    /// Range pattern
    Range(Literal, Literal),

    /// Binding pattern (variable)
    Binding(String),

    /// Constructor pattern: `Name(a, b)`
    Constructor { name: String, args: Vec<Pattern> },

    /// Tuple pattern
    Tuple(Vec<Pattern>),

    /// Slice / array pattern
    Slice(Vec<Pattern>),

    /// Or pattern
    Or(Vec<Pattern>),

    /// Pattern with binding: `p @ name`
    As(Box<Pattern>, String),
    /// Reference pattern: `&p`
    Ref(Box<Pattern>),
    /// Mutable reference pattern: `&mut p`
    MutRef(Box<Pattern>),
}

impl Pattern {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        match self {
            Pattern::Wildcard => 1,
            Pattern::Literal(_) => 1,
            Pattern::Range(_, _) => 1,
            Pattern::Binding(_) => 1,
            Pattern::Constructor { args, .. } => {
                1 + args.iter().map(|p| p.node_count()).sum::<usize>()
            }
            Pattern::Tuple(pats) | Pattern::Slice(pats) | Pattern::Or(pats) => {
                1 + pats.iter().map(|p| p.node_count()).sum::<usize>()
            }
            Pattern::As(p, _) => 1 + p.node_count(),
            Pattern::Ref(p) | Pattern::MutRef(p) => 1 + p.node_count(),
        }
    }

    /// Name bound by this pattern, if any.
    pub fn binding_name(&self) -> Option<&str> {
        match self {
            Pattern::Binding(name) => Some(name.as_str()),
            Pattern::As(_, name) => Some(name.as_str()),
            _ => None,
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Wildcard => write!(f, "_"),
            Pattern::Literal(lit) => write!(f, "{}", lit),
            Pattern::Range(start, end) => write!(f, "{}..{}", start, end),
            Pattern::Binding(name) => write!(f, "{}", name),
            Pattern::Constructor { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Pattern::Tuple(pats) => {
                write!(f, "(")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ")")
            }
            Pattern::Slice(pats) => {
                write!(f, "[")?;
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, "]")
            }
            Pattern::Or(pats) => {
                for (i, p) in pats.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", p)?;
                }
                Ok(())
            }
            Pattern::As(pat, name) => write!(f, "{} @ {}", pat, name),
            Pattern::Ref(pat) => write!(f, "&{}", pat),
            Pattern::MutRef(pat) => write!(f, "&mut {}", pat),
        }
    }
}

// ============================================================================
// Expressions
// ============================================================================

/// Expression wrapper: an [`ExprKind`] with its span and annotations.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Expr {
    /// Expression kind
    pub kind: ExprKind,

    /// Location
    pub span: Span,

    /// Attached annotations (doc comments, attributes, ...)
    pub annotations: Vec<Annotation>,
}

impl Default for ExprKind {
    /// An expression defaults to the unit literal.
    fn default() -> Self {
        ExprKind::Literal(Literal::Unit)
    }
}

impl Expr {
    /// Create a new expression
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self {
            kind,
            span,
            annotations: Vec::new(),
        }
    }

    /// Create a new expression at the default location
    pub fn new_at(kind: ExprKind) -> Self {
        Self::new(kind, Span::default())
    }

    /// Create an identifier expression
    pub fn new_identifier(name: impl Into<String>) -> Self {
        Self::new_at(ExprKind::Identifier(name.into()))
    }

    /// Create an identifier expression at the given location
    pub fn new_identifier_at(name: impl Into<String>, span: Span) -> Self {
        Self::new(ExprKind::Identifier(name.into()), span)
    }

    /// The literal this expression evaluates to, if it is a literal.
    pub fn to_literal(&self) -> Option<&Literal> {
        match &self.kind {
            ExprKind::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    /// Create a literal expression
    pub fn new_literal(lit: Literal, span: Span) -> Self {
        Self::new(ExprKind::Literal(lit), span)
    }

    /// Create a binary expression
    pub fn new_binary(op: BinOp, lhs: Expr, rhs: Expr, span: Span) -> Self {
        Self::new(
            ExprKind::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            span,
        )
    }

    /// Create a unary expression
    pub fn new_unary(op: UnOp, operand: Expr, span: Span) -> Self {
        Self::new(
            ExprKind::Unary {
                op,
                operand: Box::new(operand),
            },
            span,
        )
    }

    /// Create a method call expression
    pub fn new_method_call(object: Expr, method: &str, arguments: Vec<Expr>, span: Span) -> Self {
        Self::new(
            ExprKind::MethodCall {
                object: Box::new(object),
                method: method.to_string(),
                arguments,
            },
            span,
        )
    }

    /// Create a call expression
    pub fn new_call(function: Expr, arguments: Vec<Expr>, span: Span) -> Self {
        Self::new(
            ExprKind::Call {
                function: Box::new(function),
                arguments,
            },
            span,
        )
    }

    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.kind.node_count()
    }
}

/// Annotation attached to an expression or item.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Annotation {
    /// Kind of annotation
    pub kind: AnnotationKind,
    /// Payload of the annotation
    pub value: String,
    /// Location
    pub span: Span,
}

/// Kinds of annotation.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationKind {
    /// Documentation comment
    #[default]
    Doc,
    /// Inline annotation
    Inline,
    /// Derive annotation
    Derive,
    /// Custom annotation
    Custom(String),
}

/// Expression kinds.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExprKind {
    /// Literal value
    Literal(Literal),

    /// Identifier
    Identifier(String),

    /// Path (`a::b::c`)
    Path(Vec<String>),

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

    /// Function call
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },

    /// Method call
    MethodCall {
        object: Box<Expr>,
        method: String,
        arguments: Vec<Expr>,
    },

    /// If expression
    If {
        condition: Box<Expr>,
        then_block: Vec<(Pattern, Vec<Stmt>)>,
        else_expr: Option<(Pattern, Vec<Stmt>)>,
    },

    /// Match expression
    Match {
        discriminant: Box<Expr>,
        arms: Vec<MatchArm>,
    },

    /// Block expression
    Block(Vec<Stmt>),

    /// Tuple literal
    Tuple(Vec<Expr>),

    /// Array literal
    Array(Vec<Expr>),

    /// Dictionary literal
    Dict(Vec<(Expr, Expr)>),

    /// Field access
    FieldAccess(Box<Expr>, String),

    /// Index access
    Index(Box<Expr>, Box<Expr>),

    /// Range expression: `start..end` (the flag selects an inclusive range)
    Range(Box<Expr>, Box<Expr>, bool),

    /// Closure / lambda
    Closure(Vec<FunctionParam>, Vec<Stmt>, Option<AnnType>),

    /// Cast expression
    Cast(Box<Expr>, Box<AnnType>),

    /// Await expression
    Await(Box<Expr>),

    /// Try expression (`?`)
    Try(Box<Expr>),

    /// Parenthesised expression
    Parenthesis(Box<Expr>),

    /// Assignment expression (`target = value`)
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
    },

    /// Compound assignment expression (`target op= value`)
    CompoundAssign {
        target: Box<Expr>,
        op: BinOp,
        value: Box<Expr>,
    },

    /// Loop expression
    Loop(Vec<Stmt>),

    /// While loop expression
    While {
        condition: Box<Expr>,
        body: Vec<Stmt>,
    },

    /// Return expression
    Return(Option<Box<Expr>>),

    /// Break expression
    Break {
        label: Option<String>,
        value: Option<Box<Expr>>,
    },

    /// Continue expression
    Continue { label: Option<String> },

    /// Async block
    Async { body: Box<Expr> },

    /// Struct literal
    Struct {
        name: String,
        fields: Vec<(String, Expr)>,
    },

    /// Tuple field access (`tuple.0`)
    TupleField {
        object: Box<Expr>,
        index: usize,
    },

    /// Reference expression (`&expr` / `&mut expr`)
    Ref {
        mutable: bool,
        expr: Box<Expr>,
    },
}

impl ExprKind {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        match self {
            ExprKind::Literal(_) => 1,
            ExprKind::Identifier(_) | ExprKind::Path(_) => 1,
            ExprKind::Binary { lhs, rhs, .. } => 1 + lhs.node_count() + rhs.node_count(),
            ExprKind::Unary { operand, .. } => 1 + operand.node_count(),
            ExprKind::Call { function, arguments } => {
                1 + function.node_count()
                    + arguments.iter().map(|e| e.node_count()).sum::<usize>()
            }
            ExprKind::MethodCall { object, arguments, .. } => {
                1 + object.node_count()
                    + arguments.iter().map(|e| e.node_count()).sum::<usize>()
            }
            ExprKind::If {
                condition,
                then_block,
                else_expr,
            } => {
                1 + condition.node_count()
                    + then_block
                        .iter()
                        .map(|(_, body)| body.iter().map(|s| s.node_count()).sum::<usize>())
                        .sum::<usize>()
                    + else_expr
                        .as_ref()
                        .map(|(_, body)| body.iter().map(|s| s.node_count()).sum::<usize>())
                        .unwrap_or(0)
            }
            ExprKind::Match { discriminant, arms } => {
                1 + discriminant.node_count()
                    + arms.iter().map(|a| a.node_count()).sum::<usize>()
            }
            ExprKind::Block(stmts) => {
                1 + stmts.iter().map(|s| s.node_count()).sum::<usize>()
            }
            ExprKind::Tuple(elems) | ExprKind::Array(elems) => {
                1 + elems.iter().map(|e| e.node_count()).sum::<usize>()
            }
            ExprKind::Dict(pairs) => {
                1 + pairs
                    .iter()
                    .map(|(k, v)| k.node_count() + v.node_count())
                    .sum::<usize>()
            }
            ExprKind::FieldAccess(base, _) => 1 + base.node_count(),
            ExprKind::Index(base, index) => 1 + base.node_count() + index.node_count(),
            ExprKind::Range(start, end, _) => 1 + start.node_count() + end.node_count(),
            ExprKind::Closure(params, body, _) => {
                1 + params.len() + body.iter().map(|s| s.node_count()).sum::<usize>()
            }
            ExprKind::Cast(expr, _) => 1 + expr.node_count(),
            ExprKind::Await(expr) | ExprKind::Try(expr) | ExprKind::Parenthesis(expr) => {
                1 + expr.node_count()
            }
            ExprKind::Assign { target, value } => 1 + target.node_count() + value.node_count(),
            ExprKind::CompoundAssign { target, value, .. } => {
                1 + target.node_count() + value.node_count()
            }
            ExprKind::Loop(stmts) => 1 + stmts.iter().map(|s| s.node_count()).sum::<usize>(),
            ExprKind::While { condition, body } => {
                1 + condition.node_count() + body.iter().map(|s| s.node_count()).sum::<usize>()
            }
            ExprKind::Return(value) => value.as_ref().map(|e| 1 + e.node_count()).unwrap_or(1),
            ExprKind::Break { value, .. } => value.as_ref().map(|e| 1 + e.node_count()).unwrap_or(1),
            ExprKind::Continue { .. } => 1,
            ExprKind::Async { body } => 1 + body.node_count(),
            ExprKind::Struct { fields, .. } => {
                1 + fields.iter().map(|(_, v)| v.node_count()).sum::<usize>()
            }
            ExprKind::TupleField { object, .. } => 1 + object.node_count(),
            ExprKind::Ref { expr, .. } => 1 + expr.node_count(),
        }
    }
}

/// Match arm
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MatchArm {
    /// Pattern matched against the discriminant
    pub pattern: Pattern,
    /// Optional guard expression
    pub guard: Option<Expr>,
    /// Body of the arm
    pub body: Expr,
}

impl MatchArm {
    /// Count nodes
    pub fn node_count(&self) -> usize {
        1 + self.pattern.node_count()
            + self.guard.as_ref().map(|e| e.node_count()).unwrap_or(0)
            + self.body.node_count()
    }
}

// ============================================================================
// Operators
// ============================================================================

/// Binary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,

    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Logical
    And,
    Or,

    /// Assignment
    Assign,

    /// Half-open range
    Range,

    /// Inclusive range
    RangeInclusive,

    /// Pipe operator
    Pipe,

    /// Exponentiation
    Pow,
}

/// A sequence of statements forming a block scope.
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Block {
    /// Statements in the block
    pub statements: Vec<Stmt>,
    /// Optional trailing expression acting as the block's value
    pub tail_expr: Option<Box<Expr>>,
}

impl Block {
    /// Create a new block from statements and an optional trailing expression.
    pub fn new(statements: Vec<Stmt>, tail_expr: Option<Expr>) -> Self {
        Self {
            statements,
            tail_expr: tail_expr.map(Box::new),
        }
    }

    /// Number of AST nodes contained in this block.
    pub fn node_count(&self) -> usize {
        1 + self.statements.iter().map(|s| s.node_count()).sum::<usize>()
            + self.tail_expr.as_ref().map(|e| e.node_count()).unwrap_or(0)
    }
}

/// Unary operator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnOp {
    Neg,    // -x
    Not,    // !x
    BitNot, // ~x
    Deref,  // *x
}

// ============================================================================
// Literals
// ============================================================================

/// Literal value
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// The null value.
    Null,

    /// Unit literal `()`
    Unit,
    /// Boolean
    Bool(bool),
    /// Integer
    Int(i64),
    /// Float
    Float(f64),
    /// Character
    Char(char),
    /// String
    String(String),
    /// Byte
    Byte(u8),
    /// Raw string
    RawString(String),
    /// Byte string
    ByteString(Vec<u8>),
    /// Verbatim token sequence
    Verbatim(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Unit => write!(f, "()"),
            Literal::Null => write!(f, "null"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(x) => write!(f, "{}", x),
            Literal::Char(c) => write!(f, "'{}'", c),
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Byte(b) => write!(f, "{}u8", b),
            Literal::RawString(s) => write!(f, "r\"{}\"", s),
            Literal::ByteString(b) => write!(f, "b\"{:?}\"", b),
            Literal::Verbatim(s) => write!(f, "{}", s),
        }
    }
}

// ============================================================================
// Annotated (declared) types
// ============================================================================

/// Declared type annotation.
///
/// This is the type lattice understood by the type checker. It is
/// deliberately kept separate from the *inferred* [`Type`] used during
/// inference so that annotations remain cheap to clone and compare.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AnnType {
    /// Unit type `()`
    #[default]
    Unit,
    /// Boolean
    Bool,
    /// Integer
    Int,
    /// Float
    Float,
    /// Character
    Char,
    /// String
    String,
    /// Error type (used during recovery)
    Error,
    /// Named type (`Foo`, `Vec<T>`, ...)
    Named(String),
    /// Shared reference
    Ref(Box<AnnType>),
    /// Mutable reference
    MutRef(Box<AnnType>),
    /// Raw pointer
    Ptr(Box<AnnType>),
    /// Array / slice
    Array(Box<AnnType>),
    /// Function type
    Func(Vec<AnnType>, Box<AnnType>),
    /// Tuple type
    Tuple(Vec<AnnType>),
    /// Optional type
    Optional(Box<AnnType>),
    /// Result type
    Result(Box<AnnType>, Box<AnnType>),
}

impl AnnType {
    /// Convert a resolved [`Type`] into its annotation counterpart.
    pub fn from_type(ty: &Type) -> AnnType {
        match ty {
            Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::I128
            | Type::Isize
            | Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::U128
            | Type::Usize => AnnType::Int,
            Type::F32 | Type::F64 => AnnType::Float,
            Type::Bool => AnnType::Bool,
            Type::Char => AnnType::Char,
            Type::String => AnnType::String,
            Type::Unit => AnnType::Unit,
            Type::Function(f) => AnnType::Func(
                f.parameters.iter().map(AnnType::from_type).collect(),
                Box::new(AnnType::from_type(&f.return_type)),
            ),
            Type::Tuple(ts) => AnnType::Tuple(ts.iter().map(AnnType::from_type).collect()),
            Type::Array(t, _) => AnnType::Array(Box::new(AnnType::from_type(t))),
            Type::Ref(t) => AnnType::Ref(Box::new(AnnType::from_type(t))),
            Type::MutRef(t) => AnnType::MutRef(Box::new(AnnType::from_type(t))),
            Type::Ptr(t) => AnnType::Ptr(Box::new(AnnType::from_type(t))),
            Type::Struct(s) => AnnType::Named(s.name.clone()),
            Type::Enum(e) => AnnType::Named(e.name.clone()),
            Type::Generic(name) => AnnType::Named(name.clone()),
            Type::Var(_) | Type::Unknown => AnnType::Error,
        }
    }
}

impl fmt::Display for AnnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnnType::Unit => write!(f, "()"),
            AnnType::Bool => write!(f, "bool"),
            AnnType::Int => write!(f, "int"),
            AnnType::Float => write!(f, "float"),
            AnnType::Char => write!(f, "char"),
            AnnType::String => write!(f, "string"),
            AnnType::Error => write!(f, "<error>"),
            AnnType::Named(name) => write!(f, "{}", name),
            AnnType::Ref(inner) => write!(f, "&{}", inner),
            AnnType::MutRef(inner) => write!(f, "&mut {}", inner),
            AnnType::Ptr(inner) => write!(f, "*{}", inner),
            AnnType::Array(inner) => write!(f, "[{}]", inner),
            AnnType::Func(params, ret) => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            AnnType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            AnnType::Optional(inner) => write!(f, "Option<{}>", inner),
            AnnType::Result(ok, err) => write!(f, "Result<{}, {}>", ok, err),
        }
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

    fn span() -> SourceSpan {
        SourceSpan {
            start: SourceLocation {
                line: 1,
                column: 1,
                offset: 0,
            },
            end: SourceLocation {
                line: 1,
                column: 3,
                offset: 2,
            },
        }
    }

    #[test]
    fn test_literal_expr() {
        let expr = Expr::new(ExprKind::Literal(Literal::Int(42)), span());
        assert_eq!(expr.node_count(), 1);
    }

    #[test]
    fn test_block_node_count() {
        let stmt = Stmt::new(
            StmtKind::Let(
                Pattern::Wildcard,
                None,
                Some(Expr::new(ExprKind::Literal(Literal::Int(42)), span())),
                Vec::new(),
            ),
            span(),
        );
        assert_eq!(stmt.node_count(), 2);
    }

    #[test]
    fn test_pattern_node_count() {
        let tuple_pattern = Pattern::Tuple(vec![Pattern::Binding("a".to_string()), Pattern::Wildcard]);
        assert_eq!(tuple_pattern.node_count(), 3);
    }

    #[test]
    fn test_pattern_display() {
        assert_eq!(Pattern::Binding("x".to_string()).to_string(), "x");
        assert_eq!(Pattern::Wildcard.to_string(), "_");
    }

    #[test]
    fn test_ann_type_from_type() {
        assert_eq!(AnnType::from_type(&Type::I32), AnnType::Int);
        assert_eq!(AnnType::from_type(&Type::String), AnnType::String);
    }
}
