// Tariro_ide_idpte/src/compliance_advisor/governance_models.rs

//! Defines data models for Tariro's Ethical Governance & Oversight Mechanisms module,
//! including oversight requests, decision reviews, and intervention records.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a request for human oversight or review of a critical AGI decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversightRequest {
    pub id: String,
    pub decision_id: String, // ID of the AGI decision requiring oversight
    pub requesting_module_id: String, // Which module triggered the request
    pub risk_level: f64, // 0.0 to 1.0 (severity of potential impact)
    pub urgency: f64, // 0.0 to 1.0
    pub summary_of_decision: String,
    pub ethical_assessment_by_aleca: VerificationStatus, // ALEC-A's initial assessment
    pub security_implications: VerificationStatus, // Security assessment of request initiation
    pub created_at: DateTime<Utc>,
}

/// Records human feedback or approval on an AGI decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionReview {
    pub id: String,
    pub oversight_request_id: String,
    pub reviewer_id: String, // ID of the human reviewer
    pub review_outcome: String, // e.g., "approved", "rejected", "modified", "deferred"
    pub reviewer_comments: String,
    pub timestamp: DateTime<Utc>,
    pub ethical_compliance_status: VerificationStatus, // New: Ethical review of human review process
    pub security_verified: VerificationStatus, // New: Security verification of review data integrity
}

/// Records a human intervention or override of an AGI's autonomous action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionRecord {
    pub id: String,
    pub intervened_decision_id: String, // ID of the AGI decision that was overridden
    pub intervener_id: String, // ID of the human or authorized system component that intervened
    pub intervention_type: String, // e.g., "override", "pause_operation", "policy_update"
    pub reason_for_intervention: String,
    pub ethical_justification_by_human: String,
    pub timestamp: DateTime<Utc>,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical assessment of the intervention's impact
    pub security_integrity: VerificationStatus, // New: Security integrity of the intervention mechanism
}

/// A report summarizing the audit of governance processes and interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditReport {
    pub id: String,
    pub audit_scope: String, // e.g., "all_oversight_requests", "critical_interventions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_oversight_requests: u64,
    pub total_interventions: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of governance framework
    pub security_integrity_status: VerificationStatus, // Overall security integrity of governance data
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
