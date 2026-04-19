// Tariro_ide_idpte/src/cognitive_arch/heuristic_models.rs

//! Defines data models for heuristics, intuitive leaps, and problem context summaries
//! within Tariro's Intuition & Heuristic Generation module.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a learned problem-solving heuristic (rule of thumb or shortcut).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heuristic {
    pub id: String,
    pub name: String,
    pub description: String,
    pub applicability_conditions: HashMap<String, String>, // e.g., "problem_type": "optimization", "time_constraint": "high"
    pub suggested_action: String, // e.g., "use greedy approach", "prioritize safety"
    pub expected_accuracy: f64, // 0.0 to 1.0 (how often it's correct)
    pub discovery_source: String, // e.g., "MetaLearner", "HumanObservation", "SelfExperimentation"
    pub timestamp: DateTime<Utc>,
}

/// Represents an intuitive leap or rapid inference made by Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitiveLeap {
    pub id: String,
    pub problem_context_summary: String, // Summary of the problem state when the leap occurred
    pub inferred_solution: String, // The solution or insight arrived at intuitively
    pub confidence: f64, // Confidence in the intuitive leap (0.0 to 1.0)
    pub supporting_patterns: Vec<String>, // Key patterns or heuristics that contributed to the leap
    pub timestamp: DateTime<Utc>,
}

/// Summarizes the current context of a problem for heuristic application or intuitive leaps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemContextSummary {
    pub id: String,
    pub problem_id: String,
    pub description: String,
    pub key_features: HashMap<String, String>, // Important features of the problem state
    pub urgency_score: f64, // 0.0 to 1.0 (how quickly a decision is needed)
    pub available_resources_summary: HashMap<String, f64>, // Current resource availability
    pub timestamp: DateTime<Utc>,
}
