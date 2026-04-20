// Tariro_ide_idpte/src/agi_meta_goals/scientific_discovery_models.rs

//! Defines data models for Tariro's Autonomous Scientific Discovery & Experimentation module,
//! including hypotheses, experiment designs, research data, and scientific publications.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A testable prediction or explanation for a phenomenon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hypothesis {
    pub id: String,
    pub statement: String,
    pub predicted_outcome: String,
    pub supporting_evidence_hash: Option<String>, // Hash of foundational knowledge
    pub falsifiability_score: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // Ethical review of hypothesis implications
    pub formulated_at: DateTime<Utc>,
}

/// A detailed plan for conducting a scientific experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentDesign {
    pub id: String,
    pub hypothesis_id: String, // Link to Hypothesis
    pub methodology: String, // Step-by-step procedure
    pub variables: HashMap<String, String>, // Independent, dependent variables
    pub control_groups: Vec<String>,
    pub simulated_environment_id: Option<String>, // Link to DigitalTwin or simulation
    pub ethical_approval_status: VerificationStatus, // Ethical review of experiment design
    pub security_audited: VerificationStatus, // Security audit of experimental setup
    pub designed_at: DateTime<Utc>,
}

/// Collected raw or processed data from a scientific experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchData {
    pub id: String,
    pub experiment_id: String, // Link to ExperimentDesign
    pub data_type: String, // e.g., "sensor_readings", "simulation_logs", "qualitative_observations"
    pub storage_location_hash: String, // Hash of data location for integrity
    pub ethical_privacy_compliance: VerificationStatus, // Ethical review of data privacy
    pub collected_at: DateTime<Utc>,
}

/// A formal scientific paper or report ready for peer review and dissemination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScientificPublication {
    pub id: String,
    pub research_data_id: String, // Link to ResearchData
    pub title: String,
    pub abstract_summary: String,
    pub main_findings: Vec<String>,
    pub conclusion: String,
    pub ethical_disclosure_status: VerificationStatus, // Ethical review of publication content
    pub peer_review_status: VerificationStatus, // Placeholder for external review
    pub generated_at: DateTime<Utc>,
}

/// Audit report for autonomous scientific discovery and experimentation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_hypotheses_formulated: u64,
    pub total_experiments_conducted: u64,
    pub total_publications_generated: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
