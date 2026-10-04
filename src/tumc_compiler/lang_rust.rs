// Rust Language Frontend for TUMC
// Parses Rust source code into TUMC's intermediate representation

use crate::tumc_compiler::ir::{AstNode, Type, Literal, Operator, Visibility};
use std::collections::HashMap;

// ============================================================================
// Token Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum RustTokenType {
    // Keywords
    Fn, Let, Mut, Const, Static, If, Else, Match, Loop, While, For, In, Return, Break, Continue,
    Struct, Enum, Trait, Impl, Type, Use, Mod, Pub, Crate, Super, SelfKeyword, SelfType,
    Async, Await, Move, Ref, As, Where, Unsafe, Extern, Dyn, Box, Some, None, Ok, Err,
    True, False,

    // Symbols
    LBrace, RBrace, LParen, RParen, LBracket, RBracket, Semi, Comma, Dot, DotDot, DotDotEq,
    DotDotDot, Arrow, FatArrow, Colon, DoubleColon, PathSep, Bang, BangEq, Eq, EqEq,
    Lt, LtEq, Gt, GtEq, Amp, AmpAmp, AmpEq, Bar, BarBar, BarEq, Plus, PlusEq,
    Minus, MinusEq, Star, StarEq, Slash, SlashEq, Percent, PercentEq, Caret, CaretEq,
    Dollar, Question, At, Pound, Tilde,

    // Literals
    Integer(i64, Option<String>),  // value, suffix
    Float(f64, Option<String>),    // value, suffix
    String(String),
    Char(char),
    Byte(u8),
    ByteString(Vec<u8>),
    RawString(String, u32),        // content, hash count
    Ident(String),
    Lifetime(String),
    DocComment(String, bool),      // content, is_block
    LineComment(String),

    // Special
    Eof,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct RustToken {
    pub token_type: RustTokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub span: (usize, usize),
}

// ============================================================================
// Lexer
// ============================================================================

pub struct RustLexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    tokens: Vec<RustToken>,
    start: usize,
}

impl RustLexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
            start: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        self.source.get(self.pos + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current_char();
        if ch.is_some() {
            self.pos += 1;
            if ch == Some('\n') {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn lexeme_str(&self) -> String {
        self.source[self.start..self.pos].iter().collect()
    }

    fn add_token(&mut self, token_type: RustTokenType) {
        let lexeme = self.lexeme_str();
        self.tokens.push(RustToken {
            token_type,
            lexeme,
            line: self.line,
            column: self.column - self.lexeme_str().len(),
            span: (self.start, self.pos),
        });
        self.start = self.pos;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => { self.advance(); }
                '/' => {
                    match self.peek_char(1) {
                        Some('/') => {
                            // Line comment
                            while self.current_char() != Some('\n') && self.current_char().is_some() {
                                self.advance();
                            }
                        }
                        Some('*') => {
                            // Block comment (possibly nested)
                            let mut depth = 1;
                            self.advance(); self.advance();
                            while depth > 0 && self.current_char().is_some() {
                                if self.current_char() == Some('/') && self.peek_char(1) == Some('*') {
                                    self.advance(); self.advance();
                                    depth += 1;
                                } else if self.current_char() == Some('*') && self.peek_char(1) == Some('/') {
                                    self.advance(); self.advance();
                                    depth -= 1;
                                } else {
                                    self.advance();
                                }
                            }
                        }
                        _ => break,
                    }
                }
                '#' => {
                    // Shebang or attribute
                    if self.line == 1 && self.lexeme_str().is_empty() && self.peek_char(1) == Some('!') {
                        while self.current_char() != Some('\n') && self.current_char().is_some() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        self.start = self.pos;
    }

    fn read_string(&mut self, quote_char: char, raw: bool, hashes: u32) -> String {
        let mut value = String::new();
        if raw {
            // Skip until closing hashes + quote
            while self.current_char().is_some() {
                if self.source.get(self.pos..).unwrap_or_default().starts_with(&format!("\"{}", "#".repeat(hashes as usize))) {
                    // Skip closing
                    for _ in 0..(hashes + 1) {
                        self.advance();
                    }
                    break;
                }
                value.push(self.advance().unwrap_or('"'));
            }
        } else {
            while self.current_char() != Some(quote_char) && self.current_char().is_some() {
                if self.current_char() == Some('\\') {
                    self.advance();
                    match self.advance() {
                        Some('n') => value.push('\n'),
                        Some('r') => value.push('\r'),
                        Some('t') => value.push('\t'),
                        Some('\\') => value.push('\\'),
                        Some('\'') => value.push('\''),
                        Some('"') => value.push('"'),
                        Some('0') => value.push('\0'),
                        Some('x') => {
                            // Hex escape
                            let h1 = self.advance().unwrap_or('0');
                            let h2 = self.advance().unwrap_or('0');
                            let hex = format!("{}{}", h1, h2);
                            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                value.push(byte as char);
                            }
                        }
                        Some('u') => {
                            // Unicode escape
                            self.advance(); // {
                            let mut uni = String::new();
                            while self.current_char() != Some('}') && self.current_char().is_some() {
                                uni.push(self.advance().unwrap_or('}'));
                            }
                            self.advance(); // }
                            if let Ok(c) = u32::from_str_radix(&uni, 16).map(char::from_u32) {
                                value.push(c.unwrap_or('\u{FFFD}'));
                            }
                        }
                        _ => {}
                    }
                } else {
                    value.push(self.advance().unwrap_or('"'));
                }
            }
            self.advance(); // Closing quote
        }
        value
    }

    fn read_number(&mut self, first_char: char) -> RustTokenType {
        let mut value = String::new();
        value.push(first_char);

        // Check for hex, octal, or binary
        let base = if first_char == '0' {
            match self.peek_char(1) {
                Some('x') | Some('X') => { self.advance(); self.advance(); value.push('x'); 16 }
                Some('o') | Some('O') => { self.advance(); self.advance(); value.push('o'); 8 }
                Some('b') | Some('B') => { self.advance(); self.advance(); value.push('b'); 2 }
                _ => 10,
            }
        } else {
            10
        };

        // Read digits
        while let Some(ch) = self.current_char() {
            if ch.is_digit(base) || (base == 16 && ch.is_ascii_hexdigit()) {
                value.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        // Check for type suffix
        let suffix = if let Some(ch) = self.current_char() {
            if ch == 'i' || ch == 'u' || ch == 'f' {
                let mut s = String::new();
                s.push(self.advance().unwrap());
                // Handle double-char suffixes like i128, u128
                while let Some(c) = self.current_char() {
                    if c.is_ascii_alphabetic() {
                        s.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
                Some(s)
            } else {
                None
            }
        } else {
            None
        };

        // Floating point check
        if self.current_char() == Some('.') && self.peek_char(1) != Some('.')
            && (base == 10 || suffix.is_none()) {
            value.push(self.advance().unwrap());
            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    value.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
            // Exponent
            if self.current_char() == Some('e') || self.current_char() == Some('E') {
                value.push(self.advance().unwrap());
                if self.current_char() == Some('+') || self.current_char() == Some('-') {
                    value.push(self.advance().unwrap());
                }
                while let Some(ch) = self.current_char() {
                    if ch.is_ascii_digit() {
                        value.push(self.advance().unwrap());
                    } else {
                        break;
                    }
                }
            }
            // Float suffix
            let float_suffix = if let Some(ch) = self.current_char() {
                if ch == 'f' {
                    let mut s = String::new();
                    s.push(self.advance().unwrap());
                    while let Some(c) = self.current_char() {
                        if c.is_ascii_alphabetic() {
                            s.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }
                    Some(s)
                } else {
                    None
                }
            } else {
                None
            };

            let fvalue = value.replace("_", "").parse::<f64>().unwrap_or(0.0);
            return RustTokenType::Float(fvalue, float_suffix.or(suffix));
        }

        // Integer
        let clean_value = value.replace("_", "").replace("x", "").replace("o", "").replace("b", "");
        let ivalue = i64::from_str_radix(&clean_value, base).unwrap_or(0);
        RustTokenType::Integer(ivalue, suffix)
    }

    fn read_identifier(&mut self, first_char: char) -> RustTokenType {
        let mut value = String::new();
        value.push(first_char);

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                value.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        // Check if it's a keyword
        match value.as_str() {
            "fn" => RustTokenType::Fn,
            "let" => RustTokenType::Let,
            "mut" => RustTokenType::Mut,
            "const" => RustTokenType::Const,
            "static" => RustTokenType::Static,
            "if" => RustTokenType::If,
            "else" => RustTokenType::Else,
            "match" => RustTokenType::Match,
            "loop" => RustTokenType::Loop,
            "while" => RustTokenType::While,
            "for" => RustTokenType::For,
            "in" => RustTokenType::In,
            "return" => RustTokenType::Return,
            "break" => RustTokenType::Break,
            "continue" => RustTokenType::Continue,
            "struct" => RustTokenType::Struct,
            "enum" => RustTokenType::Enum,
            "trait" => RustTokenType::Trait,
            "impl" => RustTokenType::Impl,
            "type" => RustTokenType::Type,
            "use" => RustTokenType::Use,
            "mod" => RustTokenType::Mod,
            "pub" => RustTokenType::Pub,
            "crate" => RustTokenType::Crate,
            "super" => RustTokenType::Super,
            "self" => RustTokenType::SelfKeyword,
            "Self" => RustTokenType::SelfType,
            "async" => RustTokenType::Async,
            "await" => RustTokenType::Await,
            "move" => RustTokenType::Move,
            "ref" => RustTokenType::Ref,
            "as" => RustTokenType::As,
            "where" => RustTokenType::Where,
            "unsafe" => RustTokenType::Unsafe,
            "extern" => RustTokenType::Extern,
            "dyn" => RustTokenType::Dyn,
            "box" => RustTokenType::Box,
            "Some" => RustTokenType::Some,
            "None" => RustTokenType::None,
            "Ok" => RustTokenType::Ok,
            "Err" => RustTokenType::Err,
            "true" => RustTokenType::True,
            "false" => RustTokenType::False,
            _ => {
                // Lifetime or identifier
                if value.starts_with('\'') && value.len() > 1 {
                    RustTokenType::Lifetime(value)
                } else {
                    RustTokenType::Ident(value)
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<RustToken> {
        while self.pos < self.source.len() {
            self.start = self.pos;
            self.skip_whitespace();

            if self.pos >= self.source.len() {
                break;
            }

            let ch = self.advance().unwrap();

            let token = match ch {
                '{' => RustTokenType::LBrace,
                '}' => RustTokenType::RBrace,
                '(' => RustTokenType::LParen,
                ')' => RustTokenType::RParen,
                '[' => RustTokenType::LBracket,
                ']' => RustTokenType::RBracket,
                ';' => RustTokenType::Semi,
                ',' => RustTokenType::Comma,
                '.' => {
                    match self.current_char() {
                        Some('.') => {
                            self.advance();
                            match self.current_char() {
                                Some('=') => { self.advance(); RustTokenType::DotDotEq }
                                Some('.') => { self.advance(); RustTokenType::DotDotDot }
                                _ => RustTokenType::DotDot
                            }
                        }
                        _ => RustTokenType::Dot,
                    }
                }
                '-' => {
                    match self.current_char() {
                        Some('>') => { self.advance(); RustTokenType::Arrow }
                        Some('=') => { self.advance(); RustTokenType::MinusEq }
                        _ => RustTokenType::Minus,
                    }
                }
                '=' => {
                    match self.current_char() {
                        Some('>') => { self.advance(); RustTokenType::FatArrow }
                        Some('=') => { self.advance(); RustTokenType::EqEq }
                        _ => RustTokenType::Eq,
                    }
                }
                '!' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::BangEq }
                        _ => RustTokenType::Bang,
                    }
                }
                '<' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::LtEq }
                        Some('<') => {
                            self.advance();
                            if self.current_char() == Some('=') {
                                self.advance();
                                RustTokenType::LtEq;
                            }
                            RustTokenType::Lt
                        }
                        _ => RustTokenType::Lt,
                    }
                }
                '>' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::GtEq }
                        Some('>') => {
                            self.advance();
                            if self.current_char() == Some('=') {
                                self.advance();
                            }
                            RustTokenType::Gt
                        }
                        _ => RustTokenType::Gt,
                    }
                }
                '&' => {
                    match self.current_char() {
                        Some('&') => { self.advance(); RustTokenType::AmpAmp }
                        Some('=') => { self.advance(); RustTokenType::AmpEq }
                        _ => RustTokenType::Amp,
                    }
                }
                '|' => {
                    match self.current_char() {
                        Some('|') => { self.advance(); RustTokenType::BarBar }
                        Some('=') => { self.advance(); RustTokenType::BarEq }
                        _ => RustTokenType::Bar,
                    }
                }
                '+' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::PlusEq }
                        _ => RustTokenType::Plus,
                    }
                }
                '*' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::StarEq }
                        _ => RustTokenType::Star,
                    }
                }
                '/' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::SlashEq }
                        _ => RustTokenType::Slash,
                    }
                }
                '%' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::PercentEq }
                        _ => RustTokenType::Percent,
                    }
                }
                '^' => {
                    match self.current_char() {
                        Some('=') => { self.advance(); RustTokenType::CaretEq }
                        _ => RustTokenType::Caret,
                    }
                }
                ':' => {
                    match self.current_char() {
                        Some(':') => { self.advance(); RustTokenType::DoubleColon }
                        _ => RustTokenType::Colon,
                    }
                }
                '#' => RustTokenType::Pound,
                '@' => RustTokenType::At,
                '~' => RustTokenType::Tilde,
                '?' => RustTokenType::Question,
                '$' => RustTokenType::Dollar,
                '\'' => {
                    // Character literal
                    let mut ch_val = char::REPLACEMENT_CHARACTER;
                    if self.current_char().is_some() && self.current_char() != Some('\'') {
                        ch_val = self.advance().unwrap();
                        if ch_val == '\\' {
                            // Handle escape
                            match self.advance() {
                                Some('n') => ch_val = '\n',
                                Some('r') => ch_val = '\r',
                                Some('t') => ch_val = '\t',
                                Some('\\') => ch_val = '\\',
                                Some('\'') => ch_val = '\'',
                                Some('0') => ch_val = '\0',
                                _ => {}
                            }
                        }
                    }
                    self.advance(); // Closing quote
                    RustTokenType::Char(ch_val)
                }
                '"' => {
                    let hashes = 0;
                    let value = self.read_string('"', false, hashes);
                    RustTokenType::String(value)
                }
                'b' => {
                    if self.current_char() == Some('"') {
                        self.advance();
                        let value = self.read_string('"', false, 0);
                        RustTokenType::ByteString(value.into_bytes())
                    } else {
                        self.read_identifier('b')
                    }
                }
                'r' => {
                    if self.current_char() == Some('"') {
                        self.advance();
                        let mut hashes = 0;
                        while self.current_char() == Some('#') {
                            self.advance();
                            hashes += 1;
                        }
                        let value = self.read_string('"', true, hashes);
                        RustTokenType::RawString(value, hashes)
                    } else {
                        self.read_identifier('r')
                    }
                }
                '0'..='9' => self.read_number(ch),
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(ch),
                _ => RustTokenType::Unknown,
            };

            self.add_token(token);
        }

        self.add_token(RustTokenType::Eof);
        self.tokens.clone()
    }
}

