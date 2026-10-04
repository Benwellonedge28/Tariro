//! TUMC TypeScript/JavaScript Language Frontend
//!
//! Transpiles TypeScript source code to TUMC IR for compilation to any target.

use crate::tumc_compiler::ast::{AstNode, Program, Function, Statement, Expression, Type};
use std::collections::HashMap;

/// TypeScript-specific AST nodes
#[derive(Debug, Clone)]
pub enum TsAstNode {
    /// Type annotation
    TypeAnnotation(Type),
    /// Interface declaration
    Interface {
        name: String,
        type_params: Vec<(String, Option<Type>)>,
        extends: Vec<String>,
        body: Vec<TsTypeMember>,
    },
    /// Type alias
    TypeAlias {
        name: String,
        type_params: Vec<String>,
        type_annotation: Type,
    },
    /// Enum declaration
    Enum {
        name: String,
        members: Vec<EnumMember>,
        is_const: bool,
    },
    /// Namespace/module declaration
    Namespace {
        name: String,
        body: Vec<AstNode>,
    },
    /// Import equals
    ImportEquals {
        name: String,
        path: String,
        is_default: bool,
    },
    /// Export assignment
    ExportAssignment(bool),
    /// As expression
    As {
        expression: Expression,
        type_annotation: Type,
    },
    /// Type query (typeof)
    TypeQuery(String),
    /// Non-null assertion
    NonNull(Expression),
    /// Satisfies expression
    Satisfies {
        expression: Expression,
        type_annotation: Type,
    },
    /// Index signature
    IndexSignature {
        key: String,
        key_type: Type,
        value_type: Type,
    },
    /// Getter
    Getter {
        name: String,
        body: Vec<Statement>,
    },
    /// Setter
    Setter {
        name: String,
        params: Vec<(String, Type)>,
        body: Vec<Statement>,
    },
    /// Method signature
    MethodSignature {
        name: String,
        type_params: Vec<String>,
        params: Vec<(String, Type)>,
        return_type: Type,
    },
    /// Call signature
    CallSignature {
        type_params: Vec<String>,
        params: Vec<(String, Type)>,
        return_type: Type,
    },
    /// Construct signature
    ConstructSignature {
        type_params: Vec<String>,
        params: Vec<(String, Type)>,
        return_type: Type,
    },
}

/// Type member for interfaces
#[derive(Debug, Clone)]
pub enum TsTypeMember {
    Property {
        name: String,
        type_annotation: Type,
        optional: bool,
        readonly: bool,
    },
    Method {
        name: String,
        type_params: Vec<String>,
        params: Vec<(String, Type)>,
        return_type: Type,
    },
    GetAccessor {
        name: String,
        type_annotation: Type,
    },
    SetAccessor {
        name: String,
        param_type: Type,
    },
    Index(IndexSignature),
    CallSignature(CallSignature),
    ConstructSignature(ConstructSignature),
}

/// Enum member
#[derive(Debug, Clone)]
pub struct EnumMember {
    pub name: String,
    pub value: Option<Expression>,
}

/// TypeScript parser
pub struct TypeScriptParser {
    /// Source code
    source: String,
    /// Current position
    pos: usize,
    /// Tokens
    tokens: Vec<TsToken>,
}

/// TypeScript token
#[derive(Debug, Clone, PartialEq)]
pub enum TsToken {
    // Literals
    NumericLiteral(f64),
    StringLiteral(String),
    TemplateLiteral(String),
    RegexLiteral(String),
    True,
    False,
    Null,
    Undefined,
    This,
    Super,

    // Identifiers
    Identifier(String),

    // Keywords
    Break, Do, Instanceof, TypeOf, Case, Else, Var, Catch, Finally, Void,
    Continue, For, New, Switch, While, Debugger, Function, Return, With,
    Default, If, Throw, Delete, In, Try, Async, Await, Of, From, As,
    Class, Enum, Extends, Super, Const, Export, Import, Implements,
    Interface, Let, Package, Private, Protected, Public, Static, Yield,
    Abstract, As, Any, Boolean, Number, String, Symbol, Type, Void,
    Never, Unknown, Readonly, Keyof, Infer, Require,

    // Operators
    Plus, Minus, Star, Slash, Percent, PlusPlus, MinusMinus,
    Equal, PlusEqual, MinusEqual, StarEqual, SlashEqual, PercentEqual,
    Bang, BangEqual, EqualEqual, EqualEqualEqual, StrictNotEqual,
    Less, Greater, LessEqual, GreaterEqual,
    Ampersand, AmpersandEqual, Pipe, PipeEqual, Caret, CaretEqual,
    LeftShift, RightShift, ZeroFillRightShift,
    LeftShiftEqual, RightShiftEqual, ZeroFillRightShiftEqual,
    AmpersandAmpersand, PipePipe, Question, QuestionQuestion,
    Colon, Semicolon, Comma, Dot, DotDotDot, Hash,

    // Template
    TemplateHead(String),
    TemplateMiddle(String),
    TemplateTail(String),

    // JSX
    JsxText(String),
    JsxChild,
    JsxOpenTag(String),
    JsxCloseTag(String),
    JsxSelfCloseTag(String),

    // Other
    WhiteSpace,
    NewLine,
    SingleLineComment(String),
    MultiLineComment(String),
    Hashbang,
    Unknown(String),
    EndOfFile,
}

