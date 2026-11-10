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
        let start = fn_token.location;

        // Parse generics
        let generics = self.parse_generics()?;

        // Parse parameters
        self.expect(&TokenKind::LParen, "'(' expected")?;
        let mut params = Vec::new();

        if !self.check(&TokenKind::RParen) {
            loop {
                let param_name = self.expect_identifier()?;
                self.expect(&TokenKind::Colon, "':' expected")?;

                let param_type = self.parse_type()?;

                params.push(FunctionParam {
                    name: param_name,
                    ann_type: param_type,
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
            self.parse_type()?
        } else {
            AnnType::Unit
        };

        // Parse body
        let body = if self.check(&TokenKind::LBrace) {
            self.parse_block()?.statements
        } else if self.check(&TokenKind::Semicolon) {
            self.advance();
            Vec::new()
        } else {
            return Err(self.error("Expected '{' or ';' after function signature"));
        };

        Ok(Item::Function(Function {
            name,
            generics,
            params,
            return_type,
            body,
            visibility: Visibility::Private,
            is_async: false,
            span: SourceSpan::new(start, self.previous().location),
            ..Default::default()
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
    fn parse_type(&mut self) -> Result<AnnType, ParserError> {
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

            return Ok(AnnType::Func(params, return_type));
        }

        Ok(base_type)
    }

    /// Parse type without function
    fn parse_type_no_function(&mut self) -> Result<AnnType, ParserError> {
        let t = match self.peek().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                match name.as_str() {
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32"
                    | "u64" | "u128" | "usize" => AnnType::Int,
                    "f32" | "f64" => AnnType::Float,
                    "bool" => AnnType::Bool,
                    "char" => AnnType::Char,
                    "str" | "String" => AnnType::String,
                    "()" => AnnType::Unit,
                    _ => AnnType::Named(name), // Type reference
                }
            }
            TokenKind::Star => {
                self.advance();
                let inner = self.parse_type()?;
                AnnType::Ptr(Box::new(inner))
            }
            TokenKind::BitAnd => {
                self.advance();
                let mutable = self.check(&TokenKind::Mut);
                if mutable {
                    self.advance();
                }
                let inner = self.parse_type()?;
                if mutable {
                    AnnType::MutRef(Box::new(inner))
                } else {
                    AnnType::Ref(Box::new(inner))
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
                let _ = size;
                AnnType::Array(Box::new(inner))
            }
            TokenKind::LParen => {
                self.advance();
                if self.check(&TokenKind::RParen) {
                    self.advance();
                    return Ok(AnnType::Tuple(Vec::new()));
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
                    AnnType::Tuple(types)
                } else {
                    self.expect(&TokenKind::RParen, "')' expected")?;
                    AnnType::Func(vec![first], Box::new(AnnType::Unit))
                }
            }
            _ => AnnType::Unit,
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
                ann_type: field_type,
                span: SourceSpan::at(self.previous().location),
                visibility: Visibility::Private,
                ..Default::default()
            });

            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Item::Struct(Struct {
            name,
            ty_params: generics.iter().map(|g| g.name.clone()).collect(),
            generics,
            fields,
            span: SourceSpan::new(struct_token.location, self.previous().location),
            ..Default::default()
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

            let payload = match fields.len() {
                0 => None,
                1 => Some(fields.into_iter().next().expect("checked length")),
                _ => Some(AnnType::Tuple(fields)),
            };

            variants.push(EnumVariant {
                name: variant_name,
                fields: payload,
                span: SourceSpan::at(self.previous().location),
                ..Default::default()
            });

            if self.check(&TokenKind::Comma) {
                self.advance();
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Item::Enum(Enum {
            name,
            ty_params: generics.iter().map(|g| g.name.clone()).collect(),
            generics,
            variants,
            span: SourceSpan::new(enum_token.location, self.previous().location),
            ..Default::default()
        }))
    }

    /// Parse an impl block
    fn parse_impl(&mut self) -> Result<Item, ParserError> {
        let impl_token = self.advance();
        let generics = self.parse_generics()?;

        // Parse the optional trait name: `impl Trait for Type { .. }`
        let trait_name = if self.peek_is_identifier() && self.peek_at(1).kind == TokenKind::For {
            let name = self.expect_identifier()?;
            self.expect_keyword("for", "'for' expected")?;
            Some(name)
        } else {
            None
        };

        let type_name = self.parse_type_name()?;

        self.expect(&TokenKind::LBrace, "'{' expected")?;

        let mut functions = Vec::new();
        let mut constants = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
            if self.check(&TokenKind::Fn) {
                functions.push(self.parse_function_impl()?);
            } else if self.check_keyword("const") || self.check_keyword("let") {
                constants.push(self.parse_constant_impl()?);
            } else {
                self.advance();
            }
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        Ok(Item::Impl(ImplBlock {
            trait_name,
            type_name,
            ty_params: generics.iter().map(|g| g.name.clone()).collect(),
            functions,
            constants,
            span: SourceSpan::new(impl_token.location, self.previous().location),
        }))
    }

    /// Parse a type name (an identifier, optionally generic)
    fn parse_type_name(&mut self) -> Result<String, ParserError> {
        let name = self.expect_identifier()?;
        // Skip generic arguments on the type name, e.g. `Vec<i32>`
        if self.check(&TokenKind::Lt) {
            let mut depth = 0usize;
            loop {
                if self.is_at_end() {
                    break;
                }
                if self.check(&TokenKind::Lt) {
                    depth += 1;
                } else if self.check(&TokenKind::Gt) {
                    depth -= 1;
                }
                self.advance();
                if depth == 0 {
                    break;
                }
            }
        }
        Ok(name)
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
            span: SourceSpan::new(trait_token.location, self.previous().location),
            ..Default::default()
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
            ty_params: generics.iter().map(|g| g.name.clone()).collect(),
            generics,
            ann_type: aliased_type,
            span: SourceSpan::new(type_token.location, self.previous().location),
            ..Default::default()
        }))
    }

    /// Parse a constant
    fn parse_constant(&mut self) -> Result<Item, ParserError> {
        let const_token = self.advance();
        let name = self.expect_identifier()?;

        let const_type = if self.check(&TokenKind::Colon) {
            self.advance();
            self.parse_type()?
        } else {
            AnnType::Unit
        };

        self.expect(&TokenKind::Assign, "'=' expected")?;
        let value = self.parse_expression()?;
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Item::Constant(Constant {
            name,
            ann_type: const_type,
            value,
            span: SourceSpan::new(const_token.location, self.previous().location),
            visibility: Visibility::Private,
            ..Default::default()
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
            span: SourceSpan::at(self.previous().location),
            visibility: Visibility::Private,
            ..Default::default()
        })))
    }

    /// Parse a function inside impl
    fn parse_function_impl(&mut self) -> Result<Function, ParserError> {
        self.expect(&TokenKind::Fn, "'fn' expected")?;
        let name = self.expect_identifier()?;

        let generics = self.parse_generics()?;

        self.expect(&TokenKind::LParen, "'(' expected")?;
        let mut params = Vec::new();

        if !self.check(&TokenKind::RParen) {
            loop {
                let param_name = self.expect_identifier()?;
                self.expect(&TokenKind::Colon, "':' expected")?;
                let param_type = self.parse_type()?;
                params.push(FunctionParam {
                    name: param_name,
                    ann_type: param_type,
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
            self.parse_type()?
        } else {
            AnnType::Unit
        };

        let body = if self.check(&TokenKind::LBrace) {
            self.parse_block()?.statements
        } else {
            self.expect(&TokenKind::Semicolon, "';' expected")?;
            Vec::new()
        };

        Ok(Function {
            name,
            generics,
            params,
            return_type,
            body,
            visibility: Visibility::Private,
            is_async: false,
            span: SourceSpan::at(self.previous().location),
            ..Default::default()
        })
    }

    /// Parse a constant inside impl
    fn parse_constant_impl(&mut self) -> Result<Constant, ParserError> {
        self.advance(); // consume 'const' or 'let'
        let name = self.expect_identifier()?;

        let const_type = if self.check(&TokenKind::Colon) {
            self.advance();
            self.parse_type()?
        } else {
            AnnType::Unit
        };

        self.expect(&TokenKind::Assign, "'=' expected")?;
        let value = self.parse_expression()?;
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Constant {
            name,
            ann_type: const_type,
            value,
            span: SourceSpan::at(self.previous().location),
            visibility: Visibility::Private,
            ..Default::default()
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
            let span = SourceSpan::new(expr.span.start, value.span.end);
            return Ok(Stmt::new(
                StmtKind::Assignment(Box::new(expr), Box::new(value)),
                span,
            ));
        }

        self.expect(&TokenKind::Semicolon, "';' expected")?;
        let span = expr.span;
        Ok(Stmt::new(StmtKind::Expr(expr), span))
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

        Ok(Stmt::new(
            StmtKind::Let(pattern, let_type, initializer, Vec::new()),
            SourceSpan::new(let_token.location, self.previous().location),
        ))
    }

    /// Parse a pattern
    fn parse_pattern(&mut self) -> Result<Pattern, ParserError> {
        let token = self.peek().clone();

        match &token.kind {
            TokenKind::Underscore => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            TokenKind::Identifier(name) => {
                self.advance();
                // Check for mut
                if self.check(&TokenKind::Mut) {
                    self.advance();
                }
                Ok(Pattern::Binding(name.clone()))
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
                Ok(Pattern::Tuple(patterns))
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
                Ok(Pattern::Slice(patterns))
            }
            _ => {
                let _ = token;
                Ok(Pattern::Wildcard)
            }
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
                // `else if` is modelled as a nested `if` statement in the else branch.
                let nested = self.parse_if_statement()?;
                Some((Pattern::Wildcard, vec![nested]))
            } else {
                let stmts = self.parse_block()?.statements;
                Some((Pattern::Wildcard, stmts))
            }
        } else {
            None
        };

        Ok(Stmt::new(
            StmtKind::If(Box::new(condition), then_block.statements, else_block),
            SourceSpan::new(if_token.location, self.previous().location),
        ))
    }

    /// Parse a while statement
    fn parse_while_statement(&mut self) -> Result<Stmt, ParserError> {
        let while_token = self.advance();
        let condition = self.parse_expression()?;
        let body = self.parse_block()?;

        Ok(Stmt::new(
            StmtKind::While(Box::new(condition), body.statements),
            SourceSpan::new(while_token.location, self.previous().location),
        ))
    }

    /// Parse a for statement
    fn parse_for_statement(&mut self) -> Result<Stmt, ParserError> {
        let for_token = self.advance();
        let pattern = self.parse_pattern()?;
        self.expect_keyword("in", "'in' expected")?;
        let iterator = self.parse_expression()?;
        let body = self.parse_block()?;

        let var_name = match &pattern {
            Pattern::Binding(name) => name.clone(),
            other => other.to_string(),
        };

        Ok(Stmt::new(
            StmtKind::For(var_name, Box::new(iterator), body.statements),
            SourceSpan::new(for_token.location, self.previous().location),
        ))
    }

    /// Parse a loop statement
    fn parse_loop_statement(&mut self) -> Result<Stmt, ParserError> {
        let loop_token = self.advance();
        let body = self.parse_block()?;

        Ok(Stmt::new(
            StmtKind::Loop(body.statements),
            SourceSpan::new(loop_token.location, self.previous().location),
        ))
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

        Ok(Stmt::new(
            StmtKind::Return(value),
            SourceSpan::new(return_token.location, self.previous().location),
        ))
    }

    /// Parse a break statement
    fn parse_break_statement(&mut self) -> Result<Stmt, ParserError> {
        let break_token = self.advance();
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Stmt::new(
            StmtKind::Break,
            SourceSpan::new(break_token.location, self.previous().location),
        ))
    }

    /// Parse a continue statement
    fn parse_continue_statement(&mut self) -> Result<Stmt, ParserError> {
        let continue_token = self.advance();
        self.expect(&TokenKind::Semicolon, "';' expected")?;

        Ok(Stmt::new(
            StmtKind::Continue,
            SourceSpan::new(continue_token.location, self.previous().location),
        ))
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
            });
        }

        self.expect(&TokenKind::RBrace, "'}' expected")?;

        let span = SourceSpan::new(match_token.location, self.previous().location);
        let expr = Expr::new(
            ExprKind::Match {
                discriminant: Box::new(value),
                arms,
            },
            span,
        );
        Ok(Stmt::new(StmtKind::Expr(expr), span))
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
            let span = expr.span;
            return Ok(Expr::new(
                ExprKind::Assign {
                    target: Box::new(expr),
                    value,
                },
                span,
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
            let span = expr.span;
            return Ok(Expr::new(
                ExprKind::CompoundAssign {
                    target: Box::new(expr),
                    op,
                    value,
                },
                span,
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
            let span = SourceSpan::new(expr.span.start, rhs.span.end);
            expr = Expr::new(
                ExprKind::Binary {
                    op: BinOp::Or,
                    lhs: Box::new(expr),
                    rhs,
                },
                span,
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
            let span = SourceSpan::new(expr.span.start, rhs.span.end);
            expr = Expr::new(
                ExprKind::Binary {
                    op: BinOp::And,
                    lhs: Box::new(expr),
                    rhs,
                },
                span,
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
            let span = SourceSpan::new(expr.span.start, rhs.span.end);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                span,
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
            let span = SourceSpan::new(expr.span.start, rhs.span.end);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                span,
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
            let span = SourceSpan::new(expr.span.start, rhs.span.end);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                span,
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
                TokenKind::Percent => BinOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let rhs = Box::new(self.parse_unary()?);
            let span = SourceSpan::new(expr.span.start, rhs.span.end);
            expr = Expr::new(
                ExprKind::Binary {
                    op,
                    lhs: Box::new(expr),
                    rhs,
                },
                span,
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
                SourceSpan::at(self.previous().location),
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
                    SourceSpan::at(self.previous().location),
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
                        object: Box::new(expr),
                        method,
                        arguments,
                    },
                    SourceSpan::at(self.previous().location),
                );
            } else if self.check(&TokenKind::LBracket) {
                self.advance();
                let index = self.parse_expression()?;
                self.expect(&TokenKind::RBracket, "']' expected")?;
                expr = Expr::new(
                    ExprKind::Index(Box::new(expr), Box::new(index)),
                    SourceSpan::at(self.previous().location),
                );
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Parse primary
    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.peek().clone();

        match &token.kind {
            // Literals
            TokenKind::Integer(n) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Literal(Literal::Int(*n)),
                    SourceSpan::at(token.location),
                ))
            }
            TokenKind::Float(n) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Literal(Literal::Float(*n)),
                    SourceSpan::at(token.location),
                ))
            }
            TokenKind::String(s) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Literal(Literal::String(s.clone())),
                    SourceSpan::at(token.location),
                ))
            }
            TokenKind::Char(c) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Literal(Literal::Char(*c)),
                    SourceSpan::at(token.location),
                ))
            }
            TokenKind::Bool(b) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Literal(Literal::Bool(*b)),
                    SourceSpan::at(token.location),
                ))
            }

            // Identifier
            TokenKind::Identifier(name) => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Identifier(name.clone()),
                    SourceSpan::at(token.location),
                ))
            }

            // Grouping
            TokenKind::LParen => {
                self.advance();
                if self.check(&TokenKind::RParen) {
                    self.advance();
                    return Ok(Expr::new(
                        ExprKind::Literal(Literal::Unit),
                        SourceSpan::at(token.location),
                    ));
                }
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RParen, "')' expected")?;
                Ok(Expr::new(
                    ExprKind::Parenthesis(Box::new(expr)),
                    SourceSpan::at(token.location),
                ))
            }

            // Block
            TokenKind::LBrace => Ok(Expr::new(
                ExprKind::Block(self.parse_block()?.statements),
                SourceSpan::at(token.location),
            )),

            // If expression
            TokenKind::If => {
                self.advance();
                let condition = Box::new(self.parse_expression()?);
                let then_block = vec![(Pattern::Wildcard, self.parse_block()?.statements)];
                let else_expr = if self.check(&TokenKind::Else) {
                    self.advance();
                    if self.check(&TokenKind::If) {
                        // `else if` becomes a nested `if` statement in the else branch.
                        let nested = self.parse_if_statement()?;
                        Some((Pattern::Wildcard, vec![nested]))
                    } else {
                        Some((Pattern::Wildcard, self.parse_block()?.statements))
                    }
                } else {
                    None
                };
                Ok(Expr::new(
                    ExprKind::If {
                        condition,
                        then_block,
                        else_expr,
                    },
                    SourceSpan::at(token.location),
                ))
            }

            // Loop
            TokenKind::Loop => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Loop(self.parse_block()?.statements),
                    SourceSpan::at(token.location),
                ))
            }

            // While
            TokenKind::While => {
                self.advance();
                let condition = Box::new(self.parse_expression()?);
                let body = self.parse_block()?.statements;
                Ok(Expr::new(
                    ExprKind::While { condition, body },
                    SourceSpan::at(token.location),
                ))
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
                    });
                }
                self.expect(&TokenKind::RBrace, "'}' expected")?;
                Ok(Expr::new(
                    ExprKind::Match {
                        discriminant: value,
                        arms,
                    },
                    SourceSpan::at(token.location),
                ))
            }

            // Return
            TokenKind::Return => {
                self.advance();
                let value = if self.check(&TokenKind::Semicolon) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                Ok(Expr::new(
                    ExprKind::Return(value.map(Box::new)),
                    SourceSpan::at(token.location),
                ))
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
                    SourceSpan::at(token.location),
                ))
            }

            // Continue
            TokenKind::Continue => {
                self.advance();
                Ok(Expr::new(
                    ExprKind::Continue { label: None },
                    SourceSpan::at(token.location),
                ))
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
                Ok(Expr::new(
                    ExprKind::Array(elements),
                    SourceSpan::at(token.location),
                ))
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

    /// Peek at the token `n` positions ahead of the current one
    fn peek_at(&self, n: usize) -> &Token {
        self.tokens.get(self.current + n).unwrap_or(self.peek())
    }

    /// Check whether the current token is an identifier
    fn peek_is_identifier(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Identifier(_))
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
    ///
    /// The consumed token is returned by value. Handing out a borrow of `self`
    /// would keep the parser mutably borrowed for as long as the caller held
    /// the token, which a recursive descent parser does at nearly every call
    /// site.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
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
            assert!(matches!(
                &f.body[0].kind,
                StmtKind::Let(_, _, _, _)
            ));
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
