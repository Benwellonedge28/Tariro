// Tariro_ide_idpte/src/environmental/quantum_safe_models.rs

//! Defines data models for Tariro's Quantum-Safe Post-Quantum Cryptography Integration module,
//! including PQC algorithms, encryption keys, and secure channel statuses.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Details of a post-quantum cryptographic algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeAlgorithm {
    pub id: String,
    pub algorithm_name: String, // e.g., "Kyber", "Dilithium", "Falcon"
    pub algorithm_type: String, // e.g., "encryption", "digital_signature"
    pub security_level: u32, // NIST security level 1-5
    pub ethical_risk_assessment: VerificationStatus, // New: Ethical assessment of algorithm implications (e.g., performance impact)
    pub security_verified: VerificationStatus, // New: Security verification of algorithm implementation
    pub integrated_at: DateTime<Utc>,
}

/// A post-quantum encryption or signature key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionKey {
    pub id: String,
    pub algorithm_id: String, // Link to QuantumSafeAlgorithm
    pub key_type: String, // e.g., "public", "private"
    pub key_hash: String, // Hash of key to ensure integrity without storing key material
    pub ethical_key_usage_policy: VerificationStatus, // New: Ethical policy for key usage
    pub key_security_status: VerificationStatus, // New: Security status of key storage and lifecycle
    pub generated_at: DateTime<Utc>,
}

/// Details of a quantum-safe communication channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureChannelStatus {
    pub id: String,
    pub connection_id: String,
    pub algorithms_negotiated: Vec<String>, // List of QuantumSafeAlgorithm IDs
    pub is_quantum_resistant: bool,
    pub ethical_privacy_compliance: VerificationStatus, // New: Ethical privacy compliance for channel
    pub channel_security_integrity: VerificationStatus, // New: Security integrity of the channel
    pub established_at: DateTime<Utc>,
}

/// A report summarizing the audit of quantum-safe systems and PQC integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_pqc_integrations", "critical_communication_links"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_algorithms_integrated: u64,
    pub total_keys_managed: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of quantum-safe systems
    pub security_integrity_status: VerificationStatus, // Overall security integrity of quantum-safe systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
