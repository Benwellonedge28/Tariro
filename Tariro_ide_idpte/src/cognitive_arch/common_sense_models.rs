// Tariro_ide_idpte/src/cognitive_arch/common_sense_models.rs

//! Defines data models for representing common-sense knowledge within Tariro's
//! Common Sense Reasoning (CSR) Network.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a single common-sense fact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonSenseFact {
    pub id: String,
    pub statement: String, // e.g., "Water is wet", "Birds can fly"
    pub domain: String, // e.g., "physics", "biology", "social_norms"
    pub certainty: f64, // 0.0 to 1.0
    pub source: String, // e.g., "learned_from_experience", "communicated", "inferred"
    pub timestamp: DateTime<Utc>,
}

/// Represents a common-sense rule or heuristic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonSenseRule {
    pub id: String,
    pub rule_text: String, // e.g., "If something is a bird, then it can probably fly."
    pub antecedents: Vec<String>, // Conditions for the rule
    pub consequents: Vec<String>, // Outcomes of the rule
    pub domain: String,
    pub applicability_context: Option<String>,
    pub certainty: f64,
    pub timestamp: DateTime<Utc>,
}

/// Represents an affordance (possible action) of an object or environment in a given context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualAffordance {
    pub object_id: String, // ID of the object
    pub object_name: String, // e.g., "chair", "door", "river"
    pub affordance: String, // e.g., "can be sat on", "can be opened", "can be swum in"
    pub context: String, // e.g., "when in a room", "when not locked", "when safe to swim"
    pub certainty: f64,
    pub timestamp: DateTime<Utc>,
}
