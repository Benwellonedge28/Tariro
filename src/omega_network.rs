// src/omega_network.rs

//! Implements the OMEGA Network capabilities for Tariro AGI, enabling secure,
//! resilient, and intelligent communication within distributed AGI deployments
//! and across heterogeneous computational substrates.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};

/// Represents a node within the OMEGA Network.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmegaNode {
    pub id: String,
    pub alias: String,
    pub node_type: OmegaNodeType,
    pub status: OmegaNodeStatus,
    pub ip_address: Option<String>,
    pub last_seen: DateTime<Utc>,
    pub capabilities: Vec<String>, // e.g., "compute", "storage", "sensor_access", "agent_hosting"
    pub assigned_swarms: Vec<String>,
    pub resource_availability: ResourceAvailability,
}

/// Types of nodes within the OMEGA Network.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OmegaNodeType {
    Compute,
    Storage,
    Sensor,
    Gateway,
    AGIHost,
    QuantumAnchor,
    BioInterface,
    Custom(String),
}

/// Operational status of an OMEGA Network node.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OmegaNodeStatus {
    Online,
    Offline,
    Degraded,
    Overloaded,
    UnderMaintenance,
}

/// Represents the available resources of an OMEGA Network node.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceAvailability {
    pub cpu_usage_pct: f32,
    pub memory_usage_pct: f32,
    pub storage_free_gb: f32,
    pub network_latency_ms: f32,
}

/// Represents a secure, intelligent communication channel within the OMEGA Network.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmegaChannel {
    pub id: String,
    pub name: String,
    pub channel_type: OmegaChannelType,
    pub participants: Vec<String>, // Node IDs or Agent IDs
    pub encryption_level: EncryptionLevel,
    pub security_policy_id: String, // Reference to an AutonomyPolicy/SecurityPolicy
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

/// Types of communication channels.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum OmegaChannelType {
    PeerToPeer,
    SwarmBroadcast,
    SecureLogStream,
    CommandAndControl,
    DataStream,
    SensorFeed,
}

/// Levels of encryption used in OMEGA Channels.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EncryptionLevel {
    None,
    Light,
    Standard,
    QuantumResistant,
    PerfectForwardSecrecy,
}

/// The `OmegaNetworkService` manages the OMEGA Network nodes, channels, and communication.
pub struct OmegaNetworkService {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    nodes: HashMap<String, OmegaNode>,
    channels: HashMap<String, OmegaChannel>,
}

impl OmegaNetworkService {
    /// Creates a new `OmegaNetworkService` instance.
    pub fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self {
        OmegaNetworkService {
            config,
            telemetry_logger,
            nodes: HashMap::new(),
            channels: HashMap::new(),
        }
    }

    /// Registers a new node with the OMEGA Network.
    pub async fn register_node(&mut self, mut node: OmegaNode) -> Result<String, String> {
        node.id = format!("node_{}", Uuid::new_v4());
        node.last_seen = Utc::now();
        node.status = OmegaNodeStatus::Online;

        if self.nodes.contains_key(&node.id) {
            return Err(format!("Node with ID {} already exists.", node.id));
        }
        println!("[OmegaNetwork] Registering node: {}", node.alias);
        self.telemetry_logger.log_event(TelemetryEvent::OmegaNodeRegistered {
            node_id: node.id.clone(),
            node_type: format!("{:?}", node.node_type),
        }).await;
        self.nodes.insert(node.id.clone(), node.clone());
        Ok(node.id)
    }

