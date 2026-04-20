// Tariro_ide_idpte/src/cognitive_arch/agi_behavior_models.rs

//! Defines data models for Tariro's Predictive Modeling of AGI Behavior module,
//! including behavior predictions, internal state forecasts, and self-correction actions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A forecast of Tariro's own future actions or decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPrediction {
    pub id: String,
    pub predicted_action: String,
    pub confidence: f64, // 0.0 to 1.0
    pub context_summary: String,
    pub influencing_factors: Vec<String>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of predicted action
    pub security_verified: VerificationStatus, // New: Security verification of prediction model integrity
    pub generated_at: DateTime<Utc>,
}

/// A prediction of Tariro's future internal cognitive or emotional state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalStateForecast {
    pub id: String,
    pub predicted_state: String, // e.g., "high_cognitive_load", "curiosity_level_rising", "goal_conflict_detected"
    pub confidence: f64,
    pub influencing_factors: Vec<String>,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical assessment of internal state implications
    pub security_integrity: VerificationStatus, // New: Security integrity of internal state data
    pub generated_at: DateTime<Utc>,
}

/// An action taken by Tariro to adjust its own predicted behavior or internal state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCorrectionAction {
    pub id: String,
    pub prediction_id: String, // Link to the BehaviorPrediction or InternalStateForecast it responds to
    pub action_type: String, // e.g., "adjust_goal_priority", "initiate_self_healing", "request_human_oversight"
    pub rationale_summary: String,
    pub expected_outcome: String,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the self-correction action
    pub security_integrity: VerificationStatus, // New: Security integrity of the self-correction process
    pub executed_at: DateTime<Utc>,
}

/// A report summarizing the audit of Tariro's self-modeling processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModelingAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_predictions", "deviations_only"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_predictions_generated: u64,
    pub total_self_corrections: u64,
    pub detected_prediction_biases: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of self-modeling
    pub security_integrity_status: VerificationStatus, // Overall security integrity of self-modeling systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
