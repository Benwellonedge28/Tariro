// Tariro_ide_idpte/src/cognitive_arch/swarm_models.rs

//! Defines data models for Tariro's Distributed Cognition & Swarm Intelligence module,
//! including AGI swarms, swarm tasks, and ethical compliance records for swarm operations.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a dynamically formed AGI swarm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgiSwarm {
    pub id: String,
    pub swarm_name: String,
    pub objective: String,
    pub member_ids: Vec<String>, // IDs of participating AGI instances or sub-modules
    pub leader_id: Option<String>, // ID of the designated swarm leader
    pub formation_timestamp: DateTime<Utc>,
    pub dissolution_timestamp: Option<DateTime<Utc>>,
    pub ethical_review_status: VerificationStatus, // New: Ethical review of swarm formation/objective
    pub security_verified: VerificationStatus, // New: Security of swarm integrity and coordination
}

/// Represents a task assigned to an AGI swarm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmTask {
    pub id: String,
    pub swarm_id: String,
    pub title: String,
    pub description: String,
    pub assigned_subtasks: HashMap<String, String>, // Member ID -> Subtask Description
    pub current_status: String, // e.g., "pending", "in_progress", "completed", "failed"
    pub progress_percentage: f64,
    pub critical_resources: Vec<String>, // IDs of resources required
    pub ethical_compliance: VerificationStatus, // New: Ethical compliance of task execution
    pub security_integrity: VerificationStatus, // New: Security integrity of task execution
    pub last_updated: DateTime<Utc>,
}

/// A secure log entry for inter-swarm communication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmCommunicationLog {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub swarm_id: String,
    pub message_type: String, // e.g., "coordination_signal", "data_share", "insight_exchange"
    pub message_summary: String,
    pub encryption_status: VerificationStatus, // New: Encryption status of communication
    pub integrity_verified: VerificationStatus, // New: Integrity of message content
    pub timestamp: DateTime<Utc>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
}

/// A record summarizing the ethical compliance of swarm operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmEthicalCompliance {
    pub id: String,
    pub swarm_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub overall_ethical_compliance: VerificationStatus,
    pub identified_misalignments: Vec<String>,
    pub recommendations_for_improvement: Vec<String>,
    pub blockchain_record_hash: String, // New: Tamper-proof hash from QR-CTFO
    pub generated_at: DateTime<Utc>,
}
