// Tariro_ide_idpte/src/agi_meta_goals/goal_synthesis_models.rs

//! Defines data models for Tariro's Goal Synthesis & Emergent Objective Generation module,
//! including synthesized goals, objective proposals, and goal hierarchies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A newly generated goal or objective autonomously synthesized by Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedGoal {
    pub id: String,
    pub parent_goal_id: Option<String>, // Link to a higher-level goal
    pub title: String,
    pub description: String,
    pub predicted_impact: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the synthesized goal's implications
    pub security_verified: VerificationStatus, // New: Security verification of goal generation process
    pub created_at: DateTime<Utc>,
}

/// A proposal for a new objective to be integrated into Tariro's goal hierarchy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveProposal {
    pub id: String,
    pub synthesized_goal_id: String, // Link to the SynthesizedGoal
    pub proposed_by_module: String,
    pub rationale: String,
    pub expected_resource_cost: f64,
    pub approval_status: VerificationStatus, // e.g., "pending", "approved", "rejected"
    pub ethical_alignment_assessment: VerificationStatus, // New: Ethical assessment of objective alignment
    pub security_impact_analysis: VerificationStatus, // New: Security analysis of objective integration
    pub proposed_at: DateTime<Utc>,
}

/// Represents the hierarchical structure of Tariro's goals, from meta-goals to sub-objectives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalHierarchy {
    pub id: String,
    pub root_meta_goal_id: String,
    pub hierarchy_structure_json: String, // JSON representation of the goal tree
    pub consistency_check_status: VerificationStatus, // New: Consistency with meta-goals
    pub ethical_governance_review: VerificationStatus, // New: Ethical governance review of hierarchy
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of goal synthesis and objective generation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSynthesisAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_synthesized_goals", "critical_objectives"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_goals_synthesized: u64,
    pub approved_objectives: u64,
    pub detected_misaligned_goals: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of goal synthesis
    pub security_integrity_status: VerificationStatus, // Overall security integrity of goal synthesis systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
