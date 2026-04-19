// Tariro_ide_idpte/src/cognitive_arch/causal_models.rs

//! Defines data models for causal graphs, variables, and interventions within Tariro's Causal Reasoning Engine.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Represents a single variable in a causal graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalVariable {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub data_type: String, // e.g., "numeric", "categorical", "boolean"
    pub possible_values: Option<Vec<String>>,
}

/// Represents an edge in a causal graph, indicating a direct causal influence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    pub cause_id: String,
    pub effect_id: String,
    pub strength: f64, // e.g., 0.0 to 1.0
    pub type_name: String, // e.g., "direct_influence", "enabling_condition"
    pub conditions: Option<String>, // e.g., "if temperature > 30C"
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
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Represents an intervention on a causal variable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub variable_id: String,
    pub target_value: String, // e.g., "set_to_true", "increase_by_10%", "reduce_to_zero"
    pub description: Option<String>,
}
