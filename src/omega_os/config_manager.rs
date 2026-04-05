// src/omega_os/config_manager.rs

//! Manages high-level configuration and policy settings for OMEGA OS instances
//! from the perspective of the Tariro AGI. This module does not deal with low-level
//! OS configuration files directly, but rather with abstract configuration parameters
//! and their application to OMEGA OS instances.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a high-level configuration profile for an OMEGA OS instance.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmegaOSConfigProfile {
    pub profile_id: String,
    pub name: String,
    pub description: String,
    pub created_by_agi: bool,
    pub creation_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub parameters: HashMap<String, String>, // Key-value pairs of config settings
    pub target_os_versions: Vec<String>, // e.g., "0.1.0-alpha", "0.1.0-beta"
    pub associated_autonomy_policy_id: Option<String>, // Link to a Tariro AutonomyPolicy
}

/// Represents a command to apply a configuration profile to one or more OMEGA OS instances.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplyConfigCommand {
    pub command_id: String,
    pub profile_id: String,
    pub target_instance_ids: Vec<String>, // IDs of OmegaOSInstance to apply to
    pub initiated_by: String,
    pub timestamp: DateTime<Utc>,
    pub status: ApplyConfigStatus,
    pub results: HashMap<String, String>, // Instance ID -> Result message
}

/// Status of an apply config command.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ApplyConfigStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    PartialSuccess,
}

/// The `OmegaOSConfigManager` handles the creation, management, and application of OMEGA OS
/// configuration profiles.
pub struct OmegaOSConfigManager {
    config_profiles: HashMap<String, OmegaOSConfigProfile>,
    apply_commands: HashMap<String, ApplyConfigCommand>,
}

impl OmegaOSConfigManager {
    /// Creates a new `OmegaOSConfigManager`.
    pub fn new() -> Self {
        OmegaOSConfigManager {
            config_profiles: HashMap::new(),
            apply_commands: HashMap::new(),
        }
    }

    /// Creates a new OMEGA OS configuration profile.
    pub fn create_profile(&mut self, mut profile: OmegaOSConfigProfile) -> Result<String, String> {
        profile.profile_id = format!("os_config_{}", Uuid::new_v4());
        profile.creation_date = Utc::now();
        profile.last_updated = Utc::now();

        if self.config_profiles.contains_key(&profile.profile_id) {
            return Err(format!("Config profile with ID {} already exists.", profile.profile_id));
        }
        println!("[OmegaOSConfigManager] Creating config profile: {}", profile.name);
        let id = profile.profile_id.clone();
        self.config_profiles.insert(id.clone(), profile);
        Ok(id)
    }

    /// Retrieves an OMEGA OS configuration profile by its ID.
    pub fn get_profile(&self, profile_id: &str) -> Option<&OmegaOSConfigProfile> {
        self.config_profiles.get(profile_id)
    }

    /// Updates an existing OMEGA OS configuration profile.
    pub fn update_profile(&mut self, mut updated_profile: OmegaOSConfigProfile) -> Result<(), String> {
        if let Some(profile) = self.config_profiles.get_mut(&updated_profile.profile_id) {
            profile.name = updated_profile.name;
            profile.description = updated_profile.description;
            profile.last_updated = Utc::now();
            profile.parameters = updated_profile.parameters;
            profile.target_os_versions = updated_profile.target_os_versions;
            profile.associated_autonomy_policy_id = updated_profile.associated_autonomy_policy_id;
            Ok(())}
         else {
            Err(format!("Config profile with ID {} not found.", updated_profile.profile_id))
        }
    }

