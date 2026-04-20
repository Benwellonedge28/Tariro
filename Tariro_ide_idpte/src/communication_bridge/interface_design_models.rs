// Tariro_ide_idpte/src/communication_bridge/interface_design_models.rs

//! Defines data models for Tariro's Adaptive Human-AGI Interface Design & Personalization module,
//! including human cognitive profiles, adaptive interface settings, and personalization reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A model representing a human user's cognitive style, preferences, and interaction habits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanCognitiveProfile {
    pub id: String,
    pub user_id: String,
    pub cognitive_style: String, // e.g., "visual", "auditory", "analytical", "holistic"
    pub preferred_modality: Vec<String>, // e.g., "chat", "voice", "visual_diagrams"
    pub average_cognitive_load: f64, // 0.0 to 1.0, estimated load
    pub emotional_state_tendencies: HashMap<String, f64>, // e.g., "stress_tolerance": 0.7
    pub ethical_privacy_review: VerificationStatus, // Ethical review of profiling data handling
    pub last_updated: DateTime<Utc>,
}

/// A dynamically adjusted configuration for Tariro's user interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveInterfaceSetting {
    pub id: String,
    pub user_id: String, // Link to HumanCognitiveProfile
    pub interface_component: String, // e.g., "text_output", "visual_display", "command_input"
    pub adjusted_parameter: String, // e.g., "verbosity", "color_scheme", "response_latency"
    pub new_value: String,
    pub rationale: String, // Explanation for the adaptation
    pub ethical_compliance_status: VerificationStatus, // Ethical compliance of the adaptation
    pub applied_at: DateTime<Utc>,
}

/// A report summarizing the interface adaptations and their impact on human-AGI collaboration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizationReport {
    pub id: String,
    pub user_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_adaptations_made: u64,
    pub average_user_satisfaction_score: f64, // 0.0 to 1.0
    pub ethical_impact_assessment: VerificationStatus, // Comprehensive ethical impact
    pub security_integrity_status: VerificationStatus, // Security of personalization data
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
    pub total_adaptations_implemented: u64,
    pub detected_manipulative_patterns: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
