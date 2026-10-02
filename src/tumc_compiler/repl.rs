//! TUMC REPL (Read-Eval-Print Loop)
//!
//! Interactive development environment for TUMC with live feedback,
//! history, auto-completion, and inline help.

use crate::tumc_compiler::*;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

/// REPL state
pub struct Repl {
    /// Compiler instance
    compiler: Compiler,
    /// History of commands
    history: Vec<String>,
    /// Variables
    variables: HashMap<String, Value>,
    /// Type context
    type_context: TypeContext,
    /// Configuration
    config: ReplConfig,
    /// Multi-line input buffer
    multiline_buffer: Vec<String>,
    /// Indent level
    indent_level: usize,
}

/// REPL configuration
#[derive(Debug, Clone)]
pub struct ReplConfig {
    /// Prompt
    pub prompt: String,
    /// Multiline prompt
    pub multiline_prompt: String,
    /// Show types
    pub show_types: bool,
    /// Show timing
    pub show_timing: bool,
    /// Auto-import prelude
    pub auto_import_prelude: bool,
    /// Enable colors
    pub enable_colors: bool,
    /// History size
    pub history_size: usize,
    /// Auto-complete enabled
    pub autocomplete: bool,
}

impl Default for ReplConfig {
    fn default() -> Self {
        Self {
            prompt: "tariro> ".to_string(),
            multiline_prompt: "..".to_string(),
            show_types: true,
            show_timing: true,
            auto_import_prelude: true,
            enable_colors: true,
            history_size: 1000,
            autocomplete: true,
        }
    }
}

/// REPL value
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Function(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Value::Function(name) => write!(f, "<function {}>", name),
        }
    }
}

impl Repl {
    /// Create new REPL
    pub fn new() -> Self {
        Self {
            compiler: Compiler::new(SourceLanguage::TumcIR, TargetPlatform::Wasm),
            history: Vec::new(),
            variables: HashMap::new(),
            type_context: TypeContext::default(),
            config: ReplConfig::default(),
            multiline_buffer: Vec::new(),
            indent_level: 0,
        }
    }

    /// Create with config
    pub fn with_config(config: ReplConfig) -> Self {
        Self {
            compiler: Compiler::new(SourceLanguage::TumcIR, TargetPlatform::Wasm),
            history: Vec::new(),
            variables: HashMap::new(),
            type_context: TypeContext::default(),
            config,
            multiline_buffer: Vec::new(),
            indent_level: 0,
        }
    }