/// TypeScript lexer
pub struct TypeScriptLexer {
    source: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl TypeScriptLexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&self) -> Result<Vec<TsToken>, TsLexerError> {
        let mut tokens = Vec::new();
        let mut pos = 0;

        while pos < self.source.len() {
            let (token, next_pos) = self.lex_token(&mut pos)?;
            if let Some(t) = token {
                tokens.push(t);
            }
            pos = next_pos;
        }

        tokens.push(TsToken::EndOfFile);
        Ok(tokens)
    }

    fn lex_token(&self, pos: &mut usize) -> Result<(Option<TsToken>, usize), TsLexerError> {
        self.skip_whitespace(pos);

        if *pos >= self.source.len() {
            return Ok((None, *pos));
        }

        let ch = self.source.as_bytes()[*pos];

        // Identifiers and keywords
        if ch.is_ascii_alphabetic() || ch == b'_' || ch == b'$' {
            return self.lex_identifier_or_keyword(pos);
        }

        // Numbers
        if ch.is_ascii_digit() || (ch == b'.' && *pos + 1 < self.source.len() &&
            self.source.as_bytes()[*pos + 1].is_ascii_digit()) {
            return self.lex_number(pos);
        }

        // Strings
        if ch == b'"' || ch == b'\'' {
            return self.lex_string(pos);
        }

        // Template literals
        if ch == b'`' {
            return self.lex_template(pos);
        }

        // Regex (simplified)
        if ch == b'/' && !self.is_after_value(pos) {
            return self.lex_regex(pos);
        }

        // JSX
        if ch == b'<' {
            return self.lex_jsx(pos);
        }

        // Operators
        Ok(self.lex_operator(pos))
    }

    fn skip_whitespace(&self, pos: &mut usize) {
        while *pos < self.source.len() {
            let ch = self.source.as_bytes()[*pos];
            match ch {
                b' ' | b'\t' | b'\r' => *pos += 1,
                b'\n' => {
                    *pos += 1;
                }
                b'/' if *pos + 1 < self.source.len() => {
                    let next = self.source.as_bytes()[*pos + 1];
                    if next == b'/' {
                        // Single line comment
                        while *pos < self.source.len() && self.source.as_bytes()[*pos] != b'\n' {
                            *pos += 1;
                        }
                    } else if next == b'*' {
                        // Multi-line comment
                        *pos += 2;
                        while *pos + 1 < self.source.len() {
                            if self.source.as_bytes()[*pos] == b'*' &&
                               self.source.as_bytes()[*pos + 1] == b'/' {
                                *pos += 2;
                                break;
                            }
                            *pos += 1;
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn lex_identifier_or_keyword(&self, pos: &mut usize) -> Result<(Option<TsToken>, usize), TsLexerError> {
        let start = *pos;

        while *pos < self.source.len() {
            let ch = self.source.as_bytes()[*pos];
            if ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'$' {
                *pos += 1;
            } else {
                break;
            }
        }

        let s = &self.source[start..*pos];

        let token = match s {
            "break" => TsToken::Break,
            "do" => TsToken::Do,
            "instanceof" => TsToken::Instanceof,
            "typeof" => TsToken::TypeOf,
            "case" => TsToken::Case,
            "else" => TsToken::Else,
            "var" => TsToken::Var,
            "catch" => TsToken::Catch,
            "finally" => TsToken::Finally,
            "void" => TsToken::Void,
            "continue" => TsToken::Continue,
            "for" => TsToken::For,
            "new" => TsToken::New,
            "switch" => TsToken::Switch,
            "while" => TsToken::While,
            "debugger" => TsToken::Debugger,
            "function" => TsToken::Function,
            "return" => TsToken::Return,
            "with" => TsToken::With,
            "default" => TsToken::Default,
            "if" => TsToken::If,
            "throw" => TsToken::Throw,
            "delete" => TsToken::Delete,
            "in" => TsToken::In,
            "try" => TsToken::Try,
            "async" => TsToken::Async,
            "await" => TsToken::Await,
            "of" => TsToken::Of,
            "from" => TsToken::From,
            "as" => TsToken::As,
            "class" => TsToken::Class,
            "enum" => TsToken::Enum,
            "extends" => TsToken::Extends,
            "super" => TsToken::Super,
            "const" => TsToken::Const,
            "export" => TsToken::Export,
            "import" => TsToken::Import,
            "implements" => TsToken::Implements,
            "interface" => TsToken::Interface,
            "let" => TsToken::Let,
            "package" => TsToken::Package,
            "private" => TsToken::Private,
            "protected" => TsToken::Protected,
            "public" => TsToken::Public,
            "static" => TsToken::Static,
            "yield" => TsToken::Yield,
            "abstract" => TsToken::Abstract,
            "any" => TsToken::Any,
            "boolean" => TsToken::Boolean,
            "number" => TsToken::Number,
            "string" => TsToken::String,
            "symbol" => TsToken::Symbol,
            "type" => TsToken::Type,
            "void" => TsToken::Void,
            "never" => TsToken::Never,
            "unknown" => TsToken::Unknown,
            "readonly" => TsToken::Readonly,
            "keyof" => TsToken::Keyof,
            "infer" => TsToken::Infer,
            "true" => TsToken::True,
            "false" => TsToken::False,
            "null" => TsToken::Null,
            "undefined" => TsToken::Undefined,
            "this" => TsToken::This,
            "require" => TsToken::Require,
            _ => TsToken::Identifier(s.to_string()),
        };

        Ok((Some(token), *pos))
    }

    fn lex_number(&self, pos: &mut usize) -> Result<(Option<TsToken>, usize), TsLexerError> {
        let start = *pos;
        let mut has_dot = false;
        let mut has_exp = false;

        // Hex, octal, binary
        if self.source[*pos..].starts_with("0x") || self.source[*pos..].starts_with("0X") {
            *pos += 2;
            while *pos < self.source.len() && self.source.as_bytes()[*pos].is_ascii_hexdigit() {
                *pos += 1;
            }
            let s = &self.source[start..*pos];
            let value = u64::from_str_radix(&s[2..], 16).unwrap_or(0.0);
            return Ok((Some(TsToken::NumericLiteral(value as f64)), *pos));
        }

        if self.source[*pos..].starts_with("0o") || self.source[*pos..].starts_with("0O") {
            *pos += 2;
            while *pos < self.source.len() && self.source.as_bytes()[*pos].is_ascii_digit() {
                *pos += 1;
            }
            let s = &self.source[start..*pos];
            let value = u64::from_str_radix(&s[2..], 8).unwrap_or(0.0);
            return Ok((Some(TsToken::NumericLiteral(value as f64)), *pos));
        }

        if self.source[*pos..].starts_with("0b") || self.source[*pos..].starts_with("0B") {
            *pos += 2;
            while *pos < self.source.len() && (self.source.as_bytes()[*pos] == b'0' ||
                self.source.as_bytes()[*pos] == b'1') {
                *pos += 1;
            }
            let s = &self.source[start..*pos];
            let value = u64::from_str_radix(&s[2..], 2).unwrap_or(0.0);
            return Ok((Some(TsToken::NumericLiteral(value as f64)), *pos));
        }

        // Decimal
        while *pos < self.source.len() && self.source.as_bytes()[*pos].is_ascii_digit() {
            *pos += 1;
        }

        if *pos < self.source.len() && self.source.as_bytes()[*pos] == b'.' {
            has_dot = true;
            *pos += 1;
            while *pos < self.source.len() && self.source.as_bytes()[*pos].is_ascii_digit() {
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

        let s = &self.source[start..*pos];
        let value = s.parse::<f64>().unwrap_or(0.0);
        Ok((Some(TsToken::NumericLiteral(value)), *pos))
    }

    fn lex_string(&self, pos: &mut usize) -> Result<(Option<TsToken>, usize), TsLexerError> {
        let quote = self.source.as_bytes()[*pos];
        *pos += 1;
        let start = *pos;

        while *pos < self.source.len() {
            let ch = self.source.as_bytes()[*pos];
            if ch == quote {
                let s = &self.source[start..*pos];
                *pos += 1;
                return Ok((Some(TsToken::StringLiteral(s.to_string())), *pos));
            }
            if ch == b'\\' && *pos + 1 < self.source.len() {
                *pos += 2;
                continue;
            }
            if ch == b'\n' {
                break;
            }
            *pos += 1;
        }

        let s = &self.source[start..*pos];
        Ok((Some(TsToken::StringLiteral(s.to_string())), *pos))
    }

    fn lex_template(&self, pos: &mut usize) -> Result<(Option<TsToken>, usize), TsLexerError> {
        *pos += 1; // Skip backtick
        let start = *pos;
        let mut depth = 0;

        while *pos < self.source.len() {
            let ch = self.source.as_bytes()[*pos];
            if ch == b'`' && depth == 0 {
                let s = &self.source[start..*pos];
                *pos += 1;
                return Ok((Some(TsToken::TemplateLiteral(s.to_string())), *pos));
            }
            if ch == b'{' {
                depth += 1;
            }
            if ch == b'}' {
                depth = depth.saturating_sub(1);
            }
            if ch == b'\\' && *pos + 1 < self.source.len() {
                *pos += 2;
                continue;
            }
            *pos += 1;
        }

        let s = &self.source[start..*pos];
        Ok((Some(TsToken::TemplateLiteral(s.to_string())), *pos))
    }

    fn lex_regex(&self, pos: &mut usize) -> Result<(Option<TsToken>, usize), TsLexerError> {
        *pos += 1; // Skip /
        let start = *pos;

        while *pos < self.source.len() {
            let ch = self.source.as_bytes()[*pos];
            if ch == b'/' {
                *pos += 1;
                // Skip flags
                while *pos < self.source.len() && self.source.as_bytes()[*pos].is_ascii_alphabetic() {
                    *pos += 1;
                }
                let s = &self.source[start..*pos - 1];
                return Ok((Some(TsToken::RegexLiteral(s.to_string())), *pos));
            }
            if ch == b'\\' && *pos + 1 < self.source.len() {
                *pos += 2;
                continue;
            }
            if ch == b'\n' {
                break;
            }
            *pos += 1;
        }

        Ok((Some(TsToken::Slash), *pos))
    }

    fn lex_jsx(&self, pos: &mut usize) -> Result<(Option<TsToken>, usize), TsLexerError> {
        *pos += 1; // Skip <

        // Check for closing tag
        if self.source[*pos..].starts_with("/>") {
            *pos += 2;
            return Ok((Some(TsToken::JsxSelfCloseTag(String::new())), *pos));
        }

        if self.source[*pos..].starts_with("</") {
            *pos += 2;
            let start = *pos;
            while *pos < self.source.len() && self.source.as_bytes()[*pos] != b'>' {
                *pos += 1;
            }
            let name = &self.source[start..*pos];
            *pos += 1;
            return Ok((Some(TsToken::JsxCloseTag(name.to_string())), *pos));
        }

        // Opening tag
        let start = *pos;
        while *pos < self.source.len() && self.source.as_bytes()[*pos] != b' ' &&
               self.source.as_bytes()[*pos] != b'>' && self.source.as_bytes()[*pos] != b'/' {
            *pos += 1;
        }
        let name = &self.source[start..*pos];

        Ok((Some(TsToken::JsxOpenTag(name.to_string())), *pos))
    }

    fn lex_operator(&self, pos: &mut usize) -> (Option<TsToken>, usize) {
        let start = *pos;
        let ch1 = self.source.as_bytes()[*pos];

        match ch1 {
            b'+' => {
                if self.source[*pos..].starts_with("++") { *pos += 2; (Some(TsToken::PlusPlus), *pos) }
                else if self.source[*pos..].starts_with("+=") { *pos += 2; (Some(TsToken::PlusEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Plus), *pos) }
            }
            b'-' => {
                if self.source[*pos..].starts_with("--") { *pos += 2; (Some(TsToken::MinusMinus), *pos) }
                else if self.source[*pos..].starts_with("-=") { *pos += 2; (Some(TsToken::MinusEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Minus), *pos) }
            }
            b'*' => {
                if self.source[*pos..].starts_with("**") { *pos += 2; (Some(TsToken::Star), *pos) }
                else if self.source[*pos..].starts_with("*=") { *pos += 2; (Some(TsToken::StarEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Star), *pos) }
            }
            b'/' => {
                if self.source[*pos..].starts_with("/=") { *pos += 2; (Some(TsToken::SlashEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Slash), *pos) }
            }
            b'%' => {
                if self.source[*pos..].starts_with("%=") { *pos += 2; (Some(TsToken::PercentEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Percent), *pos) }
            }
            b'=' => {
                if self.source[*pos..].starts_with("===") { *pos += 3; (Some(TsToken::EqualEqualEqual), *pos) }
                else if self.source[*pos..].starts_with("==") { *pos += 2; (Some(TsToken::EqualEqual), *pos) }
                else if self.source[*pos..].starts_with("=>") { *pos += 2; (Some(TsToken::Equal), *pos) }
                else { *pos += 1; (Some(TsToken::Equal), *pos) }
            }
            b'!' => {
                if self.source[*pos..].starts_with("!==") { *pos += 3; (Some(TsToken::StrictNotEqual), *pos) }
                else if self.source[*pos..].starts_with("!=") { *pos += 2; (Some(TsToken::BangEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Bang), *pos) }
            }
            b'<' => {
                if self.source[*pos..].starts_with("<=") { *pos += 2; (Some(TsToken::LessEqual), *pos) }
                else if self.source[*pos..].starts_with("<<=") { *pos += 3; (Some(TsToken::LeftShiftEqual), *pos) }
                else if self.source[*pos..].starts_with("<<") { *pos += 2; (Some(TsToken::LeftShift), *pos) }
                else { *pos += 1; (Some(TsToken::Less), *pos) }
            }
            b'>' => {
                if self.source[*pos..].starts_with(">=") { *pos += 2; (Some(TsToken::GreaterEqual), *pos) }
                else if self.source[*pos..].starts_with(">>>=") { *pos += 4; (Some(TsToken::ZeroFillRightShiftEqual), *pos) }
                else if self.source[*pos..].starts_with(">>>") { *pos += 3; (Some(TsToken::ZeroFillRightShift), *pos) }
                else if self.source[*pos..].starts_with(">>=") { *pos += 3; (Some(TsToken::RightShiftEqual), *pos) }
                else if self.source[*pos..].starts_with(">>") { *pos += 2; (Some(TsToken::RightShift), *pos) }
                else { *pos += 1; (Some(TsToken::Greater), *pos) }
            }
            b'&' => {
                if self.source[*pos..].starts_with("&&=") { *pos += 3; (Some(TsToken::Ampersand), *pos) }
                else if self.source[*pos..].starts_with("&&") { *pos += 2; (Some(TsToken::AmpersandAmpersand), *pos) }
                else if self.source[*pos..].starts_with("&=") { *pos += 2; (Some(TsToken::AmpersandEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Ampersand), *pos) }
            }
            b'|' => {
                if self.source[*pos..].starts_with("||=") { *pos += 3; (Some(TsToken::Pipe), *pos) }
                else if self.source[*pos..].starts_with("||") { *pos += 2; (Some(TsToken::PipePipe), *pos) }
                else if self.source[*pos..].starts_with("|=") { *pos += 2; (Some(TsToken::PipeEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Pipe), *pos) }
            }
            b'^' => {
                if self.source[*pos..].starts_with("^=") { *pos += 2; (Some(TsToken::CaretEqual), *pos) }
                else { *pos += 1; (Some(TsToken::Caret), *pos) }
            }
            b'?' => {
                if self.source[*pos..].starts_with("?.") { *pos += 2; (Some(TsToken::Question), *pos) }
                else if self.source[*pos..].starts_with("??") { *pos += 2; (Some(TsToken::QuestionQuestion), *pos) }
                else if self.source[*pos..].starts_with("?=") { *pos += 2; (Some(TsToken::Question), *pos) }
                else { *pos += 1; (Some(TsToken::Question), *pos) }
            }
            b':' => { *pos += 1; (Some(TsToken::Colon), *pos) }
            b';' => { *pos += 1; (Some(TsToken::Semicolon), *pos) }
            b',' => { *pos += 1; (Some(TsToken::Comma), *pos) }
            b'.' => {
                if self.source[*pos..].starts_with("...") { *pos += 3; (Some(TsToken::DotDotDot), *pos) }
                else { *pos += 1; (Some(TsToken::Dot), *pos) }
            }
            b'#' => { *pos += 1; (Some(TsToken::Hash), *pos) }
            _ => { *pos += 1; (None, *pos) }
        }
    }

    fn is_after_value(&self, pos: &usize) -> bool {
        true // Simplified - would need context tracking
    }
}

/// TypeScript parser implementation
impl TypeScriptParser {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            pos: 0,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, TsParseError> {
        let lexer = TypeScriptLexer::new(&self.source);
        self.tokens = lexer.tokenize().map_err(|e| TsParseError::new(&e.message, 0))?;
        self.pos = 0;

        let mut nodes = Vec::new();

        while !self.is_at_end() {
            if let Some(node) = self.parse_statement()? {
                nodes.push(node);
            }
        }

        Ok(Program { nodes })
    }

    fn parse_statement(&mut self) -> Result<Option<AstNode>, TsParseError> {
        if self.is_at_end() {
            return Ok(None);
        }

        let token = &self.tokens[self.pos];

        match token {
            TsToken::Function => self.parse_function(),
            TsToken::Class => self.parse_class(),
            TsToken::Async => self.parse_async_statement(),
            TsToken::Const | TsToken::Let | TsToken::Var => self.parse_variable_declaration(),
            TsToken::If => self.parse_if(),
            TsToken::While => self.parse_while(),
            TsToken::For => self.parse_for(),
            TsToken::Do => self.parse_do_while(),
            TsToken::Switch => self.parse_switch(),
            TsToken::Try => self.parse_try(),
            TsToken::Return => self.parse_return(),
            TsToken::Throw => self.parse_throw(),
            TsToken::Break => { self.advance(); Ok(Some(AstNode::Break)) }
            TsToken::Continue => { self.advance(); Ok(Some(AstNode::Continue)) }
            TsToken::Export => self.parse_export(),
            TsToken::Import => self.parse_import(),
            TsToken::Interface => self.parse_interface(),
            TsToken::Type => self.parse_type_alias(),
            TsToken::Enum => self.parse_enum(),
            TsToken::Namespace | TsToken::Module => self.parse_namespace(),
            TsToken::OpenBrace => self.parse_block(),
            TsToken::Semicolon => { self.advance(); Ok(None) }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_function(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance(); // skip function

        let name = match self.current() {
            TsToken::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(TsParseError::new("Expected function name", self.pos)),
        };

        // Type parameters
        let type_params = self.parse_type_parameters()?;

        self.expect(&TsToken::LeftParen)?;

        let mut params = Vec::new();
        while !self.check(&TsToken::RightParen) {
            let param_name = match self.current() {
                TsToken::Identifier(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                TsToken::DotDotDot => {
                    self.advance();
                    match self.current() {
                        TsToken::Identifier(n) => {
                            let n = format!("...{}", n);
                            self.advance();
                            n
                        }
                        _ => return Err(TsParseError::new("Expected parameter name", self.pos)),
                    }
                }
                _ => return Err(TsParseError::new("Expected parameter name", self.pos)),
            };

            let param_type = if self.check(&TsToken::Colon) {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            let default = if self.check(&TsToken::Equal) {
                self.advance();
                Some(self.parse_expression()?)
            } else {
                None
            };

            params.push(crate::tumc_compiler::ast::Param {
                name: param_name,
                param_type,
                default,
                is_variadic: false,
                is_kwargs: false,
            });

            if !self.check(&TsToken::RightParen) {
                self.expect(&TsToken::Comma)?;
            }
        }

        self.expect(&TsToken::RightParen)?;

        let return_type = if self.check(&TsToken::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = if self.check(&TsToken::OpenBrace) {
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

    fn parse_async_statement(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance(); // skip async

        if self.check(&TsToken::Function) {
            self.advance();
            let name = match self.current() {
                TsToken::Identifier(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => "anonymous".to_string(),
            };

            self.expect(&TsToken::LeftParen)?;
            // Skip params for brevity
            while !self.check(&TsToken::RightParen) {
                self.advance();
            }
            self.expect(&TsToken::RightParen)?;

            let body = if self.check(&TsToken::OpenBrace) {
                self.parse_block()?
            } else {
                Vec::new()
            };

            Ok(Some(AstNode::Function(Function {
                name,
                params: Vec::new(),
                return_type: None,
                body,
                is_async: true,
                is_generator: false,
            })))
        } else {
            // Async arrow function or expression
            self.parse_expression()
        }
    }

    fn parse_class(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance(); // skip class

        let name = match self.current() {
            TsToken::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(TsParseError::new("Expected class name", self.pos)),
        };

        // Type parameters
        let _ = self.parse_type_parameters()?;

        // Extends
        let mut extends = Vec::new();
        if self.check(&TsToken::Extends) {
            self.advance();
            loop {
                match self.current() {
                    TsToken::Identifier(n) => {
                        extends.push(n.clone());
                        self.advance();
                    }
                    _ => break,
                }
                if !self.check(&TsToken::Comma) {
                    break;
                }
                self.advance();
            }
        }

        // Implements
        if self.check(&TsToken::Implements) {
            self.advance();
            while !self.check(&TsToken::OpenBrace) {
                self.advance();
            }
        }

        self.expect(&TsToken::OpenBrace)?;

        let mut body = Vec::new();
        while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
            // Parse class member
            if self.check(&TsToken::Identifier(_)) {
                // Property or method
                let member_name = match self.current() {
                    TsToken::Identifier(n) => n.clone(),
                    _ => String::new(),
                };
                self.advance();

                // Check for constructor
                if member_name == "constructor" {
                    self.expect(&TsToken::LeftParen)?;
                    while !self.check(&TsToken::RightParen) {
                        self.advance();
                    }
                    self.expect(&TsToken::RightParen)?;

                    if self.check(&TsToken::OpenBrace) {
                        let constructor_body = self.parse_block()?;
                        body.push(AstNode::Function(Function {
                            name: "constructor".to_string(),
                            params: Vec::new(),
                            return_type: None,
                            body: constructor_body,
                            is_async: false,
                            is_generator: false,
                        }));
                    }
                }
            } else if self.check(&TsToken::Async) || self.check(&TsToken::Static) ||
                      self.check(&TsToken::Get) || self.check(&TsToken::Set) {
                self.advance();
            } else {
                self.advance();
            }
        }

        self.expect(&TsToken::CloseBrace)?;

        Ok(Some(AstNode::Class(crate::tumc_compiler::ast::Class {
            name,
            bases: Vec::new(), // Would convert extends here
            body,
            decorators: Vec::new(),
        })))
    }

    fn parse_block(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.expect(&TsToken::OpenBrace)?;

        let mut statements = Vec::new();
        while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
        }

        self.expect(&TsToken::CloseBrace)?;

        Ok(Some(AstNode::Block(statements)))
    }

    fn parse_if(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance(); // skip if

        self.expect(&TsToken::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TsToken::RightParen)?;

        let then_branch = if let Some(block) = self.parse_block()? {
            match block {
                AstNode::Block(stmts) => stmts,
                _ => vec![block],
            }
        } else {
            Vec::new()
        };

        let mut else_branch = Vec::new();
        if self.check(&TsToken::Else) {
            self.advance();
            if self.check(&TsToken::If) {
                else_branch = vec![self.parse_if()?.unwrap()];
            } else if let Some(block) = self.parse_block()? {
                else_branch = match block {
                    AstNode::Block(stmts) => stmts,
                    _ => vec![block],
                };
            }
        }

        Ok(Some(AstNode::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        }))
    }

    fn parse_while(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance(); // skip while

        self.expect(&TsToken::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TsToken::RightParen)?;

        let body = if let Some(block) = self.parse_block()? {
            match block {
                AstNode::Block(stmts) => stmts,
                _ => vec![block],
            }
        } else {
            Vec::new()
        };

        Ok(Some(AstNode::While {
            condition: Box::new(condition),
            body,
            else_body: Vec::new(),
        }))
    }

    fn parse_for(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance(); // skip for

        self.expect(&TsToken::LeftParen)?;

        // Init
        let init = if self.check(&TsToken::Const) || self.check(&TsToken::Let) ||
                   self.check(&TsToken::Var) {
            self.parse_variable_declaration()?
        } else if !self.check(&TsToken::Semicolon) {
            Some(AstNode::ExpressionStatement(Box::new(self.parse_expression()?)))
        } else {
            None
        };

        self.expect(&TsToken::Semicolon)?;

        // Condition
        let condition = if !self.check(&TsToken::Semicolon) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(&TsToken::Semicolon)?;

        // Update
        let update = if !self.check(&TsToken::RightParen) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(&TsToken::RightParen)?;

        let body = if let Some(block) = self.parse_block()? {
            match block {
                AstNode::Block(stmts) => stmts,
                _ => vec![block],
            }
        } else {
            Vec::new()
        };

        // Desugar for loop into while loop with init/update
        Ok(Some(AstNode::Block(vec![
            init.unwrap_or(AstNode::ExpressionStatement(Box::new(Expression::Literal(
                crate::tumc_compiler::ast::Literal::Unit
            )))),
            AstNode::While {
                condition: Box::new(condition.unwrap_or(Expression::Literal(
                    crate::tumc_compiler::ast::Literal::Bool(true)
                ))),
                body: {
                    let mut stmts = body.clone();
                    if let Some(upd) = update {
                        stmts.push(AstNode::ExpressionStatement(Box::new(upd)));
                    }
                    stmts
                },
                else_body: Vec::new(),
            },
        ])))
    }

    fn parse_do_while(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let body = if let Some(block) = self.parse_block()? {
            match block {
                AstNode::Block(stmts) => stmts,
                _ => vec![block],
            }
        } else {
            Vec::new()
        };

        self.expect(&TsToken::While)?;
        self.expect(&TsToken::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TsToken::RightParen)?;

        Ok(Some(AstNode::While {
            condition: Box::new(condition),
            body,
            else_body: Vec::new(),
        }))
    }

    fn parse_switch(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        self.expect(&TsToken::LeftParen)?;
        let discriminant = self.parse_expression()?;
        self.expect(&TsToken::RightParen);

        self.expect(&TsToken::OpenBrace)?;

        let mut cases = Vec::new();
        while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
            if self.check(&TsToken::Case) {
                self.advance();
                let value = self.parse_expression()?;
                self.expect(&TsToken::Colon)?;

                let mut statements = Vec::new();
                while !self.check(&TsToken::Case) && !self.check(&TsToken::Default) &&
                      !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
                    if let Some(stmt) = self.parse_statement()? {
                        statements.push(stmt);
                    }
                }

                cases.push(AstNode::SwitchCase {
                    test: Some(Box::new(value)),
                    consequent: statements,
                });
            } else if self.check(&TsToken::Default) {
                self.advance();
                self.expect(&TsToken::Colon)?;

                let mut statements = Vec::new();
                while !self.check(&TsToken::Case) && !self.check(&TsToken::Default) &&
                      !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
                    if let Some(stmt) = self.parse_statement()? {
                        statements.push(stmt);
                    }
                }

                cases.push(AstNode::SwitchCase {
                    test: None,
                    consequent: statements,
                });
            }
        }

        self.expect(&TsToken::CloseBrace)?;

        Ok(Some(AstNode::Switch {
            discriminant: Box::new(discriminant),
            cases,
        }))
    }

    fn parse_try(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let body = if let Some(block) = self.parse_block()? {
            match block {
                AstNode::Block(stmts) => stmts,
                _ => vec![block],
            }
        } else {
            Vec::new()
        };

        let mut handlers = Vec::new();
        let mut finally_body = Vec::new();

        if self.check(&TsToken::Catch) {
            self.advance();

            let (param, handler_body) = if self.check(&TsToken::LeftParen) {
                self.advance();
                let param = match self.current() {
                    TsToken::Identifier(n) => n.clone(),
                    _ => "e".to_string(),
                };
                self.advance();
                self.expect(&TsToken::RightParen)?;

                let body = if let Some(block) = self.parse_block()? {
                    match block {
                        AstNode::Block(stmts) => stmts,
                        _ => vec![block],
                    }
                } else {
                    Vec::new()
                };
                (Some(param), body)
            } else if self.check(&TsToken::OpenBrace) {
                let body = if let Some(block) = self.parse_block()? {
                    match block {
                        AstNode::Block(stmts) => stmts,
                        _ => vec![block],
                    }
                } else {
                    Vec::new()
                };
                (None, body)
            } else {
                (None, Vec::new())
            };

            handlers.push(crate::tumc_compiler::ast::ExceptHandler {
                exception_type: None,
                name: param,
                body: handler_body,
            });
        }

        if self.check(&TsToken::Finally) {
            self.advance();
            if let Some(block) = self.parse_block()? {
                finally_body = match block {
                    AstNode::Block(stmts) => stmts,
                    _ => vec![block],
                };
            }
        }

        Ok(Some(AstNode::Try {
            body,
            handlers,
            else_body: Vec::new(),
            finally_body,
        }))
    }

    fn parse_return(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let value = if !self.check(&TsToken::Semicolon) && !self.is_at_end() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(Some(AstNode::Return(value.map(Box::new))))
    }

    fn parse_throw(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let exc = self.parse_expression()?;

        Ok(Some(AstNode::Raise(Some(Box::new(exc)))))
    }

    fn parse_variable_declaration(&mut self) -> Result<Option<AstNode>, TsParseError> {
        let keyword = match self.current() {
            TsToken::Const => "const",
            TsToken::Let => "let",
            TsToken::Var => "var",
            _ => return Err(TsParseError::new("Expected variable keyword", self.pos)),
        };
        self.advance();

        let name = match self.current() {
            TsToken::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(TsParseError::new("Expected variable name", self.pos)),
        };

        let var_type = if self.check(&TsToken::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let value = if self.check(&TsToken::Equal) {
            self.advance();
            Some(self.parse_expression()?)
        } else {
            None
        };

        // Handle destructuring (simplified)
        if self.check(&TsToken::Comma) {
            // Multiple declarations
        }

        self.expect(&TsToken::Semicolon)?;

        Ok(Some(AstNode::Declaration {
            name,
            value: value.map(Box::new),
            mutable: keyword != "const",
        }))
    }

    fn parse_export(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        if self.check(&TsToken::Default) {
            self.advance();
            if let Some(node) = self.parse_statement()? {
                return Ok(Some(AstNode::Export {
                    node: Box::new(node),
                    is_default: true,
                }));
            }
        }

        if self.check(&TsToken::OpenBrace) {
            self.advance();
            while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
                self.advance();
                if !self.check(&TsToken::CloseBrace) {
                    self.expect(&TsToken::Comma)?;
                }
            }
            self.expect(&TsToken::CloseBrace)?;

            if self.check(&TsToken::From) {
                self.advance();
                if let TsToken::StringLiteral(s) = self.current() {
                    self.advance();
                }
            }

            self.expect(&TsToken::Semicolon)?;
            return Ok(None);
        }

        if let Some(node) = self.parse_statement()? {
            Ok(Some(AstNode::Export {
                node: Box::new(node),
                is_default: false,
            }))
        } else {
            Ok(None)
        }
    }

    fn parse_import(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        if self.check(&TsToken::OpenBrace) {
            self.advance();
            while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
                self.advance();
                if self.check(&TsToken::As) {
                    self.advance();
                    self.advance(); // alias
                }
                if !self.check(&TsToken::CloseBrace) {
                    self.expect(&TsToken::Comma)?;
                }
            }
            self.expect(&TsToken::CloseBrace)?;
        } else if self.check(&TsToken::Identifier(_)) {
            self.advance();
            if self.check(&TsToken::Comma) {
                self.advance();
                self.expect(&TsToken::OpenBrace)?;
                while !self.check(&TsToken::CloseBrace) {
                    self.advance();
                    if !self.check(&TsToken::CloseBrace) {
                        self.expect(&TsToken::Comma)?;
                    }
                }
                self.expect(&TsToken::CloseBrace)?;
            }
        }

        if self.check(&TsToken::From) {
            self.advance();
        }

        if let TsToken::StringLiteral(s) = self.current() {
            self.advance();
            self.expect(&TsToken::Semicolon)?;
        }

        Ok(None)
    }

    fn parse_interface(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let name = match self.current() {
            TsToken::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(TsParseError::new("Expected interface name", self.pos)),
        };

        let _ = self.parse_type_parameters()?;

        let mut extends = Vec::new();
        if self.check(&TsToken::Extends) {
            self.advance();
            while !self.check(&TsToken::OpenBrace) {
                if let TsToken::Identifier(n) = self.current() {
                    extends.push(n.clone());
                    self.advance();
                }
                if !self.check(&TsToken::Comma) && !self.check(&TsToken::Less) {
                    break;
                }
                if self.check(&TsToken::Comma) {
                    self.advance();
                }
            }
        }

        self.expect(&TsToken::OpenBrace)?;

        let mut members = Vec::new();
        while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
            if self.check(&TsToken::Readonly) {
                self.advance();
            }

            let member_name = match self.current() {
                TsToken::Identifier(n) => n.clone(),
                _ => String::new(),
            };
            self.advance();

            if self.check(&TsToken::LeftParen) {
                // Method signature
                self.advance();
                while !self.check(&TsToken::RightParen) {
                    self.advance();
                }
                self.expect(&TsToken::RightParen)?;

                if self.check(&TsToken::Colon) {
                    self.advance();
                    let _ = self.parse_type()?;
                }

                self.expect(&TsToken::Semicolon)?;
            } else {
                // Property
                let optional = self.check(&TsToken::Question);
                if optional {
                    self.advance();
                }

                if self.check(&TsToken::Colon) {
                    self.advance();
                    let _ = self.parse_type()?;
                }

                self.expect(&TsToken::Semicolon)?;
            }

            members.push(TsTypeMember::Property {
                name: member_name,
                type_annotation: Type::Unknown,
                optional,
                readonly: false,
            });
        }

        self.expect(&TsToken::CloseBrace)?;

        Ok(Some(AstNode::Extension(TsAstNode::Interface {
            name,
            type_params: Vec::new(),
            extends,
            body: members,
        }.into())))
    }

    fn parse_type_alias(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let name = match self.current() {
            TsToken::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(TsParseError::new("Expected type name", self.pos)),
        };

        let _ = self.parse_type_parameters()?;

        self.expect(&TsToken::Equal)?;

        let type_annotation = self.parse_type()?;

        self.expect(&TsToken::Semicolon)?;

        Ok(Some(AstNode::Extension(TsAstNode::TypeAlias {
            name,
            type_params: Vec::new(),
            type_annotation,
        }.into())))
    }

    fn parse_enum(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let name = match self.current() {
            TsToken::Identifier(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(TsParseError::new("Expected enum name", self.pos)),
        };

        self.expect(&TsToken::OpenBrace)?;

        let mut members = Vec::new();
        let mut next_value = 0i64;

        while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
            let member_name = match self.current() {
                TsToken::Identifier(n) => n.clone(),
                _ => String::new(),
            };
            self.advance();

            let value = if self.check(&TsToken::Equal) {
                self.advance();
                next_value = match self.current() {
                    TsToken::NumericLiteral(n) => n as i64,
                    _ => next_value,
                };
                self.advance();
                Some(Expression::Literal(crate::tumc_compiler::ast::Literal::Int(next_value)))
            } else {
                let v = next_value;
                next_value += 1;
                Some(Expression::Literal(crate::tumc_compiler::ast::Literal::Int(v)))
            };

            members.push(EnumMember {
                name: member_name,
                value,
            });

            if !self.check(&TsToken::CloseBrace) {
                self.expect(&TsToken::Comma)?;
            }
        }

        self.expect(&TsToken::CloseBrace)?;

        Ok(Some(AstNode::Extension(TsAstNode::Enum {
            name,
            members,
            is_const: false,
        }.into())))
    }

    fn parse_namespace(&mut self) -> Result<Option<AstNode>, TsParseError> {
        self.advance();

        let name = match self.current() {
            TsToken::Identifier(n) => n.clone(),
            TsToken::StringLiteral(s) => s.clone(),
            _ => String::new(),
        };
        self.advance();

        self.expect(&TsToken::OpenBrace)?;

        let mut body = Vec::new();
        while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
            if let Some(stmt) = self.parse_statement()? {
                body.push(stmt);
            }
        }

        self.expect(&TsToken::CloseBrace)?;

        Ok(Some(AstNode::Extension(TsAstNode::Namespace {
            name,
            body,
        }.into())))
    }

    fn parse_type_parameters(&mut self) -> Result<Vec<String>, TsParseError> {
        let mut params = Vec::new();

        if self.check(&TsToken::Less) {
            self.advance();
            while !self.check(&TsToken::Greater) && !self.is_at_end() {
                if let TsToken::Identifier(n) = self.current() {
                    params.push(n.clone());
                    self.advance();
                }
                if !self.check(&TsToken::Greater) {
                    self.expect(&TsToken::Comma)?;
                }
            }
            self.expect(&TsToken::Greater)?;
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Type, TsParseError> {
        self.parse_union_type()
    }

    fn parse_union_type(&mut self) -> Result<Type, TsParseError> {
        let first = self.parse_intersection_type()?;

        if self.check(&TsToken::Pipe) {
            let mut types = vec![first];
            while self.check(&TsToken::Pipe) {
                self.advance();
                types.push(self.parse_intersection_type()?);
            }
            Ok(Type::Unknown) // Would create union type
        } else {
            Ok(first)
        }
    }

    fn parse_intersection_type(&mut self) -> Result<Type, TsParseError> {
        let first = self.parse_primary_type()?;

        if self.check(&TsToken::Ampersand) {
            let mut types = vec![first];
            while self.check(&TsToken::Ampersand) {
                self.advance();
                types.push(self.parse_primary_type()?);
            }
            Ok(Type::Unknown) // Would create intersection type
        } else {
            Ok(first)
        }
    }

    fn parse_primary_type(&mut self) -> Result<Type, TsParseError> {
        match self.current() {
            TsToken::String | TsToken::StringLiteral(_) => {
                self.advance();
                Ok(Type::String)
            }
            TsToken::Number | TsToken::NumericLiteral(_) => {
                self.advance();
                Ok(Type::F64)
            }
            TsToken::Boolean | TsToken::True | TsToken::False => {
                self.advance();
                Ok(Type::Bool)
            }
            TsToken::Any => {
                self.advance();
                Ok(Type::Unknown)
            }
            TsToken::Unknown => {
                self.advance();
                Ok(Type::Unknown)
            }
            TsToken::Never => {
                self.advance();
                Ok(Type::Unknown)
            }
            TsToken::Void => {
                self.advance();
                Ok(Type::Unit)
            }
            TsToken::Null => {
                self.advance();
                Ok(Type::Unknown)
            }
            TsToken::Undefined => {
                self.advance();
                Ok(Type::Unknown)
            }
            TsToken::Identifier(_) => {
                self.advance();
                Ok(Type::Unknown) // Would resolve type name
            }
            TsToken::OpenBracket => {
                self.advance();
                if self.check(&TsToken::CloseBracket) {
                    self.advance();
                    // Array type []
                    self.expect(&TsToken::Less)?;
                    let elem_type = self.parse_type()?;
                    self.expect(&TsToken::Greater)?;
                    Ok(Type::Array(Box::new(elem_type), None))
                } else {
                    let elem_type = self.parse_type()?;
                    self.expect(&TsToken::CloseBracket)?;
                    Ok(Type::Array(Box::new(elem_type), None))
                }
            }
            TsToken::OpenBrace => {
                self.advance();
                // Object type
                while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
                    self.advance();
                    if self.check(&TsToken::Comma) {
                        self.advance();
                    }
                }
                self.expect(&TsToken::CloseBrace)?;
                Ok(Type::Unknown)
            }
            TsToken::LeftParen => {
                self.advance();
                // Function type
                while !self.check(&TsToken::RightParen) {
                    self.advance();
                }
                self.expect(&TsToken::RightParen)?;
                if self.check(&TsToken::Arrow) {
                    self.advance();
                    let _ = self.parse_type()?;
                }
                Ok(Type::Function(Box::new(Type::Unknown), Vec::new()))
            }
            TsToken::Readonly => {
                self.advance();
                self.parse_type()
            }
            TsToken::Keyof => {
                self.advance();
                let inner = self.parse_type()?;
                Ok(Type::Unknown) // Would create keyof type
            }
            TsToken::LsRo => {
                self.advance();
                let key_type = self.parse_type()?;
                self.expect(&TsToken::RsRo)?;
                self.expect(&TsToken::Comma)?;
                let value_type = self.parse_type()?;
                Ok(Type::Unknown) // Index signature
            }
            _ => {
                self.advance();
                Ok(Type::Unknown)
            }
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Option<AstNode>, TsParseError> {
        let expr = self.parse_expression()?;

        // Check for arrow function
        if self.check(&TsToken::Equal) && self.check_next(&TsToken::Greater) {
            // This would be handled differently
        }

        self.expect(&TsToken::Semicolon)?;

        Ok(Some(AstNode::ExpressionStatement(Box::new(expr))))
    }

    fn parse_expression(&mut self) -> Result<Expression, TsParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_ternary()?;

        match self.current() {
            TsToken::Equal => {
                self.advance();
                let value = Box::new(self.parse_assignment()?);
                Ok(Expression::Assignment {
                    target: Box::new(expr.into()),
                    value,
                })
            }
            TsToken::PlusEqual | TsToken::MinusEqual | TsToken::StarEqual |
            TsToken::SlashEqual | TsToken::PercentEqual | TsToken::AmpersandEqual |
            TsToken::PipeEqual | TsToken::CaretEqual | TsToken::LeftShiftEqual |
            TsToken::RightShiftEqual | TsToken::ZeroFillRightShiftEqual => {
                self.advance();
                let value = Box::new(self.parse_assignment()?);
                Ok(Expression::Assignment {
                    target: Box::new(expr.into()),
                    value,
                })
            }
            TsToken::QuestionQuestion => {
                self.advance();
                let value = Box::new(self.parse_assignment()?);
                Ok(Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op: crate::tumc_compiler::ast::BinaryOp::NullishCoalesce,
                    right: value,
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_ternary(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_coalesce()?;

        if self.check(&TsToken::Question) {
            self.advance();
            let consequent = Box::new(self.parse_assignment()?);
            self.expect(&TsToken::Colon)?;
            let alternate = Box::new(self.parse_assignment()?);
            return Ok(Expression::Conditional {
                test: Box::new(expr.into()),
                consequent,
                alternate,
            });
        }

        Ok(expr)
    }

    fn parse_coalesce(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_or()?;

        if self.check(&TsToken::PipePipe) {
            self.advance();
            let right = Box::new(self.parse_coalesce()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::Or,
                right,
            });
        }

        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_and()?;

        if self.check(&TsToken::AmpersandAmpersand) {
            self.advance();
            let right = Box::new(self.parse_or()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::And,
                right,
            });
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_bitwise_or()?;

        if self.check(&TsToken::Ampersand) {
            self.advance();
            let right = Box::new(self.parse_and()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitAnd,
                right,
            });
        }

        Ok(expr)
    }

    fn parse_bitwise_or(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_bitwise_xor()?;

        if self.check(&TsToken::Pipe) {
            self.advance();
            let right = Box::new(self.parse_bitwise_or()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitOr,
                right,
            });
        }

        Ok(expr)
    }

    fn parse_bitwise_xor(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_bitwise_and()?;

        if self.check(&TsToken::Caret) {
            self.advance();
            let right = Box::new(self.parse_bitwise_xor()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitXor,
                right,
            });
        }

        Ok(expr)
    }

    fn parse_bitwise_and(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_equality()?;

        if self.check(&TsToken::Ampersand) && !self.check(&TsToken::AmpersandAmpersand) {
            self.advance();
            let right = Box::new(self.parse_bitwise_and()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::BitAnd,
                right,
            });
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_comparison()?;

        match self.current() {
            TsToken::EqualEqual | TsToken::EqualEqualEqual => {
                self.advance();
                let right = Box::new(self.parse_equality()?);
                Ok(Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op: crate::tumc_compiler::ast::BinaryOp::Eq,
                    right,
                })
            }
            TsToken::BangEqual | TsToken::StrictNotEqual => {
                self.advance();
                let right = Box::new(self.parse_equality()?);
                Ok(Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op: crate::tumc_compiler::ast::BinaryOp::Ne,
                    right,
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_comparison(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_shift()?;

        match self.current() {
            TsToken::Less | TsToken::LessEqual | TsToken::Greater | TsToken::GreaterEqual => {
                let op = match self.current() {
                    TsToken::Less => crate::tumc_compiler::ast::BinaryOp::Lt,
                    TsToken::LessEqual => crate::tumc_compiler::ast::BinaryOp::Le,
                    TsToken::Greater => crate::tumc_compiler::ast::BinaryOp::Gt,
                    TsToken::GreaterEqual => crate::tumc_compiler::ast::BinaryOp::Ge,
                    _ => return Ok(expr),
                };
                self.advance();
                let right = Box::new(self.parse_comparison()?);
                Ok(Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op,
                    right,
                })
            }
            TsToken::Instanceof => {
                self.advance();
                let right = Box::new(self.parse_comparison()?);
                Ok(Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op: crate::tumc_compiler::ast::BinaryOp::Is,
                    right,
                })
            }
            TsToken::In => {
                self.advance();
                let right = Box::new(self.parse_comparison()?);
                Ok(Expression::BinaryOp {
                    left: Box::new(expr.into()),
                    op: crate::tumc_compiler::ast::BinaryOp::In,
                    right,
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_shift(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_additive()?;

        if self.check(&TsToken::LeftShift) {
            self.advance();
            let right = Box::new(self.parse_shift()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::Shl,
                right,
            });
        }

        if self.check(&TsToken::RightShift) {
            self.advance();
            let right = Box::new(self.parse_shift()?);
            return Ok(Expression::BinaryOp {
                left: Box::new(expr.into()),
                op: crate::tumc_compiler::ast::BinaryOp::Shr,
                right,
            });
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expression, TsParseError> {
        let mut expr = self.parse_multiplicative()?;

        loop {
            let op = match self.current() {
                TsToken::Plus => crate::tumc_compiler::ast::BinaryOp::Add,
                TsToken::Minus => crate::tumc_compiler::ast::BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = Box::new(self.parse_multiplicative()?);
            expr = Expression::BinaryOp {
                left: Box::new(expr.into()),
                op,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, TsParseError> {
        let mut expr = self.parse_exponentiation()?;

        loop {
            let op = match self.current() {
                TsToken::Star => crate::tumc_compiler::ast::BinaryOp::Mul,
                TsToken::Slash => crate::tumc_compiler::ast::BinaryOp::Div,
                TsToken::Percent => crate::tumc_compiler::ast::BinaryOp::Rem,
                _ => break,
            };
            self.advance();
            let right = Box::new(self.parse_exponentiation()?);
            expr = Expression::BinaryOp {
                left: Box::new(expr.into()),
                op,
                right,
            };
        }

        Ok(expr)
    }

    fn parse_exponentiation(&mut self) -> Result<Expression, TsParseError> {
        let expr = self.parse_unary()?;

        // TypeScript doesn't have ** operator yet
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, TsParseError> {
        match self.current() {
            TsToken::Bang => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Not,
                    operand,
                })
            }
            TsToken::Minus => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Negate,
                    operand,
                })
            }
            TsToken::Plus => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Plus,
                    operand,
                })
            }
            TsToken::PlusPlus => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Increment,
                    operand,
                })
            }
            TsToken::MinusMinus => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Decrement,
                    operand,
                })
            }
            TsToken::TypeOf => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::TypeOf,
                    operand,
                })
            }
            TsToken::Void => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Void,
                    operand,
                })
            }
            TsToken::Delete => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::UnaryOp {
                    op: crate::tumc_compiler::ast::UnaryOp::Delete,
                    operand,
                })
            }
            TsToken::Await => {
                self.advance();
                let operand = Box::new(self.parse_unary()?);
                Ok(Expression::Await(Box::new(operand.into())))
            }
            TsToken::Async => {
                self.advance();
                self.parse_unary()
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> Result<Expression, TsParseError> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current() {
                TsToken::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();

                    while !self.check(&TsToken::RightParen) {
                        if self.check(&TsToken::DotDotDot) {
                            self.advance();
                            args.push(crate::tumc_compiler::ast::Arg::Variadic(
                                Box::new(self.parse_expression()?)
                            ));
                        } else {
                            args.push(crate::tumc_compiler::ast::Arg::Positional(
                                Box::new(self.parse_expression()?)
                            ));
                        }

                        if !self.check(&TsToken::RightParen) {
                            self.expect(&TsToken::Comma)?;
                        }
                    }

                    self.expect(&TsToken::RightParen)?;

                    expr = Expression::Call {
                        function: Box::new(expr.into()),
                        arguments: args,
                    };
                }
                TsToken::Dot => {
                    self.advance();
                    match self.current() {
                        TsToken::Identifier(n) => {
                            expr = Expression::FieldAccess {
                                object: Box::new(expr.into()),
                                field: n.clone(),
                            };
                            self.advance();
                        }
                        _ => return Err(TsParseError::new("Expected identifier after '.'", self.pos)),
                    }
                }
                TsToken::Question => {
                    self.advance();
                    if self.check(&TsToken::Dot) {
                        self.advance();
                        match self.current() {
                            TsToken::Identifier(n) => {
                                expr = Expression::FieldAccess {
                                    object: Box::new(expr.into()),
                                    field: n.clone(),
                                };
                                self.advance();
                            }
                            _ => return Err(TsParseError::new("Expected identifier", self.pos)),
                        }
                    } else if self.check(&TsToken::LeftBracket) {
                        self.advance();
                        let index = Box::new(self.parse_expression()?);
                        self.expect(&TsToken::RightBracket)?;
                        expr = Expression::OptionalChain {
                            base: Box::new(expr.into()),
                            path: vec![crate::tumc_compiler::ast::ChainElement::Index(index)],
                        };
                    } else {
                        // Optional call
                        if self.check(&TsToken::LeftParen) {
                            self.advance();
                            self.expect(&TsToken::RightParen)?;
                            expr = Expression::Call {
                                function: Box::new(expr.into()),
                                arguments: Vec::new(),
                            };
                        }
                    }
                }
                TsToken::LeftBracket => {
                    self.advance();
                    let index = Box::new(self.parse_expression()?);
                    self.expect(&TsToken::RightBracket)?;
                    expr = Expression::Index {
                        array: Box::new(expr.into()),
                        index,
                    };
                }
                TsToken::TemplateHead(_) | TsToken::TemplateMiddle(_) => {
                    // Template literal
                    let mut parts = vec![expr.into()];

                    while matches!(self.current(), TsToken::TemplateHead(_) |
                                  TsToken::TemplateMiddle(_) | TsToken::TemplateTail(_)) {
                        match self.current() {
                            TsToken::TemplateHead(s) => {
                                parts.push(Expression::Literal(
                                    crate::tumc_compiler::ast::Literal::String(s.clone())
                                ).into());
                                self.advance();
                            }
                            TsToken::TemplateMiddle(s) => {
                                parts.push(Expression::Literal(
                                    crate::tumc_compiler::ast::Literal::String(s.clone())
                                ).into());
                                self.advance();
                            }
                            TsToken::TemplateTail(s) => {
                                parts.push(Expression::Literal(
                                    crate::tumc_compiler::ast::Literal::String(s.clone())
                                ).into());
                                self.advance();
                            }
                            _ => break,
                        }

                        // Expression in template
                        parts.push(self.parse_expression()?.into());

                        if matches!(self.current(), TsToken::TemplateMiddle(_)) {
                            continue;
                        }
                    }

                    if let TsToken::TemplateTail(s) = self.current() {
                        parts.push(Expression::Literal(
                            crate::tumc_compiler::ast::Literal::String(s.clone())
                        ).into());
                        self.advance();
                    }

                    // Simplify: just use the first part for now
                    if !parts.is_empty() {
                        if let Expression::Literal(lit) = parts.remove(0) {
                            expr = lit.into();
                        }
                    }
                }
                TsToken::Bang => {
                    // Non-null assertion
                    self.advance();
                    expr = Expression::UnaryOp {
                        op: crate::tumc_compiler::ast::UnaryOp::NonNull,
                        operand: Box::new(expr.into()),
                    };
                }
                TsToken::As => {
                    // As expression
                    self.advance();
                    let type_annotation = self.parse_type()?;
                    expr = Expression::UnaryOp {
                        op: crate::tumc_compiler::ast::UnaryOp::Cast,
                        operand: Box::new(expr.into()),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, TsParseError> {
        let token = self.current().clone();

        match token {
            TsToken::NumericLiteral(n) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Float(n)))
            }
            TsToken::StringLiteral(s) => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::String(s)))
            }
            TsToken::True => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Bool(true)))
            }
            TsToken::False => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Bool(false)))
            }
            TsToken::Null => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Null))
            }
            TsToken::Undefined => {
                self.advance();
                Ok(Expression::Literal(crate::tumc_compiler::ast::Literal::Undefined))
            }
            TsToken::This => {
                self.advance();
                Ok(Expression::This)
            }
            TsToken::Identifier(n) => {
                self.advance();

                // Check for arrow function
                if self.check(&TsToken::Equal) && self.check_next(&TsToken::Greater) {
                    // Would handle arrow function
                }

                Ok(Expression::Identifier(n))
            }
            TsToken::OpenParen => {
                self.advance();

                if self.check(&TsToken::CloseParen) {
                    self.advance();
                    return Ok(Expression::Tuple(Vec::new()));
                }

                let expr = self.parse_expression()?;

                if self.check(&TsToken::Comma) {
                    self.advance();
                    let mut elements = vec![expr];
                    while !self.check(&TsToken::CloseParen) {
                        elements.push(self.parse_expression()?);
                        if !self.check(&TsToken::CloseParen) {
                            self.expect(&TsToken::Comma)?;
                        }
                    }
                    self.expect(&TsToken::CloseParen)?;
                    Ok(Expression::Tuple(elements))
                } else {
                    self.expect(&TsToken::CloseParen)?;
                    Ok(expr)
                }
            }
            TsToken::OpenBracket => {
                self.advance();
                let mut elements = Vec::new();

                while !self.check(&TsToken::CloseBracket) {
                    elements.push(self.parse_expression()?);
                    if !self.check(&TsToken::CloseBracket) {
                        self.expect(&TsToken::Comma)?;
                    }
                }

                self.expect(&TsToken::CloseBracket)?;
                Ok(Expression::Array(elements))
            }
            TsToken::OpenBrace => {
                self.advance();
                let mut properties = Vec::new();

                while !self.check(&TsToken::CloseBrace) && !self.is_at_end() {
                    let key = match self.current() {
                        TsToken::Identifier(n) => n.clone(),
                        TsToken::StringLiteral(s) => s,
                        TsToken::NumericLiteral(n) => n.to_string(),
                        _ => {
                            self.advance();
                            continue;
                        }
                    };
                    self.advance();

                    let value = if self.check(&TsToken::Colon) {
                        self.advance();
                        self.parse_expression()?
                    } else if self.check(&TsToken::OpenParen) {
                        // Method
                        self.parse_expression()?
                    } else {
                        Expression::Identifier(key.clone())
                    };

                    properties.push((Expression::Identifier(key).into(), value.into()));

                    if !self.check(&TsToken::CloseBrace) {
                        self.expect(&TsToken::Comma)?;
                    }
                }

                self.expect(&TsToken::CloseBrace)?;
                Ok(Expression::Object(properties))
            }
            TsToken::Function => {
                self.advance();
                let name = match self.current() {
                    TsToken::Identifier(n) => {
                        let n = n.clone();
                        self.advance();
                        Some(n)
                    }
                    _ => None,
                };

                self.expect(&TsToken::LeftParen)?;
                while !self.check(&TsToken::RightParen) {
                    self.advance();
                }
                self.expect(&TsToken::RightParen)?;

                let return_type = if self.check(&TsToken::Colon) {
                    self.advance();
                    let _ = self.parse_type();
                } else {
                    None
                };

                let body = if self.check(&TsToken::OpenBrace) {
                    self.parse_block()?
                } else {
                    None
                };

                Ok(Expression::Function(name, Vec::new(), Box::new(
                    Expression::Block(body.map(|b| match b {
                        AstNode::Block(s) => s,
                        _ => vec![b],
                    }).unwrap_or_default()).into()
                )))
            }
            TsToken::Async => {
                self.advance();
                if self.check(&TsToken::Function) {
                    self.advance();
                    let name = match self.current() {
                        TsToken::Identifier(n) => {
                            let n = n.clone();
                            self.advance();
                            Some(n)
                        }
                        _ => None,
                    };

                    self.expect(&TsToken::LeftParen)?;
                    while !self.check(&TsToken::RightParen) {
                        self.advance();
                    }
                    self.expect(&TsToken::RightParen)?;

                    let body = if self.check(&TsToken::OpenBrace) {
                        self.parse_block()?
                    } else {
                        None
                    };

                    Ok(Expression::Function(name, Vec::new(), Box::new(
                        Expression::Block(body.map(|b| match b {
                            AstNode::Block(s) => s,
                            _ => vec![b],
                        }).unwrap_or_default()).into()
                    )))
                } else if self.check(&TsToken::OpenParen) || self.check(&TsToken::Identifier(_)) {
                    self.parse_expression()
                } else {
                    Err(TsParseError::new("Unexpected token", self.pos))
                }
            }
            TsToken::Arrow => {
                self.advance();
                // Arrow function with no params
                let body = if self.check(&TsToken::OpenBrace) {
                    self.parse_block()?
                } else {
                    let expr = self.parse_expression()?;
                    vec![AstNode::Return(Some(Box::new(expr)))]
                };

                Ok(Expression::Lambda {
                    params: Vec::new(),
                    body: Box::new(Expression::Block(body.into()).into()),
                })
            }
            TsToken::New => {
                self.advance();
                let constructor = Box::new(self.parse_expression()?);
                self.expect(&TsToken::LeftParen)?;
                let mut args = Vec::new();

                while !self.check(&TsToken::RightParen) {
                    args.push(crate::tumc_compiler::ast::Arg::Positional(
                        Box::new(self.parse_expression()?)
                    ));
                    if !self.check(&TsToken::RightParen) {
                        self.expect(&TsToken::Comma)?;
                    }
                }

                self.expect(&TsToken::RightParen)?;

                Ok(Expression::New {
                    constructor,
                    arguments: args,
                })
            }
            TsToken::Super => {
                self.advance();
                if self.check(&TsToken::LeftParen) {
                    self.advance();
                    self.expect(&TsToken::RightParen)?;
                    Ok(Expression::Super)
                } else if self.check(&TsToken::Dot) {
                    self.advance();
                    match self.current() {
                        TsToken::Identifier(n) => {
                            self.advance();
                            Ok(Expression::FieldAccess {
                                object: Box::new(Expression::Super.into()),
                                field: n,
                            })
                        }
                        _ => Err(TsParseError::new("Expected identifier", self.pos)),
                    }
                } else {
                    Ok(Expression::Super)
                }
            }
            TsToken::Yield => {
                self.advance();
                if self.check(&TsToken::Semicolon) || self.check(&TsToken::CloseParen) {
                    Ok(Expression::Yield(None))
                } else {
                    let value = self.parse_expression()?;
                    Ok(Expression::Yield(Some(Box::new(value))))
                }
            }
            TsToken::Class => {
                self.parse_class()?;
                Ok(Expression::Identifier("class".to_string()))
            }
            _ => {
                self.advance();
                Err(TsParseError::new(&format!("Unexpected token: {:?}", token), self.pos))
            }
        }
    }

    // Helper methods
    fn current(&self) -> &TsToken {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &TsToken::EndOfFile
        }
    }

    fn check(&self, token: &TsToken) -> bool {
        std::mem::discriminant(self.current()) == std::mem::discriminant(token)
    }

    fn check_next(&self, token: &TsToken) -> bool {
        if self.pos + 1 < self.tokens.len() {
            std::mem::discriminant(&self.tokens[self.pos + 1]) == std::mem::discriminant(token)
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
        self.pos >= self.tokens.len() || matches!(self.current(), TsToken::EndOfFile)
    }

    fn expect(&mut self, token: &TsToken) -> Result<(), TsParseError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(TsParseError::new(&format!("Expected {:?}, found {:?}", token, self.current()), self.pos))
        }
    }
}

