// Tariro_ide_idpte/src/crypto_fabric/verifiable_ai_models.rs

//! Defines data models for Tariro's Zero-Knowledge Proofs for Verifiable AI Operations module,
//! including AI operation statements, ZKPs for AI, and verification protocols.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A formal statement describing a specific behavior or outcome of an AI operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AI_OperationStatement {
    pub id: String,
    pub operation_id: String, // ID of the AI function or model run
    pub statement_type: String, // e.g., "correctness_of_output", "privacy_compliance", "ethical_adherence"
    pub formal_assertion: String, // Logical assertion about the operation
    pub security_assurance_level: f64, // 0.0 to 1.0, confidence in the assertion
    pub ethical_review_status: VerificationStatus, // Ethical review of the statement validity
    pub created_at: DateTime<Utc>,
}

/// A Zero-Knowledge Proof (ZKP) specifically generated to verify an AI operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroKnowledgeProof_AI {
    pub id: String,
    pub statement_id: String, // Link to AI_OperationStatement
    pub proof_data_hash: String, // Hash of the ZKP data
    pub verification_status: VerificationStatus, // Result of ZKP verification
    pub prover_id: String, // ID of the entity that generated the proof (e.g., Tariro module, external prover)
    pub verifier_id: String, // ID of the entity that verified the proof
    pub ethical_compliance: VerificationStatus, // Ethical compliance of the ZKP process itself
    pub security_audited: VerificationStatus, // Security audit of the ZKP mechanism
    pub generated_at: DateTime<Utc>,
}

/// A set of rules and procedures for verifying the correctness or compliance of AI operations using ZKPs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationProtocol {
    pub id: String,
    pub protocol_name: String,
    pub ai_component_target: String, // Target AI component for verification
    pub zkp_scheme_used: String, // e.g., "SNARK", "STARK", "Bulletproofs"
    pub privacy_guarantee_level: f64, // 0.0 to 1.0
    pub ethical_governance_policy_id: String, // Link to applicable ethical policy
    pub security_verified: VerificationStatus, // Security verification of the protocol
    pub last_updated: DateTime<Utc>,
}

/// Audit report for Zero-Knowledge Proofs for Verifiable AI Operations processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiableAI_AuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_zkps_generated: u64,
    pub total_zkps_verified: u64,
    pub detected_proof_failures: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
