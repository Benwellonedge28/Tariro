// Tariro_ide_idpte/src/omega_os/code_models.rs

//! Defines data models for code modules, compilation results, and code deployments within OMEGA OS.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use std::collections::HashMap;

/// Represents a piece of generated or modified code within OMEGA OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeModule {
    pub id: String,
    pub name: String,
    pub version: String,
    pub source_code: String,
    pub target_service_id: Option<String>, // Which OMEGA OS service this code is for
    pub generated_by_agi_module: String, // e.g., "HACSE", "ALEC-A-Remediation"
    pub created_at: DateTime<Utc>,
    pub compilation_status: CompilationStatus,
    pub verification_status: VerificationStatus,
    pub ethical_review_status: EthicalReviewStatus,
    pub associated_meta_goals: Vec<String>,
    pub dependencies: Vec<String>, // Other code modules or libraries
}

/// Status of code compilation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationStatus {
    Pending,
    InProgress,
    Success,
    Failed(String), // Error message
}

/// Status of code verification (e.g., security, performance, integrity).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    InProgress,
    Passed,
    Failed(String), // Error message
    Skipped,
}

/// Status of ethical review by ALEC-A for a code module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EthicalReviewStatus {
    Pending,
    Approved,
    Rejected(String), // Reason for rejection
    Required,
    NotApplicable,
}

/// Represents a deployment of a code module to an OMEGA OS service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeDeployment {
    pub id: String,
    pub code_module_id: String,
    pub target_service_id: String,
    pub deployed_at: DateTime<Utc>,
    pub deployment_status: DeploymentStatus,
    pub deployed_by_agi_module: String, // e.g., "SelfModificationAgent", "AgiCoreEngine"
    pub rollback_info: Option<String>, // Instructions for rolling back
}

/// Status of a code deployment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Pending,
    InProgress,
    Success,
    Failed(String), // Error message
    RolledBack,
}
