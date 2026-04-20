// Tariro_ide_idpte/src/environmental/global_knowledge_models.rs

//! Defines data models for Tariro's Global Knowledge Integration & Synthesis module,
//! including knowledge sources, integrated knowledge graphs, and synthesis reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Details about a source from which knowledge is acquired.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSource {
    pub id: String,
    pub name: String,
    pub source_type: String, // e.g., "scientific_database", "cultural_archive", "real_time_sensor_data"
    pub reliability_score: f64, // 0.0 to 1.0, assessed by TrustReputationModule
    pub bias_assessment: VerificationStatus, // New: Bias assessment of the source
    pub ethical_data_acquisition: VerificationStatus, // New: Ethical review of data acquisition method
    pub last_accessed: DateTime<Utc>,
}

/// A unified, coherent representation of knowledge integrated from diverse sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedKnowledgeGraph {
    pub id: String,
    pub version: String,
    pub conceptual_map_summary: String,
    pub size_nodes: u64,
    pub size_edges: u64,
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of the integrated knowledge
    pub data_integrity: VerificationStatus, // New: Data integrity verification of the graph
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the process and outcome of knowledge synthesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisReport {
    pub id: String,
    pub knowledge_graph_id: String, // Link to IntegratedKnowledgeGraph
    pub synthesized_concepts: Vec<String>,
    pub resolved_contradictions: Vec<String>,
    pub emergent_patterns: Vec<String>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the synthesis process
    pub security_verified: VerificationStatus, // New: Security verification of synthesis process
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of global knowledge integration and synthesis processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalKnowledgeAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_knowledge_ingestion", "critical_data_synthesis"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sources_integrated: u64,
    pub total_contradictions_resolved: u64,
    pub detected_misinformation_attempts: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of knowledge integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of knowledge systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
