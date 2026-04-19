// Tariro_ide_idpte/src/cognitive_arch/epistemic_models.rs

//! Defines data models for Tariro's Epistemic Reasoning & Knowledge Acquisition module,
//! including knowledge claims, belief states, and information requests.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a piece of asserted knowledge or data with metadata about its provenance and reliability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeClaim {
    pub id: String,
    pub content_hash: String, // Hash of the actual knowledge content
    pub source_id: String, // e.g., URL, module ID, Sensor ID
    pub assertion_context: String, // Where/how the knowledge was asserted
    pub asserted_by: String, // Which agent or system asserted this
    pub initial_reliability_score: f64, // 0.0 to 1.0, from source assessment
    pub ethical_provenance: VerificationStatus, // New: Ethical review of knowledge source and acquisition
    pub security_verified: VerificationStatus, // New: Security of knowledge claim integrity
    pub timestamp: DateTime<Utc>,
}

/// Represents Tariro's current belief state regarding a specific knowledge claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefState {
    pub id: String,
    pub knowledge_claim_id: String,
    pub current_confidence: f64, // 0.0 (disbelief) to 1.0 (absolute belief)
    pub supporting_evidence: Vec<String>, // IDs of other KnowledgeClaims or observations
    pub conflicting_evidence: Vec<String>, // IDs of conflicting KnowledgeClaims
    pub uncertainty_factors: HashMap<String, String>, // e.g., "source_bias": "high"
    pub ethical_bias_assessed: VerificationStatus, // New: Assessment for cognitive biases in belief formation
    pub security_integrity: VerificationStatus, // New: Integrity of belief state data
    pub last_updated: DateTime<Utc>,
}

/// Represents a request for new information or verification of existing knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationRequest {
    pub id: String,
    pub requesting_module_id: String,
    pub query: String, // The question or information sought
    pub reason_for_request: String, // Why this information is needed (e.g., "resolve_contradiction", "fill_knowledge_gap")
    pub urgency: f64, // 0.0 to 1.0
    pub expected_data_characteristics: HashMap<String, String>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the information request
    pub security_implications: VerificationStatus, // New: Security implications of fulfilling the request
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing knowledge acquisition efforts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeAcquisitionReport {
    pub id: String,
    pub request_id: String,
    pub acquired_knowledge_claims: Vec<String>, // IDs of new KnowledgeClaims
    pub resolved_belief_states: Vec<String>, // IDs of updated BeliefStates
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of acquisition process
    pub security_integrity_status: VerificationStatus, // Overall security integrity of acquisition process
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
