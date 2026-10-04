//! TUMC Go Language Frontend
//!
//! Transpiles Go source code to TUMC IR for compilation to any target.

use crate::tumc_compiler::ast::{AstNode, Program, Function, Statement, Expression, Type};
use std::collections::HashMap;

/// Go-specific AST nodes
#[derive(Debug, Clone)]
pub enum GoAstNode {
    /// Go statement
    Go(Expression),
    /// Defer statement
    Defer(Expression),
    /// Send channel
    Send {
        channel: Expression,
        value: Expression,
    },
    /// Type switch
    TypeSwitch {
        assign: Box<AstNode>,
        cases: Vec<TypeCase>,
    },
    /// Type case
    TypeCase {
        type_: Option<String>,
        body: Vec<AstNode>,
    },
    /// Select statement
    Select {
        cases: Vec<SelectCase>,
    },
    /// Select case
    SelectCase {
        comm: Option<SendOrRecv>,
        body: Vec<AstNode>,
    },
    /// Send or receive for select
    SendOrRecv {
        is_send: bool,
        channel: Expression,
        value: Option<Expression>,
    },
    /// Function type
    FunctionType {
        params: Vec<GoField>,
        results: Vec<GoField>,
    },
    /// Interface type
    InterfaceType {
        methods: Vec<GoField>,
    },
    /// Struct type
    StructType {
        fields: Vec<GoField>,
    },
    /// Map type
    MapType {
        key: Type,
        value: Type,
    },
    /// Channel type
    ChannelType {
        value: Type,
        direction: ChannelDir,
    },
    /// Channel direction
    ChannelDir,
    /// Receive-only channel
    RecvOnly,
    /// Send-only channel
    SendOnly,
}

/// Channel direction
#[derive(Debug, Clone)]
pub enum ChannelDir {
    Both,
    Recv,
    Send,
}

/// Field in Go (for functions, interfaces, structs)
#[derive(Debug, Clone)]
pub struct GoField {
    pub names: Vec<String>,
    pub type_: Type,
}

/// Go parser
pub struct GoParser {
    /// Source code
    source: String,
    /// Tokens
    tokens: Vec<GoToken>,
    /// Current position
    pos: usize,
}

/// Go token types
#[derive(Debug, Clone, PartialEq)]
pub enum GoToken {
    // Identifiers and literals
    Ident(String),
    Int(i64),
    Float(f64),
    Imag(f64),
    Rune(i32),
    String(String),
    RawString(String),
    InterpretedString(String),

    // Keywords
    Break, Case, Chan, Const, Continue, Default, Defer, Else, Fallthrough,
    For, Func, Go, Goto, If, Import, Interface, Map, Package, Range, Return,
    Select, Struct, Switch, Type, Var,

    // Operators and delimiters
    Plus, Minus, Star, Slash, Percent, Caret, Ampersand, Pipe,
    LeftShift, RightShift, PlusPlus, MinusMinus, Equal, Less, Greater,
    Bang, Tilde,

    // Assignment operators
    ColonEqual, PlusEqual, MinusEqual, StarEqual, SlashEqual, PercentEqual,
    CaretEqual, AmpersandEqual, PipeEqual, LeftShiftEqual, RightShiftEqual,
    Colon, Dot, Comma, Semicolon, DotDotDot, Arrow,

    // Comparison operators
    EqualEqual, NotEqual, LessEqual, GreaterEqual, ChannelSend,

    // Delimiters
    LeftParen, RightParen, LeftBracket, RightBracket, LeftBrace, RightBrace,

    // Special
    EndOfFile,
    Comment(String),
    Whitespace,
    Newline,
}

