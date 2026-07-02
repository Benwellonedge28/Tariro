//! Learning and Knowledge entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Learning Insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsight {
    pub id: String,
    pub category: String,
    pub content: String,
    pub confidence: f32,
    pub source: String,
    pub reinforced_count: u32,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl LearningInsight {
    pub fn new(category: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            category,
            content,
            confidence: 0.5,
            source: "system".to_string(),
            reinforced_count: 1,
            created_at: now,
            last_updated: now,
        }
    }
}

/// Learning Category
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningCategory {
    Preference,
    Fact,
    Behavior,
    TopicInterest,
    CommunicationStyle,
    SystemOptimization,
    EthicalGuideline,
    KnowledgeGap,
    PredictionModel,
}

impl Default for LearningCategory {
    fn default() -> Self {
        LearningCategory::Fact
    }
}