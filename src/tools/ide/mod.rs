//! # Tariro AGI IDE Tool
//!
//! This module integrates Tariro_ide_idpte as a tool for Tariro AGI,
//! enabling the AGI to interact with IDEs for file editing, terminal
//! execution, and LSP-based code intelligence.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                      TARIRO AGI                             │
//! ├─────────────────────────────────────────────────────────────┤
//! │                    IDE Tool Layer                           │
//! │  ┌─────────────────────────────────────────────────────┐  │
//! │  │              IDETool                                 │  │
//! │  │  ┌───────────┐ ┌────────────┐ ┌─────────────────┐  │  │
//! │  │  │  File    │ │  Terminal │ │  LSP Client    │  │  │
//! │  │  │  Manager │ │  Executor  │ │  (Autocomplete │  │  │
//! │  │  │          │ │            │ │   Diagnostics)  │  │  │
//! │  │  └───────────┘ └────────────┘ └─────────────────┘  │  │
//! │  └─────────────────────────────────────────────────────┘  │
//! ├─────────────────────────────────────────────────────────────┤
//! │              Tariro_ide_idpte Bridge                      │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod ide_tool;
pub mod file_manager;
pub mod terminal_executor;
pub mod lsp_client;

pub use ide_tool::{IDETool, IDEToolConfig, IDEToolCapabilities, IDEToolResult};
pub use file_manager::{FileManager, FileOperation, FileResult};
pub use terminal_executor::{TerminalExecutor, CommandResult};
pub use lsp_client::{LSPClient, LSPRequest, LSPResponse};

use serde::{Deserialize, Serialize};

/// IDE Tool action types that the AGI can invoke
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IDEAction {
    /// Read file contents
    ReadFile { path: String },
    /// Write file contents
    WriteFile { path: String, content: String },
    /// List directory contents
    ListDir { path: String },
    /// Create directory
    CreateDir { path: String },
    /// Delete file
    DeleteFile { path: String },
    /// Execute terminal command
    ExecuteCommand { command: String, cwd: Option<String> },
    /// LSP: Get completions
    LSPCompletions { path: String, position: (u32, u32) },
    /// LSP: Get diagnostics
    LSPDiagnostics { path: String },
    /// LSP: Go to definition
    LSPGoToDefinition { path: String, position: (u32, u32) },
    /// LSP: Find references
    LSPFindReferences { path: String, position: (u32, u32) },
}

/// Result from an IDE tool action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEToolResponse {
    /// Whether the action succeeded
    pub success: bool,
    /// Action type that was performed
    pub action: IDEAction,
    /// Result data (varies by action type)
    pub data: serde_json::Value,
    /// Error message if failed
    pub error: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl IDEToolResponse {
    /// Create a successful response
    pub fn success(action: IDEAction, data: serde_json::Value, time_ms: u64) -> Self {
        Self {
            success: true,
            action,
            data,
            error: None,
            execution_time_ms: time_ms,
        }
    }

    /// Create a failed response
    pub fn failure(action: IDEAction, error: String, time_ms: u64) -> Self {
        Self {
            success: false,
            action,
            data: serde_json::Value::Null,
            error: Some(error),
            execution_time_ms: time_ms,
        }
    }
}
