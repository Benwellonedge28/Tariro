// Tariro_ide_idpte/src/environmental/multi_agent_models.rs

//! Defines data models for Tariro's Multi-Agent Coordination & Collaboration module,
//! including agent profiles, coordination strategies, and joint actions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Metadata and capabilities of an interacting agent (AGI, human, biological, robotic).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProfile {
    pub id: String,
    pub agent_name: String,
    pub agent_type: String, // e.g., "AGI", "Human", "Biological", "Robotic"
    pub capabilities: Vec<String>,
    pub ethical_framework_summary: String,
    pub communication_protocol_supported: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of agent profile integration
    pub security_verified: VerificationStatus, // New: Security verification of agent authentication
    pub last_updated: DateTime<Utc>,
}

/// Defines rules and parameters for multi-agent interaction and collaboration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationStrategy {
    pub id: String,
    pub strategy_name: String,
    pub objective_summary: String,
    pub rules_of_engagement: Vec<String>, // e.g., "prioritize_human_safety", "maximize_resource_efficiency"
    pub conflict_resolution_mechanisms: Vec<String>,
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the strategy design
    pub security_audited: VerificationStatus, // New: Security audit of the strategy implementation
    pub last_updated: DateTime<Utc>,
}

/// A collaborative action performed by multiple agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointAction {
    pub id: String,
    pub strategy_id: String,
    pub participating_agent_ids: Vec<String>,
    pub action_summary: String,
    pub outcome_summary: String,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the joint action
    pub security_integrity: VerificationStatus, // New: Security integrity of the joint action
    pub executed_at: DateTime<Utc>,
}

/// A report summarizing the audit of multi-agent interactions and coordination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAgentAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_agent_interactions", "specific_collaboration_project"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_interactions: u64,
    pub detected_conflicts: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of multi-agent system
    pub security_integrity_status: VerificationStatus, // Overall security integrity of multi-agent system
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
