// Tariro_ide_idpte/src/cognitive_arch/h_rl_models.rs

//! Defines data models for Hierarchical Reinforcement Learning (HRL),
//! including high-level goals, sub-goals, policies, and agent states.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For policy verification

/// Represents a high-level, abstract goal for the HRL agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighLevelGoal {
    pub id: String,
    pub description: String,
    pub long_term_reward_function: String, // Describes how overall success is measured
    pub priority: f64,
    pub created_at: DateTime<Utc>,
    pub source_module: String, // New: e.g., "MetaGoalManager", "CuriosityExplorationModule"
    pub dynamic_relevance: f64, // New: How relevant this goal is to current meta-goals/environment (0.0 to 1.0)
}

/// Represents a more concrete, short-term sub-goal derived from a high-level goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubGoal {
    pub id: String,
    pub parent_goal_id: String,
    pub description: String,
    pub completion_condition: String, // Describes when this sub-goal is met
    pub reward_function: String, // Describes how success for this sub-goal is measured
    pub assigned_policy_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub estimated_complexity: f64, // New: Estimated difficulty (0.0 to 1.0)
}

/// Represents a learned policy (strategy) to achieve a sub-goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub sub_goal_id: String,
    pub name: String,
    pub algorithm_type: String, // e.g., "Q-learning", "DQN", "PPO"
    pub model_parameters_summary: String, // Short summary of underlying model
    pub expected_performance: f64, // Expected success rate or reward
    pub last_trained: DateTime<Utc>,
    pub verified_safe: bool, // Checked by ALEC-A
    pub version: String,
    pub verification_status: PolicyVerificationStatus, // New: Full verification status
    pub adaptation_history: Vec<PolicyAdaptationRecord>, // New: Records changes and reasons
}

/// Comprehensive verification status for a learned policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyVerificationStatus {
    pub ethical_status: VerificationStatus,
    pub security_status: VerificationStatus,
    pub robustness_status: VerificationStatus, // New: Robustness to environmental changes
    pub overall_status: VerificationStatus,
    pub last_verified: DateTime<Utc>,
    pub verification_log_id: String,
}

/// Records an adaptation or refinement made to a policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAdaptationRecord {
    pub timestamp: DateTime<Utc>,
    pub change_summary: String,
    pub reason: String, // e.g., "improved performance", "security patch", "ethical directive"
    pub old_version: String,
    pub new_version: String,
}

/// Represents the current state of the HRL agent, including active goals and policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HRLAgentState {
    pub agent_id: String,
    pub current_high_level_goal: Option<String>,
    pub active_sub_goals: Vec<String>, // List of sub-goal IDs currently being pursued
    pub active_policies: Vec<String>, // List of policy IDs currently being executed
    pub observed_environment_state: HashMap<String, String>,
    pub last_updated: DateTime<Utc>,
}
