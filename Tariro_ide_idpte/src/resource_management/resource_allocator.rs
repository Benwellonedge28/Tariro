// Tariro_ide_idpte/src/resource_management/resource_allocator.rs

//! Manages the actual allocation and deallocation of system resources.

use crate::resource_management::resource_models::{
    ResourceRequest, ResourceType, AllocationPolicy, AllocationResult, WorkloadPriority, HealingAction,
};
use std::collections::{
    HashMap, HashSet, // Added HashSet for tracking active tasks
};
use anyhow::Result;
use tracing::{info, warn, error};
use async_trait::async_trait;
use chrono::Utc;

/// Abstract interaction with the underlying OS/Hypervisor for resource control.
#[async_trait]
pub trait SystemResourceManager: Send + Sync {
    /// Allocates resources to a specific task.
    async fn allocate(&self, task_id: &str, resources: &HashMap<ResourceType, f64>) -> Result<(), String>;
    /// Deallocates resources from a specific task.
    async fn deallocate(&self, task_id: &str, resources: &HashMap<ResourceType, f64>) -> Result<(), String>;
    /// Adjusts priority of a running task.
    async fn adjust_priority(&self, task_id: &str, priority: u8) -> Result<(), String>;
    /// Suspends a running task.
    async fn suspend_task(&self, task_id: &str) -> Result<(), String>;
    /// Resumes a suspended task.
    async fn resume_task(&self, task_id: &str) -> Result<(), String>;
    /// Requests additional system resources (e.g., from a cloud provider).
    async fn request_additional_resources( &self, resource_type: ResourceType, amount: f64) -> Result<(), String>;
}

/// Mock implementation of SystemResourceManager.
pub struct MockSystemResourceManager;

#[async_trait]
impl SystemResourceManager for MockSystemResourceManager {
    async fn allocate(&self, task_id: &str, resources: &HashMap<ResourceType, f64>) -> Result<(), String> {
        info!("MockSystemResourceManager: Allocated {:?} to {}.", resources, task_id);
        Ok(())
    }
    async fn deallocate(&self, task_id: &str, resources: &HashMap<ResourceType, f64>) -> Result<(), String> {
        info!("MockSystemResourceManager: Deallocated {:?} from {}.", resources, task_id);
        Ok(())
    }
    async fn adjust_priority(&self, task_id: &str, priority: u8) -> Result<(), String> {
        info!("MockSystemResourceManager: Adjusted priority of {} to {}.", task_id, priority);
        Ok(())
    }
    async fn suspend_task(&self, task_id: &str) -> Result<(), String> {
        warn!("MockSystemResourceManager: Suspended task {}.", task_id);
        Ok(())
    }
    async fn resume_task(&self, task_id: &str) -> Result<(), String> {
        info!("MockSystemResourceManager: Resumed task {}.", task_id);
        Ok(())
    }
    async fn request_additional_resources(&self, resource_type: ResourceType, amount: f64) -> Result<(), String> {
        info!("MockSystemResourceManager: Requested {} of {:?}.", amount, resource_type);
        Ok(())
    }
}

/// ResourceAllocator is responsible for executing resource allocation decisions.
pub struct ResourceAllocator {
    system_manager: Box<dyn SystemResourceManager>,
    active_tasks: HashSet<String>, // Keep track of tasks currently managed
    task_priorities: HashMap<String, WorkloadPriority>, // Dynamic priorities for tasks
}

impl ResourceAllocator {
    pub fn new() -> Self {
        println!("Resource Allocator initialized.");
        ResourceAllocator {
            system_manager: Box::new(MockSystemResourceManager{}),
            active_tasks: HashSet::new(),
            task_priorities: HashMap::new(),
        }
    }

    /// Dynamically adjusts the priority of a task.
    pub async fn adjust_task_priority(&mut self, task_id: &str, new_priority: u8) -> Result<(), String> {
        self.task_priorities.insert(task_id.to_string(), WorkloadPriority {
            task_id: task_id.to_string(),
            priority_score: new_priority,
            source: "ComplianceDirective".to_string(),
            enforced_until: None,
        });
        self.system_manager.adjust_priority(task_id, new_priority).await
    }

