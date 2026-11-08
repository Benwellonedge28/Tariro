//! Frontend concrete syntax tree (CST) shared by the TUMC language frontends.
//!
//! The language frontends ([`crate::tumc_compiler::lang_go`],
//! [`crate::tumc_compiler::lang_python`],
//! [`crate::tumc_compiler::lang_typescript`]) parse source text into this tree.
//!
//! # Why a separate tree?
//!
//! The compiler IR lives in [`crate::tumc_compiler::ast`] and is deliberately
//! *structural*: it models the subset of Rust-like constructs the TUMC
//! compiler understands. Source languages carry a number of constructs that the
//! IR does not (yet) model — imports, exports, type declarations, `try`/`with`,
//! interfaces, namespaces, source-level operators such as `??`.
//!
//! Rather than dropping those constructs on the floor, the frontends keep them
//! in a syntax tree that mirrors the source. A lowering pass is responsible for
//! turning a [`Program`] into the canonical IR
//! [`crate::tumc_compiler::ast::Program`].
//!
//! # Operators
//!
//! [`BinaryOp`] and [`UnaryOp`] are *source-level* operator sets. They are
//! intentionally distinct from the IR operator sets in
//! [`crate::tumc_compiler::ast`]: the IR models the operators the backend can
//! emit, while these model every operator the frontends can parse (`??`, `is`,
//! `in`, `//`, `typeof`, `++`, ...).

use crate::tumc_compiler::ast::Literal;
use crate::tumc_compiler::Type;

// ============================================================================
// Source-level operators
// ============================================================================

/// A binary operator as it appears in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    FloorDiv,
    Pow,

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
    Is,
    In,

    // Logical
    And,
    Or,

    /// `??` — nullish coalescing.
    NullishCoalesce,
}

/// A unary operator as it appears in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    /// `+x`
    Plus,
    /// `-x`
    Negate,
    /// `!x`
    Not,
    /// `~x`
    BitNot,
    /// `*x`
    Deref,
    /// `&x`
    AddressOf,
    /// `typeof x`
    TypeOf,
    /// `void x`
    Void,
    /// `x!` (TypeScript non-null assertion)
    NonNull,
    /// `x++`
    Increment,
    /// `x--`
    Decrement,
    /// `delete x`
    Delete,
    /// `(T) x`
    Cast,
}

// ============================================================================
// Program
// ============================================================================

/// A parsed source file.
#[derive(Debug, Clone, Default)]
pub struct Program {
    /// Top-level statements and declarations, in source order.
    pub nodes: Vec<AstNode>,
}

/// A statement or declaration.
pub type Statement = AstNode;

// ============================================================================
// Declarations
// ============================================================================

/// A function, method or closure parameter.
#[derive(Debug, Clone, Default)]
pub struct Param {
    /// Parameter name.
    pub name: String,

    /// Declared type, when the source annotates one.
    pub param_type: Option<Type>,

    /// Default value, when the source provides one.
    pub default: Option<Expression>,

    /// `*args` / `...rest` style parameter.
    pub is_variadic: bool,

    /// `**kwargs` style parameter.
    pub is_kwargs: bool,
}

/// A function or method declaration.
#[derive(Debug, Clone, Default)]
pub struct Function {
    /// Function name.
    pub name: String,

    /// Declared parameters.
    pub params: Vec<Param>,

    /// Declared return type, when the source annotates one.
    pub return_type: Option<Type>,

    /// Function body.
    pub body: Vec<Statement>,

    /// Whether the function is asynchronous.
    pub is_async: bool,

    /// Whether the function is a generator.
    pub is_generator: bool,
}

/// A class declaration.
#[derive(Debug, Clone, Default)]
pub struct Class {
    /// Class name.
    pub name: String,

    /// Base classes, as written in the source.
    pub bases: Vec<Expression>,

    /// Class body.
    pub body: Vec<Statement>,

    /// Decorators applied to the class.
    pub decorators: Vec<String>,
}

// ============================================================================
// TypeScript type members
// ============================================================================

/// A member of an interface or type literal.
#[derive(Debug, Clone)]
pub enum TypeMember {
    /// `name: T`
    Property {
        name: String,
        type_annotation: Type,
        optional: bool,
        readonly: bool,
    },
    /// `name(...): T`
    Method {
        name: String,
        type_params: Vec<String>,
        params: Vec<(String, Type)>,
        return_type: Type,
    },
    /// `get name(): T`
    GetAccessor {
        name: String,
        type_annotation: Type,
    },
    /// `set name(v: T)`
    SetAccessor {
        name: String,
        param_type: Type,
    },
    /// `[key: K]: V`
    Index(IndexSignature),
    /// `(...) => T`
    CallSignature(CallSignature),
    /// `new (...) => T`
    ConstructSignature(ConstructSignature),
}

