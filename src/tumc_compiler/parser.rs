//! TUMC Parser - Recursive Descent Parser
//!
//! This module provides parsing for converting token streams
//! into Abstract Syntax Trees.
//!
//! # Grammar (Simplified)
//!
//! ```text
//! program     → item* EOF
//! item        → function | struct | enum | impl | trait | constant
//! function    → "fn" IDENT "(" params? ")" "->" type block
//! struct      → "struct" IDENT "{" field* "}"
//! enum        → "enum" IDENT "{" variant* "}"
//! statement   → "let" pattern (":" type)? "=" expr ";"
//!              | "if" expr block ("else" block)?
//!              | "while" expr block
//!              | "for" pattern "in" expr block
//!              | "return" expr? ";"
//!              | expr ";"
//! expr        → assignment
//! assignment  → logic_or ("=" assignment)?
//! logic_or    → logic_and ("||" logic_and)*
//! logic_and   → equality ("&&" equality)*
//! equality    → comparison (("==" | "!=") comparison)*
//! comparison  → term (("<" | ">" | "<=" | ">=") term)*
//! term        → factor (("+" | "-") factor)*
//! factor      → unary (("*" | "/" | "%") unary)*
//! unary       → ("-" | "!" | "*" | "&") unary
//!              | call
//! call        → primary ("(" arguments ")" | "." IDENT)*
//! primary     → NUMBER | STRING | IDENT | "(" expr ")" | block
//! ```

use crate::tumc_compiler::ast::*;
use crate::tumc_compiler::lexer::{Lexer, LexerError, Token, TokenKind};
use crate::tumc_compiler::SourceLocation;
use std::collections::VecDeque;

// ============================================================================
// Parser
// ============================================================================

/// Parser state
#[derive(Debug, Clone)]
pub struct Parser {
    /// Token queue
    tokens: VecDeque<Token>,

    /// Current token index
    current: usize,

