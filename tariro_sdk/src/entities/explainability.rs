//! Explainability log entity for tracking AI decisions

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type of decision being explained
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionType {
    ActionSelection,
    PolicyEvaluation,
    EthicalJudgment,
    GoalPrioritization,
    ResourceAllocation,
    SelfModification,
    UserResponse,
    Custom(String),
}

/// Confidence level of the decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConfidenceLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

impl Default for ConfidenceLevel {
    fn default() -> Self {
        ConfidenceLevel::Medium
    }
}

/// Explainability log for AI decision transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityLog {
    pub id: String,
    pub decision_type: DecisionType,
    pub timestamp: DateTime<Utc>,
    pub input_context: String,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub output_action: String,
    pub confidence: ConfidenceLevel,
    pub confidence_score: f32,
    pub alternatives_considered: Vec<Alternative>,
    pub ethical_review: Option<EthicalReview>,
    pub user_feedback: Option<UserFeedback>,
}

impl ExplainabilityLog {
    /// Create a new explainability log
    pub fn new(decision_type: DecisionType, input_context: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            decision_type,
            timestamp: Utc::now(),
            input_context,
            reasoning_chain: Vec::new(),
            output_action: String::new(),
            confidence: ConfidenceLevel::Medium,
            confidence_score: 0.5,
            alternatives_considered: Vec::new(),
            ethical_review: None,
            user_feedback: None,
        }
    }

    /// Add a reasoning step
    pub fn add_reasoning_step(&mut self, step: ReasoningStep) {
        self.reasoning_chain.push(step);
    }

    /// Add an alternative that was considered
    pub fn add_alternative(&mut self, alternative: Alternative) {
        self.alternatives_considered.push(alternative);
    }
}

/// A single step in the reasoning chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_number: u32,
    pub description: String,
    pub evidence: Vec<String>,
    pub conclusion: String,
    pub confidence: f32,
}

/// An alternative action that was considered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub action: String,
    pub reason_rejected: String,
    pub estimated_outcome: String,
}

/// Ethical review of the decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalReview {
    pub reviewed_at: DateTime<Utc>,
    pub violations_found: Vec<EthicalViolation>,
    pub overall_ethical_score: f32,
    pub recommendations: Vec<String>,
}

/// An ethical violation detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalViolation {
    pub principle: String,
    pub severity: ViolationSeverity,
    pub description: String,
    pub mitigation: Option<String>,
}

/// Severity of ethical violation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViolationSeverity {
    Minor,
    Moderate,
    Serious,
    Critical,
}

impl Default for ViolationSeverity {
    fn default() -> Self {
        ViolationSeverity::Minor
    }
}

/// User feedback on the decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFeedback {
    pub received_at: DateTime<Utc>,
    pub rating: i32,
    pub comment: Option<String>,
    pub corrections: Vec<String>,
}