    /// Updates the status or information of an existing OMEGA Node.
    pub async fn update_node_status(&mut self, node_id: &str, status: OmegaNodeStatus) -> Result<(), String> {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.status = status;
            node.last_seen = Utc::now();
            self.telemetry_logger.log_event(TelemetryEvent::OmegaNodeStatusUpdated {
                node_id: node_id.to_string(),
                new_status: format!("{:?}", status),
            }).await;
            Ok(())
        } else {
            Err(format!("Node with ID {} not found.", node_id))
        }
    }

    /// Establishes a new secure communication channel.
    pub async fn create_channel(&mut self, mut channel: OmegaChannel) -> Result<String, String> {
        channel.id = format!("chan_{}", Uuid::new_v4());
        channel.created_at = Utc::now();
        channel.last_activity = Utc::now();

        if self.channels.contains_key(&channel.id) {
            return Err(format!("Channel with ID {} already exists.", channel.id));
        }
        println!("[OmegaNetwork] Creating channel: {}", channel.name);
        self.telemetry_logger.log_event(TelemetryEvent::OmegaChannelCreated {
            channel_id: channel.id.clone(),
            channel_type: format!("{:?}", channel.channel_type),
        }).await;
        self.channels.insert(channel.id.clone(), channel.clone());
        Ok(channel.id)
    }

    /// Sends data through a specified OMEGA Channel.
    pub async fn send_data(&self, channel_id: &str, data: &str) -> Result<(), String> {
        if let Some(channel) = self.channels.get(channel_id) {
            println!("[OmegaNetwork] Sending {} bytes over channel {}. Enc: {:?}", 
                     data.len(), channel.name, channel.encryption_level);
            self.telemetry_logger.log_event(TelemetryEvent::OmegaDataTransferred {
                channel_id: channel_id.to_string(),
                data_size_bytes: data.len() as u64,
            }).await;
            // Simulate secure transmission
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            Ok(())
        } else {
            Err(format!("Channel with ID {} not found.", channel_id))
        }
    }

    /// Periodically checks the health and performance of network nodes.
    pub async fn monitor_network_health(&self) {
        println!("[OmegaNetwork] Monitoring network health...");
        self.telemetry_logger.log_event(TelemetryEvent::PeriodicCheck {
            check_name: "OmegaNetworkHealthMonitor".to_string(),
        }).await;
        for node in self.nodes.values() {
            if node.status == OmegaNodeStatus::Offline {
                eprintln!("WARNING: OMEGA Node {} is offline.", node.alias);
                self.telemetry_logger.log_event(TelemetryEvent::SystemHealthWarning {
                    component: format!("OmegaNode:{}", node.id),
                    level: "Critical".to_string(),
                    message: "Node reported offline.".to_string(),
                }).await;
            } else if node.resource_availability.cpu_usage_pct > 90.0 {
                println!("INFO: OMEGA Node {} is under heavy load (CPU: {}%).", node.alias, node.resource_availability.cpu_usage_pct);
                self.telemetry_logger.log_event(TelemetryEvent::SystemHealthWarning {
                    component: format!("OmegaNode:{}", node.id),
                    level: "Warning".to_string(),
                    message: format!("Node high CPU usage: {}%.", node.resource_availability.cpu_usage_pct),
                }).await;
            }
        }
    }

    /// Discovers new potential nodes or resources for the OMEGA Network.
    pub async fn discover_new_resources(&self) -> Result<Vec<String>, String> {
        println!("[OmegaNetwork] Discovering new resources...");
        self.telemetry_logger.log_event(TelemetryEvent::AGIAction {
            action_type: "ResourceDiscovery".to_string(),
            details: Some("Scanning for potential OMEGA Network nodes.".to_string()),
        }).await;
        // Simulate discovery process
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(vec!["potential_node_A".to_string(), "quantum_chip_B".to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    fn setup_services() -> (OmegaNetworkService, mpsc::Receiver<TelemetryEvent>) {
        let config = AGIConfig::default();
        let (tx, rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx);
        (OmegaNetworkService::new(config, telemetry_logger), rx)
    }

    #[tokio::test]
    async fn test_register_and_update_node() {
        let (mut service, mut rx) = setup_services();
        let node = OmegaNode {
            id: String::new(), // Will be overwritten
            alias: "EdgeCompute_01".to_string(),
            node_type: OmegaNodeType::Compute,
            status: OmegaNodeStatus::Offline,
            ip_address: Some("192.168.1.100".to_string()),
            last_seen: Utc::now(),
            capabilities: vec!["heavy_compute".to_string()],
            assigned_swarms: vec![],
            resource_availability: ResourceAvailability { cpu_usage_pct: 0.0, memory_usage_pct: 0.0, storage_free_gb: 1000.0, network_latency_ms: 10.0 },
        };

        let node_id = service.register_node(node.clone()).await.unwrap();
        assert!(!node_id.is_empty());
        let registered_node = service.nodes.get(&node_id).unwrap();
        assert_eq!(registered_node.status, OmegaNodeStatus::Online);

        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::OmegaNodeRegistered { node_id: event_node_id, node_type } => {
                assert_eq!(event_node_id, node_id);
                assert_eq!(node_type, "Compute");
            },
            _ => panic!("Unexpected telemetry event"),
        }

        service.update_node_status(&node_id, OmegaNodeStatus::Degraded).await.unwrap();
        assert_eq!(service.nodes.get(&node_id).unwrap().status, OmegaNodeStatus::Degraded);

        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::OmegaNodeStatusUpdated { node_id: event_node_id, new_status } => {
                assert_eq!(event_node_id, node_id);
                assert_eq!(new_status, "Degraded");
            },
            _ => panic!("Unexpected telemetry event"),
        }
    }

    #[tokio::test]
    async fn test_create_and_send_data_channel() {
        let (mut service, mut rx) = setup_services();
        let channel = OmegaChannel {
            id: String::new(),
            name: "SecureComm_AGI-Edge".to_string(),
            channel_type: OmegaChannelType::PeerToPeer,
            participants: vec!["agi_core_01".to_string(), "edge_node_01".to_string()],
            encryption_level: EncryptionLevel::QuantumResistant,
            security_policy_id: "policy_q_sec_001".to_string(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
        };

        let channel_id = service.create_channel(channel.clone()).await.unwrap();
        assert!(!channel_id.is_empty());
        assert!(service.channels.get(&channel_id).is_some());

        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::OmegaChannelCreated { channel_id: event_channel_id, channel_type } => {
                assert_eq!(event_channel_id, channel_id);
                assert_eq!(channel_type, "PeerToPeer");
            },
            _ => panic!("Unexpected telemetry event"),
        }

        let test_data = "Hello Omega Network!";
        service.send_data(&channel_id, test_data).await.unwrap();

        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::OmegaDataTransferred { channel_id: event_channel_id, data_size_bytes } => {
                assert_eq!(event_channel_id, channel_id);
                assert_eq!(data_size_bytes, test_data.len() as u64);
            },
            _ => panic!("Unexpected telemetry event"),
        }
    }
}
