// src/dashboard/models.rs
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDashboard { pub user_id: String, pub last_updated: DateTime<Utc>, pub sections: Vec<DashboardSection>, pub preferences: HashMap<String, String> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSection { pub id: String, pub title: String, pub section_type: SectionType, pub data: serde_json::Value, pub order: i32, pub is_visible: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionType { ActivityFeed, Recommendations, ProgressTracker }
