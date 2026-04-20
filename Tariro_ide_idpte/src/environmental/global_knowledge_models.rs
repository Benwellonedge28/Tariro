// Tariro_ide_idpte/src/environmental/global_knowledge_models.rs

//! Defines data models for Tariro's Global Knowledge Integration & Cross-Cultural Understanding module,
//! including cultural contexts, knowledge sources, and cross-cultural insights.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A structured representation of cultural norms, values, beliefs, and practices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContext {
    pub id: String,
    pub culture_name: String,
    pub geographical_regions: Vec<String>,
    pub key_values: HashMap<String, String>, // e.g., "family_importance": "high", "individualism": "low"
    pub communication_styles: Vec<String>, // e.g., "high_context", "indirect"
    pub ethical_nuances: HashMap<String, String>, // Specific ethical considerations in this culture
    pub ethical_review_status: VerificationStatus, // New: Ethical review of cultural data representation
    pub security_verified: VerificationStatus, // New: Security verification of cultural data integrity
    pub last_updated: DateTime<Utc>,
}

/// Metadata about an external knowledge repository or data stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSource {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub data_format: String,
    pub reliability_score: f64, // 0.0 to 1.0, based on EpistemicReasoningModule
    pub perceived_biases: Vec<String>, // e.g., "western_centric", "corporate_funded"
    pub ethical_provenance: VerificationStatus, // New: Ethical review of knowledge source's origins and practices
    pub security_integrity: VerificationStatus, // New: Security integrity of the knowledge source
    pub last_accessed: DateTime<Utc>,
}

/// An insight or understanding derived from analyzing knowledge across multiple cultural contexts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossCulturalInsight {
    pub id: String,
    pub insight_summary: String,
    pub related_cultural_contexts: Vec<String>, // IDs of CulturalContexts involved
    pub implications_for_interaction: String,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of the insight
    pub security_integrity: VerificationStatus, // New: Security integrity of the insight generation
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of global knowledge integration processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalKnowledgeAudit {
    pub id: String,
    pub scope: String, // e.g., "all_knowledge_sources", "cultural_context_models"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sources_processed: u64,
    pub detected_biases: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of knowledge integration
    pub security_integrity_status: VerificationStatus, // Overall security integrity of knowledge systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
