//! File management entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Managed File
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedFile {
    pub id: String,
    pub name: String,
    pub path: String,
    pub file_type: String,
    pub size_bytes: u64,
    pub content_hash: String,
    pub metadata: std::collections::HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ManagedFile {
    pub fn new(name: String, path: String, file_type: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            path,
            file_type,
            size_bytes: 0,
            content_hash: String::new(),
            metadata: std::collections::HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
}