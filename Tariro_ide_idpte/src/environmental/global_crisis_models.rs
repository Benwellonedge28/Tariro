// Tariro_ide_idpte/src/environmental/global_crisis_models.rs

//! Defines data models for Tariro's Autonomous Global Crisis Response & Management module,
//! including crisis events, crisis assessments, and response plans.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A detected global crisis event (e.g., humanitarian, environmental, geopolitical).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrisisEvent {
    pub id: String,
    pub event_type: String, // e.g., "humanitarian_famine", "ecological_collapse", "geopolitical_conflict"
    pub geographical_scope: String,
    pub detected_sources: Vec<String>, // e.g., "satellite_imagery", "social_media_analysis", "news_feeds"
    pub severity_score: f64, // 0.0 to 1.0, critical=1.0
    pub ethical_impact_assessment: VerificationStatus, // Ethical assessment of the crisis nature
    pub security_threat_level: f64, // 0.0 to 1.0
    pub detected_at: DateTime<Utc>,
}

/// Tariro's real-time assessment of a detected global crisis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrisisAssessment {
    pub id: String,
    pub crisis_event_id: String, // Link to CrisisEvent
    pub current_status: String, // e.g., "escalating", "stabilizing", "contained"
    pub predicted_trajectory: String,
    pub affected_populations_count: u64,
    pub resource_needs_estimate: HashMap<String, f64>, // Resource type -> Quantity
    pub ethical_priority_review: VerificationStatus, // Ethical prioritization of response actions
    pub security_implications: VerificationStatus, // Security implications of the crisis
    pub performed_at: DateTime<Utc>,
}

/// An autonomously generated and dynamically adjusted crisis response strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePlan {
    pub id: String,
    pub crisis_assessment_id: String, // Link to CrisisAssessment
    pub plan_summary: String,
    pub coordinated_actions: Vec<String>, // List of specific actions to be taken
    pub resource_allocation_strategy: HashMap<String, String>, // Resource type -> Allocation method
    pub expected_harm_reduction: f64, // 0.0 to 1.0
    pub ethical_compliance_status: VerificationStatus, // Ethical compliance of the response plan
    pub security_verified: VerificationStatus, // Security verification of the response execution
    pub activated_at: DateTime<Utc>,
}

/// Audit report for autonomous global crisis response and management operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrisisManagementAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_crises_managed: u64,
    pub total_interventions_made: u64,
    pub detected_unintended_consequences: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
