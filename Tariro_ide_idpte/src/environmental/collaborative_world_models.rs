// Tariro_ide_idpte/src/environmental/collaborative_world_models.rs

//! Defines data models for Tariro's Collaborative World Modeling module,
//! including shared world models, model alignment metrics, and consensus mechanisms.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A coherent, joint representation of the environment shared among multiple agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedWorldModel {
    pub id: String,
    pub model_name: String,
    pub participants: Vec<String>, // IDs of agents contributing to the model
    pub current_state_summary: String,
    pub consistency_score: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the model's implications
    pub security_integrity: VerificationStatus, // New: Security verification of model data
    pub last_updated: DateTime<Utc>,
}

/// A metric quantifying the consistency and coherence between different world models or perspectives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelAlignmentMetric {
    pub id: String,
    pub model_id_1: String,
    pub model_id_2: String,
    pub alignment_score: f64, // 0.0 to 1.0
    pub discrepancy_details: Vec<String>,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical assessment of misalignment
    pub security_vulnerability: VerificationStatus, // New: Security assessment of divergence
    pub measured_at: DateTime<Utc>,
}

/// A mechanism or protocol for achieving agreement on aspects of the shared world model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMechanism {
    pub id: String,
    pub mechanism_name: String,
    pub description: String,
    pub rules: Vec<String>, // e.g., "majority_vote", "expert_override", "probabilistic_fusion"
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the consensus mechanism
    pub security_audited: VerificationStatus, // New: Security audit of mechanism implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of collaborative world modeling processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_shared_models", "critical_environmental_models"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_consensus_events: u64,
    pub detected_manipulation_attempts: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of collaborative modeling
    pub security_integrity_status: VerificationStatus, // Overall security integrity of collaborative modeling systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
