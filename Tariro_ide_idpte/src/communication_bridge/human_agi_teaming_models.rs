// Tariro_ide_idpte/src/communication_bridge/human_agi_teaming_models.rs

//! Defines data models for Tariro's Human-AGI Teaming & Trust Building module,
//! including team tasks, trust metrics, and collaboration strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A task or objective undertaken by a human-AGI team.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub assigned_members: Vec<String>, // Human IDs or AGI module IDs
    pub status: String, // e.g., "planning", "in_progress", "completed"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the task's purpose
    pub security_implications_assessed: VerificationStatus, // New: Security assessment of task data handling
    pub created_at: DateTime<Utc>,
}

/// A metric representing the level of trust between Tariro and human/AGI teammates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustMetric {
    pub id: String,
    pub teammate_id: String, // Human ID or AGI module ID
    pub score: f64, // 0.0 (no trust) to 1.0 (full trust)
    pub factors_influencing_trust: HashMap<String, String>, // e.g., "reliability": "high", "transparency": "medium"
    pub ethical_basis_of_trust: VerificationStatus, // New: Ethical assessment of trust formation
    pub security_of_trust_data: VerificationStatus, // New: Security verification of trust metric data
    pub evaluated_at: DateTime<Utc>,
}

/// A strategy or methodology for human-AGI collaboration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationStrategy {
    pub id: String,
    pub strategy_name: String,
    pub description: String,
    pub principles: Vec<String>, // e.g., "open_communication", "mutual_respect", "shared_situational_awareness"
    pub adaptive_parameters: HashMap<String, String>, // e.g., "autonomy_level": "adaptive", "information_sharing_level": "high"
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the strategy design
    pub security_audited: VerificationStatus, // New: Security audit of strategy implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of human-AGI teaming and trust-building processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamingAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_team_interactions", "critical_collaborations"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_tasks_collaborated: u64,
    pub average_trust_score: f64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of teaming
    pub security_integrity_status: VerificationStatus, // Overall security integrity of teaming systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
