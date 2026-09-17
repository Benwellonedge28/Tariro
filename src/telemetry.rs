// src/telemetry.rs

//! Handles all telemetry, logging, and monitoring aspects of the Tariro AGI.

use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

/// Represents a generic telemetry event within the AGI system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TelemetryEvent {
    // System Lifecycle
    SystemInitialized { version: String },
    SystemStarted,
    SystemShutdown,
    ServiceInitialized { service_name: String },
    ServiceShutdown { service_name: String },

    // Error & Warning
    ErrorOccurred { message: String, details: Option<String> },
    SystemHealthWarning { component: String, level: String, message: String },

    // AGI Actions & Decisions
    AGIAction { action_type: String, details: Option<String> },
    DecisionMade { decision_id: String, confidence: f32, context_summary: String },
    GoalProgressUpdated { goal_id: String, progress_pct: f32 },
    FeatureProposed { title: String, source: String },

    // Learning & Knowledge
    LearningInsightProcessed { insight_id: String, category: String },
    KnowledgeAcquired { item_id: String, title: String, category: String },
    KnowledgeGapIdentified { description: String },

    // Admin & Control
    AdminCommandIssued { command_id: String, command_type: String, initiator: String },
    AdminCommandCompleted { command_id: String, status: String, message: String },
    AutonomyLevelChanged { new_level: String, domain: String },

    // Project & Task Management
    ProjectCreated { project_id: String, project_name: String },
    ProjectUpdated { project_id: String, project_name: String, new_status: Option<String> },
    TaskCreated { task_id: String, task_title: String, project_id: String },
    TaskUpdated { task_id: String, task_title: String, new_status: Option<String> },
    TaskAssignedToSwarm { swarm_id: String, task_id: String },

    // Replication & Deployment
    ReplicationInitiated { request_id: String, target_name: String, deployment_target: String },
    ReplicationStatusUpdated { request_id: String, new_status: String },
    DeploymentCompleted { record_id: String, status: String },

    // Multi-Agent & Swarm
    NanoAgentRegistered { agent_id: String, agent_type: String },
    AgentStatusUpdated { agent_id: String, new_status: String },
    SwarmFormed { swarm_id: String, objective: String },
    SwarmDisbanded { swarm_id: String },
    AgentTaskAssigned { agent_id: String, task_id: String },

    // Network & Communication
    OmegaNodeRegistered { node_id: String, node_type: String },
    OmegaNodeStatusUpdated { node_id: String, new_status: String },
    OmegaChannelCreated { channel_id: String, channel_type: String },
    OmegaDataTransferred { channel_id: String, data_size_bytes: u64 },
    MessageSent { sender: String, recipient: String, message_type: String },
    MessageReceived { sender: String, recipient: String, message_type: String },

    // External Interactions
    HttpRequest { method: String, path: String, status_code: u16, user_agent: Option<String> },
    ExternalCall { service: String, operation: String, details: Option<String> },
    ContentGenerated { content_type: String, topic: String },

    // Performance & Metrics
    PerformanceMetricRecorded { metric_name: String, value: f64 },
    PeriodicCheck { check_name: String },

    // Custom events for future extensibility
    Custom { name: String, payload: serde_json::Value },
}

/// A struct to represent a logged telemetry entry.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelemetryLogEntry {
    pub timestamp: DateTime<Utc>,
    pub event: TelemetryEvent,
}

/// The `TelemetryLogger` is responsible for capturing, processing, and outputting AGI telemetry data.
#[derive(Clone)]
pub struct TelemetryLogger {
    sender: mpsc::Sender<TelemetryLogEntry>,
}

impl TelemetryLogger {
    /// Creates a new `TelemetryLogger` instance.
    pub fn new(buffer_size: usize) -> Self {
        let (sender, _receiver) = mpsc::channel(buffer_size);
        TelemetryLogger { sender }
    }

    /// Check if logger is initialized (sender exists)
    pub fn is_initialized(&self) -> bool {
        !self.sender.is_closed()
    }

    /// Logs a telemetry event.
    pub async fn log_event(&self, event: TelemetryEvent) {
        let entry = TelemetryLogEntry {
            timestamp: Utc::now(),
            event,
        };
        if let Err(e) = self.sender.send(entry.clone()).await {
            eprintln!("TelemetryLogger: Failed to send event {:?}: {}", entry, e);
        }
    }

    /// Log an informational message
    pub async fn log_info(&self, source: &str, message: &str) {
        self.log_event(TelemetryEvent::AGIAction {
            action_type: format!("[{}] {}", source, message),
            details: None,
        }).await;
    }

    /// Log a warning message
    pub async fn log_warning(&self, source: &str, message: &str) {
        self.log_event(TelemetryEvent::SystemHealthWarning {
            component: source.to_string(),
            level: "warning".to_string(),
            message: message.to_string(),
        }).await;
    }

    /// Log an error message
    pub async fn log_error(&self, source: &str, message: &str) {
        self.log_event(TelemetryEvent::ErrorOccurred {
            message: format!("[{}] {}", source, message),
            details: None,
        }).await;
    }

    async fn process_log_entry(&self, entry: TelemetryLogEntry) {
        // In a real system, this would:
        // 1. Store the event in a database (e.g., MongoDB, Elasticsearch)
        // 2. Send to external monitoring systems (e.g., Prometheus, Grafana, SIEM)
        // 3. Trigger alerts based on event type/content
        // 4. Feed into AGI's self-reflection and learning modules
        // 5. Optionally, save to a persistent log file

        // For now, just print it to stdout/stderr
        match &entry.event {
            TelemetryEvent::ErrorOccurred { message, .. } | TelemetryEvent::SystemHealthWarning { message, .. } => {
                eprintln!("[TELEMETRY ERROR] {}: {:?}", entry.timestamp, entry.event);
            },
            _ => {
                println!("[TELEMETRY INFO] {}: {:?}", entry.timestamp, entry.event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_telemetry_logger_initialization() {
        let telemetry_logger = TelemetryLogger::new(10);
        assert!(telemetry_logger.is_initialized());
    }

    #[tokio::test]
    async fn test_telemetry_logger_logs_event() {
        let telemetry_logger = TelemetryLogger::new(10);
        let event = TelemetryEvent::SystemStarted;
        // Just verify it doesn't panic
        telemetry_logger.log_event(event.clone()).await;
    }

    #[tokio::test]
    async fn test_telemetry_logger_logs_multiple_events() {
        let telemetry_logger = TelemetryLogger::new(10);

        let event1 = TelemetryEvent::ServiceInitialized { service_name: "TestService1".to_string() };
        let event2 = TelemetryEvent::AGIAction { action_type: "Test".to_string(), details: None };

        telemetry_logger.log_event(event1.clone()).await;
        telemetry_logger.log_event(event2.clone()).await;
        // Just verify it doesn't panic
    }

    #[tokio::test]
    async fn test_telemetry_logger_logs_error_events() {
        let telemetry_logger = TelemetryLogger::new(10);

        let error_event = TelemetryEvent::ErrorOccurred {
            message: "Critical failure".to_string(),
            details: Some("Details...".to_string())
        };
        telemetry_logger.log_event(error_event.clone()).await;
        // Just verify it doesn't panic
    }
}
