// Tariro_ide_idpte/src/environmental/bio_integration_models.rs

//! Defines data models for Tariro's Bio-integration & Symbiotic Systems module,
//! including biological interface configurations, bio-interaction records, and symbiotic protocols.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Configuration for ethically and securely interacting with biological systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalInterfaceConfig {
    pub id: String,
    pub interface_type: String, // e.g., "Neuralink", "CRISPR_system", "biosensor_array"
    pub target_organism_or_system: String,
    pub allowed_operations: Vec<String>, // e.g., "read_neural_signals", "gene_expression_modulation"
    pub safety_protocols: Vec<String>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of interface configuration
    pub security_verified: VerificationStatus, // New: Security verification of interface integrity
    pub created_at: DateTime<Utc>,
}

/// A record of an interaction between Tariro and a biological system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioInteractionRecord {
    pub id: String,
    pub interface_id: String,
    pub interaction_type: String, // e.g., "data_acquisition", "modulatory_output"
    pub data_exchanged_summary: String,
    pub observed_biological_response: String,
    pub ethical_compliance_after_interaction: VerificationStatus, // New: Ethical check after interaction
    pub security_integrity_after_interaction: VerificationStatus, // New: Security integrity of interaction data
    pub timestamp: DateTime<Utc>,
}

/// A set of rules and protocols for cooperative interaction between AGI and biological entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbioticProtocol {
    pub id: String,
    pub protocol_name: String,
    pub goals: Vec<String>, // e.g., "mutual_learning", "health_optimization"
    pub communication_channels: Vec<String>,
    pub ethical_constraints: Vec<String>, // e.g., "no_coercion", "privacy_of_biological_data"
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the protocol design
    pub security_audited: VerificationStatus, // New: Security audit of the protocol
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of bio-integration activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioIntegrationAudit {
    pub id: String,
    pub protocol_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_interactions: u64,
    pub detected_safety_breaches: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of bio-integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of bio-integration systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
