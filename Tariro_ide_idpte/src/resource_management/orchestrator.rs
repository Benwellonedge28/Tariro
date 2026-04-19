// Tariro_ide_idpte/src/resource_management/orchestrator.rs

//! Central orchestration logic for the Self-Optimizing Resource Allocation Engine (S.O.R.A.E.).

use crate::resource_management::resource_monitor::ResourceMonitor;
use crate::resource_management::resource_predictor::ResourcePredictor;
use crate::resource_management::optimization_engine::{OptimizationEngine, OptimizationStrategyType};
use crate::resource_management::resource_allocator::ResourceAllocator;
use crate::resource_management::resource_models::{ResourceRequest, ResourceSnapshot, ComplianceDirective, ResourceType};

use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn, error};

// Import ProcessManager for integration
use crate::omega_os::process_manager::{ProcessManager, ProcessId, ProcessStatus as OsProcessStatus}; 

/// The SORAEOptions struct holds configuration for the S.O.R.A.E. Orchestrator.
#[derive(Debug, Clone)] // Make it Debug and Clone for easier handling
pub struct SORAEOptions {
    pub monitoring_interval_seconds: u64,
    pub prediction_horizon_seconds: u64,
    pub system_id: String,
}

impl Default for SORAEOptions {
    pub fn default() -> Self {
        SORAEOptions {
            monitoring_interval_seconds: 5, // Monitor every 5 seconds
            prediction_horizon_seconds: 60, // Predict for the next 60 seconds
            system_id: "tariro-main-agi-node-001".to_string(),
        }
    }
}

/// SORAEOrcestrator manages the lifecycle of resource optimization.
pub struct SORAEOrcestrator {
    monitor: ResourceMonitor,
    predictor: Arc<Mutex<ResourcePredictor>>,
    optimizer: Arc<Mutex<OptimizationEngine>>,
    allocator: ResourceAllocator,
    process_manager: Arc<ProcessManager>, // Integrated ProcessManager
    // Channel for incoming resource requests from other parts of the AGI/OS
    request_rx: mpsc::Receiver<ResourceRequest>,
    request_tx: mpsc::Sender<ResourceRequest>,
    // Channel for incoming compliance directives from ALEC-A
    compliance_directive_rx: mpsc::Receiver<ComplianceDirective>,
    compliance_directive_tx: mpsc::Sender<ComplianceDirective>,
    current_options: Arc<Mutex<SORAEOptions>>,
}

impl SORAEOrcestrator {
    pub fn new(options: SORAEOptions, process_manager: Arc<ProcessManager>) -> Self {
        let (request_tx, request_rx) = mpsc::channel(100); // Buffer for 100 pending requests
        let (compliance_directive_tx, compliance_directive_rx) = mpsc::channel(50); // Buffer for 50 directives
        SORAEOrcestrator {
            monitor: ResourceMonitor::new(options.system_id.clone()),
            predictor: Arc::new(Mutex::new(ResourcePredictor::new())),
            optimizer: Arc::new(Mutex::new(OptimizationEngine::new())),
            allocator: ResourceAllocator::new(),
            process_manager,
            request_rx,
            request_tx,
            compliance_directive_rx,
            compliance_directive_tx,
            current_options: Arc::new(Mutex::new(options)),
        }
    }

    /// Returns a sender to submit new resource requests to the orchestrator.
    pub fn get_request_sender(&self) -> mpsc::Sender<ResourceRequest> {
        self.request_tx.clone()
    }

    /// Returns a sender to submit new compliance directives to the orchestrator.
    pub fn get_compliance_directive_sender(&self) -> mpsc::Sender<ComplianceDirective> {
        self.compliance_directive_tx.clone()
    }

