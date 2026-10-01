/// Module for a Real-time Event Data Ingestion System in Rust.
///
/// This module provides conceptual components for ingesting, parsing, and
/// buffering real-time event data from various sources for situational awareness.

use crate::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

/// Represents a generic event structure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: String,
    pub timestamp: u64,
    pub event_type: String,
    pub source: String,
    pub payload: serde_json::Value,
}

/// Configuration for the EventIngestor.
pub struct EventIngestorConfig {
    pub buffer_capacity: usize,
    pub enable_validation: bool,
}

impl Default for EventIngestorConfig {
    fn default() -> Self {
        EventIngestorConfig {
            buffer_capacity: 1000,
            enable_validation: true,
        }
    }
}

/// The core Event Ingestor.
pub struct EventIngestor {
    config: EventIngestorConfig,
    event_buffer: Arc<Mutex<VecDeque<Event>>>, // A buffer to hold ingested events
}

impl EventIngestor {
    pub fn new(config: EventIngestorConfig) -> Self {
        EventIngestor {
            config,
            event_buffer: Arc::new(Mutex::new(VecDeque::with_capacity(config.buffer_capacity))),
        }
    }

    /// Simulates receiving a raw event as a string (e.g., JSON).
    pub fn receive_raw_event(&self, raw_event_data: &str) -> Result<Event, String> {
        println!("  [Ingestor]: Received raw event: {}", raw_event_data);
        // In a real system, this might involve network I/O.

        let parsed_event: Event = serde_json::from_str(raw_event_data)
            .map_err(|e| format!("Failed to parse event JSON: {}", e))?;

        if self.config.enable_validation {
            self.validate_event(&parsed_event)?;
        }

        Ok(parsed_event)
    }

    /// Performs basic validation of the event.
    fn validate_event(&self, event: &Event) -> Result<(), String> {
        if event.id.is_empty() {
            return Err("Event ID cannot be empty.".to_string());
        }
        if event.timestamp == 0 {
            return Err("Event timestamp cannot be zero.".to_string());
        }
        if event.event_type.is_empty() {
            return Err("Event type cannot be empty.".to_string());
        }
        if event.source.is_empty() {
            return Err("Event source cannot be empty.".to_string());
        }
        // More complex validation logic would go here
        Ok(())
    }

    /// Conceptually stores the processed event into an internal buffer.
    pub fn store_event(&self, event: Event) -> Result<(), String> {
        let mut buffer = self.event_buffer.lock().map_err(|e| format!("Failed to lock event buffer: {}", e.to_string()))?;
        if buffer.len() >= self.config.buffer_capacity {
            buffer.pop_front(); // Remove oldest event if buffer is full
        }
        buffer.push_back(event);
        println!("  [Ingestor]: Stored event. Buffer size: {}", buffer.len());
        Ok(())
    }

    /// Retrieves and clears all events currently in the buffer.
    pub fn retrieve_all_events(&self) -> Vec<Event> {
        let mut buffer = self.event_buffer.lock().unwrap();
        buffer.drain(..).collect()
    }

    /// Returns the current number of events in the buffer.
    pub fn buffered_event_count(&self) -> usize {
        self.event_buffer.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_ingestor_new() {
        let config = EventIngestorConfig::default();
        let ingestor = EventIngestor::new(config);
        assert_eq!(ingestor.buffered_event_count(), 0);
    }

    #[test]
    fn test_receive_and_store_event() {
        let ingestor = EventIngestor::new(EventIngestorConfig::default());
        let raw_event = r#"{"id":"e1","timestamp":1678886400,"event_type":"login","source":"auth_service","payload":{"user":"test_user"}}"#;
        let event = ingestor.receive_raw_event(raw_event).unwrap();
        ingestor.store_event(event).unwrap();
        assert_eq!(ingestor.buffered_event_count(), 1);
    }

    #[test]
    fn test_buffer_capacity() {
        let config = EventIngestorConfig { buffer_capacity: 2, ..Default::default() };
        let ingestor = EventIngestor::new(config);

        let event1 = Event { id: "e1".to_string(), timestamp: 1, event_type: "t1".to_string(), source: "s1".to_string(), payload: serde_json::Value::Null };
        let event2 = Event { id: "e2".to_string(), timestamp: 2, event_type: "t2".to_string(), source: "s2".to_string(), payload: serde_json::Value::Null };
        let event3 = Event { id: "e3".to_string(), timestamp: 3, event_type: "t3".to_string(), source: "s3".to_string(), payload: serde_json::Value::Null };

        ingestor.store_event(event1.clone()).unwrap();
        ingestor.store_event(event2.clone()).unwrap();
        assert_eq!(ingestor.buffered_event_count(), 2);

        ingestor.store_event(event3.clone()).unwrap(); // Buffer is full, e1 should be removed
        assert_eq!(ingestor.buffered_event_count(), 2);

        let buffered_events = ingestor.retrieve_all_events();
        assert_eq!(buffered_events.len(), 2);
        assert!(!buffered_events.contains(&event1));
        assert!(buffered_events.contains(&event2));
        assert!(buffered_events.contains(&event3));
    }

    #[test]
    fn test_invalid_event() {
        let ingestor = EventIngestor::new(EventIngestorConfig::default());
        let raw_invalid_event = r#"{"timestamp":1678886400,"event_type":"login"}"#; // Missing ID and source
        let result = ingestor.receive_raw_event(raw_invalid_event);
        assert!(result.is_err());
    }
}

fn main() {
    let config = EventIngestorConfig::default();
    let ingestor = EventIngestor::new(config);

    println!("\n--- Event Ingestion System Demonstration ---");

    // Simulate receiving various events
    let events_to_ingest = vec![
        r#"{"id":"threat_alert_001","timestamp":1714166400,"event_type":"security_alert","source":"firewall_log","payload":{"severity":"high","rule":"suspicious_ip"}}"#,
        r#"{"id":"user_login_001","timestamp":1714166405,"event_type":"user_activity","source":"auth_service","payload":{"user_id":"alice","status":"success"}}"#,
        r#"{"id":"system_cpu_001","timestamp":1714166410,"event_type":"system_metric","source":"monitoring_agent","payload":{"cpu_usage":85.5,"temp_c":65}}"#,
        r#"{"id":"threat_alert_002","timestamp":1714166415,"event_type":"security_alert","source":"ids_system","payload":{"signature":"malware_payload","target":"server_01"}}"#,
    ];

    for (i, raw_event) in events_to_ingest.into_iter().enumerate() {
        match ingestor.receive_raw_event(raw_event) {
            Ok(event) => {
                if let Err(e) = ingestor.store_event(event) {
                    eprintln!("Error storing event: {}", e);
                }
            }
            Err(e) => eprintln!("Error receiving/processing event {}: {}", i + 1, e),
        }
        std::thread::sleep(std::time::Duration::from_millis(50)); // Simulate real-time delay
    }

    // Simulate invalid event
    let invalid_event = r#"{"timestamp":1714166420,"event_type":"malformed","source":"bad_source"}"#; // Missing ID
    match ingestor.receive_raw_event(invalid_event) {
        Ok(event) => { /* should not happen */ }
        Err(e) => eprintln!("\nAttempted to ingest invalid event. Error: {}\n", e),
    }

    println!("\nAll events ingested. Final buffer size: {}", ingestor.buffered_event_count());
    let all_events = ingestor.retrieve_all_events();
    println!("Retrieved {} events from buffer.", all_events.len());
    for event in all_events {
        println!("  - Event ID: {}, Type: {}, Source: {}", event.id, event.event_type, event.source);
    }
}
