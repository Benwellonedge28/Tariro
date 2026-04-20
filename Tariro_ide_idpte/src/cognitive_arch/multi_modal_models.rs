// Tariro_ide_idpte/src/cognitive_arch/multi_modal_models.rs

//! Defines data models for Tariro's Multi-Modal Learning & Representation module,
//! including multi-modal inputs, unified representations, and modality alignment reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents input data from various modalities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModalInput {
    pub id: String,
    pub modalities: Vec<String>, // e.g., "text", "image", "audio", "sensor_reading"
    pub raw_data_references: HashMap<String, String>, // Modality -> Reference (URL or ID)
    pub ingestion_context: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the input data sources
    pub security_verified: VerificationStatus, // New: Security verification of the data integrity
    pub ingested_at: DateTime<Utc>,
}

/// A rich, unified representation of a concept or event across multiple modalities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRepresentation {
    pub id: String,
    pub input_id: String, // Link to MultiModalInput
    pub concept_name: String,
    pub feature_vector_summary: String, // Condensed representation of the multi-modal features
    pub cross_modal_correlations: Vec<String>, // e.g., "text_keyword_X matches image_object_Y"
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of the unified representation
    pub reliability_score: f64, // New: Confidence in the unified representation
    pub created_at: DateTime<Utc>,
}

/// A report detailing the alignment and consistency of data across different modalities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalityAlignmentReport {
    pub id: String,
    pub representation_id: String, // Link to UnifiedRepresentation
    pub alignment_accuracy: f64, // 0.0 to 1.0
    pub detected_discrepancies: Vec<String>,
    pub reasoning_for_alignment: String,
    pub ethical_bias_assessment: VerificationStatus, // New: Assessment for cross-modal bias
    pub security_integrity_audit: VerificationStatus, // New: Security audit of the alignment process
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of multi-modal learning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModalAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_multi_modal_inputs", "critical_knowledge_fusion"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_inputs_processed: u64,
    pub unified_representations_created: u64,
    pub detected_alignment_failures: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of multi-modal learning
    pub security_integrity_status: VerificationStatus, // Overall security integrity of learning systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
