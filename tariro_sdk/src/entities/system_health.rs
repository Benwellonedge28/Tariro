//! System Health entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// System Health Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::Unknown
    }
}

/// System Health Log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthLog {
    pub id: String,
    pub status: HealthStatus,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub disk_usage: f32,
    pub active_agents: u32,
    pub pending_tasks: u32,
    pub timestamp: DateTime<Utc>,
    pub alerts: Vec<String>,
}

impl Default for SystemHealthLog {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            status: HealthStatus::Unknown,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            active_agents: 0,
            pending_tasks: 0,
            timestamp: Utc::now(),
            alerts: Vec::new(),
        }
    }
}