//! GitHub integration entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// GitHub Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubProfile {
    pub id: String,
    pub username: String,
    pub access_token_encrypted: String,
    pub repositories: Vec<String>,
    pub last_sync: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Default for GitHubProfile {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            username: String::new(),
            access_token_encrypted: String::new(),
            repositories: Vec::new(),
            last_sync: None,
            created_at: Utc::now(),
        }
    }
}

/// GitHub Project Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubProjectContext {
    pub repository: String,
    pub branch: String,
    pub files: Vec<String>,
    pub open_issues: u32,
    pub open_prs: u32,
    pub last_commit: Option<DateTime<Utc>>,
}

impl Default for GitHubProjectContext {
    fn default() -> Self {
        Self {
            repository: String::new(),
            branch: "main".to_string(),
            files: Vec::new(),
            open_issues: 0,
            open_prs: 0,
            last_commit: None,
        }
    }
}