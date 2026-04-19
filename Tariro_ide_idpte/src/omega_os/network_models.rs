// Tariro_ide_idpte/src/omega_os/network_models.rs

//! Defines data models for OMEGA OS network interfaces, routing, and secure channels.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a network interface within OMEGA OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub id: String,
    pub name: String, // e.g., "eth0", "wlan0", "lo"
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
    pub status: InterfaceStatus,
    pub type_name: InterfaceType,
    pub metrics: HashMap<String, f64>, // e.g., "rx_bytes", "tx_bytes", "errors"
}

/// Status of a network interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceStatus {
    Up,
    Down,
    Configuring,
    Error,
}

/// Type of network interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceType {
    Ethernet,
    Wifi,
    Loopback,
    Virtual,
    Quantum,
    Other(String),
}

/// Represents an entry in the routing table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTableEntry {
    pub destination: String, // IP network or host
    pub gateway: String,
    pub interface_id: String, // ID of the egress interface
    pub metric: u32, // Cost metric
    pub protocol: RoutingProtocol,
}

/// Type of routing protocol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingProtocol {
    Static,
    Ospf,
    Bgp,
    Rip,
    QuantumMesh,
    Other(String),
}

/// Information about a secure communication channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureChannelInfo {
    pub id: String,
    pub peer_id: String,
    pub established_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub current_pqc_alg: String, // e.g., "Kyber-768", "Dilithium-3"
    pub status: SecureChannelStatus,
    pub data_encrypted_bytes: u64,
    pub data_decrypted_bytes: u64,
    pub associated_network_interface_id: Option<String>,
}

/// Status of a secure channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecureChannelStatus {
    Establishing,
    Active,
    Degraded,
    Closed,
    Error,
}
