// Tariro_ide_idpte/src/security/adversarial_robustness_models.rs

//! Defines data models for Tariro's Adversarial Robustness Framework module,
//! including representations of adversarial attacks, detection reports,
//! and mitigation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a detected or simulated adversarial attack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdversarialAttack {
    pub id: String,
    pub attack_type: String, // e.g., "data_poisoning", "model_evasion", "prompt_injection", "trojan"
    pub target_module_id: String,
    pub detected_signature: String,
    pub severity_score: f64, // 0.0 to 1.0
    pub impact_summary: String,
    pub ethical_breach_status: VerificationStatus, // New: From ALEC-A
    pub timestamp: DateTime<Utc>,
}

/// A report detailing the detection of an adversarial attack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackDetectionReport {
    pub id: String,
    pub attack_id: String,
    pub detection_method: String, // e.g., "anomaly_detection", "semantic_consistency_check", "honeypot"
    pub confidence_score: f64, // 0.0 to 1.0
    pub mitigation_strategy_id: Option<String>,
    pub integrity_verified: VerificationStatus, // New: Integrity of the detection process
    pub reported_at: DateTime<Utc>,
}

/// A strategy for mitigating an adversarial attack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub id: String,
    pub attack_id: String,
    pub name: String,
    pub proposed_actions: Vec<String>, // e.g., "quarantine_component", "filter_input", "retrain_model"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the mitigation strategy
    pub security_review_status: VerificationStatus, // New: Security review of the mitigation strategy
    pub approved_by: String, // e.g., "Tariro_ARM", "ALEC-A"
    pub created_at: DateTime<Utc>,
}

/// A cryptographically secure log entry for an adversarial attack and its mitigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureAttackLog {
    pub id: String,
    pub attack_report: AttackDetectionReport,
    pub executed_mitigation: Option<MitigationStrategy>,
    pub post_mitigation_status: String,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub logged_at: DateTime<Utc>,
    pub log_integrity_status: VerificationStatus, // New: Integrity of the log entry
}
