// Tariro_ide_idpte/src/cognitive_arch/curiosity_exploration_models.rs

//! Defines data models for Tariro's Advanced Curious Exploration module,
//! including intrinsic reward signals, exploration strategies, and novelty reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A signal representing Tariro's internal motivation to explore based on perceived novelty or uncertainty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrinsicReward {
    pub id: String,
    pub context: String,
    pub reward_value: f64, // 0.0 to 1.0
    pub novelty_source: String, // e.g., "unknown_input_pattern", "contradictory_prediction"
    pub ethical_review_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
}

/// A strategy designed to maximize information gain and explore unknown regions of a problem space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationStrategy {
    pub id: String,
    pub strategy_name: String,
    pub target_domain: String,
    pub expected_information_gain: f64,
    pub ethical_alignment: VerificationStatus,
    pub safety_verified: VerificationStatus,
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the results of exploring a specific domain or context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyReport {
    pub id: String,
    pub exploration_id: String,
    pub novel_concepts_found: Vec<String>,
    pub information_gain_score: f64,
    pub detected_anomalies: Vec<String>,
    pub ethical_impact_assessment: VerificationStatus,
    pub security_verified: VerificationStatus,
    pub generated_at: DateTime<Utc>,
}

/// Audit report for advanced curious exploration activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_exploration_cycles: u64,
    pub total_novelty_score: f64,
    pub detected_unsafe_explorations: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
