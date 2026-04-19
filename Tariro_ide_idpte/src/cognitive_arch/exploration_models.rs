// Tariro_ide_idpte/src/cognitive_arch/exploration_models.rs

//! Defines data models for novelty scoring, intrinsic rewards, and exploration targets
//! within Tariro's Curiosity-Driven Exploration Module.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a score indicating the novelty of a particular state or observation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyScore {
    pub id: String,
    pub observation_context_summary: String, // A summary of the observation that generated the score
    pub score: f64, // 0.0 (very familiar) to 1.0 (highly novel)
    pub method: String, // e.g., "prediction_error", "state_visitation_count", "feature_entropy"
    pub timestamp: DateTime<Utc>,
}

/// Represents an intrinsic reward generated internally by Tariro's curiosity mechanism.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrinsicReward {
    pub id: String,
    pub associated_novelty_score_id: String,
    pub reward_value: f64, // A positive value that reinforces exploration
    pub explanation: String, // e.g., "High prediction error for new state."
    pub timestamp: DateTime<Utc>,
}

/// Represents a target or strategy for exploration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationTarget {
    pub id: String,
    pub description: String, // e.g., "Explore unknown region X", "Interact with novel object Y"
    pub target_state_features: HashMap<String, String>, // Desired features of the state to reach
    pub predicted_novelty_gain: f64, // Expected gain in novelty or knowledge
    pub strategy: String, // e.g., "random_walk", "goal_conditioned_policy", "perturbation"
    pub priority: f64, // How important this exploration is
    pub created_at: DateTime<Utc>,
}
