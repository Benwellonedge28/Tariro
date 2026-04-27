/// Module for Adaptive Threat Intelligence & Response in Rust.
///
/// This module conceptually outlines a system where the AGI continuously
/// monitors for emergent threats, detects sophisticated anomalies, and
/// autonomously formulates and executes proactive countermeasures. It goes
/// beyond static defenses, providing dynamic, adaptive security capabilities
/// including real-time policy adjustments and system hardening.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::thread;
use std::time::Duration;
use rand::Rng;

/// Represents various types of detected threat events.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThreatEvent {
    MalwareSignatureDetected { signature: String, source: String },
    AnomalousBehavior { user_id: String, activity: String, deviation_score: f32 },
    NetworkIntrusionAttempt { ip_address: String, port: u16 },
    QuantumAttackDetected { algorithm: String, target_system: String },
    ZeroDayExploit { exploit_name: String },
}

impl fmt::Display for ThreatEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Defines potential countermeasures for detected threats.
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseAction {
    QuarantineSystem { system_id: String },
    UpdateFirewallRules { rule: String, action: String },
    RotateKeys { system_id: String, key_type: String },
    IsolateNetworkSegment { segment_id: String },
    AlertHumanOperator { priority: u8, message: String },
    DynamicPolicyAdjustment { policy_id: String, new_param: String },
}

impl fmt::Display for ResponseAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Simulates external threat intelligence feeds and internal knowledge base.
pub struct ThreatIntelligence {
    pub known_signatures: HashSet<String>,
    pub known_anomalous_patterns: HashSet<String>,
    pub quantum_vulnerability_db: HashMap<String, String>, // Algo -> patch_version
}

impl ThreatIntelligence {
    pub fn new() -> Self {
        let mut known_signatures = HashSet::new();
        known_signatures.insert("MALWARE_X_V1".to_string());
        known_signatures.insert("RANSOMWARE_Y_V2".to_string());

        let mut known_anomalous_patterns = HashSet::new();
        known_anomalous_patterns.insert("UNUSUAL_LOGIN_LOCATION".to_string());

        let mut quantum_vulnerability_db = HashMap::new();
        quantum_vulnerability_db.insert("RSA-2048".to_string(), "Upgrade to Kyber or Dilithium".to_string());

        ThreatIntelligence {
            known_signatures,
            known_anomalous_patterns,
            quantum_vulnerability_db,
        }
    }

    pub fn has_signature(&self, signature: &str) -> bool {
        self.known_signatures.contains(signature)
    }
    pub fn is_anomalous_pattern(&self, pattern: &str) -> bool {
        self.known_anomalous_patterns.contains(pattern)
    }
    pub fn is_quantum_vulnerable(&self, algorithm: &str) -> Option<&String> {
        self.quantum_vulnerability_db.get(algorithm)
    }
}

/// The Adaptive Threat Intelligence & Response Manager.
pub struct ThreatResponseManager {
    pub intelligence: ThreatIntelligence,
    pub active_responses: Vec<ResponseAction>,
}

impl ThreatResponseManager {
    pub fn new() -> Self {
        ThreatResponseManager {
            intelligence: ThreatIntelligence::new(),
            active_responses: Vec::new(),
        }
    }

    /// Ingests and processes raw threat data.
    pub fn ingest_threat_data(&self, data: &str) {
        println!("  [ThreatManager]: Ingesting raw threat data: \"{}\". (Simulated)", data);
        // In a real system, this would parse logs, network traffic, etc.
    }

    /// Detects anomalies based on ingested data and threat intelligence.
    pub fn detect_anomaly(&self, event: &ThreatEvent) -> Option<String> {
        println!("  [ThreatManager]: Detecting anomaly for event: {}.", event);
        match event {
            ThreatEvent::MalwareSignatureDetected { signature, .. } => {
                if self.intelligence.has_signature(signature) {
                    return Some(format!("Known malware signature '{}' detected.", signature));
                }
            },
            ThreatEvent::AnomalousBehavior { activity, deviation_score, .. } => {
                if *deviation_score > 0.8 && self.intelligence.is_anomalous_pattern(activity) {
                    return Some(format!("Highly anomalous behavior pattern '{}' detected.", activity));
                }
            },
            ThreatEvent::QuantumAttackDetected { algorithm, .. } => {
                if self.intelligence.is_quantum_vulnerable(algorithm).is_some() {
                    return Some(format!("Quantum attack against vulnerable algorithm '{}' detected.", algorithm));
                }
            },
            _ => {},
        }
        None
    }

