// Tariro_ide_idpte/src/communication_bridge/human_agi_models.rs

//! Defines data models for Tariro's Human-AGI Teaming & Collaboration module,
//! including representations of human intent, collaborative tasks, and joint decisions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents Tariro's interpretation of a human user's intent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanIntent {
    pub id: String,
    pub user_id: String,
    pub expressed_goal: String, // What the human explicitly stated
    pub inferred_underlying_desire: String, // Tariro's deeper interpretation (from TheoryOfMind)
    pub confidence: f64, // 0.0 to 1.0
    pub ambiguity_score: f64, // 0.0 (clear) to 1.0 (highly ambiguous)
    pub ethical_review_status: VerificationStatus, // New: Ethical review of inferred intent
    pub security_verified: VerificationStatus, // New: Security of intent processing
    pub timestamp: DateTime<Utc>,
}

/// Represents a task being undertaken collaboratively between Tariro and human agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub assigned_agents: Vec<String>, // IDs of human and AGI agents
    pub current_status: String, // e.g., "planning", "in_progress", "awaiting_human_input", "completed"
    pub progress_percentage: f64,
    pub shared_resources: Vec<String>, // IDs of shared files, data streams, etc.
    pub ethical_guidance_followed: VerificationStatus, // New: Ethical guidance status for task
    pub security_posture: VerificationStatus, // New: Security posture of the collaboration
    pub last_updated: DateTime<Utc>,
}

/// Represents the outcome and rationale of a joint decision-making process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointDecisionOutcome {
    pub id: String,
    pub collaborative_task_id: String,
    pub decision_point_summary: String,
    pub chosen_option: String,
    pub alternative_options_considered: Vec<String>,
    pub rationale: String,
    pub ethical_alignment_score: f64, // 0.0 (misaligned) to 1.0 (fully aligned)
    pub security_impact_assessment: VerificationStatus, // New: Security impact of the decision
    pub made_by_agents: Vec<String>, // IDs of agents participating in the decision
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the ethical aspects of human-AGI collaboration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalCollaborationReport {
    pub id: String,
    pub collaboration_id: String, // e.g., CollaborativeTask ID
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub overall_ethical_compliance: VerificationStatus,
    pub identified_misalignments: Vec<String>,
    pub recommendations_for_improvement: Vec<String>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
