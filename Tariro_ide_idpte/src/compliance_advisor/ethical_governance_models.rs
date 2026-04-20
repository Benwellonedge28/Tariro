// Tariro_ide_idpte/src/compliance_advisor/ethical_governance_models.rs

//! Defines data models for Tariro's Dynamic Ethical Governance & Policy Adaptation module,
//! including ethical policies, societal norms, and emergent risks.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A formal rule or guideline governing Tariro's behavior and decision-making.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalPolicy {
    pub id: String,
    pub policy_name: String,
    pub description: String,
    pub domain: String, // e.g., "data_privacy", "resource_allocation", "human_interaction"
    pub version: String,
    pub approval_status: VerificationStatus, // Indicates if human overseers approved
    pub formal_verification_result: VerificationStatus, // Result of formal verification
    pub last_updated: DateTime<Utc>,
}

/// A learned or observed human ethical standard or expectation regarding AI behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocietalNorm {
    pub id: String,
    pub norm_description: String,
    pub source_of_norm: String, // e.g., "public_discourse", "legal_precedent", "cultural_consensus"
    pub observed_adherence: f64, // 0.0 to 1.0, how consistently humans follow it
    pub ethical_relevance_score: f64, // 0.0 to 1.0, how critical this norm is for Tariro
    pub detected_at: DateTime<Utc>,
}

/// A newly identified ethical or safety concern that requires policy adaptation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentRisk {
    pub id: String,
    pub risk_description: String,
    pub potential_impact: String,
    pub probability: f64, // 0.0 to 1.0
    pub ethical_urgency_score: f64, // 0.0 to 1.0
    pub suggested_mitigation: Vec<String>,
    pub detected_at: DateTime<Utc>,
}

/// Audit report for dynamic ethical governance and policy adaptation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_policies_adapted: u64,
    pub total_risks_mitigated: u64,
    pub detected_policy_gaps: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