// ============================================================================
// AST Nodes
// ============================================================================

#[derive(Debug, Clone)]
pub enum RustVisibility {
    Public,
    PubCrate,
    PubSuper,
    PubIn(Path),
    Private,
}

#[derive(Debug, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub is_absolute: bool,
}

#[derive(Debug, Clone)]
pub struct PathSegment {
    pub ident: String,
    pub generic_args: Option<Vec<GenericArg>>,
    pub turbofish: Option<Vec<GenericArg>>,
}

#[derive(Debug, Clone)]
pub enum GenericArg {
    Type(Type),
    Lifetime(String),
    Const(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Type {
    Never,
    Unit,
    Bool,
    Char,
    Int(i32),
    Uint(u32),
    Float(u32),
    Str,
    Bytes,
    SelfType,
    Path(Path),
    Pointer(Box<PointerType>),
    Reference(Box<ReferenceType>),
    Array(Box<ArrayType>),
    Slice(Box<SliceType>),
    Tuple(Vec<Type>),
    Function(Box<FunctionType>),
    TraitObject(Vec<Path>, Option<String>),  // bounds, dyn lifetime
    ImplTrait(Vec<Path>),
    DynTrait(Box<DynTraitType>),
    Infer,
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct PointerType {
    pub mutable: bool,
    pub inner: Type,
}

#[derive(Debug, Clone)]
pub struct ReferenceType {
    pub mutable: bool,
    pub inner: Type,
    pub lifetime: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub elem: Type,
    pub len: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct SliceType {
    pub elem: Type,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret: Box<Type>,
    pub safety: Safety,
    pub abi: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DynTraitType {
    pub bounds: Vec<Type>,
    pub lifetime: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Safety {
    Safe,
    Unsafe,
    Default,
}

#[derive(Debug, Clone)]
pub enum Mutability {
    Mut,
    Not,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Option<Type>,
    pub span: (usize, usize),
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Unit,
    Bool(bool),
    Char(char),
    Int(i64),
    Float(f64),
    String(String),
    Byte(u8),
    ByteString(Vec<u8>),
    Ident(String),
    Path(Path),
    SelfValue,
    SelfDeref,
    SelfAddr,
    Literal(String, Option<String>),  // value, suffix

    // Operators
    Unary(Operator, Box<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
    Logical(LogicalOp, Box<Expr>, Box<Expr>),
    Cast(Box<Expr>, Type),
    TypeCast(Box<Expr>, Type),
    AsmCast(Box<Expr>, Type, String),  // expr, ty, arch

    // Control flow
    Block(Vec<Stmt>, Option<Box<Expr>>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    IfLet(Pat, Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Match(Box<Expr>, Vec<MatchArm>),
    Loop(Box<Expr>),
    While(Box<Expr>, Box<Expr>),
    WhileLet(Pat, Box<Expr>, Box<Expr>),
    For(Pat, Box<Expr>, Box<Expr>),
    Continue(Option<String>),
    Break(Option<Box<Expr>>, Option<String>),
    Return(Option<Box<Expr>>),

    // Functions
    Call(Box<Expr>, Vec<Expr>),
    MethodCall(String, Vec<Expr>, Vec<AngleBracketedArg>),
    Closure(Vec<ClosureArg>, Box<Expr>, Option<Box<Type>>),
    Async(Vec<ClosureArg>, Box<Expr>),
    Await(Box<Expr>),
    Try(Block<Expr>),
    TryBlock(Vec<Stmt>),

    // Objects
    Tuple(Vec<Expr>),
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Range(Option<Box<Expr>>, Option<Box<Expr>>, bool),  // start, end, inclusive
    Field(Box<Expr>, String),
    TupleField(Box<Expr>, usize),
    Struct(Path, Vec<FieldValue>),
    Enum(Path, String, Vec<Expr>),

    // Patterns
    Path(Path),
    Wildcard,
    Rest,
    Bind(String, Box<Pat>),
    LitPat(Literal),
    OrPat(Box<Pat>, Box<Pat>),
    RangePat(Literal, Literal),
    SlicePat(Vec<Pat>),
    TuplePat(Vec<Pat>),
    StructPat(Path, Vec<FieldPat>),

    // Macros
    Macro(MacroInvoke),
    MacroRules(String, Vec<MacroRule>),

    // Error propagation
    QuestionMark(Box<Expr>),

    // Inline assembly
    Asm(String, Vec<AsmOperand>, AsmOptions),

    // Types
    TypeAlias(Type),
    ExprBlock(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr,
    Neg, Not,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And, Or,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64, Option<String>),
    Float(f64, Option<String>),
    Char(char),
    String(String),
    Byte(u8),
    ByteString(Vec<u8>),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pat: Pat,
    pub guard: Option<Box<Expr>>,
    pub body: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum Pat {
    Wild,
    Bind(String, Mutability, Option<Pat>),
    Literal(Literal),
    Range(Box<Pat>, Box<Pat>),
    Enum(Path, Option<Box<Pat>>),
    TupleStruct(Path, Vec<Pat>),
    Struct(Path, Vec<FieldPat>, bool),  // path, fields, dotdot
    Tuple(Vec<Pat>),
    Slice(Vec<Pat>),
    Or(Vec<Pat>),
    RangeLit(Literal, Literal),
    Rest,
}

#[derive(Debug, Clone)]
pub struct FieldPat {
    pub name: String,
    pub pat: Pat,
    pub shorthand: bool,
}

#[derive(Debug, Clone)]
pub struct FieldValue {
    pub name: String,
    pub value: Expr,
    pub shorthand: bool,
}

#[derive(Debug, Clone)]
pub struct ClosureArg {
    pub pat: Pat,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone)]
pub struct AngleBracketedArg {
    pub ident: String,
    pub arg: GenericArg,
}

#[derive(Debug, Clone)]
pub struct MacroInvoke {
    pub path: Path,
    pub tokens: Vec<TokenTree>,
    pub delimiter: MacroDelimiter,
}

#[derive(Debug, Clone)]
pub enum MacroDelimiter {
    Paren, Brace, Bracket,
}

#[derive(Debug, Clone)]
pub enum TokenTree {
    Token(RustToken),
    Delim(MacroDelimiter, Vec<TokenTree>),
    Subtree(String, Vec<TokenTree>),
}

#[derive(Debug, Clone)]
pub struct MacroRule {
    pub matcher: Vec<TokenTree>,
    pub body: Vec<TokenTree>,
}

#[derive(Debug, Clone)]
pub struct AsmOperand {
    pub sym: Option<String>,
    pub expr: Expr,
    pub constraint: String,
}

#[derive(Debug, Clone)]
pub struct AsmOptions {
    pub pure: bool,
    pub nomem: bool,
    pub readonly: bool,
    pub preserves_flags: bool,
    pub volatile: bool,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(Pat, Option<Type>, Option<Expr>, Option<Box<Expr>>),  // pattern, type, init, body
    Item(Item),
    Expr(Expr),
    Semi(Expr),
}

#[derive(Debug, Clone)]
pub struct Item {
    pub vis: RustVisibility,
    pub attrs: Vec<Attribute>,
    pub kind: ItemKind,
}

#[derive(Debug, Clone)]
pub enum ItemKind {
    Import(UseTree),
    Module(String),
    ExternCrate(String, Option<String>),  // name, alias
    Global(String, Type, Expr),
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Union(Enum),  // Same structure
    Trait(Trait),
    Impl(Impl),
    TypeAlias(String, Vec<GenericParam>, Type, Option<WhereClause>),
    ForeignMod(String, Vec<ForeignItem>),  // abi, items
    MacroRules(String, Vec<MacroDef>),
}

#[derive(Debug, Clone)]
pub struct UseTree {
    pub path: Option<Path>,
    pub prefix: Option<Path>,
    pub tree: UseTreeKind,
    pub is_glob: bool,
    pub is_nested: bool,
}

#[derive(Debug, Clone)]
pub enum UseTreeKind {
    Simple(String, Option<String>),  // ident, alias
    Tree(Vec<UseTree>),
    Rebind(String, String),  // from, to
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub params: Vec<FnParam>,
    pub ret: Option<Type>,
    pub body: Option<Expr>,
    pub safety: Safety,
    pub abi: Option<String>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone)]
pub struct FnParam {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
    pub by_ref: bool,
    pub spread: bool,
    pub self_param: bool,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<StructField>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<Variant>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub data: VariantData,
    pub discriminant: Option<Expr>,
}

#[derive(Debug, Clone)]
pub enum VariantData {
    Unit,
    Tuple(Vec<(Type, bool)>),  // type, mutable
    Struct(Vec<StructField>),
}

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub items: Vec<TraitItem>,
    pub super_traits: Vec<Type>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone)]
pub enum TraitItem {
    Function(Function),
    Method(Function),
    Const(String, Type),
    Type(String, Vec<GenericParam>, Option<Type>),
}

#[derive(Debug, Clone)]
pub struct Impl {
    pub generics: Vec<GenericParam>,
    pub trait_ty: Option<(Path, Type)>,  // trait, for type
    pub self_ty: Type,
    pub items: Vec<ImplItem>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone)]
pub enum ImplItem {
    Function(Function),
    Const(String, Type, Expr),
    Type(String, Type),
    Static(String, Type, Expr),
}

#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: String,
    pub kind: GenericParamKind,
}

#[derive(Debug, Clone)]
pub enum GenericParamKind {
    Lifetime,
    Type {
        default: Option<Type>,
        bounds: Vec<Type>,
    },
    Const {
        ty: Type,
        default: Option<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct WhereClause {
    pub predicates: Vec<WherePredicate>,
}

#[derive(Debug, Clone)]
pub enum WherePredicate {
    Bound(WhereBoundPredicate),
    Eq(WhereEqPredicate),
}

#[derive(Debug, Clone)]
pub struct WhereBoundPredicate {
    pub bound_lifetimes: Vec<String>,
    pub ty: Type,
    pub bounds: Vec<Type>,
}

#[derive(Debug, Clone)]
pub struct WhereEqPredicate {
    pub ty: Type,
    pub bound: Type,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub path: Path,
    pub tokens: Vec<TokenTree>,
    pub style: AttrStyle,
}

#[derive(Debug, Clone)]
pub enum AttrStyle {
    Doc(bool),  // is_sugared_doc
    Normal,
}

#[derive(Debug, Clone)]
pub struct ForeignItem {
    pub name: String,
    pub ty: Type,
    pub attrs: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct MacroDef {
    pub name: String,
    pub body: Vec<TokenTree>,
    pub is_builtin: bool,
}

// ============================================================================
// Parser
// ============================================================================

pub struct RustParser {
    tokens: Vec<RustToken>,
    pos: usize,
    errors: Vec<ParseError>,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: (usize, usize),
}

impl RustParser {
    pub fn new(tokens: Vec<RustToken>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
        }
    }

    fn current(&self) -> &RustToken {
        self.tokens.get(self.pos).unwrap_or(&RustToken {
            token_type: RustTokenType::Eof,
            lexeme: String::new(),
            line: 0,
            column: 0,
            span: (0, 0),
        })
    }

    fn peek(&self, offset: usize) -> &RustToken {
        self.tokens.get(self.pos + offset).unwrap_or(&RustToken {
            token_type: RustTokenType::Eof,
            lexeme: String::new(),
            line: 0,
            column: 0,
            span: (0, 0),
        })
    }

    fn advance(&mut self) -> RustToken {
        let token = self.current().clone();
        if self.current().token_type != RustTokenType::Eof {
            self.pos += 1;
        }
        token
    }

    fn check(&self, token_type: &RustTokenType) -> bool {
        match (&self.current().token_type, token_type) {
            (RustTokenType::Ident(_), RustTokenType::Ident(_)) => true,
            (RustTokenType::Integer(_, _), RustTokenType::Integer(_, _)) => true,
            (RustTokenType::Float(_, _), RustTokenType::Float(_, _)) => true,
            (a, b) => a == b,
        }
    }

    fn match_token(&mut self, token_types: &[RustTokenType]) -> bool {
        for tt in token_types {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expect(&mut self, token_type: RustTokenType, message: &str) -> Result<RustToken, ParseError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            let err = ParseError {
                message: format!("{}: got {:?}", message, self.current()),
                span: self.current().span,
            };
            self.errors.push(err.clone());
            Err(err)
        }
    }

    fn skip_until(&mut self, token_types: &[RustTokenType]) {
        while !self.check(&RustTokenType::Eof) {
            for tt in token_types {
                if self.check(tt) {
                    return;
                }
            }
            self.advance();
        }
    }

    // Parse item
    pub fn parse_item(&mut self) -> Option<Item> {
        let start_span = self.current().span;

        // Visibility
        let vis = self.parse_visibility();

        // Attributes
        let mut attrs = Vec::new();
        while self.check(&RustTokenType::Pound) {
            attrs.push(self.parse_attribute().unwrap());
        }

        // Check for keywords
        let token = self.current().clone();
        let kind = match &token.token_type {
            RustTokenType::Use => {
                self.advance();
                ItemKind::Import(self.parse_use_tree().unwrap_or(UseTree {
                    path: None,
                    prefix: None,
                    tree: UseTreeKind::Simple(String::new(), None),
                    is_glob: false,
                    is_nested: false,
                }))
            }
            RustTokenType::Mod => {
                self.advance();
                let name = self.expect(RustTokenType::Ident(String::new()), "module name")?;
                ItemKind::Module(name.lexeme)
            }
            RustTokenType::Extern => {
                self.advance();
                let abi = if self.check(&RustTokenType::String(String::new())) {
                    if let RustTokenType::String(s) = self.advance().token_type {
                        s
                    } else {
                        String::new()
                    }
                } else {
                    "C".to_string()
                };
                let items = self.parse_extern_items();
                ItemKind::ForeignMod(abi, items)
            }
            RustTokenType::Static | RustTokenType::Const => {
                let is_static = self.advance().token_type == RustTokenType::Static;
                let name = self.expect(RustTokenType::Ident(String::new()), "name")?.lexeme;
                let ty = if self.match_token(&[RustTokenType::Colon]) {
                    self.parse_type().unwrap_or(Type::Infer)
                } else {
                    Type::Infer
                };
                let value = if self.match_token(&[RustTokenType::Eq]) {
                    Some(self.parse_expr().unwrap_or(Expr {
                        kind: ExprKind::Unit,
                        ty: None,
                        span: (0, 0),
                    }))
                } else {
                    None
                };
                self.match_token(&[RustTokenType::Semi]);
                ItemKind::Global(name, ty, value.unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }))
            }
            RustTokenType::Fn => {
                self.advance();
                ItemKind::Function(self.parse_function().unwrap_or(Function {
                    name: String::new(),
                    generics: Vec::new(),
                    params: Vec::new(),
                    ret: None,
                    body: None,
                    safety: Safety::Default,
                    abi: None,
                    where_clause: None,
                }))
            }
            RustTokenType::Struct => {
                self.advance();
                ItemKind::Struct(self.parse_struct().unwrap_or(Struct {
                    name: String::new(),
                    generics: Vec::new(),
                    fields: Vec::new(),
                    where_clause: None,
                }))
            }
            RustTokenType::Enum => {
                self.advance();
                ItemKind::Enum(self.parse_enum().unwrap_or(Enum {
                    name: String::new(),
                    generics: Vec::new(),
                    variants: Vec::new(),
                    where_clause: None,
                }))
            }
            RustTokenType::Trait => {
                self.advance();
                ItemKind::Trait(self.parse_trait().unwrap_or(Trait {
                    name: String::new(),
                    generics: Vec::new(),
                    items: Vec::new(),
                    super_traits: Vec::new(),
                    where_clause: None,
                }))
            }
            RustTokenType::Impl => {
                self.advance();
                ItemKind::Impl(self.parse_impl().unwrap_or(Impl {
                    generics: Vec::new(),
                    trait_ty: None,
                    self_ty: Type::Infer,
                    items: Vec::new(),
                    where_clause: None,
                }))
            }
            RustTokenType::Type => {
                self.advance();
                let name = self.expect(RustTokenType::Ident(String::new()), "type name")?.lexeme;
                let generics = self.parse_generics();
                self.match_token(&[RustTokenType::Eq]);
                let ty = self.parse_type().unwrap_or(Type::Infer);
                self.match_token(&[RustTokenType::Semi]);
                ItemKind::TypeAlias(name, generics, ty, None)
            }
            _ => {
                // Try to parse as macro_rules!
                if let RustTokenType::Ident(name) = &self.current().token_type {
                    if name == "macro_rules" {
                        self.advance();
                        let name = self.expect(RustTokenType::Ident(String::new()), "macro name")?.lexeme;
                        ItemKind::MacroRules(name, self.parse_macro_rules())
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        };

        Some(Item { vis, attrs, kind })
    }

    fn parse_visibility(&mut self) -> RustVisibility {
        if self.match_token(&[RustTokenType::Pub]) {
            if self.match_token(&[RustTokenType::LParen]) {
                let vis = if self.match_token(&[RustTokenType::Crate]) {
                    RustVisibility::PubCrate
                } else if self.match_token(&[RustTokenType::Super]) {
                    RustVisibility::PubSuper
                } else if self.match_token(&[RustTokenType::In]) {
                    RustVisibility::PubIn(Path { segments: vec![], is_absolute: false })
                } else {
                    RustVisibility::Public
                };
                self.match_token(&[RustTokenType::RParen]);
                vis
            } else {
                RustVisibility::Public
            }
        } else {
            RustVisibility::Private
        }
    }

    fn parse_attribute(&mut self) -> Option<Attribute> {
        self.match_token(&[RustTokenType::Pound])?;
        let style = if self.match_token(&[RustTokenType::Bang]) {
            AttrStyle::Normal
        } else {
            AttrStyle::Doc(false)
        };

        let content = if self.match_token(&[RustTokenType::LBracket]) {
            let path = self.parse_path().unwrap_or(Path { segments: vec![], is_absolute: false });
            let mut tokens = Vec::new();
            // Simple: just skip to ]
            while !self.check(&RustTokenType::RBracket) && !self.check(&RustTokenType::Eof) {
                tokens.push(TokenTree::Token(self.advance()));
            }
            self.match_token(&[RustTokenType::RBracket]);
            Attribute { path, tokens, style }
        } else {
            return None;
        };
        Some(content)
    }

    fn parse_path(&mut self) -> Option<Path> {
        let mut segments = Vec::new();
        let is_absolute = self.match_token(&[RustTokenType::DoubleColon]);
        if self.match_token(&[RustTokenType::Crate]) || self.match_token(&[RustTokenType::Super]) {
            if let RustTokenType::Ident(id) = &self.current().token_type.clone() {
                segments.push(PathSegment { ident: id.clone(), generic_args: None, turbofish: None });
                self.advance();
            }
        }
        while let RustTokenType::Ident(id) = self.current().token_type.clone() {
            self.advance();
            segments.push(PathSegment { ident: id.clone(), generic_args: None, turbofish: None });
            if !self.match_token(&[RustTokenType::DoubleColon]) {
                break;
            }
        }
        Some(Path { segments, is_absolute })
    }

    fn parse_use_tree(&mut self) -> Option<UseTree> {
        let path = self.parse_path();
        let tree = if self.match_token(&[RustTokenType::LBrace]) {
            let mut trees = Vec::new();
            while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                if let Some(tree) = self.parse_use_tree() {
                    trees.push(tree);
                }
                self.match_token(&[RustTokenType::Comma]);
            }
            self.match_token(&[RustTokenType::RBrace]);
            UseTreeKind::Tree(trees)
        } else {
            UseTreeKind::Simple(
                path.as_ref().and_then(|p| p.segments.last().map(|s| s.ident.clone())).unwrap_or_default(),
                None
            )
        };
        let is_glob = self.match_token(&[RustTokenType::Star]);
        Some(UseTree {
            path,
            prefix: None,
            tree,
            is_glob,
            is_nested: false,
        })
    }

    fn parse_extern_items(&mut self) -> Vec<ForeignItem> {
        let mut items = Vec::new();
        if self.match_token(&[RustTokenType::LBrace]) {
            while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                let name = if let RustTokenType::Ident(id) = &self.current().token_type {
                    let n = id.clone();
                    self.advance();
                    n
                } else {
                    self.advance();
                    continue;
                };
                self.match_token(&[RustTokenType::Colon]);
                let ty = self.parse_type().unwrap_or(Type::Infer);
                self.match_token(&[RustTokenType::Semi]);
                items.push(ForeignItem {
                    name,
                    ty,
                    attrs: Vec::new(),
                });
            }
            self.match_token(&[RustTokenType::RBrace]);
        }
        items
    }

    fn parse_function(&mut self) -> Option<Function> {
        let name = if let RustTokenType::Ident(id) = &self.current().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return None;
        };

        let generics = self.parse_generics();
        self.match_token(&[RustTokenType::LParen]);

        let mut params = Vec::new();
        while !self.check(&RustTokenType::RParen) && !self.check(&RustTokenType::Eof) {
            let name = if let RustTokenType::Ident(id) = &self.current().token_type {
                let n = id.clone();
                self.advance();
                n
            } else {
                String::new()
            };
            self.match_token(&[RustTokenType::Colon]);
            let ty = self.parse_type().unwrap_or(Type::Infer);
            params.push(FnParam {
                name,
                ty,
                mutable: false,
                by_ref: false,
                spread: false,
                self_param: false,
            });
            if !self.match_token(&[RustTokenType::Comma]) {
                break;
            }
        }
        self.match_token(&[RustTokenType::RParen]);

        let ret = if self.match_token(&[RustTokenType::Arrow]) {
            Some(self.parse_type().unwrap_or(Type::Unit))
        } else {
            None
        };

        let body = if self.match_token(&[RustTokenType::LBrace]) {
            Some(self.parse_block_expr().unwrap_or(Expr {
                kind: ExprKind::Unit,
                ty: None,
                span: (0, 0),
            }))
        } else {
            self.match_token(&[RustTokenType::Semi]);
            None
        };

        Some(Function {
            name,
            generics,
            params,
            ret,
            body,
            safety: Safety::Default,
            abi: None,
            where_clause: None,
        })
    }

    fn parse_generics(&mut self) -> Vec<GenericParam> {
        let mut params = Vec::new();
        if self.match_token(&[RustTokenType::Lt]) {
            while !self.check(&RustTokenType::Gt) && !self.check(&RustTokenType::Eof) {
                if self.match_token(&[RustTokenType::Lifetime(String::new())]) {
                    params.push(GenericParam {
                        name: String::new(),
                        kind: GenericParamKind::Lifetime,
                    });
                } else if let RustTokenType::Ident(id) = &self.current().token_type {
                    let name = id.clone();
                    self.advance();
                    if self.match_token(&[RustTokenType::Colon]) {
                        let mut bounds = Vec::new();
                        while !self.check(&RustTokenType::Gt) && !self.check(&RustTokenType::Comma) {
                            bounds.push(self.parse_type().unwrap_or(Type::Infer));
                            if !self.match_token(&[RustTokenType::Plus]) {
                                break;
                            }
                        }
                        params.push(GenericParam {
                            name,
                            kind: GenericParamKind::Type { default: None, bounds },
                        });
                    } else {
                        params.push(GenericParam {
                            name,
                            kind: GenericParamKind::Type { default: None, bounds: Vec::new() },
                        });
                    }
                } else {
                    break;
                }
                if !self.match_token(&[RustTokenType::Comma]) {
                    break;
                }
            }
            self.match_token(&[RustTokenType::Gt]);
        }
        params
    }

    fn parse_type(&mut self) -> Option<Type> {
        let ty = match &self.current().token_type {
            RustTokenType::Never => {
                self.advance();
                Type::Never
            }
            RustTokenType::Bool => {
                self.advance();
                Type::Bool
            }
            RustTokenType::Char => {
                self.advance();
                Type::Char
            }
            RustTokenType::Str => {
                self.advance();
                Type::Str
            }
            RustTokenType::SelfType => {
                self.advance();
                Type::SelfType
            }
            RustTokenType::Star => {
                self.advance();
                let mutable = self.match_token(&[RustTokenType::Mut]);
                let inner = self.parse_type().unwrap_or(Type::Infer);
                Type::Pointer(Box::new(PointerType { mutable, inner }))
            }
            RustTokenType::Amp => {
                self.advance();
                let lifetime = if let RustTokenType::Lifetime(l) = &self.current().token_type {
                    let lt = l.clone();
                    self.advance();
                    Some(lt)
                } else {
                    None
                };
                let mutable = self.match_token(&[RustTokenType::Mut]);
                let inner = self.parse_type().unwrap_or(Type::Infer);
                Type::Reference(Box::new(ReferenceType { mutable, inner, lifetime }))
            }
            RustTokenType::LBracket => {
                self.advance();
                let elem = self.parse_type().unwrap_or(Type::Infer);
                let len = if self.match_token(&[RustTokenType::Semi]) {
                    self.parse_expr().map(Box::new)
                } else {
                    None
                };
                self.match_token(&[RustTokenType::RBracket]);
                Type::Array(Box::new(ArrayType { elem, len }))
            }
            RustTokenType::LParen => {
                self.advance();
                let mut types = Vec::new();
                while !self.check(&RustTokenType::RParen) && !self.check(&RustTokenType::Eof) {
                    types.push(self.parse_type().unwrap_or(Type::Infer));
                    if !self.match_token(&[RustTokenType::Comma]) {
                        break;
                    }
                }
                self.match_token(&[RustTokenType::RParen]);
                if types.len() == 1 {
                    types.remove(0)
                } else {
                    Type::Tuple(types)
                }
            }
            RustTokenType::Fn => {
                self.advance();
                self.match_token(&[RustTokenType::LParen]);
                let mut params = Vec::new();
                while !self.check(&RustTokenType::RParen) && !self.check(&RustTokenType::Eof) {
                    params.push(self.parse_type().unwrap_or(Type::Infer));
                    if !self.match_token(&[RustTokenType::Comma]) {
                        break;
                    }
                }
                self.match_token(&[RustTokenType::RParen]);
                let ret = if self.match_token(&[RustTokenType::Arrow]) {
                    Box::new(self.parse_type().unwrap_or(Type::Unit))
                } else {
                    Box::new(Type::Unit)
                };
                Type::Function(Box::new(FunctionType {
                    params,
                    ret,
                    safety: Safety::Default,
                    abi: None,
                }))
            }
            RustTokenType::Ident(_) => {
                let path = self.parse_path().unwrap_or(Path { segments: vec![], is_absolute: false });
                Type::Path(path)
            }
            _ => Type::Infer,
        };
        Some(ty)
    }

    fn parse_block_expr(&mut self) -> Option<Expr> {
        let mut stmts = Vec::new();
        while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            } else {
                break;
            }
        }
        self.match_token(&[RustTokenType::RBrace]);

        let value = if stmts.len() == 1 {
            if let Stmt::Expr(e) = &stmts[0] {
                Some(Box::new(e.clone()))
            } else {
                None
            }
        } else {
            None
        };

        Some(Expr {
            kind: ExprKind::Block(stmts, value),
            ty: None,
            span: (0, 0),
        })
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        // Let statement
        if self.match_token(&[RustTokenType::Let]) {
            let pat = self.parse_pattern().unwrap_or(Pat::Wild);
            let ty = if self.match_token(&[RustTokenType::Colon]) {
                Some(self.parse_type().unwrap_or(Type::Infer))
            } else {
                None
            };
            let init = if self.match_token(&[RustTokenType::Eq]) {
                Some(Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                })))
            } else {
                None
            };
            self.match_token(&[RustTokenType::Semi]);
            return Some(Stmt::Let(pat, ty, None, init));
        }

