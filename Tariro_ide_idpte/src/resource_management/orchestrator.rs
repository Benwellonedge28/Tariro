//! Central orchestration logic for the Self-Optimizing Resource Allocation Engine (S.O.R.A.E.).

use crate::resource_management::resource_monitor::ResourceMonitor;
use crate::resource_management::resource_predictor::ResourcePredictor;
use crate::resource_management::optimization_engine::OptimizationEngine;
use crate::resource_management::resource_allocator::ResourceAllocator;
use crate::resource_management::resource_models::{ResourceRequest, ResourceSnapshot};

use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Duration;
use tracing::{info, warn, error};

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
    optimizer: OptimizationEngine,
    allocator: ResourceAllocator,
    // Channel for incoming resource requests from other parts of the AGI/OS
    request_rx: mpsc::Receiver<ResourceRequest>,
    request_tx: mpsc::Sender<ResourceRequest>,
    current_options: Arc<Mutex<SORAEOptions>>, // Dynamically configurable options
}

impl SORAEOrcestrator {
    pub fn new(options: SORAEOptions) -> Self {
        let (request_tx, request_rx) = mpsc::channel(100); // Buffer for 100 pending requests
        SORAEOrcestrator {
            monitor: ResourceMonitor::new(options.system_id.clone()),
            predictor: Arc::new(Mutex::new(ResourcePredictor::new())),
            optimizer: OptimizationEngine::new(),
            allocator: ResourceAllocator::new(),
            request_rx,
            request_tx,
            current_options: Arc::new(Mutex::new(options)),
        }
    }

    /// Returns a sender to submit new resource requests to the orchestrator.
    pub fn get_request_sender(&self) -> mpsc::Sender<ResourceRequest> {
        self.request_tx.clone()
    }

    /// Starts the main S.O.R.A.E. loop as an asynchronous task.
    pub async fn start_loop(&mut self) {
        let initial_options = self.current_options.lock().unwrap().clone();
        info!("S.O.R.A.E. Orchestrator started with interval {}s, prediction horizon {}s.",
                 initial_options.monitoring_interval_seconds, initial_options.prediction_horizon_seconds);

        let predictor_arc = self.predictor.clone();
        let mut request_rx = self.request_rx.take().expect("Request receiver already taken");

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

            // 1. Get current state
            let current_snapshot = self.monitor.get_current_snapshot().await;

            // 2. Get pending resource requests
            let mut pending_requests: Vec<ResourceRequest> = Vec::new();
            while let Ok(req) = request_rx.try_recv() {
                pending_requests.push(req);
            }

            // 3. Predict future demand
            let predicted_demand = if let Ok(predictor) = self.predictor.lock() {
                predictor.predict_future_usage(current_options.prediction_horizon_seconds).await
            } else {
                error!("Failed to lock predictor for prediction.");
                HashMap::new()
            };

            // 4. Propose optimal allocation
            let proposed_allocations = self.optimizer.propose_allocation(
                &current_snapshot,
                &predicted_demand,
                &pending_requests,
            ).await;

            // 5. Apply ethical filters
            let ethically_approved_allocations = self.optimizer.apply_ethical_filters(
                proposed_allocations
            ).await;

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
            for (task_id, result) in results {
                if result.success {
                    info!("Task {}: Resource allocation successful.", task_id);
                    successful_allocations += 1;
                } else {
                    warn!("Task {}: Resource allocation failed: {}", task_id, result.message);
                    failed_allocations += 1;
                }
            }

            // 7. Self-Adaptive Tuning (new logic here)
            self.perform_self_tuning(successful_allocations, failed_allocations, &current_snapshot).await;
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
}
