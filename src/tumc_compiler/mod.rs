//! TUMC Compiler - Minimum Viable Compiler for Tariro Universal Meta-Compiler
//!
//! This module provides a production-ready compiler infrastructure that transforms
//! source code into optimized executables using the POCO-REAF paradigm.
//!
//! # Architecture
//!
//! ```text
//! Source Code
//!     ↓
//! [Lexer] → Tokens
//!     ↓
//! [Parser] → AST
//!     ↓
//! [Type Checker] → Typed AST
//!     ↓
//! [Optimizer] → Optimized IR
//!     ↓
//! [Code Generator] → WASM/ASM/Bytecode
//! ```
//!
//! # Supported Languages (MVP)
//!
//! - Rust (subset)
//! - Python (subset)
//! - JavaScript (subset)
//!
//! # Supported Targets (MVP)
//!
//! - WebAssembly (WASM)
//! - Rust native
//! - JavaScript/ASM.js

pub mod lexer;
pub mod ast;
pub mod parser;
pub mod type_checker;
pub mod codegen;
pub mod optimizer;
pub mod error;
pub mod macro_system;
pub mod tariro_vm;
pub mod native_codegen;
pub mod simd_vectorizer;
pub mod lsp_server;
pub mod dwarf_debug;
pub mod derive_system;
pub mod fuzzing;
pub mod profiler;
pub mod wasm_engine;
pub mod docs;
pub mod testing;
pub mod package_manager;
pub mod build_system;
pub mod repl;
pub mod jit;
pub mod wasi;
pub mod hotreload;
pub mod component;
pub mod threading;
pub mod serialization;
pub mod plugins;
pub mod gc;
pub mod i18n;
pub mod metrics;
pub mod security;
pub mod target_x86_64;
pub mod target_arm64;
pub mod target_jvm;
pub mod target_llvm;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Public API
// ============================================================================

/// Main compiler instance
pub struct Compiler {
    /// Source language
    source_language: SourceLanguage,

    /// Target platform
    target_platform: TargetPlatform,

    /// Symbol table
    symbol_table: HashMap<String, Symbol>,

    /// Type context
    type_context: TypeContext,

    /// Optimization level
    optimization_level: OptimizationLevel,
}

/// Source language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceLanguage {
    Rust,
    Python,
    JavaScript,
    /// TUMC Intermediate Representation
    TumcIR,
}

impl Default for SourceLanguage {
    fn default() -> Self {
        SourceLanguage::TumcIR
    }
}

/// Target platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetPlatform {
    /// WebAssembly
    Wasm,

    /// Rust native binary
    Native,

    /// JavaScript/ASM.js
    JavaScript,

    /// Tariro Bytecode (POCO-REAF)
    TariroBytecode,
}

impl Default for TargetPlatform {
    fn default() -> Self {
        TargetPlatform::Wasm
    }
}

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization
    None = 0,

    /// Basic optimization
    Basic = 1,

    /// Standard optimization
    #[default]
    Standard = 2,

    /// Aggressive optimization
    Aggressive = 3,
}

/// Compilation result
#[derive(Debug, Clone)]
pub struct CompilationResult {
    /// Success status
    pub success: bool,

    /// Generated code
    pub output: Vec<u8>,

    /// Output format
    pub format: OutputFormat,

    /// Diagnostics (warnings/errors)
    pub diagnostics: Vec<Diagnostic>,

    /// Compilation statistics
    pub stats: CompilationStats,

    /// Errors if failed
    pub errors: Vec<CompilerError>,
}

/// Output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// WebAssembly binary (.wasm)
    WasmBinary,

    /// WebAssembly text format (.wat)
    WasmText,

    /// JavaScript
    JavaScript,

    /// Tariro Bytecode (.tbc)
    TariroBytecode,

    /// Executable
    NativeExecutable,
}

