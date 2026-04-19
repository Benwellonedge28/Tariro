// Tariro_ide_idpte/src/resource_management/dynamic_reallocation_models.rs

//! Defines data models for Tariro's Dynamic Resource Reallocation module,
//! including reallocation requests, resource utilization snapshots, and reallocation strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a request from a module for dynamic adjustment of resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReallocationRequest {
    pub id: String,
    pub requesting_module_id: String,
    pub resource_type: String, // e.g., "CPU", "RAM", "bandwidth"
    pub requested_change: f64, // Positive for increase, negative for decrease
    pub reason: String, // Justification for the request
    pub urgency: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the request's implications
    pub security_verified: VerificationStatus, // New: Security of request integrity and authenticity
    pub timestamp: DateTime<Utc>,
}

/// A snapshot of current resource utilization data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilizationSnapshot {
    pub id: String,
    pub resource_type: String,
    pub total_available: f64,
    pub current_allocation: HashMap<String, f64>, // Module ID -> Allocated amount
    pub current_demand: HashMap<String, f64>, // Module ID -> Demanded amount
    pub timestamp: DateTime<Utc>,
    pub ethical_reporting_status: VerificationStatus, // New: Ethical review of snapshot reporting
    pub security_integrity: VerificationStatus, // New: Security integrity of snapshot data
}

/// A formulated strategy for reallocating resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReallocationStrategy {
    pub id: String,
    pub request_id: String, // ID of the originating ReallocationRequest
    pub proposed_changes: HashMap<String, f64>, // Module ID -> Change in allocation
    pub expected_system_impact: String, // e.g., "improved_performance", "stabilized_resource"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the reallocation strategy
    pub security_review_status: VerificationStatus, // New: Security review of the reallocation strategy
    pub approved_by: String, // e.g., "Tariro_DRRM", "ALEC-A", "HumanAdministrator"
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of a resource reallocation process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReallocationAudit {
    pub id: String,
    pub strategy_id: String,
    pub executed_changes: HashMap<String, f64>, // Actual changes implemented
    pub effectiveness_assessment: String,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of reallocation
    pub security_integrity_status: VerificationStatus, // Overall security integrity of reallocation
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
