//! # LSP Client
//!
//! Language Server Protocol client for IDE intelligence features.

use std::process::{Command, Stdio, ChildStdin, ChildStdout};
use std::io::{Write, BufRead, BufReader};
use serde::{Deserialize, Serialize};

/// LSP completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: Option<String>,
    pub detail: Option<String>,
    pub documentation: Option<String>,
}

/// LSP diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: DiagnosticRange,
    pub severity: String,
    pub message: String,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRange {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

/// LSP location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: DiagnosticRange,
}

/// LSP reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub uri: String,
    pub range: DiagnosticRange,
    pub name: String,
}

/// LSP request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LSPRequest {
    Initialize { root_uri: String },
    TextDocumentDidOpen { uri: String, text: String },
    TextDocumentDidChange { uri: String, text: String },
    TextDocumentCompletion { uri: String, position: (u32, u32) },
    TextDocumentDiagnostics { uri: String },
    TextDocumentDefinition { uri: String, position: (u32, u32) },
    TextDocumentReferences { uri: String, position: (u32, u32) },
}

/// LSP response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// LSP Client for communicating with language servers
pub struct LSPClient {
    command: Vec<String>,
    port: Option<u16>,
    process: Option<std::process::Child>,
}

impl LSPClient {
    /// Create a new LSP client
    pub fn new(command: &[String], port: Option<u16>) -> Self {
        Self {
            command: command.to_vec(),
            port,
            process: None,
        }
    }

    /// Start the LSP server
    pub fn start(&mut self) -> Result<(), String> {
        if self.command.is_empty() {
            return Err("No LSP command specified".to_string());
        }

        let mut cmd = Command::new(&self.command[0]);
        if self.command.len() > 1 {
            cmd.args(&self.command[1..]);
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = cmd.spawn()
            .map_err(|e| format!("Failed to start LSP server: {}", e))?;

        self.process = Some(child);
        Ok(())
    }

    /// Stop the LSP server
    pub fn stop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }
    }

    /// Get completions at a position
    pub fn get_completions(&mut self, path: &str, position: (u32, u32)) -> Result<Vec<CompletionItem>, String> {
        // Simulated completions based on path extension
        let items = match path.rsplit('.').next().unwrap_or("") {
            "rs" => vec![
                CompletionItem {
                    label: "fn".to_string(),
                    kind: Some("keyword".to_string()),
                    detail: Some("Function declaration".to_string()),
                    documentation: Some("Define a new function".to_string()),
                },
                CompletionItem {
                    label: "let".to_string(),
                    kind: Some("keyword".to_string()),
                    detail: Some("Variable binding".to_string()),
                    documentation: Some("Bind a value to a variable".to_string()),
                },
                CompletionItem {
                    label: "impl".to_string(),
                    kind: Some("keyword".to_string()),
                    detail: Some("Implementation block".to_string()),
                    documentation: Some("Define implementations for types".to_string()),
                },
            ],
            "ts" | "tsx" | "js" | "jsx" => vec![
                CompletionItem {
                    label: "const".to_string(),
                    kind: Some("keyword".to_string()),
                    detail: Some("Constant declaration".to_string()),
                    documentation: Some("Declare a constant value".to_string()),
                },
                CompletionItem {
                    label: "function".to_string(),
                    kind: Some("keyword".to_string()),
                    detail: Some("Function declaration".to_string()),
                    documentation: Some("Define a function".to_string()),
                },
            ],
            "py" => vec![
                CompletionItem {
                    label: "def".to_string(),
                    kind: Some("keyword".to_string()),
                    detail: Some("Function definition".to_string()),
                    documentation: Some("Define a function".to_string()),
                },
                CompletionItem {
                    label: "class".to_string(),
                    kind: Some("keyword".to_string()),
                    detail: Some("Class definition".to_string()),
                    documentation: Some("Define a class".to_string()),
                },
            ],
            _ => vec![
                CompletionItem {
                    label: "TODO".to_string(),
                    kind: Some("text".to_string()),
                    detail: Some("Todo comment".to_string()),
                    documentation: None,
                },
            ],
        };

        Ok(items)
    }

    /// Get diagnostics for a file
    pub fn get_diagnostics(&mut self, path: &str) -> Result<Vec<Diagnostic>, String> {
        // In a real implementation, this would communicate with the LSP server
        // For now, return an empty list as we don't have an active server
        Ok(Vec::new())
    }

    /// Go to definition
    pub fn goto_definition(&mut self, path: &str, position: (u32, u32)) -> Result<Option<Location>, String> {
        // Simulated goto definition
        Ok(Some(Location {
            uri: format!("file://{}", path),
            range: DiagnosticRange {
                start_line: position.0,
                start_column: position.1,
                end_line: position.0,
                end_column: position.1 + 10,
            },
        }))
    }

    /// Find references
    pub fn find_references(&mut self, path: &str, position: (u32, u32)) -> Result<Vec<Reference>, String> {
        // Simulated references
        Ok(vec![
            Reference {
                uri: format!("file://{}", path),
                range: DiagnosticRange {
                    start_line: position.0 + 1,
                    start_column: 0,
                    end_line: position.0 + 1,
                    end_column: 20,
                },
                name: "reference".to_string(),
            },
        ])
    }

    /// Send a request to the LSP server
    fn send_request(&mut self, request: LSPRequest) -> Result<String, String> {
        let json = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        if let Some(ref mut child) = self.process {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(json.as_bytes())
                    .map_err(|e| format!("Failed to write to LSP server: {}", e))?;
            }
        }

        Ok(json)
    }
}

impl Drop for LSPClient {
    fn drop(&mut self) {
        self.stop();
    }
}
