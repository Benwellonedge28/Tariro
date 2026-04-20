// Tariro_ide_idpte/src/environmental/societal_impact_models.rs

//! Defines data models for Tariro's Societal Impact Forecasting & Mitigation module,
//! including societal impact forecasts, mitigation strategies, and risk assessments.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A prediction of future societal changes or effects resulting from AGI actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalImpactForecast {
    pub id: String,
    pub agi_action_summary: String, // The AGI action being forecasted (e.g., "automate global logistics")
    pub predicted_effects: HashMap<String, String>, // e.g., "employment_rate": "-15%", "global_gdp": "+10%"
    pub confidence: f64, // 0.0 to 1.0
    pub forecast_horizon: String, // e.g., "5_years", "20_years"
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical assessment of the predicted impacts
    pub security_vulnerability_potential: VerificationStatus, // New: Security assessment of potential societal vulnerabilities
    pub generated_at: DateTime<Utc>,
}

/// A formulated plan to reduce negative societal impacts or enhance positive ones.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub id: String,
    pub forecast_id: String,
    pub strategy_name: String,
    pub proposed_actions: Vec<String>, // e.g., "invest_in_retraining_programs", "establish_universal_basic_income"
    pub expected_outcome_summary: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the mitigation strategy
    pub security_implications: VerificationStatus, // New: Security implications of the strategy
    pub approved_by: String, // e.g., "Tariro_SIFM", "ALEC-A", "HumanOversightCommittee"
    pub created_at: DateTime<Utc>,
}

/// An evaluation of potential societal risks associated with an AGI action or system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalRiskAssessment {
    pub id: String,
    pub agi_action_id: String, // The action or system being assessed
    pub risk_type: String, // e.g., "ethical_misalignment", "economic_disruption", "social_inequality"
    pub severity: f64, // 0.0 (low) to 1.0 (catastrophic)
    pub likelihood: f64, // 0.0 to 1.0
    pub identified_vulnerabilities: Vec<String>,
    pub ethical_analysis: VerificationStatus, // New: Deep ethical analysis of the risk
    pub security_audit: VerificationStatus, // New: Security audit of the risk assessment process
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of societal impact forecasting and mitigation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAuditReport {
    pub id: String,
    pub strategy_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub actual_impact_observed: HashMap<String, String>,
    pub effectiveness_of_mitigation: f64, // 0.0 to 1.0
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of impact management
    pub security_integrity_status: VerificationStatus, // Overall security integrity of impact management systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
