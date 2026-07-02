//! IDE integration entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// IDE Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEProfile {
    pub id: String,
    pub ide_type: String,
    pub settings: std::collections::HashMap<String, String>,
    pub extensions: Vec<String>,
    pub last_connection: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Default for IDEProfile {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            ide_type: "vscode".to_string(),
            settings: std::collections::HashMap::new(),
            extensions: Vec::new(),
            last_connection: None,
            created_at: Utc::now(),
        }
    }
}