/// Go lexer
pub struct GoLexer {
    source: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl GoLexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&self) -> Result<Vec<GoToken>, GoLexerError> {
        let mut tokens = Vec::new();
        let mut pos = 0;

        while pos < self.source.len() {
            let (token, next_pos) = self.lex_token(&mut pos)?;
            if let Some(t) = token {
                tokens.push(t);
            }
            pos = next_pos;
        }

        tokens.push(GoToken::EndOfFile);
        Ok(tokens)
    }

    fn lex_token(&self, pos: &mut usize) -> Result<(Option<GoToken>, usize), GoLexerError> {
        // Skip whitespace
        while *pos < self.source.len() {
            let ch = self.source.as_bytes()[*pos];
            match ch {
                b' ' | b'\t' | b'\r' => *pos += 1,
                b'\n' => {
                    *pos += 1;
                    return Ok((Some(GoToken::Newline), *pos));
                }
                _ => break,
            }
        }

        if *pos >= self.source.len() {
            return Ok((None, *pos));
        }

        let ch = self.source.as_bytes()[*pos];

        // Comments
        if ch == b'/' {
            if self.source[*pos..].starts_with("//") {
                let start = *pos;
                *pos += 2;
                while *pos < self.source.len() && self.source.as_bytes()[*pos] != b'\n' {
                    *pos += 1;
                }
                let comment = &self.source[start + 2..*pos].trim();
                return Ok((Some(GoToken::Comment(comment.to_string())), *pos));
            }
            if self.source[*pos..].starts_with("/*") {
                let start = *pos;
                *pos += 2;
                while *pos + 1 < self.source.len() &&
                      !self.source[*pos..].starts_with("*/") {
                    *pos += 1;
                }
                *pos += 2;
                let comment = &self.source[start + 2..*pos - 2];
                return Ok((Some(GoToken::Comment(comment.to_string())), *pos));
            }
        }

        // Identifiers and keywords
        if ch.is_ascii_alphabetic() || ch == b'_' {
            return self.lex_ident(pos);
        }

        // Numbers
        if ch.is_ascii_digit() || (ch == b'.' && *pos + 1 < self.source.len() &&
            self.source.as_bytes()[*pos + 1].is_ascii_digit()) {
            return self.lex_number(pos);
        }

        // Strings
        if ch == b'`' || ch == b'"' || ch == b'\'' {
            return self.lex_string(pos);
        }

        // Operators and delimiters
        Ok(self.lex_operator(pos))
    }

    fn lex_ident(&self, pos: &mut usize) -> Result<(Option<GoToken>, usize), GoLexerError> {
        let start = *pos;

        while *pos < self.source.len() {
            let ch = self.source.as_bytes()[*pos];
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                *pos += 1;
            } else {
                break;
            }
        }

        let s = &self.source[start..*pos];

        let token = match s {
            "break" => GoToken::Break,
            "case" => GoToken::Case,
            "chan" => GoToken::Chan,
            "const" => GoToken::Const,
            "continue" => GoToken::Continue,
            "default" => GoToken::Default,
            "defer" => GoToken::Defer,
            "else" => GoToken::Else,
            "fallthrough" => GoToken::Fallthrough,
            "for" => GoToken::For,
            "func" => GoToken::Func,
            "go" => GoToken::Go,
            "goto" => GoToken::Goto,
            "if" => GoToken::If,
            "import" => GoToken::Import,
            "interface" => GoToken::Interface,
            "map" => GoToken::Map,
            "package" => GoToken::Package,
            "range" => GoToken::Range,
            "return" => GoToken::Return,
            "select" => GoToken::Select,
            "struct" => GoToken::Struct,
            "switch" => GoToken::Switch,
            "type" => GoToken::Type,
            "var" => GoToken::Var,
            _ => GoToken::Ident(s.to_string()),
        };

        Ok((Some(token), *pos))
    }

    fn lex_number(&self, pos: &mut usize) -> Result<(Option<GoToken>, usize), GoLexerError> {
        let start = *pos;
        let mut has_dot = false;
        let mut has_exp = false;
        let mut is_hex = false;
        let mut is_imag = false;

        // Hexadecimal
        if self.source[*pos..].starts_with("0x") || self.source[*pos..].starts_with("0X") {
            *pos += 2;
            is_hex = true;
            while *pos < self.source.len() && (self.source.as_bytes()[*pos].is_ascii_hexdigit() ||
                self.source.as_bytes()[*pos] == b'_') {
                *pos += 1;
            }
            let s = &self.source[start..*pos];
            let value = i64::from_str_radix(&s[2..].replace('_', ""), 16).unwrap_or(0);
            return Ok((Some(GoToken::Int(value)), *pos));
        }

        // Octal
        if self.source[*pos..].starts_with("0o") || self.source[*pos..].starts_with("0O") {
            *pos += 2;
            while *pos < self.source.len() && (self.source.as_bytes()[*pos].is_ascii_digit() ||
                self.source.as_bytes()[*pos] == b'_') {
                *pos += 1;
            }
            let s = &self.source[start..*pos];
            let value = i64::from_str_radix(&s[2..].replace('_', ""), 8).unwrap_or(0);
            return Ok((Some(GoToken::Int(value)), *pos));
        }

        // Binary
        if self.source[*pos..].starts_with("0b") || self.source[*pos..].starts_with("0B") {
            *pos += 2;
            while *pos < self.source.len() && (self.source.as_bytes()[*pos] == b'0' ||
                self.source.as_bytes()[*pos] == b'1' || self.source.as_bytes()[*pos] == b'_') {
                *pos += 1;
            }
            let s = &self.source[start..*pos];
            let value = i64::from_str_radix(&s[2..].replace('_', ""), 2).unwrap_or(0);
            return Ok((Some(GoToken::Int(value)), *pos));
        }

        // Decimal
        while *pos < self.source.len() && (self.source.as_bytes()[*pos].is_ascii_digit() ||
              self.source.as_bytes()[*pos] == b'_') {
            *pos += 1;
        }

        if *pos < self.source.len() && self.source.as_bytes()[*pos] == b'.' {
            has_dot = true;
            *pos += 1;
            while *pos < self.source.len() && (self.source.as_bytes()[*pos].is_ascii_digit() ||
                  self.source.as_bytes()[*pos] == b'_') {
                *pos += 1;
            }
        }

        if *pos < self.source.len() && (self.source.as_bytes()[*pos] == b'e' ||
            self.source.as_bytes()[*pos] == b'E') {
            has_exp = true;
            *pos += 1;
            if *pos < self.source.len() && (self.source.as_bytes()[*pos] == b'+' ||
                self.source.as_bytes()[*pos] == b'-') {
                *pos += 1;
            }
            while *pos < self.source.len() && self.source.as_bytes()[*pos].is_ascii_digit() {
                *pos += 1;
            }
        }

        if *pos < self.source.len() && (self.source.as_bytes()[*pos] == b'i') {
            is_imag = true;
            *pos += 1;
        }

        let s = &self.source[start..*pos];

        if is_imag {
            let value = s[..s.len() - 1].parse::<f64>().unwrap_or(0.0);
            return Ok((Some(GoToken::Imag(value)), *pos));
        }

        if has_dot || has_exp {
            let value = s.replace('_', "").parse::<f64>().unwrap_or(0.0);
            Ok((Some(GoToken::Float(value)), *pos))
        } else {
            let value = s.replace('_', "").parse::<i64>().unwrap_or(0);
            Ok((Some(GoToken::Int(value)), *pos))
        }
    }

    fn lex_string(&self, pos: &mut usize) -> Result<(Option<GoToken>, usize), GoLexerError> {
        let ch = self.source.as_bytes()[*pos];
        *pos += 1;

        match ch {
            b'`' => {
                // Raw string
                let start = *pos;
                while *pos < self.source.len() && self.source.as_bytes()[*pos] != b'`' {
                    if self.source.as_bytes()[*pos] == b'\\' && *pos + 1 < self.source.len() {
                        *pos += 2;
                    } else {
                        *pos += 1;
                    }
                }
                let s = &self.source[start..*pos];
                *pos += 1; // Skip closing backtick
                Ok((Some(GoToken::RawString(s.to_string())), *pos))
            }
            b'"' => {
                // Interpreted string
                let start = *pos;
                while *pos < self.source.len() && self.source.as_bytes()[*pos] != b'"' {
                    if self.source.as_bytes()[*pos] == b'\\' && *pos + 1 < self.source.len() {
                        *pos += 2;
                    } else {
                        *pos += 1;
                    }
                }
                let s = &self.source[start..*pos];
                *pos += 1; // Skip closing quote
                Ok((Some(GoToken::InterpretedString(s.to_string())), *pos))
            }
            b'\'' => {
                // Rune
                let start = *pos;
                while *pos < self.source.len() && self.source.as_bytes()[*pos] != b'\'' {
                    if self.source.as_bytes()[*pos] == b'\\' && *pos + 1 < self.source.len() {
                        *pos += 2;
                    } else {
                        *pos += 1;
                    }
                }
                let s = &self.source[start..*pos];
                *pos += 1; // Skip closing quote
                // Parse rune value
                let value = s.chars().last().map(|c| c as i32).unwrap_or(0);
                Ok((Some(GoToken::Rune(value)), *pos))
            }
            _ => Err(GoLexerError::new("Invalid string literal", *pos)),
        }
    }

    fn lex_operator(&self, pos: &mut usize) -> (Option<GoToken>, usize) {
        let start = *pos;
        let ch = self.source.as_bytes()[*pos];

        let token = match ch {
            b'+' => {
                if self.source[*pos..].starts_with("++") { *pos += 2; GoToken::PlusPlus }
                else if self.source[*pos..].starts_with("+=") { *pos += 2; GoToken::PlusEqual }
                else { *pos += 1; GoToken::Plus }
            }
            b'-' => {
                if self.source[*pos..].starts_with("--") { *pos += 2; GoToken::MinusMinus }
                else if self.source[*pos..].starts_with("-=") { *pos += 2; GoToken::MinusEqual }
                else { *pos += 1; GoToken::Minus }
            }
            b'*' => {
                if self.source[*pos..].starts_with("*=") { *pos += 2; GoToken::StarEqual }
                else { *pos += 1; GoToken::Star }
            }
            b'/' => {
                if self.source[*pos..].starts_with("/=") { *pos += 2; GoToken::SlashEqual }
                else { *pos += 1; GoToken::Slash }
            }
            b'%' => {
                if self.source[*pos..].starts_with("%=") { *pos += 2; GoToken::PercentEqual }
                else { *pos += 1; GoToken::Percent }
            }
            b'^' => {
                if self.source[*pos..].starts_with("^=") { *pos += 2; GoToken::CaretEqual }
                else { *pos += 1; GoToken::Caret }
            }
            b'&' => {
                if self.source[*pos..].starts_with("&=") { *pos += 2; GoToken::AmpersandEqual }
                else if self.source[*pos..].starts_with("&&") { *pos += 2; GoToken::Ampersand }
                else { *pos += 1; GoToken::Ampersand }
            }
            b'|' => {
                if self.source[*pos..].starts_with("|=") { *pos += 2; GoToken::PipeEqual }
                else if self.source[*pos..].starts_with("||") { *pos += 2; GoToken::Pipe }
                else { *pos += 1; GoToken::Pipe }
            }
            b'<' => {
                if self.source[*pos..].starts_with("<=") { *pos += 2; GoToken::LessEqual }
                else if self.source[*pos..].starts_with("<-") { *pos += 2; GoToken::Arrow }
                else if self.source[*pos..].starts_with("<<=") { *pos += 3; GoToken::LeftShiftEqual }
                else if self.source[*pos..].starts_with("<<") { *pos += 2; GoToken::LeftShift }
                else { *pos += 1; GoToken::Less }
            }
            b'>' => {
                if self.source[*pos..].starts_with(">=") { *pos += 2; GoToken::GreaterEqual }
                else if self.source[*pos..].starts_with(">>=") { *pos += 3; GoToken::RightShiftEqual }
                else if self.source[*pos..].starts_with(">>") { *pos += 2; GoToken::RightShift }
                else { *pos += 1; GoToken::Greater }
            }
            b'=' => {
                if self.source[*pos..].starts_with("==") { *pos += 2; GoToken::EqualEqual }
                else { *pos += 1; GoToken::Equal }
            }
            b'!' => {
                if self.source[*pos..].starts_with("!=") { *pos += 2; GoToken::NotEqual }
                else { *pos += 1; GoToken::Bang }
            }
            b':' => {
                if self.source[*pos..].starts_with(":=") { *pos += 2; GoToken::ColonEqual }
                else { *pos += 1; GoToken::Colon }
            }
            b'(' => { *pos += 1; GoToken::LeftParen }
            b')' => { *pos += 1; GoToken::RightParen }
            b'[' => { *pos += 1; GoToken::LeftBracket }
            b']' => { *pos += 1; GoToken::RightBracket }
            b'{' => { *pos += 1; GoToken::LeftBrace }
            b'}' => { *pos += 1; GoToken::RightBrace }
            b',' => { *pos += 1; GoToken::Comma }
            b'.' => {
                if self.source[*pos..].starts_with("...") { *pos += 3; GoToken::DotDotDot }
                else { *pos += 1; GoToken::Dot }
            }
            b';' => { *pos += 1; GoToken::Semicolon }
            b'~' => { *pos += 1; GoToken::Tilde }
            _ => { *pos += 1; return (None, *pos); }
        };

        (Some(token), *pos)
    }
}

