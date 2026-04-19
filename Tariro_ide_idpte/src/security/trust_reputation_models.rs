// Tariro_ide_idpte/src/security/trust_reputation_models.rs

//! Defines data models for Tariro's Trust & Reputation System module,
//! including agent trust scores, reputation metrics, and trust events.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents Tariro's internal trust score for another agent (human or AI).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTrustScore {
    pub id: String,
    pub agent_id: String,
    pub score: f64, // 0.0 (untrustworthy) to 1.0 (fully trustworthy)
    pub factors: HashMap<String, f64>, // e.g., "competence": 0.9, "benevolence": 0.7, "integrity": 0.8
    pub last_interaction_summary: String,
    pub ethical_basis_reviewed: VerificationStatus, // New: Ethical basis of trust assessment
    pub security_verified: VerificationStatus, // New: Security of trust score calculation
    pub last_updated: DateTime<Utc>,
}

/// Represents a specific metric or factor contributing to an agent's reputation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationMetric {
    pub id: String,
    pub name: String,
    pub description: String,
    pub calculation_method: String,
    pub weight: f64, // How much this metric influences the overall score
    pub integrity_proof: String, // New: Proof of tamper-proof metric definition
    pub created_at: DateTime<Utc>,
}

/// An event that significantly impacts trust between Tariro and another agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvent {
    pub id: String,
    pub affected_agent_id: String,
    pub event_type: String, // e.g., "cooperation_success", "communication_failure", "deception_detected"
    pub description: String,
    pub impact_on_trust: f64, // Positive or negative change to trust score
    pub ethical_assessment: VerificationStatus, // New: Ethical assessment of the event
    pub security_implications: VerificationStatus, // New: Security implications of the event
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing trust relationships and potential risks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustReport {
    pub id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub analyzed_agents: Vec<String>, // IDs of agents analyzed
    pub overall_trust_landscape_summary: String,
    pub identified_trust_risks: Vec<String>,
    pub recommendations: Vec<String>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
