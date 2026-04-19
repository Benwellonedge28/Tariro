// Tariro_ide_idpte/src/cognitive_arch/learning_memory_models.rs

//! Defines data models for knowledge chunks, learning experiences, and consolidated knowledge
//! within Tariro's Continual Lifelong Learning module.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

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
}