    /// Formulates a response based on the detected anomaly and system context.
    pub fn formulate_response(&self, anomaly_description: &str, rng: &mut impl Rng) -> Vec<ResponseAction> {
        println!("  [ThreatManager]: Formulating response for: {}.", anomaly_description);
        let mut actions = Vec::new();

        if anomaly_description.contains("malware signature") {
            actions.push(ResponseAction::QuarantineSystem { system_id: "infected_host_1".to_string() });
            actions.push(ResponseAction::UpdateFirewallRules { rule: "Block C2 server IP".to_string(), action: "ADD".to_string() });
        } else if anomaly_description.contains("anomalous behavior") {
            actions.push(ResponseAction::AlertHumanOperator { priority: 8, message: format!("Urgent: {}. Review user activity.", anomaly_description) });
            actions.push(ResponseAction::DynamicPolicyAdjustment { policy_id: "user_auth_policy".to_string(), new_param: "require_mfa_for_activity".to_string() });
        } else if anomaly_description.contains("Quantum attack") {
            actions.push(ResponseAction::RotateKeys { system_id: "critical_api_gateway".to_string(), key_type: "PQC".to_string() });
            actions.push(ResponseAction::AlertHumanOperator { priority: 10, message: format!("CRITICAL: {}. Initiate emergency PQC migration.", anomaly_description) });
        } else {
            actions.push(ResponseAction::AlertHumanOperator { priority: 5, message: format!("Detected: {}. Requires review.", anomaly_description) });
        }

        // Add some probabilistic self-hardening
        if rng.gen_bool(0.3) {
            actions.push(ResponseAction::DynamicPolicyAdjustment { policy_id: "system_hardening".to_string(), new_param: "increase_log_retention".to_string() });
        }

        actions
    }

    /// Executes the formulated response actions.
    pub fn execute_action(&mut self, action: &ResponseAction) {
        println!("  [ThreatManager]: Executing response action: {}.", action);
        self.active_responses.push(action.clone());
        // In a real system, this would interact with other modules/services
        thread::sleep(Duration::from_millis(150)); // Simulate action execution
    }

    /// Runs a full adaptive threat response cycle.
    pub fn run_threat_cycle(&mut self, threat_event: ThreatEvent, rng: &mut impl Rng) {
        println!("\n--- Running Adaptive Threat Response Cycle ---");
        println!("  Incoming Threat Event: {}", threat_event);

        if let Some(anomaly_desc) = self.detect_anomaly(&threat_event) {
            println!("  Anomaly Identified: {}.", anomaly_desc);
            let responses = self.formulate_response(&anomaly_desc, rng);
            println!("  Formulated {} response actions.", responses.len());
            for action in responses {
                self.execute_action(&action);
            }
        } else {
            println!("  No significant anomaly detected by intelligence. Event logged.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_malware_detection() {
        let mut manager = ThreatResponseManager::new();
        let event = ThreatEvent::MalwareSignatureDetected { signature: "MALWARE_X_V1".to_string(), source: "endpoint_scan".to_string() };
        let anomaly = manager.detect_anomaly(&event);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().contains("Known malware signature"));
    }

    #[test]
    fn test_formulate_response_malware() {
        let manager = ThreatResponseManager::new();
        let mut rng = StepRng::new(1, 1);
        let actions = manager.formulate_response("Known malware signature detected.", &mut rng);
        assert!(!actions.is_empty());
        assert!(actions.iter().any(|a| matches!(a, ResponseAction::QuarantineSystem { .. })));
    }

    #[test]
    fn test_quantum_attack_detection() {
        let mut manager = ThreatResponseManager::new();
        let event = ThreatEvent::QuantumAttackDetected { algorithm: "RSA-2048".to_string(), target_system: "core_db".to_string() };
        let anomaly = manager.detect_anomaly(&event);
        assert!(anomaly.is_some());
        assert!(anomaly.unwrap().contains("Quantum attack against vulnerable algorithm"));
    }
}

fn main() {
    println!("\n--- Adaptive Threat Intelligence & Response Demonstration ---");

    let mut threat_manager = ThreatResponseManager::new();
    let mut rng = rand::thread_rng();

    // 1. Simulate various threat events and observe responses
    let threat_events = vec![
        ThreatEvent::MalwareSignatureDetected { signature: "MALWARE_X_V1".to_string(), source: "antivirus".to_string() },
        ThreatEvent::AnomalousBehavior { user_id: "alice@example.com".to_string(), activity: "UNUSUAL_LOGIN_LOCATION".to_string(), deviation_score: 0.95 },
        ThreatEvent::NetworkIntrusionAttempt { ip_address: "192.168.1.100".to_string(), port: 22 }, // Unidentified
        ThreatEvent::QuantumAttackDetected { algorithm: "RSA-2048".to_string(), target_system: "payment_gateway".to_string() },
        ThreatEvent::MalwareSignatureDetected { signature: "UNKNOWN_MALWARE".to_string(), source: "network_monitor".to_string() }, // Unidentified
    ];

    for (i, event) in threat_events.into_iter().enumerate() {
        println!("\n====== Scenario {} ======", i + 1);
        threat_manager.run_threat_cycle(event, &mut rng);
        thread::sleep(Duration::from_secs(1)); // Pause between scenarios
    }

    // 2. Observe active responses
    println!("\n--- Summary of Active Responses ---");
    if threat_manager.active_responses.is_empty() {
        println!("  No active responses triggered.");
    } else {
        for (i, action) in threat_manager.active_responses.iter().enumerate() {
            println!("  {}. {}", i + 1, action);
        }
    }

    println!("\nAdaptive Threat Intelligence & Response Demonstration Complete.");
}
