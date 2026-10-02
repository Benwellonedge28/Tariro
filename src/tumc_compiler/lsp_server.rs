//! TUMC Language Server Protocol (LSP) Implementation
//! Provides IDE features: autocomplete, go-to-definition, diagnostics, etc.

use crate::prelude::*;
use super::ast::*;
use super::lexer::{Lexer, Token, TokenKind};
use super::parser::Parser;
use super::type_checker::TypeChecker;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// LSP Server instance
pub struct LspServer {
    /// Document store
    documents: HashMap<String, Document>,
    /// Compilation cache
    cache: HashMap<String, CompilationCache>,
    /// Server capabilities
    capabilities: ServerCapabilities,
    /// Logger
    logger: LspLogger,
}

impl LspServer {
    /// Create new LSP server
    pub fn new() -> Self {
        LspServer {
            documents: HashMap::new(),
            cache: HashMap::new(),
            capabilities: ServerCapabilities::default(),
            logger: LspLogger::new(),
        }
    }

    /// Handle incoming request
    pub fn handle_request(&mut self, method: &str, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        self.logger.log_request(method);

        match method {
            "initialize" => self.handle_initialize(params),
            "textDocument/didOpen" => self.handle_did_open(params),
            "textDocument/didChange" => self.handle_did_change(params),
            "textDocument/didSave" => self.handle_did_save(params),
            "textDocument/didClose" => self.handle_did_close(params),
            "textDocument/completion" => self.handle_completion(params),
            "textDocument/hover" => self.handle_hover(params),
            "textDocument/definition" => self.handle_definition(params),
            "textDocument/references" => self.handle_references(params),
            "textDocument/documentSymbol" => self.handle_document_symbol(params),
            "textDocument/diagnostics" => self.handle_diagnostics(params),
            "textDocument/formatting" => self.handle_formatting(params),
            "textDocument/codeAction" => self.handle_code_action(params),
            "shutdown" => self.handle_shutdown(),
            _ => Err(LspError::MethodNotFound(method.to_string())),
        }
    }

    /// Handle initialize request
    fn handle_initialize(&self, _params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let result = json!({
            "capabilities": self.capabilities,
            "serverInfo": {
                "name": "TUMC Language Server",
                "version": "0.1.0"
            }
        });
        Ok(result)
    }

    /// Handle text document open
    fn handle_did_open(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: DidOpenTextDocumentParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();
        let text = params.text_document.text;

        // Store document
        let doc = Document {
            uri: uri.clone(),
            text: text.clone(),
            version: params.text_document.version,
            language_id: "tumc".to_string(),
        };
        self.documents.insert(uri.clone(), doc);

        // Compile and cache
        self.compile_and_cache(&uri, &text);

        Ok(serde_json::Value::Null)
    }

