// Tariro_ide_idpte/src/security/security_posture_models.rs

//! Defines data models for Tariro's Security Posture Adaptation module,
//! including security threats, posture configurations, and adaptation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Details of a detected security threat or vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityThreat {
    pub id: String,
    pub threat_name: String,
    pub description: String,
    pub severity: f64, // 0.0 to 1.0
    pub affected_components: Vec<String>,
    pub attack_vector: String,
    pub ethical_implications: Vec<String>, // New: Ethical harms from the threat
    pub data_compromise_risk: VerificationStatus, // New: Data privacy risk assessment
    pub detected_at: DateTime<Utc>,
}

/// Represents Tariro's current security configuration and defensive capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPosture {
    pub id: String,
    pub configuration_name: String,
    pub active_defenses: Vec<String>, // e.g., "quantum_safe_encryption", "zero_trust_network"
    pub resilience_score: f64, // 0.0 to 1.0
    pub compliance_status: VerificationStatus, // New: Compliance with security policies
    pub ethical_integrity: VerificationStatus, // New: Ethical review of security measures
    pub last_updated: DateTime<Utc>,
}

/// A strategy for dynamically adjusting security defenses in response to threats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationStrategy {
    pub id: String,
    pub strategy_name: String,
    pub description: String,
    pub triggers: Vec<String>, // e.g., "high_severity_threat_detected", "new_vulnerability_identified"
    pub actions: Vec<String>, // e.g., "strengthen_encryption", "isolate_compromised_module"
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the adaptation strategy
    pub security_audited: VerificationStatus, // New: Security audit of the strategy implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of security posture adaptation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_security_events", "critical_system_defenses"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_threats_detected: u64,
    pub successful_adaptations: u64,
    pub detected_false_positives: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of security adaptation
    pub security_integrity_status: VerificationStatus, // Overall security integrity of adaptation systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