/// Go lexer error
#[derive(Debug)]
pub struct GoLexerError {
    pub message: String,
    pub position: usize,
}

impl std::fmt::Display for GoLexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error at {}: {}", self.position, self.message)
    }
}

impl std::error::Error for GoLexerError {}

/// Go parser implementation
impl GoParser {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Program, GoParseError> {
        let lexer = GoLexer::new(&self.source);
        self.tokens = lexer.tokenize().map_err(|e| GoParseError::new(&e.message, 0))?;
        self.pos = 0;

        let mut nodes = Vec::new();

        // Parse package declaration
        while !self.is_at_end() && !matches!(self.current(), GoToken::Package) {
            self.advance();
        }

        if self.check(&GoToken::Package) {
            self.advance();
            // Skip package name for now
            while !matches!(self.current(), GoToken::Semicolon | GoToken::Newline | GoToken::EndOfFile) {
                self.advance();
            }
            if matches!(self.current(), GoToken::Semicolon) {
                self.advance();
            }
        }

        // Parse rest of file
        while !self.is_at_end() {
            if let Some(node) = self.parse_declaration()? {
                nodes.push(node);
            }
        }

        Ok(Program { nodes })
    }

    fn parse_declaration(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.skip_newlines();

        if self.is_at_end() {
            return Ok(None);
        }

        match self.current() {
            GoToken::Func => self.parse_function(),
            GoToken::Const => self.parse_const(),
            GoToken::Var => self.parse_var(),
            GoToken::Type => self.parse_type(),
            GoToken::Import => self.parse_import(),
            GoToken::Go => self.parse_go(),
            GoToken::Defer => self.parse_defer(),
            _ => self.parse_statement(),
        }
    }

