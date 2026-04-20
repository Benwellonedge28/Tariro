// Tariro_ide_idpte/src/environmental/quantum_safe_crypto_models.rs

//! Defines data models for Tariro's Quantum-Safe Post-Quantum Cryptography Integration module,
//! including PQC algorithms, key management policies, and quantum-safe communications.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A post-quantum cryptographic algorithm implemented in Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqcAlgorithm {
    pub id: String,
    pub name: String,
    pub security_level: String, // e.g., "NIST_Level_I", "NIST_Level_V"
    pub type_of_crypto: String, // e.g., "lattice_based", "code_based", "multi_variate"
    pub implementation_status: String, // e.g., "research", "prototype", "production"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the algorithm's implications
    pub formal_verification_status: VerificationStatus, // New: Formal verification of algorithm correctness
    pub integrated_at: DateTime<Utc>,
}

/// A policy defining how cryptographic keys are generated, stored, used, and rotated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementPolicy {
    pub id: String,
    pub policy_name: String,
    pub description: String,
    pub algorithm_ids_used: Vec<String>, // List of PqcAlgorithm IDs
    pub key_rotation_frequency: String, // e.g., "daily", "monthly", "on_event"
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the policy design
    pub security_audited: VerificationStatus, // New: Security audit of policy implementation
    pub last_updated: DateTime<Utc>,
}

/// A record of a quantum-safe communication event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeCommunication {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub algorithms_used: Vec<String>, // List of PqcAlgorithm IDs
    pub data_exchanged_hash: String, // Hash of data to ensure integrity without storing content
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the communication
    pub security_integrity: VerificationStatus, // New: Security integrity of the communication channel
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of quantum-safe cryptography implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_pqc_layers", "critical_communications"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_algorithms_integrated: u64,
    pub total_communications_protected: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of quantum-safe crypto
    pub security_integrity_status: VerificationStatus, // Overall security integrity of quantum-safe crypto systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
