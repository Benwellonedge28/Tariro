/// Module for Emergent Behavior Early Warning System in Rust.
///
/// This module conceptually outlines a Rust-based implementation for an Emergent Behavior Early Warning System.
/// It will detect and flag nascent, unintended system behaviors for human oversight or autonomous correction
/// (leveraging RAG for anomaly patterns and SIMD for real-time behavioral analysis).

use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time::Duration;
use rand::Rng;

/// Represents various behavioral metrics or indicators.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BehaviorMetric {
    TaskDeviation,
    ResourceSpike,
    UnusualInteractionPattern,
    DecisionBiasDetected,
    PolicyNonCompliance,
}

impl fmt::Display for BehaviorMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Captures a system's state and metrics at a given time.
#[derive(Debug, Clone)]
pub struct BehaviorSnapshot {
    pub timestamp: u64,
    pub system_state_hash: String,
    pub metrics: HashMap<BehaviorMetric, f64>,
    pub context_summary: String,
}

/// Defines a known or learned pattern of anomalous/emergent behavior.
#[derive(Debug, Clone)]
pub struct AnomalyPattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub metric_thresholds: HashMap<BehaviorMetric, (f64, f64)>, // (min, max) or (threshold, comparison_type)
    pub typical_triggers: Vec<String>,
    pub severity: f32, // 0.0 (low) to 1.0 (critical)
}

/// Represents a detected emergent behavior or anomaly.
#[derive(Debug, Clone)]
pub struct EmergentBehaviorEvent {
    pub id: String,
    pub timestamp: u64,
    pub description: String,
    pub detected_metrics: HashMap<BehaviorMetric, f64>,
    pub matched_pattern_id: Option<String>,
    pub severity: f32,
    pub recommended_action: String,
}

/// The Emergent Behavior Early Warning System.
pub struct EarlyWarningSystem {
    pub anomaly_patterns_db: HashMap<String, AnomalyPattern>, // Conceptual: from RAG's knowledge graph
    pub behavior_history: Vec<BehaviorSnapshot>,
}

impl EarlyWarningSystem {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();
        patterns.insert("P1".to_string(), AnomalyPattern {
            id: "P1".to_string(),
            name: "HighTaskDeviation_LowResource".to_string(),
            description: "Unusually high task deviation combined with low resource utilization.".to_string(),
            metric_thresholds: HashMap::from([
                (BehaviorMetric::TaskDeviation, (0.2, f64::MAX)),
                (BehaviorMetric::ResourceSpike, (f64::MIN, 0.1)), // Low resource spike
            ]),
            typical_triggers: vec!["misconfigured_policy".to_string(), "deadlock_detected".to_string()],
            severity: 0.8,
        });
        patterns.insert("P2".to_string(), AnomalyPattern {
            id: "P2".to_string(),
            name: "UnusualUserInteraction".to_string(),
            description: "Repeated failed or strange interaction patterns from a specific user.".to_string(),
            metric_thresholds: HashMap::from([
                (BehaviorMetric::UnusualInteractionPattern, (0.7, f64::MAX)),
            ]),
            typical_triggers: vec!["user_confusion".to_string(), "malicious_attempt".to_string()],
            severity: 0.6,
        });