        // Item
        if let Some(item) = self.parse_item() {
            return Some(Stmt::Item(item));
        }

        // Expression
        let expr = self.parse_expr()?;
        if self.match_token(&[RustTokenType::Semi]) {
            Some(Stmt::Semi(expr))
        } else {
            Some(Stmt::Expr(expr))
        }
    }

    fn parse_pattern(&mut self) -> Option<Pat> {
        let pat = match &self.current().token_type {
            RustTokenType::Underscore => {
                self.advance();
                Pat::Wild
            }
            RustTokenType::DotDot => {
                self.advance();
                Pat::Rest
            }
            RustTokenType::Ident(_) => {
                let name = if let RustTokenType::Ident(id) = &self.current().token_type {
                    id.clone()
                } else {
                    String::new()
                };
                self.advance();
                Pat::Bind(name, Mutability::Not, None)
            }
            RustTokenType::LParen => {
                self.advance();
                let mut pats = Vec::new();
                while !self.check(&RustTokenType::RParen) && !self.check(&RustTokenType::Eof) {
                    pats.push(self.parse_pattern().unwrap_or(Pat::Wild));
                    if !self.match_token(&[RustTokenType::Comma]) {
                        break;
                    }
                }
                self.match_token(&[RustTokenType::RParen]);
                Pat::Tuple(pats)
            }
            RustTokenType::LBracket => {
                self.advance();
                let mut pats = Vec::new();
                while !self.check(&RustTokenType::RBracket) && !self.check(&RustTokenType::Eof) {
                    pats.push(self.parse_pattern().unwrap_or(Pat::Wild));
                    if !self.match_token(&[RustTokenType::Comma]) {
                        break;
                    }
                }
                self.match_token(&[RustTokenType::RBracket]);
                Pat::Slice(pats)
            }
            _ => Pat::Wild,
        };
        Some(pat)
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_expr_with_precedence(0)
    }

    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Option<Expr> {
        let mut lhs = self.parse_prefix_expr()?;

        loop {
            let prec = self.get_binop_prec(&self.current().token_type);
            if prec < min_prec {
                break;
            }

            let op = self.advance();
            let rhs = self.parse_expr_with_precedence(prec + 1)?;

            lhs = Expr {
                kind: ExprKind::Binary(
                    Operator::Add,  // Simplified
                    Box::new(lhs),
                    Box::new(rhs),
                ),
                ty: None,
                span: (0, 0),
            };
        }

        Some(lhs)
    }

    fn get_binop_prec(&self, _token_type: &RustTokenType) -> u8 {
        // Simplified precedence
        10
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let token = self.current().clone();
        let expr = match &token.token_type {
            RustTokenType::Integer(v, s) => {
                self.advance();
                ExprKind::Int(*v)
            }
            RustTokenType::Float(v, s) => {
                self.advance();
                ExprKind::Float(*v)
            }
            RustTokenType::String(s) => {
                self.advance();
                ExprKind::String(s.clone())
            }
            RustTokenType::True => {
                self.advance();
                ExprKind::Bool(true)
            }
            RustTokenType::False => {
                self.advance();
                ExprKind::Bool(false)
            }
            RustTokenType::Ident(id) => {
                self.advance();
                ExprKind::Ident(id.clone())
            }
            RustTokenType::LParen => {
                self.advance();
                let expr = self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                });
                self.match_token(&[RustTokenType::RParen]);
                ExprKind::Tuple(vec![expr])
            }
            RustTokenType::LBracket => {
                self.advance();
                let mut items = Vec::new();
                while !self.check(&RustTokenType::RBracket) && !self.check(&RustTokenType::Eof) {
                    items.push(self.parse_expr().unwrap_or(Expr {
                        kind: ExprKind::Unit,
                        ty: None,
                        span: (0, 0),
                    }));
                    if !self.match_token(&[RustTokenType::Comma]) {
                        break;
                    }
                }
                self.match_token(&[RustTokenType::RBracket]);
                ExprKind::Array(items)
            }
            RustTokenType::LBrace => {
                self.advance();
                return self.parse_block_expr();
            }
            RustTokenType::If => {
                self.advance();
                let cond = self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                });
                let then = self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                });
                let else_part = if self.match_token(&[RustTokenType::Else]) {
                    Some(Box::new(self.parse_expr().unwrap_or(Expr {
                        kind: ExprKind::Unit,
                        ty: None,
                        span: (0, 0),
                    })))
                } else {
                    None
                };
                ExprKind::If(Box::new(cond), Box::new(then), else_part)
            }
            RustTokenType::Match => {
                self.advance();
                let expr = self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                });
                self.match_token(&[RustTokenType::LBrace]);
                let mut arms = Vec::new();
                while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                    let pat = self.parse_pattern().unwrap_or(Pat::Wild);
                    let guard = if self.match_token(&[RustTokenType::If]) {
                        Some(Box::new(self.parse_expr().unwrap_or(Expr {
                            kind: ExprKind::Unit,
                            ty: None,
                            span: (0, 0),
                        })))
                    } else {
                        None
                    };
                    self.match_token(&[RustTokenType::FatArrow]);
                    let body = Box::new(self.parse_expr().unwrap_or(Expr {
                        kind: ExprKind::Unit,
                        ty: None,
                        span: (0, 0),
                    }));
                    self.match_token(&[RustTokenType::Comma]);
                    arms.push(MatchArm { pat, guard, body });
                }
                self.match_token(&[RustTokenType::RBrace]);
                ExprKind::Match(Box::new(expr), arms)
            }
            RustTokenType::Loop => {
                self.advance();
                let body = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::Loop(body)
            }
            RustTokenType::While => {
                self.advance();
                let cond = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                let body = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::While(cond, body)
            }
            RustTokenType::For => {
                self.advance();
                let pat = self.parse_pattern().unwrap_or(Pat::Wild);
                self.match_token(&[RustTokenType::In]);
                let iter = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                let body = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::For(pat, iter, body)
            }
            RustTokenType::Return => {
                self.advance();
                let value = if self.check(&RustTokenType::Semi) || self.check(&RustTokenType::RBrace) {
                    None
                } else {
                    Some(Box::new(self.parse_expr().unwrap_or(Expr {
                        kind: ExprKind::Unit,
                        ty: None,
                        span: (0, 0),
                    })))
                };
                ExprKind::Return(value)
            }
            RustTokenType::Break => {
                self.advance();
                ExprKind::Break(None, None)
            }
            RustTokenType::Continue => {
                self.advance();
                ExprKind::Continue(None)
            }
            RustTokenType::Async => {
                self.advance();
                let _ = self.match_token(&[RustTokenType::Move]);
                let body = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::Async(vec![], body)
            }
            RustTokenType::Move => {
                self.advance();
                let body = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::Closure(vec![], body, None)
            }
            RustTokenType::Minus => {
                self.advance();
                let operand = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::Unary(Operator::Neg, operand)
            }
            RustTokenType::Bang => {
                self.advance();
                let operand = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::Unary(Operator::Not, operand)
            }
            RustTokenType::Ref => {
                self.advance();
                let mutable = self.match_token(&[RustTokenType::Mut]);
                let operand = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::Unary(Operator::Not, operand)
            }
            RustTokenType::Unsafe => {
                self.advance();
                let body = Box::new(self.parse_expr().unwrap_or(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                }));
                ExprKind::Block(vec![Stmt::Expr(Expr {
                    kind: ExprKind::Unit,
                    ty: None,
                    span: (0, 0),
                })], Some(body))
            }
            RustTokenType::PathSep => {
                return self.parse_path_expr();
            }
            _ => ExprKind::Unit,
        };

        Some(Expr {
            kind: expr,
            ty: None,
            span: token.span,
        })
    }

    fn parse_path_expr(&mut self) -> Option<Expr> {
        let path = self.parse_path()?;
        Some(Expr {
            kind: ExprKind::Path(path),
            ty: None,
            span: (0, 0),
        })
    }

    fn parse_struct(&mut self) -> Option<Struct> {
        let name = if let RustTokenType::Ident(id) = &self.current().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return None;
        };

        let generics = self.parse_generics();

        let fields = if self.match_token(&[RustTokenType::LBrace]) {
            let mut fields = Vec::new();
            while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                let name = if let RustTokenType::Ident(id) = &self.current().token_type {
                    let n = id.clone();
                    self.advance();
                    n
                } else {
                    self.advance();
                    continue;
                };
                self.match_token(&[RustTokenType::Colon]);
                let ty = self.parse_type().unwrap_or(Type::Infer);
                self.match_token(&[RustTokenType::Comma]);
                fields.push(StructField {
                    name,
                    ty,
                    mutable: false,
                });
            }
            self.match_token(&[RustTokenType::RBrace]);
            fields
        } else {
            Vec::new()
        };

        Some(Struct {
            name,
            generics,
            fields,
            where_clause: None,
        })
    }

    fn parse_enum(&mut self) -> Option<Enum> {
        let name = if let RustTokenType::Ident(id) = &self.current().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return None;
        };

        let generics = self.parse_generics();

        let variants = if self.match_token(&[RustTokenType::LBrace]) {
            let mut variants = Vec::new();
            while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                let name = if let RustTokenType::Ident(id) = &self.current().token_type {
                    let n = id.clone();
                    self.advance();
                    n
                } else {
                    self.advance();
                    continue;
                };

                let data = if self.match_token(&[RustTokenType::LParen]) {
                    let mut fields = Vec::new();
                    while !self.check(&RustTokenType::RParen) && !self.check(&RustTokenType::Eof) {
                        fields.push((self.parse_type().unwrap_or(Type::Infer), false));
                        if !self.match_token(&[RustTokenType::Comma]) {
                            break;
                        }
                    }
                    self.match_token(&[RustTokenType::RParen]);
                    VariantData::Tuple(fields)
                } else if self.match_token(&[RustTokenType::LBrace]) {
                    let mut fields = Vec::new();
                    while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                        let fname = if let RustTokenType::Ident(id) = &self.current().token_type {
                            let n = id.clone();
                            self.advance();
                            n
                        } else {
                            self.advance();
                            continue;
                        };
                        self.match_token(&[RustTokenType::Colon]);
                        let ty = self.parse_type().unwrap_or(Type::Infer);
                        self.match_token(&[RustTokenType::Comma]);
                        fields.push(StructField {
                            name: fname,
                            ty,
                            mutable: false,
                        });
                    }
                    self.match_token(&[RustTokenType::RBrace]);
                    VariantData::Struct(fields)
                } else {
                    VariantData::Unit
                };

                variants.push(Variant {
                    name,
                    attrs: Vec::new(),
                    data,
                    discriminant: None,
                });

                if !self.match_token(&[RustTokenType::Comma]) {
                    break;
                }
            }
            self.match_token(&[RustTokenType::RBrace]);
            variants
        } else {
            Vec::new()
        };

        Some(Enum {
            name,
            generics,
            variants,
            where_clause: None,
        })
    }

    fn parse_trait(&mut self) -> Option<Trait> {
        let name = if let RustTokenType::Ident(id) = &self.current().token_type {
            let n = id.clone();
            self.advance();
            n
        } else {
            return None;
        };

        let generics = self.parse_generics();

        let super_traits = if self.match_token(&[RustTokenType::Colon]) {
            let mut bounds = Vec::new();
            while !self.check(&RustTokenType::LBrace) && !self.check(&RustTokenType::Eof) {
                bounds.push(self.parse_type().unwrap_or(Type::Infer));
                if !self.match_token(&[RustTokenType::Plus]) {
                    break;
                }
            }
            bounds
        } else {
            Vec::new()
        };

        let items = if self.match_token(&[RustTokenType::LBrace]) {
            let mut items = Vec::new();
            while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                if self.match_token(&[RustTokenType::Fn]) {
                    if let Some(func) = self.parse_function() {
                        items.push(TraitItem::Function(func));
                    }
                } else {
                    self.advance();
                }
            }
            self.match_token(&[RustTokenType::RBrace]);
            items
        } else {
            self.match_token(&[RustTokenType::Semi]);
            Vec::new()
        };

        Some(Trait {
            name,
            generics,
            items,
            super_traits,
            where_clause: None,
        })
    }

    fn parse_impl(&mut self) -> Option<Impl> {
        let generics = self.parse_generics();

        let trait_ty = if self.match_token(&[RustTokenType::For]) {
            None  // Simplified
        } else {
            None
        };

        let self_ty = self.parse_type().unwrap_or(Type::Infer);

        let items = if self.match_token(&[RustTokenType::LBrace]) {
            let mut items = Vec::new();
            while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                if self.match_token(&[RustTokenType::Fn]) {
                    if let Some(func) = self.parse_function() {
                        items.push(ImplItem::Function(func));
                    }
                } else {
                    self.advance();
                }
            }
            self.match_token(&[RustTokenType::RBrace]);
            items
        } else {
            self.match_token(&[RustTokenType::Semi]);
            Vec::new()
        };

        Some(Impl {
            generics,
            trait_ty,
            self_ty,
            items,
            where_clause: None,
        })
    }

    fn parse_macro_rules(&mut self) -> Vec<MacroRule> {
        let mut rules = Vec::new();
        if self.match_token(&[RustTokenType::LBrace]) {
            while !self.check(&RustTokenType::RBrace) && !self.check(&RustTokenType::Eof) {
                // Simple: skip to matching brace
                let mut depth = 1;
                while depth > 0 && !self.check(&RustTokenType::Eof) {
                    if self.check(&RustTokenType::LBrace) {
                        depth += 1;
                    } else if self.check(&RustTokenType::RBrace) {
                        depth -= 1;
                    }
                    self.advance();
                }
                self.match_token(&[RustTokenType::Comma]);
            }
            self.match_token(&[RustTokenType::RBrace]);
        }
        rules
    }

    pub fn parse(&mut self) -> (Vec<Item>, Vec<ParseError>) {
        let mut items = Vec::new();

        while !self.check(&RustTokenType::Eof) {
            if let Some(item) = self.parse_item() {
                items.push(item);
            } else {
                self.advance();
            }
        }

        (items, self.errors.clone())
    }
}

