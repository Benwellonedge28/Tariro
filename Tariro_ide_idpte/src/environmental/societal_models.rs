// Tariro_ide_idpte/src/environmental/societal_models.rs

//! Defines data models for Tariro's Societal Impact Forecasting & Mitigation module,
//! including societal impact assessments, intervention strategies, and long-term forecasts.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents an evaluation of the potential societal effects of an AGI action or policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalImpactAssessment {
    pub id: String,
    pub agi_action_id: String, // ID of the AGI action or policy being assessed
    pub scope: String, // e.g., "global_economic", "regional_social", "ethical_norms"
    pub predicted_positive_impacts: Vec<String>,
    pub predicted_negative_impacts: Vec<String>,
    pub likelihood_of_impact: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the assessment itself
    pub security_verified: VerificationStatus, // New: Security verification of assessment data
    pub generated_at: DateTime<Utc>,
}

/// A plan for actions to mitigate predicted negative societal impacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionStrategy {
    pub id: String,
    pub assessment_id: String, // Link to SocietalImpactAssessment
    pub name: String,
    pub proposed_actions: Vec<String>, // e.g., "propose_new_policy", "launch_educational_campaign", "adjust_agi_behavior_param"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the intervention actions
    pub security_review_status: VerificationStatus, // New: Security review of the intervention actions
    pub approved_by: String, // e.g., "Tariro_SIF", "ALEC-A", "HumanOversightCommittee"
    pub created_at: DateTime<Utc>,
}

/// A long-term prediction of future societal states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermForecast {
    pub id: String,
    pub forecast_topic: String, // e.g., "global_economy_2050", "social_cohesion_trends"
    pub predicted_state_summary: String,
    pub confidence: f64, // 0.0 to 1.0
    pub ethical_bias_assessed: VerificationStatus, // New: Ethical bias assessment of the forecast model
    pub security_integrity: VerificationStatus, // New: Security integrity of forecast data
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of societal impact forecasting processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_agi_actions", "specific_policy_impact"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_assessments_audited: u64,
    pub detected_forecasting_biases: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of forecasting
    pub security_integrity_status: VerificationStatus, // Overall security integrity of forecasting systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
