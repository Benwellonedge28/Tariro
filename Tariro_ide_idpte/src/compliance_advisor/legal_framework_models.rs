// Tariro_ide_idpte/src/compliance_advisor/legal_framework_models.rs

//! Defines data structures for legal codes, regulations, ethical mandates, and compliance precedents.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a specific legal code or regulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalRule {
    pub id: String,
    pub title: String,
    pub jurisdiction: String, // e.g., "EU", "USA-California", "Global"
    pub description: String,
    pub effective_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub relevant_domains: Vec<String>, // e.g., "data_privacy", "child_protection", "financial_transactions"
    pub penalties: Vec<String>, // e.g., "fine", "imprisonment", "service_suspension"
    pub source_url: Option<String>,
    pub ai_summary: Option<String>, // AI-generated summary of the rule
    pub impact_assessment: Option<String>, // AI-generated assessment of impact on OMEGA OS
}

/// Represents an ethical mandate or axiom, possibly from EVAS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalMandate {
    pub id: String,
    pub title: String,
    pub description: String,
    pub enforcement_level: EnforcementLevel,
    pub relevant_agi_modules: Vec<String>, // e.g., "SORAEE", "QR-CTFO", "HACSE"
    pub origin: String, // e.g., "DAS-CP Framework", "SACC-ASI Specification", "User Defined"
    pub last_reviewed: DateTime<Utc>,
    pub ai_summary: Option<String>, // AI-generated summary of the mandate
    pub impact_assessment: Option<String>, // AI-generated assessment of impact on OMEGA OS
}

/// Level of enforcement for an ethical mandate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Strict,
    Recommended,
    Advisory,
}

/// Represents a precedent or case study for compliance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompliancePrecedent {
    pub id: String,
    pub title: String,
    pub case_summary: String,
    pub outcome: String, // e.g., "compliant", "non-compliant", "mitigated"
    pub implicated_rules: Vec<String>, // IDs of LegalRules or EthicalMandates
    pub date: DateTime<Utc>,
    pub lessons_learned: String,
}

/// The current compliance status of a specific OMEGA OS operation or module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub entity_id: String, // ID of the operation, module, or data flow being checked
    pub entity_type: String,
    pub is_compliant: bool,
    pub violations: Vec<ComplianceViolation>,
    pub last_checked: DateTime<Utc>,
    pub confidence: f64, // Confidence in the compliance assessment (0.0-1.0)
}

/// Details about a specific compliance violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub rule_id: String, // ID of the violated LegalRule or EthicalMandate
    pub severity: ViolationSeverity,
    pub description: String,
    pub recommended_action: Option<String>,
}

/// Severity of a compliance violation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// Represents the compliance status of a specific AGI module or component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleAuditStatus {
    pub module_name: String,
    pub compliant: bool,
    pub issues_found: Vec<String>,
    pub recommendations: Vec<String>,
    pub last_audited: DateTime<Utc>,
}

/// A comprehensive audit report for the entire OMEGA OS / AGI system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub report_id: String,
    pub timestamp: DateTime<Utc>,
    pub overall_compliant: bool,
    pub overall_score: f64, // e.g., 0.0-1.0
    pub summary: String,
    pub module_statuses: Vec<ModuleAuditStatus>,
    pub critical_issues: Vec<String>,
    pub recommended_system_adjustments: Vec<String>, // High-level recommendations
}
