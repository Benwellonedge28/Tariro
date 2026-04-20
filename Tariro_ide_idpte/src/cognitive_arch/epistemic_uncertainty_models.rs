// Tariro_ide_idpte/src/cognitive_arch/epistemic_uncertainty_models.rs

//! Defines data models for Tariro's Epistemic Uncertainty Quantification module,
//! including uncertainty metrics, knowledge gaps, and confidence intervals.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A metric quantifying different types of uncertainty in a given piece of knowledge or inference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyMetric {
    pub id: String,
    pub context: String, // The knowledge or inference being assessed
    pub aleatoric_uncertainty: f64, // Uncertainty due to inherent randomness (0.0 to 1.0)
    pub epistemic_uncertainty: f64, // Uncertainty due to lack of knowledge (0.0 to 1.0)
    pub total_uncertainty: f64, // Combined uncertainty
    pub ethical_implications: Vec<String>, // Ethical considerations of acting under this uncertainty
    pub security_risk_level: VerificationStatus, // Security risk associated with high uncertainty
    pub evaluated_at: DateTime<Utc>,
}

/// Identifies specific areas or questions where Tariro's knowledge is insufficient or conflicting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGap {
    pub id: String,
    pub identified_context: String,
    pub missing_information_type: String, // e.g., "missing_data", "conflicting_theories", "unexplored_variables"
    pub urgency_score: f64, // How critical it is to fill this gap (0.0 to 1.0)
    pub ethical_priority: VerificationStatus, // Ethical priority to address this knowledge gap
    pub security_implications: VerificationStatus, // Security implications of the knowledge gap
    pub discovered_at: DateTime<Utc>,
}

/// A statistical interval indicating the reliability of an estimate or prediction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceInterval {
    pub id: String,
    pub metric_id: String, // Link to UncertaintyMetric or other metric
    pub estimated_value: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence_level: f64, // e.g., 0.95 for 95% confidence
    pub ethical_interpretation_guidance: Vec<String>, // Guidance on interpreting confidence ethically
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of epistemic uncertainty quantification processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_knowledge_domains", "critical_decision_points"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_assessments_made: u64,
    pub detected_undercertainty_events: u64,
    pub detected_overconfidence_events: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
