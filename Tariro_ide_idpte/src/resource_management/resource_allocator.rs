//! Acts on resource allocation decisions to interact with the underlying OS/hypervisor.

use crate::resource_management::resource_models::{AllocationResult, ResourceType, ResourceRequest};
use std::collections::HashMap;
use async_trait::async_trait;

/// Trait for interacting with the underlying system to manage resources.
/// This abstraction allows different implementations for different OS/hypervisor types.
#[async_trait]
pub trait SystemResourceInterface: Send + Sync {
    async fn apply_resource_limits(
        &self,
        task_id: &str,
        resources: &HashMap<ResourceType, f64>,
    ) -> Result<(), String>;
    async fn release_resource_limits(&self, task_id: &str) -> Result<(), String>;
    async fn provision_new_resource(&self, resource_type: &ResourceType, amount: f64) -> Result<(), String>;
    async fn deprovision_resource(&self, resource_type: &ResourceType, amount: f64) -> Result<(), String>;
}

/// A mock implementation of `SystemResourceInterface` for initial development.
pub struct MockSystemResourceInterface;

#[async_trait]
impl SystemResourceInterface for MockSystemResourceInterface {
    async fn apply_resource_limits(
        &self,
        task_id: &str,
        resources: &HashMap<ResourceType, f64>,
    ) -> Result<(), String> {
        println!("MockSystemResourceInterface: Applying {:?} to task {}", resources, task_id);
        // Simulate OS-level calls
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        Ok(())
    }

    async fn release_resource_limits(&self, task_id: &str) -> Result<(), String> {
        println!("MockSystemResourceInterface: Releasing resources for task {}", task_id);
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        Ok(())
    }

    async fn provision_new_resource(&self, resource_type: &ResourceType, amount: f64) -> Result<(), String> {
        println!("MockSystemResourceInterface: Provisioning {} of {:?}", amount, resource_type);
        // Simulate a potentially longer operation for provisioning hardware/cloud resources
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        Ok(())
    }

    async fn deprovision_resource(&self, resource_type: &ResourceType, amount: f64) -> Result<(), String> {
        println!("MockSystemResourceInterface: Deprovisioning {} of {:?}", amount, resource_type);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        Ok(())
    }
}

/// ResourceAllocator is the execution layer that applies allocation decisions.
pub struct ResourceAllocator {
    system_interface: Box<dyn SystemResourceInterface>, // Use a trait object for flexibility
}

impl ResourceAllocator {
    pub fn new() -> Self {
        println!("ResourceAllocator initialized with MockSystemResourceInterface.");
        ResourceAllocator {
            system_interface: Box::new(MockSystemResourceInterface{}),
        }
    }

    /// Attempts to apply a set of resource allocations to specific tasks.
    /// This would be the interface to the OMEGA OS kernel or hypervisor for actual resource changes.
    pub async fn apply_allocations(
        &self,
        allocations: HashMap<String, HashMap<ResourceType, f64>>,
        current_resource_snapshot: &HashMap<ResourceType, f64>, // Available resources from monitor
        resource_requests: &[ResourceRequest], // Original requests to check for provisioning needs
    ) -> HashMap<String, AllocationResult> {
        println!("ResourceAllocator applying allocations...");
        let mut results = HashMap::new();

        for (task_id, assigned_resources) in allocations {
            let mut success = true;
            let mut message = String::new();

            // Check if provisioning is needed for this task
            if let Some(original_request) = resource_requests.iter().find(|req| req.task_id == task_id) {
                for (res_type, &req_amount) in &original_request.requested_resources {
                    let current_available = *current_resource_snapshot.get(res_type).unwrap_or(&0.0);
                    if req_amount > current_available + *assigned_resources.get(res_type).unwrap_or(&0.0) { // Simple check, might need more sophisticated logic
                        println!("Task {}: Detected potential need to provision more {:?}", task_id, res_type);
                        // In a truly autonomous system, the OptimizationEngine would decide to provision,
                        // and send a specific instruction to the allocator. Here, we simulate a reactive provision.
                        match self.system_interface.provision_new_resource(res_type, req_amount - current_available).await {
                            Ok(_) => println!("Task {}: Successfully provisioned more {:?}", task_id, res_type),
                            Err(e) => {
                                eprintln!("Task {}: Failed to provision more {:?}: {}", task_id, res_type, e);
                                success = false;
                                message = format!("Provisioning failed: {}", e);
                            }
                        }
                    }
                }
            }

            if success {
                match self.system_interface.apply_resource_limits(&task_id, &assigned_resources).await {
                    Ok(_) => message = "Resources successfully adjusted for task.".to_string(),
                    Err(e) => {
                        success = false;
                        message = format!("Failed to adjust resources: {}", e);
                    }
                }
            }

            results.insert(
                task_id.clone(),
                AllocationResult {
                    request_id: task_id,
                    allocated_resources: assigned_resources,
                    success,
                    message,
                },
            );
        }
        results
    }

    /// Releases resources back to the system.
    pub async fn release_resources(
        &self,
        task_id: &str,
        resources_to_release: &HashMap<ResourceType, f64>,
    ) -> Result<String, String> {
        println!("ResourceAllocator releasing resources for task: {}", task_id);
        match self.system_interface.release_resource_limits(task_id).await {
            Ok(_) => {
                // Optionally, deprovision if resources were specifically provisioned for this task
                // For now, a simple release is assumed.
                Ok(format!("Resources for task {} released.", task_id))
            }
            Err(e) => Err(format!("Failed to release resources for task {}: {}", task_id, e)),
        }
    }
}
