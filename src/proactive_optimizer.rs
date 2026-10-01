/// Module for Proactive Resource Allocation and Optimization in Rust.
///
/// This module outlines a conceptual system that utilizes predicted future
/// system states to proactively adjust resource provisioning and workload
/// distribution, preventing potential bottlenecks and maintaining optimal performance.

use crate::prelude::*;
use std::fmt;
use std::thread;
use std::time::Duration;

/// Reusing PredictedState and RiskLevel from `predictive_modeling.rs`.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictedState {
    pub timestamp: u64,
    pub predicted_cpu_usage: f32,
    pub predicted_memory_usage: f32,
    pub predicted_network_latency: u32,
    pub predicted_error_rate: f32,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents a command to adjust system resources or workload.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceCommand {
    ScaleUp { resource_type: String, amount: u32 },
    ScaleDown { resource_type: String, amount: u32 },
    MigrateWorkload { workload_id: String, target_node: String },
    AdjustPriority { service_name: String, priority_level: u8 },
    AlertHuman { message: String },
    // More complex commands like reconfigure network, update cache policies, etc.
}

/// The Proactive Optimizer component.
pub struct ProactiveOptimizer {
    // Configuration for optimization goals, policies, etc.
    pub optimization_enabled: bool,
}

impl ProactiveOptimizer {
    pub fn new() -> Self {
        ProactiveOptimizer { optimization_enabled: true }
    }

    /// Analyzes a predicted state and determines if proactive action is needed.
    pub fn analyze_prediction(&self, predicted_state: &PredictedState) -> Option<ResourceCommand> {
        if !self.optimization_enabled {
            println!("  [Optimizer]: Proactive optimization is disabled.");
            return None;
        }

        println!("  [Optimizer]: Analyzing predicted state with Risk Level: {}.", predicted_state.risk_level);

        match predicted_state.risk_level {
            RiskLevel::Critical => {
                println!("    -> Critical risk predicted. Immediate action required.");
                if predicted_state.predicted_cpu_usage > 90.0 {
                    Some(ResourceCommand::ScaleUp { resource_type: "CPU_Cores".to_string(), amount: 4 })
                } else if predicted_state.predicted_memory_usage > 95.0 {
                    Some(ResourceCommand::ScaleUp { resource_type: "Memory_GB".to_string(), amount: 8 })
                } else {
                    Some(ResourceCommand::AlertHuman { message: "Critical state predicted, automated action unclear.".to_string() })
                }
            },
            RiskLevel::High => {
                println!("    -> High risk predicted. Proactive scaling recommended.");
                if predicted_state.predicted_cpu_usage > 70.0 {
                    Some(ResourceCommand::ScaleUp { resource_type: "CPU_Cores".to_string(), amount: 2 })
                } else if predicted_state.predicted_memory_usage > 80.0 {
                    Some(ResourceCommand::ScaleUp { resource_type: "Memory_GB".to_string(), amount: 4 })
                } else if predicted_state.predicted_network_latency > 500 {
                    Some(ResourceCommand::AdjustPriority { service_name: "latency_sensitive_app".to_string(), priority_level: 9 })
                } else {
                    None
                }
            },
            RiskLevel::Medium => {
                println!("    -> Medium risk predicted. Minor adjustments or monitoring.");
                if predicted_state.predicted_error_rate > 1.0 {
                    Some(ResourceCommand::AdjustPriority { service_name: "error_prone_service".to_string(), priority_level: 5 })
                } else {
                    None
                }
            },
            RiskLevel::Low => {
                println!("    -> Low risk. No immediate action needed.");
                None
            },
        }
    }

    /// Executes a determined resource command.
    pub fn execute_command(&self, command: &ResourceCommand) {
        println!("  [Optimizer]: Executing command: {:?}", command);
        match command {
            ResourceCommand::ScaleUp { resource_type, amount } => {
                println!("    -> Provisioning {} extra {} units.", amount, resource_type);
            },
            ResourceCommand::ScaleDown { resource_type, amount } => {
                println!("    -> De-provisioning {} {} units.", amount, resource_type);
            },
            ResourceCommand::MigrateWorkload { workload_id, target_node } => {
                println!("    -> Migrating workload '{}' to '{}'.", workload_id, target_node);
            },
            ResourceCommand::AdjustPriority { service_name, priority_level } => {
                println!("    -> Setting priority of service '{}' to {}.".to_string(), service_name, priority_level);
            },
            ResourceCommand::AlertHuman { message } => {
                println!("    -> Sending critical alert to human operators: '{}'.", message);
            },
        }
        thread::sleep(Duration::from_millis(300)); // Simulate command execution time
        println!("  [Optimizer]: Command execution simulated.");
    }

