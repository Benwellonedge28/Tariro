/// Module for Real-time Alerting and Situational Dashboard in Rust.
///
/// This module provides conceptual components for generating real-time alerts
/// based on detected threats and simulating a situational awareness dashboard.

use crate::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Reusing the DetectedThreat structure from the threat detection module.
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

/// Represents an active alert.
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveAlert {
    pub alert_id: String,
    pub threat_id: String,
    pub severity: ThreatSeverity,
    pub message: String,
    pub triggered_at: u64,
    pub status: AlertStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
}

/// Manages the generation and lifecycle of alerts.
pub struct AlertManager {
    active_alerts: Arc<Mutex<HashMap<String, ActiveAlert>>>,
    alert_count: Arc<Mutex<usize>>,
}

impl AlertManager {
    pub fn new() -> Self {
        AlertManager {
            active_alerts: Arc::new(Mutex::new(HashMap::new())),
            alert_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Generates an alert based on a detected threat.
    pub fn generate_alert(&self, threat: DetectedThreat) -> ActiveAlert {
        let mut count = self.alert_count.lock().unwrap();
        *count += 1;
        let alert_id = format!("alert_{}", *count);
        let triggered_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let alert = ActiveAlert {
            alert_id: alert_id.clone(),
            threat_id: threat.threat_id.clone(),
            severity: threat.severity.clone(),
            message: format!("Threat Detected: {} (Severity: {:?}) - {}", threat.event_type, threat.severity, threat.description),
            triggered_at,
            status: AlertStatus::Active,
        };

        self.active_alerts.lock().unwrap().insert(alert_id.clone(), alert.clone());
        println!("  [AlertManager]: !!! NEW ALERT !!! ID: {}, Threat: {}", alert.alert_id, alert.message);
        alert
    }

    /// Simulates acknowledging an alert.
    pub fn acknowledge_alert(&self, alert_id: &str) -> Option<ActiveAlert> {
        let mut alerts = self.active_alerts.lock().unwrap();
        if let Some(alert) = alerts.get_mut(alert_id) {
            alert.status = AlertStatus::Acknowledged;
            println!("  [AlertManager]: Alert {} Acknowledged.", alert_id);
            Some(alert.clone())
        } else {
            None
        }
    }

    /// Gets all currently active alerts.
    pub fn get_active_alerts(&self) -> Vec<ActiveAlert> {
        self.active_alerts.lock().unwrap().values().cloned().filter(|a| a.status == AlertStatus::Active).collect()
    }
}

/// Conceptually represents a real-time dashboard for situational awareness.
pub struct DashboardSimulator {
    alert_manager: Arc<AlertManager>,
}

impl DashboardSimulator {
    pub fn new(alert_manager: Arc<AlertManager>) -> Self {
        DashboardSimulator { alert_manager }
    }

    /// Simulates rendering the dashboard with current alerts.
    pub fn render_dashboard(&self) {
        let active_alerts = self.alert_manager.get_active_alerts();
        println!("\n--- Situational Awareness Dashboard (Live) ---");
        println!("  Current Time: {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        println!("  Total Active Alerts: {}", active_alerts.len());

        if active_alerts.is_empty() {
            println!("  No active threats detected.");
        } else {
            for alert in active_alerts {
                println!("    [{} | {:?}] {}", alert.alert_id, alert.severity, alert.message);
            }
        }
        println!("----------------------------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_threat(id: &str, event_type: &str, severity: ThreatSeverity) -> DetectedThreat {
        DetectedThreat {
            threat_id: format!("threat_{}", id),
            event_id: format!("event_{}", id),
            event_type: event_type.to_string(),
            rule_triggered: "test_rule".to_string(),
            severity,
            description: "Mock threat description.".to_string(),
            timestamp: 0,
        }
    }

    #[test]
    fn test_generate_alert() {
        let alert_manager = AlertManager::new();
        let threat = create_mock_threat("001", "malware", ThreatSeverity::High);
        let alert = alert_manager.generate_alert(threat);
        assert_eq!(alert.status, AlertStatus::Active);
        assert_eq!(alert_manager.get_active_alerts().len(), 1);
    }

    #[test]
    fn test_acknowledge_alert() {
        let alert_manager = AlertManager::new();
        let threat = create_mock_threat("002", "phishing", ThreatSeverity::Medium);
        let alert = alert_manager.generate_alert(threat);
        let acknowledged_alert = alert_manager.acknowledge_alert(&alert.alert_id).unwrap();
        assert_eq!(acknowledged_alert.status, AlertStatus::Acknowledged);
        assert_eq!(alert_manager.get_active_alerts().len(), 0); // Acknowledged alerts are not active
    }

    #[test]
    fn test_dashboard_render() {
        let alert_manager = Arc::new(AlertManager::new());
        let dashboard = DashboardSimulator::new(alert_manager.clone());
        let threat = create_mock_threat("003", "ddos", ThreatSeverity::Critical);
        alert_manager.generate_alert(threat);

        // This test primarily checks if it runs without panic and prints output.
        // Actual output assertion would be complex.
        dashboard.render_dashboard();
        assert!(!alert_manager.get_active_alerts().is_empty());
    }
}

fn main() {
    println!("\n--- Real-time Alerting and Situational Dashboard Demonstration ---");

    let alert_manager = Arc::new(AlertManager::new());
    let dashboard = DashboardSimulator::new(alert_manager.clone());

    // Simulate incoming threats over time
    let simulated_threats = vec![
        DetectedThreat {
            threat_id: "th_001".to_string(), event_id: "evt_002".to_string(), event_type: "login_failed".to_string(),
            rule_triggered: "FailedLoginAttempts".to_string(), severity: ThreatSeverity::Medium,
            description: "Multiple failed login attempts from suspicious IP.".to_string(), timestamp: 1714166405,
        },
        DetectedThreat {
            threat_id: "th_002".to_string(), event_id: "evt_003".to_string(), event_type: "system_metric".to_string(),
            rule_triggered: "HighCpuUsage".to_string(), severity: ThreatSeverity::Low,
            description: "Sustained high CPU usage on critical server.".to_string(), timestamp: 1714166410,
        },
        DetectedThreat {
            threat_id: "th_003".to_string(), event_id: "evt_006".to_string(), event_type: "security_alert".to_string(),
            rule_triggered: "SpecificKeyword: exploit".to_string(), severity: ThreatSeverity::High,
            description: "Exploit attempt detected targeting known vulnerability.".to_string(), timestamp: 1714166425,
        },
    ];

    for (i, threat) in simulated_threats.into_iter().enumerate() {
        println!("\n--- Step {} ---", i + 1);
        alert_manager.generate_alert(threat);
        dashboard.render_dashboard();
        std::thread::sleep(std::time::Duration::from_millis(100)); // Simulate time passing
    }

    println!("\n--- Final Status ---");
    alert_manager.acknowledge_alert("alert_1");
    dashboard.render_dashboard();

    println!("\nDemonstration Complete.");
}
