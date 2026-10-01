//! TUMC Lexer - Tokenizer for source code
//!
//! This module provides lexical analysis for converting source code
//! into a stream of tokens for the parser.
//!
//! # Supported Languages
//!
//! - Rust
//! - Python
//! - JavaScript
//! - TUMC IR

use crate::tumc_compiler::{SourceLocation, SourceLanguage};
use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;

// ============================================================================
// Token Types
// ============================================================================

/// Token type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    /// Integer literal (e.g., 42, 0xFF)
    Integer(i64),

    /// Float literal (e.g., 3.14, 1e-10)
    Float(f64),

    /// String literal (e.g., "hello")
    String(String),

    /// Character literal (e.g., 'a')
    Char(char),

    /// Boolean literal (true, false)
    Bool(bool),

    /// Identifier (e.g., foo, my_var)
    Identifier(String),

    // Keywords
    /// fn, function, def
    Fn,

    /// let, const, var
    Let,

    /// if
    If,

    /// else
    Else,

    /// while
    While,

    /// for
    For,

    /// loop
    Loop,

    /// return
    Return,

    /// break
    Break,

    /// continue
    Continue,

    /// struct
    Struct,

    /// enum
    Enum,

    /// impl
    Impl,

    /// trait
    Trait,

    /// type
    Type,

    /// pub, public
    Pub,

    /// mod, module
    Mod,

    /// use, import
    Use,

    /// as
    As,

    /// match
    Match,

    /// async
    Async,

    /// await
    Await,

    /// mut, mutable
    Mut,

    /// ref, reference
    Ref,

    // Operators
    /// +
    Plus,

    /// -
    Minus,

    /// *
    Star,

    /// /
    Slash,

    /// %
    Percent,

    /// =
    Assign,

    /// ==
    Eq,

    /// !=
    Ne,

    /// <
    Lt,

    /// <=
    Le,

    /// >
    Gt,

    /// >=
    Ge,

    /// &&
    And,

    /// ||
    Or,

    /// !
    Not,

    /// &
    BitAnd,

    /// |
    BitOr,

    /// ^
    BitXor,

    /// ~
    BitNot,

    /// <<
    Shl,

    /// >>
    Shr,

    /// +=
    PlusAssign,

    /// -=
    MinusAssign,

    /// *=
    StarAssign,

    /// /=
    SlashAssign,

    // Delimiters
    /// (
    LParen,

    /// )
    RParen,

    /// {
    LBrace,

    /// }
    RBrace,

    /// [
    LBracket,

    /// ]
    RBracket,

    /// ,
    Comma,

    /// .
    Dot,

    /// :
    Colon,

    /// ::
    DoubleColon,

    /// ;
    Semicolon,

    /// ->
    Arrow,

    /// =>
    FatArrow,

    // Special
    /// Line comment (// ...)
    Comment(String),

    /// Doc comment (/// ...)
    DocComment(String),

    /// Newline
    Newline,

    /// Whitespace (spaces, tabs)
    Whitespace,

    /// End of file
    Eof,

    /// Unknown token
    Unknown(char),
}

/// Token with position information
#[derive(Debug, Clone)]
pub struct Token {
    /// Token kind
    pub kind: TokenKind,

    /// Source location
    pub location: SourceLocation,

    /// Literal value (for literals)
    pub literal: Option<String>,
}

impl Token {
    /// Create a new token
    pub fn new(kind: TokenKind, location: SourceLocation) -> Self {
        Self {
            kind,
            location,
            literal: None,
        }
    }

    /// Create a literal token
    pub fn literal(kind: TokenKind, location: SourceLocation, literal: String) -> Self {
        Self {
            kind,
            location,
            literal: Some(literal),
        }
    }

    /// Check if token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Fn
                | TokenKind::Let
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::While
                | TokenKind::For
                | TokenKind::Loop
                | TokenKind::Return
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Struct
                | TokenKind::Enum
                | TokenKind::Impl
                | TokenKind::Trait
                | TokenKind::Type
                | TokenKind::Pub
                | TokenKind::Mod
                | TokenKind::Use
                | TokenKind::As
                | TokenKind::Match
                | TokenKind::Async
                | TokenKind::Await
                | TokenKind::Mut
                | TokenKind::Ref
        )
    }

    /// Check if token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::Assign
                | TokenKind::Eq
                | TokenKind::Ne
                | TokenKind::Lt
                | TokenKind::Le
                | TokenKind::Gt
                | TokenKind::Ge
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Not
                | TokenKind::BitAnd
                | TokenKind::BitOr
                | TokenKind::BitXor
                | TokenKind::BitNot
                | TokenKind::Shl
                | TokenKind::Shr
        )
    }
}

