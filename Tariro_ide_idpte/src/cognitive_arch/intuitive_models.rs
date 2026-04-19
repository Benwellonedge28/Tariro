// Tariro_ide_idpte/src/cognitive_arch/intuitive_models.rs

//! Defines data models for Tariro's Intuitive Physics & Naive Psychology module,
//! including physical laws, behavioral patterns, and prediction outcomes.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a derived physical law or principle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalLaw {
    pub id: String,
    pub description: String,
    pub mathematical_formulation: Option<String>,
    pub domain_of_applicability: String, // e.g., "macro_scale", "fluid_dynamics", "projectile_motion"
    pub confidence: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the law's interpretation/application
    pub security_verified: VerificationStatus, // New: Security of law data integrity
    pub derived_from_experiences: Vec<String>, // IDs of SensorimotorExperiences
    pub last_updated: DateTime<Utc>,
}

/// Represents a learned pattern of human or agent behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub id: String,
    pub description: String,
    pub trigger_conditions: String,
    pub predicted_actions: Vec<String>,
    pub confidence: f64,
    pub ethical_biases_assessed: VerificationStatus, // New: Assessment for inherent biases in the pattern
    pub security_verified: VerificationStatus, // New: Security of pattern data integrity
    pub derived_from_interactions: Vec<String>, // IDs of SocialInteractionEvents
    pub last_updated: DateTime<Utc>,
}

/// Represents the outcome of an intuitive prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionOutcome {
    pub id: String,
    pub prediction_target: String,
    pub predicted_state: String,
    pub actual_state: Option<String>,
    pub accuracy_score: f64, // 0.0 to 1.0
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of the prediction
    pub security_integrity: VerificationStatus, // New: Integrity of the prediction process
    pub timestamp: DateTime<Utc>,
}

/// Current intuitive understanding of the physical world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitivePhysicsState {
    pub id: String,
    pub active_laws: Vec<String>, // IDs of PhysicalLaws currently in use
    pub environmental_context: String,
    pub overall_coherence_score: f64,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the state representation
    pub security_verified: VerificationStatus, // New: Security of the state data
    pub last_updated: DateTime<Utc>,
}

/// Current intuitive understanding of human and agent psychology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaivePsychologyProfile {
    pub id: String,
    pub target_agent_id: Option<String>, // Specific agent or "general_human"
    pub active_patterns: Vec<String>, // IDs of BehavioralPatterns currently active/relevant
    pub empathy_level: f64, // 0.0 to 1.0, current or inferred
    pub ethical_biases_assessed: VerificationStatus, // New: Ethical bias assessment of the profile
    pub security_verified: VerificationStatus, // New: Security of the profile data
    pub last_updated: DateTime<Utc>,
}
