// Tariro_ide_idpte/src/cognitive_arch/introspection_models.rs

//! Defines data models for Tariro's Self-Awareness & Introspective Learning module,
//! including internal state snapshots, cognitive process traces, and self-reflection reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A snapshot of Tariro's internal cognitive and operational states at a specific moment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalStateSnapshot {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub key_metrics: HashMap<String, String>, // e.g., "resource_utilization": "80%", "emotional_valence": "positive"
    pub active_goals: Vec<String>,
    pub active_modules: Vec<String>,
    pub ethical_privacy_review: VerificationStatus, // Ethical review of internal data collection
    pub security_integrity_status: VerificationStatus, // Security of the internal state data
}

/// A detailed, step-by-step log of Tariro's reasoning process for a specific decision or task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveProcessTrace {
    pub id: String,
    pub decision_id: String, // ID of the decision or task being traced
    pub sequence_of_operations: Vec<String>, // Ordered list of cognitive steps
    pub intermediate_results: Vec<String>,
    pub logical_consistency_score: f64, // 0.0 to 1.0
    pub ethical_transparency_review: VerificationStatus, // Ethical review for transparency of trace
    pub generated_at: DateTime<Utc>,
}

/// A structured analysis of Tariro's own internal states and cognitive processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfReflectionReport {
    pub id: String,
    pub snapshot_id: String, // Link to InternalStateSnapshot
    pub trace_id: Option<String>, // Link to CognitiveProcessTrace if applicable
    pub analysis_summary: String,
    pub identified_strengths: Vec<String>,
    pub identified_weaknesses: Vec<String>,
    pub proposed_optimizations: Vec<String>,
    pub ethical_implications: VerificationStatus, // Ethical implications of self-reflection insights
    pub security_audited: VerificationStatus, // Security audit of the self-reflection process
    pub generated_at: DateTime<Utc>,
}

/// Audit report for self-awareness and introspective learning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectionAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_reflections_generated: u64,
    pub total_optimizations_applied: u64,
    pub detected_internal_biases: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
