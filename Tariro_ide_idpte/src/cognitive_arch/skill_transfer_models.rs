// Tariro_ide_idpte/src/cognitive_arch/skill_transfer_models.rs

//! Defines data models for Tariro's Universal Skill Transfer & Knowledge Distillation module,
//! including skill transfer requests, knowledge packets, and skill distillation reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus;

/// A request to transfer a learned skill or knowledge from one domain to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTransferRequest {
    pub id: String,
    pub source_domain_id: String,
    pub target_domain_id: String,
    pub skill_id: String, // ID of the skill or knowledge to transfer
    pub context_of_transfer: String,
    pub ethical_review_status: VerificationStatus, // Ethical review of the transfer intent
    pub security_verified: VerificationStatus, // Security assessment of the transfer mechanism
    pub requested_at: DateTime<Utc>,
}

/// A compact, portable, and distilled representation of a complex skill or expertise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgePacket {
    pub id: String,
    pub source_skill_id: String, // Link to the original skill
    pub distilled_representation: String, // e.g., "RuleSet", "CompactNN", "AbstractPrinciple"
    pub transfer_format: String, // e.g., "symbolic_logic", "compressed_neural_weights"
    pub ethical_compliance: VerificationStatus, // Ethical compliance of the distilled knowledge
    pub security_audited: VerificationStatus, // Security audit of the knowledge packet itself
    pub generated_at: DateTime<Utc>,
}

/// A report detailing the process and outcome of distilling a skill or knowledge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDistillationReport {
    pub id: String,
    pub skill_id: String,
    pub distillation_method: String,
    pub compression_ratio: f64,
    pub fidelity_score: f64, // How well the distilled knowledge retains original performance
    pub ethical_impact_assessment: VerificationStatus,
    pub generated_at: DateTime<Utc>,
}

/// Audit report for universal skill transfer and knowledge distillation processes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAuditReport {
    pub id: String,
    pub scope: String, // e.g., "all_skill_transfers", "cross_domain_knowledge_sharing"
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_transfers_attempted: u64,
    pub successful_transfers: u64,
    pub detected_transfer_failures: u64,
    pub ethical_compliance_status: VerificationStatus,
    pub security_integrity_status: VerificationStatus,
    pub blockchain_record_hash: String,
    pub generated_at: DateTime<Utc>,
}
