// Tariro_ide_idpte/src/cognitive_arch/metaphorical_mapping_models.rs

//! Defines data models for Tariro's Cross-Domain Metaphorical Mapping module,
//! including metaphorical bridges, conceptual mappings, and insight transfer reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A metaphorical bridge connecting concepts from two different domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphoricalBridge {
    pub id: String,
    pub source_domain: String,
    pub target_domain: String,
    pub source_concept: String,
    pub target_concept: String,
    pub metaphorical_relation: String, // e.g., "is_structured_like", "flows_like"
    pub mapping_strength: f64, // 0.0 to 1.0
    pub ethical_connotation: Vec<String>, // Potential ethical biases in the metaphor
    pub created_at: DateTime<Utc>,
}

/// A proposal for transferring an insight from a source domain to a target domain via metaphor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightTransferProposal {
    pub id: String,
    pub bridge_id: String,
    pub source_insight: String,
    pub proposed_target_application: String,
    pub expected_benefit: String,
    pub ethical_review_status: VerificationStatus,
    pub security_verified: VerificationStatus,
    pub proposed_at: DateTime<Utc>,
}

/// A report detailing the results of applying a metaphorical insight in a new domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphoricalInsightReport {
    pub id: String,
    pub proposal_id: String,
    pub actual_outcome: String,
    pub transfer_accuracy: f64,
    pub novel_discoveries: Vec<String>,
    pub ethical_impact_assessment: VerificationStatus,
    pub generated_at: DateTime<Utc>,
}

/// Audit report for metaphorical mapping and insight transfer activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaphorAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_bridges_found: u64,
    pub successful_transfers: u64,
    pub detected_logical_fallacies: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
