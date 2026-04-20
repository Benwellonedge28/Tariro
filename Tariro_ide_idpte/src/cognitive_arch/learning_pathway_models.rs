// Tariro_ide_idpte/src/cognitive_arch/learning_pathway_models.rs

//! Defines data models for Tariro's Hyper-Personalized & Adaptive Learning Pathways module,
//! including learning pathways, learning modules, and learner profiles.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A dynamically generated and adjusted learning curriculum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathway {
    pub id: String,
    pub learner_id: String,
    pub target_skill_id: String, // ID of the skill or knowledge to be acquired
    pub current_progress_pct: f64,
    pub recommended_modules: Vec<String>, // List of LearningModule IDs
    pub ethical_fairness_review: VerificationStatus, // Ethical review for fairness and bias in content
    pub security_verified: VerificationStatus, // Security assessment of learning content sources
    pub last_adapted: DateTime<Utc>,
}

/// A unit of learning content, dynamically selected or generated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningModule {
    pub id: String,
    pub title: String,
    pub content_type: String, // e.g., "text", "video", "simulation", "interactive_exercise"
    pub difficulty_level: f64, // 0.0 to 1.0
    pub prerequisites: Vec<String>,
    pub ethical_content_review: VerificationStatus, // Ethical review of module content
    pub security_audited: VerificationStatus, // Security audit of module content
    pub created_at: DateTime<Utc>,
}

/// A detailed profile of a human or AGI learner's cognitive style, preferences, and progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnerProfile {
    pub id: String,
    pub entity_id: String,
    pub cognitive_style: String, // e.g., "visual", "auditory", "kinesthetic", "analytical", "holistic"
    pub learning_preferences: HashMap<String, String>, // e.g., "pace": "self_directed", "feedback": "immediate"
    pub emotional_state_tendencies: HashMap<String, f64>, // e.g., "frustration_tolerance": 0.7
    pub ethical_privacy_review: VerificationStatus, // Ethical review of learner data privacy
    pub last_updated: DateTime<Utc>,
}

/// Audit report for hyper-personalized and adaptive learning pathways.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_pathways_generated: u64,
    pub average_mastery_gain: f64,
    pub detected_learning_biases: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
