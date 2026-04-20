// Tariro_ide_idpte/src/crypto_fabric/zko_models.rs

//! Defines data models for Tariro's Zero-Knowledge Proofs & Privacy-Preserving AI module,
//! including zero-knowledge proofs, privacy policies, and verifiable computations.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a generated Zero-Knowledge Proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroKnowledgeProof {
    pub id: String,
    pub statement_proven: String, // What the proof asserts (e.g., "I know X without revealing X")
    pub proof_content_hash: String, // Hash of the actual cryptographic proof
    pub proof_type: String, // e.g., "SNARK", "STARK", "Bulletproofs"
    pub prover_id: String,
    pub verifier_id: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the proof's intent and implications
    pub security_verified: VerificationStatus, // New: Security verification of the proof's cryptographic soundness
    pub generated_at: DateTime<Utc>,
}

/// A policy defining rules for sensitive data usage in privacy-preserving computations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPolicy {
    pub id: String,
    pub policy_name: String,
    pub data_categories_covered: Vec<String>, // e.g., "medical_records", "personal_identifiers", "secret_algorithms"
    pub allowed_operations: Vec<String>, // Operations allowed under this policy (e.g., "sum_only", "max_differential_privacy")
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the privacy policy design
    pub security_audited: VerificationStatus, // New: Security audit of the policy enforcement mechanism
    pub last_updated: DateTime<Utc>,
}

/// Describes a computation performed using privacy-preserving techniques.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableComputation {
    pub id: String,
    pub computation_goal: String,
    pub input_data_hashes: Vec<String>, // Hashes of sensitive input data used
    pub output_data_hash: String, // Hash of the privacy-preserving output
    pub zkp_id: Option<String>, // Optional link to a ZeroKnowledgeProof used
    pub privacy_policy_id: String, // Link to the PrivacyPolicy applied
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the computation process
    pub security_integrity: VerificationStatus, // New: Security integrity of the computation
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of Zero-Knowledge Proofs and privacy-preserving AI usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZKPAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_zkp_usage", "sensitive_data_computation"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_proofs_generated: u64,
    pub detected_privacy_violations: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of ZKP usage
    pub security_integrity_status: VerificationStatus, // Overall security integrity of ZKP systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
