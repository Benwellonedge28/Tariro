// Tariro_ide_idpte/src/environmental/decentralized_governance_models.rs

//! Defines data models for Tariro's Decentralized Governance & Verifiable Consensus module,
//! including vote proposals, consensus mechanisms, and governance logs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A proposal for a decision within a decentralized governance structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposer_id: String, // ID of the entity proposing the vote
    pub options: Vec<String>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the proposal's content
    pub security_implications_assessed: VerificationStatus, // New: Security assessment of proposal execution
    pub proposed_at: DateTime<Utc>,
}

/// A mechanism used to reach verifiable agreement among participants in a decentralized network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMechanism {
    pub id: String,
    pub name: String, // e.g., "Proof-of-Stake", "Delegated-Proof-of-Stake", "Federated-Byzantine-Agreement"
    pub description: String,
    pub participants: Vec<String>, // List of participant IDs (human or AGI)
    pub transparency_score: f64, // 0.0 to 1.0
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the mechanism design
    pub security_audited: VerificationStatus, // New: Security audit of mechanism implementation
    pub implemented_at: DateTime<Utc>,
}

/// A log of governance events, including proposals, votes, and their outcomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceLog {
    pub id: String,
    pub event_type: String, // e.g., "proposal_submitted", "vote_cast", "decision_reached"
    pub event_details: String,
    pub associated_entity_id: String, // e.g., VoteProposal ID, ConsensusMechanism ID
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the event
    pub security_integrity: VerificationStatus, // New: Security integrity of the log entry
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of decentralized governance processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_governance_activities", "critical_decisions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_proposals_processed: u64,
    pub total_decisions_reached: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of governance
    pub security_integrity_status: VerificationStatus, // Overall security integrity of governance systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
