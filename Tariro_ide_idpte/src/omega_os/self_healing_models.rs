// Tariro_ide_idpte/src/omega_os/self_healing_models.rs

//! Defines data models for Tariro's Self-Healing & Resilience Optimization module,
//! including failure reports, repair actions, and resilience metrics.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A report detailing a detected internal failure or system anomaly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureReport {
    pub id: String,
    pub failure_type: String, // e.g., "module_crash", "data_corruption", "performance_degradation"
    pub affected_component: String,
    pub severity: f64, // 0.0 to 1.0
    pub root_cause_analysis_summary: String,
    pub ethical_implications: Vec<String>, // New: Ethical harms from the failure
    pub security_vulnerabilities: Vec<String>, // New: Security vulnerabilities exposed by the failure
    pub detected_at: DateTime<Utc>,
}

/// A plan or action taken to repair a detected failure or enhance resilience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairAction {
    pub id: String,
    pub failure_report_id: String, // Link to FailureReport
    pub action_description: String,
    pub estimated_recovery_time_seconds: u64,
    pub success_rate: f64, // 0.0 to 1.0
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the repair action
    pub security_audited: VerificationStatus, // New: Security audit of the repair implementation
    pub executed_at: DateTime<Utc>,
}

/// Metrics used to quantify the system's resilience against failures and threats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceMetric {
    pub id: String,
    pub metric_name: String, // e.g., "mean_time_to_recovery", "attack_surface_score"
    pub value: f64,
    pub evaluated_component: String,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of resilience measures
    pub security_robustness_rating: VerificationStatus, // New: Security robustness of the component
    pub evaluated_at: DateTime<Utc>,
}

/// A report summarizing the audit of self-healing and resilience optimization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfHealingAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_self_healing_events", "critical_infrastructure_resilience"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_failures_handled: u64,
    pub successful_repairs: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of self-healing
    pub security_integrity_status: VerificationStatus, // Overall security integrity of self-healing systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
