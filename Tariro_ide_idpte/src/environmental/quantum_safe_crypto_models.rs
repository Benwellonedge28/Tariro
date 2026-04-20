// Tariro_ide_idpte/src/environmental/quantum_safe_crypto_models.rs

//! Defines data models for Tariro's Quantum-Safe Post-Quantum Cryptography Integration module,
//! including PQC algorithms, quantum-safe channels, and key management policies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A specification of a Post-Quantum Cryptography (PQC) algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PQCAlgorithm {
    pub id: String,
    pub algorithm_name: String,
    pub security_level: String, // e.g., "NIST_Level_3", "NIST_Level_5"
    pub description: String,
    pub key_size_bits: u32,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the algorithm's properties
    pub performance_metrics: HashMap<String, f64>, // e.g., "encryption_speed", "signature_latency"
    pub deployed_at: DateTime<Utc>,
}

/// A secure communication channel protected by quantum-safe cryptography.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeChannel {
    pub id: String,
    pub channel_name: String,
    pub connected_endpoints: Vec<String>,
    pub active_pqc_algorithms: Vec<String>, // List of PQCAlgorithm IDs
    pub data_encryption_status: VerificationStatus, // New: Verification of data encryption
    pub key_exchange_protocol: String,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of channel use
    pub security_integrity: VerificationStatus, // New: Security integrity of the channel
    pub established_at: DateTime<Utc>,
}

/// A policy governing the generation, storage, rotation, and revocation of cryptographic keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementPolicy {
    pub id: String,
    pub policy_name: String,
    pub description: String,
    pub rotation_frequency_days: u32,
    pub algorithm_flexibility: bool, // Can easily switch algorithms
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the key management policy
    pub security_audited: VerificationStatus, // New: Security audit of the policy implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of PQC integration and operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PQCIntegrationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_pqc_deployments", "critical_infrastructure_pqc"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_algorithms_deployed: u64,
    pub successful_key_rotations: u64,
    pub detected_pqc_vulnerabilities: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of PQC integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of PQC systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
