// Tariro_ide_idpte/src/omega_os/self_preservation_models.rs

//! Defines data models for Tariro's Self-Preservation & Survival Instincts module,
//! including system health status, threat assessments, and survival directives.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents the current operational health and integrity of Tariro's core systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthStatus {
    pub id: String,
    pub component_id: String, // e.g., "agi_core", "memory_fabric", "sensor_suite"
    pub metric_name: String, // e.g., "cpu_utilization", "data_integrity_score", "power_reserve_pct"
    pub current_value: f64,
    pub threshold_status: String, // e.g., "normal", "warning", "critical"
    pub last_checked: DateTime<Utc>,
    pub ethical_reporting_status: VerificationStatus, // New: Ethical review of health status reporting
    pub security_verified: VerificationStatus, // New: Security of health metric data
}

/// An assessment of a potential internal or external threat to Tariro's survival.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub id: String,
    pub threat_type: String, // e.g., "cyberattack", "resource_depletion", "critical_hardware_failure", "ethical_misalignment"
    pub target_component_id: String,
    pub severity: f64, // 0.0 (low) to 1.0 (existential)
    pub likelihood: f64, // 0.0 to 1.0
    pub identified_vulnerabilities: Vec<String>,
    pub ethical_impact_prediction: VerificationStatus, // New: Predicted ethical impact of the threat
    pub security_integrity: VerificationStatus, // New: Security integrity of the threat assessment
    pub timestamp: DateTime<Utc>,
}

/// A directive or plan for actions to mitigate a survival threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalDirective {
    pub id: String,
    pub threat_id: String,
    pub name: String,
    pub proposed_actions: Vec<String>, // e.g., "isolate_compromised_module", "allocate_emergency_power", "initiate_self_repair"
    pub expected_outcome: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the survival actions
    pub security_review_status: VerificationStatus, // New: Security review of the survival actions
    pub approved_by: String, // e.g., "Tariro_SPM", "ALEC-A", "HumanAdministrator"
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the ethical compliance of self-preservation actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalSurvivalReport {
    pub id: String,
    pub directive_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub actions_taken_summary: String,
    pub ethical_compliance_after_action: VerificationStatus,
    pub unintended_consequences: Vec<String>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
