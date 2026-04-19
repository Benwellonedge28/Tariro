// Tariro_ide_idpte/src/resource_management/resource_optimization_models.rs

//! Defines data models for Tariro's Resource Allocation & Optimization module,
//! including resource demand predictions, allocation plans, and access requests.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a prediction of future resource demand.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDemandPrediction {
    pub id: String,
    pub resource_type: String, // e.g., "compute", "storage", "bandwidth", "energy"
    pub predicted_amount: f64,
    pub unit: String, // e.g., "teraflops", "petabytes", "gbps", "kilowatts"
    pub prediction_horizon: String, // e.g., "next_hour", "next_day", "next_week"
    pub confidence: f64, // 0.0 to 1.0
    pub influencing_factors: HashMap<String, String>, // e.g., "active_tasks": "high", "meta_goal": "advancement"
    pub ethical_implications_summary: Option<String>, // New: From ALEC-A review
    pub generated_at: DateTime<Utc>,
}

/// Represents a strategized plan for allocating resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocationPlan {
    pub id: String,
    pub plan_name: String,
    pub details: HashMap<String, f64>, // Component ID -> Allocated Amount
    pub resource_type: String,
    pub optimization_objective: String, // e.g., "maximize_efficiency", "minimize_cost", "maximize_fairness"
    pub ethical_review_status: VerificationStatus, // New: Ethical review status of the plan
    pub security_review_status: VerificationStatus, // New: Security review status of the plan
    pub approved_by: String, // e.g., "Tariro_SORAEE", "ALEC-A"
    pub created_at: DateTime<Utc>,
}

/// Represents a request for resource access or modification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAccessRequest {
    pub id: String,
    pub requesting_module_id: String,
    pub resource_id: String, // The specific resource or type requested
    pub access_type: String, // e.g., "read", "write", "allocate", "deallocate"
    pub justification: String,
    pub security_verified: VerificationStatus, // New: Security verification of the request
    pub ethical_verified: VerificationStatus, // New: Ethical verification of the request
    pub requested_at: DateTime<Utc>,
}

/// A report detailing the ethical implications of resource distribution decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceEthicalReport {
    pub id: String,
    pub allocation_plan_id: String,
    pub distribution_summary: String,
    pub fairness_score: f64, // 0.0 to 1.0
    pub identified_biases: Vec<String>,
    pub potential_conflicts: Vec<String>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}
