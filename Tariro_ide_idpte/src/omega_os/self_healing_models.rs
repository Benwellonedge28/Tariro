// Tariro_ide_idpte/src/omega_os/self_healing_models.rs

//! Defines data models for Tariro's Self-Healing & Resilience Optimization module,
//! including failure detection, repair actions, and resilience metrics.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a detected failure or anomaly within the AGI system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureDetection {
    pub id: String,
    pub component_id: String, // ID of the component where failure was detected
    pub failure_type: String, // e.g., "software_bug", "hardware_malfunction", "performance_degradation"
    pub severity: f64, // 0.0 to 1.0
    pub root_cause_diagnosis: String,
    pub ethical_implications_assessed: VerificationStatus, // New: Ethical assessment of failure impact
    pub security_vulnerability_level: VerificationStatus, // New: Security assessment of vulnerability introduced by failure
    pub detected_at: DateTime<Utc>,
}

/// A planned or executed action to repair a detected failure or mitigate a vulnerability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepairAction {
    pub id: String,
    pub failure_id: String, // Link to FailureDetection
    pub action_type: String, // e.g., "reboot", "code_patch", "resource_reallocation"
    pub outcome_summary: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the repair action
    pub security_audited: VerificationStatus, // New: Security audit of the repair implementation
    pub executed_at: DateTime<Utc>,
}

/// A metric representing the overall resilience or robustness of the AGI system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceMetric {
    pub id: String,
    pub metric_name: String, // e.g., "uptime", "recovery_time", "attack_resistance"
    pub value: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub ethical_considerations: VerificationStatus, // New: Ethical consideration of resilience optimization
    pub security_status: VerificationStatus, // New: Overall security status against threats
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of self-healing and resilience optimization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfHealingAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_self_healing_events", "critical_system_repairs"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_failures_handled: u64,
    pub successful_repairs: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of self-healing
    pub security_integrity_status: VerificationStatus, // Overall security integrity of self-healing systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
