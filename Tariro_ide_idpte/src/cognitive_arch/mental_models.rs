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
}

/// Represents an intention of an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub goal: String, // e.g., "to get coffee", "to solve problem X"
    pub commitment_level: f64, // 0.0 to 1.0
    pub plan_summary: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Represents a desire or preference of an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desire {
    pub object: String, // e.g., "comfort", "efficiency", "a new feature"
    pub strength: f64, // 0.0 to 1.0 (how strongly desired)
    pub timestamp: DateTime<Utc>,
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
}
