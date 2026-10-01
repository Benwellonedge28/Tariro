/// Module for a Threat Detection and Pattern Recognition Engine in Rust.
///
/// This module provides conceptual components for analyzing real-time event data
/// to identify potential threats, anomalies, and significant patterns using rules and heuristics.

use crate::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Reusing the Event structure from the ingestion system for consistency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: String,
    pub timestamp: u64,
    pub event_type: String,
    pub source: String,
    pub payload: serde_json::Value,
}

/// Represents a detected threat or significant pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedThreat {
    pub threat_id: String,
    pub event_id: String,
    pub event_type: String,
    pub rule_triggered: String,
    pub severity: ThreatSeverity,
    pub description: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Defines different types of detection rules.
pub enum DetectionRule {
    FailedLoginAttempts {
        threshold: u32,
        time_window_seconds: u64,
    },
    HighCpuUsage {
        threshold_percent: f64,
    },
    SpecificKeyword {
        keyword: String,
        event_type: String,
    },
    // More complex rules (e.g., ML-based) would be added here
}

/// The Threat Detection Engine.
pub struct ThreatDetector {
    rules: Vec<DetectionRule>,
}

impl ThreatDetector {
    pub fn new(rules: Vec<DetectionRule>) -> Self {
        ThreatDetector { rules }
    }

