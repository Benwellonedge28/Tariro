// Tariro_ide_idpte/src/cognitive_arch/creative_problem_models.rs

//! Defines data models for Tariro's Creative Problem Solving module,
//! including problem definitions, novel solutions, and divergent thinking processes.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A definition of a complex, ill-defined problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub constraints: Vec<String>,
    pub initial_assumptions: Vec<String>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the problem definition
    pub security_implications_assessed: VerificationStatus, // New: Security assessment of problem context
    pub defined_at: DateTime<Utc>,
}

/// A generated novel solution to a problem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelSolution {
    pub id: String,
    pub problem_id: String, // Link to ProblemDefinition
    pub solution_summary: String,
    pub novelty_score: f64, // 0.0 to 1.0
    pub feasibility_score: f64, // 0.0 to 1.0
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact assessment of the solution
    pub security_vulnerability_analysis: VerificationStatus, // New: Security vulnerability analysis of the solution
    pub generated_at: DateTime<Utc>,
}

/// A log of the divergent thinking process used to generate solutions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivergentThinkingProcess {
    pub id: String,
    pub problem_id: String,
    pub process_steps: Vec<String>, // e.g., "brainstorming_phase", "analogy_generation"
    pub creativity_metrics: HashMap<String, f64>, // e.g., "fluency": 0.7, "originality": 0.8
    pub ethical_considerations_log: VerificationStatus, // New: Ethical log of the thinking process
    pub security_log: VerificationStatus, // New: Security log of the thinking process
    pub performed_at: DateTime<Utc>,
}

/// A report summarizing the audit of creative problem-solving processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeProblemAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_creative_problems", "critical_innovation_challenges"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_problems_addressed: u64,
    pub total_novel_solutions: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of creative problem-solving
    pub security_integrity_status: VerificationStatus, // Overall security integrity of creative problem-solving systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
