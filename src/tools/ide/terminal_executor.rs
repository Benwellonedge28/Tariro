//! # Terminal Executor
//!
//! Handles terminal command execution for the IDE tool.

use std::process::{Command, Stdio};
use std::path::Path;
use std::io::Read;

/// Command execution result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
}

/// Terminal Executor for running commands
pub struct TerminalExecutor {
    default_timeout_seconds: u64,
}

impl TerminalExecutor {
    /// Create a new terminal executor
    pub fn new() -> Self {
        Self {
            default_timeout_seconds: 300, // 5 minutes
        }
    }

    /// Execute a command
    pub fn execute(
        &self,
        command: &str,
        cwd: &str,
        max_output: usize,
    ) -> Result<CommandResult, String> {
        let cwd_path = Path::new(cwd);

        if !cwd_path.exists() {
            return Err(format!("Working directory does not exist: {}", cwd));
        }

        // Parse command into program and arguments
        // For simplicity, use shell -c on Unix or cmd /c on Windows
        #[cfg(target_os = "windows")]
        let mut cmd = {
            let mut c = Command::new("cmd");
            c.args(["/C", command]);
            c
        };

        #[cfg(not(target_os = "windows"))]
        let mut cmd = {
            let mut c = Command::new("sh");
            c.args(["-c", command]);
            c
        };

        cmd.current_dir(cwd_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))
            .map(|output| {
                let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();

                // Truncate output if too large
                if stdout.len() > max_output {
                    stdout.truncate(max_output);
                    stdout.push_str("\n... [output truncated]");
                }

                if stderr.len() > max_output {
                    stderr.truncate(max_output);
                    stderr.push_str("\n... [output truncated]");
                }

                CommandResult {
                    stdout,
                    stderr,
                    exit_code: output.status.code(),
                    timed_out: false,
                }
            })
    }

    /// Execute a command with a timeout
    pub fn execute_with_timeout(
        &self,
        command: &str,
        cwd: &str,
        timeout_seconds: u64,
        max_output: usize,
    ) -> Result<CommandResult, String> {
        use std::time::Duration;
        use std::thread;
        use std::sync::mpsc;

        let command = command.to_string();
        let cwd = cwd.to_string();

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let executor = TerminalExecutor::new();
            let result = executor.execute(&command, &cwd, max_output);
            let _ = tx.send(result);
        });

        match rx.recv_timeout(Duration::from_secs(timeout_seconds)) {
            Ok(result) => result,
            Err(_) => Ok(CommandResult {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: None,
                timed_out: true,
            }),
        }
    }

    /// Check if a command exists in PATH
    pub fn command_exists(&self, command: &str) -> bool {
        #[cfg(target_os = "windows")]
        let which_cmd = "where";

        #[cfg(not(target_os = "windows"))]
        let which_cmd = "which";

        let result = Command::new(which_cmd)
            .arg(command)
            .output();

        result.map(|o| o.status.success()).unwrap_or(false)
    }
}

impl Default for TerminalExecutor {
    fn default() -> Self {
        Self::new()
    }
}
