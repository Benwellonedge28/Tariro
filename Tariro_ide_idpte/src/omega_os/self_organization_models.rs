// Tariro_ide_idpte/src/omega_os/self_organization_models.rs

//! Defines data models for Tariro's Self-Organization & Emergence module,
//! including emergent behavior patterns, coordination mechanisms, and reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a newly identified emergent behavior pattern within Tariro's systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentBehaviorPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub observed_components: Vec<String>, // IDs of components involved
    pub triggering_conditions: HashMap<String, String>, // What led to this pattern
    pub observed_effect: String, // Positive, negative, or neutral impact
    pub ethical_review_status: VerificationStatus, // New: Ethical assessment of the emergent pattern
    pub security_implications_status: VerificationStatus, // New: Security assessment of the pattern
    pub discovered_at: DateTime<Utc>,
}

/// Represents a mechanism for dynamic coordination between system components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationMechanism {
    pub id: String,
    pub name: String,
    pub mechanism_type: String, // e.g., "event_bus_subscription", "shared_memory_access", "direct_api_call"
    pub participating_components: Vec<String>,
    pub security_protocol: String, // e.g., "end_to_end_encryption", "access_control_lists"
    pub integrity_verified: VerificationStatus, // New: Integrity of the mechanism
    pub last_updated: DateTime<Utc>,
}

/// A directive issued by Tariro to guide its own self-organization processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfOrganizationDirective {
    pub id: String,
    pub goal: String, // The desired outcome of self-organization
    pub target_components: Vec<String>,
    pub proposed_changes: Vec<String>, // e.g., "increase communication between X and Y", "reduce coupling of A and B"
    pub expected_emergent_properties: Vec<String>,
    pub ethical_compliance_status: VerificationStatus, // From ALEC-A
    pub security_compliance_status: VerificationStatus, // From QR-CTFO
    pub issued_at: DateTime<Utc>,
}

/// A report generated from monitoring and analyzing emergent patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentPatternReport {
    pub id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub observed_patterns: Vec<String>, // IDs of EmergentBehaviorPatterns
    pub overall_impact_summary: String,
    pub ethical_risks_identified: Vec<String>,
    pub security_risks_identified: Vec<String>,
    pub recommendations: Vec<String>,
    pub report_integrity_hash: String, // Hash for report integrity
    pub generated_at: DateTime<Utc>,
}
