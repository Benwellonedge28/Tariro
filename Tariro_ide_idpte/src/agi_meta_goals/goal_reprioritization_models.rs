// Tariro_ide_idpte/src/agi_meta_goals/goal_reprioritization_models.rs

//! Defines data models for Tariro's Dynamic Goal Re-prioritization module,
//! including goal change requests, prioritization criteria, and reprioritization decisions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A request to alter the priority or focus of an existing goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalChangeRequest {
    pub id: String,
    pub goal_id: String,
    pub requested_change_type: String, // e.g., "increase_priority", "decrease_priority", "redefine_scope"
    pub rationale_summary: String,
    pub initiating_module_id: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the request's implications
    pub security_verified: VerificationStatus, // New: Security verification of request authenticity
    pub created_at: DateTime<Utc>,
}

/// Factors and rules used to determine the priority of goals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizationCriteria {
    pub id: String,
    pub criteria_name: String,
    pub associated_meta_goal_id: String, // Which meta-goal this criteria serves
    pub weight: f64, // 0.0 to 1.0
    pub ethical_alignment_score: f64, // 0.0 to 1.0, how well it aligns with ethical principles
    pub impact_prediction_score: f64, // 0.0 to 1.0, predicted positive impact
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the criteria design
    pub security_audited: VerificationStatus, // New: Security audit of the criteria implementation
    pub last_updated: DateTime<Utc>,
}

/// The outcome of a goal re-prioritization process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReprioritizationDecision {
    pub id: String,
    pub goal_id: String,
    pub old_priority: f64,
    pub new_priority: f64,
    pub rationale_summary: String,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the decision
    pub security_integrity: VerificationStatus, // New: Security integrity of the decision record
    pub decision_timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of dynamic goal re-prioritization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_goals", "critical_meta_goals"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_reprioritizations: u64,
    pub detected_inconsistencies: Vec<String>, // e.g., "conflicting_priorities", "unaligned_subgoals"
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of goal prioritization
    pub security_integrity_status: VerificationStatus, // Overall security integrity of goal management systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
