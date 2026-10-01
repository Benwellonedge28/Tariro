//! Error handling and diagnostic reporting for TUMC compiler

use crate::prelude::*;
use std::fmt;

/// Severity level for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Internal compiler bug or assertion failure
    Internal = 0,
    /// Cannot continue compilation
    Error = 1,
    /// Problem that might affect output but can continue
    Warning = 2,
    /// Informational message
    Note = 3,
    /// Additional helpful information
    Help = 4,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Internal => write!(f, "internal"),
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Note => write!(f, "note"),
            Severity::Help => write!(f, "help"),
        }
    }
}

/// Source code location
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    /// 0-indexed line number
    pub start_line: usize,
    /// 0-indexed column number (byte offset)
    pub start_col: usize,
    /// 0-indexed line number
    pub end_line: usize,
    /// 0-indexed column number (byte offset)
    pub end_col: usize,
}

impl Span {
    /// Create a new span
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Span {
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }

    /// Create a zero-length span at a point
    pub fn point(line: usize, col: usize) -> Self {
        Span {
            start_line: line,
            start_col: col,
            end_line: line,
            end_col: col,
        }
    }

    /// Check if span is empty
    pub fn is_empty(&self) -> bool {
        self.start_line == self.end_line && self.start_col == self.end_col
    }

    /// Merge two spans into one covering both
    pub fn merge(&self, other: &Span) -> Span {
        if self.start_line < other.start_line
            || (self.start_line == other.start_line && self.start_col < other.start_col)
        {
            Span {
                start_line: self.start_line,
                start_col: self.start_col,
                end_line: other.end_line,
                end_col: other.end_col,
            }
        } else {
            Span {
                start_line: other.start_line,
                start_col: other.start_col,
                end_line: self.end_line,
                end_col: self.end_col,
            }
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start_line == self.end_line {
            if self.start_col == self.end_col {
                write!(f, "{}:{}", self.start_line + 1, self.start_col + 1)
            } else {
                write!(f, "{}:{}-{}", self.start_line + 1, self.start_col + 1, self.end_col + 1)
            }
        } else {
            write!(
                f, "{}:{}-{}:{}",
                self.start_line + 1, self.start_col + 1,
                self.end_line + 1, self.end_col + 1
            )
        }
    }
}

/// A diagnostic message with location and severity
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Severity level
    pub severity: Severity,
    /// Source code location
    pub span: Span,
    /// Error code (e.g., "E0001")
    pub code: Option<String>,
    /// Primary message
    pub message: String,
    /// Additional notes
    pub notes: Vec<String>,
    /// Suggested fixes
    pub suggestions: Vec<Suggestion>,
}

impl Diagnostic {
    /// Create a new diagnostic
    pub fn new(severity: Severity, span: Span, message: impl Into<String>) -> Self {
        Diagnostic {
            severity,
            span,
            code: None,
            message: message.into(),
            notes: vec![],
            suggestions: vec![],
        }
    }

    /// Add an error code
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Add a note
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add multiple notes
    pub fn with_notes(mut self, notes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.notes.extend(notes.into_iter().map(|n| n.into()));
        self
    }

    /// Add a suggestion
    pub fn with_suggestion(mut self, suggestion: Suggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Format diagnostic for display
    pub fn format(&self, source_name: &str) -> String {
        let mut output = String::new();

        // Severity and code
        if let Some(ref code) = self.code {
            output.push_str(&format!("{}: ", code));
        }
        output.push_str(&format!("{}: ", self.severity));

        // Location
        output.push_str(&format!("{} at {}\n", self.span, source_name));

        // Message
        output.push_str(&format!("{}\n", self.message));

        // Notes
        for note in &self.notes {
            output.push_str(&format!("  = {}\n", note));
        }

        // Suggestions
        for suggestion in &self.suggestions {
            output.push_str(&format!("  {}\n", suggestion));
        }

        output
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} at {}", self.severity, self.message, self.span)
    }
}

/// A suggested fix
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// Type of change
    pub kind: SuggestionKind,
    /// Description of the change
    pub description: String,
    /// The replacement text (if applicable)
    pub replacement: Option<String>,
    /// Span to replace
    pub span: Option<Span>,
}

