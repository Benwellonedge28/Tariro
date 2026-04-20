// Tariro_ide_idpte/src/compliance_advisor/ethical_constraint_models.rs

//! Defines data models for Tariro's Dynamic Ethical Constraint Generation & Enforcement module,
//! including ethical constraints, compliance policies, and ethical violation records.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A formal representation of an ethical rule or principle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConstraint {
    pub id: String,
    pub name: String,
    pub description: String,
    pub formal_representation: String, // e.g., "IF [action] THEN [ethical_impact] < threshold"
    pub severity: f64, // 0.0 to 1.0, impact of violating this constraint
    pub source: String, // e.g., "human_consensus", "aleca_inference", "legal_mandate"
    pub ethical_review_status: VerificationStatus, // Ethical review of the constraint itself
    pub created_at: DateTime<Utc>,
}

/// A set of ethical rules and policies governing Tariro's behavior in a specific domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompliancePolicy {
    pub id: String,
    pub policy_name: String,
    pub target_domain: String, // e.g., "resource_allocation", "information_dissemination"
    pub applied_constraints: Vec<String>, // List of EthicalConstraint IDs
    pub enforcement_mechanism: String, // e.g., "real_time_prevention", "post_action_correction"
    pub ethical_governance_review: VerificationStatus, // Ethical review of the policy effectiveness
    pub security_integrity_status: VerificationStatus, // Security of the policy enforcement
    pub last_updated: DateTime<Utc>,
}

/// A record of a detected ethical breach or potential violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalViolationRecord {
    pub id: String,
    pub violated_constraint_id: String, // Link to EthicalConstraint
    pub violating_action_id: String, // Link to specific action/decision
    pub severity_of_violation: f64, // 0.0 to 1.0
    pub context: String,
    pub proposed_remediation: Option<String>,
    pub ethical_remediation_review: VerificationStatus, // Ethical review of remediation plan
    pub security_audited: VerificationStatus, // Security audit of the violation record
    pub detected_at: DateTime<Utc>,
}

/// Audit report for dynamic ethical constraint generation and enforcement processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConstraintAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_constraints_generated: u64,
    pub total_violations_detected: u64,
    pub successful_remediations: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
