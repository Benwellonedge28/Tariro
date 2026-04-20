// Tariro_ide_idpte/src/environmental/societal_impact_models.rs

//! Defines data models for Tariro's Societal Impact Forecasting & Mitigation module,
//! including societal impact scenarios, impact assessments, and mitigation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A hypothetical future outcome or chain of events with potential societal consequences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalImpactScenario {
    pub id: String,
    pub scenario_name: String,
    pub description: String,
    pub drivers: Vec<String>, // Factors leading to the scenario
    pub predicted_effects: HashMap<String, String>, // e.g., "economic_impact": "high_growth", "social_cohesion": "decreased"
    pub ethical_risk_assessment: VerificationStatus, // New: Ethical risk assessment of the scenario itself
    pub security_vulnerability_assessment: VerificationStatus, // New: Security vulnerability assessment of scenario
    pub created_at: DateTime<Utc>,
}

/// An evaluation of an AGI action's or solution's potential societal effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub id: String,
    pub action_id: String, // ID of the AGI action or solution being assessed
    pub scenario_id: String, // Link to SocietalImpactScenario
    pub predicted_positive_impacts: Vec<String>,
    pub predicted_negative_impacts: Vec<String>,
    pub confidence: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the impact assessment
    pub data_privacy_compliance: VerificationStatus, // New: Data privacy compliance for assessment data
    pub assessed_at: DateTime<Utc>,
}

/// A plan to reduce or prevent identified negative societal impacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub id: String,
    pub impact_assessment_id: String, // Link to ImpactAssessment
    pub strategy_name: String,
    pub proposed_actions: Vec<String>,
    pub estimated_cost: String,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the strategy design
    pub security_audited: VerificationStatus, // New: Security audit of strategy implementation
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of societal impact forecasting and mitigation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalImpactAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_agi_actions", "critical_societal_interventions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_assessments_performed: u64,
    pub total_mitigations_deployed: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of societal impact processes
    pub security_integrity_status: VerificationStatus, // Overall security integrity of societal impact systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
