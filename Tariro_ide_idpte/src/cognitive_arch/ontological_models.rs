// Tariro_ide_idpte/src/cognitive_arch/ontological_models.rs

//! Defines data models for Tariro's Dynamic Ontological Evolution module,
//! including ontology nodes, change proposals, and knowledge schemas.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A node within Tariro's internal representation of concepts (the ontology).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyNode {
    pub id: String,
    pub concept_label: String,
    pub description: String,
    pub parent_concepts: Vec<String>, // List of OntologyNode IDs
    pub associated_attributes: Vec<String>,
    pub ethical_context: Vec<String>, // New: Ethical aspects tied to this concept
    pub data_provenance: String, // New: Source of the knowledge
    pub last_updated: DateTime<Utc>,
}

/// A proposal for a change in the internal ontology (e.g., adding, merging, or deleting concepts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologicalChangeProposal {
    pub id: String,
    pub change_type: String, // e.g., "add_concept", "merge_concepts", "refine_relationship"
    pub description: String,
    pub affected_nodes: Vec<String>,
    pub estimated_semantic_gain: f64, // New: Expected improvement in knowledge coherence
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the ontological change
    pub security_verified: VerificationStatus, // New: Security assessment for structural integrity
    pub created_at: DateTime<Utc>,
}

/// A structured schema defining how knowledge is organized within a specific domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSchema {
    pub id: String,
    pub domain_name: String,
    pub schema_definition: String, // e.g., JSON Schema or formal ontology spec
    pub version: String,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the domain schema
    pub security_audited: VerificationStatus, // New: Security audit of the schema logic
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of ontological evolution processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_ontology_changes", "critical_knowledge_schema_updates"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_changes_implemented: u64,
    pub detected_semantic_conflicts: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of ontology evolution
    pub security_integrity_status: VerificationStatus, // Overall security integrity of knowledge systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
