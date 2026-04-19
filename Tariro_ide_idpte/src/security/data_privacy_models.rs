// Tariro_ide_idpte/src/security/data_privacy_models.rs

//! Defines data models for Tariro's Data Privacy & Anonymization module,
//! including sensitive data records, anonymization policies, and privacy audit reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a record of sensitive data identified within Tariro's systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveDataRecord {
    pub id: String,
    pub data_id: String,
    pub classification: String, // e.g., "PII", "PHI", "Confidential", "Secret"
    pub source_module_id: String,
    pub detected_features: Vec<String>, // e.g., "name", "email", "medical_record_number"
    pub privacy_level_required: String, // e.g., "anonymized", "pseudonymized", "encrypted"
    pub detected_at: DateTime<Utc>,
    pub integrity_verified: VerificationStatus, // New: Integrity of the data record
}

/// A policy defining how sensitive data should be anonymized or protected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationPolicy {
    pub id: String,
    pub name: String,
    pub target_data_class: String, // e.g., "PII"
    pub technique: String, // e.g., "differential_privacy", "k_anonymity", "tokenization"
    pub parameters: HashMap<String, String>, // e.g., "epsilon": "0.1" for differential privacy
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the policy
    pub security_review_status: VerificationStatus, // New: Security review of the policy
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the results of a privacy audit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditReport {
    pub id: String,
    pub audit_period_start: DateTime<Utc>,
    pub audit_period_end: DateTime<Utc>,
    pub compliance_score: f64, // 0.0 to 1.0
    pub non_compliant_data: Vec<String>, // IDs of non-compliant data records
    pub recommended_actions: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // New: Ethical compliance of the audit process
    pub security_integrity_status: VerificationStatus, // New: Security integrity of the audit process
    pub generated_at: DateTime<Utc>,
}

/// A record of explicit consent obtained for data usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataUsageConsent {
    pub id: String,
    pub user_id: String,
    pub data_categories: Vec<String>, // e.g., "location_data", "biometric_data"
    pub purpose_of_use: String, // e.g., "service_improvement", "research"
    pub consent_granted: bool,
    pub timestamp: DateTime<Utc>,
    pub recorded_by_qrctfo: VerificationStatus, // New: Record integrity verified by QR-CTFO
    pub policy_version: String,
}
