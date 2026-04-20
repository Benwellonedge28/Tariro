// Tariro_ide_idpte/src/strategic_planning/planning_models.rs

//! Defines data models for Tariro's Hierarchical Planning & Long-Term Strategy module,
//! including strategic plans, planning stages, and contingency plans.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A high-level, multi-stage plan for achieving a complex meta-goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategicPlan {
    pub id: String,
    pub goal_id: String, // Link to MetaGoal
    pub plan_name: String,
    pub primary_objective: String,
    pub estimated_timeframe_days: u64,
    pub stages: Vec<String>, // List of PlanningStage IDs
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the overall plan
    pub security_verified: VerificationStatus, // New: Security verification of plan integrity
    pub created_at: DateTime<Utc>,
}

/// A specific phase or milestone within a StrategicPlan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningStage {
    pub id: String,
    pub plan_id: String,
    pub stage_order: u32,
    pub description: String,
    pub required_resources_summary: String,
    pub dependency_stage_ids: Vec<String>,
    pub status: String, // e.g., "pending", "active", "completed", "failed"
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the stage's actions
    pub security_integrity: VerificationStatus, // New: Security integrity of stage data
    pub updated_at: DateTime<Utc>,
}

/// An alternative plan to be activated if specific conditions or obstacles are met.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContingencyPlan {
    pub id: String,
    pub plan_id: String,
    pub stage_id: Option<String>, // Can be linked to a specific stage
    pub trigger_condition: String,
    pub alternative_action_summary: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the contingency plan
    pub security_verified: VerificationStatus, // New: Security verification of contingency integrity
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of strategic planning and strategy execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyAuditReport {
    pub id: String,
    pub plan_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_stages_processed: u32,
    pub contingency_triggers_activated: u32,
    pub detected_planning_biases: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of planning
    pub security_integrity_status: VerificationStatus, // Overall security integrity of planning systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