    /// Processes proposed allocations and applies them to the system.
    /// This version incorporates dynamic prioritization and self-healing.
    pub async fn apply_allocations(
        &mut self,
        mut proposed_allocations: HashMap<String, HashMap<ResourceType, f64>>,
        current_available_resources: &HashMap<ResourceType, f64>,
        original_requests: &[ResourceRequest],
    ) -> HashMap<String, AllocationResult> {
        info!("ResourceAllocator: Applying allocations. {} proposed, {} available initially.", proposed_allocations.len(), current_available_resources.len());
        let mut results = HashMap::new();
        let mut currently_available = current_available_resources.clone();

        // Sort requests by dynamic priority, then by original urgency
        let mut sorted_requests_ids: Vec<String> = proposed_allocations.keys().cloned().collect();
        sorted_requests_ids.sort_by(|a, b| {
            let prio_a = self.task_priorities.get(a).map_or(0, |wp| wp.priority_score);
            let prio_b = self.task_priorities.get(b).map_or(0, |wp| wp.priority_score);
            prio_b.cmp(&prio_a) // Higher priority first
        });

        for task_id in sorted_requests_ids {
            let allocation = proposed_allocations.remove(&task_id).unwrap(); // Should exist
            let original_request = original_requests.iter().find(|req| req.task_id == task_id);
            
            let mut success = true;
            let mut message = "Allocated successfully.".to_string();
            let mut actual_allocated = HashMap::new();

            for (res_type, &amount) in &allocation {
                let available = *currently_available.get(res_type).unwrap_or(&0.0);
                if available >= amount {
                    actual_allocated.insert(res_type.clone(), amount);
                    *currently_available.get_mut(res_type).unwrap() -= amount;
                } else {
                    // Attempt self-healing for resource contention
                    warn!("Resource contention for task {} on {:?}. Available: {}, Requested: {}. Initiating self-healing.", task_id, res_type, available, amount);
                    match self.initiate_self_healing(task_id.clone(), res_type.clone(), amount - available).await {
                        Ok(healing_performed) => {
                            info!("Self-healing performed for {}: {:?}", task_id, healing_performed);
                            // After healing, attempt allocation again (simplified for mock)
                            actual_allocated.insert(res_type.clone(), amount); // Assume healing was successful enough
                            if let Some(res_mut) = currently_available.get_mut(res_type) {
                                *res_mut -= amount; // Deduct resources as if they were made available
                            }
                        },
                        Err(e) => {
                            error!("Self-healing failed for {}: {}", task_id, e);
                            success = false;
                            message = format!("Insufficient resources ({:?}) after self-healing attempt.", res_type);
                            break;
                        }
                    }
                }
            }

            if success {
                if let Err(e) = self.system_manager.allocate(&task_id, &actual_allocated).await {
                    error!("System manager failed to allocate for task {}: {}", task_id, e);
                    success = false;
                    message = format!("System manager allocation failed: {}", e);
                } else {
                    self.active_tasks.insert(task_id.clone());
                }
            }
            results.insert(task_id.clone(), AllocationResult { 
                request_id: task_id,
                allocated_resources: actual_allocated,
                success, message
            });
        }

        results
    }

    /// Implements self-healing mechanisms for resource contention or failure.
    async fn initiate_self_healing(&mut self, offending_task_id: String, res_type: ResourceType, deficit_amount: f64) -> Result<HealingAction, String> {
        info!("Self-healing for task {}: need {} of {:?}.", offending_task_id, deficit_amount, res_type);
        
        // Strategies:
        // 1. Suspend lower priority tasks to free up resources.
        // 2. Request more resources from external (simulated) providers.
        // 3. Reconfigure existing resource usage for non-critical tasks.

        // Try suspending a low-priority task (if any)
        let mut suspended_task: Option<String> = None;
        let mut task_to_suspend = None;
        let current_time = Utc::now();

        // Find a task that is active, has low priority, and is not critical
        let mut candidates_to_suspend: Vec<&String> = self.active_tasks.iter().collect();
        candidates_to_suspend.sort_by_key(|task_id| self.task_priorities.get(*task_id).map_or(255, |wp| wp.priority_score)); // Sort by lowest priority first

        for task_id in candidates_to_suspend {
            let priority = self.task_priorities.get(task_id).map_or(0, |wp| wp.priority_score);
            // Simple heuristic: if priority is low and it's not a critical system task
            if priority < 100 && task_id != &offending_task_id && !task_id.contains("system_critical") {
                task_to_suspend = Some(task_id.clone());
                break;
            }
        }

        if let Some(task_id) = task_to_suspend {
            if self.system_manager.suspend_task(&task_id).await.is_ok() {
                info!("Self-healing: Suspended task {} to free resources.", task_id);
                self.active_tasks.remove(&task_id);
                suspended_task = Some(task_id.clone());
                return Ok(HealingAction::SuspendTask(task_id));
            }
        }

        // If suspending didn't work or no suitable task, request more resources
        warn!("Self-healing: No suitable task to suspend or suspension failed. Requesting additional resources for {:?}.", res_type);
        if self.system_manager.request_additional_resources(res_type.clone(), deficit_amount).await.is_ok() {
            info!("Self-healing: Requested additional resources for {:?}.", res_type);
            return Ok(HealingAction::RequestMoreResources(res_type, deficit_amount));
        }

        // Fallback: Notify human
        Err(format!("Automated self-healing failed for {}. Human intervention required.", offending_task_id))
    }
}
