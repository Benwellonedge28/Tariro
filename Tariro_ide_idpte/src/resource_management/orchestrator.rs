//! Central orchestration logic for the Self-Optimizing Resource Allocation Engine (S.O.R.A.E.).

use crate::resource_management::resource_monitor::ResourceMonitor;
use crate::resource_management::resource_predictor::ResourcePredictor;
use crate::resource_management::optimization_engine::OptimizationEngine;
use crate::resource_management::resource_allocator::ResourceAllocator;
use crate::resource_management::resource_models::ResourceRequest;

use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// The SORAEOptions struct holds configuration for the S.O.R.A.E. Orchestrator.
pub struct SORAEOptions {
    pub monitoring_interval_seconds: u64,
    pub prediction_horizon_seconds: u64,
    pub system_id: String,
}

impl Default for SORAEOptions {
    fn default() -> Self {
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
        }
    }

    /// Returns a sender to submit new resource requests to the orchestrator.
    pub fn get_request_sender(&self) -> mpsc::Sender<ResourceRequest> {
        self.request_tx.clone()
    }

    /// Starts the main S.O.R.A.E. loop as an asynchronous task.
    pub async fn start_loop(&mut self, options: SORAEOptions) {
        println!("S.O.R.A.E. Orchestrator started with interval {}s, prediction horizon {}s.",
                 options.monitoring_interval_seconds, options.prediction_horizon_seconds);

        let predictor_arc = self.predictor.clone();
        let mut request_rx = self.request_rx.take().expect("Request receiver already taken");

        // Spawn a task for continuous monitoring
        let monitor_interval = options.monitoring_interval_seconds;
        let monitor_instance = self.monitor.new(options.system_id.clone()); // Recreate for the task
        tokio::spawn(async move {
            monitor_instance.start_monitoring(monitor_interval, |snapshot| {
                if let Ok(mut predictor) = predictor_arc.lock() {
                    predictor.feed_snapshot(snapshot);
                }
            }).await;
        });

        // Main orchestration loop
        loop {
            tokio::time::sleep(Duration::from_secs(options.monitoring_interval_seconds)).await;

            // 1. Get current state
            let current_snapshot = self.monitor.get_current_snapshot().await;

            // 2. Get pending resource requests
            let mut pending_requests: Vec<ResourceRequest> = Vec::new();
            while let Ok(req) = request_rx.try_recv() {
                pending_requests.push(req);
            }

            // 3. Predict future demand
            let predicted_demand = if let Ok(predictor) = self.predictor.lock() {
                predictor.predict_future_usage(options.prediction_horizon_seconds).await
            } else {
                eprintln!("Failed to lock predictor for prediction.");
                HashMap::new()
            };

            // 4. Propose optimal allocation
            let proposed_allocations = self.optimizer.propose_allocation(
                &current_snapshot,
                &predicted_demand,
                &pending_requests,
            ).await;

            // 5. Apply ethical filters (placeholder for now)
            let ethically_approved_allocations = self.optimizer.apply_ethical_filters(
                proposed_allocations
            ).await;

            // 6. Apply allocations
            let results = self.allocator.apply_allocations(ethically_approved_allocations).await;

            // Log or act on results
            for (task_id, result) in results {
                if result.success {
                    println!("Task {}: Resource allocation successful.", task_id);
                } else {
                    eprintln!("Task {}: Resource allocation failed: {}", task_id, result.message);
                }
            }

            // Consider dynamically adjusting monitoring_interval_seconds or prediction_horizon_seconds
            // based on system load or criticality to be "infinitely advanced and supremely autonomous"
        }
    }
}