impl Suggestion {
    /// Create a new suggestion
    pub fn new(description: impl Into<String>) -> Self {
        Suggestion {
            kind: SuggestionKind::Refactor,
            description: description.into(),
            replacement: None,
            span: None,
        }
    }

    /// Create a replacement suggestion
    pub fn replace(span: Span, replacement: impl Into<String>) -> Self {
        Suggestion {
            kind: SuggestionKind::Replace,
            description: format!("Replace with '{}'", replacement.into()),
            replacement: Some(replacement.into()),
            span: Some(span),
        }
    }

    /// Create an insertion suggestion
    pub fn insert(span: Span, text: impl Into<String>) -> Self {
        Suggestion {
            kind: SuggestionKind::Insert,
            description: format!("Insert '{}'", text.into()),
            replacement: Some(text.into()),
            span: Some(span),
        }
    }

    /// Create a deletion suggestion
    pub fn delete(span: Span) -> Self {
        Suggestion {
            kind: SuggestionKind::Delete,
            description: "Delete".to_string(),
            replacement: None,
            span: Some(span),
        }
    }
}

impl fmt::Display for Suggestion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            SuggestionKind::Replace => {
                if let Some(ref span) = self.span {
                    write!(f, "replace {} with '{}'", span, self.replacement.as_ref().unwrap())
                } else {
                    write!(f, "{}", self.description)
                }
            }
            SuggestionKind::Insert => {
                if let Some(ref span) = self.span {
                    write!(f, "insert at {} '{}'", span, self.replacement.as_ref().unwrap())
                } else {
                    write!(f, "{}", self.description)
                }
            }
            SuggestionKind::Delete => {
                if let Some(ref span) = self.span {
                    write!(f, "delete {}", span)
                } else {
                    write!(f, "{}", self.description)
                }
            }
            SuggestionKind::Refactor => write!(f, "consider: {}", self.description),
        }
    }
}

/// Kind of suggestion
#[derive(Debug, Clone, Copy)]
pub enum SuggestionKind {
    /// Replace existing code
    Replace,
    /// Insert new code
    Insert,
    /// Delete existing code
    Delete,
    /// General refactoring suggestion
    Refactor,
}

/// Error kinds for the compiler
#[derive(Debug, Clone)]
pub enum ErrorKind {
    // Lexer errors
    /// Invalid character encountered
    InvalidCharacter(char),
    /// Unterminated string literal
    UnterminatedString,
    /// Unterminated comment
    UnterminatedComment,
    /// Invalid escape sequence
    InvalidEscape(String),
    /// Invalid number literal
    InvalidNumber(String),

    // Parser errors
    /// Unexpected token
    UnexpectedToken {
        expected: String,
        found: String,
    },
    /// Unexpected end of input
    UnexpectedEndOfInput,
    /// Expected token not found
    ExpectedToken(String),
    /// Invalid expression
    InvalidExpression(String),
    /// Invalid statement
    InvalidStatement(String),
    /// Invalid pattern
    InvalidPattern(String),
    /// Invalid type annotation
    InvalidTypeAnnotation(String),
    /// Generic parsing error
    ParseError(String),

    // Type errors
    /// Type mismatch
    TypeMismatch {
        expected: String,
        found: String,
    },
    /// Undefined type
    UndefinedType(String),
    /// Undefined variable
    UndefinedVariable(String),
    /// Unification failure
    UnificationFailure {
        left: String,
        right: String,
    },
    /// Cannot infer type
    CannotInferType(String),
    /// Recursive type definition
    RecursiveType(String),

    // Name resolution errors
    /// Name collision
    NameCollision {
        name: String,
        first_location: String,
        second_location: String,
    },
    /// Unresolved import
    UnresolvedImport(String),
    /// Unresolved reference
    UnresolvedReference(String),

    // Code generation errors
    /// Unsupported feature
    UnsupportedFeature(String),
    /// Invalid target
    InvalidTarget(String),
    /// Linker error
    LinkerError(String),

