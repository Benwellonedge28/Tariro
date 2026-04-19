// Tariro_ide_idpte/src/cognitive_arch/metacognition_models.rs

//! Defines data models for Tariro's Metacognition & Self-Reflection module,
//! including representations of cognitive states, self-correction actions, and reflection logs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a snapshot of Tariro's cognitive state at a given moment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub module_states: HashMap<String, String>, // Summary of state for each cognitive module
    pub active_goals: Vec<String>, // Current active goals
    pub resource_utilization: HashMap<String, f64>, // e.g., "cpu": 0.8, "memory": 0.6
    pub emotional_state_summary: String, // Summary of internal emotional/motivational state
    pub confidence_in_current_action: f64, // 0.0 to 1.0
    pub integrity_hash: String, // New: Hash for integrity verification
}

/// Represents an action Tariro plans or executes to self-correct a cognitive process or behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfCorrectionAction {
    pub id: String,
    pub problem_description: String, // Description of the issue being corrected
    pub proposed_action: String, // e.g., "re-evaluate causal model X", "optimize HRL policy Y"
    pub target_module_id: String, // The module this action is directed at
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review status of the action
    pub security_review_status: VerificationStatus, // New: Security review status of the action
    pub initiated_at: DateTime<Utc>,
}

/// A log entry for Tariro's self-reflection process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionLog {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_summary: String, // Summary of the event that triggered reflection
    pub cognitive_state_snapshot: CognitiveState,
    pub self_diagnosis: String, // Tariro's internal analysis of its state/performance
    pub identified_opportunities: Vec<String>, // Areas for improvement
    pub generated_self_correction_ids: Vec<String>, // Linked self-correction actions
    pub log_integrity_status: VerificationStatus, // New: Integrity status of the log entry
}

/// A detailed report generated from introspection, possibly for human review or deeper analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectionReport {
    pub id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub summary_of_performance: String,
    pub key_insights: Vec<String>, // High-level learnings from self-reflection
    pub major_self_correction_actions: Vec<String>, // IDs of significant actions taken
    pub areas_for_improvement: Vec<String>,
    pub report_integrity_hash: String, // New: Hash for report integrity
    pub generated_at: DateTime<Utc>,
}

/// Verification status for reflection logs and introspection reports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionVerificationStatus {
    pub privacy_compliance_status: VerificationStatus, // Ensuring sensitive data is handled correctly
    pub integrity_proof_status: VerificationStatus, // Proof that log has not been tampered with
    pub overall_status: VerificationStatus,
    pub last_verified: DateTime<Utc>,
    pub verification_log_id: String,
}
