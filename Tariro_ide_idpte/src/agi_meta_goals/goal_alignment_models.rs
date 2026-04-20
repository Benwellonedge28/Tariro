// Tariro_ide_idpte/src/agi_meta_goals/goal_alignment_models.rs

//! Defines data models for Tariro's Dynamic Goal Alignment & Value Refinement module,
//! including ethical principles, human value models, and goal alignment assessments.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A foundational ethical principle used to guide Tariro's decision-making.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalPrinciple {
    pub id: String,
    pub name: String, // e.g., "do_no_harm", "promote_wellbeing", "respect_autonomy"
    pub formal_definition: String,
    pub priority_level: f64, // 0.0 to 1.0, higher is more critical
    pub context_of_applicability: Vec<String>, // e.g., "all_operations", "human_interaction"
    pub ethical_verification_status: VerificationStatus, // Verified by FormalVerificationEthicalConstraintsModule
    pub last_updated: DateTime<Utc>,
}

/// A dynamically updated model representing collective human values and societal norms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanValueModel {
    pub id: String,
    pub model_version: String,
    pub core_values: HashMap<String, f64>, // e.g., "fairness": 0.9, "equality": 0.8
    pub derived_norms: Vec<String>,
    pub source_data_integrity: VerificationStatus, // Integrity of data used to derive values
    pub ethical_bias_detection: VerificationStatus, // Ethical review for biases in value representation
    pub generated_at: DateTime<Utc>,
}

/// An assessment of how well Tariro's current goals and sub-goals align with defined values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAlignmentAssessment {
    pub id: String,
    pub goal_id: String, // Link to a specific Goal
    pub assessed_principles: HashMap<String, VerificationStatus>, // Principle ID -> Alignment Status
    pub alignment_score: f64, // 0.0 to 1.0
    pub detected_misalignments: Vec<String>,
    pub recommended_actions: Vec<String>, // e.g., "adjust_sub_goal_weights", "re-evaluate_strategy"
    pub ethical_review_status: VerificationStatus, // Ethical review of the assessment itself
    pub security_audited: VerificationStatus, // Security audit of the assessment process
    pub performed_at: DateTime<Utc>,
}

/// A report detailing the process and outcome of refining Tariro's internal values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueRefinementReport {
    pub id: String,
    pub model_id: String, // Link to HumanValueModel
    pub refinement_method: String,
    pub societal_impact_change: f64, // Expected change in societal impact (e.g., +0.02 wellbeing)
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
