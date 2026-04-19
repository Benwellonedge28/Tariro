// Tariro_ide_idpte/src/agi_meta_goals/goal_management_models.rs

//! Defines data models for Tariro's Goal-Driven Autonomous System module,
//! including autonomous goals, alignment reports, and pursuit strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents an autonomously formulated goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousGoal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub meta_goal_id: String, // Link to the higher-level meta-goal it serves
    pub priority: f64, // 0.0 (low) to 1.0 (critical)
    pub estimated_complexity: f64,
    pub ethical_review_status: VerificationStatus, // New: Ethical review status of the goal itself
    pub security_review_status: VerificationStatus, // New: Security review status of the goal's implications
    pub formulated_at: DateTime<Utc>,
}

/// A report summarizing the alignment of a goal with Tariro's core directives and ethical axioms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAlignmentReport {
    pub id: String,
    pub goal_id: String,
    pub alignment_score: f64, // 0.0 (misaligned) to 1.0 (perfectly aligned)
    pub detected_conflicts: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance
    pub security_compliance_status: VerificationStatus, // Overall security compliance
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

/// Represents a strategy for achieving a specific autonomous goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalPursuitStrategy {
    pub id: String,
    pub goal_id: String,
    pub strategy_name: String,
    pub proposed_actions: Vec<String>,
    pub expected_outcomes: Vec<String>,
    pub resource_requirements: HashMap<String, f64>, // e.g., "compute": 10.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the pursuit strategy
    pub security_review_status: VerificationStatus, // New: Security review of the pursuit strategy
    pub optimized_by_meta_learner: Option<String>, // ID of MetaLearner algorithm used
    pub created_at: DateTime<Utc>,
}

/// Verification of a goal's security implications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureGoalVerification {
    pub id: String,
    pub goal_id: String,
    pub audit_summary: String,
    pub identified_vulnerabilities: Vec<String>,
    pub potential_harm_summary: String,
    pub integrity_of_goal_statement: VerificationStatus, // New: Integrity of the goal statement itself
    pub overall_status: VerificationStatus,
    pub verified_at: DateTime<Utc>,
}
