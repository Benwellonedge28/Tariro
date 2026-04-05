// src/telemetry.rs

//! Handles all telemetry, logging, and monitoring aspects of the Tariro AGI.

use chrono::{DateTime, Utc};
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
pub struct TelemetryLogger {
    sender: mpsc::Sender<TelemetryLogEntry>,
    receiver: mpsc::Receiver<TelemetryLogEntry>, // Owned by the processing task
}

impl TelemetryLogger {
    /// Creates a new `TelemetryLogger` instance.
    pub fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel(buffer_size);
        TelemetryLogger { sender, receiver }
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

    /// Main loop for processing telemetry events.
    pub async fn process_events_loop(mut self) {
        println!("[TelemetryLogger] Starting event processing loop.");
        while let Some(entry) = self.receiver.recv().await {
            self.process_log_entry(entry).await;
        }
        println!("[TelemetryLogger] Event processing loop stopped.");
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
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn test_telemetry_logger_logs_event() {
        let telemetry_logger = TelemetryLogger::new(10);
        let mut receiver = telemetry_logger.receiver;
        
        let event = TelemetryEvent::SystemStarted;
        telemetry_logger.log_event(event.clone()).await;

        // The event should be received by the internal receiver
        let received_entry = time::timeout(Duration::from_millis(100),
                                          receiver.recv()).await.unwrap().unwrap();
        assert_eq!(received_entry.event, event);
    }

    #[tokio::test]
    async fn test_telemetry_logger_processes_multiple_events() {
        let telemetry_logger = TelemetryLogger::new(10);
        let mut receiver = telemetry_logger.receiver;

        let event1 = TelemetryEvent::ServiceInitialized { service_name: "TestService1".to_string() };
        let event2 = TelemetryEvent::AGIAction { action_type: "Test".to_string(), details: None };
        let event3 = TelemetryEvent::ErrorOccurred { message: "Something went wrong".to_string(), details: None };

        telemetry_logger.log_event(event1.clone()).await;
        telemetry_logger.log_event(event2.clone()).await;
        telemetry_logger.log_event(event3.clone()).await;

        let received_entry1 = time::timeout(Duration::from_millis(100),
                                          receiver.recv()).await.unwrap().unwrap();
        let received_entry2 = time::timeout(Duration::from_millis(100),
                                          receiver.recv()).await.unwrap().unwrap();
        let received_entry3 = time::timeout(Duration::from_millis(100),
                                          receiver.recv()).await.unwrap().unwrap();

        assert_eq!(received_entry1.event, event1);
        assert_eq!(received_entry2.event, event2);
        assert_eq!(received_entry3.event, event3);
    }

    #[tokio::test]
    async fn test_telemetry_logger_handles_error_events() {
        let telemetry_logger = TelemetryLogger::new(10);
        let mut receiver = telemetry_logger.receiver;

        let error_event = TelemetryEvent::ErrorOccurred { message: "Critical failure".to_string(), details: Some("Details...".to_string()) };
        telemetry_logger.log_event(error_event.clone()).await;

        let received_entry = time::timeout(Duration::from_millis(100),
                                          receiver.recv()).await.unwrap().unwrap();
        assert_eq!(received_entry.event, error_event);
        // In a real test, you'd capture stderr to ensure it was printed as error
    }
}
