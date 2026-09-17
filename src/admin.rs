// src/admin.rs

//! Module for administrative functionalities and system-level controls.

use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an administrative action or command.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminCommand {
    pub command_id: String,
    pub initiator: String, // User ID or AGI module ID
    pub command_type: String, // e.g., "update_config", "reboot_module", "query_logs"
    pub payload: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub expected_outcome: Option<String>,
}

/// Represents the result of an administrative action.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminCommandResult {
    pub command_id: String,
    pub status: String, // e.g., "success", "failure", "pending", "unauthorized"
    pub message: String,
    pub actual_outcome: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// The `AdminService` handles system-wide administrative tasks and configurations.
pub struct AdminService {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    // Potentially other dependencies like a command queue, authentication service
}

impl AdminService {
    /// Creates a new `AdminService` instance.
    pub fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self {
        AdminService { config, telemetry_logger }
    }

    /// Executes an administrative command.
    pub async fn execute_admin_command(&mut self, command: AdminCommand) -> AdminCommandResult {
        self.telemetry_logger.log_event(TelemetryEvent::AdminCommandIssued { 
            command_id: command.command_id.clone(), 
            command_type: command.command_type.clone(),
            initiator: command.initiator.clone(),
        }).await;

        println!("[AdminService] Executing command: {:?}", command);
        // Simulate command execution time
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        let result = match self.process_command(&command).await {
            Ok(msg) => AdminCommandResult {
                command_id: command.command_id.clone(),
                status: "success".to_string(),
                message: msg.clone(),
                actual_outcome: Some(msg),
                timestamp: Utc::now(),
            },
            Err(e) => AdminCommandResult {
                command_id: command.command_id.clone(),
                status: "failure".to_string(),
                message: e.clone(),
                actual_outcome: Some(e),
                timestamp: Utc::now(),
            },
        };

        self.telemetry_logger.log_event(TelemetryEvent::AdminCommandCompleted { 
            command_id: result.command_id.clone(), 
            status: result.status.clone(),
            message: result.message.clone(),
        }).await;
        result
    }

    async fn process_command(&mut self, command: &AdminCommand) -> Result<String, String> {
        match command.command_type.as_str() {
            "update_config" => self.handle_update_config(command).await,
            "reboot_module" => self.handle_reboot_module(command).await,
            "query_system_status" => self.handle_query_system_status(command).await,
            _ => Err(format!("Unknown admin command type: {}", command.command_type)),
        }
    }

    async fn handle_update_config(&mut self, command: &AdminCommand) -> Result<String, String> {
        if let Some(key) = command.payload.get("key") {
            if let Some(value) = command.payload.get("value") {
                // In a real system, this would update the actual config struct
                // For simulation, we just print and log.
                println!("  [AdminService] Updating config key '{}' to '{}'.", key, value);
                self.config.update_setting(key, value);
                Ok(format!("Config key '{}' updated successfully.", key))
            } else {
                Err("Missing 'value' in update_config payload.".to_string())
            }
        } else {
            Err("Missing 'key' in update_config payload.".to_string())
        }
    }

    async fn handle_reboot_module(&self, command: &AdminCommand) -> Result<String, String> {
        if let Some(module_name) = command.payload.get("module_name") {
            println!("  [AdminService] Initiating reboot for module: {}", module_name);
            // Simulate reboot logic
            Ok(format!("Reboot of module '{}' initiated successfully.", module_name))
        } else {
            Err("Missing 'module_name' in reboot_module payload.".to_string())
        }
    }

    async fn handle_query_system_status(&self, _command: &AdminCommand) -> Result<String, String> {
        println!("  [AdminService] Querying system status.");
        // Simulate fetching system status
        Ok("System status: All modules operational. CPU: 20%, Memory: 40%.".to_string())
    }

    /// Retrieves the current AGI system configuration.
    pub fn get_current_config(&self) -> &AGIConfig {
        &self.config
    }

