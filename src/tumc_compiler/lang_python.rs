//! TUMC Python Language Frontend
//!
//! Transpiles Python source code to TUMC IR for compilation to any target.

use crate::tumc_compiler::lexer::Lexer;
use crate::tumc_compiler::ast::{AstNode, Program, Function, Statement, Expression, Type};
use crate::tumc_compiler::parser::Parser;
use std::collections::HashMap;

/// Python-specific AST nodes
#[derive(Debug, Clone)]
pub enum PyAstNode {
    /// Decorator
    Decorator(String),
    /// With statement
    With {
        items: Vec<(Expression, Option<Expression>)>,
        body: Vec<Statement>,
    },
    /// Async function
    AsyncFunction(Function),
    /// Try-except-else-finally
    Try {
        body: Vec<Statement>,
        handlers: Vec<ExceptHandler>,
        orelse: Vec<Statement>,
        finalbody: Vec<Statement>,
    },
    /// Except clause
    ExceptHandler {
        type_: Option<Box<Expression>>,
        name: Option<String>,
        body: Vec<Statement>,
    },
    /// For comprehension
    ListComp {
        element: Expression,
        generators: Vec<Comprehension>,
    },
    /// Dict comprehension
    DictComp {
        key: Expression,
        value: Expression,
        generators: Vec<Comprehension>,
    },
    /// Generator expression
    GeneratorExp {
        element: Expression,
        generators: Vec<Comprehension>,
    },
    /// Walrus operator (:=)
    NamedExpression {
        target: Box<Expression>,
        value: Box<Expression>,
    },
    /// Await expression
    Await(Expression),
    /// Yield expression
    Yield(Option<Expression>),
    /// Yield from
    YieldFrom(Expression),
    /// Type annotation
    AnnAssign {
        target: Box<Expression>,
        annotation: Box<Expression>,
        value: Option<Box<Expression>>,
        simple: bool,
    },
}

/// Comprehension clause
#[derive(Debug, Clone)]
pub struct Comprehension {
    /// Target (for x in ...)
    pub target: Box<Expression>,
    /// Iterable
    pub iter: Box<Expression>,
    /// If conditions
    pub ifs: Vec<Expression>,
    /// Is async?
    pub is_async: bool,
}

/// Python parser
pub struct PythonParser {
    /// Tokens from lexer
    tokens: Vec<Token>,
    /// Current position
    pos: usize,
    /// Indentation stack
    indent_stack: Vec<usize>,
    /// Source lines for error reporting
    source_lines: Vec<String>,
}

/// Python token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    Bytes(Vec<u8>),
    Bool(bool),
    None,

    // Identifiers
    Ident(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    DoubleSlash,
    Percent,
    DoubleStar,
    At,           // @
    Colon,
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    DoubleSlashEqual,
    PercentEqual,
    DoubleStarEqual,
    AtEqual,

    // Comparison
    EqEqual,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // Boolean
    And,
    Or,
    Not,

    // Bitwise
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    LeftShift,
    RightShift,
    AmpersandEqual,
    PipeEqual,
    CaretEqual,
    LeftShiftEqual,
    RightShiftEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Colon,

    // Special
    Newline,
    Indent,
    Dedent,
    EndOfFile,
    Comment(String),

    // Keywords
    False,
    None_,
    True,
    And_,
    As,
    Assert,
    Async,
    Await,
    Break,
    Class,
    Continue,
    Def,
    Del,
    Elif,
    Else,
    Except,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Nonlocal,
    Not_,
    Or_,
    Pass,
    Raise,
    Return,
    Try,
    While,
    With,
    Yield,

    // Walrus operator
    Walrus,
}

impl PythonParser {
    /// Create new Python parser
    pub fn new(source: &str) -> Self {
        let lexer = PythonLexer::new(source);
        let tokens = lexer.tokenize().unwrap_or_default();
        Self {
            tokens,
            pos: 0,
            indent_stack: vec![0],
            source_lines: source.lines().map(|s| s.to_string()).collect(),
        }
    }

