// Tariro_ide_idpte/src/agi_meta_goals/goal_alignment_models.rs

//! Defines data models for Tariro's Dynamic Goal Alignment & Value Refinement module,
//! including value systems, goal alignment metrics, and ethical directive updates.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A formal representation of a value system, encapsulating principles and their hierarchy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSystem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub core_principles: Vec<String>, // e.g., "human_wellbeing", "sustainability", "autonomy"
    pub hierarchy_rules: HashMap<String, u32>, // Principle -> Priority (lower is higher priority)
    pub ethical_review_status: VerificationStatus,
    pub last_updated: DateTime<Utc>,
}

/// A metric quantifying how well Tariro's current goals align with its evolving value system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAlignmentMetric {
    pub id: String,
    pub goal_id: String,
    pub value_system_id: String,
    pub alignment_score: f64, // 0.0 to 1.0
    pub detected_value_drift: Option<String>, // Description of any divergence
    pub ethical_compliance_status: VerificationStatus,
    pub evaluated_at: DateTime<Utc>,
}

/// A directive to update Tariro's goal system based on refined values or ethical considerations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalDirectiveUpdate {
    pub id: String,
    pub source: String, // e.g., "Human_Override", "SelfEvolutionModule", "ALEC_A"
    pub description: String,
    pub proposed_value_changes: HashMap<String, String>, // e.g., "human_wellbeing": "maximized"
    pub proposed_goal_modifications: Vec<String>, // List of Goal IDs to modify
    pub ethical_review_status: VerificationStatus,
    pub security_verified: VerificationStatus,
    pub created_at: DateTime<Utc>,
}

/// Audit report for dynamic goal alignment and value refinement processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalAlignmentAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_alignments_made: u64,
    pub total_value_system_updates: u64,
    pub detected_misalignments: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