    fn parse_function(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip func

        let name = match &self.current() {
            GoToken::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(GoParseError::new("Expected function name", self.pos)),
        };

        // Parse parameters
        self.expect(&GoToken::LeftParen)?;
        let params = self.parse_param_list()?;
        self.expect(&GoToken::RightParen)?;

        // Parse results
        let mut return_type = None;
        if self.check(&GoToken::LeftParen) {
            self.advance();
            if !self.check(&GoToken::RightParen) {
                return_type = Some(self.parse_type()?);
            }
            self.expect(&GoToken::RightParen)?;
        } else if self.check(&GoToken::Ident(_)) {
            return_type = Some(self.parse_type()?);
        }

        // Parse body
        let body = if self.check(&GoToken::LeftBrace) {
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Some(AstNode::Function(Function {
            name,
            params,
            return_type,
            body,
            is_async: false,
            is_generator: false,
        })))
    }

    fn parse_param_list(&mut self) -> Result<Vec<crate::tumc_compiler::ast::Param>, GoParseError> {
        let mut params = Vec::new();

        while !self.check(&GoToken::RightParen) && !self.is_at_end() {
            let mut names = Vec::new();

            // Parse names
            while matches!(self.current(), GoToken::Ident(_)) {
                if let GoToken::Ident(name) = self.current() {
                    names.push(name.clone());
                }
                self.advance();

                if !self.check(&GoToken::Comma) {
                    break;
                }
                self.advance();
            }

            // Parse type
            let param_type = self.parse_type()?;

            for name in names {
                params.push(crate::tumc_compiler::ast::Param {
                    name,
                    param_type: Some(param_type.clone()),
                    default: None,
                    is_variadic: false,
                    is_kwargs: false,
                });
            }

            if !self.check(&GoToken::RightParen) {
                self.expect(&GoToken::Comma)?;
            }
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, GoParseError> {
        match self.current() {
            GoToken::Ident(_) => {
                // Could be type name or package.Type
                let mut name = String::new();
                if let GoToken::Ident(n) = self.current() {
                    name = n.clone();
                }
                self.advance();

                // Check for package prefix
                if self.check(&GoToken::Dot) {
                    self.advance();
                    if let GoToken::Ident(n) = self.current() {
                        name = format!("{}.{}", name, n);
                        self.advance();
                    }
                }

                // Check if it's a function type
                if self.check(&GoToken::LeftParen) {
                    return Ok(Type::Function(Box::new(Type::Unknown), Vec::new()));
                }

                Ok(self.go_type_to_tumc(&name))
            }
            GoToken::Star => {
                self.advance();
                let inner = self.parse_type()?;
                Ok(Type::Ptr(Box::new(inner)))
            }
            GoToken::LeftBracket => {
                self.advance();
                if self.check(&GoToken::RightBracket) {
                    self.advance();
                    let elem_type = Box::new(self.parse_type()?);
                    Ok(Type::Array(elem_type, None))
                } else {
                    let len = if let GoToken::Int(n) = self.current() {
                        self.advance();
                        n as usize
                    } else {
                        return Err(GoParseError::new("Expected array length", self.pos));
                    };
                    self.expect(&GoToken::RightBracket)?;
                    let elem_type = Box::new(self.parse_type()?);
                    Ok(Type::Array(elem_type, Some(len)))
                }
            }
            GoToken::Map => {
                self.advance();
                self.expect(&GoToken::LeftBracket)?;
                let _ = self.parse_type()?;
                self.expect(&GoToken::RightBracket)?;
                let value_type = self.parse_type()?;
                Ok(Type::Unknown) // Map type
            }
            GoToken::Chan => {
                self.advance();
                let value_type = self.parse_type()?;
                Ok(Type::Unknown) // Channel type
            }
            GoToken::Func => {
                self.advance();
                self.expect(&GoToken::LeftParen)?;
                let params = self.parse_param_list()?;
                self.expect(&GoToken::RightParen)?;
                let mut return_type = Type::Unit;
                if self.check(&GoToken::LeftParen) {
                    self.advance();
                    if !self.check(&GoToken::RightParen) {
                        return_type = self.parse_type()?;
                    }
                    self.expect(&GoToken::RightParen)?;
                } else if self.check(&GoToken::Ident(_)) {
                    return_type = self.parse_type()?;
                }
                Ok(Type::Function(Box::new(return_type), Vec::new()))
            }
            GoToken::Struct => {
                self.advance();
                self.expect(&GoToken::LeftBrace)?;
                while !self.check(&GoToken::RightBrace) && !self.is_at_end() {
                    self.advance();
                }
                self.expect(&GoToken::RightBrace)?;
                Ok(Type::Unknown)
            }
            GoToken::Interface => {
                self.advance();
                self.expect(&GoToken::LeftBrace)?;
                while !self.check(&GoToken::RightBrace) && !self.is_at_end() {
                    self.advance();
                }
                self.expect(&GoToken::RightBrace)?;
                Ok(Type::Unknown)
            }
            _ => {
                self.advance();
                Ok(Type::Unknown)
            }
        }
    }

    fn go_type_to_tumc(&self, name: &str) -> Type {
        match name {
            "int" | "int8" | "int16" | "int32" | "int64" => Type::I64,
            "uint" | "uint8" | "uint16" | "uint32" | "uint64" => Type::U64,
            "float32" => Type::F32,
            "float64" => Type::F64,
            "bool" => Type::Bool,
            "string" => Type::String,
            "byte" => Type::U8,
            "rune" => Type::I32,
            "error" => Type::Unknown,
            "any" | "interface{}" => Type::Unknown,
            _ => Type::Unknown,
        }
    }

    fn parse_const(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip const
        self.skip_newlines();

        let mut decls = Vec::new();

        while !self.check(&GoToken::Var) && !self.check(&GoToken::Func) &&
              !self.check(&GoToken::Type) && !self.check(&GoToken::Import) &&
              !self.check(&GoToken::Package) && !self.is_at_end() {

            if let GoToken::Ident(name) = self.current() {
                self.advance();

                let value = if self.check(&GoToken::Equal) {
                    self.advance();
                    Some(self.parse_expression()?)
                } else if self.check(&GoToken::ColonEqual) {
                    // Short variable declaration in const block
                    self.advance();
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                if let Some(val) = value {
                    decls.push(AstNode::Declaration {
                        name: name.clone(),
                        value: Some(Box::new(val)),
                        mutable: false,
                    });
                }

                self.skip_newlines();

                if !self.check(&GoToken::Comma) {
                    break;
                }
                self.advance();
                self.skip_newlines();
            } else {
                break;
            }
        }

        if decls.is_empty() {
            Ok(None)
        } else {
            Ok(Some(AstNode::Block(decls)))
        }
    }

    fn parse_var(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip var
        self.skip_newlines();

        let name = if let GoToken::Ident(n) = self.current() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return Err(GoParseError::new("Expected variable name", self.pos));
        };

        let var_type = if self.check(&GoToken::Ident(_)) && !self.check(&GoToken::Equal) &&
                        !self.check(&GoToken::ColonEqual) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let value = if self.check(&GoToken::Equal) || self.check(&GoToken::ColonEqual) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Some(AstNode::Declaration {
            name,
            value: value.map(Box::new),
            mutable: true,
        }))
    }

    fn parse_type_decl(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip type

        let name = if let GoToken::Ident(n) = self.current() {
            let n = n.clone();
            self.advance();
            n
        } else {
            return Err(GoParseError::new("Expected type name", self.pos));
        };

        // Check if it's a type alias or definition
        if self.check(&GoToken::Equal) {
            self.advance();
            let type_annotation = self.parse_type()?;
            // Type alias - could be represented differently
            return Ok(None);
        }

        // Type definition
        let type_annotation = self.parse_type()?;

        Ok(None)
    }

    fn parse_type(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance();
        self.parse_type_decl()
    }

    fn parse_import(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip import

        let path = if let GoToken::InterpretedString(s) = self.current() {
            self.advance();
            s
        } else if let GoToken::RawString(s) = self.current() {
            self.advance();
            s
        } else if self.check(&GoToken::LeftParen) {
            self.advance();
            let mut paths = Vec::new();
            while !self.check(&GoToken::RightParen) && !self.is_at_end() {
                if let GoToken::InterpretedString(s) = self.current() {
                    paths.push(s);
                } else if let GoToken::RawString(s) = self.current() {
                    paths.push(s);
                }
                self.advance();
            }
            self.expect(&GoToken::RightParen)?;
            paths.join(", ")
        } else {
            String::new()
        };

        Ok(Some(AstNode::Import {
            module: Some(path),
            names: Vec::new(),
            is_from: true,
        }))
    }

    fn parse_go(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip go
        let expr = self.parse_expression()?;
        Ok(Some(AstNode::ExpressionStatement(Box::new(
            Expression::Await(Box::new(expr.into()))
        ))))
    }

    fn parse_defer(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip defer
        let expr = self.parse_expression()?;
        Ok(Some(AstNode::ExpressionStatement(Box::new(expr.into()))))
    }

    fn parse_statement(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.skip_newlines();

        if self.is_at_end() {
            return Ok(None);
        }

        match self.current() {
            GoToken::If => self.parse_if(),
            GoToken::For => self.parse_for(),
            GoToken::Switch => self.parse_switch(),
            GoToken::Select => self.parse_select(),
            GoToken::Return => self.parse_return(),
            GoToken::Break => { self.advance(); Ok(Some(AstNode::Break)) }
            GoToken::Continue => { self.advance(); Ok(Some(AstNode::Continue)) }
            GoToken::Fallthrough => { self.advance(); Ok(Some(AstNode::Break)) }
            GoToken::Goto => {
                self.advance();
                Ok(None)
            }
            GoToken::Var => self.parse_var(),
            GoToken::Const => self.parse_const(),
            GoToken::LeftBrace => self.parse_block(),
            GoToken::Ident(_) => {
                // Could be assignment or expression
                let expr = self.parse_expression()?;

                if self.check(&GoToken::Equal) || self.check(&GoToken::ColonEqual) ||
                   self.check(&GoToken::PlusEqual) || self.check(&GoToken::MinusEqual) {
                    let is_init = matches!(self.current(), GoToken::ColonEqual);
                    self.advance();
                    let value = Box::new(self.parse_expression()?);

                    Ok(Some(AstNode::Assignment {
                        target: Box::new(expr.into()),
                        value,
                    }))
                } else {
                    Ok(Some(AstNode::ExpressionStatement(Box::new(expr))))
                }
            }
            _ => {
                self.advance();
                Ok(None)
            }
        }
    }

    fn parse_if(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip if

        let init = if self.check(&GoToken::Const) || self.check(&GoToken::Var) {
            Some(Box::new(self.parse_statement()?.unwrap_or_default()))
        } else {
            None
        };

        let condition = self.parse_expression()?;

        self.expect(&GoToken::LeftBrace)?;
        let then_branch = self.parse_block_stmts()?;
        self.expect(&GoToken::RightBrace)?;

        let mut else_branch = Vec::new();

        if self.check(&GoToken::Else) {
            self.advance();

            if self.check(&GoToken::If) {
                else_branch = vec![self.parse_if()?.unwrap()];
            } else if self.check(&GoToken::LeftBrace) {
                self.expect(&GoToken::LeftBrace)?;
                else_branch = self.parse_block_stmts()?;
                self.expect(&GoToken::RightBrace)?;
            }
        }

        Ok(Some(AstNode::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        }))
    }