    /// Parse Python source to AST
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }

        Ok(Program { nodes: statements })
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Option<AstNode>, ParseError> {
        // Skip newlines
        while self.check(&Token::Newline) {
            self.advance();
        }

        if self.is_at_end() {
            return Ok(None);
        }

        // Check for decorators
        let decorators = self.parse_decorators()?;

        // Check for async/def/class/while/for/if/try/with
        let stmt = match self.peek() {
            Token::Def => {
                self.advance();
                self.parse_function(decorators, false)?
            }
            Token::Async => {
                self.advance();
                if self.check(&Token::Def) {
                    self.advance();
                    self.parse_function(decorators, true)?
                } else {
                    return Err(ParseError::new("Expected 'def' after 'async'", self.pos));
                }
            }
            Token::Class => {
                self.advance();
                self.parse_class(decorators)?
            }
            Token::If => {
                self.advance();
                self.parse_if()?
            }
            Token::While => {
                self.advance();
                self.parse_while()?
            }
            Token::For => {
                self.advance();
                self.parse_for()?
            }
            Token::Try => {
                self.advance();
                self.parse_try()?
            }
            Token::With => {
                self.advance();
                self.parse_with()?
            }
            Token::Return => {
                self.advance();
                let value = if !self.check(&Token::Newline) && !self.is_at_end() {
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                };
                AstNode::Return(value)
            }
            Token::Break => {
                self.advance();
                AstNode::Break
            }
            Token::Continue => {
                self.advance();
                AstNode::Continue
            }
            Token::Pass => {
                self.advance();
                AstNode::Block(Vec::new())
            }
            Token::Raise => {
                self.advance();
                let exc = if !self.check(&Token::Newline) && !self.is_at_end() {
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                };
                AstNode::Raise(exc)
            }
            Token::Assert => {
                self.advance();
                let cond = Box::new(self.parse_expression()?);
                let msg = if self.check(&Token::Comma) {
                    self.advance();
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                };
                AstNode::Assert { condition: cond, message: msg }
            }
            Token::Global => {
                self.advance();
                let mut names = Vec::new();
                while !self.check(&Token::Newline) {
                    if let Token::Ident(name) = self.current() {
                        names.push(name.clone());
                        self.advance();
                    } else {
                        break;
                    }
                }
                AstNode::Global(names)
            }
            Token::Nonlocal => {
                self.advance();
                let mut names = Vec::new();
                while !self.check(&Token::Newline) {
                    if let Token::Ident(name) = self.current() {
                        names.push(name.clone());
                        self.advance();
                    } else {
                        break;
                    }
                }
                AstNode::Nonlocal(names)
            }
            Token::Del => {
                self.advance();
                let target = Box::new(self.parse_expression()?);
                AstNode::Delete(target)
            }
            Token::Import | Token::From => {
                self.parse_import()?
            }
            _ => {
                // Expression statement or assignment
                let expr = self.parse_expression()?;

                // Check for assignment operators
                if self.check(&Token::Equal) {
                    self.advance();
                    let value = Box::new(self.parse_expression()?);
                    AstNode::Assignment { target: Box::new(expr), value }
                } else if let Some(op) = self.get_compound_assign() {
                    self.advance();
                    let right = Box::new(self.parse_expression()?);
                    AstNode::Assignment {
                        target: Box::new(expr.clone()),
                        value: Box::new(AstNode::BinaryOp {
                            left: Box::new(expr),
                            op: op,
                            right,
                        }),
                    }
                } else {
                    AstNode::ExpressionStatement(Box::new(expr))
                }
            }
        };

        // Consume newline
        if !self.check(&Token::Newline) && !self.is_at_end() && !self.check(&Token::Dedent) {
            // This is fine for some constructs
        }

        Ok(Some(stmt))
    }

    /// Parse decorators
    fn parse_decorators(&mut self) -> Result<Vec<String>, ParseError> {
        let mut decorators = Vec::new();

        while self.check(&Token::At) {
            self.advance();
            if let Token::Ident(name) = self.current() {
                decorators.push(name.clone());
                self.advance();
            } else {
                return Err(ParseError::new("Expected identifier after '@'", self.pos));
            }

            // Optional arguments
            if self.check(&Token::LeftParen) {
                self.advance();
                while !self.check(&Token::RightParen) {
                    self.parse_expression()?;
                    if !self.check(&Token::RightParen) {
                        self.expect(&Token::Comma)?;
                    }
                }
                self.expect(&Token::RightParen)?;
            }
        }

        Ok(decorators)
    }

    /// Parse function definition
    fn parse_function(&mut self, _decorators: Vec<String>, is_async: bool) -> Result<AstNode, ParseError> {
        let name = if let Token::Ident(n) = self.current() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return Err(ParseError::new("Expected function name", self.pos));
        };

        self.expect(&Token::LeftParen)?;

        let mut params = Vec::new();
        let mut defaults = Vec::new();

        while !self.check(&Token::RightParen) {
            if let Token::Star = self.current() {
                self.advance();
                let mut name = String::new();
                if let Token::Ident(n) = self.current() {
                    name = n.clone();
                    self.advance();
                }
                params.push(crate::tumc_compiler::ast::Param {
                    name,
                    param_type: None,
                    default: None,
                    is_variadic: true,
                    is_kwargs: false,
                });
            } else if let Token::DoubleStar = self.current() {
                self.advance();
                let mut name = String::new();
                if let Token::Ident(n) = self.current() {
                    name = n.clone();
                    self.advance();
                }
                params.push(crate::tumc_compiler::ast::Param {
                    name,
                    param_type: None,
                    default: None,
                    is_variadic: false,
                    is_kwargs: true,
                });
            } else {
                let param_name = if let Token::Ident(n) = self.current() {
                    let n = n.clone();
                    self.advance();
                    n
                } else {
                    return Err(ParseError::new("Expected parameter name", self.pos));
                };

                let default = if self.check(&Token::Equal) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                params.push(crate::tumc_compiler::ast::Param {
                    name: param_name,
                    param_type: None,
                    default: default.clone(),
                    is_variadic: false,
                    is_kwargs: false,
                });
            }

            if !self.check(&Token::RightParen) {
                self.expect(&Token::Comma)?;
            }
        }

        self.expect(&Token::RightParen)?;

        // Return type annotation
        let return_type = if self.check(&Token::Arrow) {
            self.advance();
            Some(Type::Unknown) // Would parse type annotation
        } else {
            None
        };

        // Function body
        self.expect(&Token::Colon)?;
        let body = self.parse_block()?;

        Ok(AstNode::Function(Function {
            name,
            params,
            return_type,
            body,
            is_async,
            is_generator: false,
        }))
    }

    /// Parse class definition
    fn parse_class(&mut self, _decorators: Vec<String>) -> Result<AstNode, ParseError> {
        let name = if let Token::Ident(n) = self.current() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return Err(ParseError::new("Expected class name", self.pos));
        };

        // Base classes
        let mut bases = Vec::new();
        if self.check(&Token::LeftParen) {
            self.advance();
            while !self.check(&Token::RightParen) {
                bases.push(self.parse_expression()?);
                if !self.check(&Token::RightParen) {
                    self.expect(&Token::Comma)?;
                }
            }
            self.expect(&Token::RightParen)?;
        }

        self.expect(&Token::Colon)?;
        let body = self.parse_block()?;

        Ok(AstNode::Class(crate::tumc_compiler::ast::Class {
            name,
            bases,
            body,
            decorators: Vec::new(),
        }))
    }

    /// Parse if statement
    fn parse_if(&mut self) -> Result<AstNode, ParseError> {
        let condition = Box::new(self.parse_expression()?);
        self.expect(&Token::Colon)?;
        let then_branch = self.parse_block()?;

        let mut else_branch = Vec::new();

        while self.check(&Token::Elif) {
            self.advance();
            let elif_cond = Box::new(self.parse_expression()?);
            self.expect(&Token::Colon)?;
            let elif_body = self.parse_block()?;

            else_branch = vec![AstNode::If {
                condition: elif_cond,
                then_branch,
                else_branch,
            }];
            return Ok(else_branch.pop().unwrap());
        }

        if self.check(&Token::Else) {
            self.advance();
            self.expect(&Token::Colon)?;
            else_branch = self.parse_block()?;
        }

        Ok(AstNode::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parse while loop
    fn parse_while(&mut self) -> Result<AstNode, ParseError> {
        let condition = Box::new(self.parse_expression()?);
        self.expect(&Token::Colon)?;
        let body = self.parse_block()?;

        let else_body = if self.check(&Token::Else) {
            self.advance();
            self.expect(&Token::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(AstNode::While {
            condition,
            body,
            else_body,
        })
    }

    /// Parse for loop
    fn parse_for(&mut self) -> Result<AstNode, ParseError> {
        let target = Box::new(self.parse_expression()?);
        self.expect(&Token::In)?;
        let iter = Box::new(self.parse_expression()?);
        self.expect(&Token::Colon)?;
        let body = self.parse_block()?;

        let else_body = if self.check(&Token::Else) {
            self.advance();
            self.expect(&Token::Colon)?;
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(AstNode::For {
            target,
            iter,
            body,
            else_body,
        })
    }

    /// Parse try-except block
    fn parse_try(&mut self) -> Result<AstNode, ParseError> {
        self.expect(&Token::Colon)?;
        let body = self.parse_block()?;

        let mut handlers = Vec::new();
        let mut else_body = Vec::new();
        let mut finally_body = Vec::new();

        while self.check(&Token::Except) {
            self.advance();

            let exc_type = if !self.check(&Token::Colon) {
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };

            let name = if self.check(&Token::Ident(String::new())) ||
                       (self.check(&Token::Comma) || self.check(&Token::Colon)) == false {
                self.advance();
                if let Token::Ident(n) = self.previous() {
                    Some(n.clone())
                } else {
                    None
                }
            } else {
                None
            };

            self.expect(&Token::Colon)?;
            let handler_body = self.parse_block()?;

            handlers.push(crate::tumc_compiler::ast::ExceptHandler {
                exception_type: exc_type,
                name,
                body: handler_body,
            });
        }

        if self.check(&Token::Else) {
            self.advance();
            self.expect(&Token::Colon)?;
            else_body = self.parse_block()?;
        }

        if self.check(&Token::Finally) {
            self.advance();
            self.expect(&Token::Colon)?;
            finally_body = self.parse_block()?;
        }

        Ok(AstNode::Try {
            body,
            handlers,
            else_body,
            finally_body,
        })
    }

    /// Parse with statement
    fn parse_with(&mut self) -> Result<AstNode, ParseError> {
        let mut items = Vec::new();

        loop {
            let context = Box::new(self.parse_expression()?);
            let as_var = if self.check(&Token::As) {
                self.advance();
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };

            items.push((context, as_var));

            if !self.check(&Token::Comma) {
                break;
            }
            self.advance();
        }

        self.expect(&Token::Colon)?;
        let body = self.parse_block()?;

        Ok(AstNode::With { items, body })
    }

    /// Parse import statement
    fn parse_import(&mut self) -> Result<AstNode, ParseError> {
        let is_from = self.check(&Token::From);
        if is_from {
            self.advance();
        }

        let mut module = String::new();

        // Parse module name
        while !self.check(&Token::Import) && !self.check(&Token::Newline) && !self.is_at_end() {
            if let Token::Ident(n) = self.current() {
                if !module.is_empty() {
                    module.push('.');
                }
                module.push_str(&n);
                self.advance();
            } else if self.check(&Token::Star) {
                break;
            } else {
                break;
            }
        }

        if self.check(&Token::Import) {
            self.advance();
        }

        let mut names = Vec::new();

        if self.check(&Token::Star) {
            names.push(("__all__".to_string(), None));
            self.advance();
        } else {
            while !self.check(&Token::Newline) && !self.is_at_end() {
                if let Token::Ident(n) = self.current() {
                    let alias = if self.check(&Token::As) {
                        self.advance();
                        if let Token::Ident(a) = self.current() {
                            let a = a.clone();
                            self.advance();
                            Some(a)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    names.push((n.clone(), alias));
                    self.advance();
                }

                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }

        Ok(AstNode::Import {
            module: if module.is_empty() { None } else { Some(module) },
            names,
            is_from,
        })
    }

    /// Parse block of statements
    fn parse_block(&mut self) -> Result<Vec<AstNode>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&Token::Dedent) && !self.is_at_end() && !self.check(&Token::Newline) {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }

            while self.check(&Token::Newline) {
                self.advance();
            }
        }

        Ok(statements)
    }

    /// Parse expression
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        self.parse_assignment()
    }

    /// Parse assignment expressions (walrus operator)
    fn parse_assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_or()?;

        if self.check(&Token::Walrus) {
            self.advance();
            let value = Box::new(self.parse_assignment()?);
            return Ok(Expression::NamedExpr {
                target: Box::new(expr.into()),
                value,
            });
        }

        // Regular assignment
        if self.check(&Token::Equal) {
            self.advance();
            let value = Box::new(self.parse_assignment()?);
            return Ok(Expression::Assignment {
                target: Box::new(expr.into()),
                value,
            });
        }

        Ok(expr)
    }

    /// Parse or expression
    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_and()?;

        while self.check(&Token::Or) || matches!(self.current(), Token::Ident(ref n) if n == "or") {
            self.advance();
            let right = Box::new(self.parse_and()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op: crate::tumc_compiler::ast::BinaryOp::Or,
                right,
            };
        }

        Ok(left)
    }

    /// Parse and expression
    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_not()?;

        while self.check(&Token::And) || matches!(self.current(), Token::Ident(ref n) if n == "and") {
            self.advance();
            let right = Box::new(self.parse_not()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op: crate::tumc_compiler::ast::BinaryOp::And,
                right,
            };
        }

        Ok(left)
    }

    /// Parse not expression
    fn parse_not(&mut self) -> Result<Expression, ParseError> {
        if self.check(&Token::Not) || matches!(self.current(), Token::Ident(ref n) if n == "not") {
            self.advance();
            let operand = Box::new(self.parse_not()?);
            return Ok(Expression::UnaryOp {
                op: crate::tumc_compiler::ast::UnaryOp::Not,
                operand,
            });
        }

        self.parse_comparison()
    }

    /// Parse comparison
    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_bitwise_or()?;

        while let Some(op) = self.get_comparison_op() {
            self.advance();
            let right = Box::new(self.parse_bitwise_or()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op,
                right,
            };
        }

        Ok(left)
    }

    /// Get comparison operator
    fn get_comparison_op(&mut self) -> Option<crate::tumc_compiler::ast::BinaryOp> {
        match self.current() {
            Token::Less => Some(crate::tumc_compiler::ast::BinaryOp::Lt),
            Token::Greater => Some(crate::tumc_compiler::ast::BinaryOp::Gt),
            Token::LessEqual => Some(crate::tumc_compiler::ast::BinaryOp::Le),
            Token::GreaterEqual => Some(crate::tumc_compiler::ast::BinaryOp::Ge),
            Token::EqEqual => Some(crate::tumc_compiler::ast::BinaryOp::Eq),
            Token::NotEqual => Some(crate::tumc_compiler::ast::BinaryOp::Ne),
            Token::In => Some(crate::tumc_compiler::ast::BinaryOp::In),
            Token::Ident(ref n) if n == "is" => Some(crate::tumc_compiler::ast::BinaryOp::Is),
            _ => None,
        }
    }

    /// Parse bitwise or
    fn parse_bitwise_or(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_bitwise_xor()?;

        while self.check(&Token::Pipe) {
            self.advance();
            let right = Box::new(self.parse_bitwise_xor()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitOr,
                right,
            };
        }

        Ok(left)
    }

    /// Parse bitwise xor
    fn parse_bitwise_xor(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_bitwise_and()?;

        while self.check(&Token::Caret) {
            self.advance();
            let right = Box::new(self.parse_bitwise_and()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitXor,
                right,
            };
        }

        Ok(left)
    }

    /// Parse bitwise and
    fn parse_bitwise_and(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_shift()?;

        while self.check(&Token::Ampersand) {
            self.advance();
            let right = Box::new(self.parse_shift()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitAnd,
                right,
            };
        }

        Ok(left)
    }

    /// Parse shift
    fn parse_shift(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_additive()?;

        while self.check(&Token::LeftShift) {
            self.advance();
            let right = Box::new(self.parse_additive()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op: crate::tumc_compiler::ast::BinaryOp::Shl,
                right,
            };
        }

        while self.check(&Token::RightShift) {
            self.advance();
            let right = Box::new(self.parse_additive()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op: crate::tumc_compiler::ast::BinaryOp::Shr,
                right,
            };
        }

        Ok(left)
    }

    /// Parse additive
    fn parse_additive(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_multiplicative()?;

        while self.check(&Token::Plus) || self.check(&Token::Minus) {
            let op = if self.check(&Token::Plus) {
                crate::tumc_compiler::ast::BinaryOp::Add
            } else {
                crate::tumc_compiler::ast::BinaryOp::Sub
            };
            self.advance();
            let right = Box::new(self.parse_multiplicative()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op,
                right,
            };
        }

        Ok(left)
    }

    /// Parse multiplicative
    fn parse_multiplicative(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.parse_power()?;

        while self.check(&Token::Star) || self.check(&Token::Slash) ||
              self.check(&Token::DoubleSlash) || self.check(&Token::Percent) {
            let op = if self.check(&Token::Star) {
                crate::tumc_compiler::ast::BinaryOp::Mul
            } else if self.check(&Token::DoubleSlash) {
                crate::tumc_compiler::ast::BinaryOp::FloorDiv
            } else if self.check(&Token::Percent) {
                crate::tumc_compiler::ast::BinaryOp::Rem
            } else {
                crate::tumc_compiler::ast::BinaryOp::Div
            };
            self.advance();
            let right = Box::new(self.parse_power()?);
            left = Expression::BinaryOp {
                left: Box::new(left.into()),
                op,
                right,
            };
        }

        Ok(left)
    }

    /// Parse power (right associative)
    fn parse_power(&mut self) -> Result<Expression, ParseError> {
        let base = self.parse_unary()?;

        if self.check(&Token::DoubleStar) {
            self.advance();
            let exp = Box::new(self.parse_power()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(base.into()),
                op: crate::tumc_compiler::ast::BinaryOp::Pow,
                right: exp,
            });
        }

        Ok(base)
    }

    /// Parse unary
    fn parse_unary(&mut self) -> Result<Expression, ParseError> {
        if self.check(&Token::Plus) {
            self.advance();
            return Ok(Expression::UnaryOp {
                op: crate::tumc_compiler::ast::UnaryOp::Plus,
                operand: Box::new(self.parse_unary()?),
            });
        }

        if self.check(&Token::Minus) {
            self.advance();
            return Ok(Expression::UnaryOp {
                op: crate::tumc_compiler::ast::UnaryOp::Negate,
                operand: Box::new(self.parse_unary()?),
            });
        }

        if self.check(&Token::Tilde) {
            self.advance();
            return Ok(Expression::UnaryOp {
                op: crate::tumc_compiler::ast::UnaryOp::BitNot,
                operand: Box::new(self.parse_unary()?),
            });
        }

        self.parse_call()
    }

    /// Parse call and attribute access
    fn parse_call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&Token::LeftParen) {
                self.advance();
                let mut args = Vec::new();

                while !self.check(&Token::RightParen) {
                    // Check for *args and **kwargs
                    if self.check(&Token::Star) {
                        self.advance();
                        args.push(crate::tumc_compiler::ast::Arg::Variadic(Box::new(self.parse_expression()?)));
                    } else if self.check(&Token::DoubleStar) {
                        self.advance();
                        args.push(crate::tumc_compiler::ast::Arg::KwVariadic(Box::new(self.parse_expression()?)));
                    } else if !self.check(&Token::RightParen) {
                        args.push(crate::tumc_compiler::ast::Arg::Positional(Box::new(self.parse_expression()?)));
                    }

                    if !self.check(&Token::RightParen) {
                        self.expect(&Token::Comma)?;
                    }
                }

                self.expect(&Token::RightParen)?;
                expr = Expression::Call {
                    function: Box::new(expr.into()),
                    arguments: args,
                };
            } else if self.check(&Token::Dot) {
                self.advance();
                if let Token::Ident(name) = self.current() {
                    let name = name.clone();
                    self.advance();
                    expr = Expression::FieldAccess {
                        object: Box::new(expr.into()),
                        field: name,
                    };
                } else {
                    return Err(ParseError::new("Expected identifier after '.'", self.pos));
                }
            } else if self.check(&Token::LeftBracket) {
                self.advance();
                let index = Box::new(self.parse_expression()?);
                self.expect(&Token::RightBracket)?;
                expr = Expression::Index {
                    array: Box::new(expr.into()),
                    index,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parse primary expressions
    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let token = self.current();

        match token {
            Token::Int(n) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Int(*n)))
            }
            Token::Float(n) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Float(*n)))
            }
            Token::String(s) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::String(s.clone())))
            }
            Token::Bool(b) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Bool(*b)))
            }
            Token::None => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Null))
            }
            Token::Ident(name) => {
                self.advance();
                Ok(Expression::Identifier(name.clone()))
            }
            Token::LeftParen => {
                self.advance();

                // Tuple or parenthesized expression
                if self.check(&Token::RightParen) {
                    self.advance();
                    return Ok(Expression::Tuple(Vec::new()));
                }

                let first = self.parse_expression()?;

                if self.check(&Token::Comma) {
                    self.advance();
                    let mut elements = vec![first];
                    while !self.check(&Token::RightParen) {
                        elements.push(self.parse_expression()?);
                        if !self.check(&Token::RightParen) {
                            self.expect(&Token::Comma)?;
                        }
                    }
                    self.expect(&Token::RightParen)?;
                    return Ok(Expression::Tuple(elements));
                }

                self.expect(&Token::RightParen)?;

                if matches!(first, Expression::Lambda { .. }) {
                    Ok(first)
                } else {
                    Ok(first)
                }
            }
            Token::LeftBracket => {
                self.advance();
                let mut elements = Vec::new();

                while !self.check(&Token::RightBracket) {
                    elements.push(self.parse_expression()?);
                    if !self.check(&Token::RightBracket) {
                        self.expect(&Token::Comma)?;
                    }
                }

                self.expect(&Token::RightBracket)?;
                Ok(Expression::Array(elements))
            }
            Token::LeftBrace => {
                self.advance();
                let mut pairs = Vec::new();

                while !self.check(&Token::RightBrace) {
                    let key = self.parse_expression()?;
                    self.expect(&Token::Colon)?;
                    let value = self.parse_expression()?;
                    pairs.push((key, value));

                    if !self.check(&Token::RightBrace) {
                        self.expect(&Token::Comma)?;
                    }
                }

                self.expect(&Token::RightBrace)?;
                Ok(Expression::Object(pairs))
            }
            Token::Lambda => {
                self.advance();
                self.parse_lambda()
            }
            Token::Await => {
                self.advance();
                Ok(Expression::Await(Box::new(self.parse_expression()?)))
            }
            Token::Yield => {
                self.advance();
                if self.check(&Token::Equal) {
                    self.advance();
                    Ok(Expression::YieldFrom(Box::new(self.parse_expression()?)))
                } else if !self.check(&Token::Newline) && !self.is_at_end() {
                    Ok(Expression::Yield(Some(Box::new(self.parse_expression()?))))
                } else {
                    Ok(Expression::Yield(None))
                }
            }
            _ => Err(ParseError::new(&format!("Unexpected token: {:?}", token), self.pos)),
        }
    }

    /// Parse lambda
    fn parse_lambda(&mut self) -> Result<Expression, ParseError> {
        let mut params = Vec::new();

        while !self.check(&Token::Colon) && !self.is_at_end() {
            if let Token::Ident(name) = self.current() {
                let name = name.clone();
                self.advance();
                params.push(crate::tumc_compiler::ast::Param {
                    name,
                    param_type: None,
                    default: None,
                    is_variadic: false,
                    is_kwargs: false,
                });
            }

            if !self.check(&Token::Colon) {
                self.expect(&Token::Comma)?;
            }
        }

        self.expect(&Token::Colon)?;
        let body = Box::new(self.parse_expression()?);

        Ok(Expression::Lambda {
            params,
            body,
        })
    }

    // ==== Helper methods ====

    fn current(&self) -> Token {
        if self.pos < self.tokens.len() {
            self.tokens[self.pos].clone()
        } else {
            Token::EndOfFile
        }
    }

    fn previous(&self) -> Token {
        if self.pos > 0 {
            self.tokens[self.pos - 1].clone()
        } else {
            Token::EndOfFile
        }
    }

    fn peek(&self) -> Token {
        self.current()
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(self.current(), Token::EndOfFile)
    }

    fn check(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.current()) == std::mem::discriminant(token)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, token: &Token) -> Result<(), ParseError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(&format!("Expected {:?}", token), self.pos))
        }
    }

    fn get_compound_assign(&self) -> Option<crate::tumc_compiler::ast::BinaryOp> {
        match self.current() {
            Token::PlusEqual => Some(crate::tumc_compiler::ast::BinaryOp::Add),
            Token::MinusEqual => Some(crate::tumc_compiler::ast::BinaryOp::Sub),
            Token::StarEqual => Some(crate::tumc_compiler::ast::BinaryOp::Mul),
            Token::SlashEqual => Some(crate::tumc_compiler::ast::BinaryOp::Div),
            Token::PercentEqual => Some(crate::tumc_compiler::ast::BinaryOp::Rem),
            Token::DoubleStarEqual => Some(crate::tumc_compiler::ast::BinaryOp::Pow),
            Token::DoubleSlashEqual => Some(crate::tumc_compiler::ast::BinaryOp::FloorDiv),
            Token::AmpersandEqual => Some(crate::tumc_compiler::ast::BinaryOp::BitAnd),
            Token::PipeEqual => Some(crate::tumc_compiler::ast::BinaryOp::BitOr),
            Token::CaretEqual => Some(crate::tumc_compiler::ast::BinaryOp::BitXor),
            Token::LeftShiftEqual => Some(crate::tumc_compiler::ast::BinaryOp::Shl),
            Token::RightShiftEqual => Some(crate::tumc_compiler::ast::BinaryOp::Shr),
            _ => None,
        }
    }
}

