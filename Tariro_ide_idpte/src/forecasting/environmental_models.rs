// Tariro_ide_idpte/src/forecasting/environmental_models.rs

//! Defines data models for Tariro's Environmental Impact Assessment & Mitigation module,
//! including ecological footprint metrics, environmental data points, and mitigation plans.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a quantified ecological footprint of Tariro's operations or a specific process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcologicalFootprint {
    pub id: String,
    pub entity_id: String, // ID of the operation/module/system being assessed
    pub footprint_metrics: HashMap<String, f64>, // e.g., "carbon_emissions_kg": 100.5, "water_usage_liters": 500.0
    pub environmental_impact_score: f64, // 0.0 (low impact) to 1.0 (high impact)
    pub assessed_period_start: DateTime<Utc>,
    pub assessed_period_end: DateTime<Utc>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the assessment process
    pub security_verified: VerificationStatus, // New: Security of footprint calculation
    pub generated_at: DateTime<Utc>,
}

/// Represents a single data point from environmental monitoring (e.g., sensor reading).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalDataPoint {
    pub id: String,
    pub sensor_id: String,
    pub data_type: String, // e.g., "air_quality_ozone", "water_pH", "temperature"
    pub value: f64,
    pub unit: String, // e.g., "ppm", "pH", "celsius"
    pub timestamp: DateTime<Utc>,
    pub integrity_verified: VerificationStatus, // New: Integrity of the data point
    pub source: String, // e.g., "Tariro_sensor_network", "External_NOAA_API"
}

/// A plan outlining actions to mitigate negative environmental impact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationPlan {
    pub id: String,
    pub related_footprint_id: String,
    pub name: String,
    pub proposed_actions: Vec<String>, // e.g., "optimize_energy_consumption", "switch_to_renewable_resources", "implement_resource_recycling"
    pub expected_impact_reduction: HashMap<String, f64>, // e.g., "carbon_emissions_reduction_pct": 20.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the plan
    pub security_review_status: VerificationStatus, // New: Security review of the plan
    pub approved_by: String, // e.g., "Tariro_EIM", "ALEC-A"
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the ethical assessment of environmental impacts and mitigations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalEthicalReport {
    pub id: String,
    pub assessment_id: String, // ID of EcologicalFootprint or MitigationPlan
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance
    pub identified_conflicts_with_ecology: Vec<String>,
    pub recommendations: Vec<String>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