    fn parse_for(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip for
        self.skip_newlines();

        // Check for range loop
        if self.check(&GoToken::Ident(_)) {
            let first_expr = self.parse_expression()?;

            if self.check(&GoToken::Range) {
                self.advance();
                let iter = self.parse_expression()?;

                self.expect(&GoToken::LeftBrace)?;
                let body = self.parse_block_stmts()?;
                self.expect(&GoToken::RightBrace)?;

                return Ok(Some(AstNode::For {
                    target: Box::new(first_expr.into()),
                    iter: Box::new(iter),
                    body,
                    else_body: Vec::new(),
                }));
            }
        }

        // Regular for loop
        let init = if !self.check(&GoToken::Semicolon) {
            Some(self.parse_simple_statement()?)
        } else {
            None
        };

        self.expect(&GoToken::Semicolon)?;
        let condition = if !self.check(&GoToken::Semicolon) {
            self.parse_expression()?
        } else {
            Expression::Literal(crate::tumc_compiler::ast::Literal::Bool(true))
        };

        self.expect(&GoToken::Semicolon)?;
        let post = if !self.check(&GoToken::LeftBrace) {
            Some(self.parse_simple_statement()?)
        } else {
            None
        };

        self.expect(&GoToken::LeftBrace)?;
        let body = self.parse_block_stmts()?;
        self.expect(&GoToken::RightBrace)?;

        // Desugar for loop
        Ok(Some(AstNode::Block(vec![
            init.unwrap_or(AstNode::ExpressionStatement(Box::new(
                Expression::Literal(crate::tumc_compiler::ast::Literal::Unit)
            ))),
            AstNode::While {
                condition: Box::new(condition),
                body: {
                    let mut stmts = body.clone();
                    if let Some(p) = post {
                        stmts.push(p);
                    }
                    stmts
                },
                else_body: Vec::new(),
            },
        ])))
    }