    /// Handle text document change
    fn handle_did_change(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: DidChangeTextDocumentParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();

        if let Some(doc) = self.documents.get_mut(&uri) {
            // Apply changes
            for change in params.content_changes {
                if let Some(range) = change.range {
                    let (start, end) = self.offset_to_position(&doc.text, &range);
                    doc.text = format!(
                        "{}{}{}",
                        &doc.text[..start],
                        change.text,
                        &doc.text[end..]
                    );
                } else {
                    doc.text = change.text;
                }
            }
            doc.version = params.text_document.version;

            // Recompile
            self.compile_and_cache(&uri, &doc.text);
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle text document save
    fn handle_did_save(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        // Could trigger file save diagnostics here
        Ok(serde_json::Value::Null)
    }

    /// Handle text document close
    fn handle_did_close(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: DidCloseTextDocumentParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        self.documents.remove(&params.text_document.uri);
        self.cache.remove(&params.text_document.uri);

        Ok(serde_json::Value::Null)
    }

    /// Handle completion request
    fn handle_completion(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: CompletionParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();
        let position = params.position;

        if let Some(cache) = self.cache.get(&uri) {
            let completions = self.get_completions(&cache.ast, position);
            return Ok(serde_json::to_value(completions).unwrap());
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle hover request
    fn handle_hover(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: HoverParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();
        let position = params.position;

        if let Some(cache) = self.cache.get(&uri) {
            if let Some(hover) = self.get_hover_info(&cache.ast, position) {
                return Ok(serde_json::to_value(hover).unwrap());
            }
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle go-to-definition request
    fn handle_definition(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: DefinitionParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();
        let position = params.position;

        if let Some(cache) = self.cache.get(&uri) {
            if let Some(locations) = self.get_definition_locations(&cache.ast, position) {
                return Ok(serde_json::to_value(locations).unwrap());
            }
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle find-references request
    fn handle_references(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: ReferenceParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();
        let position = params.position;

        if let Some(cache) = self.cache.get(&uri) {
            let references = self.find_references(&cache.ast, position);
            return Ok(serde_json::to_value(references).unwrap());
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle document symbols request
    fn handle_document_symbol(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: DocumentSymbolParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();

        if let Some(cache) = self.cache.get(&uri) {
            let symbols = self.extract_document_symbols(&cache.ast);
            return Ok(serde_json::to_value(symbols).unwrap());
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle diagnostics request
    fn handle_diagnostics(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: DiagnosticsParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();

        if let Some(cache) = self.cache.get(&uri) {
            let diagnostics = &cache.diagnostics;
            return Ok(serde_json::to_value(diagnostics).unwrap());
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle formatting request
    fn handle_formatting(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: FormattingParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();

        if let Some(doc) = self.documents.get(&uri) {
            let formatted = self.format_code(&doc.text, &params.options);
            let edits = vec![TextEdit {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position {
                        line: doc.text.lines().count() as u32,
                        character: 0,
                    },
                },
                new_text: formatted,
            }];
            return Ok(serde_json::to_value(edits).unwrap());
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle code action request
    fn handle_code_action(&mut self, params: serde_json::Value) -> Result<serde_json::Value, LspError> {
        let params: CodeActionParams = serde_json::from_value(params)
            .map_err(|e| LspError::InvalidParams(e.to_string()))?;

        let uri = params.text_document.uri.clone();

        if let Some(cache) = self.cache.get(&uri) {
            let actions = self.get_code_actions(&cache.diagnostics, &params.range);
            return Ok(serde_json::to_value(actions).unwrap());
        }

        Ok(serde_json::Value::Null)
    }

    /// Handle shutdown request
    fn handle_shutdown(&self) -> Result<serde_json::Value, LspError> {
        Ok(serde_json::Value::Null)
    }

    // ==================== Helper Methods ====================

    /// Compile document and cache results
    fn compile_and_cache(&mut self, uri: &str, text: &str) {
        let mut lexer = Lexer::new(text);
        let tokens: Vec<Token> = lexer.tokenize().unwrap_or_default();

        let mut parser = Parser::new(text, tokens);
        let ast = parser.parse_program().unwrap_or_else(|_| Program::default());

        let mut checker = TypeChecker::new();
        let _ = checker.check_program(&ast);

        let diagnostics = checker.errors.iter().map(|e| Diagnostic {
            range: Range::default(),
            severity: Some(DiagnosticSeverity::Error),
            code: Some(NumberOrString::Number(1)),
            source: Some("TUMC".to_string()),
            message: e.to_string(),
            related_information: None,
        }).collect();

        self.cache.insert(uri.to_string(), CompilationCache {
            ast,
            tokens,
            diagnostics,
            symbols: HashMap::new(),
        });
    }

    /// Get completions at position
    fn get_completions(&self, ast: &Program, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Add keywords
        for keyword in &["fn", "let", "if", "else", "while", "for", "struct", "enum", "trait", "impl", "return", "pub", "mod", "use", "const", "static", "type", "match", "loop", "break", "continue"] {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::Keyword),
                detail: Some(format!("Keyword: {}", keyword)),
                ..Default::default()
            });
        }

        // Add types
        for type_name in &["i32", "i64", "f32", "f64", "bool", "char", "String", "Vec", "Option", "Result", "Box", "Rc", "Arc"] {
            completions.push(CompletionItem {
                label: type_name.to_string(),
                kind: Some(CompletionItemKind::Class),
                detail: Some(format!("Type: {}", type_name)),
                ..Default::default()
            });
        }

        // Add functions from AST
        for item in &ast.items {
            if let Item::Function(func) = item {
                let params: Vec<String> = func.params.iter().map(|p| p.name.clone()).collect();
                completions.push(CompletionItem {
                    label: func.name.clone(),
                    kind: Some(CompletionItemKind::Function),
                    detail: Some(format!("fn {}({})", func.name, params.join(", "))),
                    insert_text: Some(format!("{}(${{1:}})", func.name)),
                    ..Default::default()
                });
            }
        }

        completions
    }

    /// Get hover information at position
    fn get_hover_info(&self, ast: &Program, position: Position) -> Option<Hover> {
        // Simplified: return info for first function
        for item in &ast.items {
            if let Item::Function(func) = item {
                return Some(Hover {
                    contents: HoverContents::Scalar(MarkedString::String(format!(
                        "Function: {}",
                        func.name
                    ))),
                    range: None,
                });
            }
        }
        None
    }

    /// Get definition locations
    fn get_definition_locations(&self, ast: &Program, position: Position) -> Option<Vec<Location>> {
        let mut locations = Vec::new();

        for item in &ast.items {
            if let Item::Function(func) = item {
                locations.push(Location {
                    uri: "file:///.tumc".to_string(),
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 10 },
                    },
                });
            }
        }

        if locations.is_empty() {
            None
        } else {
            Some(locations)
        }
    }

    /// Find all references
    fn find_references(&self, ast: &Program, position: Position) -> Vec<Location> {
        let mut references = Vec::new();

        for item in &ast.items {
            if let Item::Function(func) = item {
                references.push(Location {
                    uri: "file:///.tumc".to_string(),
                    range: Range::default(),
                });
            }
        }

        references
    }

    /// Extract document symbols
    fn extract_document_symbols(&self, ast: &Program) -> Vec<DocumentSymbol> {
        let mut symbols = Vec::new();

        for item in &ast.items {
            match item {
                Item::Function(func) => {
                    symbols.push(DocumentSymbol {
                        name: func.name.clone(),
                        detail: Some(format!("({})", func.params.len())),
                        kind: SymbolKind::Function,
                        range: Range::default(),
                        selection_range: Range::default(),
                        children: Some(vec![]),
                        tags: None,
                        deprecated: None,
                    });
                }
                Item::Struct(def) => {
                    symbols.push(DocumentSymbol {
                        name: def.name.clone(),
                        detail: Some("struct".to_string()),
                        kind: SymbolKind::Class,
                        range: Range::default(),
                        selection_range: Range::default(),
                        children: Some(vec![]),
                        tags: None,
                        deprecated: None,
                    });
                }
                Item::Enum(def) => {
                    symbols.push(DocumentSymbol {
                        name: def.name.clone(),
                        detail: Some("enum".to_string()),
                        kind: SymbolKind::Enum,
                        range: Range::default(),
                        selection_range: Range::default(),
                        children: Some(vec![]),
                        tags: None,
                        deprecated: None,
                    });
                }
                _ => {}
            }
        }

        symbols
    }

    /// Get code actions for diagnostics
    fn get_code_actions(&self, diagnostics: &[Diagnostic], _range: &Range) -> Vec<CodeAction> {
        let mut actions = Vec::new();

        for diag in diagnostics {
            if diag.message.contains("undefined") {
                actions.push(CodeAction {
                    title: "Create definition".to_string(),
                    kind: Some(CodeActionKind::QuickFix),
                    diagnostics: Some(vec![diag.clone()]),
                    edit: Some(WorkspaceEdit {
                        changes: Some(HashMap::new()),
                        document_changes: None,
                    }),
                    ..Default::default()
                });
            }
        }

        actions
    }

    /// Format code
    fn format_code(&self, text: &str, _options: &FormattingOptions) -> String {
        // Simple formatting: add proper indentation
        let mut result = String::new();
        let mut indent_level = 0;
        let mut in_string = false;

        for ch in text.chars() {
            match ch {
                '{' if !in_string => {
                    result.push(ch);
                    indent_level += 1;
                    result.push('\n');
                    result.push_str(&"    ".repeat(indent_level));
                }
                '}' if !in_string => {
                    indent_level = indent_level.saturating_sub(1);
                    result.push_str(&"    ".repeat(indent_level));
                    result.push(ch);
                    result.push('\n');
                }
                '\n' => {
                    result.push('\n');
                    result.push_str(&"    ".repeat(indent_level));
                }
                '"' => {
                    in_string = !in_string;
                    result.push(ch);
                }
                _ => result.push(ch),
            }
        }

        result
    }

    /// Convert offset to position
    fn offset_to_position(&self, text: &str, range: &Range) -> (usize, usize) {
        let mut lines = text.lines();
        let mut start = 0;
        let mut end = 0;

        for (i, line) in lines.enumerate() {
            if i == range.start.line as usize {
                start = lines.as_str().find(line).unwrap_or(0) + range.start.character as usize;
            }
            if i == range.end.line as usize {
                end = lines.as_str().find(line).unwrap_or(0) + range.end.character as usize;
                break;
            }
        }

        (start, end)
    }
}

// ==================== LSP Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    #[serde(rename = "textDocumentSync")]
    pub text_document_sync: Option<TextDocumentSyncOptions>,
    pub completion: Option<CompletionOptions>,
    pub hover: Option<HoverOptions>,
    #[serde(rename = "definitionProvider")]
    pub definition_provider: Option<bool>,
    #[serde(rename = "referencesProvider")]
    pub references_provider: Option<bool>,
    #[serde(rename = "documentSymbolProvider")]
    pub document_symbol_provider: Option<bool>,
    #[serde(rename = "codeActionProvider")]
    pub code_action_provider: Option<bool>,
    pub formatting: Option<bool>,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        ServerCapabilities {
            text_document_sync: Some(TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::Full),
                will_save: None,
                will_save_wait_until: None,
                save: None,
            }),
            completion: Some(CompletionOptions {
                resolve_provider: Some(false),
                trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                work_done_progress: None,
            }),
            hover: Some(HoverOptions { work_done_progress: None }),
            definition_provider: Some(true),
            references_provider: Some(true),
            document_symbol_provider: Some(true),
            code_action_provider: Some(true),
            formatting: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentSyncOptions {
    pub open_close: Option<bool>,
    pub change: Option<TextDocumentSyncKind>,
    #[serde(rename = "willSave")]
    pub will_save: Option<bool>,
    #[serde(rename = "willSaveWaitUntil")]
    pub will_save_wait_until: Option<bool>,
    pub save: Option<SaveOptions>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextDocumentSyncKind {
    None,
    Full,
    Incremental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveOptions {
    pub include_text: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionOptions {
    #[serde(rename = "resolveProvider")]
    pub resolve_provider: Option<bool>,
    #[serde(rename = "triggerCharacters")]
    pub trigger_characters: Option<Vec<String>>,
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverOptions {
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub uri: String,
    pub text: String,
    pub version: i32,
    #[serde(rename = "languageId")]
    pub language_id: String,
}

#[derive(Debug, Clone)]
pub struct CompilationCache {
    pub ast: Program,
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
    pub symbols: HashMap<String, SymbolInfo>,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub type_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: Option<DiagnosticSeverity>,
    pub code: Option<NumberOrString>,
    pub source: Option<String>,
    pub message: String,
    #[serde(rename = "relatedInformation")]
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(i32),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRelatedInformation {
    pub location: Location,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Default for Range {
    fn default() -> Self {
        Range {
            start: Position { line: 0, character: 0 },
            end: Position { line: 0, character: 0 },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    #[serde(rename = "labelDetails")]
    pub label_details: Option<CompletionItemLabelDetails>,
    pub kind: Option<CompletionItemKind>,
    pub detail: Option<String>,
    #[serde(rename = "insertText")]
    pub insert_text: Option<String>,
    #[serde(rename = "insertTextFormat")]
    pub insert_text_format: Option<InsertTextFormat>,
    pub text_edit: Option<TextEdit>,
    pub documentation: Option<Documentation>,
}

impl Default for CompletionItem {
    fn default() -> Self {
        CompletionItem {
            label: String::new(),
            label_details: None,
            kind: None,
            detail: None,
            insert_text: None,
            insert_text_format: None,
            text_edit: None,
            documentation: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItemLabelDetails {
    pub detail: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InsertTextFormat {
    PlainText = 1,
    Snippet = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    pub range: Range,
    #[serde(rename = "newText")]
    pub new_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Documentation {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkupContent {
    pub kind: MarkupKind,
    pub value: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkupKind {
    PlainText,
    Markdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hover {
    pub contents: HoverContents,
    pub range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    Scalar(MarkedString),
    Array(Vec<MarkedString>),
    MarkupContent(MarkupContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkedString {
    String(String),
    LanguageString(LanguageString),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageString {
    pub language: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub context: ReferenceContext,
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceContext {
    pub include_declaration: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSymbolParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub range: Range,
    #[serde(rename = "selectionRange")]
    pub selection_range: Range,
    pub children: Option<Vec<DocumentSymbol>>,
    pub tags: Option<Vec<SymbolTag>>,
    pub deprecated: Option<bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SymbolTag {
    Deprecated = 1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub options: FormattingOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingOptions {
    #[serde(rename = "tabSize")]
    pub tab_size: u32,
    #[serde(rename = "insertSpaces")]
    pub insert_spaces: bool,
    #[serde(rename = "trimTrailingWhitespace")]
    pub trim_trailing_whitespace: Option<bool>,
    pub insert_final_newline: Option<bool>,
    #[serde(rename = "trimFinalNewlines")]
    pub trim_final_newlines: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub range: Range,
    pub context: CodeActionContext,
    #[serde(rename = "workDoneProgress")]
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionContext {
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAction {
    pub title: String,
    pub kind: Option<CodeActionKind>,
    pub diagnostics: Option<Vec<Diagnostic>>,
    pub edit: Option<WorkspaceEdit>,
    pub command: Option<Command>,
}

impl Default for CodeAction {
    fn default() -> Self {
        CodeAction {
            title: String::new(),
            kind: None,
            diagnostics: None,
            edit: None,
            command: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CodeActionKind {
    QuickFix,
    Refactor,
    RefactorExtract,
    RefactorInline,
    RefactorRewrite,
    Source,
    SourceOrganizeImports,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceEdit {
    pub changes: Option<HashMap<String, Vec<TextEdit>>>,
    #[serde(rename = "documentChanges")]
    pub document_changes: Option<DocumentChanges>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentChanges {
    TextEdits(Vec<TextEdit>),
    Creates(CreateFile),
    Renames(RenameFile),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFile {
    pub uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameFile {
    pub old_uri: String,
    pub new_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidOpenTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentItem {
    pub uri: String,
    #[serde(rename = "languageId")]
    pub language_id: String,
    pub version: i32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidChangeTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: VersionedTextDocumentIdentifier,
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentContentChangeEvent {
    pub range: Option<Range>,
    #[serde(rename = "rangeLength")]
    pub range_length: Option<u32>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidSaveTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidCloseTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: TextDocumentIdentifier,
}

// ==================== LSP Error ====================

#[derive(Debug, Clone)]
pub struct LspError {
    pub code: i32,
    pub message: String,
}

impl LspError {
    pub fn method_not_found(method: String) -> Self {
        LspError {
            code: -32601,
            message: format!("Method not found: {}", method),
        }
    }

    pub fn invalid_params(msg: String) -> Self {
        LspError {
            code: -32602,
            message: format!("Invalid params: {}", msg),
        }
    }

    pub fn internal_error(msg: String) -> Self {
        LspError {
            code: -32603,
            message: format!("Internal error: {}", msg),
        }
    }
}

impl std::fmt::Display for LspError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " LSP Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for LspError {}

// ==================== Logger ====================

struct LspLogger {
    requests: usize,
    errors: usize,
}

impl LspLogger {
    fn new() -> Self {
        LspLogger { requests: 0, errors: 0 }
    }

    fn log_request(&mut self, method: &str) {
        self.requests += 1;
        println!("LSP: Request #{} - {}", self.requests, method);
    }

    fn log_error(&mut self, error: &str) {
        self.errors += 1;
        eprintln!("LSP Error: {}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_server_creation() {
        let server = LspServer::new();
        assert!(server.documents.is_empty());
        assert!(server.cache.is_empty());
    }

    #[test]
    fn test_completion_items() {
        let mut server = LspServer::new();
        let completions = server.get_completions(&Program::default(), Position { line: 0, character: 0 });

        // Should include keywords
        let labels: Vec<&str> = completions.iter().filter_map(|c| {
            c.kind.map(|k| k as i32 == 14).and_then(|is_kw| {
                if is_kw { Some(c.label.as_str()) } else { None }
            })
        }).collect();

        assert!(labels.contains(&"fn"));
        assert!(labels.contains(&"let"));
    }

    #[test]
    fn test_document_symbols() {
        let server = LspServer::new();
        let symbols = server.extract_document_symbols(&Program::default());
        assert!(symbols.is_empty());
    }
}