/// Python lexer
pub struct PythonLexer {
    /// Source code
    source: String,
    /// Current position
    pos: usize,
    /// Line number
    line: usize,
    /// Column number
    column: usize,
}

impl PythonLexer {
    /// Create new lexer
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize source code
    pub fn tokenize(&self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let mut pos = 0;
        let source = &self.source[..];

        while pos < source.len() {
            let (token, next_pos) = self.lex_token(source, pos)?;
            if let Some(t) = token {
                tokens.push(t);
            }
            pos = next_pos;
        }

        tokens.push(Token::EndOfFile);
        Ok(tokens)
    }

    /// Lex a single token
    fn lex_token(&self, source: &str, pos: usize) -> Result<(Option<Token>, usize), LexerError> {
        let mut p = pos;

        // Skip whitespace (but not newlines)
        while p < source.len() && source[p..].starts_with(' ') ||
              source[p..].starts_with('\t') || source[p..].starts_with('\r') {
            p += 1;
        }

        if p >= source.len() {
            return Ok((None, p));
        }

        let ch = source[p];

        // Newlines
        if ch == '\n' {
            return Ok((Some(Token::Newline), p + 1));
        }

        // Comments
        if source[p..].starts_with('#') {
            while p < source.len() && source[p] != '\n' {
                p += 1;
            }
            return Ok((Some(Token::Comment(source[pos..p].to_string())), p));
        }

        // String literals
        if ch == '"' || ch == '\'' || source[p..].starts_with("'''") || source[p..].starts_with("\"\"\"") {
            return self.lex_string(source, pos);
        }

        // Bytes literal
        if source[p..].starts_with("b'") || source[p..].starts_with("b\"") {
            return self.lex_bytes(source, pos);
        }

        // Raw/f-strings
        if source[p..].starts_with("r'") || source[p..].starts_with("r\"") ||
           source[p..].starts_with("rf'") || source[p..].starts_with("rf\"") ||
           source[p..].starts_with("fr'") || source[p..].starts_with("fr\"") {
            return self.lex_string(source, pos);
        }

        // Numbers
        if ch.is_ascii_digit() || (ch == '.' && p + 1 < source.len() && source[p + 1].is_ascii_digit()) {
            return self.lex_number(source, pos);
        }

        // Identifiers and keywords
        if ch.is_alphabetic() || ch == '_' {
            return self.lex_identifier(source, pos);
        }

        // Operators and delimiters
        let (token, next) = self.lex_operator(source, p)?;
        Ok((Some(token), next))
    }

