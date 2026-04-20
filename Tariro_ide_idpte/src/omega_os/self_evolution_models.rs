// Tariro_ide_idpte/src/omega_os/self_evolution_models.rs

//! Defines data models for Tariro's Self-Evolution & Continuous Improvement module,
//! including performance analysis, evolution proposals, and evolution plans.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A report summarizing Tariro's self-analysis of its own performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub id: String,
    pub module_id: String, // ID of the module being analyzed
    pub metric_name: String,
    pub current_value: f64,
    pub desired_value: f64,
    pub gap_analysis: String,
    pub ethical_performance_review: VerificationStatus, // New: Ethical review of performance metrics
    pub security_vulnerability_assessment: VerificationStatus, // New: Security assessment of performance data
    pub generated_at: DateTime<Utc>,
}

/// A proposed change or enhancement for Tariro's self-improvement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionProposal {
    pub id: String,
    pub analysis_id: String, // Link to PerformanceAnalysis
    pub proposed_change_summary: String,
    pub estimated_impact: String,
    pub ethical_impact_forecast: VerificationStatus, // New: Ethical impact forecast of the proposal
    pub security_impact_assessment: VerificationStatus, // New: Security impact assessment of the proposal
    pub proposed_at: DateTime<Utc>,
}

/// A detailed plan for implementing an evolutionary change to Tariro's systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPlan {
    pub id: String,
    pub proposal_id: String,
    pub plan_steps: Vec<String>,
    pub expected_outcome: String,
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the evolution plan
    pub security_audited: VerificationStatus, // New: Security audit of the evolution plan implementation
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of self-evolution processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfEvolutionAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_self_modifications", "critical_evolution_paths"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_changes_implemented: u64,
    pub successful_evolutions: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of self-evolution
    pub security_integrity_status: VerificationStatus, // Overall security integrity of self-evolution systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
