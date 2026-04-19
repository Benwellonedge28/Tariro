// Tariro_ide_idpte/src/cognitive_arch/analogy_models.rs

//! Defines data models for representing source and target domains, and the mappings
//! that constitute an analogy within Tariro's Analogical Reasoning System.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For solution verification

/// Represents a well-understood domain from which knowledge or solutions can be transferred.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceDomain {
    pub id: String,
    pub name: String,
    pub description: String,
    pub problem_description: String,
    pub solution_description: String,
    pub key_relations: HashMap<String, String>, // e.g., "problem_type": "flow_control", "component_interaction": "fluid_dynamics"
    pub abstract_structure: String, // e.g., "A -> B causes C, A and B are resources"
    pub knowledge_representation_type: String, // New: e.g., "causal_graph", "conceptual_hierarchy"
}

/// Represents a novel or less-understood domain to which knowledge or solutions might be applicable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetDomain {
    pub id: String,
    pub name: String,
    pub description: String,
    pub problem_description: String,
    pub key_relations: HashMap<String, String>, // e.g., "problem_type": "data_transfer", "component_interaction": "packet_flow"
    pub desired_outcome: String,
    pub target_knowledge_representation_type: String, // New
}

/// Represents the mapping (analogy) between a source and a target domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogyMapping {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub correspondences: HashMap<String, String>, // e.g., "fluid_flow": "data_stream", "pipe": "network_channel"
    pub abstract_mapping: String, // Abstract structure shared between domains
    pub confidence: f64, // 0.0 to 1.0
    pub discovered_by: String, // e.g., "Tariro_ARS", "Human"
    pub timestamp: DateTime<Utc>,
    pub refinement_history: Vec<AnalogyRefinement>, // New: Tracks how analogy was refined
}

/// Tracks refinement steps of an analogy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogyRefinement {
    pub timestamp: DateTime<Utc>,
    pub adjustment_summary: String,
    pub new_confidence: f64,
    pub reason: String,
}

/// Represents an adapted solution for the target domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptedSolution {
    pub id: String,
    pub original_solution_id: String, // ID of the original solution/source domain
    pub target_domain_id: String,
    pub adapted_content: String, // The modified solution for the target domain
    pub adaptation_log: String, // Details of how the solution was adapted
    pub verification_status: SolutionVerificationStatus, // New: Verification status
    pub created_at: DateTime<Utc>,
}

/// Comprehensive verification status for an adapted solution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionVerificationStatus {
    pub ethical_status: VerificationStatus,
    pub security_status: VerificationStatus,
    pub correctness_status: VerificationStatus, // New: Has the adapted solution been validated?
    pub overall_status: VerificationStatus,
    pub last_verified: DateTime<Utc>,
    pub verification_log_id: String,
}
