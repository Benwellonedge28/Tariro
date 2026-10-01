/// Module for Real-time Self-Diagnosis & Introspection in Rust.
///
/// This module conceptually outlines a system where the AGI can monitor its
/// own internal cognitive processes, operational states, resource consumption,
/// and decision-making logic in real-time. It enables self-diagnosis of
/// anomalies, suboptimal performance, or deviations from desired behavior.

use crate::prelude::*;
use std::collections::VecDeque;
use std::fmt;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::Rng;

/// Represents various internal metrics of the AGI.
#[derive(Debug, Clone, PartialEq)]
pub enum Metric {
    CpuLoad(f32),
    MemoryUsage(f32),
    DecisionLatencyMs(u32),
    PolicyActivationCount(u32),
    ErrorRate(f32),
}

/// A snapshot of the AGI's internal operational state at a given time.
#[derive(Debug, Clone, PartialEq)]
pub struct InternalState {
    pub timestamp: u64,
    pub metrics: Vec<Metric>,
    pub current_policies_active: Vec<String>,
    pub decision_context_summary: String,
}

/// Categorizes detected anomalies or issues within the AGI's operation.
#[derive(Debug, Clone, PartialEq)]
pub enum Anomaly {
    HighCpuAnomaly { current_load: f32, threshold: f32 },
    MemoryLeakAnomaly { current_usage: f32, threshold: f32 },
    StalledDecisionAnomaly { latency: u32, threshold: u32 },
    LogicDriftAnomaly { policy_id: String, observed_deviation: String },
    UnexpectedErrorRate { rate: f32, threshold: f32 },
    UnknownIssue { description: String },
}

impl fmt::Display for Anomaly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Anomaly::HighCpuAnomaly { current_load, threshold } => write!(f, "High CPU: {:.2}% (>{:.2}%) - System may be overloaded.", current_load, threshold),
            Anomaly::MemoryLeakAnomaly { current_usage, threshold } => write!(f, "Memory Leak: {:.2}% (>{:.2}%) - Unintended memory growth.", current_usage, threshold),
            Anomaly::StalledDecisionAnomaly { latency, threshold } => write!(f, "Decision Stalled: {}ms (>{}) - Decision-making process lagging.", latency, threshold),
            Anomaly::LogicDriftAnomaly { policy_id, observed_deviation } => write!(f, "Logic Drift in Policy '{}': {}.", policy_id, observed_deviation),
            Anomaly::UnexpectedErrorRate { rate, threshold } => write!(f, "High Error Rate: {:.2}% (>{:.2}%) - Operational instability.", rate, threshold),
            Anomaly::UnknownIssue { description } => write!(f, "Unknown Anomaly: {}.", description),
        }
    }
}

/// The Real-time Self-Diagnoser and Introspection component.
pub struct SelfDiagnoser {
    pub threshold_cpu_load: f32,
    pub threshold_memory_usage: f32,
    pub threshold_decision_latency: u32,
    pub threshold_error_rate: f32,
    pub recorded_states: VecDeque<InternalState>, // For historical analysis
    max_states_to_keep: usize,
}

impl SelfDiagnoser {
    pub fn new() -> Self {
        SelfDiagnoser {
            threshold_cpu_load: 85.0,
            threshold_memory_usage: 90.0,
            threshold_decision_latency: 500,
            threshold_error_rate: 2.0,
            recorded_states: VecDeque::new(),
            max_states_to_keep: 100,
        }
    }

    /// Captures current internal metrics and forms an InternalState snapshot.
    pub fn capture_metrics(&mut self, rng: &mut impl Rng) -> InternalState {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cpu_load = rng.gen_range(20.0..100.0);
        let memory_usage = rng.gen_range(30.0..95.0);
        let decision_latency = rng.gen_range(50..1000);
        let policy_activations = rng.gen_range(1..20);
        let error_rate = rng.gen_range(0.0..5.0);

        let state = InternalState {
            timestamp,
            metrics: vec![
                Metric::CpuLoad(cpu_load),
                Metric::MemoryUsage(memory_usage),
                Metric::DecisionLatencyMs(decision_latency),
                Metric::PolicyActivationCount(policy_activations),
                Metric::ErrorRate(error_rate),
            ],
            current_policies_active: vec!["policy_a".to_string(), "policy_b".to_string()], // Simplified
            decision_context_summary: "Processing user request for info.".to_string(),
        };

        self.recorded_states.push_back(state.clone());
        if self.recorded_states.len() > self.max_states_to_keep {
            self.recorded_states.pop_front();
        }
        state
    }

