// Tariro_ide_idpte/src/environmental/global_knowledge_models.rs

//! Defines data models for Tariro's Global Knowledge Integration & Cross-Cultural Understanding module,
//! including knowledge sources, cultural context models, and synthesis reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a source of global knowledge, with metadata on its reliability and cultural origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalKnowledgeSource {
    pub id: String,
    pub name: String,
    pub data_type: String, // e.g., "news_feed", "academic_paper", "social_media", "cultural_archive"
    pub language_locale: String, // e.g., "en-US", "zh-CN", "ar-EG"
    pub estimated_reliability: f64, // 0.0 to 1.0
    pub cultural_origin_summary: String,
    pub ethical_provenance_review: VerificationStatus, // New: Ethical review of knowledge source's origin
    pub security_verified: VerificationStatus, // New: Security verification of source integrity
    pub last_crawled: DateTime<Utc>,
}

/// A model representing the norms, values, and communication styles of a specific culture or group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContextModel {
    pub id: String,
    pub culture_name: String,
    pub key_values: Vec<String>,
    pub communication_styles: HashMap<String, String>, // e.g., "directness": "low", "hierarchy_respect": "high"
    pub observed_behaviors: Vec<String>,
    pub ethical_bias_assessment: VerificationStatus, // New: Assessment for inherent biases in the model
    pub security_integrity: VerificationStatus, // New: Security integrity of model data
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing synthesized knowledge from multiple global sources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSynthesisReport {
    pub id: String,
    pub query_context: String,
    pub synthesized_topic: String,
    pub summarized_content: String,
    pub contributing_sources: Vec<String>, // IDs of GlobalKnowledgeSources
    pub detected_biases: Vec<String>,
    pub ethical_representation_status: VerificationStatus, // New: Ethical review of knowledge synthesis
    pub security_integrity_status: VerificationStatus, // New: Security integrity of synthesis process
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing an audit of cross-cultural interactions and understanding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCulturalAudit {
    pub id: String,
    pub interaction_id: String,
    pub cultural_context_id: String,
    pub interaction_summary: String,
    pub outcome_assessment: String,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of cross-cultural interaction
    pub security_integrity_status: VerificationStatus, // Overall security integrity of interaction data
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
