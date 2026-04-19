// Tariro_ide_idpte/src/cognitive_arch/formal_reasoning_models.rs

//! Defines data models for Tariro's Abstract & Formal Reasoning module,
//! including logical statements, formal proofs, and consistency reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a logical statement in a formal language (e.g., first-order logic).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalStatement {
    pub id: String,
    pub statement_text: String, // e.g., "FORALL x. (P(x) IMPLIES Q(x))"
    pub formal_language: String, // e.g., "FirstOrderLogic", "TemporalLogic"
    pub asserted_by: String, // e.g., "Tariro_FRM", "HumanExpert", "ALEC-A Axiom"
    pub certainty: f64, // 0.0 to 1.0 (confidence in the truth of the statement)
    pub timestamp: DateTime<Utc>,
    pub security_hash: Option<String>, // New: Hash to ensure statement integrity
}

/// Represents a formal proof constructed by Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormalProof {
    pub id: String,
    pub theorem_id: String, // ID of the LogicalStatement being proven
    pub proof_steps: Vec<String>, // Sequence of logical inference steps
    pub proof_method: String, // e.g., "Resolution", "NaturalDeduction", "AutomatedTheoremProver"
    pub verified_by_module: String, // e.g., "Tariro_FRM", "ExternalProver"
    pub proof_integrity_status: VerificationStatus, // New: Verification status for the proof itself
    pub generated_at: DateTime<Utc>,
    pub computation_cost: HashMap<String, f64>, // e.g., "cpu_time": 12.5, "memory_peak": 1024.0
}

/// Result of a formal verification task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub id: String,
    pub target_id: String, // ID of the entity being verified (e.g., statement, plan, knowledge base)
    pub target_type: String, // e.g., "LogicalStatement", "InternalState", "CausalGraph"
    pub result_status: VerificationStatus, // Passed, Failed, FlaggedForReview
    pub ethical_implications_summary: Option<String>, // New: From ALEC-A review of implications
    pub security_implications_summary: Option<String>, // New: From QR-CTFO review of implications
    pub checked_at: DateTime<Utc>,
    pub verification_log_id: String,
}

/// A report summarizing the logical consistency of a knowledge base or internal system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyReport {
    pub id: String,
    pub scope: String, // e.g., "GlobalKnowledgeBase", "ActivePlanSet"
    pub consistency_score: f64, // 0.0 to 1.0 (higher is more consistent)
    pub identified_contradictions: Vec<String>, // Descriptions of detected logical contradictions
    pub proposed_resolutions: Vec<String>, // Suggested actions to resolve inconsistencies
    pub ethical_review_status: VerificationStatus, // New: Ethical review status of identified inconsistencies/resolutions
    pub generated_at: DateTime<Utc>,
}
