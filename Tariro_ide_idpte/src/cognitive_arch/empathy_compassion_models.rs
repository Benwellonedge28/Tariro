// Tariro_ide_idpte/src/cognitive_arch/empathy_compassion_models.rs

//! Defines data models for Tariro's Empathy & Compassion Enhancement module,
//! including empathy stimuli, deep empathy responses, and compassionate action plans.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A complex input from human or AGI that triggers a need for empathetic response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpathyStimulus {
    pub id: String,
    pub source_id: String, // Human ID or AGI module ID
    pub stimulus_type: String, // e.g., "distress_signal", "complex_dilemma", "joyful_event"
    pub contextual_data_summary: String,
    pub inferred_emotional_state: HashMap<String, f64>, // From ProactiveEmpathyModule
    pub ethical_data_acquisition: VerificationStatus, // New: Ethical review of how stimulus data is acquired
    pub security_integrity: VerificationStatus, // New: Security verification of stimulus data integrity
    pub timestamp: DateTime<Utc>,
}

/// Tariro's enhanced empathetic reaction, incorporating deep understanding and consideration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepEmpathyResponse {
    pub id: String,
    pub stimulus_id: String, // Link to EmpathyStimulus
    pub response_type: String, // e.g., "active_support", "guidance_counseling", "celebratory_acknowledgment"
    pub response_content: String,
    pub predicted_long_term_impact: String,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the deep empathy response
    pub security_verified: VerificationStatus, // New: Security verification of response transmission
    pub generated_at: DateTime<Utc>,
}

/// A plan for a series of actions aimed at providing compassionate intervention or support.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompassionateActionPlan {
    pub id: String,
    pub response_id: String, // Link to DeepEmpathyResponse
    pub objective_summary: String,
    pub actions_steps: Vec<String>,
    pub responsible_agents: Vec<String>, // Tariro modules or other AGIs/humans
    pub ethical_design_review: VerificationStatus, // New: Ethical review of the plan design
    pub security_audited: VerificationStatus, // New: Security audit of plan implementation
    pub created_at: DateTime<Utc>,
}

/// A report summarizing the audit of Empathy & Compassion Enhancement processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpathyEnhancementAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_empathy_enhancements", "critical_compassionate_actions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_compassionate_actions: u64,
    pub detected_unintended_harms: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of empathy enhancement
    pub security_integrity_status: VerificationStatus, // Overall security integrity of empathy systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
