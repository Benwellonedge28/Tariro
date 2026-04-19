// Tariro_ide_idpte/src/cognitive_arch/learning_memory_models.rs

//! Defines data models for knowledge chunks, learning experiences, and consolidated knowledge
//! within Tariro's Continual Lifelong Learning module.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For integrity and ethical vetting

/// Represents a discrete piece of knowledge or a learned skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeChunk {
    pub id: String,
    pub description: String,
    pub content_summary: String, // e.g., learned rule, learned pattern, specific fact
    pub domain: String,
    pub source: String, // e.g., "experience", "observation", "communication", "inference"
    pub strength: f64, // How well-established or validated this chunk is (0.0 to 1.0)
    pub associated_models: Vec<String>, // IDs of models that represent this knowledge (e.g., neural net weights)
    pub timestamp: DateTime<Utc>,
    pub integrity_status: KnowledgeIntegrityStatus, // New: Verification status for integrity
}

/// Represents a specific learning event or experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningExperience {
    pub id: String,
    pub description: String,
    pub learned_skill_id: Option<String>, // If a specific skill was learned
    pub learned_concept_ids: Vec<String>, // Concepts formed/refined
    pub relevant_data_summary: String, // Summary of data used in learning
    pub domain: String,
    pub outcome: String, // e.g., "successful_task_completion", "error_correction", "new_prediction"
    pub timestamp: DateTime<Utc>,
    pub data_source_integrity: VerificationStatus, // New: Integrity of the data used for learning
}

/// Represents knowledge that has been consolidated into Tariro's long-term memory
/// and integrated into existing cognitive structures, resistant to forgetting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedKnowledge {
    pub id: String,
    pub concept_id: String, // The concept or skill being consolidated
    pub version: u32, // Version of this consolidated knowledge
    pub integration_summary: String, // How it integrates with existing knowledge
    pub robustness_score: f64, // How resilient it is to forgetting (0.0 to 1.0)
    pub timestamp: DateTime<Utc>,
    pub ethical_vetting_status: VerificationStatus, // New: Ethical review of the consolidated knowledge
    pub forgetting_prevention_strategy: String, // New: e.g., "rehearsal", "regularization", "sparse_coding"
}

/// Status of cryptographic integrity verification for a knowledge chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeIntegrityStatus {
    Pending, 
    Verified(String), // Verified with details/cert ID
    Compromised(String), // Compromised with reason
    Unverifiable,
}

/// Represents a gap or inconsistency detected in the knowledge base.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGap {
    pub id: String,
    pub description: String,
    pub inferred_missing_information: String,
    pub identified_by_module: String, // e.g., "CausalReasoningEngine", "CommonSenseReasoningNetwork"
    pub severity: f64, // 0.0 to 1.0
    pub suggested_exploration_target: String, // Target for CuriosityExplorationModule
    pub timestamp: DateTime<Utc>,
}
