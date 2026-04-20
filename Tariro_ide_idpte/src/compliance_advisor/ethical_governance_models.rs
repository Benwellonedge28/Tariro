// Tariro_ide_idpte/src/compliance_advisor/ethical_governance_models.rs

//! Defines data models for Tariro's Ethical AI Governance & Policy Enforcement module,
//! including ethical policies, compliance verifications, and ethical decision logs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A formal statement or rule defining ethical principles and acceptable behavior for Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalPolicy {
    pub id: String,
    pub policy_name: String,
    pub principle: String, // e.g., "beneficence", "non-maleficence", "autonomy", "justice"
    pub rule_set_description: String, // How the principle is translated into actionable rules
    pub version: String,
    pub ethical_review_status: VerificationStatus, // Ethical review of the policy itself
    pub security_integrity_status: VerificationStatus, // Security of policy enforcement mechanisms
    pub created_at: DateTime<Utc>,
}

/// A record of verifying an AI operation's compliance against a specific ethical policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceVerification {
    pub id: String,
    pub policy_id: String, // Link to EthicalPolicy
    pub ai_operation_id: String, // ID of the AI operation being verified
    pub verification_result: VerificationStatus, // Passed or Failed
    pub detected_violations: Vec<String>,
    pub ethical_compliance_level: f64, // 0.0 to 1.0
    pub security_audited: VerificationStatus, // Security audit of the verification process
    pub performed_at: DateTime<Utc>,
}

/// A log entry for decisions made by Tariro that have significant ethical implications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalDecisionLog {
    pub id: String,
    pub decision_summary: String,
    pub involved_modules: Vec<String>, // Modules that contributed to the decision
    pub ethical_policy_applied_id: Option<String>, // Link to EthicalPolicy
    pub moral_reasoning_summary: String, // Explanation of the moral rationale
    pub human_oversight_intervention: Option<String>, // Record of human approval/override
    pub ethical_integrity: VerificationStatus, // Ethical integrity of the decision
    pub recorded_at: DateTime<Utc>,
}

/// Audit report for ethical AI governance and policy enforcement processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_policies_enforced: u64,
    pub total_violations_flagged: u64,
    pub total_human_interventions: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
