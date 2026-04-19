// Tariro_ide_idpte/src/environmental/social_models.rs

//! Defines data models for Tariro's Social Intelligence & Norms module,
//! including social norms, interaction events, and cultural contexts.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a learned or observed social norm, custom, or etiquette.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    pub id: String,
    pub description: String,
    pub context: String, // e.g., "professional_settings", "online_forums", "family_interactions"
    pub severity_of_violation: f64, // 0.0 (minor) to 1.0 (severe)
    pub inferred_purpose: String, // e.g., "maintain harmony", "facilitate cooperation"
    pub cultural_context_id: String, // Link to specific cultural context
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the norm itself
    pub security_verified: VerificationStatus, // New: Security of norm data integrity
    pub learned_at: DateTime<Utc>,
}

/// Represents an observed interaction event within a social system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInteractionEvent {
    pub id: String,
    pub participants: Vec<String>, // IDs of AGI and human participants
    pub interaction_type: String, // e.g., "conversation", "group_task", "conflict_resolution"
    pub observed_behaviors: Vec<String>,
    pub detected_norm_violations: Vec<String>, // IDs of SocialNorms violated
    pub inferred_emotions: HashMap<String, String>, // Participant ID -> Inferred Emotion (from TheoryOfMind)
    pub timestamp: DateTime<Utc>,
    pub integrity_verified: VerificationStatus, // New: Integrity of the event record
}

/// Represents a collection of relevant cultural information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalContext {
    pub id: String,
    pub name: String,
    pub demographics: HashMap<String, String>,
    pub key_values: Vec<String>,
    pub communication_styles: Vec<String>,
    pub associated_social_norms: Vec<String>, // IDs of SocialNorms specific to this culture
    pub last_updated: DateTime<Utc>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of cultural data collection
}

/// A report summarizing Tariro's ethical compliance in social interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalSocialComplianceReport {
    pub id: String,
    pub audit_period_start: DateTime<Utc>,
    pub audit_period_end: DateTime<Utc>,
    pub overall_compliance_score: f64, // 0.0 to 1.0
    pub identified_norm_violations: Vec<String>, // IDs of SocialInteractionEvents with violations
    pub recommendations_for_social_conduct: Vec<String>,
    pub ethical_review_status: VerificationStatus, // Overall ethical compliance of report
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
