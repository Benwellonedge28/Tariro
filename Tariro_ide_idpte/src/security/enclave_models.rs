// Tariro_ide_idpte/src/security/enclave_models.rs

//! Defines data models for Tariro's Secure Enclave & Trusted Execution Environments module,
//! including secure enclave configurations, trusted execution reports, and sensitive operations.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Configuration for a secure enclave or trusted execution environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureEnclaveConfig {
    pub id: String,
    pub enclave_type: String, // e.g., "Intel SGX", "ARM TrustZone", "Custom Hardware Enclave"
    pub allocated_resources: HashMap<String, String>, // e.g., "memory": "128MB", "cpu_cores": "2"
    pub allowed_operations: Vec<String>, // List of approved functions to run within the enclave
    pub integrity_measurement_hash: String, // Expected hash of the enclave's initial state
    pub ethical_review_status: VerificationStatus, // New: Ethical review of enclave configuration
    pub security_verified: VerificationStatus, // New: Security verification of enclave configuration
    pub created_at: DateTime<Utc>,
}

/// A report from a trusted execution environment after performing an operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedExecutionReport {
    pub id: String,
    pub operation_id: String,
    pub enclave_id: String,
    pub execution_status: String, // e.g., "success", "failure", "tamper_detected"
    pub output_hash: String, // Hash of the output data, verifiable outside the enclave
    pub attestation_report_link: String, // Link to hardware attestation report (from HardwareSecurityModule)
    pub ethical_compliance_after_exec: VerificationStatus, // New: Ethical check of operation within enclave
    pub security_integrity_after_exec: VerificationStatus, // New: Security integrity of operation within enclave
    pub timestamp: DateTime<Utc>,
}

/// Represents a sensitive operation that requires execution within a secure enclave.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveOperation {
    pub id: String,
    pub operation_name: String,
    pub input_data_hash: String, // Hash of input data, to verify integrity
    pub required_enclave_capabilities: Vec<String>, // e.g., "crypto_ops", "private_data_processing"
    pub expected_ethical_impact: VerificationStatus, // New: Expected ethical impact of this operation
    pub expected_security_risk: VerificationStatus, // New: Expected security risk of this operation
    pub requested_by_module: String,
    pub created_at: DateTime<Utc>,
}

/// A log entry for auditing secure enclave operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnclaveAuditLog {
    pub id: String,
    pub enclave_id: String,
    pub operation_id: String,
    pub audit_summary: String,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of enclave usage
    pub security_integrity_status: VerificationStatus, // Overall security integrity of enclave usage
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