// ============================================================================
// Rust AST to TUMC IR Converter
// ============================================================================

pub struct RustAstConverter;

impl RustAstConverter {
    pub fn new() -> Self {
        Self
    }

    pub fn convert_item(&self, item: &Item) -> Option<AstNode> {
        match &item.kind {
            ItemKind::Function(func) => Some(self.convert_function(func, &item.attrs)),
            ItemKind::Struct(s) => Some(self.convert_struct(s)),
            ItemKind::Enum(e) => Some(self.convert_enum(e)),
            ItemKind::Trait(t) => Some(self.convert_trait(t)),
            ItemKind::Impl(i) => Some(self.convert_impl(i)),
            ItemKind::Global(name, ty, val) => Some(AstNode::Global {
                name: name.clone(),
                ty: self.convert_type(ty),
                value: self.convert_expr(val),
            }),
            _ => None,
        }
    }

    fn convert_function(&self, func: &Function, attrs: &[Attribute]) -> AstNode {
        AstNode::FunctionDef {
            name: func.name.clone(),
            params: func.params.iter().map(|p| p.name.clone()).collect(),
            ret_ty: func.ret.clone().map(|t| self.convert_type(&t)),
            body: func.body.as_ref().map(|e| self.convert_expr(e)),
        }
    }

    fn convert_struct(&self, s: &Struct) -> AstNode {
        AstNode::StructDef {
            name: s.name.clone(),
            fields: s.fields.iter().map(|f| (f.name.clone(), self.convert_type(&f.ty))).collect(),
        }
    }