/// Diagnostic message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Severity
    pub severity: DiagnosticSeverity,

    /// Message
    pub message: String,

    /// Source location
    pub location: SourceLocation,

    /// Suggestion (optional)
    pub suggestion: Option<String>,
}

/// Diagnostic severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// Source location
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

/// Compilation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompilationStats {
    /// Source lines
    pub source_lines: usize,

    /// Tokens produced
    pub tokens_produced: usize,

    /// AST nodes
    pub ast_nodes: usize,

    /// Compilation time (ms)
    pub compilation_time_ms: u64,

    /// Output size (bytes)
    pub output_size: usize,
}

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub symbol_type: Type,
    pub location: SourceLocation,
    pub scope: u32,
}

/// Symbol kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Variable,
    Type,
    Constant,
    Module,
    Parameter,
}

/// Type context for type checking
#[derive(Debug, Clone, Default)]
pub struct TypeContext {
    /// Current scope level
    scope_level: u32,

    /// Available types
    types: HashMap<String, Type>,

    /// Function signatures
    functions: HashMap<String, FunctionSignature>,
}

/// Function signature
#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
}

// ============================================================================
// Implementation
// ============================================================================

impl Compiler {
    /// Create a new compiler instance
    pub fn new(source: SourceLanguage, target: TargetPlatform) -> Self {
        Self {
            source_language: source,
            target_platform: target,
            symbol_table: HashMap::new(),
            type_context: TypeContext::default(),
            optimization_level: OptimizationLevel::Standard,
        }
    }

    /// Set optimization level
    pub fn set_optimization(&mut self, level: OptimizationLevel) {
        self.optimization_level = level;
    }

    /// Compile source code
    pub fn compile(&mut self, source: &str) -> CompilationResult {
        let start = std::time::Instant::now();
        let mut diagnostics = Vec::new();
        let mut errors = Vec::new();

        // 1. Lexical Analysis
        let lexer_result = lexer::Lexer::new(source).tokenize();
        let tokens = match lexer_result {
            Ok(t) => t,
            Err(e) => {
                errors.push(CompilerError::LexerError(e));
                return CompilationResult {
                    success: false,
                    output: Vec::new(),
                    format: OutputFormat::WasmBinary,
                    diagnostics,
                    stats: CompilationStats::default(),
                    errors,
                };
            }
        };

        let tokens_produced = tokens.len();

        // 2. Parsing
        let ast = match parser::Parser::new(tokens).parse() {
            Ok(ast) => ast,
            Err(e) => {
                errors.push(CompilerError::ParserError(e));
                return CompilationResult {
                    success: false,
                    output: Vec::new(),
                    format: OutputFormat::WasmBinary,
                    diagnostics,
                    stats: CompilationStats::default(),
                    errors,
                };
            }
        };

        let ast_nodes = ast.node_count();

        // 3. Type Checking
        let typed_ast = match type_checker::TypeChecker::new(&mut self.type_context).check(&ast) {
            Ok(typed) => typed,
            Err(e) => {
                errors.push(CompilerError::TypeError(e));
                return CompilationResult {
                    success: false,
                    output: Vec::new(),
                    format: OutputFormat::WasmBinary,
                    diagnostics,
                    stats: CompilationStats::default(),
                    errors,
                };
            }
        };

        // 4. Optimization
        let optimized = if self.optimization_level != OptimizationLevel::None {
            optimizer::Optimizer::new(self.optimization_level).optimize(typed_ast)
        } else {
            typed_ast
        };

        // 5. Code Generation
        let output = match codegen::CodeGenerator::new(self.target_platform).generate(&optimized) {
            Ok(output) => output,
            Err(e) => {
                errors.push(CompilerError::CodegenError(e));
                return CompilationResult {
                    success: false,
                    output: Vec::new(),
                    format: OutputFormat::WasmBinary,
                    diagnostics,
                    stats: CompilationStats::default(),
                    errors,
                };
            }
        };

        let compilation_time = start.elapsed().as_millis() as u64;

        CompilationResult {
            success: true,
            output: output.clone(),
            format: self.target_format(),
            diagnostics,
            stats: CompilationStats {
                source_lines: source.lines().count(),
                tokens_produced,
                ast_nodes,
                compilation_time_ms: compilation_time,
                output_size: output.len(),
            },
            errors,
        }
    }