/// TypeScript parse error
#[derive(Debug)]
pub struct TsParseError {
    pub message: String,
    pub position: usize,
}

impl TsParseError {
    pub fn new(message: &str, position: usize) -> Self {
        Self {
            message: message.to_string(),
            position,
        }
    }
}

impl std::fmt::Display for TsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at {}: {}", self.position, self.message)
    }
}

impl std::error::Error for TsParseError {}

/// TypeScript lexer error
#[derive(Debug)]
pub struct TsLexerError {
    pub message: String,
    pub position: usize,
}

impl std::fmt::Display for TsLexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lexer error at {}: {}", self.position, self.message)
    }
}

impl std::error::Error for TsLexerError {}

/// TypeScript transpiler
pub struct TypeScriptTranspiler {
    /// Type environment
    type_env: HashMap<String, Type>,
}

impl TypeScriptTranspiler {
    pub fn new() -> Self {
        Self {
            type_env: HashMap::new(),
        }
    }

    pub fn transpile(&mut self, source: &str) -> Result<Program, TsParseError> {
        let mut parser = TypeScriptParser::new(source);
        parser.parse()
    }
}

impl Default for TypeScriptTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_keywords() {
        let source = "function const let var if else for while return";
        let lexer = TypeScriptLexer::new(source);
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0);
    }

    #[test]
    fn test_parse_function() {
        let source = "function add(a: number, b: number): number { return a + b; }";
        let mut parser = TypeScriptParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_class() {
        let source = "class Point { x: number; y: number; }";
        let mut parser = TypeScriptParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_arrow_function() {
        let source = "const add = (a: number, b: number): number => a + b;";
        let mut parser = TypeScriptParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_interface() {
        let source = "interface Point { x: number; y: number; }";
        let mut parser = TypeScriptParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }

    #[test]
    fn test_parse_enum() {
        let source = "enum Color { Red, Green, Blue }";
        let mut parser = TypeScriptParser::new(source);
        let ast = parser.parse();
        assert!(ast.is_ok());
    }
}
