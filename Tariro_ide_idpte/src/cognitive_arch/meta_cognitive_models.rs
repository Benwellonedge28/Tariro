// Tariro_ide_idpte/src/cognitive_arch/meta_cognitive_models.rs

//! Defines data models for Tariro's Meta-Cognitive Self-Regulation module,
//! including cognitive process audits, self-regulation strategies, and efficiency metrics.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A record of Tariro autonomously monitoring its own internal thinking or decision process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveProcessAudit {
    pub id: String,
    pub process_type: String, // e.g., "causal_discovery", "long_term_planning"
    pub audit_summary: String,
    pub detected_bottlenecks: Vec<String>,
    pub detected_errors_or_fallacies: Vec<String>,
    pub monitored_at: DateTime<Utc>,
}

/// A strategy developed by Tariro to autonomously adjust its own cognitive parameters or workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfRegulationStrategy {
    pub id: String,
    pub audit_id: String, // Link to CognitiveProcessAudit
    pub proposed_adjustments: HashMap<String, String>, // e.g., "learning_rate": "reduced", "search_depth": "increased"
    pub expected_performance_gain: f64, // 0.0 to 1.0
    pub ethical_review_status: VerificationStatus, // Ethical review of the self-adjustment
    pub security_verified: VerificationStatus, // Security verification of the self-modification
    pub strategy_generated_at: DateTime<Utc>,
}

/// A quantifiable measure of Tariro's cognitive efficiency and performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyMetric {
    pub id: String,
    pub context: String,
    pub throughput_score: f64,
    pub accuracy_score: f64,
    pub resource_utilization_ratio: f64,
    pub ethical_impact_score: f64, // Impact of efficiency on ethical alignment
    pub evaluated_at: DateTime<Utc>,
}

/// A report summarizing the audit of meta-cognitive self-regulation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_self_adjustments", "critical_system_parameter_tuning"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_audits_performed: u64,
    pub total_adjustments_applied: u64,
    pub detected_instability_events: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
