// Tariro_ide_idpte/src/environmental/world_modeling_models.rs

//! Defines data models for Tariro's Collaborative World Modeling module,
//! including shared world models, model alignment metrics, and collaborative observations.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A representation of the world agreed upon and maintained by multiple agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedWorldModel {
    pub id: String,
    pub model_name: String,
    pub description: String,
    pub participants: Vec<String>, // IDs of participating agents
    pub current_state_hash: String, // Hash of the current model state for integrity check
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the world model's biases
    pub security_integrity: VerificationStatus, // New: Security verification of model tampering
    pub last_updated: DateTime<Utc>,
}

/// A metric used to assess how well different agents' internal world models align with each other.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAlignmentMetric {
    pub id: String,
    pub model_id: String,
    pub alignment_score: f64, // 0.0 (no alignment) to 1.0 (perfect alignment)
    pub discrepancies_detected: Vec<String>,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical assessment of misalignment
    pub security_risk_level: VerificationStatus, // New: Security risk assessment of model divergence
    pub evaluated_at: DateTime<Utc>,
}

/// An observation or data point contributed by an agent to the shared world model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeObservation {
    pub id: String,
    pub world_model_id: String,
    pub observer_id: String,
    pub observation_content_summary: String,
    pub confidence: f64,
    pub ethical_data_contribution: VerificationStatus, // New: Ethical review of data contribution
    pub security_data_provenance: VerificationStatus, // New: Security verification of data origin
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of collaborative world modeling processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelingAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_world_models", "specific_domain_models"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_models_maintained: u64,
    pub total_discrepancies_resolved: u64,
    pub detected_manipulation_attempts: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of world modeling
    pub security_integrity_status: VerificationStatus, // Overall security integrity of world modeling systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