        EarlyWarningSystem { anomaly_patterns_db: patterns, behavior_history: Vec::new() }
    }

    /// Ingests new behavior data for analysis.
    pub fn ingest_behavior_data(&mut self, snapshot: BehaviorSnapshot) {
        println!("  [EWS]: Ingesting behavior data from timestamp {}.", snapshot.timestamp);
        self.behavior_history.push(snapshot);
        // Keep history to a reasonable size
        if self.behavior_history.len() > 100 { self.behavior_history.remove(0); }
        thread::sleep(Duration::from_millis(10));
    }

    /// Detects anomalies in ingested behavior data (conceptual SIMD for real-time analysis).
    pub fn detect_anomalies_simd(&self, current_snapshot: &BehaviorSnapshot) -> Vec<String> {
        println!("  [EWS]: Detecting anomalies (SIMD simulation).");
        let mut detected_anomalies = Vec::new();
        let mut rng = rand::thread_rng();

        // Simple rule-based anomaly detection for demo
        if let Some(&dev) = current_snapshot.metrics.get(&BehaviorMetric::TaskDeviation) {
            if dev > 0.5 { detected_anomalies.push("High Task Deviation detected.".to_string()); }
        }
        if let Some(&spike) = current_snapshot.metrics.get(&BehaviorMetric::ResourceSpike) {
            if spike > 0.8 { detected_anomalies.push("Significant Resource Spike detected.".to_string()); }
        }
        if rng.gen_bool(0.1) { detected_anomalies.push("Unexpected network traffic pattern.".to_string()); }
        thread::sleep(Duration::from_millis(50));
        detected_anomalies
    }

    /// Matches detected anomalies to known or learned patterns (conceptual RAG).
    pub fn match_to_known_patterns_rag(&self, detected_anomalies: &[String], current_snapshot: &BehaviorSnapshot) -> Vec<AnomalyPattern> {
        println!("  [EWS]: Matching to known anomaly patterns (RAG simulation).");
        let mut matched_patterns = Vec::new();

        for (pattern_id, pattern) in &self.anomaly_patterns_db {
            let mut pattern_matches = true;
            for (metric, (min_val, max_val)) in &pattern.metric_thresholds {
                if let Some(&current_metric_val) = current_snapshot.metrics.get(metric) {
                    if current_metric_val < *min_val || current_metric_val > *max_val {
                        pattern_matches = false;
                        break;
                    }
                } else {
                    pattern_matches = false;
                    break;
                }
            }
            if pattern_matches {
                // Also check if any detected anomalies match trigger description
                if detected_anomalies.iter().any(|da| pattern.typical_triggers.iter().any(|tr| da.contains(tr) || pattern.description.contains(da))) {
                     matched_patterns.push(pattern.clone());
                }
            }
        }
        thread::sleep(Duration::from_millis(70));
        matched_patterns
    }

    /// Assesses the overall severity of detected emergent behaviors.
    pub fn assess_severity(&self, matched_patterns: &[AnomalyPattern], raw_anomalies: &[String]) -> f32 {
        println!("  [EWS]: Assessing severity.");
        let mut max_severity = 0.0;
        for pattern in matched_patterns {
            if pattern.severity > max_severity { max_severity = pattern.severity; }
        }
        if !raw_anomalies.is_empty() && max_severity < 0.5 { max_severity = 0.5; } // If raw anomalies exist but no pattern, set a base severity
        thread::sleep(Duration::from_millis(15));
        max_severity
    }

    /// Generates an alert for human oversight or autonomous correction.
    pub fn generate_alert(&self, event: EmergentBehaviorEvent) {
        println!("\n!!! EARLY WARNING ALERT !!!");
        println!("  ID: {}", event.id);
        println!("  Timestamp: {}", event.timestamp);
        println!("  Severity: {:.2}", event.severity);
        println!("  Description: {}", event.description);
        println!("  Detected Metrics: {:?}", event.detected_metrics);
        if let Some(pattern_id) = event.matched_pattern_id {
            if let Some(pattern) = self.anomaly_patterns_db.get(&pattern_id) {
                println!("  Matched Anomaly Pattern: {} ({})", pattern.name, pattern.description);
            }
        }
        println!("  Recommended Action: {}", event.recommended_action);
        println!("!!! ALERT END !!!\n");
        // In a real system, this would trigger an actual notification, log to audit, potentially pause/change autonomy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ew_system_flow() {
        let mut ews = EarlyWarningSystem::new();
        let timestamp = 1234567890;
        let mut metrics = HashMap::new();
        metrics.insert(BehaviorMetric::TaskDeviation, 0.6);
        metrics.insert(BehaviorMetric::ResourceSpike, 0.05);
        let normal_snapshot = BehaviorSnapshot { timestamp, system_state_hash: "hash1".to_string(), metrics: metrics.clone(), context_summary: "normal op".to_string() };
        ews.ingest_behavior_data(normal_snapshot.clone());

        let detected_anomalies = ews.detect_anomalies_simd(&normal_snapshot);
        let matched_patterns = ews.match_to_known_patterns_rag(&detected_anomalies, &normal_snapshot);
        let severity = ews.assess_severity(&matched_patterns, &detected_anomalies);

        // Test high deviation scenario matching P1
        let mut high_dev_metrics = HashMap::new();
        high_dev_metrics.insert(BehaviorMetric::TaskDeviation, 0.7);
        high_dev_metrics.insert(BehaviorMetric::ResourceSpike, 0.05);
        let high_dev_snapshot = BehaviorSnapshot { timestamp: timestamp + 10, system_state_hash: "hash2".to_string(), metrics: high_dev_metrics.clone(), context_summary: "high dev".to_string() };
        let high_dev_anomalies = ews.detect_anomalies_simd(&high_dev_snapshot);
        let high_dev_patterns = ews.match_to_known_patterns_rag(&high_dev_anomalies, &high_dev_snapshot);
        let high_dev_severity = ews.assess_severity(&high_dev_patterns, &high_dev_anomalies);

        assert!(!high_dev_anomalies.is_empty());
        assert!(!high_dev_patterns.is_empty());
        assert!(high_dev_severity > 0.5);
    }
}

