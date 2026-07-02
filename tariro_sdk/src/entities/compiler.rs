//! Compiler project entity for meta-compilation

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Programming language for compilation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    C,
    Cpp,
    Java,
    Custom(String),
}

/// Compilation target platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetPlatform {
    Linux,
    Windows,
    MacOS,
    WebAssembly,
    Embedded,
    Custom(String),
}

/// Compilation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompilationStatus {
    Pending,
    Compiling,
    Successful,
    Failed,
    Optimized,
}

impl Default for CompilationStatus {
    fn default() -> Self {
        CompilationStatus::Pending
    }
}

/// A compiler project for code generation and compilation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerProject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source_code: String,
    pub language: Language,
    pub target_platform: TargetPlatform,
    pub status: CompilationStatus,
    pub created_at: DateTime<Utc>,
    pub compiled_at: Option<DateTime<Utc>>,
    pub output_binary: Option<Vec<u8>>,
    pub compilation_logs: Vec<CompilationLog>,
    pub optimization_level: OptimizationLevel,
}

impl CompilerProject {
    /// Create a new compiler project
    pub fn new(name: String, description: String, source_code: String, language: Language) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            source_code,
            language,
            target_platform: TargetPlatform::Linux,
            status: CompilationStatus::Pending,
            created_at: Utc::now(),
            compiled_at: None,
            output_binary: None,
            compilation_logs: Vec::new(),
            optimization_level: OptimizationLevel::Default,
        }
    }
}

/// Optimization level for compilation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    Default,
    Size,
    Speed,
    Aggressive,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Default
    }
}

/// Compilation log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationLog {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source_location: Option<String>,
}

/// Log levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}