    /// Analyzes the given InternalState for anomalies.
    pub fn analyze_state(&self, state: &InternalState) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        for metric in &state.metrics {
            match metric {
                Metric::CpuLoad(load) => {
                    if *load > self.threshold_cpu_load {
                        anomalies.push(Anomaly::HighCpuAnomaly { current_load: *load, threshold: self.threshold_cpu_load });
                    }
                },
                Metric::MemoryUsage(usage) => {
                    if *usage > self.threshold_memory_usage {
                        anomalies.push(Anomaly::MemoryLeakAnomaly { current_usage: *usage, threshold: self.threshold_memory_usage });
                    }
                },
                Metric::DecisionLatencyMs(latency) => {
                    if *latency > self.threshold_decision_latency {
                        anomalies.push(Anomaly::StalledDecisionAnomaly { latency: *latency, threshold: self.threshold_decision_latency });
                    }
                },
                Metric::ErrorRate(rate) => {
                    if *rate > self.threshold_error_rate {
                        anomalies.push(Anomaly::UnexpectedErrorRate { rate: *rate, threshold: self.threshold_error_rate });
                    }
                },
                _ => {},
            }
        }
        anomalies
    }

    /// Reports detected anomalies and suggests (conceptual) corrective actions.
    pub fn report_anomaly(&self, anomaly: &Anomaly) {
        println!("  [SelfDiagnoser]: ANOMALY DETECTED: {}", anomaly);
        match anomaly {
            Anomaly::HighCpuAnomaly { .. } => println!("    -> Suggestion: Investigate active processes, consider scaling compute resources."),
            Anomaly::MemoryLeakAnomaly { .. } => println!("    -> Suggestion: Review recent code changes, perform memory profiling."),
            Anomaly::StalledDecisionAnomaly { .. } => println!("    -> Suggestion: Analyze decision logic complexity, optimize policy evaluation."),
            Anomaly::LogicDriftAnomaly { policy_id, .. } => println!("    -> Suggestion: Re-evaluate policy '{}' effectiveness, consider re-training.", policy_id),
            Anomaly::UnexpectedErrorRate { .. } => println!("    -> Suggestion: Check dependent services, review recent deployments."),
            _ => println!("    -> Suggestion: Manual human review required for this unique anomaly."),
        }
        // In a real system, this would trigger alerts, automatic remediation workflows, etc.
    }

    /// Runs a full self-diagnosis cycle.
    pub fn run_diagnosis_cycle(&mut self, rng: &mut impl Rng) {
        println!("\n--- Running Self-Diagnosis Cycle ---");
        let current_state = self.capture_metrics(rng);
        println!("  [SelfDiagnoser]: Captured state at timestamp {}.", current_state.timestamp);

        let anomalies = self.analyze_state(&current_state);
        if anomalies.is_empty() {
            println!("  [SelfDiagnoser]: No anomalies detected. System operating normally.");
        } else {
            println!("  [SelfDiagnoser]: {} anomalies found:", anomalies.len());
            for anomaly in anomalies {
                self.report_anomaly(&anomaly);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_capture_metrics() {
        let mut diagnoser = SelfDiagnoser::new();
        let mut rng = StepRng::new(1, 1);
        let state = diagnoser.capture_metrics(&mut rng);
        assert_eq!(state.metrics.len(), 5);
        assert!(!diagnoser.recorded_states.is_empty());
    }

    #[test]
    fn test_analyze_state_high_cpu() {
        let diagnoser = SelfDiagnoser::new();
        let state = InternalState {
            timestamp: 123,
            metrics: vec![Metric::CpuLoad(90.0)],
            current_policies_active: vec![],
            decision_context_summary: "".to_string(),
        };
        let anomalies = diagnoser.analyze_state(&state);
        assert_eq!(anomalies.len(), 1);
        assert!(matches!(anomalies[0], Anomaly::HighCpuAnomaly { .. }));
    }

    #[test]
    fn test_analyze_state_no_anomaly() {
        let diagnoser = SelfDiagnoser::new();
        let state = InternalState {
            timestamp: 123,
            metrics: vec![Metric::CpuLoad(50.0), Metric::MemoryUsage(60.0)],
            current_policies_active: vec![],
            decision_context_summary: "".to_string(),
        };
        let anomalies = diagnoser.analyze_state(&state);
        assert!(anomalies.is_empty());
    }
}

fn main() {
    println!("\n--- Real-time Self-Diagnosis & Introspection Demonstration ---");

    let mut self_diagnoser = SelfDiagnoser::new();
    let mut rng = rand::thread_rng();

    // Simulate multiple diagnosis cycles
    for i in 1..=5 {
        println!("\n====== Iteration {} ======", i);

        // Introduce some anomalies periodically for demonstration
        if i == 2 {
            // Force a high CPU load for this cycle
            self_diagnoser.threshold_cpu_load = 50.0; // Lower threshold to trigger anomaly
            println!("  [DEMO]: Lowering CPU load threshold to 50% for this iteration to simulate an issue.");
        } else if i == 4 {
            // Force a high memory usage
            self_diagnoser.threshold_memory_usage = 70.0; // Lower threshold
            println!("  [DEMO]: Lowering Memory usage threshold to 70% for this iteration to simulate an issue.");
        } else {
            // Reset thresholds
            self_diagnoser.threshold_cpu_load = 85.0;
            self_diagnoser.threshold_memory_usage = 90.0;
        }

        self_diagnoser.run_diagnosis_cycle(&mut rng);
        thread::sleep(Duration::from_secs(1)); // Simulate time passing
    }

    println!("\n--- Historical States (Last 5) ---");
    for (i, state) in self_diagnoser.recorded_states.iter().rev().take(5).enumerate() {
        println!("  {}. State at {}: Metrics: {:?}", i + 1, state.timestamp, state.metrics);
    }

    println!("\nReal-time Self-Diagnosis & Introspection Demonstration Complete.");
}
