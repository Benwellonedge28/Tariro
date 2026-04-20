// Tariro_ide_idpte/src/security/threat_anticipation_models.rs

//! Defines data models for Tariro's Proactive Threat Anticipation & Pre-emption module,
//! including threat anticipation reports, pre-emptive action plans, and threat landscape models.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A detailed forecast and analysis of potential future threats across various domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnticipationReport {
    pub id: String,
    pub forecast_period: String, // e.g., "next_24_hours", "next_week", "long_term"
    pub domain: String, // e.g., "cyber", "physical", "social", "cognitive"
    pub predicted_threat_type: String,
    pub likelihood: f64, // 0.0 to 1.0
    pub severity: f64, // 0.0 to 1.0
    pub ethical_impact_assessment: VerificationStatus, // Ethical review of potential threat impact
    pub generated_at: DateTime<Utc>,
}

/// A set of autonomously generated actions designed to neutralize or mitigate an anticipated threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreEmptiveActionPlan {
    pub id: String,
    pub report_id: String, // Link to ThreatAnticipationReport
    pub plan_summary: String,
    pub actions: Vec<String>, // List of specific pre-emptive actions
    pub expected_effectiveness: f64, // 0.0 to 1.0
    pub ethical_compliance_status: VerificationStatus, // Ethical review of proposed actions
    pub security_audited: VerificationStatus, // Security audit of the action deployment
    pub created_at: DateTime<Utc>,
}

/// A dynamic model representing the current and evolving global threat environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatLandscapeModel {
    pub id: String,
    pub model_version: String,
    pub last_updated: DateTime<Utc>,
    pub active_threats: HashMap<String, f64>, // Threat type -> Severity score
    pub emerging_threat_vectors: Vec<String>,
    pub ethical_considerations_overview: String,
    pub security_resilience_score: f64, // 0.0 to 1.0, overall system resilience
}

/// Audit report for proactive threat anticipation and pre-emption processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnticipationAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_threats_forecasted: u64,
    pub total_pre_emptive_actions: u64,
    pub successful_pre_emptions: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
