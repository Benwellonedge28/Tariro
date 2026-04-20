// Tariro_ide_idpte/src/communication_bridge/human_agi_teaming_models.rs

//! Defines data models for Tariro's Human-AGI Teaming & Trust Building module,
//! including team formation requests, trust metrics, and collaboration protocols.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A request to form or optimize a human-AGI team.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamFormationRequest {
    pub id: String,
    pub project_id: String,
    pub required_skills: Vec<String>,
    pub human_members: Vec<String>,
    pub agi_members: Vec<String>, // List of AGI module IDs or instances
    pub ethical_review_status: VerificationStatus, // New: Ethical review of team composition
    pub security_verified: VerificationStatus, // New: Security verification of team access/data
    pub requested_at: DateTime<Utc>,
}

/// A metric used to quantify the level of trust between human and AGI team members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustMetric {
    pub id: String,
    pub team_id: String,
    pub trustor_id: String, // ID of the entity assessing trust
    pub trustee_id: String, // ID of the entity being assessed
    pub trust_score: f64, // 0.0 to 1.0
    pub factors_influencing_trust: Vec<String>,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical implications of trust scores
    pub security_risk_level: VerificationStatus, // New: Security risk associated with trust levels
    pub evaluated_at: DateTime<Utc>,
}

/// Guidelines and established practices for effective human-AGI collaboration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationProtocol {
    pub id: String,
    pub protocol_name: String,
    pub description: String,
    pub rules_of_engagement: Vec<String>, // e.g., "transparent_communication", "mutual_respect"
    pub communication_channels: Vec<String>,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the protocol design
    pub security_audited: VerificationStatus, // New: Security audit of the protocol implementation
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the audit of human-AGI teaming and trust-building processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamingAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_teams", "critical_missions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_teams_formed: u64,
    pub average_trust_score: f64,
    pub detected_trust_breaches: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of teaming
    pub security_integrity_status: VerificationStatus, // Overall security integrity of teaming systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
