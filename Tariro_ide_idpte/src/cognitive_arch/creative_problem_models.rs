// Tariro_ide_idpte/src/cognitive_arch/creative_problem_models.rs

//! Defines data models for Tariro's Creative Problem Solving module,
//! including problem definitions, novel solutions, and divergent thinking processes.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A detailed description of a complex, ill-defined problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub constraints: Vec<String>,
    pub objectives: Vec<String>,
    pub ethical_considerations: Vec<String>, // New: Ethical aspects inherent in the problem
    pub security_implications: Vec<String>, // New: Security risks related to the problem context
    pub defined_at: DateTime<Utc>,
}

/// A generated novel and effective solution to a problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelSolution {
    pub id: String,
    pub problem_id: String, // Link to ProblemDefinition
    pub solution_summary: String,
    pub innovation_score: f64, // 0.0 to 1.0, how novel/creative the solution is
    pub feasibility_score: f64, // 0.0 to 1.0, how practical the solution is
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of the proposed solution
    pub security_robustness: VerificationStatus, // New: Security robustness assessment of the solution
    pub generated_at: DateTime<Utc>,
}

/// A log of the divergent thinking process, capturing ideas, analogies, and conceptual leaps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivergentThinkingProcess {
    pub id: String,
    pub problem_id: String,
    pub creative_techniques_used: Vec<String>, // e.g., "analogy", "brainstorming", "morphological_analysis"
    pub intermediate_ideas: Vec<String>,
    pub ethical_process_review: VerificationStatus, // New: Ethical review of the creative process
    pub security_logging_integrity: VerificationStatus, // New: Security verification of process logs
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of the Creative Problem Solving module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeProblemAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_creative_tasks", "critical_innovation_projects"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_problems_solved: u64,
    pub solutions_deployed: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of creative problem-solving
    pub security_integrity_status: VerificationStatus, // Overall security integrity of creative systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
