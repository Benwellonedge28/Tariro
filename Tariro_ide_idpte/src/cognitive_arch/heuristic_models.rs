// Tariro_ide_idpte/src/cognitive_arch/heuristic_models.rs

//! Defines data models for heuristics, intuitive leaps, and problem context summaries
//! within Tariro's Intuition & Heuristic Generation module.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For heuristic verification

/// Represents a learned problem-solving heuristic (rule of thumb or shortcut).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heuristic {
    pub id: String,
    pub name: String,
    pub description: String,
    pub applicability_conditions: HashMap<String, String>, // e.g., "problem_type": "optimization", "time_constraint": "high"
    pub suggested_action: String, // e.g., "use greedy approach", "prioritize safety"
    pub expected_accuracy: f64, // 0.0 to 1.0 (how often it's correct)
    pub discovery_source: String, // e.g., "MetaLearner", "HumanObservation", "SelfExperimentation"
    pub timestamp: DateTime<Utc>,
    pub generation_origin: String, // New: e.g., "self_discovered", "meta_learner_optimized", "human_encoded"
    pub verification_status: HeuristicVerificationStatus, // New: Verification status for the heuristic
}

/// Represents an intuitive leap or rapid inference made by Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitiveLeap {
    pub id: String,
    pub problem_context_summary: String, // Summary of the problem state when the leap occurred
    pub inferred_solution: String, // The solution or insight arrived at intuitively
    pub confidence: f64, // Confidence in the intuitive leap (0.0 to 1.0)
    pub supporting_patterns: Vec<String>, // Key patterns or heuristics that contributed to the leap
    pub timestamp: DateTime<Utc>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the intuitive leap
    pub security_review_status: VerificationStatus, // New: Security review of the intuitive leap
}

/// Summarizes the current context of a problem for heuristic application or intuitive leaps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemContextSummary {
    pub id: String,
    pub problem_id: String,
    pub description: String,
    pub key_features: HashMap<String, String>, // Important features of the problem state
    pub urgency_score: f64, // 0.0 to 1.0 (how quickly a decision is needed)
    pub available_resources_summary: HashMap<String, f64>, // Current resource availability
    pub timestamp: DateTime<Utc>,
}

/// Comprehensive verification status for a heuristic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicVerificationStatus {
    pub ethical_compliance_status: VerificationStatus,
    pub safety_guarantee_status: VerificationStatus,
    pub logical_consistency_status: VerificationStatus, // Consistent with other knowledge
    pub overall_status: VerificationStatus,
    pub last_verified: DateTime<Utc>,
    pub verification_log_id: String,
}

/// Represents feedback on the performance of a heuristic or intuitive leap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicPerformanceFeedback {
    pub heuristic_id: String,
    pub problem_id: String,
    pub actual_outcome_success: f64, // 0.0 to 1.0
    pub feedback_summary: String,
    pub timestamp: DateTime<Utc>,
    pub resource_cost: HashMap<String, f64>, // Resources consumed during application
}
