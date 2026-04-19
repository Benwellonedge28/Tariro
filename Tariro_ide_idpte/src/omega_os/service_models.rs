// Tariro_ide_idpte/src/omega_os/service_models.rs

//! Defines data models for OMEGA OS internal services, their capabilities, and status.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a core OMEGA OS service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsService {
    pub id: String,
    pub name: String,
    pub description: String,
    pub service_type: ServiceType,
    pub capabilities: Vec<ServiceCapability>, // What this service can do
    pub status: ServiceStatus,
    pub registered_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub endpoints: HashMap<String, String>, // e.g., "grpc_api": "127.0.0.1:50051", "rest_api": "/api/v1/service"
    pub dependencies: Vec<String>, // IDs of other services this one depends on
    pub managed_by_agi_module: Option<String>, // e.g., "SORAEE", "ALEC-A"
}

/// Categorization of OMEGA OS services.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    SystemCore,
    ResourceManagement,
    Security,
    Cryptography,
    Networking,
    Storage,
    ProcessManagement,
    AI_Core,
    AGI_Module(String), // e.g., "HACSE", "ALEC-A"
    Application(String), // User-facing applications
    Driver(String),
    Utility,
    Other(String),
}

/// A specific capability offered by a service.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceCapability {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, String>, // Expected input parameters
    pub output_schema: String, // Expected output schema (e.g., JSON schema, protobuf definition)
    pub requires_permissions: Vec<String>,
    pub requires_ethical_review: bool,
}

/// Current operational status of a service.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceStatus {
    Running,
    Paused,
    Stopped,
    Degraded,
    Error,
    Initializing,
}