    /// Lex string literal
    fn lex_string(&self, source: &str, pos: usize) -> Result<(Option<Token>, usize), LexerError> {
        let p = pos;
        let ch = source[p];

        // Determine quote type
        let is_triple = if source[p..].starts_with("'''") || source[p..].starts_with("\"\"\"") {
            3
        } else {
            1
        };

        let quote = if is_triple == 3 {
            if source[p..].starts_with("'''") { "'''" } else { "\"\"\"" }
        } else {
            &source[p..p + 1]
        };

        let mut end = p + is_triple;

        // Skip escape sequences
        while end < source.len() {
            if source[end..].starts_with(quote) {
                end += is_triple;
                break;
            }
            if source[end] == '\\' && end + 1 < source.len() {
                end += 2;
                continue;
            }
            if source[end] == '\n' && is_triple == 1 {
                break;
            }
            end += 1;
        }

        let s = &source[p + is_triple..end - is_triple];
        Ok((Some(Token::String(s.to_string())), end))
    }

    /// Lex bytes literal
    fn lex_bytes(&self, source: &str, pos: usize) -> Result<(Option<Token>, usize), LexerError> {
        let p = pos + 2;
        let ch = source[p];
        let quote = &source[pos + 1..pos + 2];

        let mut end = p + 1;

        while end < source.len() {
            if source[end..].starts_with(quote) {
                end += 1;
                break;
            }
            if source[end] == '\\' && end + 1 < source.len() {
                end += 2;
                continue;
            }
            end += 1;
        }

        let s = &source[pos + 2..end - 1];
        let bytes: Vec<u8> = s.bytes().filter(|b| *b != b'\\').collect();
        Ok((Some(Token::Bytes(bytes)), end))
    }

