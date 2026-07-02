//! MetaProject entity for goal management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MetaProject status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetaProjectStatus {
    Planning,
    Active,
    Paused,
    Completed,
    Cancelled,
}

impl Default for MetaProjectStatus {
    fn default() -> Self {
        MetaProjectStatus::Planning
    }
}

/// MetaProject entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaProject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: MetaProjectStatus,
    pub goals: Vec<String>,
    pub sub_projects: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl MetaProject {
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            status: MetaProjectStatus::Planning,
            goals: Vec::new(),
            sub_projects: Vec::new(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }
}