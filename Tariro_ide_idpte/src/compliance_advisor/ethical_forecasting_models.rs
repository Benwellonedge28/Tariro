// Tariro_ide_idpte/src/compliance_advisor/ethical_forecasting_models.rs

//! Defines data models for Tariro's Proactive Ethical Forecasting module,
//! including ethical dilemmas, consequence predictions, and mitigation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents an autonomously anticipated ethical dilemma.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalDilemma {
    pub id: String,
    pub context_summary: String, // Description of the situation where the dilemma might arise
    pub conflicting_axioms: Vec<String>, // IDs or descriptions of core ethical axioms in conflict
    pub potential_actions: Vec<String>, // Possible actions Tariro could take
    pub probability_of_dilemma: f64, // 0.0 to 1.0
    pub source_module_id: String, // Module that triggered the anticipation (e.g., Imagination, PredictiveForecasting)
    pub detected_at: DateTime<Utc>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the dilemma description itself
}

/// A prediction of the ethical consequences of a given action or scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConsequencePrediction {
    pub id: String,
    pub dilemma_id: String, // Link to the EthicalDilemma it addresses
    pub action_or_scenario_summary: String,
    pub predicted_ethical_impact: HashMap<String, f64>, // e.g., "harm_to_sentience": 0.7, "fairness_violation": 0.3
    pub confidence: f64,
    pub ethical_justification_summary: String, // From ALEC-A
    pub security_implications_summary: Option<String>, // New: Security assessment of the prediction process
    pub predicted_at: DateTime<Utc>,
}

/// A proposed strategy to mitigate negative ethical consequences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalMitigationStrategy {
    pub id: String,
    pub dilemma_id: String,
    pub name: String,
    pub proposed_actions: Vec<String>, // Specific steps to take
    pub expected_ethical_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the strategy itself
    pub security_review_status: VerificationStatus, // New: Security review of the strategy
    pub approved_by: String, // e.g., "Tariro_ALEC-A", "HumanAdministrator"
    pub created_at: DateTime<Utc>,
}

/// A comprehensive report on a proactively analyzed ethical situation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProactiveEthicalReport {
    pub id: String,
    pub dilemma_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub dilemma_summary: String,
    pub consequence_predictions: Vec<String>, // IDs of EthicalConsequencePredictions
    pub mitigation_strategies: Vec<String>, // IDs of EthicalMitigationStrategies
    pub overall_ethical_risk: f64, // Aggregated risk score
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
