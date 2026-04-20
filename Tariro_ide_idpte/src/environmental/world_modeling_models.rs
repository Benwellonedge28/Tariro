// Tariro_ide_idpte/src/environmental/world_modeling_models.rs

//! Defines data models for Tariro's Collaborative World Modeling module,
//! including shared world models, updates, and consensus mechanisms.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A comprehensive and shared representation of the operational environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedWorldModel {
    pub id: String,
    pub model_version: u64,
    pub consensus_percentage: f64, // 0.0 to 1.0, how much agents agree on this model
    pub core_elements_summary: String, // e.g., "objects: [tree, car], agents: [human_1, Tariro_AGI]"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the world model's fairness
    pub security_integrity: VerificationStatus, // New: Security verification of model data integrity
    pub last_updated: DateTime<Utc>,
}

/// A proposed change or addition to the SharedWorldModel from an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelUpdate {
    pub id: String,
    pub model_id: String, // Link to SharedWorldModel
    pub proposing_agent_id: String,
    pub update_description: String,
    pub confidence: f64, // Confidence of the proposing agent in the update
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of the proposed update
    pub security_validity: VerificationStatus, // New: Security validity of the update source
    pub proposed_at: DateTime<Utc>,
}

/// Rules and procedures for reconciling differing world views and reaching consensus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusMechanism {
    pub id: String,
    pub mechanism_name: String,
    pub description: String,
    pub voting_rules: Vec<String>, // e.g., "majority_vote", "weighted_trust"
    pub conflict_resolution_methods: Vec<String>,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the consensus mechanism
    pub security_audited: VerificationStatus, // New: Security audit of mechanism implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of collaborative world modeling processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelingAuditReport {
    pub id: String,
    pub model_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_updates_processed: u64,
    pub detected_discrepancies: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of world modeling
    pub security_integrity_status: VerificationStatus, // Overall security integrity of world modeling systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
