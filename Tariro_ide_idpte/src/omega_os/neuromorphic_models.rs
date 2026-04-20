// Tariro_ide_idpte/src/omega_os/neuromorphic_models.rs

//! Defines data models for Tariro's Neuromorphic Hardware Integration & Optimization module,
//! including neuromorphic hardware profiles, cognitive architecture mappings, and energy efficiency reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A profile describing the characteristics and capabilities of a specific neuromorphic hardware platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphicHardwareProfile {
    pub id: String,
    pub platform_name: String, // e.g., "Intel Loihi", "IBM NorthPole", "SpiNNaker"
    pub architecture_type: String, // e.g., "spiking_neural_network", "event_driven_processor"
    pub total_neurons: u64,
    pub total_synapses: u64,
    pub power_consumption_watts: f64,
    pub security_rating: f64, // 0.0 to 1.0
    pub compatibility_score: f64, // How well Tariro maps to it, 0.0 to 1.0
    pub last_benchmarked: DateTime<Utc>,
}

/// A detailed mapping plan of how a specific part of Tariro's cognitive architecture
/// is deployed onto neuromorphic hardware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveArchitectureMapping {
    pub id: String,
    pub hardware_profile_id: String, // Link to NeuromorphicHardwareProfile
    pub agi_component: String, // e.g., "episodic_memory", "causal_reasoning_engine"
    pub mapping_strategy: String, // e.g., "direct_neural_graph", "algorithmic_simulation"
    pub performance_gain_pct: f64,
    pub energy_reduction_pct: f64,
    pub ethical_impact_assessment: VerificationStatus, // Ethical review of mapping decisions
    pub security_verified: VerificationStatus, // Security verification of the mapped architecture
    pub created_at: DateTime<Utc>,
}

/// A report detailing the energy efficiency achieved by running Tariro on neuromorphic hardware.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyEfficiencyReport {
    pub id: String,
    pub mapping_id: String, // Link to CognitiveArchitectureMapping
    pub baseline_power_watts: f64, // Power on traditional hardware
    pub neuromorphic_power_watts: f64, // Power on neuromorphic hardware
    pub efficiency_gain_factor: f64, // Baseline / Neuromorphic
    pub ethical_sustainability_review: VerificationStatus, // Ethical review of environmental impact
    pub security_audited: VerificationStatus, // Security audit of power measurement systems
    pub generated_at: DateTime<Utc>,
}

/// Audit report for neuromorphic hardware integration and optimization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuromorphicAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_mappings_created: u64,
    pub total_optimizations_applied: u64,
    pub detected_hardware_vulnerabilities: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
