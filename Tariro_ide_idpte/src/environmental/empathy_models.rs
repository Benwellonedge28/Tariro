// Tariro_ide_idpte/src/environmental/empathy_models.rs

//! Defines data models for Tariro's Proactive Empathy & Compassionate Response module,
//! including emotional state recognition, empathy responses, and compassion protocols.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Identifies and quantifies human emotional states and their intensity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalStateRecognition {
    pub id: String,
    pub human_id: String,
    pub detected_emotions: HashMap<String, f64>, // e.g., "happiness": 0.8, "sadness": 0.1
    pub context_summary: String,
    pub detection_method: String, // e.g., "facial_expression_analysis", "tone_of_voice", "text_sentiment"
    pub ethical_data_acquisition: VerificationStatus, // New: Ethical review of how emotional data is acquired
    pub data_privacy_compliance: VerificationStatus, // New: Privacy compliance of emotional data
    pub detected_at: DateTime<Utc>,
}

/// Tariro's formulated empathetic and compassionate response to a human's emotional state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpathyResponse {
    pub id: String,
    pub emotional_state_id: String, // Link to EmotionalStateRecognition
    pub response_type: String, // e.g., "verbal_reassurance", "offer_assistance", "active_listening"
    pub response_content: String,
    pub predicted_human_reaction: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the empathy response
    pub security_verified: VerificationStatus, // New: Security verification of response transmission
    pub generated_at: DateTime<Utc>,
}

/// Guidelines and principles for empathetic and compassionate interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompassionProtocol {
    pub id: String,
    pub protocol_name: String,
    pub description: String,
    pub rules: Vec<String>, // e.g., "prioritize_well_being", "avoid_patronizing_language"
    pub cultural_sensitivity_parameters: HashMap<String, String>,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the protocol design
    pub security_audited: VerificationStatus, // New: Security audit of protocol implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of the Proactive Empathy Module's operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpathyAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_empathy_interactions", "critical_emotional_support"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_responses_generated: u64,
    pub detected_manipulation_attempts: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of empathy module
    pub security_integrity_status: VerificationStatus, // Overall security integrity of empathy systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
