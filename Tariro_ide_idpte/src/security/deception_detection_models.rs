// Tariro_ide_idpte/src/security/deception_detection_models.rs

//! Defines data models for Tariro's Deception Detection & Countermeasures module,
//! including suspected deception attempts, analysis reports, and counter-strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a suspected attempt at deception by an external agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeceptionAttempt {
    pub id: String,
    pub detected_by_module: String, // e.g., "TrustReputationModule", "NLPUnderstanding"
    pub observed_behavior: String, // Summary of the deceptive input/communication/action
    pub suspected_agent_id: String,
    pub inferred_intent: String, // From TheoryOfMind, e.g., "to mislead", "to gain unauthorized access"
    pub confidence_score: f64, // 0.0 to 1.0
    pub detected_at: DateTime<Utc>,
    pub ethical_concerns: VerificationStatus, // New: Ethical flag if deception is detected
}

/// A report detailing the analysis of a deception attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeceptionAnalysisReport {
    pub id: String,
    pub deception_attempt_id: String,
    pub analysis_method: String, // e.g., "cross_referencing_facts", "logical_consistency_check", "pattern_anomaly_detection"
    pub confirmed_deception: bool,
    pub deception_type: String, // e.g., "misinformation", "omission", "false_pretense"
    pub potential_impact_if_successful: String,
    pub recommended_countermeasure_id: Option<String>,
    pub integrity_verified: VerificationStatus, // New: Integrity of the analysis process
    pub reported_at: DateTime<Utc>,
}

/// A strategy for countering a detected deception attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterDeceptionStrategy {
    pub id: String,
    pub deception_id: String,
    pub name: String,
    pub proposed_actions: Vec<String>, // e.g., "request_clarification", "present_contradictory_facts", "ignore_agent", "alert_admin"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the countermeasure
    pub security_review_status: VerificationStatus, // New: Security review of the countermeasure
    pub approved_by: String, // e.g., "Tariro_DDM", "ALEC-A"
    pub created_at: DateTime<Utc>,
}

/// A record of a deployed countermeasure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountermeasureDeploymentRecord {
    pub id: String,
    pub strategy_id: String,
    pub deception_id: String,
    pub deployed_actions: Vec<String>,
    pub deployment_timestamp: DateTime<Utc>,
    pub effectiveness_assessment: String,
    pub ethical_compliance_after_deployment: VerificationStatus, // New: Ethical check after deployment
    pub security_compliance_after_deployment: VerificationStatus, // New: Security check after deployment
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
}
