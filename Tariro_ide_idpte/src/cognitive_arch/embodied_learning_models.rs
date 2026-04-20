// Tariro_ide_idpte/src/cognitive_arch/embodied_learning_models.rs

//! Defines data models for Tariro's Embodied Learning & Skill Acquisition module,
//! including skill definitions, embodied actions, and skill acquisition reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// A definition of a learned skill, including required capabilities and performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub id: String,
    pub skill_name: String,
    pub description: String,
    pub mastery_level: f64, // 0.0 to 1.0
    pub required_enclave_capabilities: Vec<String>, // e.g., "robotic_arm_control", "simulated_physics_engine"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the skill's potential use
    pub safety_verified: VerificationStatus, // New: Safety verification of the skill execution
    pub acquired_at: DateTime<Utc>,
}

/// A specific action or set of actions performed by Tariro in an environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbodiedAction {
    pub id: String,
    pub skill_id: String, // Link to SkillDefinition
    pub action_parameters: HashMap<String, String>, // e.g., "force": "50N", "duration": "5s"
    pub sensory_feedback_summary: String,
    pub outcome_score: f64, // 0.0 to 1.0
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of the action
    pub security_audited: VerificationStatus, // New: Security audit of the action execution
    pub performed_at: DateTime<Utc>,
}

/// A report detailing the process of acquiring or refining a specific skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAcquisitionReport {
    pub id: String,
    pub skill_id: String,
    pub learning_source: String, // e.g., "digital_twin_simulation", "physical_interaction_logs"
    pub mastery_improvement: f64,
    pub detected_skill_vulnerabilities: Vec<String>,
    pub ethical_impact_assessment: VerificationStatus, // New: Ethical impact of the acquired skill
    pub security_robustness_rating: VerificationStatus, // New: Security robustness of the skill implementation
    pub generated_at: DateTime<Utc>,
}

/// A report summarizing the audit of embodied learning processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbodiedAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_skill_acquisitions", "critical_embodied_actions"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_skills_acquired: u64,
    pub master_level_increase: f64,
    pub detected_safety_breaches: u64,
    pub ethical_compliance_status: VerificationStatus, // Overall ethical compliance of embodied learning
    pub security_integrity_status: VerificationStatus, // Overall security integrity of learning systems
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