    /// Run the REPL
    pub fn run(&mut self) -> ReplResult {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        println!("Tariro REPL v0.1.0");
        println!("Type :help for help, :quit to exit\n");

        loop {
            let prompt = if self.indent_level > 0 {
                "   ".repeat(self.indent_level) + &self.config.multiline_prompt
            } else {
                self.config.prompt.clone()
            };

            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut line = String::new();
            match handle.read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    // Handle commands
                    if trimmed.starts_with(':') {
                        match self.handle_command(trimmed) {
                            ReplCommand::Continue => continue,
                            ReplCommand::Exit => break,
                            ReplCommand::Ok => {}
                        }
                    } else {
                        // Add to history
                        self.history.push(trimmed.to_string());
                        if self.history.len() > self.config.history_size {
                            self.history.remove(0);
                        }

                        // Process input
                        let result = self.process_input(trimmed);
                        if let Some(output) = result {
                            println!("{}", output);
                        }
                    }
                }
                Err(_) => break,
            }
        }

        ReplResult {
            commands_executed: self.history.len(),
            errors: 0,
        }
    }

    /// Handle REPL command
    fn handle_command(&mut self, cmd: &str) -> ReplCommand {
        let parts: Vec<&str> = cmd[1..].split_whitespace().collect();
        let cmd_name = parts.first().unwrap_or(&"");

        match *cmd_name {
            "help" => {
                self.show_help();
                ReplCommand::Continue
            }
            "quit" | "exit" => ReplCommand::Exit,
            "clear" => {
                print!("\x1B[2J\x1B[H");
                ReplCommand::Continue
            }
            "history" => {
                for (i, cmd) in self.history.iter().enumerate() {
                    println!("{}  {}", i + 1, cmd);
                }
                ReplCommand::Continue
            }
            "vars" => {
                for (name, value) in &self.variables {
                    println!("{}: {}", name, value);
                }
                ReplCommand::Continue
            }
            "type" => {
                if let Some(var) = parts.get(1) {
                    if let Some(value) = self.variables.get(*var) {
                        println!("{}", self.type_of(value));
                    }
                }
                ReplCommand::Continue
            }
            "load" => {
                if let Some(path) = parts.get(1) {
                    self.load_file(path);
                }
                ReplCommand::Continue
            }
            "reset" => {
                self.reset();
                println!("REPL state reset");
                ReplCommand::Continue
            }
            "config" => {
                self.show_config();
                ReplCommand::Continue
            }
            "set" => {
                if let Some(key) = parts.get(1) {
                    if let Some(value) = parts.get(2) {
                        self.set_config(key, value);
                    }
                }
                ReplCommand::Continue
            }
            _ => {
                println!("Unknown command: {}", cmd_name);
                ReplCommand::Continue
            }
        }
    }

    /// Process input
    fn process_input(&mut self, input: &str) -> Option<String> {
        let start = std::time::Instant::now();

        // Check for multiline start/end
        let trimmed = input.trim();

        // Handle multiline
        if trimmed.ends_with('{') || trimmed.ends_with('(') {
            self.multiline_buffer.push(trimmed.to_string());
            self.indent_level += 1;
            return None;
        }

        if self.indent_level > 0 {
            self.multiline_buffer.push(trimmed.to_string());
            self.indent_level -= 1;

            if self.indent_level == 0 {
                // Execute multiline block
                let code = self.multiline_buffer.join("\n");
                self.multiline_buffer.clear();
                return self.execute_code(&code, start);
            }
            return None;
        }

        self.execute_code(input, start)
    }

    /// Execute code
    fn execute_code(&mut self, code: &str, start: std::time::Instant) -> Option<String> {
        // Try to compile
        let result = self.compiler.compile(code);

        if result.success {
            let duration = start.elapsed();

            // Extract value if there's a final expression
            let output = self.extract_result(&result);

            if self.config.show_timing {
                return Some(format!("{} (took {:?})", output, duration));
            }
            return Some(output);
        } else {
            // Show errors
            for error in &result.errors {
                eprintln!("Error: {}", error);
            }
            return None;
        }
    }

    /// Extract result from compilation
    fn extract_result(&self, result: &CompilationResult) -> String {
        // For now, just indicate success
        format!("Compiled successfully ({} bytes)", result.output.len())
    }

    /// Get type of value
    fn type_of(&self, value: &Value) -> String {
        match value {
            Value::Int(_) => "i64".to_string(),
            Value::Float(_) => "f64".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Function(_) => "function".to_string(),
        }
    }

    /// Show help
    fn show_help(&self) {
        println!(
            r#"
Tariro REPL Commands:
  :help          Show this help
  :quit, :exit   Exit the REPL
  :clear         Clear the screen
  :history       Show command history
  :vars          Show defined variables
  :type <var>    Show type of variable
  :load <file>   Load and execute file
  :reset         Reset REPL state
  :config        Show configuration
  :set <key> <val>  Set configuration

Expressions:
  Use standard Tariro syntax for expressions.
  Multi-line input: end lines with {{ for blocks.
"#
        );
    }

    /// Show configuration
    fn show_config(&self) {
        println!("REPL Configuration:");
        println!("  prompt: {}", self.config.prompt);
        println!("  show_types: {}", self.config.show_types);
        println!("  show_timing: {}", self.config.show_timing);
        println!("  autocomplete: {}", self.config.autocomplete);
    }

    /// Set configuration
    fn set_config(&mut self, key: &str, value: &str) {
        match key {
            "prompt" => self.config.prompt = value.to_string(),
            "show_types" => self.config.show_types = value == "true",
            "show_timing" => self.config.show_timing = value == "true",
            "autocomplete" => self.config.autocomplete = value == "true",
            _ => println!("Unknown config key: {}", key),
        }
    }

    /// Load file
    fn load_file(&mut self, path: &str) {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                for line in content.lines() {
                    if let Some(output) = self.process_input(line) {
                        println!("{}", output);
                    }
                }
            }
            Err(e) => println!("Failed to load file: {}", e),
        }
    }

    /// Reset state
    fn reset(&mut self) {
        self.variables.clear();
        self.history.clear();
        self.multiline_buffer.clear();
        self.indent_level = 0;
    }

    /// Get completions for partial input
    pub fn get_completions(&self, partial: &str) -> Vec<String> {
        let keywords = vec![
            "fn", "let", "mut", "if", "else", "loop", "while", "return",
            "struct", "enum", "impl", "trait", "pub", "mod", "use",
            "true", "false", "nil", "i32", "i64", "f32", "f64", "bool",
        ];

        let mut completions = Vec::new();

        // Add keyword completions
        for kw in &keywords {
            if kw.starts_with(partial) {
                completions.push(kw.to_string());
            }
        }

        // Add variable completions
        for var in self.variables.keys() {
            if var.starts_with(partial) {
                completions.push(var.clone());
            }
        }

        completions
    }

    /// Export history to file
    pub fn export_history(&self, path: &str) -> io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        for cmd in &self.history {
            writeln!(file, "{}", cmd)?;
        }
        Ok(())
    }

    /// Import history from file
    pub fn import_history(&mut self, path: &str) -> io::Result<()> {
        let file = std::fs::File::open(path)?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(cmd) = line {
                self.history.push(cmd);
            }
        }
        Ok(())
    }
}

