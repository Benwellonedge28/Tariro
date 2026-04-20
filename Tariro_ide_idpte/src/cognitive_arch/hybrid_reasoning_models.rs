// Tariro_ide_idpte/src/cognitive_arch/hybrid_reasoning_models.rs

//! Defines data models for Tariro's Symbolic & Neural Hybrid Reasoning module,
//! including symbolic expressions, neural representations, and hybrid reasoning tasks.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// An abstract representation of a logical rule or symbolic expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolicExpression {
    pub id: String,
    pub expression_content: String, // e.g., "IF A AND B THEN C"
    pub logical_framework: String, // e.g., "First_Order_Logic", "Description_Logic"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the symbolic rule
    pub security_verified: VerificationStatus, // New: Security verification of the logical consistency
    pub created_at: DateTime<Utc>,
}

/// A high-dimensional representation of a concept or pattern within a neural network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralRepresentation {
    pub id: String,
    pub modality_source: String, // e.g., "text", "image"
    pub feature_vector_summary: String,
    pub mapping_to_symbol_id: Option<String>, // Optional link to SymbolicExpression
    pub ethical_bias_score: f64, // New: Score indicating potential bias in the representation
    pub last_updated: DateTime<Utc>,
}

/// A task requiring the combination of symbolic logic and neural patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridReasoningTask {
    pub id: String,
    pub task_description: String,
    pub symbolic_inputs: Vec<String>, // List of SymbolicExpression IDs
    pub neural_inputs: Vec<String>, // List of NeuralRepresentation IDs
    pub inferred_outcome: String,
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of the hybrid reasoning
    pub reasoning_confidence: f64, // 0.0 to 1.0
    pub processed_at: DateTime<Utc>,
}

/// A report summarizing the audit of symbolic and neural hybrid reasoning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_hybrid_tasks", "critical_decision_logic"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_tasks_processed: u64,
    pub successful_reasoning_events: u64,
    pub detected_logical_inconsistencies: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of hybrid reasoning
    pub security_integrity_status: VerificationStatus, // Overall security integrity of hybrid systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
