//! Macro system for TUMC - Declarative and procedural macros
//! Implements hygienic macros with proper scope handling

use crate::prelude::*;
use super::ast::{self, *};
use super::lexer::{Lexer, Token, TokenKind};
use super::parser::Parser;
use std::collections::{HashMap, HashSet};

/// Macro expansion environment
#[derive(Debug, Clone)]
pub struct MacroExpansionEnv {
    /// Current scope level
    scope_level: usize,
    /// Variable bindings in current scope
    bindings: HashMap<String, Token>,
    /// Macro definitions
    macros: HashMap<String, MacroDefinition>,
    /// Hygiene context
    hygiene: HygieneContext,
}

impl MacroExpansionEnv {
    pub fn new() -> Self {
        MacroExpansionEnv {
            scope_level: 0,
            bindings: HashMap::new(),
            macros: HashMap::new(),
            hygiene: HygieneContext::new(),
        }
    }

    /// Enter a new scope
    pub fn enter_scope(&mut self) {
        self.scope_level += 1;
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) {
        self.scope_level -= 1;
        // Clean up bindings from exited scope
        self.bindings.retain(|_, binding| binding.scope >= self.scope_level);
    }

    /// Add a macro definition
    pub fn add_macro(&mut self, name: String, macro_: MacroDefinition) {
        self.macrosacros.insert(name, macro_);
    }

    /// Look up a macro
    pub fn get_macro(&self, name: &str) -> Option<&MacroDefinition> {
        self.macros.get(name)
    }

    /// Create a unique identifier for hygiene
    pub fn fresh_ident(&mut self, base: &str) -> Token {
        self.hygiene.fresh_ident(base, self.scope_level)
    }

