// Tariro_ide_idpte/src/cognitive_arch/advanced_epistemic_models.rs

//! Defines data models for Tariro's Advanced Epistemic Logic & Trust Reasoning module,
//! including epistemic states, trust metrics, and source reliability assessments.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A representation of Tariro's knowledge state about a specific proposition or domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistemicState {
    pub id: String,
    pub proposition: String, // The knowledge statement being evaluated
    pub belief_strength: f64, // 0.0 (disbelief) to 1.0 (certain belief)
    pub justification_summary: String,
    pub sources_considered: Vec<String>, // IDs of sources
    pub ethical_transparency_review: VerificationStatus, // Ethical review of its own knowledge justification
    pub security_integrity_status: VerificationStatus, // Security of the epistemic process
    pub evaluated_at: DateTime<Utc>,
}

/// A metric quantifying the trustworthiness of an information source or entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustMetric {
    pub id: String,
    pub entity_id: String, // ID of the source or entity
    pub trust_score: f64, // 0.0 (untrustworthy) to 1.0 (highly trustworthy)
    pub factors_influencing_trust: HashMap<String, f64>, // e.g., "historical_accuracy": 0.9, "bias_tendency": -0.2
    pub ethical_bias_detection: VerificationStatus, // Ethical review for bias in trust assessment
    pub last_updated: DateTime<Utc>,
}

/// An assessment of the reliability of a specific information source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReliability {
    pub id: String,
    pub source_id: String, // ID of the information source
    pub reliability_score: f64, // 0.0 (unreliable) to 1.0 (highly reliable)
    pub data_consistency_score: f64, // Consistency with other known facts
    pub ethical_flag: Vec<String>, // Any ethical concerns with the source
    pub assessed_at: DateTime<Utc>,
}

/// Audit report for advanced epistemic logic and trust reasoning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistemicAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_knowledge_assertions", "critical_trust_evaluations"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_propositions_evaluated: u64,
    pub total_trust_assessments: u64,
    pub detected_misinformation_events: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
