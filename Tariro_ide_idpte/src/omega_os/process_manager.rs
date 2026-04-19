// Tariro_ide_idpte/src/omega_os/process_manager.rs

//! Manages the lifecycle of computational processes within OMEGA OS.
//! This module provides the core functionality for spawning, monitoring, and controlling
//! processes and services, acting as a crucial component for resource management and system self-actualization.

use anyhow::Result;
use tokio::process::Command;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};

/// Unique identifier for a process.
pub type ProcessId = String;

/// Represents the current status of an OMEGA OS process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessStatus {
    Running,
    Stopped,
    Paused,
    Terminated,
    Error,
    Starting,
}

/// Information about a managed process.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub id: ProcessId,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub status: ProcessStatus,
    pub pid: Option<u32>, // Actual OS PID if available
    pub started_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub resource_usage: HashMap<String, f64>, // Mock resource usage metrics
    pub managed_by_agi_module: Option<String>, // e.g., "SORAEE", "HACSE"
}

/// The OMEGA OS Process Manager.
pub struct ProcessManager {
    processes: Mutex<HashMap<ProcessId, ProcessInfo>>,
    // In a real OS, this would interact with kernel-level process management APIs.
}

impl ProcessManager {
    pub fn new() -> Self {
        info!("OMEGA OS Process Manager initialized.");
        ProcessManager { processes: Mutex::new(HashMap::new()) }
    }

    /// Spawns a new process based on a command and arguments.
    /// Returns the ProcessId if successful.
    pub async fn spawn_process(
        &self,
        name: &str,
        command: &str,
        args: Vec<String>,
        managed_by: Option<String>,
    ) -> Result<ProcessId, String> {
        let process_id = Uuid::new_v4().to_string();
        info!("ProcessManager: Spawning new process '{}' (ID: {}).", name, process_id);

        // In a real system, this would execute the command.
        // For now, simulate by adding to internal map and setting status to Running.
        let mut processes_guard = self.processes.lock().unwrap();
        let process_info = ProcessInfo {
            id: process_id.clone(),
            name: name.to_string(),
            command: command.to_string(),
            args: args.clone(),
            status: ProcessStatus::Starting,
            pid: Some(1000 + processes_guard.len() as u32), // Mock PID
            started_at: Utc::now(),
            last_updated: Utc::now(),
            resource_usage: HashMap::new(),
            managed_by_agi_module: managed_by,
        };
        processes_guard.insert(process_id.clone(), process_info);

        // Simulate process startup time
        sleep(Duration::from_millis(100)).await;
        if let Some(p) = processes_guard.get_mut(&process_id) {
            p.status = ProcessStatus::Running;
            p.last_updated = Utc::now();
            info!("ProcessManager: Process '{}' (ID: {}) is now Running.", name, process_id);
        }

        Ok(process_id)
    }

    /// Terminates a running process.
    pub async fn kill_process(&self, process_id: &ProcessId) -> Result<(), String> {
        info!("ProcessManager: Attempting to kill process {}.", process_id);
        let mut processes_guard = self.processes.lock().unwrap();
        if let Some(p) = processes_guard.get_mut(process_id) {
            if p.status == ProcessStatus::Running || p.status == ProcessStatus::Paused {
                p.status = ProcessStatus::Terminated;
                p.last_updated = Utc::now();
                info!("ProcessManager: Process '{}' (ID: {}) terminated.", p.name, process_id);
                Ok(())
            } else {
                Err(format!("Process {} is not running or paused, cannot terminate.", process_id))
            }
        } else {
            Err(format!("Process with ID {} not found.", process_id))
        }
    }

    /// Suspends a running process.
    pub async fn pause_process(&self, process_id: &ProcessId) -> Result<(), String> {
        info!("ProcessManager: Attempting to pause process {}.", process_id);
        let mut processes_guard = self.processes.lock().unwrap();
        if let Some(p) = processes_guard.get_mut(process_id) {
            if p.status == ProcessStatus::Running {
                p.status = ProcessStatus::Paused;
                p.last_updated = Utc::now();
                info!("ProcessManager: Process '{}' (ID: {}) paused.", p.name, process_id);
                Ok(())
            } else {
                Err(format!("Process {} is not running, cannot pause.", process_id))
            }
        } else {
            Err(format!("Process with ID {} not found.", process_id))
        }
    }

    /// Resumes a paused process.
    pub async fn resume_process(&self, process_id: &ProcessId) -> Result<(), String> {
        info!("ProcessManager: Attempting to resume process {}.", process_id);
        let mut processes_guard = self.processes.lock().unwrap();
        if let Some(p) = processes_guard.get_mut(process_id) {
            if p.status == ProcessStatus::Paused {
                p.status = ProcessStatus::Running;
                p.last_updated = Utc::now();
                info!("ProcessManager: Process '{}' (ID: {}) resumed.", p.name, process_id);
                Ok(())
            } else {
                Err(format!("Process {} is not paused, cannot resume.", process_id))
            }
        } else {
            Err(format!("Process with ID {} not found.", process_id))
        }
    }

    /// Retrieves information about a specific process.
    pub async fn get_process_info(&self, process_id: &ProcessId) -> Option<ProcessInfo> {
        self.processes.lock().unwrap().get(process_id).cloned()
    }

    /// Lists all currently managed processes.
    pub async fn list_processes(&self) -> Vec<ProcessInfo> {
        self.processes.lock().unwrap().values().cloned().collect()
    }
}
