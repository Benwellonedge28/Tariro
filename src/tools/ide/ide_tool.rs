//! # IDE Tool - Main Interface
//!
//! The main IDE tool that provides file management, terminal execution,
//! and LSP capabilities to Tariro AGI.

use crate::tools::ide::{IDEToolResponse, IDEAction};
use crate::tools::ide::file_manager::FileManager;
use crate::tools::ide::terminal_executor::TerminalExecutor;
use crate::tools::ide::lsp_client::LSPClient;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// IDE Tool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEToolConfig {
    /// Enable file management operations
    pub enable_file_operations: bool,
    /// Enable terminal command execution
    pub enable_terminal: bool,
    /// Enable LSP features (autocomplete, diagnostics, etc.)
    pub enable_lsp: bool,
    /// Root workspace directory
    pub workspace_root: String,
    /// LSP server command (e.g., ["rust-analyzer"])
    pub lsp_command: Vec<String>,
    /// LSP server port (for stdio communication)
    pub lsp_port: Option<u16>,
    /// Maximum terminal output size in bytes
    pub max_terminal_output: usize,
    /// Allowed terminal commands (security)
    pub allowed_commands: Vec<String>,
    /// Blocked terminal commands (security)
    pub blocked_commands: Vec<String>,
}

impl Default for IDEToolConfig {
    fn default() -> Self {
        Self {
            enable_file_operations: true,
            enable_terminal: true,
            enable_lsp: true,
            workspace_root: std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| ".".to_string()),
            lsp_command: vec!["rust-analyzer".to_string()],
            lsp_port: None,
            max_terminal_output: 1024 * 1024, // 1MB
            allowed_commands: vec![],
            blocked_commands: vec![
                "rm -rf /".to_string(),
                "rm -rf /home".to_string(),
                "del /f /q".to_string(),
                "format".to_string(),
            ],
        }
    }
}

/// IDE Tool capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEToolCapabilities {
    pub can_read_files: bool,
    pub can_write_files: bool,
    pub can_list_directories: bool,
    pub can_create_directories: bool,
    pub can_delete_files: bool,
    pub can_execute_commands: bool,
    pub can_lsp_completions: bool,
    pub can_lsp_diagnostics: bool,
    pub can_lsp_goto_definition: bool,
    pub can_lsp_find_references: bool,
}

impl Default for IDEToolCapabilities {
    fn default() -> Self {
        Self {
            can_read_files: true,
            can_write_files: true,
            can_list_directories: true,
            can_create_directories: true,
            can_delete_files: true,
            can_execute_commands: true,
            can_lsp_completions: true,
            can_lsp_diagnostics: true,
            can_lsp_goto_definition: true,
            can_lsp_find_references: true,
        }
    }
}

/// Result type for IDE tool operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEToolResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

impl IDEToolResult {
    pub fn ok(output: String) -> Self {
        Self {
            success: true,
            output,
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
        }
    }
}

/// IDE Tool - Main entry point for IDE operations
pub struct IDETool {
    config: IDEToolConfig,
    capabilities: IDEToolCapabilities,
    file_manager: FileManager,
    terminal: TerminalExecutor,
    lsp_client: Option<LSPClient>,
}

impl IDETool {
    /// Create a new IDE tool
    pub fn new(config: IDEToolConfig) -> Self {
        let capabilities = IDEToolCapabilities {
            can_read_files: config.enable_file_operations,
            can_write_files: config.enable_file_operations,
            can_list_directories: config.enable_file_operations,
            can_create_directories: config.enable_file_operations,
            can_delete_files: config.enable_file_operations,
            can_execute_commands: config.enable_terminal,
            can_lsp_completions: config.enable_lsp,
            can_lsp_diagnostics: config.enable_lsp,
            can_lsp_goto_definition: config.enable_lsp,
            can_lsp_find_references: config.enable_lsp,
        };

        let lsp_client = if config.enable_lsp {
            Some(LSPClient::new(&config.lsp_command, config.lsp_port))
        } else {
            None
        };

        Self {
            config,
            capabilities,
            file_manager: FileManager::new(),
            terminal: TerminalExecutor::new(),
            lsp_client,
        }
    }

    /// Get tool capabilities
    pub fn get_capabilities(&self) -> IDEToolCapabilities {
        self.capabilities.clone()
    }

    /// Get tool configuration
    pub fn get_config(&self) -> &IDEToolConfig {
        &self.config
    }