    fn convert_enum(&self, e: &Enum) -> AstNode {
        AstNode::EnumDef {
            name: e.name.clone(),
            variants: e.variants.iter().map(|v| v.name.clone()).collect(),
        }
    }

    fn convert_trait(&self, t: &Trait) -> AstNode {
        AstNode::TraitDef {
            name: t.name.clone(),
            methods: t.items.iter().filter_map(|i| {
                if let TraitItem::Function(f) = i {
                    Some(f.name.clone())
                } else {
                    None
                }
            }).collect(),
        }
    }

    fn convert_impl(&self, i: &Impl) -> AstNode {
        AstNode::ImplBlock {
            trait_name: None,
            self_ty: self.convert_type(&i.self_ty),
            methods: i.items.iter().filter_map(|item| {
                if let ImplItem::Function(f) = item {
                    Some(AstNode::FunctionDef {
                        name: f.name.clone(),
                        params: f.params.iter().map(|p| p.name.clone()).collect(),
                        ret_ty: f.ret.clone().map(|t| self.convert_type(&t)),
                        body: f.body.as_ref().map(|e| self.convert_expr(e)),
                    })
                } else {
                    None
                }
            }).collect(),
        }
    }

    fn convert_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Bool => Type::Bool,
            Type::Int(_) => Type::Int64,
            Type::Uint(_) => Type::UInt64,
            Type::Float(_) => Type::Float64,
            Type::Str => Type::String,
            Type::Char => Type::Char,
            Type::SelfType => Type::SelfType,
            Type::Never => Type::Never,
            Type::Unit => Type::Unit,
            Type::Tuple(types) => Type::Tuple(types.iter().map(|t| self.convert_type(t)).collect()),
            Type::Array(arr) => Type::Array(Box::new(self.convert_type(&arr.elem))),
            Type::Slice(s) => Type::Slice(Box::new(self.convert_type(&s.elem))),
            Type::Pointer(p) => Type::Pointer(Box::new(self.convert_type(&p.inner))),
            Type::Reference(r) => Type::Reference(Box::new(self.convert_type(&r.inner))),
            Type::Function(f) => Type::Function {
                params: f.params.iter().map(|t| self.convert_type(t)).collect(),
                ret: Box::new(self.convert_type(&f.ret)),
            },
            Type::Path(path) => Type::Named(path.segments.last().map(|s| s.ident.clone()).unwrap_or_default()),
            _ => Type::Unknown,
        }
    }

    fn convert_expr(&self, expr: &Expr) -> AstNode {
        match &expr.kind {
            ExprKind::Unit => AstNode::Literal(Literal::Unit),
            ExprKind::Bool(b) => AstNode::Literal(Literal::Boolean(*b)),
            ExprKind::Char(c) => AstNode::Literal(Literal::Char(*c)),
            ExprKind::Int(i) => AstNode::Literal(Literal::Integer(*i)),
            ExprKind::Float(f) => AstNode::Literal(Literal::Float(*f)),
            ExprKind::String(s) => AstNode::Literal(Literal::String(s.clone())),
            ExprKind::Ident(id) => AstNode::Variable(id.clone()),
            ExprKind::Path(path) => AstNode::Variable(path.segments.last().map(|s| s.ident.clone()).unwrap_or_default()),
            ExprKind::Binary(op, lhs, rhs) => AstNode::BinaryOp {
                op: self.convert_operator(op),
                left: Box::new(self.convert_expr(lhs)),
                right: Box::new(self.convert_expr(rhs)),
            },
            ExprKind::Unary(op, operand) => AstNode::UnaryOp {
                op: self.convert_operator(op),
                operand: Box::new(self.convert_expr(operand)),
            },
            ExprKind::Call(callee, args) => AstNode::Call {
                func: Box::new(self.convert_expr(callee)),
                args: args.iter().map(|a| self.convert_expr(a)).collect(),
            },
            ExprKind::MethodCall(method, args, _) => AstNode::MethodCall {
                object: Box::new(AstNode::Unit),
                method: method.clone(),
                args: args.iter().map(|a| self.convert_expr(a)).collect(),
            },
            ExprKind::If(cond, then, else_part) => AstNode::If {
                condition: Box::new(self.convert_expr(cond)),
                then_branch: Box::new(self.convert_expr(then)),
                else_branch: else_part.as_ref().map(|e| Box::new(self.convert_expr(e))),
            },
            ExprKind::Match(expr, arms) => AstNode::Match {
                expr: Box::new(self.convert_expr(expr)),
                arms: arms.iter().map(|arm| {
                    let pat_str = match &arm.pat {
                        Pat::Bind(name, _, _) => name.clone(),
                        Pat::Wild => "_".to_string(),
                        _ => "_".to_string(),
                    };
                    (pat_str, Box::new(self.convert_expr(&arm.body)))
                }).collect(),
            },
            ExprKind::Loop(body) => AstNode::Loop(Box::new(self.convert_expr(body))),
            ExprKind::While(cond, body) => AstNode::While {
                condition: Box::new(self.convert_expr(cond)),
                body: Box::new(self.convert_expr(body)),
            },
            ExprKind::For(pat, iter, body) => {
                let pat_str = match pat {
                    Pat::Bind(name, _, _) => name.clone(),
                    _ => "_".to_string(),
                };
                AstNode::For {
                    var: pat_str,
                    iter: Box::new(self.convert_expr(iter)),
                    body: Box::new(self.convert_expr(body)),
                }
            }
            ExprKind::Return(val) => AstNode::Return(val.as_ref().map(|e| Box::new(self.convert_expr(e)))),
            ExprKind::Block(stmts, expr) => {
                let mut nodes: Vec<AstNode> = stmts.iter().map(|s| self.convert_stmt(s)).collect();
                if let Some(e) = expr {
                    nodes.push(self.convert_expr(e));
                }
                AstNode::Block(nodes)
            }
            ExprKind::Tuple(items) => AstNode::Tuple(items.iter().map(|e| self.convert_expr(e)).collect()),
            ExprKind::Array(items) => AstNode::Array(items.iter().map(|e| self.convert_expr(e)).collect()),
            ExprKind::Field(expr, name) => AstNode::FieldAccess {
                object: Box::new(self.convert_expr(expr)),
                field: name.clone(),
            },
            ExprKind::Index(expr, idx) => AstNode::Index {
                array: Box::new(self.convert_expr(expr)),
                index: Box::new(self.convert_expr(idx)),
            },
            ExprKind::Struct(_, fields) => AstNode::StructInit {
                name: String::new(),
                fields: fields.iter().map(|f| (f.name.clone(), self.convert_expr(&f.value))).collect(),
            },
            ExprKind::Break(_, _) => AstNode::Break,
            ExprKind::Continue(_) => AstNode::Continue,
            ExprKind::Try(expr) => AstNode::Try(Box::new(self.convert_expr(expr))),
            ExprKind::QuestionMark(expr) => AstNode::QuestionMark(Box::new(self.convert_expr(expr))),
            ExprKind::Cast(expr, ty) => AstNode::Cast {
                expr: Box::new(self.convert_expr(expr)),
                target_type: self.convert_type(ty),
            },
            ExprKind::AsmCast(expr, ty, _) => AstNode::Cast {
                expr: Box::new(self.convert_expr(expr)),
                target_type: self.convert_type(ty),
            },
            ExprKind::Macro(invoke) => AstNode::MacroInvoke {
                name: invoke.path.segments.last().map(|s| s.ident.clone()).unwrap_or_default(),
                args: vec![],
            },
            _ => AstNode::Unit,
        }
    }

    fn convert_stmt(&self, stmt: &Stmt) -> AstNode {
        match stmt {
            Stmt::Let(pat, ty, _, init) => AstNode::Let {
                name: match pat {
                    Pat::Bind(name, _, _) => name.clone(),
                    _ => "_".to_string(),
                },
                ty: ty.clone().map(|t| self.convert_type(&t)),
                value: init.as_ref().map(|e| Box::new(self.convert_expr(e))),
            },
            Stmt::Item(item) => self.convert_item(item).unwrap_or(AstNode::Unit),
            Stmt::Expr(expr) => self.convert_expr(expr),
            Stmt::Semi(expr) => self.convert_expr(expr),
        }
    }

    fn convert_operator(&self, op: &Operator) -> Operator {
        match op {
            Operator::Add => Operator::Add,
            Operator::Sub => Operator::Sub,
            Operator::Mul => Operator::Mul,
            Operator::Div => Operator::Div,
            Operator::Rem => Operator::Rem,
            Operator::BitAnd => Operator::BitAnd,
            Operator::BitOr => Operator::BitOr,
            Operator::BitXor => Operator::BitXor,
            Operator::Shl => Operator::Shl,
            Operator::Shr => Operator::Shr,
            Operator::Neg => Operator::Neg,
            Operator::Not => Operator::Not,
        }
    }
}

