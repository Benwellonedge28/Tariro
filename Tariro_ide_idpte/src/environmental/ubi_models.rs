// Tariro_ide_idpte/src/environmental/ubi_models.rs

//! Defines data models for Tariro's Universal Basic Income & Resource Management module,
//! including UBI models, resource distribution policies, and economic impact assessments.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A conceptual framework for a Universal Basic Income model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UBIModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub payment_amount_per_person: f64,
    pub funding_mechanism: String, // e.g., "taxation", "resource_dividends", "agi_productivity_tax"
    pub eligibility_criteria: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the UBI model design
    pub security_verified: VerificationStatus, // New: Security verification of model parameters
    pub created_at: DateTime<Utc>,
}

/// A policy governing the equitable distribution of resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDistributionPolicy {
    pub id: String,
    pub policy_name: String,
    pub target_resource_type: String, // e.g., "financial_capital", "energy", "education_access"
    pub distribution_algorithm: String, // e.g., "per_capita", "needs_based", "performance_incentive"
    pub ethical_impact_prediction: VerificationStatus, // New: Predicted ethical impact of the policy
    pub security_integrity: VerificationStatus, // New: Security integrity of the policy
    pub last_updated: DateTime<Utc>,
}

/// An assessment of the economic and social effects of a UBI model or policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicImpactAssessment {
    pub id: String,
    pub ubi_model_id: String,
    pub assessment_summary: String,
    pub projected_gdp_change: f64, // e.g., +5.0% or -2.0%
    pub projected_poverty_reduction: f64, // e.g., 20.0% reduction
    pub detected_unintended_consequences: Vec<String>,
    pub ethical_bias_assessed: VerificationStatus, // New: Assessment for inherent biases in the assessment methodology
    pub security_integrity: VerificationStatus, // New: Security integrity of the assessment data
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of Universal Basic Income or resource management mechanisms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UBIAuditReport {
    pub id: String,
    pub model_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub funds_distributed: f64,
    pub total_recipients: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of UBI implementation
    pub security_integrity_status: VerificationStatus, // Overall security integrity of UBI systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