    /// Get target format
    fn target_format(&self) -> OutputFormat {
        match self.target_platform {
            TargetPlatform::Wasm => OutputFormat::WasmBinary,
            TargetPlatform::JavaScript => OutputFormat::JavaScript,
            TargetPlatform::TariroBytecode => OutputFormat::TariroBytecode,
            TargetPlatform::Native => OutputFormat::NativeExecutable,
        }
    }

    /// Compile to string output (for debugging)
    pub fn compile_to_string(&mut self, source: &str) -> Result<String, Vec<CompilerError>> {
        let result = self.compile(source);
        if result.success {
            Ok(String::from_utf8_lossy(&result.output).to_string())
        } else {
            Err(result.errors)
        }
    }

    /// Compile TUMC IR to target
    pub fn compile_ir(&mut self, ir: &ast::Program) -> CompilationResult {
        let start = std::time::Instant::now();

        // Type check
        let typed_ast = match type_checker::TypeChecker::new(&mut self.type_context).check(ir) {
            Ok(typed) => typed,
            Err(e) => {
                return CompilationResult {
                    success: false,
                    output: Vec::new(),
                    format: self.target_format(),
                    diagnostics: Vec::new(),
                    stats: CompilationStats::default(),
                    errors: vec![CompilerError::TypeError(e)],
                };
            }
        };

        // Optimize
        let optimized = if self.optimization_level != OptimizationLevel::None {
            optimizer::Optimizer::new(self.optimization_level).optimize(typed_ast)
        } else {
            typed_ast
        };

        // Generate code
        let output = match codegen::CodeGenerator::new(self.target_platform).generate(&optimized) {
            Ok(output) => output,
            Err(e) => {
                return CompilationResult {
                    success: false,
                    output: Vec::new(),
                    format: self.target_format(),
                    diagnostics: Vec::new(),
                    stats: CompilationStats::default(),
                    errors: vec![CompilerError::CodegenError(e)],
                };
            }
        };

        let compilation_time = start.elapsed().as_millis() as u64;

        CompilationResult {
            success: true,
            output: output.clone(),
            format: self.target_format(),
            diagnostics: Vec::new(),
            stats: CompilationStats {
                source_lines: 0,
                tokens_produced: 0,
                ast_nodes: optimized.node_count(),
                compilation_time_ms: compilation_time,
                output_size: output.len(),
            },
            errors: Vec::new(),
        }
    }
}

/// Compiler error types
#[derive(Debug, Clone)]
pub enum CompilerError {
    LexerError(lexer::LexerError),
    ParserError(parser::ParserError),
    TypeError(type_checker::TypeError),
    CodegenError(codegen::CodegenError),
    OptimizerError(optimizer::OptimizerError),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilerError::LexerError(e) => write!(f, "Lexer error: {}", e),
            CompilerError::ParserError(e) => write!(f, "Parser error: {}", e),
            CompilerError::TypeError(e) => write!(f, "Type error: {}", e),
            CompilerError::CodegenError(e) => write!(f, "Code generation error: {}", e),
            CompilerError::OptimizerError(e) => write!(f, "Optimizer error: {}", e),
        }
    }
}

impl std::error::Error for CompilerError {}

// ============================================================================
// Type System
// ============================================================================

