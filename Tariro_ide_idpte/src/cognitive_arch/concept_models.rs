// Tariro_ide_idpte/src/cognitive_arch/concept_models.rs

//! Defines data models for raw data streams, discovered concepts, and conceptual hierarchies
//! within Tariro's Unsupervised Concept Formation module.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For concept verification

/// Represents a stream of raw, unlabeled data that Tariro observes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawDataStream {
    pub id: String,
    pub name: String,
    pub data_type: String, // e.g., "sensor_readings", "text_snippets", "image_pixels"
    pub source_module: String, // e.g., "NetworkManager", "MMIF", "StorageAbstraction"
    pub description: Option<String>,
    pub sample_data_summary: String, // A short summary or example of the data
    pub timestamp: DateTime<Utc>,
    pub data_privacy_level: Option<String>, // New: e.g., "public", "confidential"
    pub data_integrity_hash: Option<String>, // New: Hash for integrity check
}

/// Represents a concept autonomously discovered by Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredConcept {
    pub id: String,
    pub name: String, // Automatically generated name or descriptor
    pub description: String,
    pub features: HashMap<String, String>, // Key characteristics or attributes of the concept
    pub examples_summary: Vec<String>, // Summaries of data points belonging to this concept
    pub parent_concepts: Vec<String>, // IDs of more general concepts
    pub child_concepts: Vec<String>, // IDs of more specific concepts
    pub confidence: f64, // Confidence in the validity and coherence of the concept
    pub discovered_from_stream_id: String,
    pub discovered_at: DateTime<Utc>,
    pub last_refined: DateTime<Utc>,
    pub verification_status: ConceptVerificationStatus, // New: Verification status
}

/// Represents a hierarchy of concepts, showing their relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualHierarchy {
    pub root_concept_id: String,
    pub levels: HashMap<u32, Vec<String>>, // Level number -> list of concept IDs at that level
    pub relationships: Vec<ConceptualRelationship>, // New: Richer relationship types
    pub last_updated: DateTime<Utc>,
    pub integrity_verified: bool, // New: Overall integrity of the hierarchy
}

/// Comprehensive verification status for a discovered concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptVerificationStatus {
    pub ethical_bias_status: VerificationStatus, // e.g., for fairness in categorization
    pub logical_consistency_status: VerificationStatus, // Consistent with other knowledge
    pub security_integrity_status: VerificationStatus, // Integrity of the concept's definition
    pub overall_status: VerificationStatus,
    pub last_verified: DateTime<Utc>,
    pub verification_log_id: String,
}

/// Represents a richer, verified relationship between two concepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualRelationship {
    pub from_concept_id: String,
    pub to_concept_id: String,
    pub relationship_type: String, // e.g., "is_a", "part_of", "cause_of", "analogous_to"
    pub strength: f64, // Confidence in the relationship
    pub verified_by: VerificationStatus, // How this relationship was verified
    pub timestamp: DateTime<Utc>,
}