// ============================================================================
// Lexer Implementation
// ============================================================================

/// Lexer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerState {
    Normal,
    String,
    Char,
    Comment,
    BlockComment,
}

/// Lexer for tokenizing source code
pub struct Lexer<'a> {
    /// Source code
    source: &'a str,

    /// Character iterator
    chars: Peekable<Chars<'a>>,

    /// Current position
    position: usize,

    /// Current line
    line: usize,

    /// Current column
    column: usize,

    /// Tokens
    tokens: VecDeque<Token>,

    /// Current state
    state: LexerState,

    /// Source language
    language: SourceLanguage,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            position: 0,
            line: 1,
            column: 1,
            tokens: VecDeque::new(),
            state: LexerState::Normal,
        }
    }

    /// Set source language
    pub fn with_language(mut self, language: SourceLanguage) -> Self {
        self.language = language;
        self
    }

    /// Tokenize the source
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        while let Some(_) = self.chars.peek() {
            self.scan_token()?;
        }

        let eof_location = SourceLocation {
            line: self.line,
            column: self.column,
            offset: self.position,
        };
        self.tokens.push_back(Token::new(TokenKind::Eof, eof_location));

        Ok(self.tokens.into())
    }

    /// Scan a single token
    fn scan_token(&mut self) -> Result<(), LexerError> {
        let ch = self.peek();

        let start_location = SourceLocation {
            line: self.line,
            column: self.column,
            offset: self.position,
        };

        match self.state {
            LexerState::Normal => {
                self.advance();
                match ch {
                    // Whitespace
                    ' ' | '\t' | '\r' => {
                        // Skip whitespace
                    }
                    '\n' => {
                        self.line += 1;
                        self.column = 1;
                    }
                    // Operators and delimiters
                    '+' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens
                                .push_back(Token::new(TokenKind::PlusAssign, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::Plus, start_location));
                        }
                    }
                    '-' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens
                                .push_back(Token::new(TokenKind::MinusAssign, start_location));
                        } else if self.peek_next() == '>' {
                            self.advance();
                            self.tokens
                                .push_back(Token::new(TokenKind::Arrow, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::Minus, start_location));
                        }
                    }
                    '*' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens
                                .push_back(Token::new(TokenKind::StarAssign, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::Star, start_location));
                        }
                    }
                    '/' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens
                                .push_back(Token::new(TokenKind::SlashAssign, start_location));
                        } else if self.peek_next() == '/' {
                            self.advance();
                            self.scan_line_comment();
                        } else if self.peek_next() == '*' {
                            self.advance();
                            self.state = LexerState::BlockComment;
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::Slash, start_location));
                        }
                    }
                    '%' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::Percent, start_location));
                    }
                    '=' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::Eq, start_location));
                        } else if self.peek_next() == '>' {
                            self.advance();
                            self.tokens
                                .push_back(Token::new(TokenKind::FatArrow, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::Assign, start_location));
                        }
                    }
                    '!' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::Ne, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::Not, start_location));
                        }
                    }
                    '<' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::Le, start_location));
                        } else if self.peek_next() == '<' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::Shl, start_location));
                        } else {
                            self.tokens.push_back(Token::new(TokenKind::Lt, start_location));
                        }
                    }
                    '>' => {
                        if self.peek_next() == '=' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::Ge, start_location));
                        } else if self.peek_next() == '>' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::Shr, start_location));
                        } else {
                            self.tokens.push_back(Token::new(TokenKind::Gt, start_location));
                        }
                    }
                    '&' => {
                        if self.peek_next() == '&' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::And, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::BitAnd, start_location));
                        }
                    }
                    '|' => {
                        if self.peek_next() == '|' {
                            self.advance();
                            self.tokens.push_back(Token::new(TokenKind::Or, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::BitOr, start_location));
                        }
                    }
                    '^' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::BitXor, start_location));
                    }
                    '~' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::BitNot, start_location));
                    }
                    '(' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::LParen, start_location));
                    }
                    ')' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::RParen, start_location));
                    }
                    '{' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::LBrace, start_location));
                    }
                    '}' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::RBrace, start_location));
                    }
                    '[' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::LBracket, start_location));
                    }
                    ']' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::RBracket, start_location));
                    }
                    ',' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::Comma, start_location));
                    }
                    '.' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::Dot, start_location));
                    }
                    ':' => {
                        if self.peek_next() == ':' {
                            self.advance();
                            self.tokens
                                .push_back(Token::new(TokenKind::DoubleColon, start_location));
                        } else {
                            self.tokens
                                .push_back(Token::new(TokenKind::Colon, start_location));
                        }
                    }
                    ';' => {
                        self.tokens
                            .push_back(Token::new(TokenKind::Semicolon, start_location));
                    }
                    '"' => {
                        self.state = LexerState::String;
                    }
                    '\'' => {
                        self.state = LexerState::Char;
                    }
                    // Numeric literals
                    '0'..='9' => {
                        self.lex_number(ch, start_location)?;
                    }
                    // Identifiers and keywords
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.lex_identifier_or_keyword(ch, start_location)?;
                    }
                    _ => {
                        self.tokens
                            .push_back(Token::new(TokenKind::Unknown(ch), start_location));
                    }
                }
            }
            LexerState::String => {
                self.lex_string(start_location)?;
            }
            LexerState::Char => {
                self.lex_char(start_location)?;
            }
            LexerState::Comment => {
                // Should not happen
                self.state = LexerState::Normal;
            }
            LexerState::BlockComment => {
                self.lex_block_comment(start_location)?;
            }
        }

        Ok(())
    }

    /// Lex a string literal
    fn lex_string(&mut self, start: SourceLocation) -> Result<(), LexerError> {
        let mut value = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                self.advance();
                self.state = LexerState::Normal;
                self.tokens.push_back(Token::literal(
                    TokenKind::String(value.clone()),
                    start,
                    value,
                ));
                return Ok(());
            } else if ch == '\\' {
                self.advance();
                if let Some(&escaped) = self.chars.peek() {
                    self.advance();
                    let c = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        '0' => '\0',
                        'u' => {
                            // Unicode escape \u{XXXX}
                            self.advance();
                            let mut hex = String::new();
                            for _ in 0..4 {
                                if let Some(&h) = self.chars.peek() {
                                    if h.is_ascii_hexdigit() {
                                        hex.push(h);
                                        self.advance();
                                    }
                                }
                            }
                            let code = u32::from_str_radix(&hex, 16).unwrap_or(0xFFFD);
                            char::from_u32(code).unwrap_or('\u{FFFD}')
                        }
                        _ => escaped,
                    };
                    value.push(c);
                }
            } else if ch == '\n' {
                return Err(LexerError::UnterminatedString {
                    location: start,
                });
            } else {
                self.advance();
                value.push(ch);
            }
        }

        Err(LexerError::UnterminatedString { location: start })
    }

    /// Lex a character literal
    fn lex_char(&mut self, start: SourceLocation) -> Result<(), LexerError> {
        let mut value = char::MAX;

        if let Some(&ch) = self.chars.peek() {
            if ch == '\'' {
                return Err(LexerError::EmptyChar { location: start });
            }

            self.advance();

            if ch == '\\' {
                if let Some(&escaped) = self.chars.peek() {
                    self.advance();
                    value = match escaped {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '\\' => '\\',
                        '"' => '"',
                        '\'' => '\'',
                        '0' => '\0',
                        _ => escaped,
                    };
                }
            } else {
                value = ch;
            }
        }

        if let Some(&'\'') = self.chars.peek() {
            self.advance();
            self.state = LexerState::Normal;
            self.tokens
                .push_back(Token::literal(TokenKind::Char(value), start, value.to_string()));
            Ok(())
        } else {
            Err(LexerError::UnterminatedChar { location: start })
        }
    }

    /// Lex a line comment
    fn scan_line_comment(&mut self) {
        let mut comment = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '\n' {
                break;
            }
            self.advance();
            comment.push(ch);
        }

        // Check if it's a doc comment
        let kind = if comment.starts_with('/') && comment.len() > 1 {
            TokenKind::DocComment(comment[1..].to_string())
        } else {
            TokenKind::Comment(comment)
        };

        let location = SourceLocation {
            line: self.line,
            column: self.column,
            offset: self.position,
        };
        self.tokens.push_back(Token::new(kind, location));
    }

    /// Lex a block comment
    fn lex_block_comment(&mut self, start: SourceLocation) -> Result<(), LexerError> {
        let mut comment = String::new();
        let mut depth = 1;

        while let Some(&ch) = self.chars.peek() {
            if ch == '/' && self.position + 1 < self.source.len() {
                self.advance();
                if let Some(&next) = self.chars.peek() {
                    if next == '*' {
                        depth += 1;
                        self.advance();
                    }
                }
            } else if ch == '*' && self.position + 1 < self.source.len() {
                self.advance();
                if let Some(&next) = self.chars.peek() {
                    if next == '/' {
                        depth -= 1;
                        self.advance();
                        if depth == 0 {
                            break;
                        }
                    }
                }
            } else {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.advance();
                comment.push(ch);
            }
        }

        if depth > 0 {
            return Err(LexerError::UnterminatedBlockComment { location: start });
        }

        let location = SourceLocation {
            line: self.line,
            column: self.column,
            offset: self.position,
        };
        self.tokens.push_back(Token::new(TokenKind::Comment(comment), location));

        self.state = LexerState::Normal;
        Ok(())
    }

    /// Lex a numeric literal
    fn lex_number(&mut self, first: char, start: SourceLocation) -> Result<(), LexerError> {
        let mut value = String::new();
        value.push(first);

        // Check for hex, octal, or binary
        let base = if first == '0' {
            if let Some(&next) = self.chars.peek() {
                match next {
                    'x' | 'X' => {
                        self.advance();
                        value.push('x');
                        16
                    }
                    'o' | 'O' => {
                        self.advance();
                        value.push('o');
                        8
                    }
                    'b' | 'B' => {
                        self.advance();
                        value.push('b');
                        2
                    }
                    _ => 10,
                }
            } else {
                10
            }
        } else {
            10
        };

        // Read digits
        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit()
                || (base == 16 && (ch.is_ascii_hexdigit() || ch == '_'))
                || (base == 2 && (ch == '0' || ch == '1' || ch == '_'))
            {
                self.advance();
                if ch != '_' {
                    value.push(ch);
                }
            } else if ch == '_' && base != 10 {
                self.advance();
            } else {
                break;
            }
        }

        // Check for floating point
        if let Some(&'.') = self.chars.peek() {
            if let Some(&next) = self.nth_char(1) {
                if next.is_ascii_digit() || next == '_' {
                    self.advance();
                    value.push('.');

                    while let Some(&ch) = self.chars.peek() {
                        if ch.is_ascii_digit() || ch == '_' {
                            self.advance();
                            if ch != '_' {
                                value.push(ch);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }

            // Check for exponent
            if let Some(&'e' | &'E') = self.chars.peek() {
                self.advance();
                value.push('e');

                if let Some(&'+' | &'-') = self.chars.peek() {
                    self.advance();
                    value.push(if self.source.chars().nth(self.position - 1) == Some('-') {
                        '-'
                    } else {
                        '+'
                    });
                }

                while let Some(&ch) = self.chars.peek() {
                    if ch.is_ascii_digit() || ch == '_' {
                        self.advance();
                        if ch != '_' {
                            value.push(ch);
                        }
                    } else {
                        break;
                    }
                }
            }

            // Parse float
            let float_value: f64 = value.parse().map_err(|_| LexerError::InvalidFloat {
                value: value.clone(),
                location: start,
            })?;

            self.tokens
                .push_back(Token::literal(TokenKind::Float(float_value), start, value));
        } else {
            // Parse integer
            let clean_value = value.replace('_', "");
            let int_value: i64 = if clean_value.starts_with("0x") || clean_value.starts_with("0X")
            {
                i64::from_str_radix(&clean_value[2..], 16)
            } else if clean_value.starts_with("0o") || clean_value.starts_with("0O") {
                i64::from_str_radix(&clean_value[2..], 8)
            } else if clean_value.starts_with("0b") || clean_value.starts_with("0B") {
                i64::from_str_radix(&clean_value[2..], 2)
            } else {
                clean_value.parse()
            }
            .map_err(|_| LexerError::InvalidInteger {
                value: clean_value,
                location: start,
            })?;

            self.tokens
                .push_back(Token::literal(TokenKind::Integer(int_value), start, value));
        }

        Ok(())
    }

    /// Lex an identifier or keyword
    fn lex_identifier_or_keyword(
        &mut self,
        first: char,
        start: SourceLocation,
    ) -> Result<(), LexerError> {
        let mut value = String::new();
        value.push(first);

        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
                value.push(ch);
            } else {
                break;
            }
        }

        // Check for keywords
        let token = match value.as_str() {
            // Rust keywords
            "fn" => TokenKind::Fn,
            "let" => TokenKind::Let,
            "const" => TokenKind::Let, // const treated as let
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "loop" => TokenKind::Loop,
            "return" => TokenKind::Return,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "struct" => TokenKind::Struct,
            "enum" => TokenKind::Enum,
            "impl" => TokenKind::Impl,
            "trait" => TokenKind::Trait,
            "type" => TokenKind::Type,
            "pub" => TokenKind::Pub,
            "mod" => TokenKind::Mod,
            "use" => TokenKind::Use,
            "as" => TokenKind::As,
            "match" => TokenKind::Match,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "mut" => TokenKind::Mut,
            "ref" => TokenKind::Ref,
            "true" => TokenKind::Bool(true),
            "false" => TokenKind::Bool(false),
            // Python keywords (mapped to Rust equivalents)
            "def" => TokenKind::Fn,
            "class" => TokenKind::Struct,
            "import" | "from" => TokenKind::Use,
            "global" => TokenKind::Mut,
            // JavaScript keywords
            "function" => TokenKind::Fn,
            "var" => TokenKind::Let,
            "let" | "const" => TokenKind::Let,
            "new" => TokenKind::Struct,
            "extends" => TokenKind::Impl,
            _ => TokenKind::Identifier(value.clone()),
        };

        self.tokens
            .push_back(Token::literal(token, start, value));

        Ok(())
    }

    /// Peek at next character without advancing
    fn peek(&self) -> char {
        self.chars.peek().copied().unwrap_or('\0')
    }

    /// Peek at character n positions ahead
    fn peek_next(&self) -> char {
        self.nth_char(1)
    }

    /// Get character n positions ahead
    fn nth_char(&self, n: usize) -> char {
        self.source.chars().nth(self.position + n).unwrap_or('\0')
    }

    /// Advance to next character
    fn advance(&mut self) {
        if self.position < self.source.len() {
            self.position += 1;
            self.column += 1;
        }
        self.chars.next();
    }
}

