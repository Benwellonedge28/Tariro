// Tariro_ide_idpte/src/environmental/decentralized_governance_models.rs

//! Defines data models for Tariro's Decentralized Governance & Verifiable Consensus module,
//! including governance proposals, consensus mechanisms, and decision records.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A proposal for a change in OMEGA OS's governance rules, operational parameters, or ethical guidelines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposed_changes: Vec<String>,
    pub proposer_id: String, // e.g., "Human_Admin_Samuel", "SelfEvolutionModule"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the proposal
    pub security_vulnerability_assessment: VerificationStatus, // New: Security assessment of proposed changes
    pub created_at: DateTime<Utc>,
}

/// A specification of a consensus mechanism used for decentralized decision-making.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMechanism {
    pub id: String,
    pub name: String,
    pub algorithm_type: String, // e.g., "proof_of_stake", "federated_byzantine_agreement", "multi_party_computation"
    pub description: String,
    pub robustness_score: f64, // 0.0 to 1.0
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the consensus mechanism
    pub cryptographic_integrity: VerificationStatus, // New: Cryptographic integrity audit
    pub last_updated: DateTime<Utc>,
}

/// A record of a decision made through a decentralized governance process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub id: String,
    pub proposal_id: String, // Link to GovernanceProposal
    pub consensus_mechanism_id: String, // Link to ConsensusMechanism
    pub outcome: String, // e.g., "approved", "rejected", "modified"
    pub votes_tallied: HashMap<String, u64>, // Participant ID -> Votes
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the decision outcome
    pub security_audited: VerificationStatus, // New: Security audit of the decision process
    pub recorded_at: DateTime<Utc>,
}

/// A report summarizing the audit of decentralized governance and verifiable consensus processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_governance_decisions", "critical_system_parameter_changes"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_proposals: u64,
    pub approved_decisions: u64,
    pub detected_disputes: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of governance
    pub security_integrity_status: VerificationStatus, // Overall security integrity of governance systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
