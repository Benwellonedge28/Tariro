/// Module for Dynamic Resource Management and Graceful Degradation in Rust.
///
/// This module conceptually outlines a system that can adapt to resource constraints
/// or partial failures by strategically reducing non-critical functionality,
/// rather than outright crashing, ensuring continued operation.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

/// Represents different functional components of the system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SystemComponent {
    CoreAI,
    UserInterface,
    DataIngestion,
    AnalyticsEngine,
    ReportingService,
    ExternalIntegration,
}

impl fmt::Display for SystemComponent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents the current status and load of a system resource.
#[derive(Debug, Clone)]
pub struct ResourceStatus {
    pub cpu_usage_percent: f32,
    pub memory_usage_percent: f32,
    pub latency_ms: u32,
    pub active_requests: u32,
    pub healthy: bool,
}

impl Default for ResourceStatus {
    fn default() -> Self {
        ResourceStatus { cpu_usage_percent: 0.0, memory_usage_percent: 0.0, latency_ms: 50, active_requests: 0, healthy: true }
    }
}

/// Defines strategies for graceful degradation.
#[derive(Debug, Clone, PartialEq)]
pub enum DegradationStrategy {
    ReduceFeatures { component: SystemComponent, features: Vec<String> },
    PrioritizeCritical { critical_components: Vec<SystemComponent> },
    ShedLoad { component: SystemComponent, percentage: u32 },
    SwitchToFallback { component: SystemComponent, fallback_service: String },
    // Add more strategies like rate limiting, caching, etc.
}

/// The Dynamic Resource Manager.
pub struct ResourceManager {
    resource_status: Arc<RwLock<HashMap<SystemComponent, ResourceStatus>>>,
    active_degradation: Arc<RwLock<Option<DegradationStrategy>>>,
    thresholds: HashMap<SystemComponent, ResourceThresholds>,
}

struct ResourceThresholds {
    cpu_critical: f32,
    memory_critical: f32,
    latency_critical: u32,
}

impl ResourceManager {
    pub fn new(initial_components: Vec<SystemComponent>) -> Self {
        let mut status_map = HashMap::new();
        let mut thresholds_map = HashMap::new();

        for component in initial_components {
            status_map.insert(component.clone(), ResourceStatus::default());
            thresholds_map.insert(component, ResourceThresholds {
                cpu_critical: 85.0,
                memory_critical: 90.0,
                latency_critical: 500,
            });
        }

        ResourceManager {
            resource_status: Arc::new(RwLock::new(status_map)),
            active_degradation: Arc::new(RwLock::new(None)),
            thresholds: thresholds_map,
        }
    }

    /// Updates the status of a specific system component.
    pub fn update_resource_status(&self, component: SystemComponent, new_status: ResourceStatus) {
        let mut status_guard = self.resource_status.write().unwrap();
        status_guard.insert(component.clone(), new_status);
        drop(status_guard);
        self.evaluate_degradation_needs(component);
    }

    /// Evaluates if degradation is needed for a component.
    fn evaluate_degradation_needs(&self, component: SystemComponent) {
        let status_guard = self.resource_status.read().unwrap();
        let current_status = status_guard.get(&component).unwrap();
        let thresholds = self.thresholds.get(&component).unwrap();

        if !current_status.healthy ||
           current_status.cpu_usage_percent > thresholds.cpu_critical ||
           current_status.memory_usage_percent > thresholds.memory_critical ||
           current_status.latency_ms > thresholds.latency_critical
        {
            println!("  [ResourceManager]: {} is under stress. Considering degradation.", component);
            // Simple strategy: if CoreAI is stressed, prioritize it and shed load from UI
            if component == SystemComponent::CoreAI && *self.active_degradation.read().unwrap() == None {
                self.apply_degradation(DegradationStrategy::PrioritizeCritical { critical_components: vec![SystemComponent::CoreAI] });
                self.apply_degradation(DegradationStrategy::ShedLoad { component: SystemComponent::UserInterface, percentage: 50 });
            }
        } else {
            // If conditions improve, consider restoring full functionality
            if *self.active_degradation.read().unwrap() != None {
                println!("  [ResourceManager]: {} conditions improved. Considering restoration.", component);
                self.restore_full_functionality();
            }
        }
    }

    /// Applies a specific degradation strategy.
    pub fn apply_degradation(&self, strategy: DegradationStrategy) {
        let mut active_degradation_guard = self.active_degradation.write().unwrap();
        *active_degradation_guard = Some(strategy.clone());
        println!("  [ResourceManager]: Applying degradation strategy: {:?}", strategy);
        // In a real system, this would trigger actual system changes
        thread::sleep(Duration::from_millis(200));
    }

