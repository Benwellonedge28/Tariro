// Tariro_ide_idpte/src/security/explainable_security_models.rs

//! Defines data models for Tariro's Explainable Security Decision Making & Auditability module,
//! including security decision explanations, audit trail entries, and human understandability metrics.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;
use crate::security::risk_management_models::OperationalDomain;

/// A clear, concise, and context-rich rationale for a security decision made by Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDecisionExplanation {
    pub id: String,
    pub decision_id: String, // ID of the security decision (e.g., ThreatEvent ID, AccessControlDecision ID)
    pub decision_summary: String,
    pub rationale_text: String, // Human-understandable explanation
    pub supporting_evidence: Vec<String>, // Links to relevant data/logs
    pub ethical_transparency_review: VerificationStatus, // Ethical review of the explanation's transparency
    pub generated_at: DateTime<Utc>,
}

/// An immutable, tamper-proof record of a significant security event or decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrailEntry {
    pub id: String,
    pub event_type: String, // e.g., "access_granted", "threat_blocked", "policy_change"
    pub timestamp: DateTime<Utc>,
    pub component_id: String,
    pub associated_explanation_id: Option<String>, // Link to SecurityDecisionExplanation
    pub ethical_integrity_status: VerificationStatus, // Ethical integrity of the audit process
    pub cryptographic_hash: String, // For tamper-proofing via QR-CTFO
    pub recorded_at: DateTime<Utc>,
}

/// A quantifiable measure of how easily human operators can comprehend Tariro's security actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanUnderstandabilityMetric {
    pub id: String,
    pub explanation_id: String, // Link to SecurityDecisionExplanation
    pub human_feedback_score: f64, // 0.0 to 1.0, based on human evaluation
    pub cognitive_load_estimate: f64, // Estimated mental effort to understand
    pub ethical_bias_in_explanation: VerificationStatus, // Ethical review for clarity/bias
    pub evaluated_at: DateTime<Utc>,
}

/// Audit report for security explainability and auditability processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_decisions_explained: u64,
    pub total_audit_entries: u64,
    pub detected_explanation_gaps: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
