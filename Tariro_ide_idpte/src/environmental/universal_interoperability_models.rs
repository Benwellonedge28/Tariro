// Tariro_ide_idpte/src/environmental/universal_interoperability_models.rs

//! Defines data models for Tariro's Universal Interoperability & Omni-Protocol Autonomy module,
//! including protocol definitions, interoperability standards, and communication logs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A description of a communication protocol, digital or physical.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolDefinition {
    pub id: String,
    pub name: String,
    pub version: String,
    pub specification_url: Option<String>,
    pub data_formats_supported: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // New: Ethical review of protocol standards
    pub security_vulnerability_level: VerificationStatus, // New: Security assessment of protocol robustness
    pub created_at: DateTime<Utc>,
}

/// A standard or framework for enabling seamless communication between disparate systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteroperabilityStandard {
    pub id: String,
    pub standard_name: String,
    pub description: String,
    pub protocols_covered: Vec<String>, // List of ProtocolDefinition IDs
    pub compatibility_score: f64, // 0.0 to 1.0
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the standard's design
    pub security_audited: VerificationStatus, // New: Security audit of standard implementation
    pub last_updated: DateTime<Utc>,
}

/// A log of communication events, including protocol translation and data exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationLog {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub protocol_used: String,
    pub data_exchanged_hash: String, // Hash of data to ensure integrity without storing content
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the communication
    pub security_integrity: VerificationStatus, // New: Security integrity of the communication channel
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of universal interoperability processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteroperabilityAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_communications", "critical_system_interactions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_protocols_adapted: u64,
    pub total_communications_logged: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of interoperability
    pub security_integrity_status: VerificationStatus, // Overall security integrity of interoperability systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
