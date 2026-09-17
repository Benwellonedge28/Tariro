use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents a single "thought" or internal cognitive event by Tariro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: String, // e.g., "DecisionPoint", "DataRetrieval", "HypothesisGeneration", "ActionPlanning"
    pub description: String,
    pub associated_data: HashMap<String, String>, // e.g., "decision_id": "xyz", "query_text": "abc"
    pub causal_links: Vec<String>, // IDs of preceding ThoughtEvents that influenced this one
    pub confidence_score: Option<f64>, // Confidence in the internal thought
}

// Represents a recorded sequence of Tariro's thought process for a specific task or decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtProcessRecording {
    pub recording_id: String,
    pub start_timestamp: DateTime<Utc>,
    pub end_timestamp: Option<DateTime<Utc>>,
    pub triggering_event: String, // e.g., "user_query_X", "autonomous_task_Y"
    pub thought_events: Vec<ThoughtEvent>,
    pub summary: String,
}

pub struct ThoughtRecorder {
    active_recordings: Mutex<HashMap<String, ThoughtProcessRecording>>, // key: triggering_event_id
    completed_recordings: Mutex<Vec<ThoughtProcessRecording>>,
}

impl ThoughtRecorder {
    pub fn new() -> Self {
        ThoughtRecorder {
            active_recordings: Mutex::new(HashMap::new()),
            completed_recordings: Mutex::new(Vec::new()),
        }
    }

    // Starts recording a new thought process
    pub fn start_recording(&self, triggering_event_id: &str, description: &str) -> String {
        let recording_id = Uuid::new_v4().to_string();
        let new_recording = ThoughtProcessRecording {
            recording_id: recording_id.clone(),
            start_timestamp: Utc::now(),
            end_timestamp: None,
            triggering_event: triggering_event_id.to_string(),
            thought_events: Vec::new(),
            summary: format!("Recording initiated for: {}", description),
        };
        self.active_recordings.lock().unwrap().insert(triggering_event_id.to_string(), new_recording);
        println!("Recording started for event: {}", triggering_event_id);
        recording_id
    }

    // Logs a new thought event to an active recording
    pub fn log_thought_event(&self, triggering_event_id: &str, event: ThoughtEvent) {
        if let Some(recording) = self.active_recordings.lock().unwrap().get_mut(triggering_event_id) {
            recording.thought_events.push(event);
        } else {
            println!("Warning: Attempted to log event for non-existent active recording: {}", triggering_event_id);
        }
    }

    // Ends an active recording and moves it to completed_recordings
    pub fn end_recording(&self, triggering_event_id: &str, final_summary: &str) -> Option<ThoughtProcessRecording> {
        if let Some(mut recording) = self.active_recordings.lock().unwrap().remove(triggering_event_id) {
            recording.end_timestamp = Some(Utc::now());
            recording.summary = final_summary.to_string();
            println!("Recording ended for event: {}. Summary: {}", triggering_event_id, final_summary);
            let mut completed = self.completed_recordings.lock().unwrap();
            completed.push(recording.clone());
            Some(recording)
        } else {
            println!("Warning: Attempted to end non-existent active recording: {}", triggering_event_id);
            None
        }
    }

    pub fn get_completed_recordings(&self) -> Vec<ThoughtProcessRecording> {
        self.completed_recordings.lock().unwrap().clone()
    }
}

pub struct ThoughtPlaybackEngine {
    recorded_processes: Arc<ThoughtRecorder>,
}

impl ThoughtPlaybackEngine {
    pub fn new(recorded_processes: Arc<ThoughtRecorder>) -> Self {
        ThoughtPlaybackEngine { recorded_processes }
    }

    // Retrieves and "plays back" a recorded thought process
    pub fn playback_recording(&self, recording_id: &str) -> Option<Vec<ThoughtEvent>> {
        let all_recordings = self.recorded_processes.get_completed_recordings();
        if let Some(recording) = all_recordings.iter().find(|r| r.recording_id == recording_id) {
            println!("Playing back recording: {}", recording.recording_id);
            // In a real UI, this would stream events or present them in a visual timeline
            for event in &recording.thought_events {
                println!("[{:?}] ({})\n: {}", event.timestamp, event.event_type, event.description);
                // Simulate pausing for interactive review if in a UI
                // std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Some(recording.thought_events.clone())
        } else {
            println!("Recording not found for playback: {}", recording_id);
            None
        }
    }

    // Provides a summary view of available recordings
    pub fn list_recording_summaries(&self) -> Vec<HashMap<String, String>> {
        self.recorded_processes.get_completed_recordings().iter()
            .map(|r| {
                HashMap::from([
                    ("id".to_string(), r.recording_id.clone()),
                    ("trigger".to_string(), r.triggering_event.clone()),
                    ("summary".to_string(), r.summary.clone()),
                    ("start_time".to_string(), r.start_timestamp.to_rfc3339()),
                    ("event_count".to_string(), r.thought_events.len().to_string()),
                ])
            })
            .collect()
    }
}


/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_thought_recorder_playback() {
        let recorder = Arc::new(ThoughtRecorder::new());
        let playback_engine = ThoughtPlaybackEngine::new(recorder.clone());

        let event_id = "user_query_123".to_string();
        let recording_id = recorder.start_recording(&event_id, "User asked about system status");

        recorder.log_thought_event(&event_id, ThoughtEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: "DataRetrieval".to_string(),
            description: "Retrieved current CPU load from system metrics.".to_string(),
            associated_data: HashMap::from([("metric".to_string(), "cpu_load".to_string())]),
            causal_links: vec![],
            confidence_score: Some(0.98),
        });

        recorder.log_thought_event(&event_id, ThoughtEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() + Duration::seconds(1),
            event_type: "DecisionPoint".to_string(),
            description: "Decided to report current CPU load to user.".to_string(),
            associated_data: HashMap::new(),
            causal_links: vec!["DataRetrieval_cpu_load".to_string()], // Placeholder link
            confidence_score: Some(0.95),
        });

        let completed_recording = recorder.end_recording(&event_id, "Provided system status update.");
        assert!(completed_recording.is_some());
        assert_eq!(completed_recording.unwrap().thought_events.len(), 2);

        let summaries = playback_engine.list_recording_summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0]["id"], recording_id);

        let played_events = playback_engine.playback_recording(&recording_id);
        assert!(played_events.is_some());
        assert_eq!(played_events.unwrap().len(), 2);
    }
}
*/