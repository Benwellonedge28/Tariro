// Tariro_ide_idpte/src/crypto_fabric/quantum_safe_models.rs

//! Defines data models for Tariro's Quantum-Safe Encryption module,
//! including quantum-safe keys, ciphers, and encryption policy reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a quantum-safe cryptographic key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeKey {
    pub id: String,
    pub algorithm: String, // e.g., "Dilithium", "Kyber", "Falcon"
    pub key_type: String, // e.g., "public", "private", "symmetric"
    pub key_strength_bits: u32,
    pub usage_purpose: String, // e.g., "data_encryption", "digital_signature", "key_exchange"
    pub creation_timestamp: DateTime<Utc>,
    pub expiration_timestamp: DateTime<Utc>,
    pub integrity_hash: String, // New: Hash to ensure key integrity
    pub storage_location: String, // New: Secure storage location
}

/// Represents a quantum-safe cipher suite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeCipher {
    pub id: String,
    pub name: String,
    pub encryption_algorithm: String,
    pub key_exchange_algorithm: String,
    pub digital_signature_algorithm: String,
    pub post_quantum_strength: f64, // 0.0 to 1.0, based on current known attacks
    pub standardization_status: String, // e.g., "NIST_candidate", "IETF_standard"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the cipher
    pub security_review_status: VerificationStatus, // New: Security review of the cipher
}

/// A report on compliance with quantum-safe encryption policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionPolicyReport {
    pub id: String,
    pub audit_period_start: DateTime<Utc>,
    pub audit_period_end: DateTime<Utc>,
    pub compliance_score: f64, // 0.0 to 1.0
    pub non_compliant_areas: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub integrity_proof: String, // New: Proof that the report itself is tamper-proof
    pub generated_at: DateTime<Utc>,
}

/// Analysis of current and future post-quantum vulnerabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumThreatAnalysis {
    pub id: String,
    pub analysis_timestamp: DateTime<Utc>,
    pub predicted_quantum_attack_date: Option<DateTime<Utc>>,
    pub current_vulnerable_systems: Vec<String>,
    pub recommended_upgrades: Vec<String>,
    pub risk_score: f64, // 0.0 to 1.0
    pub ethical_considerations: String, // New: Ethical implications of quantum threat (from ALEC-A)
    pub security_integrity_status: VerificationStatus, // New: Integrity of the analysis
}
