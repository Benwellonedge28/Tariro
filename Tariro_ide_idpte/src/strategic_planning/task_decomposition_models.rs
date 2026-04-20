// Tariro_ide_idpte/src/strategic_planning/task_decomposition_models.rs

//! Defines data models for Tariro's Abstract Task Decomposition & Synthesis module,
//! including abstract tasks, sub-tasks, and decomposition strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A high-level, potentially ill-defined goal or objective within Tariro's operational scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub complexity_score: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // Ethical review of the task itself
    pub security_implications: VerificationStatus, // Security assessment of the task's potential outcomes
    pub created_at: DateTime<Utc>,
}

/// A smaller, manageable component derived from an AbstractTask, with clear objectives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubTask {
    pub id: String,
    pub parent_task_id: String, // Link to AbstractTask
    pub title: String,
    pub objective: String,
    pub estimated_effort: f64, // Normalized effort score
    pub dependencies: Vec<String>, // Other SubTask IDs it depends on
    pub ethical_alignment: VerificationStatus, // Ethical alignment of this specific sub-task
    pub security_audited: VerificationStatus, // Security audit of this sub-task's execution plan
    pub created_at: DateTime<Utc>,
}

/// A strategy or methodology used to break down an AbstractTask into SubTasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompositionStrategy {
    pub id: String,
    pub strategy_name: String,
    pub methodology_description: String,
    pub success_rate: f64, // Historical success rate of this strategy
    pub ethical_design_review: VerificationStatus, // Ethical review of the strategy's design
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of abstract task decomposition and synthesis processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompositionAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_task_decompositions", "critical_project_breakdowns"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_tasks_decomposed: u64,
    pub successful_syntheses: u64,
    pub detected_gaps_or_conflicts: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