/// REPL command result
#[derive(Debug)]
enum ReplCommand {
    Continue,
    Exit,
    Ok,
}

/// REPL result
#[derive(Debug)]
pub struct ReplResult {
    pub commands_executed: usize,
    pub errors: usize,
}

/// REPL session
pub struct ReplSession {
    /// REPL instance
    repl: Repl,
    /// Session ID
    session_id: u64,
    /// Start time
    start_time: std::time::Instant,
}

impl ReplSession {
    /// Create new session
    pub fn new() -> Self {
        Self {
            repl: Repl::new(),
            session_id: rand::random(),
            start_time: std::time::Instant::now(),
        }
    }

    /// Run session
    pub fn run(&mut self) -> ReplResult {
        self.repl.run()
    }

    /// Get session info
    pub fn session_info(&self) -> SessionInfo {
        SessionInfo {
            session_id: self.session_id,
            uptime: self.start_time.elapsed(),
            history_size: self.repl.history.len(),
        }
    }
}

/// Session info
#[derive(Debug)]
pub struct SessionInfo {
    pub session_id: u64,
    pub uptime: std::time::Duration,
    pub history_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let repl = Repl::new();
        assert!(repl.history.is_empty());
    }

    #[test]
    fn test_repl_with_config() {
        let config = ReplConfig {
            show_types: false,
            ..Default::default()
        };
        let repl = Repl::with_config(config);
        assert!(!repl.config.show_types);
    }

    #[test]
    fn test_value_display() {
        assert_eq!(Value::Int(42).to_string(), "42");
        assert_eq!(Value::Float(3.14).to_string(), "3.14");
        assert_eq!(Value::Bool(true).to_string(), "true");
        assert_eq!(Value::String("hello".to_string()).to_string(), "\"hello\"");
    }

    #[test]
    fn test_completions() {
        let repl = Repl::new();
        let completions = repl.get_completions("f");
        assert!(completions.contains(&"fn".to_string()));
        assert!(completions.contains(&"false".to_string()));
    }
}