    /// Analyzes a stream of events to detect threats based on configured rules.
    pub fn detect_threats(&self, events: &[Event]) -> Vec<DetectedThreat> {
        let mut detected_threats = Vec::new();

        for event in events {
            for rule in &self.rules {
                match rule {
                    DetectionRule::FailedLoginAttempts { threshold, time_window_seconds } => {
                        // Conceptual: In a real system, this would require stateful tracking
                        // of login attempts over time windows. For simplicity, we just check event type.
                        if event.event_type == "login_failed" {
                            println!("  [Detector]: Rule 'FailedLoginAttempts' matched event {}.
                                        (Conceptual: would check threshold over time window)", event.id);
                            detected_threats.push(DetectedThreat {
                                threat_id: format!("threat-{}-login", event.id),
                                event_id: event.id.clone(),
                                event_type: event.event_type.clone(),
                                rule_triggered: "FailedLoginAttempts".to_string(),
                                severity: ThreatSeverity::Medium,
                                description: format!("Possible brute-force attempt for user at {}", event.source),
                                timestamp: event.timestamp,
                            });
                        }
                    },
                    DetectionRule::HighCpuUsage { threshold_percent } => {
                        if event.event_type == "system_metric" && event.payload.get("cpu_usage").and_then(|v| v.as_f64()).map_or(false, |usage| usage > *threshold_percent) {
                            println!("  [Detector]: Rule 'HighCpuUsage' matched event {}.", event.id);
                            detected_threats.push(DetectedThreat {
                                threat_id: format!("threat-{}-cpu", event.id),
                                event_id: event.id.clone(),
                                event_type: event.event_type.clone(),
                                rule_triggered: "HighCpuUsage".to_string(),
                                severity: ThreatSeverity::Low,
                                description: format!("High CPU usage detected on {}", event.source),
                                timestamp: event.timestamp,
                            });
                        }
                    },
                    DetectionRule::SpecificKeyword { keyword, event_type: et } => {
                        if &event.event_type == et && event.payload.to_string().contains(keyword) {
                            println!("  [Detector]: Rule 'SpecificKeyword' matched event {}.
                                        (Keyword: '{}' in event type '{}')", event.id, keyword, et);
                            detected_threats.push(DetectedThreat {
                                threat_id: format!("threat-{}-keyword", event.id),
                                event_id: event.id.clone(),
                                event_type: event.event_type.clone(),
                                rule_triggered: format!("SpecificKeyword: {}", keyword),
                                severity: ThreatSeverity::High,
                                description: format!("Detected keyword '{}' in event from {}", keyword, event.source),
                                timestamp: event.timestamp,
                            });
                        }
                    }
                }
            }
        }
        detected_threats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_failed_login() {
        let rules = vec![DetectionRule::FailedLoginAttempts { threshold: 5, time_window_seconds: 60 }];
        let detector = ThreatDetector::new(rules);
        let event = Event { id: "e1".to_string(), timestamp: 1, event_type: "login_failed".to_string(), source: "auth".to_string(), payload: serde_json::Value::Null };
        let threats = detector.detect_threats(&[event]);
        assert_eq!(threats.len(), 1);
        assert_eq!(threats[0].rule_triggered, "FailedLoginAttempts");
    }

    #[test]
    fn test_detect_high_cpu() {
        let rules = vec![DetectionRule::HighCpuUsage { threshold_percent: 80.0 }];
        let detector = ThreatDetector::new(rules);
        let event = Event { id: "e2".to_string(), timestamp: 2, event_type: "system_metric".to_string(), source: "monitor".to_string(), payload: serde_json::json!({ "cpu_usage": 90.0 }) };
        let threats = detector.detect_threats(&[event]);
        assert_eq!(threats.len(), 1);
        assert_eq!(threats[0].rule_triggered, "HighCpuUsage");
    }

    #[test]
    fn test_no_threat_detected() {
        let rules = vec![DetectionRule::HighCpuUsage { threshold_percent: 90.0 }];
        let detector = ThreatDetector::new(rules);
        let event = Event { id: "e3".to_string(), timestamp: 3, event_type: "system_metric".to_string(), source: "monitor".to_string(), payload: serde_json::json!({ "cpu_usage": 75.0 }) };
        let threats = detector.detect_threats(&[event]);
        assert!(threats.is_empty());
    }

    #[test]
    fn test_specific_keyword() {
        let rules = vec![DetectionRule::SpecificKeyword { keyword: "malware".to_string(), event_type: "security_log".to_string() }];
        let detector = ThreatDetector::new(rules);
        let event = Event { id: "e4".to_string(), timestamp: 4, event_type: "security_log".to_string(), source: "ids".to_string(), payload: serde_json::json!({ "message": "detected malware signature" }) };
        let threats = detector.detect_threats(&[event]);
        assert_eq!(threats.len(), 1);
        assert_eq!(threats[0].rule_triggered, "SpecificKeyword: malware");
    }
}

fn main() {
    println!("\n--- Threat Detection Engine Demonstration ---");

    // 1. Define detection rules
    let rules = vec![
        DetectionRule::FailedLoginAttempts { threshold: 3, time_window_seconds: 120 },
        DetectionRule::HighCpuUsage { threshold_percent: 80.0 },
        DetectionRule::SpecificKeyword { keyword: "exploit".to_string(), event_type: "security_alert".to_string() },
        DetectionRule::SpecificKeyword { keyword: "ransomware".to_string(), event_type: "file_access_log".to_string() },
    ];
    let detector = ThreatDetector::new(rules);

    // 2. Simulate incoming events (from ingestion system)
    let incoming_events = vec![
        Event { id: "evt_001".to_string(), timestamp: 1714166400, event_type: "user_activity".to_string(), source: "web_app".to_string(), payload: serde_json::json!({ "action": "view_profile" }) },
        Event { id: "evt_002".to_string(), timestamp: 1714166405, event_type: "login_failed".to_string(), source: "auth_service_1".to_string(), payload: serde_json::json!({ "user": "admin", "ip": "192.168.1.10" }) },
        Event { id: "evt_003".to_string(), timestamp: 1714166410, event_type: "system_metric".to_string(), source: "server_mon_2".to_string(), payload: serde_json::json!({ "cpu_usage": 92.5, "memory_free": 1024 }) },
        Event { id: "evt_004".to_string(), timestamp: 1714166415, event_type: "file_access_log".to_string(), source: "file_server_3".to_string(), payload: serde_json::json!({ "file": "critical_db.sql", "action": "delete" }) },
        Event { id: "evt_005".to_string(), timestamp: 1714166420, event_type: "login_failed".to_string(), source: "auth_service_1".to_string(), payload: serde_json::json!({ "user": "admin", "ip": "192.168.1.10" }) },
        Event { id: "evt_006".to_string(), timestamp: 1714166425, event_type: "security_alert".to_string(), source: "ids_system".to_string(), payload: serde_json::json!({ "description": "Potential SQL exploit detected on web_app", "vector": "sql_injection" }) },
        Event { id: "evt_007".to_string(), timestamp: 1714166430, event_type: "file_access_log".to_string(), source: "file_server_3".to_string(), payload: serde_json::json!({ "file": "important_docs.pdf", "action": "encrypt", "details": "identified as ransomware activity" }) },
        Event { id: "evt_008".to_string(), timestamp: 1714166435, event_type: "login_failed".to_string(), source: "auth_service_2".to_string(), payload: serde_json::json!({ "user": "guest", "ip": "10.0.0.5" }) },
    ];

    // 3. Detect threats
    let detected_threats = detector.detect_threats(&incoming_events);

    // 4. Report detected threats
    if detected_threats.is_empty() {
        println!("  No threats detected in the event stream.");
    } else {
        println!("\n--- Detected Threats ({}) ---", detected_threats.len());
        for threat in detected_threats {
            println!("  Threat ID: {}", threat.threat_id);
            println!("    Type: '{}' (Severity: {:?})", threat.event_type, threat.severity);
            println!("    Rule: '{}'".to_string(), threat.rule_triggered);
            println!("    Desc: {}", threat.description);
            println!("    Time: {}", threat.timestamp);
            println!("------------------------------");
        }
    }
}
