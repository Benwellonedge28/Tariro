// Tariro_ide_idpte/src/environmental/global_knowledge_models.rs

//! Defines data models for Tariro's Global Knowledge Integration & Synthesis module,
//! including integrated knowledge graphs, knowledge alignment metrics, and source trust.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A comprehensive, synthesized knowledge representation, potentially a semantic graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedKnowledgeGraph {
    pub id: String,
    pub graph_name: String,
    pub summary: String,
    pub num_nodes: u64,
    pub num_edges: u64,
    pub ethical_data_provenance: VerificationStatus, // New: Ethical review of knowledge source
    pub data_integrity_verified: VerificationStatus, // New: Data integrity verification for graph
    pub last_updated: DateTime<Utc>,
}

/// A metric representing how well new knowledge aligns with existing knowledge and beliefs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeAlignmentMetric {
    pub id: String,
    pub knowledge_graph_id: String,
    pub new_data_id: String,
    pub alignment_score: f64, // 0.0 (contradictory) to 1.0 (perfectly aligned)
    pub discrepancies_identified: Vec<String>,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical assessment of knowledge conflicts
    pub security_risk_level: VerificationStatus, // New: Security risk of knowledge injection
    pub evaluated_at: DateTime<Utc>,
}

/// A rating of the trustworthiness of a specific knowledge source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSourceTrust {
    pub id: String,
    pub source_identifier: String, // e.g., URL, publisher name, AGI ID
    pub trust_score: f64, // 0.0 (unreliable) to 1.0 (highly trustworthy)
    pub factors_influencing_trust: HashMap<String, String>, // e.g., "peer_review": "high", "bias_detection": "low"
    pub ethical_source_assessment: VerificationStatus, // New: Ethical assessment of source motives
    pub security_of_source_data: VerificationStatus, // New: Security assessment of source data transmission
    pub last_evaluated: DateTime<Utc>,
}

/// A report summarizing the audit of global knowledge integration processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeIntegrationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_knowledge_ingestion", "critical_scientific_updates"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sources_processed: u64,
    pub total_contradictions_resolved: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of knowledge integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of knowledge integration systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
