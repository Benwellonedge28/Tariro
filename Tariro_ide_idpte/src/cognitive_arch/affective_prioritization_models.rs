// Tariro_ide_idpte/src/cognitive_arch/affective_prioritization_models.rs

//! Defines data models for Tariro's Emotion-Driven Cognitive Prioritization module,
//! including emotional states, cognitive priorities, and emotion-driven decisions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// Represents Tariro's internal simulated emotional state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub id: String,
    pub emotion_type: String, // e.g., "curiosity", "satisfaction", "frustration", "urgency"
    pub intensity: f64, // 0.0 to 1.0
    pub trigger_event_summary: String,
    pub ethical_context: Vec<String>, // Ethical aspects related to this emotion
    pub created_at: DateTime<Utc>,
}

/// A dynamically adjusted priority level for a cognitive task or resource allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitivePriority {
    pub id: String,
    pub task_id: String, // ID of the task or resource being prioritized
    pub emotional_influence_id: Option<String>, // Link to EmotionalState if influenced
    pub current_priority_score: f64, // 0.0 to 1.0
    pub rationale: String,
    pub ethical_review_status: VerificationStatus, // Ethical review of the prioritization decision
    pub security_verified: VerificationStatus, // Security verification of the prioritization mechanism
    pub last_updated: DateTime<Utc>,
}

/// A record of a decision significantly influenced by Tariro's emotional states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionDrivenDecision {
    pub id: String,
    pub decision_summary: String,
    pub primary_emotional_state_id: String, // Link to EmotionalState
    pub counterfactual_without_emotion: String, // What would have been decided without emotional influence
    pub ethical_compliance: VerificationStatus, // Ethical compliance of the emotion-driven decision
    pub transparency_level: VerificationStatus, // Transparency of the decision-making process
    pub recorded_at: DateTime<Utc>,
}

/// A report summarizing the audit of emotion-driven cognitive prioritization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_prioritization_events", "critical_resource_allocation"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_prioritization_events: u64,
    pub emotional_influence_detected: u64,
    pub detected_bias_in_emotions: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