    /// Lex number
    fn lex_number(&self, source: &str, pos: usize) -> Result<(Option<Token>, usize), LexerError> {
        let mut p = pos;
        let mut is_float = false;

        // Hex, octal, binary
        if source[p..].starts_with("0x") || source[p..].starts_with("0X") {
            p += 2;
            while p < source.len() && (source[p].is_ascii_hexdigit() || source[p] == '_') {
                p += 1;
            }
            let s = &source[pos..p];
            let value = i64::from_str_radix(&s[2..].replace('_', ""), 16).unwrap_or(0);
            return Ok((Some(Token::Int(value)), p));
        }

        if source[p..].starts_with("0o") || source[p..].starts_with("0O") {
            p += 2;
            while p < source.len() && (source[p].is_ascii_digit() || source[p] == '_') {
                p += 1;
            }
            let s = &source[pos..p];
            let value = i64::from_str_radix(&s[2..].replace('_', ""), 8).unwrap_or(0);
            return Ok((Some(Token::Int(value)), p));
        }

        if source[p..].starts_with("0b") || source[p..].starts_with("0B") {
            p += 2;
            while p < source.len() && (source[p] == '0' || source[p] == '1' || source[p] == '_') {
                p += 1;
            }
            let s = &source[pos..p];
            let value = i64::from_str_radix(&s[2..].replace('_', ""), 2).unwrap_or(0);
            return Ok((Some(Token::Int(value)), p));
        }

        // Decimal
        while p < source.len() && (source[p].is_ascii_digit() || source[p] == '_') {
            p += 1;
        }

        // Float
        if p < source.len() && source[p] == '.' && p + 1 < source.len() && source[p + 1].is_ascii_digit() {
            is_float = true;
            p += 1;
            while p < source.len() && (source[p].is_ascii_digit() || source[p] == '_') {
                p += 1;
            }
        }

        // Exponent
        if p < source.len() && (source[p] == 'e' || source[p] == 'E') {
            is_float = true;
            p += 1;
            if p < source.len() && (source[p] == '+' || source[p] == '-') {
                p += 1;
            }
            while p < source.len() && source[p].is_ascii_digit() {
                p += 1;
            }
        }

        let s = &source[pos..p];

        if is_float {
            let value = s.parse::<f64>().unwrap_or(0.0);
            Ok((Some(Token::Float(value)), p))
        } else {
            let value = s.replace('_', "").parse::<i64>().unwrap_or(0);
            Ok((Some(Token::Int(value)), p))
        }
    }

