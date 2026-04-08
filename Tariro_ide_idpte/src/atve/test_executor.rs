// src/atve/test_executor.rs
use std::process::{Command, Output};
use std::time::Duration;
use tokio::time::timeout;

pub struct TestResult {
    pub test_name: String,
    pub status: String,
    pub duration_ms: u64,
    pub output: String,
    pub error: Option<String>,
}

pub struct TestExecutor {
    project_root: String,
}

impl TestExecutor {
    pub fn new(project_root: String) -> Self {
        TestExecutor { project_root }
    }

    pub async fn execute_tests(&self, test_code_path: &str, timeout_sec: u64) -> Vec<TestResult> {
        println!("[TestExecutor] Simulating execution of tests in {}", test_code_path);
        let mut results = Vec::new();
        let test1_status = if rand::random::<f32>() < 0.8 { "Passed".to_string() } else { "Failed".to_string() };
        results.push(TestResult {
            test_name: format!("test_{}_feature_A", test_code_path.replace("/", "_")),
            status: test1_status.clone(),
            duration_ms: rand::random::<u64>() % 100 + 50,
            output: format!("Running test_feature_A... {}", if test1_status == "Passed" { "ok" } else { "failed" }),
            error: if test1_status == "Failed" { Some("Assertion failed: value mismatch.".to_string()) } else { None },
        });
        results
    }

    pub async fn run_command_and_capture_output(&self, command_str: &str, timeout_sec: u64) -> Result<Output, String> {
        let parts: Vec<&str> = command_str.split_whitespace().collect();
        if parts.is_empty() { return Err("Command string cannot be empty.".to_string()); }
        let program = parts[0];
        let args = &parts[1..];

        let command_future = Command::new(program)
            .args(args)
            .current_dir(&self.project_root)
            .output();

        match timeout(Duration::from_secs(timeout_sec), command_future).await {
            Ok(Ok(output)) => Ok(output),
            Ok(Err(e)) => Err(format!("Failed to execute command '{}': {}", command_str, e)),
            Err(_) => Err(format!("Command '{}' timed out after {} seconds", command_str, timeout_sec)),
        }
    }
}
