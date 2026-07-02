//! Knowledge management entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Knowledge Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: String,
    pub topic: String,
    pub content: String,
    pub source: String,
    pub confidence: f32,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
}

impl KnowledgeItem {
    pub fn new(topic: String, content: String, source: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            topic,
            content,
            source,
            confidence: 0.5,
            tags: Vec::new(),
            created_at: now,
            last_accessed: now,
        }
    }
}