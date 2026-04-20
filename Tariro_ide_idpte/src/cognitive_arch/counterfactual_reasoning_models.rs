// Tariro_ide_idpte/src/cognitive_arch/counterfactual_reasoning_models.rs

//! Defines data models for Tariro's Counterfactual Reasoning & Scenario Planning module,
//! including scenario definitions, counterfactual outcomes, and intervention analyses.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A detailed description of a hypothetical scenario or past event with altered variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioDefinition {
    pub id: String,
    pub name: String,
    pub base_event_description: String,
    pub altered_variables: HashMap<String, String>, // e.g., "policy_change": "universal_basic_income"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the scenario's premises
    pub security_implications_assessment: VerificationStatus, // New: Security assessment of hypothetical changes
    pub created_at: DateTime<Utc>,
}

/// The predicted outcome of a hypothetical scenario or counterfactual analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualOutcome {
    pub id: String,
    pub scenario_id: String, // Link to ScenarioDefinition
    pub predicted_events: Vec<String>,
    pub likelihood_score: f64, // 0.0 to 1.0
    pub ethical_consequences: Vec<String>, // New: Ethical implications of the predicted outcome
    pub security_risks: Vec<String>, // New: Security risks identified in the predicted outcome
    pub generated_at: DateTime<Utc>,
}

/// An analysis of a proposed intervention within a simulated scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionAnalysis {
    pub id: String,
    pub scenario_id: String, // Link to ScenarioDefinition
    pub proposed_intervention: String,
    pub predicted_impact: String,
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of the intervention
    pub security_robustness: VerificationStatus, // New: Security robustness of the intervention
    pub analyzed_at: DateTime<Utc>,
}

/// A report summarizing the audit of counterfactual reasoning and scenario planning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_what_if_analysis", "critical_policy_simulations"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_scenarios_simulated: u64,
    pub interventions_evaluated: u64,
    pub detected_biases: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of counterfactual reasoning
    pub security_integrity_status: VerificationStatus, // Overall security integrity of scenario planning systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
