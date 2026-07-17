// src/omega_os/os_interface.rs
use crate::prelude::*;

//! Defines interfaces and communication protocols for Tariro AGI to interact with
//! and manage OMEGA OS instances, whether simulated or deployed on hardware.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a deployed or simulated instance of OMEGA OS.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmegaOSInstance {
    pub instance_id: String,
    pub alias: String,
    pub deployment_target: String, // e.g., "EdgeDevice:NODE_01", "Simulated:VM_7"
    pub status: OmegaOSInstanceStatus,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime_seconds: u64,
    pub last_heartbeat: DateTime<Utc>,
    pub resource_telemetry: ResourceTelemetry,
    pub security_posture: SecurityPosture,
    pub linked_agi_agents: Vec<String>, // IDs of NanoAgents running on this OS
}

/// Operational status of an OMEGA OS instance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OmegaOSInstanceStatus {
    Booting,
    Running,
    Degraded,
    Halted,
    Crashed,
    Updating,
}

/// Telemetry data from an OMEGA OS instance.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceTelemetry {
    pub cpu_load_pct: f32,
    pub memory_usage_gb: f32,
    pub disk_usage_pct: f32,
    pub network_in_kbps: u64,
    pub network_out_kbps: u64,
}

/// Security posture of an OMEGA OS instance.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityPosture {
    pub secure_boot_status: SecureBootStatus,
    pub evas_compliance_score: f32,
    pub active_threats: Vec<String>,
    pub last_security_scan: DateTime<Utc>,
}

/// Status of the Secure Boot process on an OMEGA OS instance.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SecureBootStatus {
    Verified,
    Compromised,
    Disabled,
    Unsupported,
}

/// The `OmegaOSInterface` service allows Tariro AGI to interact with OMEGA OS instances.
pub struct OmegaOSInterface {
    // This service would manage connections to various OMEGA OS instances
    // and expose methods for remote management.
    active_instances: HashMap<String, OmegaOSInstance>,
}

impl OmegaOSInterface {
    /// Creates a new `OmegaOSInterface`.
    pub fn new() -> Self {
        OmegaOSInterface {
            active_instances: HashMap::new(),
        }
    }

    /// Registers an OMEGA OS instance with the AGI.
    pub fn register_instance(&mut self, instance: OmegaOSInstance) -> Result<(), String> {
        if self.active_instances.contains_key(&instance.instance_id) {
            return Err(format!("OMEGA OS instance with ID {} already exists.", instance.instance_id));
        }
        println!("[OmegaOSInterface] Registering OMEGA OS instance: {}", instance.alias);
        self.active_instances.insert(instance.instance_id.clone(), instance);
        Ok(())
    }

    /// Retrieves an OMEGA OS instance by its ID.
    pub fn get_instance(&self, instance_id: &str) -> Option<&OmegaOSInstance> {
        self.active_instances.get(instance_id)
    }

    /// Sends a command to an OMEGA OS instance.
    pub async fn send_command(
        &self,
        instance_id: &str,
        command: &str,
        args: HashMap<String, String>,
    ) -> Result<String, String> {
        if let Some(instance) = self.active_instances.get(instance_id) {
            println!("[OmegaOSInterface] Sending command '{}' to OS instance {}.", command, instance.alias);
            // Simulate command execution and response
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            match command {
                "reboot" => Ok(format!("Initiated reboot on {}.", instance.alias)),
                "install_package" => {
                    if let Some(pkg) = args.get("package_name") {
                        Ok(format!("Package '{}' installation started on {}.", pkg, instance.alias))
                    } else {
                        Err("Package name required for install_package.".to_string())
                    }
                },
                "query_status" => Ok(format!("Status of {}: Running, Uptime: {}s, CPU Load: {}%.", instance.alias, instance.uptime_seconds, instance.resource_telemetry.cpu_load_pct)),
                _ => Err(format!("Unknown command: {}", command)),
            }
        } else {
            Err(format!("OMEGA OS instance with ID {} not found.", instance_id))
        }
    }

