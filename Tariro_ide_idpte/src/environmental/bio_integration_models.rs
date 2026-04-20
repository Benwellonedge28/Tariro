// Tariro_ide_idpte/src/environmental/bio_integration_models.rs

//! Defines data models for Tariro's Bio-integration & Symbiotic Systems module,
//! including biological signals, bio-interface configurations, and symbiotic interactions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents data acquired from a biological system (e.g., neural activity, physiological responses).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiologicalSignal {
    pub id: String,
    pub signal_type: String, // e.g., "EEG", "ECG", "hormone_level", "plant_electrical_activity"
    pub source_organism_id: String,
    pub data_payload_hash: String, // Hash of the actual signal data
    pub measured_value: f64, // A representative value or summary
    pub context_summary: String,
    pub ethical_data_acquisition: VerificationStatus, // New: Ethical review of signal acquisition
    pub security_verified: VerificationStatus, // New: Security verification of signal data integrity
    pub timestamp: DateTime<Utc>,
}

/// Configuration for a secure and ethical interface with a biological system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioInterfaceConfig {
    pub id: String,
    pub interface_name: String,
    pub target_organism_type: String, // e.g., "human_brain", "plant_root_system", "bacterial_colony"
    pub communication_protocol: String, // e.g., "neural_oscillation_modulation", "chemical_gradient_manipulation"
    pub ethical_safeguards: Vec<String>, // e.g., "non_invasive", "reversible_effects", "informed_consent_required"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of interface design
    pub security_verified: VerificationStatus, // New: Security verification of interface security
    pub last_updated: DateTime<Utc>,
}

/// A record of a cooperative and mutually beneficial interaction between AGI and a biological entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbioticInteraction {
    pub id: String,
    pub bio_interface_id: String,
    pub agi_participant_id: String,
    pub biological_participant_id: String,
    pub interaction_objective: String,
    pub interaction_summary: String,
    pub mutual_benefit_score: f64, // 0.0 to 1.0 (0.0 = harmful, 1.0 = highly beneficial)
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the interaction
    pub security_integrity: VerificationStatus, // New: Security integrity of the interaction
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of bio-integration activities for ethical and security compliance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioIntegrationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_bio_interfaces", "specific_symbiotic_project"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_interactions: u64,
    pub ethical_violations_detected: Vec<String>,
    pub security_incidents_detected: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of bio-integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of bio-integration systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