/// An index signature `[key: K]: V`.
#[derive(Debug, Clone)]
pub struct IndexSignature {
    /// Name of the index parameter.
    pub key: String,
    /// Type of the index parameter.
    pub key_type: Type,
    /// Type of the indexed value.
    pub value_type: Type,
}

/// A call signature `(...) => T`.
#[derive(Debug, Clone)]
pub struct CallSignature {
    /// Generic type parameters.
    pub type_params: Vec<String>,
    /// Declared parameters.
    pub params: Vec<(String, Type)>,
    /// Declared return type.
    pub return_type: Type,
}

/// A construct signature `new (...) => T`.
#[derive(Debug, Clone)]
pub struct ConstructSignature {
    /// Generic type parameters.
    pub type_params: Vec<String>,
    /// Declared parameters.
    pub params: Vec<(String, Type)>,
    /// Declared return type.
    pub return_type: Type,
}

/// A member of an enum declaration.
#[derive(Debug, Clone)]
pub struct EnumMember {
    /// Member name.
    pub name: String,
    /// Initialiser, when the source provides one.
    pub value: Option<Expression>,
}

// ============================================================================
// Statements
// ============================================================================

/// A statement or declaration in the source tree.
#[derive(Debug, Clone)]
pub enum AstNode {
    // -- Declarations ----------------------------------------------------

    /// A function, method or function expression.
    Function(Function),

    /// A class declaration.
    Class(Class),

    /// A variable or constant declaration.
    Declaration {
        /// Declared name.
        name: String,
        /// Initialiser, when the source provides one.
        value: Option<Box<Expression>>,
        /// Whether the binding is mutable.
        mutable: bool,
    },

    /// An `import` / `use` statement.
    Import {
        /// Imported module path.
        module: Option<String>,
        /// Imported names.
        names: Vec<String>,
        /// Whether the import uses the `from` form.
        is_from: bool,
    },

    /// An `export` statement.
    Export {
        /// The exported declaration.
        node: Box<AstNode>,
        /// Whether the export is the default one.
        is_default: bool,
    },

    /// An interface declaration.
    Interface {
        /// Interface name.
        name: String,
        /// Generic type parameters.
        type_params: Vec<(String, Option<Type>)>,
        /// Extended interfaces.
        extends: Vec<String>,
        /// Interface body.
        body: Vec<TypeMember>,
    },

    /// A type alias declaration.
    TypeAlias {
        /// Alias name.
        name: String,
        /// Generic type parameters.
        type_params: Vec<String>,
        /// Aliased type.
        type_annotation: Type,
    },

    /// An enum declaration.
    Enum {
        /// Enum name.
        name: String,
        /// Enum members.
        members: Vec<EnumMember>,
        /// Whether the enum is a `const enum`.
        is_const: bool,
    },

    /// A namespace / module declaration.
    Namespace {
        /// Namespace name.
        name: String,
        /// Namespace body.
        body: Vec<Statement>,
    },

    // -- Blocks and control flow ----------------------------------------

    /// A nested block of statements.
    Block(Vec<AstNode>),

    /// An `if` / `elif` / `else` chain.
    If {
        /// Controlling condition.
        condition: Box<Expression>,
        /// Statements executed when the condition holds.
        then_branch: Vec<AstNode>,
        /// Alternative statements.
        else_branch: Vec<AstNode>,
    },

    /// A `while` loop.
    While {
        /// Controlling condition.
        condition: Box<Expression>,
        /// Loop body.
        body: Vec<AstNode>,
        /// Loop `else` body.
        else_body: Vec<AstNode>,
    },

    /// A `for` loop.
    For {
        /// Loop variable.
        target: Box<Expression>,
        /// Iterable expression.
        iter: Box<Expression>,
        /// Loop body.
        body: Vec<AstNode>,
        /// Loop `else` body.
        else_body: Vec<AstNode>,
    },

    /// A `switch` / `match` statement.
    Switch {
        /// The expression being switched on.
        discriminant: Box<Expression>,
        /// The cases of the switch.
        cases: Vec<AstNode>,
    },

    /// A single case of a [`AstNode::Switch`].
    SwitchCase {
        /// The matched value, or `None` for the default case.
        test: Option<Box<Expression>>,
        /// The statements of the case.
        consequent: Vec<AstNode>,
    },

    /// A `try` statement.
    Try {
        /// The protected body.
        body: Vec<AstNode>,
        /// The exception handlers.
        handlers: Vec<AstNode>,
        /// The `else` body.
        else_body: Vec<AstNode>,
        /// The `finally` body.
        finally_body: Vec<AstNode>,
    },