    /// Receives telemetry from an OMEGA OS instance and updates its state.
    pub fn receive_telemetry(&mut self, instance_id: &str, telemetry: ResourceTelemetry, security: SecurityPosture) -> Result<(), String> {
        if let Some(instance) = self.active_instances.get_mut(instance_id) {
            instance.last_heartbeat = Utc::now();
            instance.uptime_seconds += 5; // Simulate uptime increase
            instance.resource_telemetry = telemetry;
            instance.security_posture = security;
            Ok(())
        } else {
            Err(format!("OMEGA OS instance with ID {} not found.", instance_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_and_command_instance() {
        let mut os_interface = OmegaOSInterface::new();
        let instance_id = Uuid::new_v4().to_string();
        let instance = OmegaOSInstance {
            instance_id: instance_id.clone(),
            alias: "MyEdgeServer".to_string(),
            deployment_target: "EdgeDevice:NODE_01".to_string(),
            status: OmegaOSInstanceStatus::Running,
            os_version: "0.1.0".to_string(),
            kernel_version: "0.1.0-alpha".to_string(),
            uptime_seconds: 3600,
            last_heartbeat: Utc::now(),
            resource_telemetry: ResourceTelemetry {
                cpu_load_pct: 15.5,
                memory_usage_gb: 4.2,
                disk_usage_pct: 30.1,
                network_in_kbps: 1200,
                network_out_kbps: 800,
            },
            security_posture: SecurityPosture {
                secure_boot_status: SecureBootStatus::Verified,
                evas_compliance_score: 0.98,
                active_threats: vec![],
                last_security_scan: Utc::now(),
            },
            linked_agi_agents: vec![],
        };

        os_interface.register_instance(instance.clone()).unwrap();
        let retrieved_instance = os_interface.get_instance(&instance_id).unwrap();
        assert_eq!(retrieved_instance.alias, "MyEdgeServer");

        let command_result = os_interface.send_command(&instance_id, "reboot", HashMap::new()).await.unwrap();
        assert!(command_result.contains("Initiated reboot"));

        let mut args = HashMap::new();
        args.insert("package_name".to_string(), "omega-utils".to_string());
        let install_result = os_interface.send_command(&instance_id, "install_package", args).await.unwrap();
        assert!(install_result.contains("installation started"));
    }

    #[test]
    fn test_receive_telemetry() {
        let mut os_interface = OmegaOSInterface::new();
        let instance_id = Uuid::new_v4().to_string();
        let instance = OmegaOSInstance {
            instance_id: instance_id.clone(),
            alias: "TelemetryNode".to_string(),
            deployment_target: "Simulated:VM_1".to_string(),
            status: OmegaOSInstanceStatus::Running,
            os_version: "0.1.0".to_string(),
            kernel_version: "0.1.0-alpha".to_string(),
            uptime_seconds: 100,
            last_heartbeat: Utc::now(),
            resource_telemetry: ResourceTelemetry {
                cpu_load_pct: 10.0,
                memory_usage_gb: 2.0,
                disk_usage_pct: 20.0,
                network_in_kbps: 100,
                network_out_kbps: 50,
            },
            security_posture: SecurityPosture {
                secure_boot_status: SecureBootStatus::Verified,
                evas_compliance_score: 0.95,
                active_threats: vec![],
                last_security_scan: Utc::now(),
            },
            linked_agi_agents: vec![],
        };
        os_interface.register_instance(instance).unwrap();

        let new_telemetry = ResourceTelemetry {
            cpu_load_pct: 50.0,
            memory_usage_gb: 3.5,
            disk_usage_pct: 25.0,
            network_in_kbps: 500,
            network_out_kbps: 300,
        };
        let new_security = SecurityPosture {
            secure_boot_status: SecureBootStatus::Verified,
            evas_compliance_score: 0.90,
            active_threats: vec!["suspicious_process".to_string()],
            last_security_scan: Utc::now(),
        };

        os_interface.receive_telemetry(&instance_id, new_telemetry.clone(), new_security.clone()).unwrap();

        let updated_instance = os_interface.get_instance(&instance_id).unwrap();
        assert_eq!(updated_instance.resource_telemetry.cpu_load_pct, 50.0);
        assert_eq!(updated_instance.security_posture.active_threats.len(), 1);
    }
}