    /// Processes a predicted state, determines, and executes a proactive action.
    pub fn process_predicted_state(&self, predicted_state: PredictedState) {
        println!("\nReceived Predicted State (at {}s): CPU: {:.2}%, Mem: {:.2}%, Latency: {}ms, Errors: {:.2}% (Risk: {})",
                 predicted_state.timestamp, predicted_state.predicted_cpu_usage, predicted_state.predicted_memory_usage,
                 predicted_state.predicted_network_latency, predicted_state.predicted_error_rate, predicted_state.risk_level);

        if let Some(command) = self.analyze_prediction(&predicted_state) {
            self.execute_command(&command);
        } else {
            println!("  [Optimizer]: No proactive action determined for this predicted state.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_critical_cpu() {
        let optimizer = ProactiveOptimizer::new();
        let predicted = PredictedState {
            timestamp: 1,
            predicted_cpu_usage: 95.0,
            predicted_memory_usage: 50.0,
            predicted_network_latency: 100,
            predicted_error_rate: 0.1,
            risk_level: RiskLevel::Critical,
        };
        let command = optimizer.analyze_prediction(&predicted);
        assert_eq!(command, Some(ResourceCommand::ScaleUp { resource_type: "CPU_Cores".to_string(), amount: 4 }));
    }

    #[test]
    fn test_analyze_high_memory() {
        let optimizer = ProactiveOptimizer::new();
        let predicted = PredictedState {
            timestamp: 1,
            predicted_cpu_usage: 60.0,
            predicted_memory_usage: 85.0,
            predicted_network_latency: 200,
            predicted_error_rate: 0.5,
            risk_level: RiskLevel::High,
        };
        let command = optimizer.analyze_prediction(&predicted);
        assert_eq!(command, Some(ResourceCommand::ScaleUp { resource_type: "Memory_GB".to_string(), amount: 4 }));
    }

    #[test]
    fn test_low_risk_no_action() {
        let optimizer = ProactiveOptimizer::new();
        let predicted = PredictedState {
            timestamp: 1,
            predicted_cpu_usage: 20.0,
            predicted_memory_usage: 30.0,
            predicted_network_latency: 50,
            predicted_error_rate: 0.0,
            risk_level: RiskLevel::Low,
        };
        let command = optimizer.analyze_prediction(&predicted);
        assert_eq!(command, None);
    }

    #[test]
    fn test_optimization_disabled() {
        let mut optimizer = ProactiveOptimizer::new();
        optimizer.optimization_enabled = false;
        let predicted = PredictedState {
            timestamp: 1, predicted_cpu_usage: 95.0, predicted_memory_usage: 50.0, predicted_network_latency: 100, predicted_error_rate: 0.1,
            risk_level: RiskLevel::Critical,
        };
        let command = optimizer.analyze_prediction(&predicted);
        assert_eq!(command, None);
    }
}

fn main() {
    println!("\n--- Proactive Resource Allocation and Optimization Demonstration ---");

    let optimizer = ProactiveOptimizer::new();

    // 1. Simulate predicted states coming from the Predictive Model

    // Low risk state
    let predicted_low_risk = PredictedState {
        timestamp: 1714168800,
        predicted_cpu_usage: 30.0,
        predicted_memory_usage: 45.0,
        predicted_network_latency: 70,
        predicted_error_rate: 0.1,
        risk_level: RiskLevel::Low,
    };
    optimizer.process_predicted_state(predicted_low_risk);

    // Medium risk state (high error rate)
    let predicted_medium_risk = PredictedState {
        timestamp: 1714168860,
        predicted_cpu_usage: 50.0,
        predicted_memory_usage: 60.0,
        predicted_network_latency: 150,
        predicted_error_rate: 2.5, // Elevated error rate
        risk_level: RiskLevel::Medium,
    };
    optimizer.process_predicted_state(predicted_medium_risk);

    // High risk state (high CPU prediction)
    let predicted_high_risk_cpu = PredictedState {
        timestamp: 1714168920,
        predicted_cpu_usage: 75.0, // High CPU
        predicted_memory_usage: 70.0,
        predicted_network_latency: 300,
        predicted_error_rate: 0.5,
        risk_level: RiskLevel::High,
    };
    optimizer.process_predicted_state(predicted_high_risk_cpu);

    // Critical risk state (very high Memory prediction)
    let predicted_critical_memory = PredictedState {
        timestamp: 1714168980,
        predicted_cpu_usage: 80.0,
        predicted_memory_usage: 96.0, // Critical Memory
        predicted_network_latency: 600,
        predicted_error_rate: 1.0,
        risk_level: RiskLevel::Critical,
    };
    optimizer.process_predicted_state(predicted_critical_memory);

    // Critical risk state (automated action unclear, alerts human)
    let predicted_critical_unclear = PredictedState {
        timestamp: 1714169040,
        predicted_cpu_usage: 88.0,
        predicted_memory_usage: 88.0,
        predicted_network_latency: 1100, // Critical latency but no direct scale up rule
        predicted_error_rate: 7.0,
        risk_level: RiskLevel::Critical,
    };
    optimizer.process_predicted_state(predicted_critical_unclear);

    println!("\nProactive Resource Allocation and Optimization Demonstration Complete.");
}