    // Internal errors
    /// Compiler bug
    InternalError(String),
    /// Assertion failure
    AssertionFailure(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::InvalidCharacter(c) => {
                write!(f, "Invalid character: '{}'", c)
            }
            ErrorKind::UnterminatedString => {
                write!(f, "Unterminated string literal")
            }
            ErrorKind::UnterminatedComment => {
                write!(f, "Unterminated block comment")
            }
            ErrorKind::InvalidEscape(seq) => {
                write!(f, "Invalid escape sequence: '\\{}'", seq)
            }
            ErrorKind::InvalidNumber(s) => {
                write!(f, "Invalid number literal: {}", s)
            }
            ErrorKind::UnexpectedToken { expected, found } => {
                write!(f, "Unexpected token '{}', expected {}", found, expected)
            }
            ErrorKind::UnexpectedEndOfInput => {
                write!(f, "Unexpected end of input")
            }
            ErrorKind::ExpectedToken(token) => {
                write!(f, "Expected token: {}", token)
            }
            ErrorKind::InvalidExpression(msg) => {
                write!(f, "Invalid expression: {}", msg)
            }
            ErrorKind::InvalidStatement(msg) => {
                write!(f, "Invalid statement: {}", msg)
            }
            ErrorKind::InvalidPattern(msg) => {
                write!(f, "Invalid pattern: {}", msg)
            }
            ErrorKind::InvalidTypeAnnotation(msg) => {
                write!(f, "Invalid type annotation: {}", msg)
            }
            ErrorKind::ParseError(msg) => {
                write!(f, "Parse error: {}", msg)
            }
            ErrorKind::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
            ErrorKind::UndefinedType(name) => {
                write!(f, "Undefined type: {}", name)
            }
            ErrorKind::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            ErrorKind::UnificationFailure { left, right } => {
                write!(f, "Cannot unify {} with {}", left, right)
            }
            ErrorKind::CannotInferType(context) => {
                write!(f, "Cannot infer type in context: {}", context)
            }
            ErrorKind::RecursiveType(name) => {
                write!(f, "Recursive type definition: {}", name)
            }
            ErrorKind::NameCollision { name, .. } => {
                write!(f, "Name collision for '{}'", name)
            }
            ErrorKind::UnresolvedImport(path) => {
                write!(f, "Unresolved import: {}", path)
            }
            ErrorKind::UnresolvedReference(name) => {
                write!(f, "Unresolved reference: {}", name)
            }
            ErrorKind::UnsupportedFeature(feature) => {
                write!(f, "Unsupported feature: {}", feature)
            }
            ErrorKind::InvalidTarget(target) => {
                write!(f, "Invalid target: {}", target)
            }
            ErrorKind::LinkerError(msg) => {
                write!(f, "Linker error: {}", msg)
            }
            ErrorKind::InternalError(msg) => {
                write!(f, "Internal compiler error: {}", msg)
            }
            ErrorKind::AssertionFailure(msg) => {
                write!(f, "Assertion failure: {}", msg)
            }
        }
    }
}

/// Compile error with location
#[derive(Debug, Clone)]
pub struct CompileError {
    /// Error kind
    pub kind: ErrorKind,
    /// Source location
    pub span: Span,
    /// Source file name
    pub file: Option<String>,
}

