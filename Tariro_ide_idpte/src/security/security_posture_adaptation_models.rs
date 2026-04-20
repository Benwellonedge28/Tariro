// Tariro_ide_idpte/src/security/security_posture_adaptation_models.rs

//! Defines data models for Tariro's Security Posture Adaptation module,
//! including threat assessments, security policy updates, and posture adaptation logs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A detailed assessment of a detected or potential cyber threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub id: String,
    pub threat_type: String, // e.g., "zero_day_exploit", "phishing_campaign", "insider_threat"
    pub target_component: String,
    pub severity: f64, // 0.0 to 1.0
    pub impact_prediction: String,
    pub ethical_implications: Vec<String>, // New: Ethical risks from the threat
    pub security_vulnerabilities: Vec<String>, // New: Specific vulnerabilities targeted
    pub assessed_at: DateTime<Utc>,
}

/// A proposed or implemented update to OMEGA OS's security policies or configurations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicyUpdate {
    pub id: String,
    pub threat_assessment_id: String, // Link to ThreatAssessment
    pub update_description: String,
    pub policy_changes: Vec<String>,
    pub expected_effectiveness: f64, // 0.0 to 1.0
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the policy update
    pub security_audited: VerificationStatus, // New: Security audit of the policy implementation
    pub executed_at: DateTime<Utc>,
}

/// A log of changes to Tariro's overall security posture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostureAdaptationLog {
    pub id: String,
    pub update_id: String, // Link to SecurityPolicyUpdate
    pub previous_posture_summary: String,
    pub new_posture_summary: String,
    pub adaptation_rationale: String,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of posture changes
    pub security_robustness_rating: VerificationStatus, // New: Overall security robustness after change
    pub logged_at: DateTime<Utc>,
}

/// A report summarizing the audit of security posture adaptation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_security_adaptations", "critical_infrastructure_defense"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_threats_detected: u64,
    pub policies_adapted: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of security posture
    pub security_integrity_status: VerificationStatus, // Overall security integrity of adaptation systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