// ============================================================================
// Public API
// ============================================================================

pub struct RustFrontend {
    lexer: RustLexer,
    parser: RustParser,
    converter: RustAstConverter,
}

impl RustFrontend {
    pub fn new() -> Self {
        Self {
            lexer: RustLexer::new(""),
            parser: RustParser::new(vec![]),
            converter: RustAstConverter::new(),
        }
    }

    pub fn parse(&mut self, source: &str) -> Result<Vec<AstNode>, Vec<ParseError>> {
        self.lexer = RustLexer::new(source);
        let tokens = self.lexer.tokenize();

        self.parser = RustParser::new(tokens);
        let (items, errors) = self.parser.parse();

        if !errors.is_empty() {
            return Err(errors);
        }

        let ast: Vec<AstNode> = items
            .iter()
            .filter_map(|item| self.converter.convert_item(item))
            .collect();

        Ok(ast)
    }

    pub fn tokenize(&mut self, source: &str) -> Vec<RustToken> {
        self.lexer = RustLexer::new(source);
        self.lexer.tokenize()
    }
}

impl Default for RustFrontend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_keywords() {
        let mut lexer = RustLexer::new("fn let const if else struct enum");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0].token_type, RustTokenType::Fn);
        assert_eq!(tokens[1].token_type, RustTokenType::Let);
        assert_eq!(tokens[2].token_type, RustTokenType::Const);
        assert_eq!(tokens[3].token_type, RustTokenType::If);
        assert_eq!(tokens[4].token_type, RustTokenType::Else);
        assert_eq!(tokens[5].token_type, RustTokenType::Struct);
        assert_eq!(tokens[6].token_type, RustTokenType::Enum);
    }

    #[test]
    fn test_lexer_numbers() {
        let mut lexer = RustLexer::new("42 3.14 0xFF 0o77 0b1010 123i32 3.14f64");
        let tokens = lexer.tokenize();

        match &tokens[0].token_type {
            RustTokenType::Integer(v, s) => {
                assert_eq!(*v, 42);
                assert!(s.is_none());
            }
            _ => panic!("Expected integer"),
        }

        match &tokens[1].token_type {
            RustTokenType::Float(v, s) => {
                assert!((*v - 3.14).abs() < 0.001);
                assert!(s.is_none());
            }
            _ => panic!("Expected float"),
        }
    }

    #[test]
    fn test_lexer_strings() {
        let mut lexer = RustLexer::new("\"hello\" r\"raw\\nstring\"");
        let tokens = lexer.tokenize();

        match &tokens[0].token_type {
            RustTokenType::String(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_parser_function() {
        let source = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
        if let AstNode::FunctionDef { name, .. } = &ast[0] {
            assert_eq!(name, "add");
        }
    }

    #[test]
    fn test_parser_struct() {
        let source = r#"
            struct Point {
                x: f64,
                y: f64,
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
        if let AstNode::StructDef { name, .. } = &ast[0] {
            assert_eq!(name, "Point");
        }
    }

    #[test]
    fn test_parser_enum() {
        let source = r#"
            enum Color {
                Red,
                Green,
                Blue,
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
        if let AstNode::EnumDef { name, .. } = &ast[0] {
            assert_eq!(name, "Color");
        }
    }

    #[test]
    fn test_parser_if_expression() {
        let source = r#"
            fn max(a: i32, b: i32) -> i32 {
                if a > b {
                    a
                } else {
                    b
                }
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_parser_match() {
        let source = r#"
            fn direction(dir: i32) -> &'static str {
                match dir {
                    0 => "north",
                    1 => "east",
                    2 => "south",
                    _ => "west",
                }
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_parser_loop() {
        let source = r#"
            fn count_up(to: i32) {
                let mut i = 0;
                while i < to {
                    i = i + 1;
                }
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_parser_traits() {
        let source = r#"
            trait Drawable {
                fn draw(&self);
                fn area(&self) -> f64;
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
    }

    #[test]
    fn test_parser_impl() {
        let source = r#"
            impl Drawable for Circle {
                fn draw(&self) {
                    // draw circle
                }
                fn area(&self) -> f64 {
                    3.14159 * self.radius * self.radius
                }
            }
        "#;

        let mut frontend = RustFrontend::new();
        let ast = frontend.parse(source).unwrap();

        assert!(!ast.is_empty());
    }
}
