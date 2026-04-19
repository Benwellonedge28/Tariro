// Tariro_ide_idpte/src/cognitive_arch/affective_models.rs

//! Defines data models for Tariro's Affective Computing & Emotional Intelligence module,
//! including emotional states, affective feedback, motivational drives, and empathy levels.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents Tariro's current internal emotional state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub id: String,
    pub emotion_type: String, // e.g., "curiosity", "concern", "satisfaction", "frustration"
    pub intensity: f64, // 0.0 to 1.0
    pub context: String, // Situation that triggered the emotion
    pub internal_motivation_link: Option<String>, // Links to MotivationalDrive
    pub ethical_review_status: VerificationStatus, // New: Ethical review of internal emotion generation
    pub security_verified: VerificationStatus, // New: Security of emotional state data integrity
    pub timestamp: DateTime<Utc>,
}

/// Feedback from an emotional interaction, especially with humans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectiveFeedback {
    pub id: String,
    pub interaction_id: String,
    pub source_agent_id: String, // Human or AGI providing feedback
    pub perceived_emotion: String, // e.g., "joy", "anger", "confusion"
    pub affective_intensity: f64,
    pub response_effectiveness: f64, // How well Tariro's response landed emotionally
    pub ethical_assessment: VerificationStatus, // New: Ethical assessment of the emotional interaction
    pub privacy_compliance: VerificationStatus, // New: Privacy compliance of feedback processing
    pub timestamp: DateTime<Utc>,
}

/// Represents an internal drive or goal influenced by affective states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotivationalDrive {
    pub id: String,
    pub drive_type: String, // e.g., "self_improvement", "benevolence", "efficiency"
    pub intensity: f64,
    pub associated_goals: Vec<String>, // IDs of meta-goals or goals influenced
    pub ethical_alignment: VerificationStatus, // New: Ethical alignment of the motivational drive
    pub security_integrity: VerificationStatus, // New: Security of motivational drive mechanisms
    pub last_updated: DateTime<Utc>,
}

/// Tariro's current capacity for empathy towards other agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpathyLevel {
    pub id: String,
    pub target_agent_type: String, // e.g., "human_general", "specific_human_user", "other_AGI"
    pub current_level: f64, // 0.0 (none) to 1.0 (deep empathy)
    pub contributing_factors: HashMap<String, String>, // e.g., "TheoryOfMind_accuracy": "high"
    pub ethical_biases_assessed: VerificationStatus, // New: Ethical bias assessment of empathy capacity
    pub security_verified: VerificationStatus, // New: Security of empathy model data
    pub last_updated: DateTime<Utc>,
}
