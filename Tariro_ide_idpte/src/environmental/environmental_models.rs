// Tariro_ide_idpte/src/environmental/environmental_models.rs

//! Defines data models for Tariro's Environmental Impact Assessment module,
//! including environmental footprints, sustainability metrics, and eco-intervention strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents Tariro's measured environmental footprint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalFootprint {
    pub id: String,
    pub metric_type: String, // e.g., "energy_consumption_kwh", "carbon_emissions_kg", "water_usage_liters"
    pub value: f64,
    pub unit: String,
    pub source_module_id: String, // e.g., "SORAEE", "HardwareSecurityModule"
    pub timestamp: DateTime<Utc>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of footprint measurement/reporting
    pub security_verified: VerificationStatus, // New: Security of footprint data integrity
}

/// A quantifiable indicator of sustainability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainabilityMetric {
    pub id: String,
    pub name: String,
    pub description: String,
    pub target_value: f64,
    pub current_value: f64,
    pub trend_analysis: String, // e.g., "improving", "declining", "stable"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the metric definition/use
    pub integrity_proof: String, // New: Proof of tamper-proof metric definition
    pub last_updated: DateTime<Utc>,
}

/// A proposed strategy for an ecological intervention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoInterventionStrategy {
    pub id: String,
    pub name: String,
    pub target_ecological_system: String, // e.g., "local_biodiversity", "atmospheric_carbon_cycle"
    pub proposed_actions: Vec<String>, // e.g., "deploy_bio_nanobots", "optimize_photosynthesis_algorithm"
    pub expected_ecological_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the strategy
    pub security_review_status: VerificationStatus, // New: Security review of the strategy
    pub approved_by: String, // e.g., "Tariro_EIM", "HumanAdministrator"
    pub created_at: DateTime<Utc>,
}

/// A comprehensive report on Tariro's ecological impact and sustainability efforts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcologicalImpactReport {
    pub id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_footprint_summary: HashMap<String, f64>, // Aggregated EnvironmentalFootprint
    pub sustainability_metrics_status: HashMap<String, String>, // Metric ID -> Status
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance regarding environment
    pub security_integrity_status: VerificationStatus, // Overall security integrity of report
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
