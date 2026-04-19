// Tariro_ide_idpte/src/cognitive_arch/embodied_models.rs

//! Defines data models for Tariro's Embodied Cognition & Sensorimotor Learning module,
//! including sensorimotor experiences, physical capabilities, action feedback, and embodied states.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents data from a single physical interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorimotorExperience {
    pub id: String,
    pub sensor_data: HashMap<String, f64>, // e.g., "camera_depth": 1.2, "tactile_pressure": 0.5
    pub action_taken: String,
    pub action_parameters: HashMap<String, f64>,
    pub outcome_observation: String,
    pub surprise_score: f64, // 0.0 to 1.0, difference between predicted and actual outcome
    pub learned_associations: Vec<String>, // IDs of concepts or physical laws updated
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of the interaction
    pub security_verified: VerificationStatus, // New: Security of sensorimotor data processing
    pub timestamp: DateTime<Utc>,
}

/// Represents a specific physical capability of Tariro (actuator or sensor).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalCapability {
    pub id: String,
    pub name: String,
    pub description: String,
    pub component_id: String, // Link to HardwareSecurityModule record
    pub range_of_motion_or_detection: HashMap<String, (f64, f64)>, // min, max
    pub precision: f64,
    pub ethical_constraints: Vec<String>, // e.g., "max_force_near_humans", "privacy_sensitive_sensor"
    pub status: String, // e.g., "online", "calibrating", "faulty", "offline"
}

/// Feedback from an executed physical action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionFeedback {
    pub id: String,
    pub action_id: String,
    pub success_rate: f64, // 0.0 to 1.0
    pub measured_errors: HashMap<String, f64>,
    pub external_forces_detected: Vec<String>,
    pub hardware_health_status: VerificationStatus, // New: Status of hardware after action
    pub security_integrity_verified: VerificationStatus, // New: Integrity of feedback data
    pub timestamp: DateTime<Utc>,
}

/// Represents the current physical state of the AGI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbodiedState {
    pub id: String,
    pub location_summary: String,
    pub orientation: Vec<f64>, // [pitch, roll, yaw]
    pub internal_battery_or_energy: f64,
    pub active_capabilities: Vec<String>, // IDs of PhysicalCapabilities currently in use
    pub environment_map_hash: String, // Hash of current spatial representation
    pub health_and_safety_status: VerificationStatus, // Overall physical health and safety status
    pub last_updated: DateTime<Utc>,
}