    /// Errors collected
    errors: Vec<ParserError>,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: VecDeque::from(tokens),
            current: 0,
            errors: Vec::new(),
        }
    }

    /// Parse the tokens into a program
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut items = Vec::new();

        while !self.is_at_end() {
            match self.parse_item() {
                Ok(Some(item)) => items.push(item),
                Ok(None) => {}
                Err(e) => {
                    self.errors.push(e.clone());
                    // Skip to next item
                    self.synchronize();
                    if self.is_at_end() {
                        break;
                    }
                }
            }
        }

        if self.errors.is_empty() {
            Ok(Program::new(items))
        } else {
            Err(self.errors[0].clone())
        }
    }

    /// Parse a single item
    fn parse_item(&mut self) -> Result<Option<Item>, ParserError> {
        // Skip visibility modifier
        if self.check(&TokenKind::Pub) {
            self.advance();
        }

        // Check for module keyword
        if self.check(&TokenKind::Mod) {
            return self.parse_module();
        }

        // Check for use/import
        if self.check(&TokenKind::Use) {
            self.advance();
            // For now, skip use statements
            while !self.check(&TokenKind::Semicolon) && !self.is_at_end() {
                self.advance();
            }
            if !self.is_at_end() {
                self.advance();
            }
            return Ok(None);
        }

        // Check for function keyword
        if self.check(&TokenKind::Fn) {
            return self.parse_function().map(Some);
        }

        // Check for struct keyword
        if self.check(&TokenKind::Struct) {
            return self.parse_struct().map(Some);
        }

        // Check for enum keyword
        if self.check(&TokenKind::Enum) {
            return self.parse_enum().map(Some);
        }

        // Check for impl keyword
        if self.check(&TokenKind::Impl) {
            return self.parse_impl().map(Some);
        }

        // Check for trait keyword
        if self.check(&TokenKind::Trait) {
            return self.parse_trait().map(Some);
        }

        // Check for type keyword
        if self.check(&TokenKind::Type) {
            return self.parse_type_alias().map(Some);
        }

        // Check for const keyword
        if self.check_keyword("const") || self.check_keyword("let") {
            return self.parse_constant().map(Some);
        }

        // Skip other items we don't handle yet
        if self.peek().is_keyword() {
            self.advance();
            while !self.check(&TokenKind::Semicolon) && !self.is_at_end() {
                self.advance();
            }
            if !self.is_at_end() {
                self.advance();
            }
            return Ok(None);
        }

        // Not an item we recognize
        Ok(None)
    }

    /// Parse a function
    fn parse_function(&mut self) -> Result<Item, ParserError> {
        let fn_token = self.advance(); // Consume 'fn'

        let name = self.expect_identifier()?;
        let location = fn_token.location;

        // Parse generics
        let generics = self.parse_generics()?;

        // Parse parameters
        self.expect(&TokenKind::LParen, "'(' expected")?;
        let mut parameters = Vec::new();

        if !self.check(&TokenKind::RParen) {
            loop {
                let param_name = self.expect_identifier()?;
                self.expect(&TokenKind::Colon, "':' expected")?;

                let param_type = self.parse_type()?;

                parameters.push(Param {
                    name: param_name,
                    param_type,
                    is_mutable: false,
                    is_reference: false,
                    location: self.previous().location,
                });

                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RParen, "')' expected")?;

        // Parse return type
        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        // Parse body
        let body = if self.check(&TokenKind::LBrace) {
            self.parse_block()?
        } else if self.check(&TokenKind::Semicolon) {
            self.advance();
            Block::new(vec![], None)
        } else {
            return Err(self.error("Expected '{' or ';' after function signature"));
        };

        Ok(Item::Function(Function {
            name,
            generics,
            parameters,
            return_type,
            body,
            visibility: Visibility::Private,
            is_async: false,
            is_external: false,
            location: SourceSpan {
                start: location.start,
                end: self.previous().location.end,
            },
        }))
    }

    /// Parse generics
    fn parse_generics(&mut self) -> Result<Vec<GenericParam>, ParserError> {
        let mut generics = Vec::new();

        if self.check(&TokenKind::Lt) {
            self.advance();

            while !self.check(&TokenKind::Gt) && !self.is_at_end() {
                let name = self.expect_identifier()?;
                generics.push(GenericParam {
                    name,
                    bounds: Vec::new(),
                });

                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }

            self.expect(&TokenKind::Gt, "'>' expected")?;
        }

        Ok(generics)
    }

    /// Parse a type
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        let base_type = self.parse_type_no_function()?;

        // Check for function type
        if self.check(&TokenKind::LParen) {
            self.advance();
            let mut params = Vec::new();

            if !self.check(&TokenKind::RParen) {
                loop {
                    params.push(self.parse_type()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
            self.expect(&TokenKind::RParen, "')' expected")?;

            self.expect(&TokenKind::Arrow, "'->' expected")?;
            let return_type = Box::new(self.parse_type()?);

            return Ok(Type::Function(Box::new(FunctionType {
                parameters: params,
                return_type,
                variadic: false,
            })));
        }

        Ok(base_type)
    }

    /// Parse type without function
    fn parse_type_no_function(&mut self) -> Result<Type, ParserError> {
        let t = match self.peek().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                match name.as_str() {
                    "i8" => Type::I8,
                    "i16" => Type::I16,
                    "i32" => Type::I32,
                    "i64" => Type::I64,
                    "i128" => Type::I128,
                    "isize" => Type::Isize,
                    "u8" => Type::U8,
                    "u16" => Type::U16,
                    "u32" => Type::U32,
                    "u64" => Type::U64,
                    "u128" => Type::U128,
                    "usize" => Type::Usize,
                    "f32" => Type::F32,
                    "f64" => Type::F64,
                    "bool" => Type::Bool,
                    "char" => Type::Char,
                    "str" => Type::String,
                    "String" => Type::String,
                    "()" => Type::Unit,
                    _ => Type::Unknown, // Type reference
                }
            }
            TokenKind::Star => {
                self.advance();
                let inner = self.parse_type()?;
                Type::Ptr(Box::new(inner))
            }
            TokenKind::BitAnd => {
                self.advance();
                let mutable = self.check(&TokenKind::Mut);
                if mutable {
                    self.advance();
                }
                let inner = self.parse_type()?;
                if mutable {
                    Type::MutRef(Box::new(inner))
                } else {
                    Type::Ref(Box::new(inner))
                }
            }
            TokenKind::LBracket => {
                self.advance();
                let inner = self.parse_type()?;
                self.expect(&TokenKind::Semicolon, "';' expected")?;
                let size = if let TokenKind::Integer(n) = &self.peek().kind {
                    let n = *n as usize;
                    self.advance();
                    Some(n)
                } else {
                    None
                };
                self.expect(&TokenKind::RBracket, "']' expected")?;
                Type::Array(Box::new(inner), size)
            }
            TokenKind::LParen => {
                self.advance();
                if self.check(&TokenKind::RParen) {
                    self.advance();
                    return Ok(Type::Tuple(Vec::new()));
                }
                let first = self.parse_type()?;
                if self.check(&TokenKind::Comma) {
                    self.advance();
                    let mut types = vec![first];
                    while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                        types.push(self.parse_type()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RParen, "')' expected")?;
                    Type::Tuple(types)
                } else {
                    self.expect(&TokenKind::RParen, "')' expected")?;
                    Type::Function(Box::new(FunctionType {
                        parameters: vec![first],
                        return_type: Box::new(Type::Unit),
                        variadic: false,
                    }))
                }
            }
            _ => Type::Unknown,
        };

        Ok(t)
    }

    /// Parse a struct
    fn parse_struct(&mut self) -> Result<Item, ParserError> {
        let struct_token = self.advance();
        let name = self.expect_identifier()?;
        let generics = self.parse_generics()?;

        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            // Skip visibility
            if self.check(&TokenKind::Pub) {
                self.advance();
            }

            let field_name = self.expect_identifier()?;
            self.expect(&TokenKind::Colon, "':' expected")?;
            let field_type = self.parse_type()?;

            fields.push(Field {
                name: field_name,
                field_type,
                visibility: Visibility::Private,
                location: self.previous().location,
            });

            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Item::Struct(Struct {
            name,
            generics,
            fields,
            location: SourceSpan {
                start: struct_token.location.start,
                end: self.previous().location.end,
            },
        }))
    }

    /// Parse an enum
    fn parse_enum(&mut self) -> Result<Item, ParserError> {
        let enum_token = self.advance();
        let name = self.expect_identifier()?;
        let generics = self.parse_generics()?;

        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut variants = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let variant_name = self.expect_identifier()?;

            let mut fields = Vec::new();
            if self.check(&TokenKind::LParen) {
                self.advance();
                while !self.check(&TokenKind::RParen) && !self.is_at_end() {
                    fields.push(self.parse_type()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(&TokenKind::RParen, "')' expected")?;
            }

            variants.push(EnumVariant {
                name: variant_name,
                fields,
                location: self.previous().location,
            });

            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Item::Enum(Enum {
            name,
            generics,
            variants,
            location: SourceSpan {
                start: enum_token.location.start,
                end: self.previous().location.end,
            },
        }))
    }

    /// Parse an impl block
    fn parse_impl(&mut self) -> Result<Item, ParserError> {
        let impl_token = self.advance();
        let generics = self.parse_generics()?;

        // Parse trait name (optional)
        let trait_name = if self.check_identifier("for") {
            Some(self.previous().literal.clone().unwrap_or_default())
        } else {
            None
        };

        let for_type = self.parse_type()?;

        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut items = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            if self.check(&TokenKind::Fn) {
                items.push(ImplItem::Function(self.parse_function_impl()?));
            } else if self.check_keyword("const") || self.check_keyword("let") {
                items.push(ImplItem::Constant(self.parse_constant_impl()?));
            } else {
                self.advance();
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Item::Impl(Impl {
            name: trait_name,
            generics,
            trait_name: None,
            for_type,
            items,
            location: SourceSpan {
                start: impl_token.location.start,
                end: self.previous().location.end,
            },
        }))
    }

    /// Parse a trait
    fn parse_trait(&mut self) -> Result<Item, ParserError> {
        let trait_token = self.advance();
        let name = self.expect_identifier()?;
        let generics = self.parse_generics()?;

        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut items = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            if self.check(&TokenKind::Fn) {
                self.advance();
                let fn_name = self.expect_identifier()?;
                self.expect(&TokenKind::LParen, "'(' expected")?;

                let mut params = Vec::new();
                if !self.check(&TokenKind::RParen) {
                    loop {
                        let param_name = self.expect_identifier()?;
                        self.expect(&TokenKind::Colon, "':' expected")?;
                        let param_type = self.parse_type()?;
                        params.push((param_name, param_type));
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RParen, "')' expected")?;

                let return_type = if self.check(&TokenKind::Arrow) {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };

                self.expect(&TokenKind::Semicolon, "';' expected")?;

                items.push(TraitItem::Function {
                    name: fn_name,
                    sig: FunctionSig {
                        generics: Vec::new(),
                        parameters: params,
                        return_type,
                    },
                });
            } else {
                self.advance();
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Item::Trait(Trait {
            name,
            generics,
            items,
            location: SourceSpan {
                start: trait_token.location.start,
                end: self.previous().location.end,
            },
        }))
    }

    /// Parse a type alias
    fn parse_type_alias(&mut self) -> Result<Item, ParserError> {
        let type_token = self.advance();
        let name = self.expect_identifier()?;
        let generics = self.parse_generics()?;

        self.expect(&TokenKind::Assign, "'=' expected")?;
        let aliased_type = self.parse_type()?;
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Item::TypeAlias(TypeAlias {
            name,
            generics,
            aliased_type,
            location: SourceSpan {
                start: type_token.location.start,
                end: self.previous().location.end,
            },
        }))
    }

    /// Parse a constant
    fn parse_constant(&mut self) -> Result<Item, ParserError> {
        let const_token = self.advance();
        let name = self.expect_identifier()?;

        let const_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(&TokenKind::Assign, "'=' expected")?;
        let value = self.parse_expression()?;
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Item::Constant(Constant {
            name,
            const_type: const_type.unwrap_or(Type::Unknown),
            value,
            location: SourceSpan {
                start: const_token.location.start,
                end: self.previous().location.end,
            },
        }))
    }

    /// Parse a module
    fn parse_module(&mut self) -> Result<Option<Item>, ParserError> {
        self.advance(); // consume 'mod'
        let name = self.expect_identifier()?;
        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut items = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            match self.parse_item() {
                Ok(Some(item)) => items.push(item),
                Ok(None) => {}
                Err(_) => {
                    self.synchronize();
                }
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Some(Item::Module(Module {
            name,
            items,
            location: self.previous().location,
        })))
    }

    /// Parse a function inside impl
    fn parse_function_impl(&mut self) -> Result<Function, ParserError> {
        self.expect(&TokenKind::Fn, "'fn' expected")?;
        let name = self.expect_identifier()?;

        let generics = self.parse_generics()?;

        self.expect(&TokenKind::LParen, "'(' expected")?;
        let mut parameters = Vec::new();

        if !self.check(&TokenKind::RParen) {
            loop {
                let param_name = self.expect_identifier()?;
                self.expect(&TokenKind::Colon, "':' expected")?;
                let param_type = self.parse_type()?;
                parameters.push(Param {
                    name: param_name,
                    param_type,
                    is_mutable: false,
                    is_reference: false,
                    location: self.previous().location,
                });

                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RParen, "')' expected")?;

        let return_type = if self.check(&TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = if self.check(&TokenKind::LBrace) {
            self.parse_block()?
        } else {
            self.expect(&TokenKind::Semicolon, "';' expected")?;
            Block::new(vec![], None)
        };

        Ok(Function {
            name,
            generics,
            parameters,
            return_type,
            body,
            visibility: Visibility::Private,
            is_async: false,
            is_external: false,
            location: self.previous().location,
        })
    }

    /// Parse a constant inside impl
    fn parse_constant_impl(&mut self) -> Result<Constant, ParserError> {
        self.advance(); // consume 'const' or 'let'
        let name = self.expect_identifier()?;

        let const_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(&TokenKind::Assign, "'=' expected")?;
        let value = self.parse_expression()?;
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Constant {
            name,
            const_type: const_type.unwrap_or(Type::Unknown),
            value,
            location: self.previous().location,
        })
    }

    // ============================================================================
    // Statement Parsing
    // ============================================================================

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<Stmt, ParserError> {
        if self.check(&TokenKind::Let) || self.check_keyword("let") {
            return self.parse_let_statement();
        }

        if self.check(&TokenKind::If) {
            return self.parse_if_statement();
        }

        if self.check(&TokenKind::While) {
            return self.parse_while_statement();
        }

        if self.check(&TokenKind::For) {
            return self.parse_for_statement();
        }

        if self.check(&TokenKind::Loop) {
            return self.parse_loop_statement();
        }

        if self.check(&TokenKind::Return) {
            return self.parse_return_statement();
        }

        if self.check(&TokenKind::Break) {
            return self.parse_break_statement();
        }

        if self.check(&TokenKind::Continue) {
            return self.parse_continue_statement();
        }

        if self.check(&TokenKind::Match) {
            return self.parse_match_statement();
        }

        // Expression statement
        let expr = self.parse_expression()?;

        // Check for assignment
        if self.check(&TokenKind::Assign) {
            self.advance();
            let value = self.parse_expression()?;
            self.expect(&TokenKind::Semicolon, "';' expected")?;
            return Ok(Stmt::Assign {
                target: expr,
                value,
                location: SourceSpan {
                    start: expr.location.start,
                    end: value.location.end,
                },
            });
        }

        self.expect(&TokenKind::Semicolon, "';' expected")?;
        Ok(Stmt::Expr {
            expr,
            location: expr.location,
        })
    }

    /// Parse a let statement
    fn parse_let_statement(&mut self) -> Result<Stmt, ParserError> {
        let let_token = self.advance();
        let pattern = self.parse_pattern()?;

        let let_type = if self.check(&TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let initializer = if self.check(&TokenKind::Assign) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Stmt::Let {
            pattern,
            let_type,
            initializer,
            location: SourceSpan {
                start: let_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a pattern
    fn parse_pattern(&mut self) -> Result<Pattern, ParserError> {
        let token = self.peek();

        match &token.kind {
            TokenKind::Underscore => {
                self.advance();
                Ok(Pattern::Wildcard(token.location))
            }
            TokenKind::Identifier(name) => {
                self.advance();
                // Check for mut
                let mutable = if self.check(&TokenKind::Mut) {
                    self.advance();
                    true
                } else {
                    false
                };
                Ok(Pattern::Variable {
                    name: name.clone(),
                    mutable,
                    location: token.location,
                })
            }
            TokenKind::LParen => {
                self.advance();
                let mut patterns = Vec::new();
                if !self.check(&TokenKind::RParen) {
                    loop {
                        patterns.push(self.parse_pattern()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RParen, "')' expected")?;
                Ok(Pattern::Tuple(patterns, token.location))
            }
            TokenKind::LBracket => {
                self.advance();
                let mut patterns = Vec::new();
                while !self.check(&TokenKind::RBracket) && !self.is_at_end() {
                    patterns.push(self.parse_pattern()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.expect(&TokenKind::RBracket, "']' expected")?;
                Ok(Pattern::Array(patterns, token.location))
            }
            _ => Ok(Pattern::Wildcard(token.location)),
        }
    }

    /// Parse an if statement
    fn parse_if_statement(&mut self) -> Result<Stmt, ParserError> {
        let if_token = self.advance();
        let condition = self.parse_expression()?;
        let then_block = self.parse_block()?;

        let else_block = if self.check(&TokenKind::Else) {
            self.advance();
            if self.check(&TokenKind::If) {
                Some(Box::new(self.parse_if_statement()?))
            } else {
                Some(Box::new(Stmt::Block {
                    block: self.parse_block()?,
                    location: self.previous().location,
                }))
            }
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_block,
            else_block,
            location: SourceSpan {
                start: if_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a while statement
    fn parse_while_statement(&mut self) -> Result<Stmt, ParserError> {
        let while_token = self.advance();
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;

        Ok(Stmt::While {
            condition,
            body,
            label: None,
            location: SourceSpan {
                start: while_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a for statement
    fn parse_for_statement(&mut self) -> Result<Stmt, ParserError> {
        let for_token = self.advance();
        let pattern = self.parse_pattern()?;
        self.expect_keyword("in", "'in' expected")?;
        let iterator = self.parse_expression()?;
        let body = self.parse_block()?;

        Ok(Stmt::For {
            pattern,
            iterator,
            body,
            label: None,
            location: SourceSpan {
                start: for_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a loop statement
    fn parse_loop_statement(&mut self) -> Result<Stmt, ParserError> {
        let loop_token = self.advance();
        let body = self.parse_block()?;

        Ok(Stmt::Loop {
            body,
            label: None,
            location: SourceSpan {
                start: loop_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a return statement
    fn parse_return_statement(&mut self) -> Result<Stmt, ParserError> {
        let return_token = self.advance();

        let value = if self.check(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Stmt::Return {
            value,
            location: SourceSpan {
                start: return_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a break statement
    fn parse_break_statement(&mut self) -> Result<Stmt, ParserError> {
        let break_token = self.advance();
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Stmt::Break {
            label: None,
            location: SourceSpan {
                start: break_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a continue statement
    fn parse_continue_statement(&mut self) -> Result<Stmt, ParserError> {
        let continue_token = self.advance();
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Stmt::Continue {
            label: None,
            location: SourceSpan {
                start: continue_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    /// Parse a match statement
    fn parse_match_statement(&mut self) -> Result<Stmt, ParserError> {
        let match_token = self.advance();
        let value = self.parse_expression()?;
        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut arms = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            let guard = if self.check_keyword("if") {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.expect(&TokenKind::FatArrow, "'=>' expected")?;
            let body = self.parse_expression()?;
            if self.check(&TokenKind::Comma) {
                self.advance();
            }

            arms.push(MatchArm {
                pattern,
                guard,
                body,
                location: self.previous().location,
            });
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Stmt::Match {
            value,
            arms,
            location: SourceSpan {
                start: match_token.location.start,
                end: self.previous().location.end,
            },
        })
    }

    // ============================================================================
    // Expression Parsing (Pratt Parser)
    // ============================================================================

    /// Parse an expression using Pratt parsing
    fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        self.parse_assignment()
    }

    /// Parse assignment expression
    fn parse_assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.parse_or()?;

        if self.check(&TokenKind::Assign) {
            self.advance();
            let value = Box::new(self.parse_assignment()?);
            return Ok(Expr::new(
                ExprKind::Assign {
                    target: Box::new(expr),
                    value,
                },
                expr.location,
            ));
        }

        // Compound assignment
        let compound_op = match &self.peek().kind {
            TokenKind::PlusAssign => Some(BinOp::Add),
            TokenKind::MinusAssign => Some(BinOp::Sub),
            TokenKind::StarAssign => Some(BinOp::Mul),
            TokenKind::SlashAssign => Some(BinOp::Div),
            _ => None,
        };

        if let Some(op) = compound_op {
            self.advance();
            let value = Box::new(self.parse_assignment()?);
            return Ok(Expr::new(
                ExprKind::CompoundAssign {
                    target: Box::new(expr),
                    op,
                    value,
                },
                expr.location,
            ));
        }

        Ok(expr)
    }

    /// Parse logical OR
    fn parse_or(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_and()?;

        while self.check(&TokenKind::Or) || self.check(&TokenKind::Or) {
            let op = self.advance();
            let rhs = Box::new(self.parse_and()?);
            expr = Expr::new(
                ExprKind::Binary {
                    op: BinOp::Or,
                    lhs: Box::new(expr),
                    rhs,
                },
                SourceSpan {
                    start: expr.location.start,
                    end: rhs.location.end,
                },
            );
        }

        Ok(expr)
    }

    /// Parse logical AND
    fn parse_and(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_equality()?;

        while self.check(&TokenKind::And) || self.check(&TokenKind::And) {
            self.advance();
            let rhs = Box::new(self.parse_equality()?);
            expr = Expr::new(
                ExprKind::Binary {
                    op: BinOp::And,
                    lhs: Box::new(expr),
                    rhs,
                },
                SourceSpan {
                    start: expr.location.start,
                    end: rhs.location.end,
                },
            );
        }

        Ok(expr)
    }

    /// Parse equality
    fn parse_equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_comparison()?;

        while let TokenKind::Eq | TokenKind::Ne = &self.peek().kind {
            let op = match self.peek().kind {
                TokenKind::Eq => BinOp::Eq,
                TokenKind::Ne => BinOp::Ne,
                _ => unreachable!(),
            };
            self.advance();
            let rhs = Box::new(self.parse_comparison()?);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                SourceSpan {
                    start: expr.location.start,
                    end: rhs.location.end,
                },
            );
        }

        Ok(expr)
    }

    /// Parse comparison
    fn parse_comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_term()?;

        while let TokenKind::Lt | TokenKind::Le | TokenKind::Gt | TokenKind::Ge = &self.peek().kind
        {
            let op = match self.peek().kind {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Le => BinOp::Le,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::Ge => BinOp::Ge,
                _ => unreachable!(),
            };
            self.advance();
            let rhs = Box::new(self.parse_term()?);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                SourceSpan {
                    start: expr.location.start,
                    end: rhs.location.end,
                },
            );
        }

        Ok(expr)
    }

    /// Parse term (+ -)
    fn parse_term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_factor()?;

        while let TokenKind::Plus | TokenKind::Minus = &self.peek().kind {
            let op = match self.peek().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let rhs = Box::new(self.parse_factor()?);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                SourceSpan {
                    start: expr.location.start,
                    end: rhs.location.end,
                },
            );
        }

        Ok(expr)
    }

    /// Parse factor (* / %)
    fn parse_factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_unary()?;

        while let TokenKind::Star | TokenKind::Slash | TokenKind::Percent = &self.peek().kind {
            let op = match self.peek().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Rem,
                _ => unreachable!(),
            };
            self.advance();
            let rhs = Box::new(self.parse_unary()?);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                SourceSpan {
                    start: expr.location.start,
                    end: rhs.location.end,
                },
            );
        }

        Ok(expr)
    }

    /// Parse unary
    fn parse_unary(&mut self) -> Result<Expr, ParserError> {
        if let TokenKind::Minus | TokenKind::Not | TokenKind::BitAnd | TokenKind::Star = &self.peek().kind
        {
            let op = match self.peek().kind {
                TokenKind::Minus => UnOp::Neg,
                TokenKind::Not => UnOp::Not,
                TokenKind::BitAnd => UnOp::Deref,
                TokenKind::Star => UnOp::Deref,
                _ => unreachable!(),
            };
            self.advance();
            let operand = Box::new(self.parse_unary()?);
            return Ok(Expr::new(
                ExprKind::Unary { op, operand },
                self.previous().location,
            ));
        }

        self.parse_call()
    }

    /// Parse call
    fn parse_call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&TokenKind::LParen) {
                self.advance();
                let mut arguments = Vec::new();
                if !self.check(&TokenKind::RParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RParen, "')' expected")?;
                expr = Expr::new(
                    ExprKind::Call {
                        function: Box::new(expr),
                        arguments,
                    },
                    self.previous().location,
                );
            } else if self.check(&TokenKind::Dot) {
                self.advance();
                let method = self.expect_identifier()?;
                self.expect(&TokenKind::LParen, "'(' expected")?;
                let mut arguments = Vec::new();
                if !self.check(&TokenKind::RParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RParen, "')' expected")?;
                expr = Expr::new(
                    ExprKind::MethodCall {
                        receiver: Box::new(expr),
                        method,
                        arguments,
                    },
                    self.previous().location,
                );
            } else if self.check(&TokenKind::LBracket) {
                self.advance();
                let index = self.parse_expression()?;
                self.expect(&TokenKind::RBracket, "']' expected")?;
                expr = Expr::new(
                    ExprKind::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    },
                    self.previous().location,
                );
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parse primary
    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.peek();

        match &token.kind {
            // Literals
            TokenKind::Integer(n) => {
                self.advance();
                Ok(Expr::new(ExprKind::Literal(Literal::Int(*n)), token.location))
            }
            TokenKind::Float(n) => {
                self.advance();
                Ok(Expr::new(ExprKind::Literal(Literal::Float(*n)), token.location))
            }
            TokenKind::String(s) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Literal(Literal::String(s.clone())),
                    token.location,
                ))
            }
            TokenKind::Char(c) => {
                self.advance();
                Ok(Expr::new(ExprKind::Literal(Literal::Char(*c)), token.location))
            }
            TokenKind::Bool(b) => {
                self.advance();
                Ok(Expr::new(ExprKind::Literal(Literal::Bool(*b)), token.location))
            }

            // Identifier
            TokenKind::Identifier(name) => {
                self.advance();
                Ok(Expr::new(ExprKind::Identifier(name.clone()), token.location))
            }

            // Grouping
            TokenKind::LParen => {
                self.advance();
                if self.check(&TokenKind::RParen) {
                    self.advance();
                    return Ok(Expr::new(ExprKind::Literal(Literal::Unit), token.location));
                }
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RParen, "')' expected")?;
                Ok(Expr::new(ExprKind::Grouped(Box::new(expr)), token.location))
            }

            // Block
            TokenKind::LBrace => Ok(Expr::new(ExprKind::Block(self.parse_block()?), token.location)),

            // If expression
            TokenKind::If => {
                self.advance();
                let condition = Box::new(self.parse_expression()?);
                let then_block = self.parse_block()?;
                let else_expr = if self.check(&TokenKind::Else) {
                    self.advance();
                    Some(Box::new(if self.check(&TokenKind::If) {
                        Expr::new(
                            ExprKind::If {
                                condition: Box::new(self.parse_expression()?),
                                then_block: self.parse_block()?,
                                else_expr: None,
                            },
                            self.previous().location,
                        )
                    } else {
                        Expr::new(
                            ExprKind::Block(self.parse_block()?),
                            self.previous().location,
                        )
                    }))
                } else {
                    None
                };
                Ok(Expr::new(
                    ExprKind::If {
                        condition,
                        then_block,
                        else_expr,
                    },
                    token.location,
                ))
            }

            // Loop
            TokenKind::Loop => {
                self.advance();
                Ok(Expr::new(ExprKind::Loop(Box::new(self.parse_block()?)), token.location))
            }

            // While
            TokenKind::While => {
                self.advance();
                let condition = Box::new(self.parse_expression()?);
                let body = Box::new(self.parse_block()?);
                Ok(Expr::new(ExprKind::While { condition, body }, token.location))
            }

            // Match
            TokenKind::Match => {
                self.advance();
                let value = Box::new(self.parse_expression()?);
                self.expect(&TokenKind::LBrace, "'{' expected")?;
                let mut arms = Vec::new();
                while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
                    let pattern = self.parse_pattern()?;
                    let guard = if self.check_keyword("if") {
                        self.advance();
                        Some(self.parse_expression()?)
                    } else {
                        None
                    };
                    self.expect(&TokenKind::FatArrow, "'=>' expected")?;
                    let body = self.parse_expression()?;
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    }
                    arms.push(MatchArm {
                        pattern,
                        guard,
                        body,
                        location: self.previous().location,
                    });
                }
                self.expect(&TokenKind::RBrace, "'}' expected")?;
                Ok(Expr::new(ExprKind::Match { value, arms }, token.location))
            }

            // Return
            TokenKind::Return => {
                self.advance();
                let value = if self.check(&TokenKind::Semicolon) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                Ok(Expr::new(ExprKind::Return(value.map(Box::new)), token.location))
            }

            // Break
            TokenKind::Break => {
                self.advance();
                let value = if self.check(&TokenKind::Semicolon) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                Ok(Expr::new(
                    ExprKind::Break {
                        label: None,
                        value: value.map(Box::new),
                    },
                    token.location,
                ))
            }

            // Continue
            TokenKind::Continue => {
                self.advance();
                Ok(Expr::new(ExprKind::Continue { label: None }, token.location))
            }

            // Array literal
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                if !self.check(&TokenKind::RBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RBracket, "']' expected")?;
                Ok(Expr::new(ExprKind::Array(elements), token.location))
            }

            _ => Err(self.error(&format!("Unexpected token: {:?}", token.kind))),
        }
    }

    /// Parse a block
    fn parse_block(&mut self) -> Result<Block, ParserError> {
        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut statements = Vec::new();

        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Block::new(statements, None))
    }

    // ============================================================================
    // Helper Methods
    // ============================================================================

    /// Check if at end
    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    /// Peek at current token
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::new(
            TokenKind::Eof,
            SourceLocation { line: 0, column: 0, offset: 0 },
        ))
    }

    /// Get previous token
    fn previous(&self) -> &Token {
        &self.tokens.get(self.current - 1).unwrap_or(self.peek())
    }

    /// Advance to next token
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Check token kind
    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    /// Check identifier matches
    fn check_identifier(&self, name: &str) -> bool {
        if let TokenKind::Identifier(n) = &self.peek().kind {
            n == name
        } else {
            false
        }
    }

    /// Check keyword
    fn check_keyword(&self, keyword: &str) -> bool {
        if let TokenKind::Identifier(n) = &self.peek().kind {
            match n.as_str() {
                "let" | "const" | "if" | "else" | "while" | "for" | "loop" | "return"
                | "break" | "continue" | "fn" | "struct" | "enum" | "impl" | "trait"
                | "type" | "pub" | "mod" | "use" | "as" | "match" | "async" | "await"
                | "mut" | "ref" | "in" | "true" | "false" => n == keyword,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Expect a token
    fn expect(&mut self, kind: &TokenKind, message: &str) -> Result<(), ParserError> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    /// Expect identifier
    fn expect_identifier(&mut self) -> Result<String, ParserError> {
        if let TokenKind::Identifier(name) = &self.peek().kind.clone() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(self.error("Expected identifier"))
        }
    }

    /// Expect keyword
    fn expect_keyword(&mut self, keyword: &str, message: &str) -> Result<(), ParserError> {
        if self.check_keyword(keyword) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    /// Create error
    fn error(&self, message: &str) -> ParserError {
        ParserError {
            message: message.to_string(),
            location: self.peek().location,
        }
    }

    /// Synchronize to next item
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon {
                return;
            }

            match &self.peek().kind {
                TokenKind::Fn
                | TokenKind::Struct
                | TokenKind::Enum
                | TokenKind::Impl
                | TokenKind::Trait
                | TokenKind::Type
                | TokenKind::Mod => return,
                _ => {}
            }

            self.advance();
        }
    }
}

// ============================================================================
// Parser Error
// ============================================================================

/// Parser error
#[derive(Debug, Clone)]
pub struct ParserError {
    pub message: String,
    pub location: SourceLocation,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse error at {}:{}: {}",
            self.location.line, self.location.column, self.message
        )
    }
}

impl std::error::Error for ParserError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Program {
        let tokens = Lexer::new(source).tokenize().unwrap();
        Parser::new(tokens).parse().unwrap()
    }

    #[test]
    fn test_parse_function() {
        let program = parse("fn add(a: i32, b: i32) -> i32 { a + b }");
        assert_eq!(program.items.len(), 1);
        assert!(matches!(&program.items[0], Item::Function(_)));
    }

    #[test]
    fn test_parse_struct() {
        let program = parse("struct Point { x: i32, y: i32 }");
        assert_eq!(program.items.len(), 1);
        assert!(matches!(&program.items[0], Item::Struct(_)));
    }

    #[test]
    fn test_parse_enum() {
        let program = parse("enum Color { Red, Green, Blue }");
        assert_eq!(program.items.len(), 1);
        assert!(matches!(&program.items[0], Item::Enum(_)));
    }

    #[test]
    fn test_parse_let() {
        let program = parse("fn main() { let x = 42; }");
        if let Item::Function(f) = &program.items[0] {
            assert!(matches!(&f.body.statements[0], Stmt::Let { .. }));
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_parse_if() {
        let program = parse("fn test() { if true { 1 } else { 2 } }");
        if let Item::Function(f) = &program.items[0] {
            assert!(matches!(&f.body.statements[0], Stmt::If { .. }));
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_parse_while() {
        let program = parse("fn test() { while true { break; } }");
        if let Item::Function(f) = &program.items[0] {
            assert!(matches!(&f.body.statements[0], Stmt::While { .. }));
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_parse_for() {
        let program = parse("fn test() { for i in 0..10 { } }");
        if let Item::Function(f) = &program.items[0] {
            assert!(matches!(&f.body.statements[0], Stmt::For { .. }));
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_parse_return() {
        let program = parse("fn test() { return 42; }");
        if let Item::Function(f) = &program.items[0] {
            assert!(matches!(&f.body.statements[0], Stmt::Return { .. }));
        } else {
            panic!("Expected function");
        }
    }

    #[test]
    fn test_parse_binary_expr() {
        let program = parse("fn test() { let x = 1 + 2 * 3; }");
        if let Item::Function(f) = &program.items[0] {
            if let Stmt::Let { initializer: Some(expr), .. } = &f.body.statements[0] {
                assert!(matches!(&expr.kind, ExprKind::Binary { op: BinOp::Add, .. }));
            } else {
                panic!("Expected let with initializer");
            }
        } else {
            panic!("Expected function");
        }
    }
}
