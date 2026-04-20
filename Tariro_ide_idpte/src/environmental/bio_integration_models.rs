// Tariro_ide_idpte/src/environmental/bio_integration_models.rs

//! Defines data models for Tariro's Bio-integration & Symbiotic Systems module,
//! including bio-interface configurations, symbiotic interaction records, and bio-ethical guidelines.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Configuration for interfacing with biological systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioInterfaceConfig {
    pub id: String,
    pub interface_type: String, // e.g., "neural_link", "prosthetic_control", "environmental_sensor_array"
    pub target_organism_type: String, // e.g., "human", "plant", "microbe"
    pub communication_protocol: String,
    pub safety_protocols: Vec<String>, // e.g., "fail_safe_shutdown", "minimum_power_output"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the interface design
    pub security_verified: VerificationStatus, // New: Security verification of interface integrity
    pub created_at: DateTime<Utc>,
}

/// A record of real-time or historical symbiotic interactions between Tariro and biological entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbioticInteractionRecord {
    pub id: String,
    pub interface_id: String,
    pub interacting_entity_id: String, // ID of the biological entity
    pub interaction_type: String, // e.g., "data_exchange", "actuator_control", "environmental_modulation"
    pub observed_biological_response: String,
    pub agi_action_summary: String,
    pub ethical_compliance_after_interaction: VerificationStatus, // New: Ethical check of the interaction
    pub security_integrity_after_interaction: VerificationStatus, // New: Security integrity of interaction data
    pub timestamp: DateTime<Utc>,
}

/// Predefined rules or principles for ethical bio-integration and interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioEthicalGuideline {
    pub id: String,
    pub guideline_name: String,
    pub description: String,
    pub applicability_scope: String, // e.g., "all_human_interaction", "microbial_ecosystems"
    pub core_principles: Vec<String>, // e.g., "do_no_harm", "respect_autonomy", "enhance_wellbeing"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the guideline itself
    pub security_verified: VerificationStatus, // New: Security of guideline data integrity
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of bio-integration activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioIntegrationAudit {
    pub id: String,
    pub scope: String, // e.g., "all_bio_interfaces", "specific_symbiotic_project"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_interactions: u64,
    pub detected_violations: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of bio-integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of bio-integration systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
