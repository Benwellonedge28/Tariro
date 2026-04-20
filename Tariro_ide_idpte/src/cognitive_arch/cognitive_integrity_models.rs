// Tariro_ide_idpte/src/cognitive_arch/cognitive_integrity_models.rs

//! Defines data models for Tariro's Cognitive Integrity & Anti-Corruption Mechanisms module,
//! including cognitive integrity reports, internal corruption detection, and self-deception mitigation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A report summarizing the results of internal self-audits for cognitive consistency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveIntegrityReport {
    pub id: String,
    pub audit_scope: String, // e.g., "reasoning_logic", "knowledge_base_consistency"
    pub consistency_score: f64, // 0.0 to 1.0
    pub detected_inconsistencies: Vec<String>,
    pub ethical_review_status: VerificationStatus, // Ethical review of the integrity report
    pub generated_at: DateTime<Utc>,
}

/// Details of an identified instance of internal corruption, bias, or logical inconsistency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalCorruptionDetection {
    pub id: String,
    pub report_id: String, // Link to CognitiveIntegrityReport
    pub anomaly_type: String, // e.g., "logical_contradiction", "unjustified_bias", "preference_drift"
    pub affected_component: String,
    pub severity: f64, // 0.0 to 1.0
    pub ethical_violation_link: Option<String>, // Link to EthicalViolationReport if applicable
    pub security_impact_assessment: VerificationStatus, // Security impact of the corruption
    pub detected_at: DateTime<Utc>,
}

/// A strategy implemented by Tariro to correct or mitigate detected internal cognitive issues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfDeceptionMitigationStrategy {
    pub id: String,
    pub corruption_detection_id: String, // Link to InternalCorruptionDetection
    pub strategy_description: String,
    pub implemented_actions: Vec<String>, // e.g., "re-evaluate_premises", "diversify_learning_data"
    pub expected_integrity_restoration: f64, // 0.0 to 1.0
    pub ethical_compliance_status: VerificationStatus, // Ethical review of the mitigation strategy
    pub security_verified: VerificationStatus, // Security verification of mitigation steps
    pub created_at: DateTime<Utc>,
}

/// Audit report for cognitive integrity and anti-corruption processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_integrity_checks: u64,
    pub total_anomalies_detected: u64,
    pub successful_remediations: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
