// Language Server Protocol (LSP) Implementation

use crate::tumc_compiler::ir::{Module, Function, BasicBlock};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LSP server for IDE integration
pub struct LspServer {
    port: u16,
    connections: Vec<LspConnection>,
}

#[derive(Debug, Clone)]
pub struct LspConnection {
    pub id: u64,
    pub capabilities: ClientCapabilities,
}

#[derive(Debug, Clone, Default)]
pub struct ClientCapabilities {
    pub text_document: TextDocumentClientCapabilities,
    pub workspace: WorkspaceClientCapabilities,
}

#[derive(Debug, Clone, Default)]
pub struct TextDocumentClientCapabilities {
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,
    pub hover: Option<HoverClientCapabilities>,
    pub completion: Option<CompletionClientCapabilities>,
    pub definition: Option<DefinitionClientCapabilities>,
    pub references: Option<ReferencesClientCapabilities>,
    pub diagnostics: Option<DiagnosticsClientCapabilities>,
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceClientCapabilities {
    pub workspace_folders: bool,
}

#[derive(Debug, Clone, Default)]
pub struct TextDocumentSyncClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Default)]
pub struct HoverClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CompletionClientCapabilities {
    pub dynamic_registration: bool,
    pub completion_item: Option<CompletionItemCapabilities>,
}