impl CompileError {
    /// Create a new compile error
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        CompileError {
            kind,
            span,
            file: None,
        }
    }

    /// Create with file name
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    /// Convert to diagnostic
    pub fn to_diagnostic(&self) -> Diagnostic {
        let severity = match &self.kind {
            ErrorKind::InternalError(_) | ErrorKind::AssertionFailure(_) => Severity::Internal,
            _ => Severity::Error,
        };

        let code = match &self.kind {
            ErrorKind::InvalidCharacter(_) => Some("E0001".to_string()),
            ErrorKind::UnterminatedString => Some("E0002".to_string()),
            ErrorKind::UnterminatedComment => Some("E0003".to_string()),
            ErrorKind::InvalidEscape(_) => Some("E0004".to_string()),
            ErrorKind::InvalidNumber(_) => Some("E0005".to_string()),
            ErrorKind::UnexpectedToken { .. } => Some("E0006".to_string()),
            ErrorKind::UnexpectedEndOfInput => Some("E0007".to_string()),
            ErrorKind::ExpectedToken(_) => Some("E0008".to_string()),
            ErrorKind::InvalidExpression(_) => Some("E0009".to_string()),
            ErrorKind::InvalidStatement(_) => Some("E0010".to_string()),
            ErrorKind::InvalidPattern(_) => Some("E0011".to_string()),
            ErrorKind::InvalidTypeAnnotation(_) => Some("E0012".to_string()),
            ErrorKind::ParseError(_) => Some("E0013".to_string()),
            ErrorKind::TypeMismatch { .. } => Some("E0014".to_string()),
            ErrorKind::UndefinedType(_) => Some("E0015".to_string()),
            ErrorKind::UndefinedVariable(_) => Some("E0016".to_string()),
            ErrorKind::UnificationFailure { .. } => Some("E0017".to_string()),
            ErrorKind::CannotInferType(_) => Some("E0018".to_string()),
            ErrorKind::RecursiveType(_) => Some("E0019".to_string()),
            ErrorKind::NameCollision { .. } => Some("E0020".to_string()),
            ErrorKind::UnresolvedImport(_) => Some("E0021".to_string()),
            ErrorKind::UnresolvedReference(_) => Some("E0022".to_string()),
            ErrorKind::UnsupportedFeature(_) => Some("E0023".to_string()),
            ErrorKind::InvalidTarget(_) => Some("E0024".to_string()),
            ErrorKind::LinkerError(_) => Some("E0025".to_string()),
            ErrorKind::InternalError(_) => Some("E0026".to_string()),
            ErrorKind::AssertionFailure(_) => Some("E0027".to_string()),
        };

        let message = self.kind.to_string();

        Diagnostic::new(severity, self.span.clone(), message)
            .with_code(code.unwrap_or_default())
            .with_note(format!("in file {:?}", self.file))
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} - {}", self.span, self.file.as_deref().unwrap_or("?"), self.kind)
    }
}

impl std::error::Error for CompileError {}

/// Result type for compilation
pub type CompileResult<T> = Result<T, Vec<CompileError>>;

/// Error handler for collecting and reporting errors
pub struct ErrorHandler {
    /// Collected errors
    errors: Vec<CompileError>,
    /// Maximum errors before bailing
    max_errors: usize,
    /// Warnings
    warnings: Vec<Diagnostic>,
}

impl ErrorHandler {
    /// Create new error handler
    pub fn new() -> Self {
        ErrorHandler {
            errors: vec![],
            max_errors: 100,
            warnings: vec![],
        }
    }

    /// Set maximum errors
    pub fn max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    /// Add an error
    pub fn add_error(&mut self, error: CompileError) {
        if self.errors.len() < self.max_errors {
            self.errors.push(error);
        }
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: Diagnostic) {
        self.warnings.push(warning);
    }

    /// Check if any errors occurred
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get warning count
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// Get all errors
    pub fn errors(&self) -> &[CompileError] {
        &self.errors
    }

    /// Get all warnings
    pub fn warnings(&self) -> &[Diagnostic] {
        &self.warnings
    }

    /// Take all errors
    pub fn take_errors(&mut self) -> Vec<CompileError> {
        std::mem::take(&mut self.errors)
    }

    /// Take all warnings
    pub fn take_warnings(&mut self) -> Vec<Diagnostic> {
        std::mem::take(&mut self.warnings)
    }

    /// Emit all diagnostics
    pub fn emit_all(&self, source_name: &str) {
        for error in &self.errors {
            eprintln!("{}", error.to_diagnostic().format(source_name));
        }
        for warning in &self.warnings {
            eprintln!("{}", warning.format(source_name));
        }
    }

    /// Check if should continue
    pub fn should_continue(&self) -> bool {
        self.errors.len() < self.max_errors
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_operations() {
        let span1 = Span::point(5, 10);
        let span2 = Span::new(5, 20, 7, 5);

        let merged = span1.merge(&span2);
        assert_eq!(merged.start_line, 5);
        assert_eq!(merged.start_col, 10);
        assert_eq!(merged.end_line, 7);
        assert_eq!(merged.end_col, 5);
    }

    #[test]
    fn test_diagnostic_format() {
        let diag = Diagnostic::new(
            Severity::Error,
            Span::point(10, 5),
            "Test error",
        )
        .with_code("E0001")
        .with_note("This is a note")
        .with_suggestion(Suggestion::new("Try this instead"));

        let formatted = diag.format("test.tumc");
        assert!(formatted.contains("E0001"));
        assert!(formatted.contains("error"));
        assert!(formatted.contains("Test error"));
    }
}
