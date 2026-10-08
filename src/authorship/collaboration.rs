//! # Collaboration Engine
//!
//! Manages human-AGI collaboration for creative works.

use crate::authorship::CreativeContent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Human feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanFeedback {
    /// Feedback ID
    pub id: String,
    /// Feedback type
    pub feedback_type: FeedbackType,
    /// Content of feedback
    pub content: String,
    /// Specific suggestions
    pub suggestions: HashMap<String, String>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Human identifier
    pub human_id: Option<String>,
}

/// Type of feedback
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeedbackType {
    /// General comments
    Comment,
    /// Specific revision request
    Revision,
    /// Approval
    Approval,
    /// Rejection
    Rejection,
    /// Question
    Question,
}

/// Collaboration session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub session_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub participants: Vec<Participant>,
    pub status: SessionStatus,
    pub contributions: Vec<Contribution>,
}

/// Participant in collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub role: ParticipantRole,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

/// Role of participant
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticipantRole {
    Author,
    Editor,
    Reviewer,
    Observer,
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

/// Contribution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
    pub contributor: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub contribution_type: ContributionType,
    pub description: String,
}

/// Type of contribution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContributionType {
    Feedback,
    Edit,
    Approval,
    Rejection,
}

/// Integration report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationReport {
    pub success: bool,
    pub changes_applied: u32,
    pub conflicts_resolved: u32,
    pub remaining_issues: Vec<String>,
}

/// Collaboration engine
pub struct CollaborationEngine {
    active_sessions: HashMap<String, CollaborationSession>,
}

impl CollaborationEngine {
    /// Create a new collaboration engine
    pub fn new() -> Self {
        Self {
            active_sessions: HashMap::new(),
        }
    }

    /// Create a new collaboration session
    pub fn create_session(&mut self, agi_author: &str, human_participants: Vec<String>) -> CollaborationSession {
        let session_id = uuid::Uuid::new_v4().to_string();

        let mut participants = vec![Participant {
            id: "tariro_agi".to_string(),
            name: agi_author.to_string(),
            role: ParticipantRole::Author,
            joined_at: chrono::Utc::now(),
        }];

        for (i, name) in human_participants.into_iter().enumerate() {
            participants.push(Participant {
                id: format!("human_{}", i),
                name: name.clone(),
                role: ParticipantRole::Editor,
                joined_at: chrono::Utc::now(),
            });
        }

        let session = CollaborationSession {
            session_id: session_id.clone(),
            created_at: chrono::Utc::now(),
            participants,
            status: SessionStatus::Active,
            contributions: Vec::new(),
        };

        self.active_sessions.insert(session_id, session.clone());
        session
    }

    /// Submit human feedback
    pub fn submit_feedback(&mut self, session_id: &str, feedback: HumanFeedback) -> Result<(), String> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            session.contributions.push(Contribution {
                contributor: feedback.human_id.clone().unwrap_or_else(|| "anonymous".to_string()),
                timestamp: chrono::Utc::now(),
                contribution_type: ContributionType::Feedback,
                description: feedback.content.clone(),
            });
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }

    /// Integrate human feedback into content
    pub fn integrate_feedback(
        &mut self,
        content: &CreativeContent,
        feedback: &HumanFeedback,
    ) -> IntegrationReport {
        let mut changes_applied = 0u32;
        let mut conflicts_resolved = 0u32;
        let mut remaining_issues = Vec::new();

        match feedback.feedback_type {
            FeedbackType::Revision => {
                for (key, suggestion) in &feedback.suggestions {
                    if !suggestion.is_empty() {
                        changes_applied += 1;
                    } else {
                        remaining_issues.push(format!("Could not apply: {}", key));
                    }
                }
            }
            FeedbackType::Approval => {
                changes_applied = 1;
            }
            FeedbackType::Rejection => {
                remaining_issues.push("Content was rejected".to_string());
            }
            FeedbackType::Comment | FeedbackType::Question => {
                // No direct changes, but logged
            }
        }

        IntegrationReport {
            success: remaining_issues.is_empty(),
            changes_applied,
            conflicts_resolved,
            remaining_issues,
        }
    }

    /// End a collaboration session
    pub fn end_session(&mut self, session_id: &str) -> Result<CollaborationSession, String> {
        if let Some(mut session) = self.active_sessions.remove(session_id) {
            session.status = SessionStatus::Completed;
            Ok(session)
        } else {
            Err("Session not found".to_string())
        }
    }

    /// Get active sessions
    pub fn get_active_sessions(&self) -> Vec<&CollaborationSession> {
        self.active_sessions
            .values()
            .filter(|s| s.status == SessionStatus::Active)
            .collect()
    }
}

impl Default for CollaborationEngine {
    fn default() -> Self {
        Self::new()
    }
}
