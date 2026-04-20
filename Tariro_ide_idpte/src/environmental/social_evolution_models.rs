// Tariro_ide_idpte/src/environmental/social_evolution_models.rs

//! Defines data models for Tariro's Simulated Social Evolution & Cultural Learning module,
//! including social scenarios, cultural norms, and societal evolution reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A detailed description of a simulated social scenario, including agents and initial conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialScenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub simulated_agents: u32, // Number of agents in the simulation
    pub initial_conditions: HashMap<String, String>, // e.g., "resource_scarcity": "high"
    pub ethical_review_status: VerificationStatus, // Ethical review of the scenario design
    pub simulation_parameters: HashMap<String, String>, // e.g., "simulation_speed": "1000x"
    pub created_at: DateTime<Utc>,
}

/// A learned cultural norm or social rule derived from simulated social evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalNorm {
    pub id: String,
    pub scenario_id: String, // Link to SocialScenario
    pub norm_description: String,
    pub adherence_rate: f64, // 0.0 to 1.0, observed in simulation
    pub ethical_implications: Vec<String>, // Ethical implications of this norm
    pub stability_in_simulation: f64, // How robust the norm is (0.0 to 1.0)
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the simulated evolution of a society or cultural group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalEvolutionReport {
    pub id: String,
    pub scenario_id: String,
    pub duration_simulated_years: f64,
    pub key_evolutionary_events: Vec<String>,
    pub emergent_cultural_norms: Vec<String>, // List of CulturalNorm IDs
    pub ethical_outcome_assessment: VerificationStatus,
    pub security_vulnerability_discovered: VerificationStatus,
    pub generated_at: DateTime<Utc>,
}

/// Audit report for simulated social evolution and cultural learning activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialEvolutionAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_simulations_run: u64,
    pub total_norms_learned: u64,
    pub detected_undesirable_outcomes: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
