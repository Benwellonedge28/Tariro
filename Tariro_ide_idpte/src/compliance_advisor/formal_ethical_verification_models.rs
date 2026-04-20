// Tariro_ide_idpte/src/compliance_advisor/formal_ethical_verification_models.rs

//! Defines data models for Tariro's Formal Verification of Ethical Constraints & Policies module,
//! including formal specifications, proofs of compliance, and ethical violation reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A precise, mathematically rigorous representation of an ethical principle or constraint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormalSpecification {
    pub id: String,
    pub ethical_rule_id: String, // Link to an EthicalConstraint or EthicalPrinciple
    pub specification_language: String, // e.g., "Coq", "Agda", "TLA+"
    pub specification_code: String, // The actual formal specification
    pub ethical_review_status: VerificationStatus, // Ethical review of the specification itself
    pub security_verified: VerificationStatus, // Security verification of specification correctness
    pub created_at: DateTime<Utc>,
}

/// A cryptographically verifiable proof that an algorithm or system adheres to a FormalSpecification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfCompliance {
    pub id: String,
    pub specification_id: String, // Link to FormalSpecification
    pub system_component_id: String, // The component being verified
    pub proof_type: String, // e.g., "Zero-Knowledge Proof", "Automated Theorem Prover Output"
    pub verification_result: VerificationStatus, // Proof status: Passed/Failed
    pub ethical_guarantee: String, // Statement of ethical adherence provided by the proof
    pub security_audited: VerificationStatus, // Security audit of the proof generation process
    pub generated_at: DateTime<Utc>,
}

/// A report detailing a detected breach of an EthicalConstraint or FormalSpecification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalViolationReport {
    pub id: String,
    pub violated_rule_id: String, // Link to EthicalConstraint or FormalSpecification
    pub offending_component_id: String,
    pub violation_description: String,
    pub severity_score: f64, // 0.0 to 1.0
    pub root_cause_analysis: String,
    pub recommended_remediation: Vec<String>,
    pub ethical_review_status: VerificationStatus, // Ethical review of the violation assessment
    pub security_impact_assessment: VerificationStatus, // Security assessment of the violation
    pub detected_at: DateTime<Utc>,
}

/// Audit report for formal verification of ethical constraints and policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_ethical_verifications", "critical_control_logic"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_specifications_verified: u64,
    pub total_proofs_generated: u64,
    pub detected_verification_failures: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
