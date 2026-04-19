// Tariro_ide_idpte/src/security/hardware_security_models.rs

//! Defines data models for Tariro's Secure Hardware Integration module,
//! including hardware attestation reports, peripheral security policies,
//! and ethical hardware access records.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a report from an autonomous hardware attestation process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareAttestationReport {
    pub id: String,
    pub hardware_component_id: String,
    pub attestation_result: String, // e.g., "Verified", "Tampered", "Mismatch"
    pub firmware_version: String,
    pub last_known_good_hash: String, // Hash of expected firmware/state
    pub current_hash: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of attestation process/outcome
    pub security_verified: VerificationStatus, // New: Security of the attestation mechanism itself
    pub timestamp: DateTime<Utc>,
}

/// A policy defining secure access and interaction rules for a physical peripheral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeripheralSecurityPolicy {
    pub id: String,
    pub peripheral_id: String,
    pub policy_name: String,
    pub access_rules: HashMap<String, String>, // e.g., "read_permission": "AGICore_only", "write_permission": "ALEC-A_approved"
    pub monitoring_level: String, // e.g., "high", "medium", "low"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the policy
    pub security_review_status: VerificationStatus, // New: Security review of the policy
    pub last_updated: DateTime<Utc>,
}

/// A secure log entry for interactions with physical hardware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInteractionLog {
    pub id: String,
    pub component_id: String,
    pub interaction_type: String, // e.g., "read_sensor", "actuate_robot_arm", "power_cycle"
    pub initiating_module_id: String,
    pub interaction_details: String,
    pub security_verified: VerificationStatus, // New: Security verification of the interaction
    pub ethical_compliance_status: VerificationStatus, // New: Ethical compliance of the interaction
    pub timestamp: DateTime<Utc>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
}

/// A record summarizing the ethical assessment of hardware access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalHardwareAccessRecord {
    pub id: String,
    pub interaction_log_id: String,
    pub ethical_impact_score: f64, // 0.0 (negative) to 1.0 (positive)
    pub identified_risks: Vec<String>,
    pub recommendations: Vec<String>,
    pub ethical_review_status: VerificationStatus, // Overall ethical status of access
    pub generated_at: DateTime<Utc>,
}
