// Tariro_ide_idpte/src/security/threat_intelligence_models.rs

//! Defines data models for Tariro's Real-time Threat Intelligence Fusion & Adaptive Defense module,
//! including threat feeds, threat events, and adaptive defense strategies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;
use crate::security::risk_management_models::OperationalDomain;

/// A source of real-time threat information from internal sensors or external feeds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatFeed {
    pub id: String,
    pub feed_name: String,
    pub feed_type: String, // e.g., "cyber_honeypot", "social_media_monitor", "physical_sensor_network"
    pub reliability_score: f64, // 0.0 to 1.0
    pub data_source_origin: String,
    pub ethical_privacy_review: VerificationStatus, // Ethical review for privacy violations in data collection
    pub security_integrity_status: VerificationStatus, // Security of the feed data itself
    pub last_updated: DateTime<Utc>,
}

/// A detected security incident or potential threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub id: String,
    pub threat_feed_id: String, // Link to ThreatFeed
    pub event_description: String,
    pub detected_context: String, // Where the threat was detected
    pub threat_type: String, // e.g., "malware_infection", "social_engineering_attempt", "physical_intrusion"
    pub severity_score: f64, // 0.0 to 1.0
    pub ethical_impact_assessment: VerificationStatus, // Ethical implications of the threat
    pub security_vulnerability_link: String, // CVE or internal vulnerability ID
    pub detected_at: DateTime<Utc>,
}

/// A dynamically generated and deployed defense plan to counter a detected threat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveDefenseStrategy {
    pub id: String,
    pub threat_event_id: String, // Link to ThreatEvent
    pub strategy_description: String,
    pub implemented_actions: Vec<String>, // List of specific actions taken
    pub expected_effectiveness: f64, // 0.0 to 1.0
    pub ethical_compliance_status: VerificationStatus, // Ethical review of the defense actions
    pub security_audited: VerificationStatus, // Security audit of the defense implementation
    pub activated_at: DateTime<Utc>,
}

/// Audit report for real-time threat intelligence fusion and adaptive defense processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceAuditReport {
    pub id: String,
    pub scope: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_threats_detected: u64,
    pub total_defenses_deployed: u64,
    pub undetected_breaches: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