// ============================================================================
// Lexer Errors
// ============================================================================

/// Lexer error
#[derive(Debug, Clone)]
pub enum LexerError {
    /// Unterminated string literal
    UnterminatedString { location: SourceLocation },

    /// Unterminated character literal
    UnterminatedChar { location: SourceLocation },

    /// Empty character literal
    EmptyChar { location: SourceLocation },

    /// Unterminated block comment
    UnterminatedBlockComment { location: SourceLocation },

    /// Invalid integer literal
    InvalidInteger { value: String, location: SourceLocation },

    /// Invalid float literal
    InvalidFloat { value: String, location: SourceLocation },

    /// Invalid escape sequence
    InvalidEscape { character: char, location: SourceLocation },
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnterminatedString { location } => {
                write!(f, "Unterminated string at {}:{}", location.line, location.column)
            }
            LexerError::UnterminatedChar { location } => {
                write!(f, "Unterminated character at {}:{}", location.line, location.column)
            }
            LexerError::EmptyChar { location } => {
                write!(f, "Empty character literal at {}:{}", location.line, location.column)
            }
            LexerError::UnterminatedBlockComment { location } => {
                write!(f, "Unterminated block comment at {}:{}", location.line, location.column)
            }
            LexerError::InvalidInteger { value, location } => {
                write!(
                    f,
                    "Invalid integer literal '{}' at {}:{}",
                    value, location.line, location.column
                )
            }
            LexerError::InvalidFloat { value, location } => {
                write!(
                    f,
                    "Invalid float literal '{}' at {}:{}",
                    value, location.line, location.column
                )
            }
            LexerError::InvalidEscape { character, location } => {
                write!(
                    f,
                    "Invalid escape sequence '\\{}' at {}:{}",
                    character, location.line, location.column
                )
            }
        }
    }
}