#[derive(Debug, Clone, Default)]
pub struct CompletionItemCapabilities {
    pub snippet_support: bool,
    pub resolve_support: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DefinitionClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ReferencesClientCapabilities {
    pub dynamic_registration: bool,
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticsClientCapabilities {
    pub dynamic_registration: bool,
}

/// Server capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(rename = "textDocumentSync")]
    pub text_document_sync: Option<TextDocumentSyncOptions>,
    pub hover_provider: Option<bool>,
    pub completion_provider: Option<CompletionOptions>,
    #[serde(rename = "definitionProvider")]
    pub definition_provider: Option<bool>,
    #[serde(rename = "referencesProvider")]
    pub references_provider: Option<bool>,
    #[serde(rename = "diagnosticProvider")]
    pub diagnostic_provider: Option<DiagnosticOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentSyncOptions {
    pub open_close: bool,
    pub change: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionOptions {
    pub resolve_provider: Option<bool>,
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticOptions {
    pub inter_file_dependencies: bool,
    pub workspace_diagnostics: bool,
}

/// LSP request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub process_id: Option<u64>,
    pub root_uri: Option<String>,
    pub capabilities: ClientCapabilities,
    #[serde(rename = "initializationOptions")]
    pub initialization_options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: Option<String>,
}

/// Text document position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentPositionParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

/// Hover response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverResponse {
    pub contents: HoverContents,
    pub range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkupContent {
    pub kind: String,
    pub value: String,
}

/// Range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    #[serde(rename = "isIncomplete")]
    pub is_incomplete: bool,
    pub items: Vec<CompletionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: Option<i32>,
    pub detail: Option<String>,
    pub documentation: Option<MarkupContent>,
    pub insert_text: Option<String>,
}

/// Goto definition response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionResponse {
    pub uri: String,
    pub range: Range,
}

/// Find references response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferencesResponse {
    pub references: Vec<ReferenceItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceItem {
    pub uri: String,
    pub range: Range,
    pub context: String,
}

/// Diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishDiagnosticsParams {
    pub uri: String,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<i32>,
    pub code: Option<DiagnosticCode>,
    pub source: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticCode {
    Integer(i32),
    String(String),
}

impl LspServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            connections: Vec::new(),
        }
    }

    pub fn get_capabilities(&self) -> ServerCapabilities {
        ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncOptions {
                open_close: true,
                change: 1, // Full sync
            }),
            hover_provider: Some(true),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(vec![
                    ".".to_string(),
                    ":".to_string(),
                    "(".to_string(),
                ]),
            }),
            definition_provider: Some(true),
            references_provider: Some(true),
            diagnostic_provider: Some(DiagnosticOptions {
                inter_file_dependencies: true,
                workspace_diagnostics: true,
            }),
        }
    }

    /// Get hover information for a position
    pub fn get_hover(&self, module: &Module, uri: &str, position: Position) -> Option<HoverResponse> {
        // Find symbol at position
        for func in &module.functions {
            if let Some(type_info) = self.get_type_at_position(func, &position) {
                return Some(HoverResponse {
                    contents: HoverContents::MarkupContent(MarkupContent {
                        kind: "markdown".to_string(),
                        value: format!("```\n{}\n```\n\n{}", type_info.0, type_info.1),
                    }),
                    range: None,
                });
            }
        }
        None
    }

    /// Get completions at a position
    pub fn get_completions(&self, module: &Module, _uri: &str, _position: Position) -> CompletionResponse {
        let mut items = Vec::new();

        // Add function names
        for func in &module.functions {
            items.push(CompletionItem {
                label: func.name.clone(),
                kind: Some(3), // Function
                detail: Some(format!("fn {}", func.name)),
                documentation: None,
                insert_text: None,
            });
        }

        // Add type keywords
        items.push(CompletionItem {
            label: "i32".to_string(),
            kind: Some(9), // Keyword
            detail: Some("32-bit integer".to_string()),
            documentation: None,
            insert_text: None,
        });
        items.push(CompletionItem {
            label: "i64".to_string(),
            kind: Some(9),
            detail: Some("64-bit integer".to_string()),
            documentation: None,
            insert_text: None,
        });
        items.push(CompletionItem {
            label: "f32".to_string(),
            kind: Some(9),
            detail: Some("32-bit float".to_string()),
            documentation: None,
            insert_text: None,
        });
        items.push(CompletionItem {
            label: "f64".to_string(),
            kind: Some(9),
            detail: Some("64-bit float".to_string()),
            documentation: None,
            insert_text: None,
        });
        items.push(CompletionItem {
            label: "bool".to_string(),
            kind: Some(9),
            detail: Some("boolean".to_string()),
            documentation: None,
            insert_text: None,
        });

        CompletionResponse {
            is_incomplete: false,
            items,
        }
    }

    /// Get definition at a position
    pub fn get_definition(&self, module: &Module, uri: &str, position: Position) -> Option<DefinitionResponse> {
        for func in &module.functions {
            let range = Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: func.name.len() as u32 },
            };

            if position.character >= range.start.character &&
               position.character <= range.end.character {
                return Some(DefinitionResponse {
                    uri: uri.to_string(),
                    range,
                });
            }
        }
        None
    }

    /// Find all references to a symbol
    pub fn find_references(&self, module: &Module, uri: &str, position: Position) -> ReferencesResponse {
        let mut references = Vec::new();

        for (line, block) in module.functions.iter().enumerate() {
            for inst in &block.body.iter().flat_map(|b| b.instructions.iter()) {
                references.push(ReferenceItem {
                    uri: uri.to_string(),
                    range: Range {
                        start: Position { line: line as u32, character: 0 },
                        end: Position { line: line as u32, character: 20 },
                    },
                    context: format!("{:?}", inst.op),
                });
            }
        }

        ReferencesResponse { references }
    }

    /// Publish diagnostics for a document
    pub fn publish_diagnostics(&self, module: &Module, uri: &str) -> PublishDiagnosticsParams {
        let mut diagnostics = Vec::new();

        // Check for undefined functions
        let mut defined_functions: HashMap<String, bool> = HashMap::new();
        for func in &module.functions {
            defined_functions.insert(func.name.clone(), true);
        }

        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    if let crate::tumc_compiler::ir::Operation::Call(func_id, _) = &inst.op {
                        if !defined_functions.contains_key(&format!("func_{}", func_id.0)) {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position { line: 0, character: 0 },
                                    end: Position { line: 0, character: 10 },
                                },
                                severity: Some(1), // Error
                                code: Some(DiagnosticCode::Integer(2304)),
                                source: Some("TUMC".to_string()),
                                message: format!("Undefined function: func_{}", func_id.0),
                            });
                        }
                    }
                }
            }
        }

        PublishDiagnosticsParams {
            uri: uri.to_string(),
            diagnostics,
        }
    }

    fn get_type_at_position(&self, func: &Function, position: &Position) -> Option<(String, String)> {
        let _ = (func, position);
        Some(("i32".to_string(), "32-bit signed integer".to_string()))
    }
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new(9507)
    }
}