    /// Expand all macros in an expression
    pub fn expand_macros(&mut self, expr: &Expr) -> Result<Expr, MacroError> {
        match &expr.kind {
            ExprKind::Call { function, arguments } => {
                // Check if function is a macro
                if let ExprKind::Identifier(name) = &function.kind {
                    if let Some(macro_) = self.get_macro(name) {
                        return self.expand_macro_call(macro_, arguments);
                    }
                }

                // Recursively expand in arguments
                let mut expanded_args = Vec::new();
                for arg in arguments {
                    expanded_args.push(self.expand_macros(arg)?);
                }

                Ok(Expr {
                    kind: ExprKind::Call {
                        function: Box::new(self.expand_macros(function)?),
                        arguments: expanded_args,
                    },
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::MethodCall { object, method, arguments } => {
                Ok(Expr {
                    kind: ExprKind::MethodCall {
                        object: Box::new(self.expand_macros(object)?),
                        method: method.clone(),
                        arguments: arguments.iter()
                            .map(|a| self.expand_macros(a))
                            .collect::<Result<Vec<_>, _>>()?,
                    },
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::Binary { op, lhs, rhs } => {
                Ok(Expr {
                    kind: ExprKind::Binary {
                        op: op.clone(),
                        lhs: Box::new(self.expand_macros(lhs)?),
                        rhs: Box::new(self.expand_macros(rhs)?),
                    },
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::Unary { op, operand } => {
                Ok(Expr {
                    kind: ExprKind::Unary {
                        op: op.clone(),
                        operand: Box::new(self.expand_macros(operand)?),
                    },
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::Block(statements) => {
                self.enter_scope();
                let expanded_stmts = statements.iter()
                    .map(|s| self.expand_stmt_macros(s))
                    .collect::<Result<Vec<_>, _>>()?;
                self.exit_scope();

                Ok(Expr {
                    kind: ExprKind::Block(expanded_stmts),
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::Array(elements) => {
                Ok(Expr {
                    kind: ExprKind::Array(
                        elements.iter()
                            .map(|e| self.expand_macros(e))
                            .collect::<Result<Vec<_>, _>>()?
                    ),
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::Tuple(elements) => {
                Ok(Expr {
                    kind: ExprKind::Tuple(
                        elements.iter()
                            .map(|e| self.expand_macros(e))
                            .collect::<Result<Vec<_>, _>>()?
                    ),
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::If { condition, then_block, else_expr } => {
                Ok(Expr {
                    kind: ExprKind::If {
                        condition: Box::new(self.expand_macros(condition)?),
                        then_block: then_block.iter()
                            .map(|(p, b)| {
                                Ok((p.clone(), b.iter()
                                    .map(|s| self.expand_stmt_macros(s))
                                    .collect::<Result<Vec<_>, _>>()?))
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                        else_expr: if let Some((p, b)) = else_expr {
                            Some((p.clone(), b.iter()
                                .map(|s| self.expand_stmt_macros(s))
                                .collect::<Result<Vec<_>, _>>()?))
                        } else {
                            None
                        },
                    },
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            ExprKind::Match { discriminant, arms } => {
                Ok(Expr {
                    kind: ExprKind::Match {
                        discriminant: Box::new(self.expand_macros(discriminant)?),
                        arms: arms.iter()
                            .map(|arm| {
                                Ok(MatchArm {
                                    pattern: self.expand_pattern_macros(&arm.pattern)?,
                                    guard: arm.guard.as_ref()
                                        .map(|g| self.expand_macros(g))
                                        .transpose()?,
                                    body: self.expand_macros(&arm.body)?,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()?,
                    },
                    span: expr.span.clone(),
                    annotations: expr.annotations.clone(),
                })
            }
            _ => Ok(expr.clone()),
        }
    }

    /// Expand macros in a statement
    fn expand_stmt_macros(&mut self, stmt: &Stmt) -> Result<Stmt, MacroError> {
        match &stmt.kind {
            StmtKind::Let(pattern, ann_type, value, body) => {
                Ok(Stmt {
                    kind: StmtKind::Let(
                        self.expand_pattern_macros(pattern)?,
                        ann_type.clone(),
                        if let Some(v) = value {
                            Some(self.expand_macros(v)?)
                        } else {
                            None
                        },
                        body.iter()
                            .map(|s| self.expand_stmt_macros(s))
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                    span: stmt.span.clone(),
                })
            }
            StmtKind::Expr(expr) => {
                Ok(Stmt {
                    kind: StmtKind::Expr(self.expand_macros(expr)?),
                    span: stmt.span.clone(),
                })
            }
            StmtKind::Assignment(lhs, rhs) => {
                Ok(Stmt {
                    kind: StmtKind::Assignment(
                        Box::new(self.expand_macros(lhs)?),
                        Box::new(self.expand_macros(rhs)?),
                    ),
                    span: stmt.span.clone(),
                })
            }
            StmtKind::If(condition, then_block, else_block) => {
                Ok(Stmt {
                    kind: StmtKind::If(
                        Box::new(self.expand_macros(condition)?),
                        then_block.iter()
                            .map(|s| self.expand_stmt_macros(s))
                            .collect::<Result<Vec<_>, _>>()?,
                        else_block.as_ref().map(|(p, b)| {
                            Ok((p.clone(), b.iter()
                                .map(|s| self.expand_stmt_macros(s))
                                .collect::<Result<Vec<_>, _>>()?))
                        }).transpose()?,
                    ),
                    span: stmt.span.clone(),
                })
            }
            StmtKind::While(condition, body) => {
                Ok(Stmt {
                    kind: StmtKind::While(
                        Box::new(self.expand_macros(condition)?),
                        body.iter()
                            .map(|s| self.expand_stmt_macros(s))
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                    span: stmt.span.clone(),
                })
            }
            StmtKind::For(var, iterable, body) => {
                Ok(Stmt {
                    kind: StmtKind::For(
                        var.clone(),
                        Box::new(self.expand_macros(iterable)?),
                        body.iter()
                            .map(|s| self.expand_stmt_macros(s))
                            .collect::<Result<Vec<_>, _>>()?,
                    ),
                    span: stmt.span.clone(),
                })
            }
            _ => Ok(stmt.clone()),
        }
    }

    /// Expand macros in a pattern
    fn expand_pattern_macros(&mut self, pattern: &Pattern) -> Result<Pattern, MacroError> {
        Ok(pattern.clone()) // Patterns don't typically contain macros
    }

    /// Expand a macro call
    fn expand_macro_call(&mut self, macro_: &MacroDefinition, arguments: &[Expr]) -> Result<Expr, MacroError> {
        match macro_ {
            MacroDefinition::Declarative(rules) => {
                self.expand_declarative_macro(rules, arguments)
            }
            MacroDefinition::Procedural(attr) => {
                self.expand_procedural_macro(attr, arguments)
            }
        }
    }

    /// Expand a declarative macro
    fn expand_declarative_macro(&mut self, rules: &[MacroRule], arguments: &[Expr]) -> Result<Expr, MacroError> {
        for rule in rules {
            if let Some(bindings) = rule.matcher.match_tokens(arguments) {
                return rule.expander.expand(&bindings);
            }
        }
        Err(MacroError::NoMatchingRule {
            arguments: arguments.len(),
        })
    }

    /// Expand a procedural macro
    fn expand_procedural_macro(&mut self, _attr: &ProcMacroAttribute, arguments: &[Expr]) -> Result<Expr, MacroError> {
        // Procedural macros would be handled by external proc-macro crates
        // For now, return the arguments as-is wrapped in a block
        Ok(Expr {
            kind: ExprKind::Block(
                arguments.iter().map(|a| Stmt {
                    kind: StmtKind::Expr(a.clone()),
                    span: a.span.clone(),
                }).collect()
            ),
            span: Span::default(),
            annotations: vec![],
        })
    }
}

/// Hygiene context for macro expansion
#[derive(Debug, Clone)]
pub struct HygieneContext {
    /// Counter for generating unique identifiers
    counter: usize,
    /// Mapping from original to hygiene-marked identifiers
    marks: HashMap<String, HashSet<String>>,
}

impl HygieneContext {
    pub fn new() -> Self {
        HygieneContext {
            counter: 0,
            marks: HashMap::new(),
        }
    }

    /// Create a fresh identifier with hygiene marking
    pub fn fresh_ident(&mut self, base: &str, scope: usize) -> Token {
        let unique = format!("{}_{}_{}", base, self.counter, scope);
        self.counter += 1;

        self.marks.entry(base.to_string()).or_default().insert(unique.clone());

        Token {
            kind: TokenKind::Identifier(unique),
            span: Span::default(),
            scope,
        }
    }
}

/// Macro definition
#[derive(Debug, Clone)]
pub enum MacroDefinition {
    /// Declarative macro with pattern matching rules
    Declarative(Vec<MacroRule>),
    /// Procedural macro
    Procedural(ProcMacroAttribute),
}

/// A rule in a declarative macro
#[derive(Debug, Clone)]
pub struct MacroRule {
    /// Pattern matcher
    pub matcher: MacroMatcher,
    /// Expander
    pub expander: MacroExpander,
    /// Original source span
    pub span: Span,
}

/// Macro matcher for declarative macros
#[derive(Debug, Clone)]
pub struct MacroMatcher {
    /// Pattern tokens
    pub patterns: Vec<MatcherToken>,
    /// Metadata
    pub meta: MacroMetadata,
}

impl MacroMatcher {
    /// Try to match arguments against this pattern
    pub fn match_tokens(&self, arguments: &[Expr]) -> Option<TokenBindings> {
        if arguments.len() != self.patterns.len() {
            return None;
        }

        let mut bindings = TokenBindings::new();

        for (pattern, arg) in self.patterns.iter().zip(arguments.iter()) {
            if !pattern.matches(arg, &mut bindings) {
                return None;
            }
        }

        Some(bindings)
    }
}

/// Token in a macro pattern
#[derive(Debug, Clone)]
pub enum MatcherToken {
    /// Match any expression and bind it
    Expr(String),
    /// Match any statement and bind it
    Stmt(String),
    /// Match type annotation
    Type(String),
    /// Match pattern
    Pattern(String),
    /// Match literal value
    Literal,
    /// Match identifier
    Ident,
    /// Match block
    Block(String),
    /// Match repetition
    Repetition {
        name: String,
        kind: RepetitionKind,
        inner: Vec<MatcherToken>,
    },
    /// Match optional item
    Optional {
        name: String,
        inner: Vec<MatcherToken>,
    },
    /// Match alternation
    Alternation(Vec<Vec<MatcherToken>>),
    /// Match exactly this token
    Exact(TokenKind),
    /// Match any token
    Any,
}

/// Repetition kind for macro patterns
#[derive(Debug, Clone, Copy)]
pub enum RepetitionKind {
    /// Zero or more (*)
    ZeroOrMore,
    /// One or more (+)
    OneOrMore,
    /// Zero or one (?)
    ZeroOrOne,
}

/// Metadata for macro rules
#[derive(Debug, Clone)]
pub struct MacroMetadata {
    /// Rule name for debugging
    pub name: String,
    /// Original source location
    pub span: Span,
}

/// Token bindings from macro matching
#[derive(Debug, Clone, Default)]
pub struct TokenBindings {
    /// Bound expressions
    exprs: HashMap<String, Vec<Expr>>,
    /// Bound statements
    stmts: HashMap<String, Vec<Stmt>>,
    /// Bound types
    types: HashMap<String, Vec<AnnType>>,
    /// Bound patterns
    patterns: HashMap<String, Vec<Pattern>>,
    /// Bound identifiers
    idents: HashMap<String, Vec<String>>,
}

impl TokenBindings {
    pub fn new() -> Self {
        TokenBindings {
            exprs: HashMap::new(),
            stmts: HashMap::new(),
            types: HashMap::new(),
            patterns: HashMap::new(),
            idents: HashMap::new(),
        }
    }

    /// Add an expression binding
    pub fn add_expr(&mut self, name: &str, expr: Expr) {
        self.exprs.entry(name.to_string()).or_default().push(expr);
    }

    /// Add a statement binding
    pub fn add_stmt(&mut self, name: &str, stmt: Stmt) {
        self.stmts.entry(name.to_string()).or_default().push(stmt);
    }

    /// Get bound expressions
    pub fn get_exprs(&self, name: &str) -> &[Expr] {
        self.exprs.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get bound statements
    pub fn get_stmts(&self, name: &str) -> &[Stmt] {
        self.stmts.get(name).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Check if a binding exists
    pub fn has_binding(&self, name: &str) -> bool {
        self.exprs.contains_key(name)
            || self.stmts.contains_key(name)
            || self.types.contains_key(name)
            || self.patterns.contains_key(name)
            || self.idents.contains_key(name)
    }
}

/// Macro expander
#[derive(Debug, Clone)]
pub struct MacroExpander {
    /// Template tokens
    pub template: Vec<TemplateToken>,
    /// Delimiter type
    pub delimiter: Delimiter,
}

impl MacroExpander {
    /// Expand the template with bindings
    pub fn expand(&self, bindings: &TokenBindings) -> Result<Expr, MacroError> {
        let mut result_stmts = Vec::new();

        for token in &self.template {
            let stmts = token.expand(bindings)?;
            result_stmts.extend(stmts);
        }

        if result_stmts.len() == 1 {
            if let StmtKind::Expr(expr) = result_stmts[0].kind {
                return Ok(expr);
            }
        }

        Ok(Expr {
            kind: ExprKind::Block(result_stmts),
            span: Span::default(),
            annotations: vec![],
        })
    }
}

/// Token in an expansion template
#[derive(Debug, Clone)]
pub enum TemplateToken {
    /// Literal token
    Literal(TokenKind),
    /// Insert a bound expression
    Expr(String),
    /// Insert a bound statement
    Stmt(String),
    /// Insert multiple statements
    Stmts(String),
    /// Insert a type
    Type(String),
    /// Insert an identifier
    Ident(String),
    /// Insert the identifier with `$crate` prefix
    CrateIdent(String),
    /// Repetition
    Repetition {
        name: String,
        kind: RepetitionKind,
        separator: Option<TokenKind>,
        inner: Vec<TemplateToken>,
    },
    /// Conditional
    Condition {
        name: String,
        then: Vec<TemplateToken>,
        else_: Option<Vec<TemplateToken>>,
    },
    /// Token pasting (concatenation)
    Concat(Vec<TemplateToken>),
    /// Stringification
    Stringify(Vec<TemplateToken>),
}

/// Delimiter for macro expansion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
    /// ( ... )
    Parenthesis,
    /// [ ... ]
    Bracket,
    /// { ... }
    Brace,
}

/// Procedural macro attribute
#[derive(Debug, Clone)]
pub struct ProcMacroAttribute {
    /// Macro kind
    pub kind: ProcMacroKind,
    /// Function name
    pub function: String,
    /// Attributes
    pub attributes: Vec<Attribute>,
}

/// Kind of procedural macro
#[derive(Debug, Clone, Copy)]
pub enum ProcMacroKind {
    /// Function-like macro: macro_rules! foo { ... }
    FunctionLike,
    /// Derive macro: #[derive(Foo)]
    Derive,
    /// Attribute macro: #[foo]
    Attribute,
}

/// Macro error types
#[derive(Debug, Clone)]
pub enum MacroError {
    /// No matching rule found
    NoMatchingRule {
        arguments: usize,
    },
    /// Repetition mismatch
    RepetitionMismatch {
        name: String,
        expected: usize,
        found: usize,
    },
    /// Missing binding
    MissingBinding {
        name: String,
    },
    /// Invalid repetition
    InvalidRepetition {
        name: String,
    },
    /// Tokenization error
    TokenizationError(String),
    /// Expansion error
    ExpansionError(String),
    /// Hygiene violation
    HygieneViolation(String),
    /// Recursive macro expansion limit
    ExpansionDepthExceeded {
        limit: usize,
        current: usize,
    },
}

impl fmt::Display for MacroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MacroError::NoMatchingRule { arguments } => {
                write!(f, "No matching rule for {} arguments", arguments)
            }
            MacroError::RepetitionMismatch { name, expected, found } => {
                write!(f, "Repetition '{}': expected {}, found {}", name, expected, found)
            }
            MacroError::MissingBinding { name } => {
                write!(f, "Missing binding for '${}'", name)
            }
            MacroError::InvalidRepetition { name } => {
                write!(f, "Invalid repetition '{}'", name)
            }
            MacroError::TokenizationError(msg) => {
                write!(f, "Tokenization error: {}", msg)
            }
            MacroError::ExpansionError(msg) => {
                write!(f, "Expansion error: {}", msg)
            }
            MacroError::HygieneViolation(msg) => {
                write!(f, "Hygiene violation: {}", msg)
            }
            MacroError::ExpansionDepthExceeded { limit, current } => {
                write!(f, "Expansion depth {} exceeds limit {}", current, limit)
            }
        }
    }
}

impl std::error::Error for MacroError {}

/// Macro expansion engine
pub struct MacroEngine {
    /// Global environment
    env: MacroExpansionEnv,
    /// Maximum expansion depth
    max_depth: usize,
    /// Built-in macros
    builtins: HashMap<String, MacroDefinition>,
}

impl MacroEngine {
    pub fn new() -> Self {
        let mut engine = MacroEngine {
            env: MacroExpansionEnv::new(),
            max_depth: 100,
            builtins: HashMap::new(),
        };
        engine.register_builtins();
        engine
    }

    /// Register built-in macros
    fn register_builtins(&mut self) {
        // `println!` macro
        self.builtins.insert("println".to_string(), MacroDefinition::Declarative(vec![
            MacroRule {
                matcher: MacroMatcher {
                    patterns: vec![MatcherToken::Expr("fmt".to_string())],
                    meta: MacroMetadata {
                        name: "println".to_string(),
                        span: Span::default(),
                    },
                },
                expander: MacroExpander {
                    template: vec![
                        TemplateToken::Literal(TokenKind::Ident("print".to_string())),
                        TemplateToken::Expr("fmt".to_string()),
                        TemplateToken::Literal(TokenKind::Literal("\n".to_string())),
                    ],
                    delimiter: Delimiter::Parenthesis,
                },
                span: Span::default(),
            }
        ]));

        // `vec!` macro
        self.builtins.insert("vec".to_string(), MacroDefinition::Declarative(vec![
            MacroRule {
                matcher: MacroMatcher {
                    patterns: vec![
                        MatcherToken::Repetition {
                            name: "elem".to_string(),
                            kind: RepetitionKind::ZeroOrMore,
                            inner: vec![MatcherToken::Expr("e".to_string())],
                        }
                    ],
                    meta: MacroMetadata {
                        name: "vec".to_string(),
                        span: Span::default(),
                    },
                },
                expander: MacroExpander {
                    template: vec![
                        TemplateToken::Literal(TokenKind::Ident("Vec".to_string())),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Bang)),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Lt)),
                        TemplateToken::Expr("elem".to_string()),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Gt)),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Colon)),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Colon)),
                        TemplateToken::Literal(TokenKind::Ident("new".to_string())),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::LParen)),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::RParen)),
                    ],
                    delimiter: Delimiter::Bracket,
                },
                span: Span::default(),
            }
        ]));

        // `assert_eq!` macro
        self.builtins.insert("assert_eq".to_string(), MacroDefinition::Declarative(vec![
            MacroRule {
                matcher: MacroMatcher {
                    patterns: vec![
                        MatcherToken::Expr("left".to_string()),
                        MatcherToken::Expr("right".to_string()),
                    ],
                    meta: MacroMetadata {
                        name: "assert_eq".to_string(),
                        span: Span::default(),
                    },
                },
                expander: MacroExpander {
                    template: vec![
                        TemplateToken::Literal(TokenKind::Ident("assert".to_string())),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Bang)),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::LParen)),
                        TemplateToken::Expr("left".to_string()),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::EqEq)),
                        TemplateToken::Expr("right".to_string()),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::RParen)),
                    ],
                    delimiter: Delimiter::Parenthesis,
                },
                span: Span::default(),
            }
        ]));

        // `format!` macro
        self.builtins.insert("format".to_string(), MacroDefinition::Declarative(vec![
            MacroRule {
                matcher: MacroMatcher {
                    patterns: vec![MatcherToken::Expr("fmt".to_string())],
                    meta: MacroMetadata {
                        name: "format".to_string(),
                        span: Span::default(),
                    },
                },
                expander: MacroExpander {
                    template: vec![
                        TemplateToken::Ident("String".to_string()),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Colon)),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::Colon)),
                        TemplateToken::Ident("from_utf8_lossy".to_string()),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::LParen)),
                        TemplateToken::Expr("fmt".to_string()),
                        TemplateToken::Literal(TokenKind::Punct(PunctKind::RParen)),
                    ],
                    delimiter: Delimiter::Parenthesis,
                },
                span: Span::default(),
            }
        ]));
    }

    /// Register a macro
    pub fn register_macro(&mut self, name: String, definition: MacroDefinition) {
        self.env.add_macro(name, definition);
    }

    /// Expand macros in a program
    pub fn expand_program(&mut self, program: &mut Program) -> Result<(), MacroError> {
        let mut depth = 0;

        loop {
            let mut expanded = false;

            for item in &mut program.items {
                if let Item::Function(func) = item {
                    for stmt in &mut func.body {
                        if let StmtKind::Expr(expr) = &mut stmt.kind {
                            if let Ok(new_expr) = self.env.expand_macros(expr) {
                                if new_expr != *expr {
                                    *expr = new_expr;
                                    expanded = true;
                                }
                            }
                        }
                    }
                }
            }

            depth += 1;
            if depth > self.max_depth {
                return Err(MacroError::ExpansionDepthExceeded {
                    limit: self.max_depth,
                    current: depth,
                });
            }

            if !expanded {
                break;
            }
        }

        Ok(())
    }

    /// Get macro by name
    pub fn get_macro(&self, name: &str) -> Option<&MacroDefinition> {
        self.env.get_macro(name).or_else(|| self.builtins.get(name))
    }

    /// Check if macro exists
    pub fn has_macro(&self, name: &str) -> bool {
        self.get_macro(name).is_some()
    }
}

impl Default for MacroExpansionEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MacroEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Token kinds for macros
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PunctKind {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Not,
    And,
    Or,
    AndAnd,
    OrOr,
    EqEq,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Shl,
    Shr,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    RemEq,
    XorEq,
    AndEq,
    OrEq,
    ShlEq,
    ShrEq,
    At,
    Dot,
    DotDot,
    DotDotEq,
    DotDotDot,
    DotDotDotDot,
    Colon,
    ColonColon,
    Semi,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Bang,
    Question,
    Tilde,
    ColonEq,
}

impl MatcherToken {
    /// Check if this pattern matches an expression
    pub fn matches(&self, expr: &Expr, bindings: &mut TokenBindings) -> bool {
        match self {
            MatcherToken::Expr(name) => {
                bindings.add_expr(name, expr.clone());
                true
            }
            MatcherToken::Literal => {
                matches!(expr.kind, ExprKind::Literal(_))
            }
            MatcherToken::Ident => {
                matches!(expr.kind, ExprKind::Identifier(_))
            }
            MatcherToken::Block(name) => {
                if let ExprKind::Block(_) = expr.kind {
                    bindings.add_expr(name, expr.clone());
                    true
                } else {
                    false
                }
            }
            MatcherToken::Any => true,
            MatcherToken::Exact(tk) => {
                // Check if expression matches the token kind
                match tk {
                    TokenKind::Identifier(_) => matches!(expr.kind, ExprKind::Identifier(_)),
                    TokenKind::Literal(_) => matches!(expr.kind, ExprKind::Literal(_)),
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl TemplateToken {
    /// Expand this template token
    pub fn expand(&self, bindings: &TokenBindings) -> Result<Vec<Stmt>, MacroError> {
        match self {
            TemplateToken::Literal(tk) => {
                Ok(vec![Stmt {
                    kind: StmtKind::Expr(Expr {
                        kind: ExprKind::Identifier(match tk {
                            TokenKind::Ident(s) => s.clone(),
                            TokenKind::Literal(Literal::String(s)) => s.clone(),
                            _ => "()".to_string(),
                        }),
                        span: Span::default(),
                        annotations: vec![],
                    }),
                    span: Span::default(),
                }])
            }
            TemplateToken::Expr(name) => {
                let exprs = bindings.get_exprs(name);
                if exprs.is_empty() {
                    return Err(MacroError::MissingBinding { name: name.clone() });
                }
                Ok(exprs.iter().map(|e| Stmt {
                    kind: StmtKind::Expr(e.clone()),
                    span: e.span.clone(),
                }).collect())
            }
            TemplateToken::Stmt(name) => {
                let stmts = bindings.get_stmts(name);
                if stmts.is_empty() {
                    return Err(MacroError::MissingBinding { name: name.clone() });
                }
                Ok(stmts.to_vec())
            }
            TemplateToken::Stmts(name) => {
                Ok(bindings.get_stmts(name).to_vec())
            }
            TemplateToken::Ident(name) => {
                let idents = bindings.idents.get(name).map(|v| v.as_slice()).unwrap_or(&[]);
                if idents.is_empty() {
                    return Err(MacroError::MissingBinding { name: name.clone() });
                }
                Ok(idents.iter().map(|id| Stmt {
                    kind: StmtKind::Expr(Expr {
                        kind: ExprKind::Identifier(id.clone()),
                        span: Span::default(),
                        annotations: vec![],
                    }),
                    span: Span::default(),
                }).collect())
            }
            TemplateToken::Repetition { name, separator, inner, .. } => {
                let items = bindings.get_exprs(name);
                let mut result = Vec::new();

                for (i, item) in items.iter().enumerate() {
                    for token in inner {
                        let mut stmts = token.expand(bindings)?;
                        result.append(&mut stmts);
                    }

                    if let Some(sep) = separator {
                        if i < items.len() - 1 {
                            result.push(Stmt {
                                kind: StmtKind::Expr(Expr {
                                    kind: ExprKind::Identifier(match sep {
                                        TokenKind::Ident(s) => s.clone(),
                                        _ => "()".to_string(),
                                    }),
                                    span: Span::default(),
                                    annotations: vec![],
                                }),
                                span: Span::default(),
                            });
                        }
                    }
                }

                Ok(result)
            }
            TemplateToken::Condition { name, then, else_ } => {
                if bindings.has_binding(name) {
                    let mut result = Vec::new();
                    for token in then {
                        result.append(&mut token.expand(bindings)?);
                    }
                    Ok(result)
                } else if let Some(else_tokens) = else_ {
                    let mut result = Vec::new();
                    for token in else_tokens {
                        result.append(&mut token.expand(bindings)?);
                    }
                    Ok(result)
                } else {
                    Ok(vec![])
                }
            }
            _ => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_expansion_env() {
        let mut env = MacroExpansionEnv::new();
        env.enter_scope();

        let ident = env.fresh_ident("foo");
        assert!(ident.kind.to_string().contains("foo"));

        env.exit_scope();
    }

    #[test]
    fn test_macro_binding() {
        let mut bindings = TokenBindings::new();
        bindings.add_expr("x", Expr::new_identifier("42"));

        assert_eq!(bindings.get_exprs("x").len(), 1);
        assert!(bindings.has_binding("x"));
        assert!(!bindings.has_binding("y"));
    }

    #[test]
    fn test_builtin_macros() {
        let engine = MacroEngine::new();
        assert!(engine.has_macro("println"));
        assert!(engine.has_macro("vec"));
        assert!(engine.has_macro("assert_eq"));
    }
}
