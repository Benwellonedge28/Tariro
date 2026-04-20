// Tariro_ide_idpte/src/cognitive_arch/meta_learning_models.rs

//! Defines data models for Tariro's Lifelong Meta-Learning module,
//! including learning trajectories, algorithm benchmarks, and meta-knowledge base entries.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A record of a learning process over time, capturing progress and efficiency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningTrajectory {
    pub id: String,
    pub task_id: String,
    pub algorithm_id: String,
    pub initial_performance: f64,
    pub final_performance: f64,
    pub learning_duration_ms: u64,
    pub ethical_alignment_history: Vec<String>,
    pub recorded_at: DateTime<Utc>,
}

/// Benchmarking data for a specific learning algorithm across various tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmBenchmark {
    pub id: String,
    pub algorithm_name: String,
    pub task_domains: Vec<String>,
    pub average_accuracy: f64,
    pub average_speed_ms: f64,
    pub ethical_robustness_score: f64, // 0.0 to 1.0
    pub security_vulnerability_rating: VerificationStatus,
    pub last_updated: DateTime<Utc>,
}

/// An entry in the meta-knowledge base, representing learned "knowledge about learning".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaKnowledgeEntry {
    pub id: String,
    pub learned_principle: String, // e.g., "Algorithm X is better than Y for data of type Z"
    pub empirical_basis_ids: Vec<String>, // List of LearningTrajectory or AlgorithmBenchmark IDs
    pub confidence_level: f64,
    pub ethical_implications: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Audit report for lifelong meta-learning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearningAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_algorithms_benchmarked: u64,
    pub total_meta_insights_generated: u64,
    pub detected_learning_regressions: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
