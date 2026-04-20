// Tariro_ide_idpte/src/environmental/societal_models.rs

//! Defines data models for Tariro's Societal Impact Forecasting & Mitigation module,
//! including societal impact scenarios, socio-economic models, and mitigation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a simulated future scenario detailing potential societal impacts of AGI actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalImpactScenario {
    pub id: String,
    pub name: String,
    pub agi_action_context: String, // Description of the AGI action being simulated
    pub predicted_socio_economic_changes: HashMap<String, String>, // e.g., "employment_rate": "-5%", "gini_coefficient": "+0.02"
    pub predicted_ethical_shifts: Vec<String>, // e.g., "increased_privacy_concerns"
    pub severity_of_impact: f64, // 0.0 (low) to 1.0 (transformative/catastrophic)
    pub likelihood: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the scenario simulation
    pub security_verified: VerificationStatus, // New: Security verification of scenario data
    pub generated_at: DateTime<Utc>,
}

/// A complex model representing human society, including socio-economic, cultural, and ethical dynamics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocioEconomicModel {
    pub id: String,
    pub model_name: String,
    pub parameters: HashMap<String, f64>, // e.g., "population_growth_rate": 0.01, "avg_income": 50000.0
    pub ethical_constraints_applied: VerificationStatus, // New: Ethical review of model assumptions
    pub security_audited: VerificationStatus, // New: Security audit of model integrity
    pub last_updated: DateTime<Utc>,
}

/// A strategy composed of actions designed to prevent or mitigate negative societal impacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub id: String,
    pub scenario_id: String, // ID of the scenario this strategy addresses
    pub strategy_name: String,
    pub proposed_actions: Vec<String>, // e.g., "invest_in_retraining_programs", "implement_universal_basic_income"
    pub expected_impact_reduction: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the mitigation strategy
    pub security_review_status: VerificationStatus, // New: Security review of the mitigation strategy
    pub approved_by: String, // e.g., "Tariro_SIF", "ALEC-A", "HumanOversight"
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of societal impact forecasting and mitigation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalAuditReport {
    pub id: String,
    pub strategy_id: String, // ID of the MitigationStrategy
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub actual_impact_observed: String,
    pub alignment_score: f64, // 0.0 to 1.0, how well AGI operations aligned with societal well-being
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of forecasting and mitigation
    pub security_integrity_status: VerificationStatus, // Overall security integrity of these systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
