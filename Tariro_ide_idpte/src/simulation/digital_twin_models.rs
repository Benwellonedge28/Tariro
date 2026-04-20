// Tariro_ide_idpte/src/simulation/digital_twin_models.rs

//! Defines data models for Tariro's Digital Twin & Simulation Environment module,
//! including digital twin models, simulation scenarios, and simulation results.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A high-fidelity virtual replica of Tariro (or a subsystem) and its operational environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinModel {
    pub id: String,
    pub model_name: String,
    pub scope_description: String, // e.g., "full_agi_cognitive_architecture", "resource_management_subsystem"
    pub fidelity_metrics: HashMap<String, f64>, // e.g., "accuracy_real_world_prediction": 0.95
    pub data_sources_sync: Vec<String>, // IDs of DataManager sources
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the digital twin's design
    pub security_verified: VerificationStatus, // New: Security verification of digital twin integrity
    pub last_updated: DateTime<Utc>,
}

/// Parameters and conditions for a specific simulation run within the digital twin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenario {
    pub id: String,
    pub twin_model_id: String,
    pub scenario_name: String,
    pub environmental_inputs: HashMap<String, String>, // e.g., "external_temp": "25C"
    pub agi_internal_state_start: String, // Summary of starting internal state
    pub test_hypothesis: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the simulation scenario
    pub security_audited: VerificationStatus, // New: Security audit of the scenario for vulnerabilities
    pub created_at: DateTime<Utc>,
}

/// The outcomes and analysis of a simulation run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub id: String,
    pub scenario_id: String,
    pub outcome_summary: String,
    pub observed_agi_behavior_summary: String,
    pub insights_extracted: Vec<String>,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical assessment of simulation outcomes
    pub security_integrity: VerificationStatus, // New: Security integrity of simulation results
    pub completed_at: DateTime<Utc>,
}

/// A report summarizing the audit of digital twin and simulation activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_simulations", "critical_failure_scenarios"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_simulations_run: u64,
    pub detected_anomalies: Vec<String>, // e.g., "unrealistic_behavior", "simulation_drift"
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of simulation environment
    pub security_integrity_status: VerificationStatus, // Overall security integrity of simulation systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
