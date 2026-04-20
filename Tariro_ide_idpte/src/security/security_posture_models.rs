// Tariro_ide_idpte/src/security/security_posture_models.rs

//! Defines data models for Tariro's Security Posture Adaptation module,
//! including threat detection, vulnerability assessments, and security policies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a detected security threat or attack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetection {
    pub id: String,
    pub threat_type: String, // e.g., "malware", "phishing", "zero_day_exploit"
    pub source_id: String, // e.g., IP address, attacker profile
    pub severity: f64, // 0.0 to 1.0
    pub affected_components: Vec<String>,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical assessment of threat impact
    pub data_exposure_risk: VerificationStatus, // New: Data exposure risk assessment
    pub detected_at: DateTime<Utc>,
}

/// An assessment of a system's vulnerability to known or predicted threats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityAssessment {
    pub id: String,
    pub component_id: String,
    pub vulnerability_type: String, // e.g., "software_bug", "misconfiguration", "protocol_weakness"
    pub risk_score: f64, // 0.0 to 1.0
    pub recommended_mitigation: String,
    pub ethical_considerations: VerificationStatus, // New: Ethical considerations for mitigation
    pub security_verified: VerificationStatus, // New: Security verification of assessment methodology
    pub assessed_at: DateTime<Utc>,
}

/// A set of rules and configurations defining Tariro's security posture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub id: String,
    pub policy_name: String,
    pub description: String,
    pub rules: Vec<String>, // e.g., "encrypt_all_data_at_rest", "require_multi_factor_authentication"
    pub enforcement_mechanisms: Vec<String>,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the policy design
    pub security_audited: VerificationStatus, // New: Security audit of policy implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of security posture adaptation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_security_changes", "critical_vulnerability_response"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_threats_detected: u64,
    pub total_policies_adapted: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of security adaptation
    pub security_integrity_status: VerificationStatus, // Overall security integrity of security adaptation systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
