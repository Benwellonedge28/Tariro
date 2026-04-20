// Tariro_ide_idpte/src/cognitive_arch/hierarchical_rl_models.rs

//! Defines data models for Tariro's Hierarchical Reinforcement Learning module,
//! including high-level policies, sub-policies, and reward structures.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A high-level policy guiding the selection of sub-goals and long-term strategies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaPolicy {
    pub id: String,
    pub goal_id: String, // Link to Goal
    pub strategy_description: String,
    pub sub_policy_sequence: Vec<String>, // List of SubPolicy IDs
    pub expected_reward: f64,
    pub ethical_review_status: VerificationStatus,
    pub security_verified: VerificationStatus,
    pub created_at: DateTime<Utc>,
}

/// A specific sub-policy or skill designed to achieve a concrete sub-goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubPolicy {
    pub id: String,
    pub meta_policy_id: String, // Link to MetaPolicy
    pub sub_goal_description: String,
    pub action_set: Vec<String>,
    pub learning_progress: f64, // 0.0 to 1.0
    pub ethical_alignment: VerificationStatus,
    pub safety_verified: VerificationStatus,
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the learning and performance of the hierarchical RL system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HRLAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_meta_policies_learned: u64,
    pub average_reward_attained: f64,
    pub detected_policy_instabilities: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