/// TUMC Type representation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Integer types
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,

    /// Unsigned integer types
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,

    /// Floating point types
    F32,
    F64,

    /// Boolean
    Bool,

    /// Character
    Char,

    /// String
    String,

    /// Unit (void)
    Unit,

    /// Function type
    Function(Box<FunctionType>),

    /// Tuple type
    Tuple(Vec<Type>),

    /// Array type
    Array(Box<Type>, Option<usize>),

    /// Reference type
    Ref(Box<Type>),

    /// Mutable reference
    MutRef(Box<Type>),

    /// Pointer type
    Ptr(Box<Type>),

    /// Struct type
    Struct(StructType),

    /// Enum type
    Enum(EnumType),

    /// Generic type parameter
    Generic(String),

    /// Type variable (for inference)
    Var(u32),

    /// Unknown type (for error recovery)
    Unknown,
}

/// Function type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Box<Type>,
    pub variadic: bool,
}

/// Struct type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

/// Enum type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumType {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

/// Enum variant
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<Type>,
}

impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}

impl Type {
    /// Get type size in bytes (if known)
    pub fn size(&self) -> Option<usize> {
        match self {
            Type::I8 | Type::U8 | Type::Bool => Some(1),
            Type::I16 | Type::U16 => Some(2),
            Type::I32 | Type::U32 | Type::F32 => Some(4),
            Type::I64 | Type::U64 | Type::F64 => Some(8),
            Type::I128 | Type::U128 => Some(16),
            Type::Isize | Type::Usize => Some(8), // 64-bit
            Type::Char => Some(4),
            Type::String => None, // Pointer + length
            Type::Unit => Some(0),
            Type::Ref(t) | Type::MutRef(t) | Type::Ptr(t) => Some(8), // 64-bit pointer
            Type::Array(t, len) => {
                t.size().and_then(|s| len.map(|l| s * l))
            }
            Type::Tuple(types) => {
                types.iter().filter_map(|t| t.size()).sum::<Option<usize>>()
            }
            Type::Struct(s) => {
                s.fields.iter().filter_map(|(_, t)| t.size()).sum::<Option<usize>>()
            }
            _ => None,
        }
    }

    /// Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::I128
                | Type::Isize
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::U128
                | Type::Usize
                | Type::F32
                | Type::F64
        )
    }

    /// Check if type is integral
    pub fn is_integral(&self) -> bool {
        matches!(
            self,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::I128
                | Type::Isize
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::U128
                | Type::Usize
                | Type::Bool
                | Type::Char
        )
    }

    /// Check if type is floating point
    pub fn is_float(&self) -> bool {
        matches!(self, Type::F32 | Type::F64)
    }

    /// Check if type is pointer-like
    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::Ref(_) | Type::MutRef(_) | Type::Ptr(_))
    }

    /// Get type name as string
    pub fn name(&self) -> String {
        match self {
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::I128 => "i128".to_string(),
            Type::Isize => "isize".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::U128 => "u128".to_string(),
            Type::Usize => "usize".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "string".to_string(),
            Type::Unit => "unit".to_string(),
            Type::Function(f) => {
                let params = f
                    .parameters
                    .iter()
                    .map(|t| t.name())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) -> {}", params, f.return_type.name())
            }
            Type::Tuple(types) => {
                let fields = types.iter().map(|t| t.name()).collect::<Vec<_>>().join(", ");
                format!("({})", fields)
            }
            Type::Array(t, len) => {
                if let Some(l) = len {
                    format!("[{}; {}]", t.name(), l)
                } else {
                    format!("[{}]", t.name())
                }
            }
            Type::Ref(t) => format!("&{}", t.name()),
            Type::MutRef(t) => format!("&mut {}", t.name()),
            Type::Ptr(t) => format!("*{}", t.name()),
            Type::Struct(s) => s.name.clone(),
            Type::Enum(e) => e.name.clone(),
            Type::Generic(name) => name.clone(),
            Type::Var(id) => format!("T{}", id),
            Type::Unknown => "unknown".to_string(),
        }
    }

    /// Unify two types (for type inference)
    pub fn unify(&self, other: &Type) -> Option<Type> {
        if self == other {
            return Some(self.clone());
        }

        match (self, other) {
            // Type variables can unify with anything
            (Type::Var(_), t) => Some(t.clone()),
            (t, Type::Var(_)) => Some(t.clone()),
            // Numeric unification
            (a, b) if a.is_numeric() && b.is_numeric() => Some(Type::I32),
            // Pointer unification
            (Type::Ref(a), Type::Ref(b)) => a.unify(b).map(Type::Ref),
            (Type::MutRef(a), Type::MutRef(b)) => a.unify(b).map(Type::MutRef),
            (Type::Ref(a), Type::MutRef(b)) if *a == *b => Some(Type::Ref(a.clone())),
            (Type::MutRef(a), Type::Ref(b)) if *a == *b => Some(Type::Ref(a.clone())),
            // Function unification
            (
                Type::Function(f1),
                Type::Function(f2),
            ) => {
                if f1.parameters.len() == f2.parameters.len() {
                    let params: Vec<Type> = f1
                        .parameters
                        .iter()
                        .zip(f2.parameters.iter())
                        .filter_map(|(a, b)| a.unify(b))
                        .collect();
                    let ret = f1.return_type.unify(&f2.return_type)?;
                    Some(Type::Function(Box::new(FunctionType {
                        parameters: params,
                        return_type: ret,
                        variadic: f1.variadic || f2.variadic,
                    })))
                } else {
                    None
                }
            }
            // Tuple unification
            (Type::Tuple(a), Type::Tuple(b)) if a.len() == b.len() => {
                let unified: Vec<Type> = a
                    .iter()
                    .zip(b.iter())
                    .filter_map(|(a, b)| a.unify(b))
                    .collect();
                Some(Type::Tuple(unified))
            }
            _ => None,
        }
    }
}