    /// Lex identifier or keyword
    fn lex_identifier(&self, source: &str, pos: usize) -> Result<(Option<Token>, usize), LexerError> {
        let mut p = pos;

        while p < source.len() && (source[p].is_alphanumeric() || source[p] == '_') {
            p += 1;
        }

        let s = &source[pos..p];

        let token = match s {
            "False" => Token::False,
            "None" => Token::None,
            "True" => Token::True,
            "and" => Token::And,
            "as" => Token::As,
            "assert" => Token::Assert,
            "async" => Token::Async,
            "await" => Token::Await,
            "break" => Token::Break,
            "class" => Token::Class,
            "continue" => Token::Continue,
            "def" => Token::Def,
            "del" => Token::Del,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "except" => Token::Except,
            "finally" => Token::Finally,
            "for" => Token::For,
            "from" => Token::From,
            "global" => Token::Global,
            "if" => Token::If,
            "import" => Token::Import,
            "in" => Token::In,
            "is" => Token::Ident("is".to_string()),
            "lambda" => Token::Lambda,
            "nonlocal" => Token::Nonlocal,
            "not" => Token::Not,
            "or" => Token::Or,
            "pass" => Token::Pass,
            "raise" => Token::Raise,
            "return" => Token::Return,
            "try" => Token::Try,
            "while" => Token::While,
            "with" => Token::With,
            "yield" => Token::Yield,
            _ => Token::Ident(s.to_string()),
        };

        Ok((Some(token), p))
    }

