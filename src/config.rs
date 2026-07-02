// src/config.rs

//! Manages the configuration and settings for the Tariro AGI.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Re-export specific types from the SDK
pub use tariro_sdk::entities::{AGIControlSetting, AGICategory, AutonomyPolicy, CapabilityStatus, WorkflowRule, ServicePrice};
pub use tariro_sdk::entities::{Goal, WellbeingLog, ExplainabilityLog, PerformanceLog, LearningInsight, FeatureProposal, SystemHealthLog};
pub use tariro_sdk::entities::{Conversation, KnowledgeItem, ManagedFile};
pub use tariro_sdk::entities::{GitHubProfile, GitHubProjectContext, IDEProfile};
pub use tariro_sdk::entities::{ProjectTask, NotificationRecord};
pub use tariro_sdk::entities::{NanoAgent, MetaProject, CompilerProject, CompilerSnapshot, DeploymentRecord};

/// Represents the overall configuration for the Tariro AGI.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AGIConfig {
    pub system_name: String,
    pub system_version: String,
    pub environment: String,
    pub log_level: String,
    pub database_url: String,
    pub api_port: u16,
    pub agi_settings: HashMap<String, String>, // Dynamic AGI-specific settings
    pub super_admin_email: String, // For critical notifications
    pub qsm_monitoring_interval_seconds: u64, // Interval for QSM threat monitoring
    pub system_health_monitoring_interval_seconds: u64, // Interval for general system health checks
    pub self_correction_enabled: bool, // Enable/disable self-correction mechanisms
    pub self_modification_interval_seconds: u64, // NEW: Interval for autonomous self-modification cycles
    pub self_modification_approval: String, // NEW: "full_auto", "confirm_admin", "always_confirm"
}

impl Default for AGIConfig {
    fn default() -> Self {
        let mut agi_settings = HashMap::new();
        agi_settings.insert("autonomy.default_level".to_string(), "suggest_only".to_string());
        agi_settings.insert("autonomy.evas_filter_enabled".to_string(), "true".to_string());
        agi_settings.insert("agi.self_modification_enabled".to_string(), "true".to_string());
        agi_settings.insert("agi.learning_rate".to_string(), "0.01".to_string());
        agi_settings.insert("omega_network.enabled".to_string(), "true".to_string());
        agi_settings.insert("omega_network.mesh_protocol_version".to_string(), "1.0".to_string());
        agi_settings.insert("compiler.default_target_platform".to_string(), "wasm".to_string());

        AGIConfig {
            system_name: "Tariro AGI".to_string(),
            system_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            log_level: "info".to_string(),
            database_url: "mongodb://localhost:27017/tariro_agi".to_string(),
            api_port: 8080,
            agi_settings,
            super_admin_email: "samuelmukandara@gmail.com".to_string(),
            qsm_monitoring_interval_seconds: 600,
            system_health_monitoring_interval_seconds: 300,
            self_correction_enabled: true,
            self_modification_interval_seconds: 900, // NEW: Default to 15 minutes
            self_modification_approval: "full_auto".to_string(), // NEW: Default to full auto for self-modification
        }
    }
}

impl AGIConfig {
    /// Loads the configuration from a TOML file.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let config_str = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        toml::from_str(&config_str)
            .map_err(|e| format!("Failed to parse config file: {}", e))?
    }

    /// Saves the current configuration to a TOML file.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(path, toml_string)
            .map_err(|e| format!("Failed to write config file: {}", e))?
    }

    /// Retrieves a setting from `agi_settings`.
    pub fn get_setting(&self, key: &str) -> Option<String> {
        self.agi_settings.get(key).cloned()
    }

    /// Updates or adds a setting in `agi_settings`.
    pub fn update_setting(&mut self, key: &str, value: &str) {
        self.agi_settings.insert(key.to_string(), value.to_string());
    }

    /// Merges an `AGIControlSetting` entity into the current configuration.
    pub fn apply_agi_control(&mut self, control: &AGIControlSetting) {
        self.update_setting(&control.control_key, &control.value);
        println!("[Config] Applied AGI Control: {} = {}", control.control_key, control.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = AGIConfig::default();
        assert_eq!(config.system_name, "Tariro AGI");
        assert_eq!(config.api_port, 8080);
        assert!(config.agi_settings.get("autonomy.default_level").unwrap() == "suggest_only");
        assert_eq!(config.super_admin_email, "samuelmukandara@gmail.com");
        assert_eq!(config.qsm_monitoring_interval_seconds, 600);
        assert_eq!(config.system_health_monitoring_interval_seconds, 300);
        assert_eq!(config.self_correction_enabled, true);
        assert_eq!(config.self_modification_interval_seconds, 900); 
        assert_eq!(config.self_modification_approval, "full_auto"); 
    }

    #[test]
    fn test_load_and_save_config() {
        let mut config = AGIConfig::default();
        config.system_name = "Test Tariro".to_string();
        config.update_setting("test.key", "test_value");
        config.super_admin_email = "test@example.com".to_string();
        config.qsm_monitoring_interval_seconds = 300;
        config.system_health_monitoring_interval_seconds = 120;
        config.self_correction_enabled = false;
        config.self_modification_interval_seconds = 60; 
        config.self_modification_approval = "confirm_admin".to_string(); 

        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_config.toml");

        config.save(&file_path).unwrap();
        let loaded_config = AGIConfig::load(&file_path).unwrap();

        assert_eq!(loaded_config.system_name, "Test Tariro");
        assert_eq!(loaded_config.get_setting("test.key").unwrap(), "test_value");
        assert_eq!(loaded_config.super_admin_email, "test@example.com");
        assert_eq!(loaded_config.qsm_monitoring_interval_seconds, 300);
        assert_eq!(loaded_config.system_health_monitoring_interval_seconds, 120);
        assert_eq!(loaded_config.self_correction_enabled, false);
        assert_eq!(loaded_config.self_modification_interval_seconds, 60); 
        assert_eq!(loaded_config.self_modification_approval, "confirm_admin"); 

        temp_dir.close().unwrap();
    }

    #[test]
    fn test_apply_agi_control() {
        let mut config = AGIConfig::default();
        let control = AGIControlSetting {
            id: Some("ctrl_1".to_string()),
            control_key: "new.feature.enabled".to_string(),
            value: "true".to_string(),
            category: Some("features".to_string()),
            label: "New Feature Toggle".to_string(),
            description: None,
            last_modified_by: None,
        };
        config.apply_agi_control(&control);
        assert_eq!(config.get_setting("new.feature.enabled").unwrap(), "true");
    }
}
