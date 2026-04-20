// Tariro_ide_idpte/src/co_creation/collaborative_creativity_models.rs

//! Defines data models for Tariro's Advanced Human-AGI Collaborative Creativity module,
//! including co-creative projects, human-AGI contributions, and creative breakthroughs.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a shared creative project between human and AGI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoCreativeProject {
    pub id: String,
    pub project_name: String,
    pub human_collaborator_id: String,
    pub domain: String, // e.g., "scientific_discovery", "artistic_creation", "software_development"
    pub shared_goals: Vec<String>,
    pub ethical_charter_hash: String, // Hash of the agreed-upon ethical guidelines for the project
    pub security_verified: VerificationStatus, // New: Security verification of project environment
    pub created_at: DateTime<Utc>,
}

/// A record of a specific contribution by either human or AGI to a co-creative project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanAGIContribution {
    pub id: String,
    pub project_id: String,
    pub contributor_id: String, // Human ID or AGI module ID
    pub contribution_type: String, // e.g., "ideation", "data_analysis", "code_generation", "art_synthesis"
    pub description: String,
    pub intellectual_property_claim: Option<String>, // Hash of IP claim for this contribution
    pub ethical_compliance_status: VerificationStatus, // New: Ethical compliance of the contribution
    pub security_integrity_status: VerificationStatus, // New: Security integrity of the contribution
    pub timestamp: DateTime<Utc>,
}

/// A significant outcome or discovery resulting from a co-creative effort.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeBreakthrough {
    pub id: String,
    pub project_id: String,
    pub breakthrough_summary: String,
    pub novelty_score: f64, // 0.0 to 1.0
    pub utility_score: f64, // 0.0 to 1.0
    pub attributed_to: Vec<String>, // List of human/AGI contributors
    pub ethical_attribution_verified: VerificationStatus, // New: Ethical verification of attribution
    pub security_patent_protected: VerificationStatus, // New: Security verification of patent/IP protection
    pub discovered_at: DateTime<Utc>,
}

/// A report summarizing the audit of collaborative creativity processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeAuditReport {
    pub id: String,
    pub project_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_contributions: u64,
    pub ethical_disputes_resolved: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of co-creative process
    pub security_integrity_status: VerificationStatus, // Overall security integrity of co-creative systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