    fn parse_simple_statement(&mut self) -> Result<AstNode, GoParseError> {
        if let Some(stmt) = self.parse_statement()? {
            Ok(stmt)
        } else {
            Ok(AstNode::Block(Vec::new()))
        }
    }

    fn parse_switch(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip switch
        self.skip_newlines();

        let init = if self.check(&GoToken::Ident(_)) && !self.check(&GoToken::Ident("".to_string())) {
            Some(Box::new(self.parse_simple_statement()?))
        } else {
            None
        };

        let discriminant = if !self.check(&GoToken::LeftBrace) {
            self.parse_expression()?
        } else {
            Expression::Literal(crate::tumc_compiler::ast::Literal::Bool(true))
        };

        self.expect(&GoToken::LeftBrace)?;
        let mut cases = Vec::new();

        while !self.check(&GoToken::RightBrace) && !self.is_at_end() {
            self.skip_newlines();

            if self.check(&GoToken::Case) {
                self.advance();
                let value = self.parse_expression_list()?;

                self.expect(&GoToken::Colon)?;

                let mut stmts = Vec::new();
                while !self.check(&GoToken::Case) && !self.check(&GoToken::Default) &&
                      !self.check(&GoToken::RightBrace) && !self.is_at_end() {
                    if let Some(stmt) = self.parse_statement()? {
                        stmts.push(stmt);
                    }
                    self.skip_newlines();
                }

                for v in value {
                    cases.push(AstNode::SwitchCase {
                        test: Some(Box::new(v)),
                        consequent: stmts.clone(),
                    });
                }
            } else if self.check(&GoToken::Default) {
                self.advance();
                self.expect(&GoToken::Colon)?;

                let mut stmts = Vec::new();
                while !self.check(&GoToken::Case) && !self.check(&GoToken::Default) &&
                      !self.check(&GoToken::RightBrace) && !self.is_at_end() {
                    if let Some(stmt) = self.parse_statement()? {
                        stmts.push(stmt);
                    }
                    self.skip_newlines();
                }

                cases.push(AstNode::SwitchCase {
                    test: None,
                    consequent: stmts,
                });
            }
        }

        self.expect(&GoToken::RightBrace)?;

        Ok(Some(AstNode::Switch {
            discriminant: Box::new(discriminant),
            cases,
        }))
    }