    /// Execute an IDE action
    pub fn execute(&mut self, action: IDEAction) -> IDEToolResponse {
        let start = Instant::now();

        let result = match action.clone() {
            IDEAction::ReadFile { path } => {
                if !self.capabilities.can_read_files {
                    return IDEToolResponse::failure(
                        action,
                        "File operations disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                match self.file_manager.read_file(&path) {
                    Ok(content) => IDEToolResponse::success(
                        action,
                        serde_json::json!({ "content": content, "path": path }),
                        start.elapsed().as_millis() as u64,
                    ),
                    Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                }
            }

            IDEAction::WriteFile { path, content } => {
                if !self.capabilities.can_write_files {
                    return IDEToolResponse::failure(
                        action,
                        "File write operations disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                match self.file_manager.write_file(&path, &content) {
                    Ok(_) => IDEToolResponse::success(
                        action,
                        serde_json::json!({ "path": path, "bytes_written": content.len() }),
                        start.elapsed().as_millis() as u64,
                    ),
                    Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                }
            }

            IDEAction::ListDir { path } => {
                if !self.capabilities.can_list_directories {
                    return IDEToolResponse::failure(
                        action,
                        "Directory listing disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                match self.file_manager.list_directory(&path) {
                    Ok(entries) => IDEToolResponse::success(
                        action,
                        serde_json::json!({ "path": path, "entries": entries }),
                        start.elapsed().as_millis() as u64,
                    ),
                    Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                }
            }

            IDEAction::CreateDir { path } => {
                if !self.capabilities.can_create_directories {
                    return IDEToolResponse::failure(
                        action,
                        "Directory creation disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                match self.file_manager.create_directory(&path) {
                    Ok(_) => IDEToolResponse::success(
                        action,
                        serde_json::json!({ "path": path }),
                        start.elapsed().as_millis() as u64,
                    ),
                    Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                }
            }

            IDEAction::DeleteFile { path } => {
                if !self.capabilities.can_delete_files {
                    return IDEToolResponse::failure(
                        action,
                        "File deletion disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                match self.file_manager.delete_file(&path) {
                    Ok(_) => IDEToolResponse::success(
                        action,
                        serde_json::json!({ "path": path }),
                        start.elapsed().as_millis() as u64,
                    ),
                    Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                }
            }

            IDEAction::ExecuteCommand { command, cwd } => {
                if !self.capabilities.can_execute_commands {
                    return IDEToolResponse::failure(
                        action,
                        "Terminal execution disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }

                // Security check
                if self.is_command_blocked(&command) {
                    return IDEToolResponse::failure(
                        action,
                        "Command blocked for security".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }

                let cwd = cwd.unwrap_or_else(|| self.config.workspace_root.clone());
                match self.terminal.execute(&command, &cwd, self.config.max_terminal_output) {
                    Ok(output) => IDEToolResponse::success(
                        action,
                        serde_json::json!({
                            "command": command,
                            "cwd": cwd,
                            "output": output.stdout,
                            "exit_code": output.exit_code
                        }),
                        start.elapsed().as_millis() as u64,
                    ),
                    Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                }
            }

            IDEAction::LSPCompletions { path, position } => {
                if !self.capabilities.can_lsp_completions {
                    return IDEToolResponse::failure(
                        action,
                        "LSP completions disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                if let Some(ref mut lsp) = self.lsp_client {
                    match lsp.get_completions(&path, position) {
                        Ok(items) => IDEToolResponse::success(
                            action,
                            serde_json::json!({ "completions": items }),
                            start.elapsed().as_millis() as u64,
                        ),
                        Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                    }
                } else {
                    IDEToolResponse::failure(
                        action,
                        "LSP client not initialized".to_string(),
                        start.elapsed().as_millis() as u64,
                    )
                }
            }

            IDEAction::LSPDiagnostics { path } => {
                if !self.capabilities.can_lsp_diagnostics {
                    return IDEToolResponse::failure(
                        action,
                        "LSP diagnostics disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                if let Some(ref mut lsp) = self.lsp_client {
                    match lsp.get_diagnostics(&path) {
                        Ok(diagnostics) => IDEToolResponse::success(
                            action,
                            serde_json::json!({ "diagnostics": diagnostics }),
                            start.elapsed().as_millis() as u64,
                        ),
                        Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                    }
                } else {
                    IDEToolResponse::failure(
                        action,
                        "LSP client not initialized".to_string(),
                        start.elapsed().as_millis() as u64,
                    )
                }
            }

            IDEAction::LSPGoToDefinition { path, position } => {
                if !self.capabilities.can_lsp_goto_definition {
                    return IDEToolResponse::failure(
                        action,
                        "LSP goto definition disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                if let Some(ref mut lsp) = self.lsp_client {
                    match lsp.goto_definition(&path, position) {
                        Ok(location) => IDEToolResponse::success(
                            action,
                            serde_json::json!({ "location": location }),
                            start.elapsed().as_millis() as u64,
                        ),
                        Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                    }
                } else {
                    IDEToolResponse::failure(
                        action,
                        "LSP client not initialized".to_string(),
                        start.elapsed().as_millis() as u64,
                    )
                }
            }

            IDEAction::LSPFindReferences { path, position } => {
                if !self.capabilities.can_lsp_find_references {
                    return IDEToolResponse::failure(
                        action,
                        "LSP find references disabled".to_string(),
                        start.elapsed().as_millis() as u64,
                    );
                }
                if let Some(ref mut lsp) = self.lsp_client {
                    match lsp.find_references(&path, position) {
                        Ok(references) => IDEToolResponse::success(
                            action,
                            serde_json::json!({ "references": references }),
                            start.elapsed().as_millis() as u64,
                        ),
                        Err(e) => IDEToolResponse::failure(action, e, start.elapsed().as_millis() as u64),
                    }
                } else {
                    IDEToolResponse::failure(
                        action,
                        "LSP client not initialized".to_string(),
                        start.elapsed().as_millis() as u64,
                    )
                }
            }
        };

        result
    }

    /// Check if a command is blocked for security
    fn is_command_blocked(&self, command: &str) -> bool {
        let cmd_lower = command.to_lowercase();

        // Check blocked commands
        for blocked in &self.config.blocked_commands {
            if cmd_lower.contains(&blocked.to_lowercase()) {
                return true;
            }
        }

        // Check allowed commands (if list is non-empty, only allow those)
        if !self.config.allowed_commands.is_empty() {
            let mut allowed = false;
            for allowed_cmd in &self.config.allowed_commands {
                if cmd_lower.contains(&allowed_cmd.to_lowercase()) {
                    allowed = true;
                    break;
                }
            }
            if !allowed {
                return true;
            }
        }

        false
    }
}

impl Default for IDETool {
    fn default() -> Self {
        Self::new(IDEToolConfig::default())
    }
}
