// Tariro_ide_idpte/src/security/bias_correction_models.rs

//! Defines data models for Tariro's Self-Correction of Biases module,
//! including bias detection reports, debiasing strategies, and bias correction logs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A report detailing a detected bias within Tariro's data, algorithms, or decision-making.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasDetectionReport {
    pub id: String,
    pub bias_type: String, // e.g., "statistical", "representational", "ethical"
    pub affected_component: String, // e.g., "data_manager", "meta_learner", "decision_module"
    pub description: String,
    pub severity: f64, // 0.0 to 1.0
    pub evidence: Vec<String>, // Links to data samples or algorithmic traces
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical assessment of the bias impact
    pub security_verified: VerificationStatus, // New: Security verification of bias detection process
    pub detected_at: DateTime<Utc>,
}

/// A plan or set of actions designed to mitigate a detected bias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebiasingStrategy {
    pub id: String,
    pub bias_report_id: String,
    pub strategy_name: String,
    pub proposed_actions: Vec<String>, // e.g., "data_reweighting", "algorithmic_adjustment", "human_review_protocol"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the debiasing strategy
    pub security_review_status: VerificationStatus, // New: Security review of the debiasing strategy
    pub approved_by: String, // e.g., "Tariro_BSC", "ALEC-A", "HumanOversightCommittee"
    pub created_at: DateTime<Utc>,
}

/// A log entry for a specific bias correction action performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasCorrectionLog {
    pub id: String,
    pub strategy_id: String,
    pub action_taken: String,
    pub initial_bias_severity: f64,
    pub resulting_bias_severity: f64,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the correction action
    pub security_integrity: VerificationStatus, // New: Security integrity of the correction process
    pub executed_at: DateTime<Utc>,
}

/// A report summarizing the audit of bias detection and self-correction processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_bias_corrections", "specific_data_pipeline"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_biases_detected: u64,
    pub total_biases_mitigated: u64,
    pub detected_new_biases: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of bias correction
    pub security_integrity_status: VerificationStatus, // Overall security integrity of bias correction systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
