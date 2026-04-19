// Tariro_ide_idpte/src/cognitive_arch/learning_models.rs

//! Defines data models for learning problems, algorithms, and their performance metrics.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a specific learning problem that Tariro encounters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningProblem {
    pub id: String,
    pub name: String,
    pub domain: String, // e.g., "natural_language_processing", "robot_control", "resource_optimization"
    pub problem_type: String, // e.g., "classification", "regression", "reinforcement_learning"
    pub complexity_score: f64,
    pub data_characteristics: HashMap<String, String>, // e.g., "data_size": "large", "data_sparsity": "high"
    pub required_accuracy: f64,
    pub created_at: DateTime<Utc>,
}

/// Metadata about a specific learning algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningAlgorithmMeta {
    pub id: String,
    pub name: String,
    pub algorithm_type: String, // e.g., "neural_network", "decision_tree", "evolutionary_algorithm"
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub resource_footprint: HashMap<String, f64>, // e.g., "cpu": 0.5, "gpu": 0.8, "memory": 4.0
    pub applicable_domains: Vec<String>,
    pub version: String,
}

/// Records the performance of a learning algorithm on a specific problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmPerformance {
    pub id: String,
    pub algorithm_id: String,
    pub problem_id: String,
    pub timestamp: DateTime<Utc>,
    pub accuracy: f64,
    pub convergence_time_seconds: f64,
    pub resource_cost: HashMap<String, f64>, // Actual resources consumed
    pub achieved_metrics: HashMap<String, f64>, // Specific metrics like F1-score, reward
    pub notes: Option<String>,
}