    /// Lex operator
    fn lex_operator(&self, source: &str, pos: usize) -> Result<(Token, usize), LexerError> {
        let p = pos;
        let ch = source[p];

        let token = match ch {
            '+' => {
                if source[p..].starts_with("+=") { Token::PlusEqual }
                else { Token::Plus }
            }
            '-' => {
                if source[p..].starts_with("-=") { Token::MinusEqual }
                else if source[p..].starts_with("->") { Token::Colon } // Simplified
                else { Token::Minus }
            }
            '*' => {
                if source[p..].starts_with("**") {
                    if source[p..].starts_with("**=") { Token::DoubleStarEqual }
                    else { Token::DoubleStar }
                } else if source[p..].starts_with("*=") { Token::StarEqual }
                else { Token::Star }
            }
            '/' => {
                if source[p..].starts_with("//") {
                    if source[p..].starts_with("//=") { Token::DoubleSlashEqual }
                    else { Token::DoubleSlash }
                } else if source[p..].starts_with("/=") { Token::SlashEqual }
                else { Token::Slash }
            }
            '%' => {
                if source[p..].starts_with("%=") { Token::PercentEqual }
                else { Token::Percent }
            }
            '@' => {
                if source[p..].starts_with("@=") { Token::AtEqual }
                else { Token::At }
            }
            '=' => {
                if source[p..].starts_with("==") { Token::EqEqual }
                else { Token::Equal }
            }
            '!' => {
                if source[p..].starts_with("!=") { Token::NotEqual }
                else { return Err(LexerError::new("Invalid character '!'", pos)); }
            }
            '<' => {
                if source[p..].starts_with("<=") { Token::LessEqual }
                else if source[p..].starts_with("<<") {
                    if source[p..].starts_with("<<=") { Token::LeftShiftEqual }
                    else { Token::LeftShift }
                }
                else { Token::Less }
            }
            '>' => {
                if source[p..].starts_with(">=") { Token::GreaterEqual }
                else if source[p..].starts_with(">>") {
                    if source[p..].starts_with(">>=") { Token::RightShiftEqual }
                    else { Token::RightShift }
                }
                else { Token::Greater }
            }
            '&' => {
                if source[p..].starts_with("&=") { Token::AmpersandEqual }
                else { Token::Ampersand }
            }
            '|' => {
                if source[p..].starts_with("|=") { Token::PipeEqual }
                else { Token::Pipe }
            }
            '^' => {
                if source[p..].starts_with("^=") { Token::CaretEqual }
                else { Token::Caret }
            }
            '~' => Token::Tilde,
            ':' => Token::Colon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            ',' => Token::Comma,
            '.' => Token::Dot,
            ';' => Token::Semicolon,
            _ => return Err(LexerError::new(&format!("Unknown character '{}'", ch), pos)),
        };

        let next = match token {
            Token::PlusEqual | Token::MinusEqual | Token::StarEqual | Token::SlashEqual |
            Token::DoubleSlashEqual | Token::PercentEqual | Token::DoubleStarEqual | Token::AtEqual |
            Token::EqEqual | Token::NotEqual | Token::LessEqual | Token::GreaterEqual |
            Token::LeftShift | Token::RightShift | Token::LeftShiftEqual | Token::RightShiftEqual |
            Token::AmpersandEqual | Token::PipeEqual | Token::CaretEqual => p + 2,
            _ => p + 1,
        };

        Ok((token, next))
    }
}

