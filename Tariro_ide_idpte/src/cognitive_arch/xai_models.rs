// Tariro_ide_idpte/src/cognitive_arch/xai_models.rs

//! Defines data models for Tariro's Explainable AI (XAI) module,
//! including explanation requests, generated explanations, and XAI compliance reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A request from a human or internal system for an explanation of an AGI decision or behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationRequest {
    pub id: String,
    pub decision_id: String, // ID of the decision or action to be explained
    pub context_summary: String, // Brief summary of the context of the decision
    pub requested_by_entity: String, // e.g., "Human_Admin", "ALEC-A_Auditor"
    pub requested_at: DateTime<Utc>,
}

/// A clear, human-understandable explanation for an AGI decision or internal reasoning process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub id: String,
    pub request_id: String,
    pub explained_decision_id: String,
    pub explanation_text: String,
    pub explanation_type: String, // e.g., "feature_importance", "counterfactual", "rule_based"
    pub fidelity_score: f64, // 0.0 to 1.0, how accurately it reflects internal workings
    pub ethical_transparency_rating: VerificationStatus, // New: Ethical review of explanation honesty
    pub security_integrity: VerificationStatus, // New: Security verification of explanation tampering
    pub generated_at: DateTime<Utc>,
}

/// Metrics used to evaluate the quality and effectiveness of explanations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplainabilityMetric {
    pub id: String,
    pub metric_name: String, // e.g., "fidelity", "comprehensibility", "actionability"
    pub value: f64,
    pub explanation_id: String,
    pub evaluated_at: DateTime<Utc>,
}

/// A report summarizing the compliance and ethical standing of XAI outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XAIComplianceReport {
    pub id: String,
    pub scope: String, // e.g., "all_explanations", "critical_decisions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_explanations_generated: u64,
    pub misleading_explanations_detected: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of XAI outputs
    pub security_integrity_status: VerificationStatus, // Overall security integrity of XAI systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
