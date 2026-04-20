// Tariro_ide_idpte/src/cognitive_arch/unstructured_data_models.rs

//! Defines data models for Tariro's Self-Supervised Learning for Unstructured Data module,
//! including unlabeled data streams, self-supervised tasks, and learned representations.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A descriptor for a stream or dataset of raw, unstructured, and unlabeled data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlabeledDataStream {
    pub id: String,
    pub stream_name: String,
    pub data_modality: String, // e.g., "text", "image", "audio", "video", "sensor_data"
    pub data_source_origin: String, // e.g., "web_crawl", "internal_sensor_network", "simulated_environment"
    pub volume_gb: f64,
    pub ethical_privacy_review: VerificationStatus, // Ethical review of data collection/usage
    pub security_integrity_status: VerificationStatus, // Security of the data stream
    pub last_processed: DateTime<Utc>,
}

/// A proxy task automatically generated to extract meaningful patterns from unlabeled data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfSupervisedTask {
    pub id: String,
    pub stream_id: String, // Link to UnlabeledDataStream
    pub task_type: String, // e.g., "masked_language_modeling", "image_inpainting", "contrastive_learning"
    pub objective_function_metric: String,
    pub expected_representation_quality: f64, // 0.0 to 1.0
    pub ethical_bias_detection: VerificationStatus, // Ethical review for potential bias introduction
    pub security_audited: VerificationStatus, // Security audit of the SSL algorithm
    pub created_at: DateTime<Utc>,
}

/// A meaningful embedding, feature, or concept extracted from unstructured data via SSL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedRepresentation {
    pub id: String,
    pub task_id: String, // Link to SelfSupervisedTask
    pub representation_type: String, // e.g., "word_embedding", "image_features", "causal_factors"
    pub dimension: u32, // Dimensionality of the representation
    pub transfer_learning_potential: f64, // 0.0 to 1.0, how well it transfers to downstream tasks
    pub ethical_integrity: VerificationStatus, // Ethical integrity of the learned representation
    pub security_audited: VerificationStatus, // Security audit of the representation's generation
    pub generated_at: DateTime<Utc>,
}

/// Audit report for self-supervised learning processes for unstructured data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSL_AuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_data_streams_processed: u64,
    pub total_representations_learned: u64,
    pub detected_bias_introductions: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
