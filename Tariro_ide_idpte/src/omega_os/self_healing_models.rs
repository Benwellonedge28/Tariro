// Tariro_ide_idpte/src/omega_os/self_healing_models.rs

//! Defines data models for Tariro's Self-Healing & Fault Tolerance module,
//! including fault reports, recovery actions, and system health states.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a report detailing a detected or predicted system fault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultReport {
    pub id: String,
    pub fault_type: String, // e.g., "resource_exhaustion", "component_failure", "logical_error"
    pub severity: f64, // 0.0 (minor) to 1.0 (critical)
    pub prediction_confidence: f64, // If predicted
    pub affected_components: Vec<String>,
    pub root_cause_summary: String, // From CausalReasoningEngine
    pub timestamp: DateTime<Utc>,
    pub report_integrity_hash: String, // New: Hash for integrity of the fault report
}

/// Represents a proposed or executed action to mitigate a fault.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAction {
    pub id: String,
    pub fault_report_id: String,
    pub description: String,
    pub action_type: String, // e.g., "restart_component", "reallocate_resources", "rollback_update"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the action
    pub security_review_status: VerificationStatus, // New: Security review of the action
    pub executed_at: Option<DateTime<Utc>>,
    pub actual_outcome_summary: Option<String>,
}

/// Represents a snapshot of the overall system health state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthState {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub overall_score: f64, // 0.0 (critical) to 1.0 (perfect)
    pub component_health: HashMap<String, f64>, // Component ID -> Health Score
    pub active_faults: Vec<String>, // IDs of current fault reports
    pub pending_recovery_actions: Vec<String>, // IDs of actions being considered/executed
    pub integrity_verified: VerificationStatus, // New: Integrity of the health state data itself
}

/// A report summarizing the ethical analysis of a fault and recovery process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalRecoveryAnalysis {
    pub id: String,
    pub fault_report_id: String,
    pub recovery_action_id: Option<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical status of recovery
    pub unintended_side_effects: Vec<String>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}
