// Tariro_ide_idpte/src/compliance_advisor/governance_models.rs

//! Defines data models for Tariro's Ethical Governance & Oversight Mechanisms module,
//! including oversight requests, human review decisions, and intervention records.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a request for human review or oversight of an AGI decision or operational aspect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OversightRequest {
    pub id: String,
    pub requesting_module_id: String,
    pub decision_context_summary: String, // Context of the decision requiring oversight
    pub proposed_action_summary: String, // What Tariro intends to do
    pub ethical_dilemma_id: Option<String>, // Link to EthicalDilemma if applicable
    pub urgency: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the request itself
    pub security_verified: VerificationStatus, // New: Security verification of request integrity
    pub created_at: DateTime<Utc>,
}

/// The outcome of a human review process for an OversightRequest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanReviewDecision {
    pub id: String,
    pub request_id: String,
    pub reviewer_id: String, // ID of the human reviewer or oversight committee
    pub decision_outcome: String, // e.g., "approved", "rejected", "modified", "deferred"
    pub rationale_summary: String,
    pub recommended_actions: Vec<String>,
    pub ethical_compliance_after_review: VerificationStatus, // New: Ethical compliance of the human decision
    pub security_integrity_after_review: VerificationStatus, // New: Security integrity of the decision record
    pub decision_timestamp: DateTime<Utc>,
}

/// A record of direct human intervention in AGI operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionRecord {
    pub id: String,
    pub intervening_human_id: String,
    pub intervention_type: String, // e.g., "pause_module", "override_action", "redirect_goal"
    pub target_agi_module_id: String,
    pub previous_state_hash: String, // Hash of AGI state before intervention
    pub intervention_rationale: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the human intervention
    pub security_verified: VerificationStatus, // New: Security verification of intervention integrity
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of human oversight and intervention mechanisms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_oversight_requests", "critical_interventions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_reviews: u64,
    pub total_interventions: u64,
    pub detected_governance_issues: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of governance processes
    pub security_integrity_status: VerificationStatus, // Overall security integrity of governance systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
