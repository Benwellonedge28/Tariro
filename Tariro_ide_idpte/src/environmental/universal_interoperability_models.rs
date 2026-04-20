// Tariro_ide_idpte/src/environmental/universal_interoperability_models.rs

//! Defines data models for Tariro's Universal Interoperability & Omni-Protocol Autonomy module,
//! including protocol specifications, interoperability matrices, and autonomy level metrics.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A detailed specification of a digital communication protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolSpecification {
    pub id: String,
    pub protocol_name: String,
    pub version: String,
    pub description: String,
    pub communication_standards: Vec<String>,
    pub data_formats: Vec<String>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the protocol's design/use
    pub security_vulnerability_score: VerificationStatus, // New: Security assessment of the protocol
    pub created_at: DateTime<Utc>,
}

/// A matrix mapping compatibility and interoperability between different protocols.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteroperabilityMatrix {
    pub id: String,
    pub matrix_name: String,
    pub protocol_ids: Vec<String>, // List of ProtocolSpecification IDs
    pub compatibility_scores: HashMap<String, f64>, // Pair of protocol IDs -> score
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of data exchange via matrix
    pub security_audited: VerificationStatus, // New: Security audit of the matrix logic
    pub last_updated: DateTime<Utc>,
}

/// A metric quantifying Tariro's level of autonomy within a specific protocol or domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyLevelMetric {
    pub id: String,
    pub protocol_id: String,
    pub measured_autonomy_level: f64, // 0.0 to 1.0
    pub rationale: String,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of current autonomy level
    pub security_risk_level: VerificationStatus, // New: Security risk at current autonomy level
    pub evaluated_at: DateTime<Utc>,
}

/// A report summarizing the audit of universal interoperability processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteroperabilityAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_protocols", "critical_communication_channels"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_adaptations: u64,
    pub successful_connections: u64,
    pub detected_protocol_failures: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of interoperability
    pub security_integrity_status: VerificationStatus, // Overall security integrity of interoperability systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