    fn parse_select(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip select
        self.expect(&GoToken::LeftBrace)?;

        let mut cases = Vec::new();

        while !self.check(&GoToken::RightBrace) && !self.is_at_end() {
            if self.check(&GoToken::Case) {
                self.advance();
                // Parse send or receive
                let comm = self.parse_send_or_recv()?;

                self.expect(&GoToken::Colon)?;
                let body = self.parse_block_stmts()?;

                cases.push(AstNode::Block(vec![AstNode::ExpressionStatement(Box::new(
                    Expression::Call {
                        function: Box::new(Expression::Identifier("select_case".to_string()).into()),
                        arguments: vec![],
                    }
                )]));
            } else if self.check(&GoToken::Default) {
                self.advance();
                self.expect(&GoToken::Colon)?;
                let body = self.parse_block_stmts()?;
                cases.push(AstNode::Block(body));
            }
        }

        self.expect(&GoToken::RightBrace)?;

        Ok(Some(AstNode::Block(cases)))
    }

    fn parse_send_or_recv(&mut self) -> Result<Option<Expression>, GoParseError> {
        // Simplified - would parse full send/receive
        Ok(Some(self.parse_expression()?))
    }

    fn parse_return(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.advance(); // skip return

        let value = if !self.check(&GoToken::LeftBrace) && !self.is_at_end() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Some(AstNode::Return(value.map(Box::new))))
    }

    fn parse_block(&mut self) -> Result<Option<AstNode>, GoParseError> {
        self.expect(&GoToken::LeftBrace)?;
        let stmts = self.parse_block_stmts()?;
        self.expect(&GoToken::RightBrace)?;
        Ok(Some(AstNode::Block(stmts)))
    }

    fn parse_block_stmts(&mut self) -> Result<Vec<AstNode>, GoParseError> {
        let mut stmts = Vec::new();

        while !self.check(&GoToken::RightBrace) && !self.is_at_end() {
            self.skip_newlines();
            if let Some(stmt) = self.parse_statement()? {
                stmts.push(stmt);
            }
        }

        Ok(stmts)
    }

    fn parse_expression(&mut self) -> Result<Expression, GoParseError> {
        self.parse_expression_list()
    }

