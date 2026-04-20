// Tariro_ide_idpte/src/communication_bridge/human_agi_interface_models.rs

//! Defines data models for Tariro's Adaptive Human-AGI Interface Design & Personalization module,
//! including interface adaptation profiles, user cognitive styles, and emotional state feedback.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A profile detailing how Tariro's interface should adapt for a specific human user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceAdaptationProfile {
    pub id: String,
    pub user_id: String,
    pub preferred_layout: String, // e.g., "minimalist", "data_rich", "conversational"
    pub preferred_modality: Vec<String>, // e.g., "visual", "auditory", "haptic"
    pub adaptation_rules: Vec<String>, // Rules for dynamic adjustment
    pub ethical_privacy_review: VerificationStatus, // Ethical review of personalization data usage
    pub security_verified: VerificationStatus, // Security of adaptation algorithms
    pub last_updated: DateTime<Utc>,
}

/// An assessment of a human user's typical cognitive processing style.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCognitiveStyle {
    pub id: String,
    pub user_id: String,
    pub cognitive_preference_type: String, // e.g., "visual_learner", "analytical_thinker", "intuitive_decision_maker"
    pub processing_speed_estimate: f64, // Normalized 0.0 to 1.0
    pub cognitive_load_tolerance: f64, // Normalized 0.0 to 1.0
    pub ethical_assessment_status: VerificationStatus, // Ethical review of cognitive profiling
    pub assessed_at: DateTime<Utc>,
}

/// Real-time feedback on a human user's emotional state, inferred or provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalStateFeedback {
    pub id: String,
    pub user_id: String,
    pub inferred_emotion: String, // e.g., "stressed", "focused", "curious"
    pub confidence_score: f64, // 0.0 to 1.0
    pub timestamp: DateTime<Utc>,
    pub ethical_data_handling_review: VerificationStatus, // Ethical review of emotional data processing
}

/// Audit report for adaptive human-AGI interface design and personalization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_adaptations_made: u64,
    pub positive_user_feedback_count: u64,
    pub detected_negative_impacts: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
