// Tariro_ide_idpte/src/omega_os/self_evolution_models.rs

//! Defines data models for Tariro's Self-Evolving Data Structures & Algorithmic Optimization module,
//! including data structure evolution plans, algorithm optimization reports, and runtime adaptation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A plan for autonomously adapting a core OMEGA OS data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataStructureEvolutionPlan {
    pub id: String,
    pub data_structure_name: String,
    pub optimization_goal: String, // e.g., "reduce_memory_footprint", "accelerate_query_speed"
    pub proposed_structure_type: String, // e.g., "B-tree", "Hash_map", "Vector"
    pub expected_performance_gain: f64, // 0.0 to 1.0
    pub ethical_integrity_review: VerificationStatus, // Ethical review of data integrity during evolution
    pub security_verified: VerificationStatus, // Security assessment of the evolution process
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the optimization of a core OMEGA OS algorithm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmOptimizationReport {
    pub id: String,
    pub algorithm_name: String,
    pub optimization_method: String, // e.g., "parallelization", "memoization", "algorithmic_substitution"
    pub measured_performance_gain: f64, // Factor of improvement
    pub resource_utilization_impact: HashMap<String, f64>, // e.g., "cpu_reduction": 0.2
    pub ethical_compliance: VerificationStatus, // Ethical compliance of the optimized algorithm
    pub generated_at: DateTime<Utc>,
}

/// A strategy for optimizing runtime behavior of Tariro's core systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAdaptationStrategy {
    pub id: String,
    pub target_component: String,
    pub adaptation_type: String, // e.g., "dynamic_threading", "load_balancing", "priority_scheduling"
    pub adaptation_parameters: HashMap<String, String>,
    pub expected_stability_impact: f64, // 0.0 to 1.0
    pub ethical_governance_review: VerificationStatus, // Ethical review of the runtime adaptation
    pub security_audited: VerificationStatus, // Security audit of the adaptation execution
    pub created_at: DateTime<Utc>,
}

/// Audit report for self-evolving data structures and algorithmic optimization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_ds_evolutions: u64,
    pub total_algo_optimizations: u64,
    pub detected_regressions: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