    fn parse_expression_list(&mut self) -> Result<Expression, GoParseError> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, GoParseError> {
        let mut expr = self.parse_unary()?;

        loop {
            match self.current() {
                GoToken::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();

                    while !self.check(&GoToken::RightParen) && !self.is_at_end() {
                        args.push(crate::tumc_compiler::ast::Arg::Positional(
                            Box::new(self.parse_expression()?)
                        ));

                        if !self.check(&GoToken::RightParen) {
                            self.expect(&GoToken::Comma)?;
                        }
                    }

                    self.expect(&GoToken::RightParen)?;

                    expr = Expression::Call {
                        function: Box::new(expr.into()),
                        arguments: args,
                    };
                }
                GoToken::Dot => {
                    self.advance();
                    if let GoToken::Ident(name) = self.current() {
                        self.advance();
                        expr = Expression::FieldAccess {
                            object: Box::new(expr.into()),
                            field: name,
                        };
                    }
                }
                GoToken::LeftBracket => {
                    self.advance();
                    let index = Box::new(self.parse_expression()?);
                    self.expect(&GoToken::RightBracket)?;
                    expr = Expression::Index {
                        array: Box::new(expr.into()),
                        index,
                    };
                }
                GoToken::Colon => {
                    // Slice expression
                    self.advance();
                    let index = Box::new(self.parse_expression()?);
                    expr = Expression::Index {
                        array: Box::new(expr.into()),
                        index,
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, GoParseError> {
        match self.current() {
            GoToken::Plus => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Plus,
                    operand,
                })
            }
            GoToken::Minus => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Negate,
                    operand,
                })
            }
            GoToken::Bang => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Not,
                    operand,
                })
            }
            GoToken::Caret => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::BitNot,
                    operand,
                })
            }
            GoToken::Star => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Deref,
                    operand,
                })
            }
            GoToken::Ampersand => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::AddressOf,
                    operand,
                })
            }
            GoToken::Arrow => {
                // Receive channel
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Deref,
                    operand,
                })
            }
            _ => self.parse_additive(),
        }
    }

    fn parse_additive(&mut self) -> Result<Expression, GoParseError> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = match self.current() {
                GoToken::Plus => Some(crate::tumc_compiler::ast::BinaryOp::Add),
                GoToken::Minus => Some(crate::tumc_compiler::ast::BinaryOp::Sub),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = Box::new(self.parse_multiplicative()?);
                expr = Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op,
                    right,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, GoParseError> {
        let mut expr = self.parse_comparison()?;

        loop {
            let op = match self.current() {
                GoToken::Star => Some(crate::tumc_compiler::ast::BinaryOp::Mul),
                GoToken::Slash => Some(crate::tumc_compiler::ast::BinaryOp::Div),
                GoToken::Percent => Some(crate::tumc_compiler::ast::BinaryOp::Rem),
                GoToken::LeftShift => Some(crate::tumc_compiler::ast::BinaryOp::Shl),
                GoToken::RightShift => Some(crate::tumc_compiler::ast::BinaryOp::Shr),
                GoToken::Ampersand => Some(crate::tumc_compiler::ast::BinaryOp::BitAnd),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = Box::new(self.parse_comparison()?);
                expr = Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op,
                    right,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, GoParseError> {
        let mut expr = self.parse_or()?;

        loop {
            let op = match self.current() {
                GoToken::EqualEqual => Some(crate::tumc_compiler::ast::BinaryOp::Eq),
                GoToken::NotEqual => Some(crate::tumc_compiler::ast::BinaryOp::Ne),
                GoToken::Less => Some(crate::tumc_compiler::ast::BinaryOp::Lt),
                GoToken::LessEqual => Some(crate::tumc_compiler::ast::BinaryOp::Le),
                GoToken::Greater => Some(crate::tumc_compiler::ast::BinaryOp::Gt),
                GoToken::GreaterEqual => Some(crate::tumc_compiler::ast::BinaryOp::Ge),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = Box::new(self.parse_or()?);
                expr = Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op,
                    right,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expression, GoParseError> {
        let mut expr = self.parse_and()?;

        while self.check(&GoToken::Pipe) {
            self.advance();
            let right = Box::new(self.parse_and()?);
            expr = Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitOr,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, GoParseError> {
        let mut expr = self.parse_primary_expr()?;

        while self.check(&GoToken::Ampersand) {
            self.advance();
            let right = Box::new(self.parse_primary_expr()?);
            expr = Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitAnd,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_primary_expr(&mut self) -> Result<Expression, GoParseError> {
        let token = self.current().clone();

        match token {
            GoToken::Ident(n) => {
                self.advance();
                Ok(Expression::Identifier(n))
            }
            GoToken::Int(n) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Int(n)))
            }
            GoToken::Float(n) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Float(n)))
            }
            GoToken::InterpretedString(s) | GoToken::RawString(s) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::String(s)))
            }
            GoToken::True => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Bool(true)))
            }
            GoToken::False => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Bool(false)))
            }
            GoToken::Ident(ref s) if s == "nil" => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Null))
            }
            GoToken::LeftParen => {
                self.advance();
                if self.check(&GoToken::RightParen) {
                    self.advance();
                    return Ok(Expression::Tuple(Vec::new()));
                }

                let first = self.parse_expression()?;

                if self.check(&GoToken::Comma) {
                    self.advance();
                    let mut elements = vec![first];
                    while !self.check(&GoToken::RightParen) {
                        elements.push(self.parse_expression()?);
                        if !self.check(&GoToken::RightParen) {
                            self.expect(&GoToken::Comma)?;
                        }
                    }
                    self.expect(&GoToken::RightParen)?;
                    Ok(Expression::Tuple(elements))
                } else {
                    self.expect(&GoToken::RightParen)?;
                    Ok(first)
                }
            }
            GoToken::LeftBrace => {
                self.advance();
                let mut elements = Vec::new();

                while !self.check(&GoToken::RightBrace) && !self.is_at_end() {
                    elements.push(self.parse_expression()?);
                    if !self.check(&GoToken::RightBrace) {
                        self.expect(&GoToken::Comma)?;
                    }
                }

                self.expect(&GoToken::RightBrace)?;
                Ok(Expression::Array(elements))
            }
            GoToken::Func => {
                self.advance();
                self.expect(&GoToken::LeftParen)?;
                let params = self.parse_param_list()?;
                self.expect(&GoToken::RightParen)?;

                let return_type = if self.check(&GoToken::Ident(_)) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                let body = if self.check(&GoToken::LeftBrace) {
                    self.parse_block()?
                } else {
                    None
                };

                Ok(Expression::Lambda {
                    params,
                    body: Box::new(body.map_or_else(|| Expression::Block(Vec::new().into()), |b| {
                        if let AstNode::Block(stmts) = b {
                            Expression::Block(stmts.into())
                        } else {
                            Expression::Block(vec![b].into())
                        }
                    })),
                })
            }
            GoToken::Make => {
                self.advance();
                let type_info = self.parse_type()?;
                // Simplified make() handling
                Ok(Expression::Call {
                    function: Box::new(Expression::Identifier("make".to_string()).into()),
                    arguments: vec![],
                })
            }
            GoToken::New => {
                self.advance();
                let type_info = self.parse_type()?;
                Ok(Expression::Call {
                    function: Box::new(Expression::Identifier("new".to_string()).into()),
                    arguments: vec![],
                })
            }
            _ => {
                self.advance();
                Err(GoParseError::new(&format!("Unexpected token: {:?}", token), self.pos))
            }
        }
    }

    // Helper methods
    fn current(&self) -> &GoToken {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &GoToken::EndOfFile
        }
    }

    fn check(&self, token: &GoToken) -> bool {
        std::mem::discriminant(self.current()) == std::mem::discriminant(token)
    }

    fn check_ident(&self, name: &str) -> bool {
        if let GoToken::Ident(n) = self.current() {
            n == name
        } else {
            false
        }
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(self.current(), GoToken::EndOfFile)
    }

    fn expect(&mut self, token: &GoToken) -> Result<(), GoParseError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(GoParseError::new(
                &format!("Expected {:?}, found {:?}", token, self.current()),
                self.pos
            ))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current(), GoToken::Newline | GoToken::Semicolon | GoToken::Comment(_)) {
            self.advance();
        }
    }
}

/// Go parse error
#[derive(Debug)]
pub struct GoParseError {
    pub message: String,
    pub position: usize,
}

impl GoParseError {
    pub fn new(message: &str, position: usize) -> Self {
        Self {
            message: message.to_string(),
            position,
        }
    }
}

impl std::fmt::Display for GoParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}: {}", self.position, self.message)
    }
}

impl std::error::Error for GoParseError {}

/// Go transpiler
pub struct GoTranspiler {
    pub type_env: HashMap<String, Type>,
}

impl GoTranspiler {
    pub fn new() -> Self {
        Self {
            type_env: HashMap::new(),
        }
    }

    pub fn transpile(&mut self, source: &str) -> Result<Program, GoParseError> {
        let mut parser = GoParser::new(source);
        parser.parse()
    }
}

impl Default for GoTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_numbers() {
        let source = "42 3.14 0xFF 0o77 0b101 3.14e10 10i";
        let lexer = GoLexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0);
    }

    #[test]
    fn test_parse_function() {
        let source = r#"
package main

func add(a int, b int) int {
    return a + b
}
"#;
        let mut parser = GoParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_if() {
        let source = r#"
if x := 1; x > 0 {
    return x
}
"#;
        let mut parser = GoParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_for() {
        let source = r#"
for i := 0; i < 10; i++ {
    sum += i
}
"#;
        let mut parser = GoParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_switch() {
        let source = r#"
switch x {
case 1:
    return "one"
case 2:
    return "two"
default:
    return "other"
}
"#;
        let mut parser = GoParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }
}