    /// Initiates a command to apply a configuration profile to target OMEGA OS instances.
    pub async fn apply_config_profile(
        &mut self,
        profile_id: &str,
        target_instance_ids: Vec<String>,
        initiated_by: &str,
    ) -> Result<String, String> {
        let profile = self.config_profiles.get(profile_id)
            .ok_or(format!("Config profile {} not found.", profile_id))?;
        
        let command_id = format!("apply_cmd_{}", Uuid::new_v4());
        let mut command = ApplyConfigCommand {
            command_id: command_id.clone(),
            profile_id: profile_id.to_string(),
            target_instance_ids: target_instance_ids.clone(),
            initiated_by: initiated_by.to_string(),
            timestamp: Utc::now(),
            status: ApplyConfigStatus::Pending,
            results: HashMap::new(),
        };
        self.apply_commands.insert(command_id.clone(), command.clone());

        println!("[OmegaOSConfigManager] Applying profile '{}' to {} instances.", profile.name, target_instance_ids.len());

        // Simulate applying config to each instance
        let mut partial_success = false;
        for instance_id in &target_instance_ids {
            // In a real system, this would involve calling out to OmegaOSInterface for each instance
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let success = rand::random::<f32>() > 0.1; // 90% success rate
            if success {
                command.results.insert(instance_id.clone(), "Success: Configuration applied.".to_string());
            } else {
                command.results.insert(instance_id.clone(), "Failed: Instance unreachable or incompatible.".to_string());
                partial_success = true;
            }
        }

        command.status = if partial_success {
            ApplyConfigStatus::PartialSuccess
        } else {
            ApplyConfigStatus::Completed
        };

        self.apply_commands.insert(command_id.clone(), command.clone()); // Update command status

        Ok(command_id)
    }

    /// Retrieves the status of an apply config command.
    pub fn get_apply_command_status(&self, command_id: &str) -> Option<&ApplyConfigCommand> {
        self.apply_commands.get(command_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_and_apply_config_profile() {
        let mut config_manager = OmegaOSConfigManager::new();
        let profile_id = Uuid::new_v4().to_string();
        let profile = OmegaOSConfigProfile {
            profile_id: profile_id.clone(),
            name: "ProductionServerConfig".to_string(),
            description: "Standard config for production OMEGA OS servers.".to_string(),
            created_by_agi: true,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            parameters: HashMap::from([
                ("kernel.logging_level".to_string(), "info".to_string()),
                ("network.firewall_enabled".to_string(), "true".to_string()),
            ]),
            target_os_versions: vec!["0.1.0-beta".to_string(), "0.1.0-release".to_string()],
            associated_autonomy_policy_id: Some("AP_ProdSec".to_string()),
        };

        let created_profile_id = config_manager.create_profile(profile.clone()).unwrap();
        assert_eq!(created_profile_id, profile_id);
        assert!(config_manager.get_profile(&profile_id).is_some());

        let target_instances = vec!["os_instance_001".to_string(), "os_instance_002".to_string()];
        let command_id = config_manager.apply_config_profile(&profile_id, target_instances.clone(), "Tariro AGI").await.unwrap();
        assert!(!command_id.is_empty());

        let command_status = config_manager.get_apply_command_status(&command_id).unwrap();
        assert!(command_status.status == ApplyConfigStatus::Completed || command_status.status == ApplyConfigStatus::PartialSuccess);
        assert_eq!(command_status.target_instance_ids.len(), 2);
    }

    #[test]
    fn test_update_config_profile() {
        let mut config_manager = OmegaOSConfigManager::new();
        let profile_id = Uuid::new_v4().to_string();
        let profile = OmegaOSConfigProfile {
            profile_id: profile_id.clone(),
            name: "DevConfig".to_string(),
            description: "".to_string(),
            created_by_agi: false,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            parameters: HashMap::new(),
            target_os_versions: vec![],
            associated_autonomy_policy_id: None,
        };
        config_manager.create_profile(profile.clone()).unwrap();

        let mut updated_profile = config_manager.get_profile(&profile_id).unwrap().clone();
        updated_profile.name = "StagingConfig".to_string();
        updated_profile.parameters.insert("debug_mode".to_string(), "true".to_string());

        config_manager.update_profile(updated_profile.clone()).unwrap();
        let retrieved_profile = config_manager.get_profile(&profile_id).unwrap();
        assert_eq!(retrieved_profile.name, "StagingConfig");
        assert_eq!(retrieved_profile.parameters.get("debug_mode"), Some(&"true".to_string()));
    }
}
