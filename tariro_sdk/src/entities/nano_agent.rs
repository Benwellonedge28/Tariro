//! NanoAgent and Swarm entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Agent status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentStatus {
    Idle,
    Working,
    Completed,
    Failed,
    Suspended,
}

impl Default for AgentStatus {
    fn default() -> Self {
        AgentStatus::Idle
    }
}

/// NanoAgent entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NanoAgent {
    pub id: String,
    pub name: String,
    pub agent_type: String,
    pub status: AgentStatus,
    pub capabilities: Vec<String>,
    pub current_task: Option<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub parent_swarm_id: Option<String>,
}

impl NanoAgent {
    pub fn new(name: String, agent_type: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            agent_type,
            status: AgentStatus::Idle,
            capabilities: Vec::new(),
            current_task: None,
            metadata: HashMap::new(),
            created_at: now,
            last_active: now,
            parent_swarm_id: None,
        }
    }
}

/// Swarm entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swarm {
    pub id: String,
    pub name: String,
    pub agent_ids: Vec<String>,
    pub status: AgentStatus,
    pub coordination_mode: String,
    pub created_at: DateTime<Utc>,
}

impl Default for Swarm {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            agent_ids: Vec::new(),
            status: AgentStatus::Idle,
            coordination_mode: "distributed".to_string(),
            created_at: Utc::now(),
        }
    }
}