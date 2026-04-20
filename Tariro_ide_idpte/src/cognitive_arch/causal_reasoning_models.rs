// Tariro_ide_idpte/src/cognitive_arch/causal_reasoning_models.rs

//! Defines data models for Tariro's Advanced Causal Discovery & Inference module,
//! including causal models, causal graphs, and causal inference reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A representation of a discovered causal structure, detailing relationships between variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub variables: Vec<String>, // List of variables in the model
    pub discovered_relationships: Vec<String>, // e.g., "A causes B", "C mediates D and E"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the model for bias/fairness
    pub data_integrity_verified: VerificationStatus, // New: Verification of underlying data integrity
    pub created_at: DateTime<Utc>,
}

/// A graphical representation of causal links and their directions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalGraph {
    pub id: String,
    pub model_id: String, // Link to CausalModel
    pub graph_format: String, // e.g., "DAG", "Bayesian_Network"
    pub nodes: Vec<String>, // Variables as nodes
    pub edges: Vec<String>, // Directed causal links as edges
    pub ethical_visualization_review: VerificationStatus, // New: Ethical review of how the graph is presented
    pub security_audited: VerificationStatus, // New: Security audit of the graph generation process
    pub generated_at: DateTime<Utc>,
}

/// A report detailing a specific causal conclusion or counterfactual analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalInferenceReport {
    pub id: String,
    pub model_id: String, // Link to CausalModel
    pub inference_query: String, // e.g., "What if X had not happened?"
    pub inferred_outcome: String,
    pub causal_strength_score: f64, // 0.0 to 1.0
    pub ethical_implications: Vec<String>, // New: Ethical implications of the inference
    pub transparency_level: VerificationStatus, // New: Transparency of the inference explanation
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of causal discovery and inference processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_causal_models", "critical_decision_support"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_models_discovered: u64,
    pub inferences_made: u64,
    pub detected_biases: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of causal processes
    pub security_integrity_status: VerificationStatus, // Overall security integrity of causal systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
