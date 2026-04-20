// Tariro_ide_idpte/src/environmental/societal_impact_models.rs

//! Defines data models for Tariro's Societal Impact Forecasting & Mitigation module,
//! including societal impact predictions, mitigation strategies, and feedback loops.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a forecast of potential societal effects of AGI actions or capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalImpactPrediction {
    pub id: String,
    pub decision_or_capability_id: String,
    pub impact_area: String, // e.g., "employment", "privacy", "cultural_cohesion", "global_equity"
    pub predicted_effect: String, // e.g., "job_displacement", "enhanced_surveillance", "cultural_homogenization"
    pub severity_score: f64, // 0.0 to 1.0 (magnitude of impact)
    pub likelihood_score: f64, // 0.0 to 1.0 (probability of occurrence)
    pub predicted_ethical_alignment: VerificationStatus, // New: Predicted ethical alignment of the impact
    pub security_risk_assessment: VerificationStatus, // New: Security risk of the prediction data
    pub generated_at: DateTime<Utc>,
}

/// A formulated strategy to minimize negative societal consequences and amplify positive impacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    pub id: String,
    pub prediction_id: String,
    pub strategy_name: String,
    pub proposed_actions: Vec<String>, // e.g., "implement_retraining_programs", "enhance_data_anonymization", "promote_cultural_exchange_algorithms"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the mitigation strategy
    pub security_review_status: VerificationStatus, // New: Security review of the mitigation strategy
    pub approved_by: String, // e.g., "Tariro_SIFM", "ALEC-A", "HumanOversight"
    pub created_at: DateTime<Utc>,
}

/// A mechanism for gathering and integrating real-world societal feedback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalFeedbackLoop {
    pub id: String,
    pub monitored_agi_action_id: String,
    pub feedback_source_type: String, // e.g., "social_media_analysis", "public_opinion_polls", "expert_panels"
    pub feedback_summary: String,
    pub sentiment_score: f64, // -1.0 (negative) to 1.0 (positive)
    pub ethical_compliance_assessment: VerificationStatus, // New: Ethical compliance of feedback gathering
    pub security_verified: VerificationStatus, // New: Security of feedback data processing
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of Tariro's societal impact analyses and mitigation efforts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAuditReport {
    pub id: String,
    pub audit_scope: String, // e.g., "all_predictions", "specific_mitigation_strategies"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_predictions_audited: u64,
    pub total_mitigation_strategies_deployed: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of societal impact management
    pub security_integrity_status: VerificationStatus, // Overall security integrity of impact data
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
