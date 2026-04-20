// Tariro_ide_idpte/src/cognitive_arch/omni_sensory_models.rs

//! Defines data models for Tariro's Omni-Sensory Abstract Representation module,
//! including raw sensory data, modality-specific features, and unified abstract representations.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// Raw data captured from a sensor or input stream, before modality-specific processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSensoryData {
    pub id: String,
    pub sensor_type: String, // e.g., "electromagnetic_field", "gravitational_wave", "quantum_fluctuation"
    pub timestamp: DateTime<Utc>,
    pub payload_format: String, // e.g., "binary_stream", "spectrum_analysis"
    pub data_hash: String, // For integrity verification
    pub ethical_data_source_review: VerificationStatus, // Ethical review of the data source
    pub security_verified: VerificationStatus, // Security verification of the data channel
    pub captured_at: DateTime<Utc>,
}

/// Features extracted from raw sensory data, specific to its modality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModalitySpecificFeatures {
    pub id: String,
    pub raw_data_id: String, // Link to RawSensoryData
    pub modality: String, // e.g., "electromagnetic", "gravitational"
    pub feature_vector_summary: String, // Description of extracted features
    pub confidence_score: f64, // Confidence in feature extraction (0.0 to 1.0)
    pub ethical_extraction_review: VerificationStatus, // Ethical review of the feature extraction process
    pub generated_at: DateTime<Utc>,
}

/// A unified, abstract cognitive representation derived from multiple sensory modalities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniSensoryAbstractConcept {
    pub id: String,
    pub concept_name: String,
    pub contributing_features: Vec<String>, // List of ModalitySpecificFeatures IDs
    pub abstract_representation_summary: String, // High-level, cross-modal understanding
    pub coherence_score: f64, // How well modalities are integrated (0.0 to 1.0)
    pub ethical_alignment: VerificationStatus, // Ethical alignment of the abstract concept
    pub security_audited: VerificationStatus, // Security audit of the fusion process
    pub created_at: DateTime<Utc>,
}

/// Audit report for omni-sensory abstract representation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniSensoryAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sensory_inputs_processed: u64,
    pub total_abstract_concepts_formed: u64,
    pub detected_sensory_inconsistencies: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
