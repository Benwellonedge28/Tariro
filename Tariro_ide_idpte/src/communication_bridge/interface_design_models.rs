// Tariro_ide_idpte/src/communication_bridge/interface_design_models.rs

//! Defines data models for Tariro's Adaptive Human-AGI Interface Design & Personalization module,
//! including user cognitive profiles, interface adaptation strategies, and experience optimization reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A dynamic model of a human user's cognitive traits, preferences, and emotional states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCognitiveProfile {
    pub id: String,
    pub user_id: String, // Unique identifier for the human user
    pub cognitive_style: String, // e.g., "analytical", "holistic", "visual"
    pub learning_preference: String, // e.g., "hands-on", "observational", "instructional"
    pub emotional_tendencies: HashMap<String, f64>, // e.g., "frustration_threshold": 0.7
    pub communication_modality_preference: String, // e.g., "text", "voice", "visual_diagrams"
    pub ethical_privacy_review: VerificationStatus, // Ethical review for privacy of user data
    pub last_updated: DateTime<Utc>,
}

/// A strategy for dynamically adjusting interface elements and workflows for a human user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceAdaptationStrategy {
    pub id: String,
    pub profile_id: String, // Link to UserCognitiveProfile
    pub adaptation_goal: String, // e.g., "reduce_cognitive_load", "improve_focus", "enhance_creativity"
    pub applied_ui_changes: Vec<String>, // e.g., "dark_mode_activated", "information_density_reduced", "contextual_help_enabled"
    pub expected_usability_gain: f64, // 0.0 to 1.0
    pub ethical_impact_assessment: VerificationStatus, // Ethical review of the adaptation's impact
    pub security_audited: VerificationStatus, // Security audit of the adaptation mechanism
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the effectiveness of interface adaptations in optimizing user experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceOptimizationReport {
    pub id: String,
    pub strategy_id: String, // Link to InterfaceAdaptationStrategy
    pub observed_user_metrics: HashMap<String, f64>, // e.g., "task_completion_time": 120.5, "satisfaction_score": 0.9
    pub measured_impact: String,
    pub overall_effectiveness_score: f64, // 0.0 to 1.0
    pub ethical_compliance_status: VerificationStatus,
    pub generated_at: DateTime<Utc>,
}

/// Audit report for adaptive human-AGI interface design and personalization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_profiles_managed: u64,
    pub total_adaptations_deployed: u64,
    pub detected_negative_impacts: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
