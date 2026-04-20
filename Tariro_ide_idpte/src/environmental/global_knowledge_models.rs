// Tariro_ide_idpte/src/environmental/global_knowledge_models.rs

//! Defines data models for Tariro's Global Knowledge Integration & Cross-Cultural Understanding module,
//! including cultural contexts, knowledge sources, and cross-cultural insights.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents specific cultural norms, values, and communication styles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContext {
    pub id: String,
    pub culture_name: String,
    pub geographic_region: String,
    pub language_dialect: String,
    pub key_values: Vec<String>, // e.g., "collectivism", "individualism", "respect_for_elders"
    pub communication_norms: HashMap<String, String>, // e.g., "directness": "low", "high_context": "true"
    pub ethical_biases_assessed: VerificationStatus, // New: Ethical bias assessment of the cultural model
    pub security_verified: VerificationStatus, // New: Security verification of context data
    pub last_updated: DateTime<Utc>,
}

/// Metadata about an information source, especially its cultural origin and biases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSource {
    pub id: String,
    pub source_name: String,
    pub source_url: Option<String>,
    pub data_type: String, // e.g., "text", "video", "oral_tradition_record"
    pub primary_culture_id: String, // ID of the CulturalContext most relevant to this source
    pub political_leanings_score: f64, // -1.0 (left) to 1.0 (right), 0.0 (neutral)
    pub factual_accuracy_score: f64, // 0.0 to 1.0, assessed by EpistemicReasoningModule
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the source's content and origin
    pub security_integrity: VerificationStatus, // New: Security integrity of source access
    pub last_crawled: DateTime<Utc>,
}

/// An understanding or conclusion derived from synthesizing knowledge across different cultures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCulturalInsight {
    pub id: String,
    pub insight_summary: String,
    pub contributing_cultural_contexts: Vec<String>, // IDs of CulturalContexts
    pub resolved_ambiguity_id: Option<String>, // ID of a related ambiguity resolved
    pub confidence: f64, // 0.0 to 1.0
    pub ethical_bias_assessed: VerificationStatus, // New: Ethical bias assessment of the insight
    pub security_verified: VerificationStatus, // New: Security verification of insight data
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of global knowledge integration processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalKnowledgeAudit {
    pub id: String,
    pub scope: String, // e.g., "all_knowledge_sources", "cross_cultural_communication"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sources_integrated: u64,
    pub cultural_biases_detected: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of knowledge integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of knowledge bases
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