    /// A `with` statement.
    With {
        /// The managed resources.
        items: Vec<(Expression, Option<Expression>)>,
        /// The protected body.
        body: Vec<AstNode>,
    },

    /// A single exception handler of a [`AstNode::Try`].
    ExceptHandler {
        /// The caught exception type, when one is given.
        type_: Option<Box<Expression>>,
        /// The name the exception is bound to, when one is given.
        name: Option<String>,
        /// The handler body.
        body: Vec<AstNode>,
    },

    // -- Simple statements -----------------------------------------------

    /// An expression evaluated for its effect.
    ExpressionStatement(Box<Expression>),

    /// An assignment.
    Assignment {
        /// The assigned target.
        target: Box<Expression>,
        /// The assigned value.
        value: Box<Expression>,
    },

    /// A `return` statement.
    Return(Option<Box<Expression>>),

    /// A `raise` / `throw` statement.
    Raise(Option<Box<Expression>>),

    /// A `break` statement.
    Break,

    /// A `continue` statement.
    Continue,

    /// An `assert` statement.
    Assert {
        /// The asserted condition.
        condition: Box<Expression>,
        /// The optional failure message.
        message: Option<Box<Expression>>,
    },

    /// A `delete` statement.
    Delete {
        /// The deleted operand.
        target: Box<Expression>,
    },

    /// A `global` declaration.
    Global {
        /// The names declared global.
        names: Vec<String>,
    },

    /// A `nonlocal` declaration.
    Nonlocal {
        /// The names declared non-local.
        names: Vec<String>,
    },
}

// ============================================================================
// Expressions
// ============================================================================

/// An expression in the source tree.
#[derive(Debug, Clone)]
pub enum Expression {
    /// A literal value.
    Literal(Literal),

    /// A bare identifier.
    Identifier(String),

    /// The `this` keyword.
    This,

    /// The `super` keyword.
    Super,

    /// A binary operation.
    BinaryOp {
        /// Left operand.
        left: Box<Expression>,
        /// The operator.
        op: BinaryOp,
        /// Right operand.
        right: Box<Expression>,
    },

    /// A unary operation.
    UnaryOp {
        /// The operator.
        op: UnaryOp,
        /// The operand.
        operand: Box<Expression>,
    },

    /// An assignment expression.
    Assignment {
        /// The assigned target.
        target: Box<Expression>,
        /// The assigned value.
        value: Box<Expression>,
    },

    /// A walrus assignment (`:=`).
    NamedExpr {
        /// The assigned target.
        target: Box<Expression>,
        /// The assigned value.
        value: Box<Expression>,
    },

    /// A function call.
    Call {
        /// The callee.
        function: Box<Expression>,
        /// The call arguments.
        arguments: Vec<Expression>,
    },

    /// A `new` expression.
    New {
        /// The constructor.
        constructor: Box<Expression>,
        /// The constructor arguments.
        arguments: Vec<Expression>,
    },

    /// A field access (`obj.field`).
    FieldAccess {
        /// The receiver.
        object: Box<Expression>,
        /// The field name.
        field: String,
    },

    /// An indexed access (`arr[i]`).
    Index {
        /// The indexed value.
        array: Box<Expression>,
        /// The index.
        index: Box<Expression>,
    },

    /// An optional chain (`a?.b`).
    OptionalChain {
        /// The chain base.
        base: Box<Expression>,
        /// The chain elements.
        path: Vec<ChainElement>,
    },

    /// A tuple literal.
    Tuple(Vec<Expression>),

    /// An array literal.
    Array(Vec<Expression>),

    /// An object literal.
    Object(Vec<(Expression, Expression)>),

    /// A block used as an expression.
    Block(Vec<AstNode>),

    /// A lambda expression.
    Lambda {
        /// The declared parameters.
        params: Vec<Param>,
        /// The lambda body.
        body: Box<Expression>,
    },

    /// A named function expression.
    Function(String, Vec<Param>, Box<Expression>),

    /// A conditional expression (`a ? b : c`).
    Conditional {
        /// The controlling condition.
        test: Box<Expression>,
        /// The value when the condition holds.
        consequent: Box<Expression>,
        /// The value otherwise.
        alternate: Box<Expression>,
    },

    /// An `await` expression.
    Await(Box<Expression>),

    /// A `yield` expression.
    Yield(Option<Box<Expression>>),

    /// A `yield from` expression.
    YieldFrom(Box<Expression>),
}

/// A single step of an optional chain.
#[derive(Debug, Clone)]
pub enum ChainElement {
    /// `?.prop`
    Property(String),
    /// `?.[expr]`
    Index(Box<Expression>),
    /// A `?.` that is followed by a call.
    Optional,
}