// ============================================================================
// TUMC Intermediate Representation (IR)
// ============================================================================

/// TUMC IR instruction
#[derive(Debug, Clone)]
pub enum IRInstruction {
    /// Load immediate value
    LoadImm { dest: u32, value: i64 },

    /// Load from memory
    Load { dest: u32, base: u32, offset: i32 },

    /// Store to memory
    Store { base: u32, offset: i32, value: u32 },

    /// Arithmetic operations
    Add { dest: u32, lhs: u32, rhs: u32 },
    Sub { dest: u32, lhs: u32, rhs: u32 },
    Mul { dest: u32, lhs: u32, rhs: u32 },
    Div { dest: u32, lhs: u32, rhs: u32 },

    /// Comparison
    Cmp { dest: u32, lhs: u32, rhs: u32, op: CmpOp },

    /// Jump
    Jmp { target: u32 },
    JmpIf { cond: u32, target: u32 },

    /// Function call
    Call { dest: u32, func: u32, args: Vec<u32> },

    /// Return
    Return { value: Option<u32> },

    /// Phi node (for SSA)
    Phi { dest: u32, branches: Vec<(u32, u32)> },
}

/// Comparison operation
#[derive(Debug, Clone, Copy)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_size() {
        assert_eq!(Type::I32.size(), Some(4));
        assert_eq!(Type::F64.size(), Some(8));
        assert_eq!(Type::Bool.size(), Some(1));
    }

    #[test]
    fn test_type_is_numeric() {
        assert!(Type::I32.is_numeric());
        assert!(Type::F64.is_numeric());
        assert!(!Type::Bool.is_numeric());
    }

    #[test]
    fn test_type_unify() {
        assert_eq!(Type::I32.unify(&Type::I32), Some(Type::I32));
        assert!(Type::I32.unify(&Type::Bool).is_some()); // Numeric unification
    }

    #[test]
    fn test_compiler_creation() {
        let compiler = Compiler::new(SourceLanguage::Rust, TargetPlatform::Wasm);
        assert_eq!(compiler.source_language, SourceLanguage::Rust);
        assert_eq!(compiler.target_platform, TargetPlatform::Wasm);
    }
}
