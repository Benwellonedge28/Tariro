// Tariro_ide_idpte/src/cognitive_arch/intuitive_common_sense_models.rs

//! Defines data models for Tariro's Intuitive Physics & Psychology Integration module,
//! including physical event models, mental state models, and common sense inferences.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A representation of Tariro's intuitive understanding of physical dynamics (e.g., gravity, collision).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalEventModel {
    pub id: String,
    pub object_id: String,
    pub interaction_type: String, // e.g., "free_fall", "elastic_collision", "sliding_friction"
    pub predicted_state_change: String,
    pub stability_score: f64, // 0.0 to 1.0
    pub ethical_safety_review: VerificationStatus, // New: Ethical review of the physical prediction safety
    pub generated_at: DateTime<Utc>,
}

/// A representation of Tariro's intuitive inference about a human's mental state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalStateModel {
    pub id: String,
    pub human_id: String,
    pub inferred_belief: String,
    pub inferred_desire: String,
    pub inferred_intention: String,
    pub confidence_score: f64, // 0.0 to 1.0
    pub ethical_privacy_review: VerificationStatus, // New: Ethical review for privacy violations
    pub generated_at: DateTime<Utc>,
}

/// A reasoning outcome derived from Tariro's integrated intuitive knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonSenseInference {
    pub id: String,
    pub query: String,
    pub inference_summary: String,
    pub physical_basis_id: Option<String>, // Link to PhysicalEventModel
    pub psychological_basis_id: Option<String>, // Link to MentalStateModel
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of the common-sense reasoning
    pub reasoning_pathway: Vec<String>, // New: Steps in deriving the common sense outcome
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of intuitive physics and psychology integration processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonSenseAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_common_sense_decisions", "human_interaction_logic"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_physical_predictions: u64,
    pub total_mental_inferences: u64,
    pub detected_common_sense_violations: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
