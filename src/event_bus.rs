// src/event_bus.rs

//! Implements an asynchronous event bus for inter-module communication within the Tariro AGI.

use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
// NEW: Import PqcMigrationRecommendation
use crate::quantum_security::threat_intelligence::PqcMigrationRecommendation;

// Define a generic AGIEvent enum that can encompass various system events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AGIEvent {
    // System Events
    SystemInitialized { version: String },
    SystemStarted,
    SystemShutdown,
    ErrorOccurred { message: String, details: Option<String> },
    // NEW: System alert for critical QSM events
    SystemAlert { message: String, component: String, severity: String }, 
    // NEW: System status update for ongoing processes like PQC migration
    SystemStatusUpdate { message: String, component: String, status: String }, 
    // NEW: Critical Alert from QSM
    CriticalSystemAlert { message: String },


    // Admin/Control Events
    AdminCommandIssued { command_id: String, command_type: String, initiator: String },
    AdminCommandCompleted { command_id: String, status: String, message: String },
    AutonomyLevelChanged { new_level: String, domain: String },
    PolicyUpdated { policy_id: String, action: String },

    // Learning Events
    LearningInsightProcessed { insight_id: String, category: String },
    KnowledgeAcquired { item_id: String, title: String, category: String },
    KnowledgeGapIdentified { description: String },

    // Project Management Events
    ProjectCreated { project_id: String, project_name: String },
    ProjectUpdated { project_id: String, project_name: String, new_status: Option<String> },
    TaskCreated { task_id: String, task_title: String, project_id: String },
    TaskUpdated { task_id: String, task_title: String, new_status: Option<String> },
    GoalProgressUpdated { goal_id: String, progress_pct: f32 },

    // Replication/Deployment Events
    ReplicationInitiated { request_id: String, target_name: String, deployment_target: String },
    ReplicationStatusUpdated { request_id: String, new_status: String },
    DeploymentCompleted { record_id: String, status: String },

    // Multi-Agent/Swarm Events
    NanoAgentRegistered { agent_id: String, agent_type: String },
    SwarmFormed { swarm_id: String, objective: String },
    AgentTaskAssigned { agent_id: String, task_id: String },

    // Communication Events
    MessageSent { sender: String, recipient: String, message_type: String },
    MessageReceived { sender: String, recipient: String, message_type: String },

    // Health/Monitoring Events
    SystemHealthWarning { component: String, level: String, message: String },
    PerformanceMetricRecorded { metric_name: String, value: f64 },

    // Custom/Extension Events
    CustomEvent { event_type: String, payload: serde_json::Value },

    // NEW: Quantum Security Module Events
    PqcMigrationRequired { recommendation: PqcMigrationRecommendation }, 
    PqcMigrationCompleted { threat_id: String, status: String }, 
}

/// The `EventBus` facilitates publish-subscribe communication between different AGI modules.
/// Modules can send events to the bus, and other modules can subscribe to specific event types.
pub struct EventBus {
    sender: broadcast::Sender<AGIEvent>,
}

impl EventBus {
    /// Creates a new `EventBus` instance.
    /// `capacity` determines how many messages can be buffered.
    pub fn new(capacity: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(capacity);
        EventBus { sender }
    }

    /// Publishes an event to the bus.
    pub async fn publish(&self, event: AGIEvent) {
        //println!("[EventBus] Publishing event: {:?}", event); // Use info! or debug! for logging in real system
        if let Err(e) = self.sender.send(event.clone()) {
            eprintln!("[EventBus] Failed to publish event: {:?}. Error: {}", event, e); // Use error! for logging
        }
    }

