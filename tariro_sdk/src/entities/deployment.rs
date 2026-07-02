//! Deployment record entity for tracking deployments

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Deployment environment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentEnvironment {
    Development,
    Staging,
    Production,
    Testing,
    Custom(String),
}

/// Deployment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentStatus {
    Pending,
    InProgress,
    Successful,
    Failed,
    RolledBack,
}

/// Deployment record tracking all deployments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRecord {
    pub id: String,
    pub project_id: String,
    pub version: String,
    pub environment: DeploymentEnvironment,
    pub status: DeploymentStatus,
    pub deployed_at: DateTime<Utc>,
    pub deployed_by: String,
    pub rollback_of: Option<String>,
    pub deployment_details: DeploymentDetails,
    pub health_check: HealthCheckResult,
}

impl DeploymentRecord {
    /// Create a new deployment record
    pub fn new(project_id: String, version: String, environment: DeploymentEnvironment) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            version,
            environment,
            status: DeploymentStatus::Pending,
            deployed_at: Utc::now(),
            deployed_by: "tariro".to_string(),
            rollback_of: None,
            deployment_details: DeploymentDetails::default(),
            health_check: HealthCheckResult::default(),
        }
    }

    /// Mark deployment as successful
    pub fn mark_success(&mut self) {
        self.status = DeploymentStatus::Successful;
    }

    /// Mark deployment as failed
    pub fn mark_failed(&mut self) {
        self.status = DeploymentStatus::Failed;
    }

    /// Mark deployment as rolled back
    pub fn mark_rollback(&mut self, original_id: String) {
        self.status = DeploymentStatus::RolledBack;
        self.rollback_of = Some(original_id);
    }
}

/// Details about the deployment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeploymentDetails {
    pub artifacts: Vec<Artifact>,
    pub configuration_hash: String,
    pub environment_variables: std::collections::HashMap<String, String>,
    pub resources_allocated: ResourceAllocation,
}

/// Deployment artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub artifact_type: ArtifactType,
    pub size_bytes: u64,
    pub checksum: String,
}

/// Type of artifact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArtifactType {
    Binary,
    Library,
    Container,
    Configuration,
    Script,
}

/// Resource allocation for deployment
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceAllocation {
    pub cpu_cores: f32,
    pub memory_mb: u64,
    pub storage_mb: u64,
    pub replicas: u32,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckResult {
    pub passed: bool,
    pub checks: Vec<HealthCheck>,
    pub overall_score: f32,
}

/// Individual health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub duration_ms: u64,
}