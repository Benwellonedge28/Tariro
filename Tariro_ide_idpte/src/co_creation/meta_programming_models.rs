// Tariro_ide_idpte/src/co_creation/meta_programming_models.rs

//! Defines data models for Tariro's Collaborative Meta-Programming & Self-Optimization module,
//! including code generation requests, optimized code modules, and architectural change proposals.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A request from a human or another AGI instance for Tariro to generate or modify its own code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenerationRequest {
    pub id: String,
    pub target_component: String, // e.g., "MetaLearner", "ALEC-A Orchestrator"
    pub desired_functionality: String,
    pub constraints: Vec<String>, // e.g., "optimize_for_speed", "reduce_memory_footprint", "ethical_compliance"
    pub ethical_review_status: VerificationStatus, // Ethical review of the request's intent
    pub security_verified: VerificationStatus, // Security assessment of the code generation scope
    pub requested_at: DateTime<Utc>,
}

/// A segment of Tariro's own source code that has been generated, refactored, or optimized by itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedCodeModule {
    pub id: String,
    pub request_id: Option<String>, // Link to CodeGenerationRequest if applicable
    pub module_path: String,
    pub original_code_hash: String, // Hash of the code before optimization
    pub optimized_code_hash: String, // Hash of the new/optimized code
    pub performance_gain: f64, // Expected performance improvement (0.0 to 1.0)
    pub ethical_compliance: VerificationStatus, // Ethical compliance of the generated code
    pub security_audited: VerificationStatus, // Security audit of the generated code
    pub generated_at: DateTime<Utc>,
}

/// A proposal for a significant modification to Tariro's internal architectural design.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalChangeProposal {
    pub id: String,
    pub proposed_change_summary: String,
    pub affected_modules: Vec<String>,
    pub expected_system_impact: String,
    pub formal_verification_result: VerificationStatus, // Result of formal verification
    pub ethical_review_status: VerificationStatus, // Comprehensive ethical review of architectural change
    pub security_impact_assessment: VerificationStatus, // Comprehensive security impact assessment
    pub proposed_at: DateTime<Utc>,
}

/// Audit report for collaborative meta-programming and self-optimization activities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgrammingAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_code_modifications", "critical_architectural_changes"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_code_generated: u64,
    pub total_architectural_changes: u64,
    pub detected_self_corruption_attempts: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