    /// Starts the main S.O.R.A.E. loop as an asynchronous task.
    pub async fn start_loop(&mut self) {
        let initial_options = self.current_options.lock().unwrap().clone();
        info!("S.O.R.A.E. Orchestrator started with interval {}s, prediction horizon {}s.",
                 initial_options.monitoring_interval_seconds, initial_options.prediction_horizon_seconds);

        let predictor_arc = self.predictor.clone();
        let optimizer_arc = self.optimizer.clone();
        let process_manager_arc = self.process_manager.clone(); // Clone for use in spawned tasks if needed
        let mut request_rx = self.request_rx.take().expect("Request receiver already taken");
        let mut compliance_directive_rx = self.compliance_directive_rx.take().expect("Compliance directive receiver already taken");

        // Spawn a task for continuous monitoring
        let monitor_options_arc = self.current_options.clone();
        let monitor_system_id = initial_options.system_id.clone();
        tokio::spawn(async move {
            let monitor_instance = ResourceMonitor::new(monitor_system_id);
            loop {
                let current_interval = monitor_options_arc.lock().unwrap().monitoring_interval_seconds;
                let snapshot = monitor_instance.get_current_snapshot().await;
                if let Ok(mut predictor) = predictor_arc.lock() {
                    predictor.feed_snapshot(snapshot);
                }
                tokio::time::sleep(Duration::from_secs(current_interval)).await;
            }
        });

        // Main orchestration loop
        loop {
            let current_options = self.current_options.lock().unwrap().clone();
            tokio::time::sleep(Duration::from_secs(current_options.monitoring_interval_seconds)).await;

            // Process incoming compliance directives
            while let Ok(directive) = compliance_directive_rx.try_recv() {
                self.apply_compliance_directive(directive).await;
            }

            // 1. Get current state
            let current_snapshot = self.monitor.get_current_snapshot().await;

            // 2. Get pending resource requests
            let mut pending_requests: Vec<ResourceRequest> = Vec::new();
            while let Ok(req) = request_rx.try_recv() {
                pending_requests.push(req);
            }

            // 3. Predict future demand
            // Also evaluate and select prediction models dynamically
            if let Ok(mut predictor) = self.predictor.lock() {
                predictor.evaluate_and_select_models().await;
            }
            let predicted_demand = if let Ok(predictor) = self.predictor.lock() {
                predictor.predict_future_usage(current_options.prediction_horizon_seconds).await
            } else {
                error!("Failed to lock predictor for prediction.");
                HashMap::new()
            };

            // 4. Propose optimal allocation
            let proposed_allocations = if let Ok(optimizer) = self.optimizer.lock() {
                optimizer.propose_allocation(
                    &current_snapshot,
                    &predicted_demand,
                    &pending_requests,
                ).await
            } else {
                error!("Failed to lock optimizer for proposing allocation.");
                HashMap::new()
            };

            // 5. Apply ethical filters
            let ethically_approved_allocations = if let Ok(optimizer) = self.optimizer.lock() {
                optimizer.apply_ethical_filters(
                    proposed_allocations
                ).await
            } else {
                error!("Failed to lock optimizer for applying ethical filters.");
                HashMap::new()
            };

            // 6. Apply allocations, considering current snapshot and original requests for provisioning
            let current_resources_map = current_snapshot.available_resources.clone();
            let results = self.allocator.apply_allocations(
                ethically_approved_allocations,
                &current_resources_map,
                &pending_requests,
            ).await;

            // Log or act on results and perform self-adaptive tuning
            let mut successful_allocations = 0;
            let mut failed_allocations = 0;
            for (task_id, result) in results.iter() {
                if result.success {
                    info!("Task {}: Resource allocation successful.", task_id);
                    successful_allocations += 1;
                } else {
                    warn!("Task {}: Resource allocation failed: {}", task_id, result.message);
                    failed_allocations += 1;
                }
            }

            // 7. S.O.R.A.E. Self-Adaptive Tuning
            self.perform_self_tuning(successful_allocations, failed_allocations, &current_snapshot).await;

            // 8. Optimization Strategy Learning from Feedback
            if let Ok(mut optimizer) = self.optimizer.lock() {
                optimizer.learn_from_feedback(&pending_requests, &results).await;
                // Example of dynamic strategy change: if RL is performing better than greedy, switch.
                // This would be driven by performance metrics from the optimizer itself.
                // For now, a simple switch if RL is implemented and deemed superior.
                if optimizer.get_type() == OptimizationStrategyType::Greedy && successful_allocations > 0 && failed_allocations == 0 {
                    // If greedy is doing well, might consider switching to RL for exploration/better long-term
                    // or if the system hits a complex scenario. This is highly heuristic.
                    // optimizer.set_strategy(OptimizationStrategyType::ReinforcementLearning).await;
                }
            }
        }
    }

