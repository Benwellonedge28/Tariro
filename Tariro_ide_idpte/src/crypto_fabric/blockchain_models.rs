// Tariro_ide_idpte/src/crypto_fabric/blockchain_models.rs

//! Defines data models for Tariro's Decentralized Consensus & Blockchain Integration module,
//! including blockchain records, consensus mechanisms, and decentralized decisions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a verifiable transaction or event recorded on an immutable blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainRecord {
    pub id: String,
    pub event_type: String, // e.g., "AGIDecision", "StateChange", "ResourceAllocation"
    pub entity_id: String, // ID of the entity affected or responsible
    pub data_hash: String, // Cryptographic hash of the event data
    pub previous_record_hash: String, // Hash of the previous record in the chain
    pub timestamp: DateTime<Utc>,
    pub ethical_compliance_status: VerificationStatus, // New: Ethical review of the recorded event
    pub integrity_verified: VerificationStatus, // New: Integrity of the record verified by QR-CTFO
}

/// Describes a consensus mechanism used for decentralized governance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMechanism {
    pub id: String,
    pub mechanism_type: String, // e.g., "Proof-of-Stake", "Delegated Proof-of-Stake", "Multi-signature"
    pub participants: Vec<String>, // IDs of entities involved in consensus
    pub voting_rules: String,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the consensus mechanism design
    pub security_audited: VerificationStatus, // New: Security audit of the consensus mechanism
    pub last_updated: DateTime<Utc>,
}

/// Represents a decision reached through a decentralized consensus process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecentralizedDecision {
    pub id: String,
    pub topic: String, // What the decision is about
    pub proposal_hash: String, // Hash of the proposal document/code
    pub consensus_mechanism_id: String,
    pub outcome: String, // e.g., "approved", "rejected", "deferred"
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the decision outcome
    pub security_integrity: VerificationStatus, // New: Security integrity of the decision process
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of blockchain operations and decentralized governance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_agi_decisions", "resource_allocations"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_records_audited: u64,
    pub detected_anomalies: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of blockchain usage
    pub security_integrity_status: VerificationStatus, // Overall security integrity of blockchain systems
    pub generated_at: DateTime<Utc>,
}
