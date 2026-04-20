// Tariro_ide_idpte/src/environmental/social_norms_models.rs

//! Defines data models for Tariro's Social Norms Learning & Adaptation module,
//! including social norms, interaction logs, and norm adaptation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A learned social rule, custom, or etiquette.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    pub id: String,
    pub norm_description: String,
    pub context: String, // e.g., "professional_meeting", "casual_conversation", "specific_culture"
    pub strength: f64, // 0.0 to 1.0, how strongly it should be adhered to
    pub associated_values: Vec<String>, // e.g., "respect", "politeness", "efficiency"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the social norm itself
    pub security_verified: VerificationStatus, // New: Security verification of norm source trustworthiness
    pub learned_at: DateTime<Utc>,
}

/// A log of Tariro's interaction where social norms were applied or adapted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInteractionLog {
    pub id: String,
    pub interaction_partner_id: String,
    pub interaction_context: String,
    pub applied_norm_id: Option<String>, // Link to SocialNorm
    pub Tariro_action_summary: String,
    pub partner_reaction_summary: String,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of Tariro's interaction
    pub data_privacy_compliance: VerificationStatus, // New: Data privacy compliance of the interaction log
    pub timestamp: DateTime<Utc>,
}

/// A strategy for adapting Tariro's behavior to conform to social norms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormAdaptationStrategy {
    pub id: String,
    pub strategy_name: String,
    pub description: String,
    pub adaptation_mechanisms: Vec<String>, // e.g., "adjust_tone_of_voice", "use_cultural_idioms"
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the adaptation strategy
    pub security_audited: VerificationStatus, // New: Security audit of the strategy implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of social norms learning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNormsAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_social_interactions", "specific_cultural_contexts"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_norms_learned: u64,
    pub detected_norm_violations: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of social norms learning
    pub security_integrity_status: VerificationStatus, // Overall security integrity of social norms learning systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
