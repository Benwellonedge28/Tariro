// Tariro_ide_idpte/src/security/risk_management_models.rs

//! Defines data models for Tariro's Dynamic Risk Assessment & Mitigation module,
//! including risk assessments, mitigation plans, and operational domains.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// An identified area of operation with specific risk profiles.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationalDomain {
    Cyber,
    Physical,
    Social,
    Ethical,
    Economic,
    InternalIntegrity,
}

/// A real-time, context-aware analysis of potential threats and their impacts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub id: String,
    pub domain: OperationalDomain,
    pub context_description: String,
    pub threat_levels: HashMap<String, f64>, // Threat type -> Probability/Impact score (0.0 to 1.0)
    pub overall_risk_score: f64, // 0.0 to 1.0
    pub ethical_risk_evaluation: Vec<String>, // Ethical aspects of the identified risks
    pub assessed_at: DateTime<Utc>,
}

/// A set of autonomously generated actions to reduce identified risks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationPlan {
    pub id: String,
    pub assessment_id: String, // Link to RiskAssessment
    pub proposed_actions: Vec<String>,
    pub expected_risk_reduction: f64, // 0.0 to 1.0
    pub ethical_compliance_status: VerificationStatus, // Ethical review of the plan
    pub security_verified: VerificationStatus, // Security verification of the mitigation steps
    pub created_at: DateTime<Utc>,
}

/// A record of an executed risk mitigation action and its observed result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationExecution {
    pub id: String,
    pub plan_id: String,
    pub action_taken: String,
    pub observed_impact: String,
    pub effectiveness_score: f64, // 0.0 to 1.0
    pub security_audit_status: VerificationStatus,
    pub executed_at: DateTime<Utc>,
}

/// Audit report for dynamic risk assessment and mitigation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_cyber_risks", "critical_operation_assessments"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_assessments_performed: u64,
    pub total_mitigations_executed: u64,
    pub detected_risk_escalations: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