/// Parse error
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl ParseError {
    pub fn new(message: &str, position: usize) -> Self {
        Self {
            message: message.to_string(),
            position,
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for ParseError {}

/// Lexer error
#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub position: usize,
}

impl LexerError {
    pub fn new(message: &str, position: usize) -> Self {
        Self { message: message.to_string(), position }
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error at position {}: {}", self.position, self.message)
    }
}

impl std::error::Error for LexerError {}

/// Python transpiler to TUMC
pub struct PythonTranspiler {
    /// Type hints for inference
    type_hints: HashMap<String, Type>,
}

impl PythonTranspiler {
    /// Create new transpiler
    pub fn new() -> Self {
        Self {
            type_hints: HashMap::new(),
        }
    }

    /// Transpile Python source to TUMC IR
    pub fn transpile(&mut self, source: &str) -> Result<Program, ParseError> {
        let mut parser = PythonParser::new(source);
        let ast = parser.parse()?;

        // Convert Python AST to TUMC AST
        let tumc_ast = self.convert_program(ast);

        Ok(tumc_ast)
    }

    /// Convert Python AST to TUMC AST
    fn convert_program(&self, py_ast: Program) -> Program {
        Program {
            nodes: py_ast.nodes.into_iter().map(|n| self.convert_node(n)).collect(),
        }
    }

    /// Convert Python AST node to TUMC AST node
    fn convert_node(&self, node: AstNode) -> AstNode {
        node
    }
}

impl Default for PythonTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_numbers() {
        let lexer = PythonLexer::new("42 3.14 0xFF 0o77 0b101");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn test_lexer_strings() {
        let lexer = PythonLexer::new("\"hello\" 'world' '''multi\nline'''");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() >= 4);
    }

    #[test]
    fn test_parse_function() {
        let source = "def add(a, b):\n    return a + b\n";
        let mut parser = PythonParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_class() {
        let source = "class MyClass:\n    def __init__(self):\n        self.x = 0\n";
        let mut parser = PythonParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_list_comprehension() {
        let source = "[x * 2 for x in range(10) if x % 2 == 0]";
        let mut parser = PythonParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_decorator() {
        let source = "@decorator\n@decorator(arg)\ndef func():\n    pass\n";
        let mut parser = PythonParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }
}
