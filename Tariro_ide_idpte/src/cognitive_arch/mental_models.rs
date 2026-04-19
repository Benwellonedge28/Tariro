// Tariro_ide_idpte/src/cognitive_arch/mental_models.rs

//! Defines data models for representing mental states (beliefs, intentions, desires)
//! and overall mental models of agents within Tariro's Theory of Mind module.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a belief held by an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub content: String, // e.g., "temperature is rising", "user wants X"
    pub certainty: f64, // 0.0 to 1.0
    pub source: String, // e.g., "observation", "communication", "inference"
    pub timestamp: DateTime<Utc>,
    pub privacy_level: Option<String>, // New: e.g., "public", "private", "sensitive"
}

/// Represents an intention of an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub goal: String, // e.g., "to get coffee", "to solve problem X"
    pub commitment_level: f64, // 0.0 to 1.0
    pub plan_summary: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub ethical_risk_score: Option<f64>, // New: Score from ALEC-A
}

/// Represents a desire or preference of an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desire {
    pub object: String, // e.g., "comfort", "efficiency", "a new feature"
    pub strength: f64, // 0.0 to 1.0 (how strongly desired)
    pub timestamp: DateTime<Utc>,
    pub cultural_context: Option<String>, // New: e.g., "western_individualistic", "eastern_collectivist"
}

/// Represents a cognitive bias detected in an agent's thinking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveBias {
    pub bias_type: String, // e.g., "confirmation_bias", "availability_heuristic"
    pub description: String,
    pub strength: f64, // How strongly this bias affects the agent
    pub detected_at: DateTime<Utc>,
}

/// Represents a cultural nuance influencing an agent's behavior or interpretation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalNuance {
    pub culture_id: String, // e.g., "japanese", "american", "corporate_tech"
    pub nuance_type: String, // e.g., "indirect_communication", "hierarchy_respect"
    pub description: String,
    pub relevance_score: f64, // How relevant this nuance is in current context
    pub detected_at: DateTime<Utc>,
}

/// Metadata related to the privacy of mental model data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyMetadata {
    pub encryption_status: String, // e.g., "encrypted", "partially_encrypted", "unencrypted"
    pub data_anonymized: bool,
    pub access_restrictions: Vec<String>, // e.g., "only_aleca", "only_human_admin"
}

/// Metadata related to the security of mental model data, protecting against manipulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetadata {
    pub integrity_verified: bool,
    pub manipulation_attempts_detected: u32,
    pub last_audit: DateTime<Utc>,
}

/// A comprehensive mental model of another agent (human or AGI).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMentalModel {
    pub agent_id: String,
    pub agent_type: String, // e.g., "human", "agi"
    pub beliefs: Vec<Belief>,
    pub intentions: Vec<Intention>,
    pub desires: Vec<Desire>,
    pub capabilities: Vec<String>, // Known capabilities of the agent
    pub last_updated: DateTime<Utc>,
    pub trust_level: f64, // Tariro's internal trust of this agent
    pub cognitive_biases: Vec<CognitiveBias>, // New: Detected cognitive biases
    pub cultural_nuances: Vec<CulturalNuance>, // New: Detected cultural nuances
    pub privacy_metadata: Option<PrivacyMetadata>, // New: Privacy settings for this model
    pub security_metadata: Option<SecurityMetadata>, // New: Security audit results for this model
}
