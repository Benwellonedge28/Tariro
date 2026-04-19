// Tariro_ide_idpte/src/omega_os/self_evolution_models.rs

//! Defines data models for Tariro's Autonomous System Upgrades & Evolution module,
//! including upgrade proposals, patch deployments, and evolutionary algorithm states.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a proposed change or improvement to Tariro's system (code, architecture, algorithms).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeProposal {
    pub id: String,
    pub target_component_id: String,
    pub description: String,
    pub proposed_changes_summary: String,
    pub estimated_impact: String, // e.g., "+10% efficiency", "fixed critical bug"
    pub justification: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the proposed upgrade
    pub security_vulnerability_scan: VerificationStatus, // New: Security scan result for the proposal
    pub generated_at: DateTime<Utc>,
}

/// A record of a deployed code patch or system change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchDeployment {
    pub id: String,
    pub upgrade_proposal_id: String,
    pub version_before: String,
    pub version_after: String,
    pub deployed_components: Vec<String>,
    pub deployment_status: String, // e.g., "success", "rollback", "failed"
    pub deployment_timestamp: DateTime<Utc>,
    pub ethical_compliance_after_deployment: VerificationStatus, // New: Ethical check after deployment
    pub security_integrity_after_deployment: VerificationStatus, // New: Security check after deployment
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
}

/// Represents the current state and parameters of Tariro's self-improvement evolutionary algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionaryAlgorithmState {
    pub id: String,
    pub algorithm_name: String,
    pub current_generation: u64,
    pub population_size: u32,
    pub fitness_function_metrics: HashMap<String, f64>,
    pub ethical_constraints_applied: VerificationStatus, // New: Ethical review of evolutionary parameters
    pub security_hardened: VerificationStatus, // New: Security hardening of the algorithm itself
    pub last_updated: DateTime<Utc>,
}

/// A report summarizing the ethical compliance of an autonomous upgrade process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalUpgradeReport {
    pub id: String,
    pub upgrade_id: String, // ID of the UpgradeProposal
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub overall_ethical_compliance: VerificationStatus,
    pub identified_risks: Vec<String>,
    pub recommendations_for_future: Vec<String>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
