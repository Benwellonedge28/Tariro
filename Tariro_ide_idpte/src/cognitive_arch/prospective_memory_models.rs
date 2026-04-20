// Tariro_ide_idpte/src/cognitive_arch/prospective_memory_models.rs

//! Defines data models for Tariro's Prospective Memory & Goal-Oriented Recall module,
//! including prospective intentions, trigger events, and action execution records.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a future-oriented task, reminder, or commitment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProspectiveIntention {
    pub id: String,
    pub description: String,
    pub target_date_time: Option<DateTime<Utc>>, // For time-based triggers
    pub trigger_condition_summary: Option<String>, // For event-based or context-based triggers
    pub associated_goal_id: Option<String>, // Link to a higher-level goal
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the intention's purpose
    pub security_verified: VerificationStatus, // New: Security verification of intention integrity
    pub created_at: DateTime<Utc>,
}

/// A specific condition or event that activates a prospective intention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerEvent {
    pub id: String,
    pub intention_id: String,
    pub event_type: String, // e.g., "time_reached", "sensory_input_detected", "internal_state_met"
    pub description: String,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of the trigger
    pub security_validity: VerificationStatus, // New: Security validity of the trigger source
    pub timestamp: DateTime<Utc>,
}

/// A record of a completed prospective action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionExecutionRecord {
    pub id: String,
    pub intention_id: String,
    pub action_performed: String,
    pub outcome_summary: String,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the executed action
    pub security_integrity: VerificationStatus, // New: Security integrity of the execution process
    pub execution_timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of prospective memory operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProspectiveMemoryAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_intentions", "critical_reminders"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_intentions_registered: u64,
    pub total_actions_executed: u64,
    pub detected_failures: Vec<String>, // e.g., "forgotten_task", "missed_trigger"
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of prospective memory
    pub security_integrity_status: VerificationStatus, // Overall security integrity of prospective memory systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
