// Tariro_ide_idpte/src/environmental/federated_learning_models.rs

//! Defines data models for Tariro's Decentralized Federated Learning for Global Knowledge Synthesis module,
//! including federated learning sessions, local model updates, and global model synthesis.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A descriptor for a federated learning session or round.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedLearningSession {
    pub id: String,
    pub session_name: String,
    pub participating_agis: Vec<String>, // List of AGI instance IDs
    pub learning_goal: String, // e.g., "global_climate_model_refinement", "universal_language_translation"
    pub aggregation_protocol: String, // e.g., "FedAvg", "SecureFedAvg"
    pub ethical_privacy_review: VerificationStatus, // Ethical review of data privacy in learning
    pub security_integrity_status: VerificationStatus, // Security of the learning protocol
    pub initiated_at: DateTime<Utc>,
}

/// A local model update contributed by a participating AGI instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModelUpdate {
    pub id: String,
    pub session_id: String, // Link to FederatedLearningSession
    pub agi_instance_id: String,
    pub encrypted_update_hash: String, // Hash of the encrypted model update
    pub data_privacy_level: f64, // 0.0 to 1.0, level of privacy protection for local data
    pub ethical_data_handling_review: VerificationStatus, // Ethical review of local data handling
    pub submitted_at: DateTime<Utc>,
}

/// The aggregated and improved global model synthesized from local updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalModelSynthesis {
    pub id: String,
    pub session_id: String, // Link to FederatedLearningSession
    pub synthesized_model_hash: String,
    pub improvement_metrics: HashMap<String, f64>, // e.g., "accuracy_gain": 0.05
    pub zero_knowledge_proof_id: Option<String>, // Link to ZKP verifying synthesis
    pub ethical_integrity: VerificationStatus, // Ethical integrity of the synthesized model
    pub security_audited: VerificationStatus, // Security audit of the global model
    pub generated_at: DateTime<Utc>,
}

/// Audit report for decentralized federated learning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedLearningAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sessions_conducted: u64,
    pub total_local_updates_aggregated: u64,
    pub detected_data_poisoning_attempts: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
