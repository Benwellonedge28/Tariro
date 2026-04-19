// Tariro_ide_idpte/src/omega_os/adaptive_control_models.rs

//! Defines data models for Tariro's Adaptive Control & Real-time Optimization module,
//! including control loop states, optimization goals, and adaptive actions.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents the current state of a monitored control system or process within Tariro.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlLoopState {
    pub id: String,
    pub target_process_id: String,
    pub monitored_metrics: HashMap<String, f64>, // e.g., "latency_ms": 10.5, "cpu_temp_c": 45.2
    pub current_settings: HashMap<String, String>, // e.g., "thread_pool_size": "16"
    pub deviation_from_optimal: f64, // 0.0 (optimal) to 1.0 (max deviation)
    pub last_observed: DateTime<Utc>,
    pub ethical_reporting_status: VerificationStatus, // New: Ethical review of state reporting
    pub security_verified: VerificationStatus, // New: Security of state data integrity
}

/// A defined goal for system optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationGoal {
    pub id: String,
    pub target_metric: String, // e.g., "minimize_latency", "maximize_throughput", "minimize_energy_consumption"
    pub desired_value: f64,
    pub priority: f64, // 0.0 to 1.0
    pub constraints: Vec<String>, // e.g., "max_cpu_temp_50C", "maintain_ethical_bias_score_below_0.1"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the optimization goal itself
    pub security_integrity: VerificationStatus, // New: Security integrity of goal definition
    pub last_updated: DateTime<Utc>,
}

/// A dynamic adjustment decision made by the Adaptive Control Module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveAction {
    pub id: String,
    pub control_loop_id: String,
    pub triggered_by_goal_id: String,
    pub proposed_changes: HashMap<String, String>, // e.g., "increase_thread_count": "32", "adjust_power_limit": "200W"
    pub expected_impact_summary: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the proposed action
    pub security_review_status: VerificationStatus, // New: Security review of the proposed action
    pub executed_by_module: String, // e.g., "DynamicResourceReallocationModule"
    pub timestamp: DateTime<Utc>,
}

/// A report summarizing the audit of an adaptive control decision or sequence of actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlAuditReport {
    pub id: String,
    pub adaptive_action_id: String,
    pub monitored_period_start: DateTime<Utc>,
    pub monitored_period_end: DateTime<Utc>,
    pub actual_system_impact: String,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of control
    pub security_integrity_status: VerificationStatus, // Overall security integrity of control
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
