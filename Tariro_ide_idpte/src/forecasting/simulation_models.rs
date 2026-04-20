// Tariro_ide_idpte/src/forecasting/simulation_models.rs

//! Defines data models for Tariro's Digital Twin & Simulation Environment module,
//! including digital twin states, simulation runs, and environmental models.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A snapshot of the digital twin's internal state and simulated external environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinState {
    pub id: String,
    pub simulation_id: String,
    pub timestamp: DateTime<Utc>,
    pub agi_internal_state_summary: String, // e.g., "cognitive_load: high, active_goals: [G1, G2]"
    pub simulated_environment_summary: String, // e.g., "weather: clear, market_trend: stable"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the simulated state
    pub security_integrity: VerificationStatus, // New: Security verification of state data
}

/// A record of a simulation run within the digital twin environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationRun {
    pub id: String,
    pub scenario_description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String, // e.g., "running", "completed", "aborted"
    pub predicted_outcome_summary: String,
    pub actual_outcome_summary: Option<String>,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the simulation itself
    pub security_audited: VerificationStatus, // New: Security audit of the simulation environment
    pub generated_at: DateTime<Utc>,
}

/// Parameters defining the behavior and properties of the simulated environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalModel {
    pub id: String,
    pub model_name: String,
    pub parameters: HashMap<String, String>, // e.g., "temperature_range": "-10C_to_30C"
    pub fidelity_level: f64, // 0.0 to 1.0
    pub ethical_bias_assessment: VerificationStatus, // New: Ethical assessment of biases in environmental model
    pub security_vulnerability: VerificationStatus, // New: Security assessment of model vulnerabilities
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of digital twin operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwinAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_simulations", "critical_scenario_tests"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_simulations_run: u64,
    pub detected_anomalies: Vec<String>, // e.g., "simulation_drift", "unforeseen_interaction"
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of digital twin
    pub security_integrity_status: VerificationStatus, // Overall security integrity of digital twin systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
