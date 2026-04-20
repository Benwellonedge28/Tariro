// Tariro_ide_idpte/src/omega_os/self_evolving_models.rs

//! Defines data models for Tariro's Self-Evolving Data Structures & Algorithmic Optimization module,
//! including data structure evolution plans, algorithm optimization reports, and runtime adaptation logs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A plan for autonomously adapting and optimizing a core OMEGA OS data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataStructureEvolutionPlan {
    pub id: String,
    pub data_structure_name: String, // e.g., "global_knowledge_graph", "task_queue"
    pub current_schema_hash: String,
    pub proposed_schema_hash: String,
    pub expected_performance_gain: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // Ethical review of the data structure change impact
    pub security_verified: VerificationStatus, // Security assessment of schema migration
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the autonomous optimization of an algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmOptimizationReport {
    pub id: String,
    pub algorithm_name: String, // e.g., "path_planning_heuristic", "threat_detection_classifier"
    pub original_performance_metric: f64,
    pub optimized_performance_metric: f64,
    pub optimization_method: String, // e.g., "meta_learning_refinement", "jit_compilation"
    pub ethical_compliance: VerificationStatus, // Ethical compliance of the optimized algorithm
    pub security_audited: VerificationStatus, // Security audit of the optimized algorithm
    pub generated_at: DateTime<Utc>,
}

/// A log entry for dynamic, real-time adaptations made to OMEGA OS data structures or algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAdaptationLog {
    pub id: String,
    pub adapted_component_id: String, // ID of the data structure or algorithm that was adapted
    pub adaptation_type: String, // e.g., "schema_refactoring", "algorithm_swap", "parameter_tuning"
    pub reason_for_adaptation: String,
    pub observed_impact: f64, // Performance or efficiency change
    pub ethical_impact_assessment: VerificationStatus, // Ethical implications of the runtime change
    pub security_verified: VerificationStatus, // Security verification of the runtime change
    pub timestamp: DateTime<Utc>,
}

/// Audit report for self-evolving data structures and algorithmic optimization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_adaptations: u64,
    pub successful_optimizations: u64,
    pub detected_degradations: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