impl std::error::Error for LexerError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(source: &str) -> Vec<Token> {
        Lexer::new(source).tokenize().unwrap()
    }

    #[test]
    fn test_integer_literals() {
        let tokens = tokenize("42 0xFF 0o77 0b1010");
        assert!(matches!(tokens[0].kind, TokenKind::Integer(42)));
        assert!(matches!(tokens[1].kind, TokenKind::Integer(255)));
        assert!(matches!(tokens[2].kind, TokenKind::Integer(63)));
        assert!(matches!(tokens[3].kind, TokenKind::Integer(10)));
    }

    #[test]
    fn test_float_literals() {
        let tokens = tokenize("3.14 1.5e-10 0.5");
        assert!(matches!(tokens[0].kind, TokenKind::Float(_)));
        assert!(matches!(tokens[1].kind, TokenKind::Float(_)));
        assert!(matches!(tokens[2].kind, TokenKind::Float(_)));
    }

    #[test]
    fn test_string_literals() {
        let tokens = tokenize("\"hello\" \"world\\n\"");
        assert!(matches!(&tokens[0].kind, TokenKind::String(s) if s == "hello"));
        assert!(matches!(&tokens[1].kind, TokenKind::String(s) if s.contains('\\')));
    }

    #[test]
    fn test_keywords() {
        let tokens = tokenize("fn let if else while for loop return");
        assert!(matches!(tokens[0].kind, TokenKind::Fn));
        assert!(matches!(tokens[1].kind, TokenKind::Let));
        assert!(matches!(tokens[2].kind, TokenKind::If));
        assert!(matches!(tokens[3].kind, TokenKind::Else));
    }

    #[test]
    fn test_operators() {
        let tokens = tokenize("+ - * / % == != < <= > >=");
        assert!(matches!(tokens[0].kind, TokenKind::Plus));
        assert!(matches!(tokens[1].kind, TokenKind::Minus));
        assert!(matches!(tokens[2].kind, TokenKind::Star));
        assert!(matches!(tokens[3].kind, TokenKind::Slash));
        assert!(matches!(tokens[4].kind, TokenKind::Percent));
        assert!(matches!(tokens[5].kind, TokenKind::Eq));
        assert!(matches!(tokens[6].kind, TokenKind::Ne));
    }

    #[test]
    fn test_identifiers() {
        let tokens = tokenize("foo bar _private myVar");
        assert!(matches!(&tokens[0].kind, TokenKind::Identifier(s) if s == "foo"));
        assert!(matches!(&tokens[1].kind, TokenKind::Identifier(s) if s == "bar"));
        assert!(matches!(&tokens[2].kind, TokenKind::Identifier(s) if s == "_private"));
        assert!(matches!(&tokens[3].kind, TokenKind::Identifier(s) if s == "myVar"));
    }

    #[test]
    fn test_comments() {
        let tokens = tokenize("// comment\n42");
        assert!(matches!(&tokens[0].kind, TokenKind::Comment(c) if c == " comment"));
    }

    #[test]
    fn test_block_comments() {
        let tokens = tokenize("/* multi\nline */ 42");
        assert!(matches!(&tokens[0].kind, TokenKind::Comment(_)));
    }

    #[test]
    fn test_arrows() {
        let tokens = tokenize("-> =>");
        assert!(matches!(tokens[0].kind, TokenKind::Arrow));
        assert!(matches!(tokens[1].kind, TokenKind::FatArrow));
    }

    #[test]
    fn test_compound_assign() {
        let tokens = tokenize("+= -= *= /=");
        assert!(matches!(tokens[0].kind, TokenKind::PlusAssign));
        assert!(matches!(tokens[1].kind, TokenKind::MinusAssign));
        assert!(matches!(tokens[2].kind, TokenKind::StarAssign));
        assert!(matches!(tokens[3].kind, TokenKind::SlashAssign));
    }
}
