// src/event_bus.rs

//! Implements an asynchronous event bus for inter-module communication within the Tariro AGI.

use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Define a generic AGIEvent enum that can encompass various system events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AGIEvent {
    // System Events
    SystemInitialized { version: String },
    SystemStarted,
    SystemShutdown,
    ErrorOccurred { message: String, details: Option<String> },

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
        //println!("[EventBus] Publishing event: {:?}", event);
        if let Err(e) = self.sender.send(event.clone()) {
            eprintln!("[EventBus] Failed to publish event: {:?}. Error: {}", event, e);
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

        event_bus.publish(event1.clone()).await;
        event_bus.publish(event2.clone()).await;

        // Receiver 1 should get both events
        assert_eq!(receiver1.recv().await.unwrap(), event1);
        assert_eq!(receiver1.recv().await.unwrap(), event2);

        // Receiver 2 should also get both events
        assert_eq!(receiver2.recv().await.unwrap(), event1);
        assert_eq!(receiver2.recv().await.unwrap(), event2);
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

        // A new subscriber will only receive events *after* it subscribes, plus any buffered ones.
        // With capacity 1, if sender sends two quickly, only the last one sent will be kept.
        // So, a new subscriber should get event2 (the latest one that fits buffer capacity).
        let event3 = AGIEvent::SystemInitialized { version: "1.0".to_string() };
        event_bus.publish(event3.clone()).await;

        // The receiver should get the latest event that fits in the buffer.
        // In a broadcast channel with capacity N, a new receiver will miss messages if N messages
        // have been sent *after* the receiver was created and *before* it tries to receive.
        // If the sender sends faster than receiver, some messages might be dropped if the buffer is full.

        // Let's test a simpler scenario: sender sends, then subscriber gets.
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