    /// Restores the system to full functionality if conditions allow.
    pub fn restore_full_functionality(&self) {
        let mut active_degradation_guard = self.active_degradation.write().unwrap();
        if active_degradation_guard.is_some() {
            println!("  [ResourceManager]: Restoring full system functionality.");
            *active_degradation_guard = None;
            // In a real system, this would reverse degradation actions
            thread::sleep(Duration::from_millis(200));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let manager = ResourceManager::new(vec![SystemComponent::CoreAI, SystemComponent::UserInterface]);
        let status = manager.resource_status.read().unwrap();
        assert_eq!(status.len(), 2);
        assert!(status.get(&SystemComponent::CoreAI).is_some());
    }

    #[test]
    fn test_degradation_trigger() {
        let manager = ResourceManager::new(vec![SystemComponent::CoreAI, SystemComponent::UserInterface]);
        manager.update_resource_status(SystemComponent::CoreAI, ResourceStatus { cpu_usage_percent: 90.0, ..Default::default() });
        let active_degradation = manager.active_degradation.read().unwrap();
        assert!(active_degradation.is_some());
        assert!(matches!(*active_degradation.as_ref().unwrap(), DegradationStrategy::PrioritizeCritical { .. }));
    }

    #[test]
    fn test_restoration_trigger() {
        let manager = ResourceManager::new(vec![SystemComponent::CoreAI, SystemComponent::UserInterface]);
        manager.update_resource_status(SystemComponent::CoreAI, ResourceStatus { cpu_usage_percent: 90.0, ..Default::default() });
        // Simulate conditions improving
        manager.update_resource_status(SystemComponent::CoreAI, ResourceStatus { cpu_usage_percent: 20.0, ..Default::default() });
        let active_degradation = manager.active_degradation.read().unwrap();
        assert!(active_degradation.is_none());
    }
}

fn main() {
    println!("\n--- Dynamic Resource Management and Graceful Degradation Demonstration ---");

    let components = vec![
        SystemComponent::CoreAI,
        SystemComponent::UserInterface,
        SystemComponent::DataIngestion,
        SystemComponent::AnalyticsEngine,
    ];
    let resource_manager = ResourceManager::new(components);
    println!("Resource Manager initialized.");

    // 1. Simulate normal operation
    println!("\n--- Normal Operation ---");
    resource_manager.update_resource_status(SystemComponent::CoreAI, ResourceStatus { cpu_usage_percent: 20.0, memory_usage_percent: 30.0, latency_ms: 100, active_requests: 50, healthy: true });
    resource_manager.update_resource_status(SystemComponent::UserInterface, ResourceStatus { cpu_usage_percent: 15.0, memory_usage_percent: 25.0, latency_ms: 80, active_requests: 120, healthy: true });
    thread::sleep(Duration::from_secs(1));

    // 2. Simulate CoreAI under heavy load (trigger degradation)
    println!("\n--- CoreAI Under Heavy Load ---");
    resource_manager.update_resource_status(SystemComponent::CoreAI, ResourceStatus { cpu_usage_percent: 95.0, memory_usage_percent: 80.0, latency_ms: 600, active_requests: 500, healthy: true });
    thread::sleep(Duration::from_secs(2));

    // 3. Simulate continued heavy load, but now with some components failing
    println!("\n--- Partial Component Failure ---");
    resource_manager.update_resource_status(SystemComponent::DataIngestion, ResourceStatus { healthy: false, ..Default::default() });
    resource_manager.update_resource_status(SystemComponent::AnalyticsEngine, ResourceStatus { cpu_usage_percent: 90.0, ..Default::default() });
    thread::sleep(Duration::from_secs(2));

    // 4. Simulate recovery for CoreAI and DataIngestion
    println!("\n--- Recovery Started ---");
    resource_manager.update_resource_status(SystemComponent::CoreAI, ResourceStatus { cpu_usage_percent: 40.0, memory_usage_percent: 50.0, latency_ms: 150, active_requests: 100, healthy: true });
    resource_manager.update_resource_status(SystemComponent::DataIngestion, ResourceStatus { healthy: true, ..Default::default() });
    thread::sleep(Duration::from_secs(2));

    // 5. System should restore full functionality
    println!("\n--- Full Functionality Restored ---");
    resource_manager.update_resource_status(SystemComponent::AnalyticsEngine, ResourceStatus { cpu_usage_percent: 30.0, ..Default::default() });
    thread::sleep(Duration::from_secs(1));

    println!("\nDynamic Resource Management and Graceful Degradation Demonstration Complete.");
}
