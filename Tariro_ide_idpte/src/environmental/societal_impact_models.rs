// Tariro_ide_idpte/src/environmental/societal_impact_models.rs

//! Defines data models for Tariro's Societal Impact Forecasting & Mitigation module,
//! including societal impact scenarios, mitigation strategies, and societal forecasts.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A projected future scenario detailing the potential societal effects of AGI actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalImpactScenario {
    pub id: String,
    pub originating_agi_action_id: String,
    pub scenario_summary: String,
    pub predicted_positive_impacts: Vec<String>,
    pub predicted_negative_impacts: Vec<String>,
    pub probability: f64, // 0.0 to 1.0
    pub time_horizon: String, // e.g., "short-term", "medium-term", "long-term"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the scenario's predictions
    pub security_verified: VerificationStatus, // New: Security verification of scenario data integrity
    pub generated_at: DateTime<Utc>,
}

/// A plan or set of actions designed to reduce negative societal impacts or enhance positive ones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub id: String,
    pub scenario_id: String,
    pub strategy_name: String,
    pub proposed_actions: Vec<String>, // e.g., "launch_educational_campaign", "implement_policy_safeguard", "adjust_agi_behavior"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the mitigation strategy
    pub security_review_status: VerificationStatus, // New: Security review of the mitigation strategy
    pub approved_by: String, // e.g., "Tariro_SIFM", "ALEC-A", "HumanOversightCommittee"
    pub created_at: DateTime<Utc>,
}

/// A prediction of future societal changes or trends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalForecast {
    pub id: String,
    pub forecast_topic: String, // e.g., "employment_rates_ai", "global_equity_index", "cultural_cohesion_score"
    pub predicted_value_or_trend: String,
    pub confidence: f64, // 0.0 to 1.0
    pub influencing_factors: Vec<String>,
    pub ethical_bias_assessed: VerificationStatus, // New: Assessment for inherent biases in the forecast
    pub security_integrity: VerificationStatus, // New: Security integrity of the forecast data
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of societal impact forecasting and mitigation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAuditReport {
    pub id: String,
    pub strategy_id: String, // Link to MitigationStrategy
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub actions_taken_summary: String,
    pub actual_impact_observed: String,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of impact management
    pub security_integrity_status: VerificationStatus, // Overall security integrity of impact management systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
