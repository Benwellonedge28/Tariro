// Tariro_ide_idpte/src/communication_bridge/skill_transfer_models.rs

//! Defines data models for Tariro's Human-AGI Skill Transfer & Scaffolding module,
//! including skill transfer requests, learning pathways, and scaffolding strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A request from either human or AGI to transfer a specific skill or knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTransferRequest {
    pub id: String,
    pub requester_id: String, // Human ID or AGI module ID
    pub target_id: String, // Human ID or AGI module ID
    pub skill_description: String,
    pub desired_proficiency_level: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the skill transfer's purpose
    pub security_verified: VerificationStatus, // New: Security verification of skill data integrity
    pub requested_at: DateTime<Utc>,
}

/// A structured plan or curriculum for human or AGI to acquire a new skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathway {
    pub id: String,
    pub skill_id: String, // Refers to the skill being learned
    pub learner_id: String,
    pub steps_summary: Vec<String>,
    pub estimated_completion_time_hours: f64,
    pub scaffolding_level: f64, // 0.0 to 1.0 (more scaffolding = higher value)
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the pathway design
    pub security_audited: VerificationStatus, // New: Security audit of learning material security
    pub created_at: DateTime<Utc>,
}

/// A strategy or method used to support human learning and development.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldingStrategy {
    pub id: String,
    pub strategy_name: String,
    pub description: String,
    pub adaptive_parameters: HashMap<String, String>, // e.g., "feedback_frequency": "high", "hint_level": "medium"
    pub ethical_application_review: VerificationStatus, // New: Ethical review of scaffolding application
    pub security_integrity: VerificationStatus, // New: Security integrity of scaffolding process
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of Human-AGI Skill Transfer processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTransferAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_skill_transfers", "human_learning_pathways"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_transfers_initiated: u64,
    pub successful_transfers: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of skill transfer
    pub security_integrity_status: VerificationStatus, // Overall security integrity of skill transfer systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
