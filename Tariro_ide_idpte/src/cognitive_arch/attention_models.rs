// Tariro_ide_idpte/src/cognitive_arch/attention_models.rs

//! Defines data models for Tariro's Attention & Salience Detection module,
//! including salient features, attention policies, and resource focus adjustments.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A detected feature or stimulus that requires cognitive attention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalientFeature {
    pub id: String,
    pub feature_type: String, // e.g., "novel_object", "sudden_noise", "goal_relevant_pattern"
    pub source_modality: String, // e.g., "vision", "audio", "internal_state"
    pub saliency_score: f64, // 0.0 (low) to 1.0 (high)
    pub context_relevance: f64, // 0.0 to 1.0, relevance to current goals/tasks
    pub detected_at: DateTime<Utc>,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of directing attention
    pub security_verified: VerificationStatus, // New: Security verification of feature validity
}

/// Defines rules and parameters for directing cognitive attention and resource allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionPolicy {
    pub id: String,
    pub policy_name: String,
    pub priority_rules: Vec<String>, // e.g., "prioritize_safety_alerts", "focus_on_active_goal"
    pub distraction_threshold: f64, // 0.0 to 1.0
    pub computational_budget_limit: HashMap<String, String>, // e.g., "cpu_percent": "0.8"
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the policy design
    pub security_audited: VerificationStatus, // New: Security audit of the policy implementation
    pub last_updated: DateTime<Utc>,
}

/// A dynamic adjustment in computational resources or processing priorities due to attention shift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceFocusAdjustment {
    pub id: String,
    pub policy_id: String,
    pub triggered_by_feature_id: String,
    pub adjusted_resources: HashMap<String, f64>, // e.g., "cpu_cores": 2.0, "memory_gb": 4.0
    pub processing_priority_change: f64, // positive for increase, negative for decrease
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the adjustment
    pub security_review_status: VerificationStatus, // New: Security review of the adjustment
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of attention mechanisms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_attention_shifts", "specific_policy_evaluation"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_attention_shifts: u64,
    pub detected_biases: Vec<String>, // e.g., "over_focus_on_negative_stimuli"
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of attention mechanisms
    pub security_integrity_status: VerificationStatus, // Overall security integrity of attention systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
