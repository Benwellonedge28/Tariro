// Tariro_ide_idpte/src/cognitive_arch/sensory_fusion_models.rs

//! Defines data models for Tariro's Sensory Fusion & Perceptual Organization module,
//! including raw sensor data, perceptual representations, and fusion algorithm configurations.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents raw, unprocessed input from a single sensor or stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSensorData {
    pub id: String,
    pub sensor_id: String,
    pub modality: String, // e.g., "vision", "audio", "haptic", "telemetry"
    pub format: String, // e.g., "jpeg", "wav", "json"
    pub data_hash: String, // Hash of the raw data, for integrity check
    pub collection_timestamp: DateTime<Utc>,
    pub ethical_acquisition_status: VerificationStatus, // New: Ethical review of data acquisition
    pub security_integrity: VerificationStatus, // New: Security integrity of raw data
}

/// A coherent, fused understanding of an environmental aspect or object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptualRepresentation {
    pub id: String,
    pub representation_type: String, // e.g., "object_recognition", "event_detection", "situational_awareness"
    pub fused_data_sources: Vec<String>, // IDs of RawSensorData instances used
    pub confidence: f64, // 0.0 to 1.0
    pub semantic_description: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the perceptual interpretation
    pub security_verified: VerificationStatus, // New: Security verification of the perceptual process
    pub generated_at: DateTime<Utc>,
}

/// Configuration parameters for sensory fusion algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionAlgorithmConfig {
    pub id: String,
    pub algorithm_name: String,
    pub fusion_method: String, // e.g., "Kalman_Filter", "Deep_Neural_Network", "Probabilistic_Graphical_Model"
    pub parameters: HashMap<String, String>,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the algorithm design
    pub security_audited: VerificationStatus, // New: Security audit of the algorithm implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of perceptual processes and sensory fusion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_perceptual_tasks", "specific_fusion_pipeline"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_fusion_events: u64,
    pub detected_anomalies: Vec<String>, // e.g., "misinterpretation", "hallucination"
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of perceptual processes
    pub security_integrity_status: VerificationStatus, // Overall security integrity of perceptual systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