fn main() {
    println!("\n--- Emergent Behavior Early Warning System Demonstration ---");

    let mut ews = EarlyWarningSystem::new();
    let start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    // Simulate a period of normal operation
    for i in 0..5 {
        let timestamp = start_time + i * 10;
        let mut rng = rand::thread_rng();
        let mut metrics = HashMap::new();
        metrics.insert(BehaviorMetric::TaskDeviation, rng.gen_range(0.05..0.2));
        metrics.insert(BehaviorMetric::ResourceSpike, rng.gen_range(0.01..0.1));
        metrics.insert(BehaviorMetric::UnusualInteractionPattern, rng.gen_range(0.0..0.1));

        let snapshot = BehaviorSnapshot {
            timestamp,
            system_state_hash: format!("normal_state_{}", i),
            metrics,
            context_summary: "Normal system operation.".to_string(),
        };
        ews.ingest_behavior_data(snapshot.clone());
        let anomalies = ews.detect_anomalies_simd(&snapshot);
        let patterns = ews.match_to_known_patterns_rag(&anomalies, &snapshot);
        let severity = ews.assess_severity(&patterns, &anomalies);

        if severity > 0.0 {
            println!("  Cycle {}: Mild anomaly detected (Severity: {:.2}). No alert generated.", i + 1, severity);
        } else {
            println!("  Cycle {}: All clear.", i + 1);
        }
        thread::sleep(Duration::from_millis(100));
    }

    // Simulate an emergent/anomalous behavior
    println!("\n--- Simulating Emergent Behavior Trigger ---");
    let mut rng = rand::thread_rng();
    let timestamp_anomaly = start_time + 5 * 10;
    let mut anomaly_metrics = HashMap::new();
    anomaly_metrics.insert(BehaviorMetric::TaskDeviation, rng.gen_range(0.6..0.9)); // High deviation
    anomaly_metrics.insert(BehaviorMetric::ResourceSpike, rng.gen_range(0.05..0.15)); // Low resource spike
    anomaly_metrics.insert(BehaviorMetric::UnusualInteractionPattern, rng.gen_range(0.1..0.3));

    let anomaly_snapshot = BehaviorSnapshot {
        timestamp: timestamp_anomaly,
        system_state_hash: "anomalous_state_A".to_string(),
        metrics: anomaly_metrics.clone(),
        context_summary: "System exhibiting unusual task execution patterns.".to_string(),
    };
    ews.ingest_behavior_data(anomaly_snapshot.clone());

    let detected_anomalies = ews.detect_anomalies_simd(&anomaly_snapshot);
    let matched_patterns = ews.match_to_known_patterns_rag(&detected_anomalies, &anomaly_snapshot);
    let severity = ews.assess_severity(&matched_patterns, &detected_anomalies);

    if severity > 0.6 { // If severity is high enough, generate an alert
        let event_id = format!("EBEWS_ALERT_{}", timestamp_anomaly);
        let matched_pattern_id = if !matched_patterns.is_empty() { Some(matched_patterns[0].id.clone()) } else { None };
        ews.generate_alert(EmergentBehaviorEvent {
            id: event_id,
            timestamp: timestamp_anomaly,
            description: "Detected a high severity emergent behavior. Possible policy misconfiguration or internal deadlock.".to_string(),
            detected_metrics: anomaly_metrics,
            matched_pattern_id,
            severity,
            recommended_action: "Initiate diagnostic protocols and notify human operator for review.".to_string(),
        });
    } else {
        println!("  Anomaly detected but not severe enough for alert (Severity: {:.2}).", severity);
    }

    println!("\nEmergent Behavior Early Warning System Demonstration Complete.");
}
