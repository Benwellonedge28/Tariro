// Tariro_ide_idpte/src/crypto_fabric/digital_identity_models.rs

//! Defines data models for Tariro's Quantum-Resistant Digital Identity & Access Management module,
//! including quantum-resistant identities, access policies, and identity verification reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A digital identity secured against both classical and future quantum computing attacks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumResistantIdentity {
    pub id: String,
    pub entity_type: String, // e.g., "AGI_module", "Human_user", "External_service"
    pub public_key_qr: String, // Quantum-resistant public key
    pub identity_descriptor: String, // Human-readable descriptor
    pub security_assurance_level: f64, // 0.0 to 1.0, based on QR algorithm strength
    pub ethical_privacy_review: VerificationStatus, // Ethical review of identity data privacy
    pub created_at: DateTime<Utc>,
}

/// A rule governing access to Tariro's internal resources or external services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    pub id: String,
    pub policy_name: String,
    pub target_resource_id: String,
    pub required_identity_attributes: HashMap<String, String>, // e.g., "role": "super_admin", "location": "secure_enclave"
    pub access_granted_status: VerificationStatus, // Verification status of the policy logic
    pub quantum_resistant_signature: String, // Signed by QR-CTFO
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the verification process for a digital identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationReport {
    pub id: String,
    pub identity_id: String, // Link to QuantumResistantIdentity
    pub verification_method: String,
    pub verification_result: VerificationStatus, // Passed or Failed
    pub detected_anomalies: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Ethical review of the verification process
    pub security_integrity_status: VerificationStatus, // Security of the verification process
    pub generated_at: DateTime<Utc>,
}

/// Audit report for quantum-resistant digital identity and access management processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_identities_managed: u64,
    pub total_access_grants_denied: u64,
    pub detected_identity_thefts: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
