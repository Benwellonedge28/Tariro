// Tariro_ide_idpte/src/crypto_fabric/secure_multi_party_models.rs

//! Defines data models for Tariro's Secure Multi-Party Computation for Data Collaboration module,
//! including SMPC sessions, shared data fragments, and computation result proofs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A descriptor for a secure multi-party computation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMPC_Session {
    pub id: String,
    pub session_name: String,
    pub participants: Vec<String>, // List of entity IDs involved
    pub computation_goal: String, // e.g., "average_salary_calculation", "private_ml_model_training"
    pub protocol_used: String, // e.g., "GarbledCircuits", "HomomorphicEncryption", "SecretSharing"
    pub ethical_privacy_review: VerificationStatus, // Ethical review of the privacy guarantees
    pub security_integrity_status: VerificationStatus, // Security of the protocol implementation
    pub initiated_at: DateTime<Utc>,
}

/// An encrypted or obfuscated fragment of data contributed by a participant to an SMPC session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedDataFragment {
    pub id: String,
    pub smpc_session_id: String, // Link to SMPC_Session
    pub participant_id: String,
    pub encrypted_payload_hash: String, // Hash of the encrypted data
    pub data_privacy_level: f64, // 0.0 to 1.0, level of privacy protection
    pub ethical_data_handling_review: VerificationStatus, // Ethical review of data handling
    pub submitted_at: DateTime<Utc>,
}

/// A cryptographically verifiable proof that a computation result is correct, without revealing inputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationResultProof {
    pub id: String,
    pub smpc_session_id: String, // Link to SMPC_Session
    pub result_summary: String,
    pub proof_type: String, // e.g., "Zero-Knowledge Proof", "Homomorphic Decryption Proof"
    pub proof_status: VerificationStatus, // Verification status of the proof
    pub ethical_integrity: VerificationStatus, // Ethical review of the computation fairness
    pub security_audited: VerificationStatus, // Security audit of the proof generation
    pub generated_at: DateTime<Utc>,
}

/// Audit report for secure multi-party computation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMPC_AuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_smpc_sessions: u64,
    pub total_data_fragments_processed: u64,
    pub detected_privacy_breaches: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
