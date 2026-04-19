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
    // New fields for evolution and verification
    pub genome_id: Option<String>, // Link to evolutionary genome if applicable
    pub verification_status: Option<AlgorithmVerificationStatus>,
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

/// Represents the genetic blueprint of an evolving algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmGenome {
    pub id: String,
    pub algorithm_type: String, // Corresponds to LearningAlgorithmMeta.algorithm_type
    pub genes: HashMap<String, String>, // e.g., "layers": "3", "activation": "relu", "crossover_rate": "0.7"
    pub generation: u32,
    pub parent_genomes: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Status of ethical and security verification for an algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending, 
    Passed(String), // Passed with summary/cert ID
    Failed(String), // Failed with reason
    FlaggedForReview(String), // Requires human intervention
}

/// Comprehensive verification status for a learning algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmVerificationStatus {
    pub ethical_status: VerificationStatus,
    pub security_status: VerificationStatus,
    pub compliance_status: VerificationStatus, // e.g., legal, regulatory
    pub overall_status: VerificationStatus,
    pub last_verified: DateTime<Utc>,
    pub verification_log_id: String,
}

/// New performance metric definition for autonomous metric generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicMetric {
    pub id: String,
    pub name: String,
    pub description: String,
    pub formula_or_logic: String, // e.g., "f1_score * (1 - resource_cost_penalty)"
    pub context_parameters: HashMap<String, String>, // Parameters affecting its calculation (e.g., "max_latency": "10ms")
    pub goal_id: String, // Metric is tied to a specific goal
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
