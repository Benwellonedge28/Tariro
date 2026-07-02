//! Wellbeing tracking entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Wellbeing Log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellbeingLog {
    pub id: String,
    pub metric_type: String,
    pub value: f32,
    pub notes: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl WellbeingLog {
    pub fn new(metric_type: String, value: f32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            metric_type,
            value,
            notes: None,
            timestamp: Utc::now(),
        }
    }
}