// Tariro_ide_idpte/src/cognitive_arch/long_term_memory_models.rs

//! Defines data models for Tariro's Long-Term Semantic Memory & Knowledge Graph module,
//! including semantic facts, knowledge graph nodes and edges, and semantic memory audit reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a structured piece of knowledge or a fact within the semantic memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFact {
    pub id: String,
    pub predicate: String, // e.g., "isA", "hasProperty", "causes"
    pub subject_node_id: String,
    pub object_node_id: String,
    pub confidence: f64, // 0.0 to 1.0
    pub source_id: String, // ID of the KnowledgeSource or module that asserted this fact
    pub context_summary: String, // Brief summary of the context in which the fact was learned
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the fact's potential biases
    pub security_integrity: VerificationStatus, // New: Security verification of fact authenticity
    pub asserted_at: DateTime<Utc>,
}

/// Represents an entity or concept in the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphNode {
    pub id: String,
    pub label: String,
    pub node_type: String, // e.g., "Concept", "Entity", "Event", "Property"
    pub description: String,
    pub associated_data_ids: Vec<String>, // Links to ManagedFiles or KnowledgeItems
    pub ethical_representation_status: VerificationStatus, // New: Ethical review of how this node is represented
    pub security_access_control: VerificationStatus, // New: Security review of access controls for this node
    pub last_updated: DateTime<Utc>,
}

/// Represents a relationship between two nodes in the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphEdge {
    pub id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: String, // e.g., "is_a", "has_part", "causes", "related_to"
    pub confidence: f64,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the relationship's implications
    pub security_integrity: VerificationStatus, // New: Security integrity of the relationship
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of long-term semantic memory and knowledge graph integrity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemoryAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_semantic_facts", "specific_knowledge_domain"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_facts_audited: u64,
    pub detected_inconsistencies: Vec<String>,
    pub detected_biases: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of semantic memory
    pub security_integrity_status: VerificationStatus, // Overall security integrity of knowledge graph
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