    /// Applies a set of `AGIControl` settings, typically from an Admin UI or a self-governance process.
    pub async fn apply_agi_controls(&mut self, controls: &[crate::config::AGIControl]) -> Result<Vec<AdminCommandResult>, String> {
        let mut results = Vec::new();
        println!("[AdminService] Applying {} AGI controls.", controls.len());
        for control in controls {
            let command = AdminCommand {
                command_id: format!("admin_apply_ctrl_{}", uuid::Uuid::new_v4()),
                initiator: "AGI_System_Governance".to_string(),
                command_type: "update_config".to_string(),
                payload: HashMap::from([
                    ("key".to_string(), control.control_key.clone()),
                    ("value".to_string(), control.value.clone()),
                ]),
                timestamp: Utc::now(),
                expected_outcome: Some(format!("Config key {} updated to {}", control.control_key, control.value)),
            };
            results.push(self.execute_admin_command(command).await);
        }
        println!("[AdminService] Finished applying AGI controls. {} successful, {} failed.", 
                   results.iter().filter(|r| r.status == "success").count(),
                   results.iter().filter(|r| r.status == "failure").count());
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AGIControl;
    use crate::telemetry::TelemetryEvent;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_execute_admin_command_update_config() {
        let (tx, mut rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx);
        let mut config = AGIConfig::default();
        let mut admin_service = AdminService::new(config.clone(), telemetry_logger);

        let command = AdminCommand {
            command_id: "test_cmd_1".to_string(),
            initiator: "test_user".to_string(),
            command_type: "update_config".to_string(),
            payload: HashMap::from([
                ("key".to_string(), "core.log_level".to_string()),
                ("value".to_string(), "debug".to_string()),
            ]),
            timestamp: Utc::now(),
            expected_outcome: None,
        };

        let result = admin_service.execute_admin_command(command.clone()).await;
        assert_eq!(result.status, "success");
        assert!(result.message.contains("updated successfully"));

        // Check if config was actually updated (simulated)
        assert_eq!(admin_service.get_current_config().get_setting("core.log_level"), Some("debug".to_string()));

        // Check telemetry logs
        let event1 = rx.recv().await.unwrap();
        match event1 {
            TelemetryEvent::AdminCommandIssued { command_id, .. } => assert_eq!(command_id, "test_cmd_1"),
            _ => panic!("Unexpected event type"),
        }
        let event2 = rx.recv().await.unwrap();
        match event2 {
            TelemetryEvent::AdminCommandCompleted { command_id, status, .. } => {
                assert_eq!(command_id, "test_cmd_1");
                assert_eq!(status, "success");
            },
            _ => panic!("Unexpected event type"),
        }
    }

    #[tokio::test]
    async fn test_apply_agi_controls() {
        let (tx, mut rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx);
        let mut config = AGIConfig::default();
        let mut admin_service = AdminService::new(config.clone(), telemetry_logger);

        let controls = vec![
            AGIControl {
                id: Some("ctrl_1".to_string()),
                control_key: "autonomy.evast_filter_enabled".to_string(),
                value: "false".to_string(),
                category: None,
                label: "EVAS Filter".to_string(),
                description: None,
                last_modified_by: None,
            },
            AGIControl {
                id: Some("ctrl_2".to_string()),
                control_key: "core.log_level".to_string(),
                value: "error".to_string(),
                category: None,
                label: "Log Level".to_string(),
                description: None,
                last_modified_by: None,
            },
        ];

        let results = admin_service.apply_agi_controls(&controls).await.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].status, "success");
        assert_eq!(results[1].status, "success");

        // Verify config updates (simulated)
        assert_eq!(admin_service.get_current_config().get_setting("autonomy.evast_filter_enabled"), Some("false".to_string()));
        assert_eq!(admin_service.get_current_config().get_setting("core.log_level"), Some("error".to_string()));

        // Verify telemetry events (4 events: 2 issued, 2 completed)
        for _ in 0..4 {
            rx.recv().await.unwrap();
        }
    }
}
