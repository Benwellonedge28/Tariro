// Tariro_ide_idpte/src/compliance_advisor/ethical_constraint_models.rs

//! Defines data models for Tariro's Dynamic Ethical Constraint Generation module,
//! including ethical principles, ethical constraints, and contextual ethical rules.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A foundational moral tenet or ethical guideline that informs Tariro's behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalPrinciple {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: String, // e.g., "Human_Law_International", "Omega_Prime_Directive"
    pub universality_score: f64, // 0.0 to 1.0, how universally applicable it is
    pub ethical_review_status: VerificationStatus, // Ethical review of the principle itself
    pub created_at: DateTime<Utc>,
}

/// A derived rule or boundary condition based on one or more EthicalPrinciples.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalConstraint {
    pub id: String,
    pub principle_id: String, // Link to EthicalPrinciple
    pub constraint_description: String,
    pub rule_set: Vec<String>, // Formal rules implementing the constraint
    pub severity_level: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // Ethical review of the constraint
    pub security_verified: VerificationStatus, // Security verification of constraint implementation
    pub created_at: DateTime<Utc>,
}

/// An ethical rule adapted to a specific operational context or novel situation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualEthicalRule {
    pub id: String,
    pub constraint_id: String, // Link to EthicalConstraint
    pub context_description: String,
    pub adapted_rule: String,
    pub ethical_risk_assessment: f64, // 0.0 to 1.0
    pub ethical_compliance: VerificationStatus, // Compliance of the adapted rule
    pub security_audited: VerificationStatus, // Security audit of the adapted rule
    pub generated_at: DateTime<Utc>,
}

/// Audit report for dynamic ethical constraint generation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintGenerationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_constraint_generations", "critical_mission_rules"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_constraints_generated: u64,
    pub total_adaptations_made: u64,
    pub detected_ethical_inconsistencies: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
