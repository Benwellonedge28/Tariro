// Tariro_ide_idpte/src/cognitive_arch/causal_models.rs

//! Defines data models for causal graphs, variables, and interventions within Tariro's Causal Reasoning Engine.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{Utc, DateTime};

/// Represents a single variable in a causal graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalVariable {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub data_type: String, // e.g., "numeric", "categorical", "boolean"
    pub possible_values: Option<Vec<String>>,
    pub is_latent: bool, // New: Indicates if the variable is unobserved or inferred
}

/// Represents an edge in a causal graph, indicating a direct causal influence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    pub cause_id: String,
    pub effect_id: String,
    pub strength: f64, // e.g., 0.0 to 1.0
    pub type_name: String, // e.g., "direct_influence", "enabling_condition"
    pub conditions: Option<String>, // e.g., "if temperature > 30C"
    pub causal_mechanism_summary: Option<String>, // New: A brief explanation of the mechanism
}

/// Represents a causal graph, defining relationships between variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalGraph {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub variables: HashMap<String, CausalVariable>, // ID -> CausalVariable
    pub edges: Vec<CausalEdge>,
    pub discovered_by: String, // e.g., "Tariro_CRE", "HumanExpert"
    pub last_updated: DateTime<Utc>,
    pub hierarchy_level: u32, // New: Indicates the abstraction level of this graph
}

/// Represents an intervention on a causal variable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub variable_id: String,
    pub target_value: String, // e.g., "set_to_true", "increase_by_10%", "reduce_to_zero"
    pub description: Option<String>,
    pub predicted_impact_summary: Option<String>, // New: Summary of expected changes
}

/// Represents a design for an active causal discovery experiment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalExperimentDesign {
    pub id: String,
    pub name: String,
    pub hypothesis: String, // The causal hypothesis to be tested
    pub interventions_proposed: Vec<Intervention>,
    pub observations_to_collect: Vec<String>, // Variables to monitor
    pub expected_outcome_summary: String,
    pub ethical_review_status: VerificationStatus, // Status from ALEC-A review
    pub safety_protocol_summary: String,
    pub created_at: DateTime<Utc>,
}

/// Represents a prediction of a security vulnerability using causal models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedVulnerability {
    pub id: String,
    pub causal_path_summary: String, // Description of the causal chain leading to vulnerability
    pub severity: f64, // 0.0 to 1.0
    pub likelihood: f64, // 0.0 to 1.0
    pub recommended_intervention: Intervention,
    pub predicted_impact: String, // What could happen if exploited
    pub discovered_at: DateTime<Utc>,
}
