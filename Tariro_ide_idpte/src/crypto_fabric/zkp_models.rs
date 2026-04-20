// Tariro_ide_idpte/src/crypto_fabric/zkp_models.rs

//! Defines data models for Tariro's Zero-Knowledge Proofs & Privacy-Preserving AI module,
//! including zero-knowledge proofs, privacy policy compliance, and verifiable computation tasks.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a generated Zero-Knowledge Proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroKnowledgeProof {
    pub id: String,
    pub prover_id: String,
    pub verifier_id: String,
    pub assertion_summary: String, // What the proof proves (e.g., "input X is within range [A,B]")
    pub proof_data: Vec<u8>, // The actual ZKP bytes
    pub proof_type: String, // e.g., "SNARK", "STARK", "Bulletproofs"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the proof's purpose and context
    pub security_verified: VerificationStatus, // New: Security verification of the proof generation process
    pub generated_at: DateTime<Utc>,
}

/// Assessment of whether a computation or data handling adheres to privacy policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyPolicyCompliance {
    pub id: String,
    pub policy_id: String, // ID of the relevant privacy policy
    pub target_computation_id: String,
    pub compliance_status: bool,
    pub violations_identified: Vec<String>,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of non-compliance
    pub security_breach_potential: VerificationStatus, // New: Security assessment of non-compliance
    pub evaluated_at: DateTime<Utc>,
}

/// A task that requires verifiable computation using privacy-preserving techniques.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableComputationTask {
    pub id: String,
    pub task_name: String,
    pub input_data_hash: String, // Hash of sensitive input data (not the data itself)
    pub computation_logic_hash: String, // Hash of the computation logic
    pub expected_output_format: String,
    pub required_privacy_level: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the computation task
    pub security_review_status: VerificationStatus, // New: Security review of the computation task
    pub requested_by_module: String,
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of privacy-preserving operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditReport {
    pub id: String,
    pub computation_task_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub zkp_usage_summary: String,
    pub data_exposure_risk: f64, // 0.0 (no risk) to 1.0 (high risk)
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of privacy-preserving operations
    pub security_integrity_status: VerificationStatus, // Overall security integrity of privacy-preserving systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
