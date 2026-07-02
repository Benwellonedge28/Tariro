//! Notification entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Notification Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
    ApprovalRequired,
}

impl Default for NotificationType {
    fn default() -> Self {
        NotificationType::Info
    }
}

/// Notification Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationStatus {
    Unread,
    Read,
    Acknowledged,
    Dismissed,
}

impl Default for NotificationStatus {
    fn default() -> Self {
        NotificationStatus::Unread
    }
}

/// Notification Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRecord {
    pub id: String,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub status: NotificationStatus,
    pub source: String,
    pub action_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub read_at: Option<DateTime<Utc>>,
}

impl NotificationRecord {
    pub fn new(notification_type: NotificationType, title: String, message: String, source: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            notification_type,
            title,
            message,
            status: NotificationStatus::Unread,
            source,
            action_url: None,
            created_at: Utc::now(),
            read_at: None,
        }
    }
}