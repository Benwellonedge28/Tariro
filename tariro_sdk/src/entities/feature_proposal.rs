//! Feature Proposal entity for self-modification system

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source of a feature proposal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Source {
    SelfMonitoring,
    UserRequest,
    SystemOptimization,
    EthicalReview,
    PerformanceAnalysis,
    SecurityAudit,
    Custom(String),
}

impl Default for Source {
    fn default() -> Self {
        Source::SelfMonitoring
    }
}

impl ToString for Source {
    fn to_string(&self) -> String {
        match self {
            Source::SelfMonitoring => "SelfMonitoring".to_string(),
            Source::UserRequest => "UserRequest".to_string(),
            Source::SystemOptimization => "SystemOptimization".to_string(),
            Source::EthicalReview => "EthicalReview".to_string(),
            Source::PerformanceAnalysis => "PerformanceAnalysis".to_string(),
            Source::SecurityAudit => "SecurityAudit".to_string(),
            Source::Custom(s) => s.clone(),
        }
    }
}

/// Priority levels for feature proposals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_string(),
            Priority::Medium => "Medium".to_string(),
            Priority::High => "High".to_string(),
            Priority::Critical => "Critical".to_string(),
        }
    }
}

/// Status of a feature proposal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Proposed,
    UnderReview,
    Approved,
    Rejected,
    InProgress,
    Completed,
    Cancelled,
}

impl Default for Status {
    fn default() -> Self {
        Status::Proposed
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Proposed => "Proposed".to_string(),
            Status::UnderReview => "UnderReview".to_string(),
            Status::Approved => "Approved".to_string(),
            Status::Rejected => "Rejected".to_string(),
            Status::InProgress => "InProgress".to_string(),
            Status::Completed => "Completed".to_string(),
            Status::Cancelled => "Cancelled".to_string(),
        }
    }
}

/// A feature proposal for self-modification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub source: Source,
    pub priority: Priority,
    pub status: Status,
    pub proposed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub target_modules: Vec<String>,
    pub expected_impact: String,
    pub risk_assessment: RiskAssessment,
    pub approval_chain: Vec<String>,
    pub implementation_details: Option<ImplementationDetails>,
}

impl FeatureProposal {
    /// Create a new feature proposal
    pub fn new(title: String, description: String, source: Source, priority: Priority) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            source,
            priority,
            status: Status::Proposed,
            proposed_at: now,
            updated_at: now,
            target_modules: Vec::new(),
            expected_impact: String::new(),
            risk_assessment: RiskAssessment::default(),
            approval_chain: Vec::new(),
            implementation_details: None,
        }
    }
}

/// Risk assessment for a feature proposal
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskAssessment {
    pub level: RiskLevel,
    pub potential_impacts: Vec<String>,
    pub mitigation_strategies: Vec<String>,
    pub rollback_plan: Option<String>,
}

/// Risk levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Minimal,
    Low,
    Medium,
    High,
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self {
        RiskLevel::Low
    }
}

/// Implementation details for approved proposals
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImplementationDetails {
    pub code_changes: Vec<CodeChange>,
    pub test_plan: TestPlan,
    pub deployment_strategy: DeploymentStrategy,
    pub estimated_effort_hours: f32,
}

/// Code change specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeChange {
    pub file_path: String,
    pub change_type: ChangeType,
    pub description: String,
    pub code_snippet: Option<String>,
}

/// Type of code change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Add,
    Modify,
    Delete,
    Refactor,
}

/// Test plan for implementation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestPlan {
    pub unit_tests: Vec<String>,
    pub integration_tests: Vec<String>,
    pub performance_benchmarks: Vec<String>,
    pub acceptance_criteria: Vec<String>,
}

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeploymentStrategy {
    pub deployment_type: DeploymentType,
    pub rollback_enabled: bool,
    pub canary_percentage: Option<f32>,
    pub requires_approval: bool,
}

/// Type of deployment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DeploymentType {
    #[default]
    Immediate,
    Gradual,
    Canary,
    BlueGreen,
}