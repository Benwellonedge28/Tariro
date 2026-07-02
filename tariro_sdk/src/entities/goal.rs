//! Goal entity for Tariro AGI meta-goal management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Goal priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoalPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for GoalPriority {
    fn default() -> Self {
        GoalPriority::Medium
    }
}

/// Goal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GoalStatus {
    Active,
    Completed,
    Failed,
    Suspended,
    Archived,
}

impl Default for GoalStatus {
    fn default() -> Self {
        GoalStatus::Active
    }
}

/// A goal in the Tariro AGI system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: GoalPriority,
    pub status: GoalStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub parent_goal_id: Option<String>,
    pub sub_goals: Vec<String>,
    pub metrics: GoalMetrics,
    pub progress_history: Vec<ProgressUpdate>,
}

impl Goal {
    /// Create a new goal
    pub fn new(title: String, description: String, priority: GoalPriority) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            priority,
            status: GoalStatus::Active,
            created_at: now,
            updated_at: now,
            completed_at: None,
            parent_goal_id: None,
            sub_goals: Vec::new(),
            metrics: GoalMetrics::default(),
            progress_history: Vec::new(),
        }
    }

    /// Update goal progress
    pub fn update_progress(&mut self, progress: f32, notes: String) {
        self.progress_history.push(ProgressUpdate {
            timestamp: Utc::now(),
            progress,
            notes,
        });
        self.updated_at = Utc::now();
    }

    /// Mark goal as completed
    pub fn complete(&mut self) {
        self.status = GoalStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

/// Metrics for tracking goal progress
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GoalMetrics {
    pub progress: f32,
    pub success_criteria: Vec<String>,
    pub current_metrics: std::collections::HashMap<String, f32>,
    pub benchmark_values: std::collections::HashMap<String, f32>,
}

/// Progress update record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressUpdate {
    pub timestamp: DateTime<Utc>,
    pub progress: f32,
    pub notes: String,
}