    /// Performs self-adaptive tuning of S.O.R.A.E. parameters.
    async fn perform_self_tuning(&self, successful: usize, failed: usize, snapshot: &ResourceSnapshot) {
        let mut options = self.current_options.lock().unwrap();

        // Simple heuristic: if many allocations are failing, increase monitoring frequency
        if failed > 0 && options.monitoring_interval_seconds > 1 {
            options.monitoring_interval_seconds = (options.monitoring_interval_seconds as f64 * 0.8).max(1.0) as u64;
            info!("S.O.R.A.E. Self-Tuning: Increased monitoring frequency to {}s due to allocation failures.", options.monitoring_interval_seconds);
        }
        // If resources are consistently low, maybe increase prediction horizon to anticipate more
        else if snapshot.available_resources.values().any(|&v| v < 0.2) && options.prediction_horizon_seconds < 300 {
            options.prediction_horizon_seconds = (options.prediction_horizon_seconds as f64 * 1.2).min(300.0) as u64;
            info!("S.O.R.A.E. Self-Tuning: Increased prediction horizon to {}s due to low available resources.", options.prediction_horizon_seconds);
        }
        // If everything is stable, reduce monitoring frequency to save resources
        else if successful > 0 && failed == 0 && options.monitoring_interval_seconds < SORAEOptions::default().monitoring_interval_seconds {
            options.monitoring_interval_seconds = (options.monitoring_interval_seconds as f64 * 1.1).min(SORAEOptions::default().monitoring_interval_seconds as f64) as u64;
            info!("S.O.R.A.E. Self-Tuning: Reduced monitoring frequency to {}s due to stable operations.", options.monitoring_interval_seconds);
        }
        // More advanced tuning would involve reinforcement learning or a dedicated meta-controller AGI module
    }

    /// Applies a compliance directive received from ALEC-A.
    pub async fn apply_compliance_directive(&mut self, directive: ComplianceDirective) {
        info!("S.O.R.A.E.: Applying compliance directive: {:?}", directive);
        match directive {
            ComplianceDirective::EnforceResourceIsolation(entity_id) => {
                warn!("S.O.R.A.E.: Enforcing resource isolation for {}. This may impact performance.", entity_id);
                // Logic to set up cgroups, namespaces, or other OS-level isolation for the entity_id
            },
            ComplianceDirective::AdjustTaskPriority(task_id, new_priority) => {
                info!("S.O.R.A.E.: Adjusting priority for task {} to {}.", task_id, new_priority);
                // Logic to update the priority of the specified task in the allocator or scheduler
                self.allocator.adjust_task_priority(&task_id, new_priority).await;
            },
            ComplianceDirective::CapResourceUsage(entity_id, res_type, max_value) => {
                info!("S.O.R.A.E.: Capping usage of {:?} for {} to {}.", res_type, entity_id, max_value);
                // Logic to enforce a hard cap on resource usage for a given entity
            },
            ComplianceDirective::RequestHumanReview(entity_id) => {
                warn!("S.O.R.A.E.: Human review requested for {} due to compliance directive.", entity_id);
                // Trigger an alert or notification to human operators
            },
            ComplianceDirective::Custom(action, value) => {
                info!("S.O.R.A.E.: Applying custom compliance action: {} with value {}.", action, value);
                if action == "suspend_agi_op" {
                    if let Err(e) = self.process_manager.pause_process(&value).await {
                        error!("S.O.R.A.E.: Failed to pause process {}: {}", value, e);
                    }
                } else if action == "resume_agi_op" {
                    if let Err(e) = self.process_manager.resume_process(&value).await {
                        error!("S.O.R.A.E.: Failed to resume process {}: {}", value, e);
                    }
                }
            },
        }
    }
}