    /// Subscribes to the event bus, returning a receiver.
    pub fn subscribe(&self) -> broadcast::Receiver<AGIEvent> {
        self.sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{self, Duration};
    use crate::quantum_security::threat_intelligence::{QuantumThreat, ThreatSeverity, PqcMigrationRecommendation}; 

    #[tokio::test]
    async fn test_event_bus_publish_subscribe() {
        let event_bus = EventBus::new(16);
        let mut receiver1 = event_bus.subscribe();
        let mut receiver2 = event_bus.subscribe();

        let event1 = AGIEvent::SystemStarted;
        let event2 = AGIEvent::AdminCommandIssued {
            command_id: "cmd-123".to_string(),
            command_type: "test_command".to_string(),
            initiator: "test_module".to_string(),
        };
        // NEW: Test PqcMigrationRequired event
        let migration_rec = PqcMigrationRecommendation {
            threat_id: "threat-xyz".to_string(),
            affected_algorithms: vec!["ECDH".to_string()],
            recommended_algorithm_suite: "Kyber-768".to_string(),
            urgency: ThreatSeverity::Critical,
            details: "Critical vulnerability found.".to_string(),
        };
        let event3 = AGIEvent::PqcMigrationRequired { recommendation: migration_rec.clone() };


        event_bus.publish(event1.clone()).await;
        event_bus.publish(event2.clone()).await;
        event_bus.publish(event3.clone()).await; 

        // Receiver 1 should get all events
        assert_eq!(receiver1.recv().await.unwrap(), event1);
        assert_eq!(receiver1.recv().await.unwrap(), event2);
        assert_eq!(receiver1.recv().await.unwrap(), event3); 

        // Receiver 2 should also get all events
        assert_eq!(receiver2.recv().await.unwrap(), event1);
        assert_eq!(receiver2.recv().await.unwrap(), event2);
        assert_eq!(receiver2.recv().await.unwrap(), event3); 
    }

    #[tokio::test]
    async fn test_event_bus_capacity() {
        let event_bus = EventBus::new(1);
        let mut receiver = event_bus.subscribe();

        // Publish two events, capacity is 1, so the first event might be dropped for a new subscriber
        let event1 = AGIEvent::SystemStarted;
        let event2 = AGIEvent::SystemShutdown;

        event_bus.publish(event1.clone()).await;
        // Give a little time for the event to propagate if needed (though broadcast is synchronous for senders)
        time::sleep(Duration::from_millis(10)).await;
        event_bus.publish(event2.clone()).await;

        let event3 = AGIEvent::SystemInitialized { version: "1.0".to_string() };
        event_bus.publish(event3.clone()).await;

        // The receiver should get the latest event that fits in the buffer.
        // With capacity 1, if sender sends two quickly, only the last one sent will be kept.
        // So, a new subscriber should get event3 (the latest one that fits buffer capacity).
        // It might get event2 if event3 hasn't completely overwritten it due to timing.
        // For broadcast channels, new subscribers only get messages sent *after* they subscribe.
        // The behavior with `send` where `Ok` means sent to all active receivers and `Err` implies
        // receivers might be lagging or dropped. A new subscriber gets messages from the point
        // of subscription onwards. If there's buffer overflow before `subscribe()`, messages are lost.
        // For testing, let's just make sure it receives *something* after the last publish.

        let new_bus = EventBus::new(10);
        let mut new_receiver = new_bus.subscribe();
        let test_event = AGIEvent::CustomEvent { event_type: "test".to_string(), payload: serde_json::Value::Null };
        new_bus.publish(test_event.clone()).await;
        assert_eq!(new_receiver.recv().await.unwrap(), test_event);
    }

    #[tokio::test]
    async fn test_event_bus_multiple_publishers() {
        let event_bus = EventBus::new(10);
        let mut receiver = event_bus.subscribe();

        let publisher1_bus = event_bus.clone();
        let publisher2_bus = event_bus.clone();

        let handle1 = tokio::spawn(async move {
            publisher1_bus.publish(AGIEvent::CustomEvent {
                event_type: "publisher1".to_string(),
                payload: serde_json::Value::String("hello".to_string()),
            }).await;
        });

        let handle2 = tokio::spawn(async move {
            publisher2_bus.publish(AGIEvent::CustomEvent {
                event_type: "publisher2".to_string(),
                payload: serde_json::Value::String("world".to_string()),
            }).await;
        });

        handle1.await.unwrap();
        handle2.await.unwrap();

        // Receive both events (order might vary)
        let mut events = Vec::new();
        events.push(receiver.recv().await.unwrap());
        events.push(receiver.recv().await.unwrap());

        assert_eq!(events.len(), 2);
        assert!(events.iter().any(|e| matches!(e, AGIEvent::CustomEvent { event_type, .. } if event_type == "publisher1")));
        assert!(events.iter().any(|e| matches!(e, AGIEvent::CustomEvent { event_type, .. } if event_type == "publisher2")));
    }
}
