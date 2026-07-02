//! Performance logging entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Performance Log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceLog {
    pub id: String,
    pub operation: String,
    pub duration_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl PerformanceLog {
    pub fn new(operation: String, duration_ms: u64, success: bool) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            operation,
            duration_ms,
            success,
            error_message: None,
            timestamp: Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }
}