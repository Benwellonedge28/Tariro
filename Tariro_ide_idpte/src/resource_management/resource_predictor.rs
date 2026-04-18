//! Predicts future resource demands based on historical data and current trends.

use crate::resource_management::resource_models::{ResourceSnapshot, ResourceType};
use std::collections::{HashMap, VecDeque};

const HISTORY_SIZE: usize = 100; // Number of past snapshots to keep for prediction

/// ResourcePredictor uses historical data to forecast future resource needs.
pub struct ResourcePredictor {
    history: VecDeque<ResourceSnapshot>,
    // Potentially include trained models or prediction algorithms here
}

impl ResourcePredictor {
    pub fn new() -> Self {
        println!("ResourcePredictor initialized.");
        ResourcePredictor {
            history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    /// Feeds a new resource snapshot into the prediction model's history.
    pub fn feed_snapshot(&mut self, snapshot: ResourceSnapshot) {
        if self.history.len() == HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(snapshot);
        // In a real system, this would also update/retrain any internal prediction models
    }

    /// Predicts future resource usage for a given time horizon (in seconds).
    /// Returns a predicted ResourceSnapshot or a summary of predicted changes.
    pub async fn predict_future_usage(
        &self,
        time_horizon_seconds: u64,
    ) -> HashMap<ResourceType, f64> {
        // Placeholder for actual prediction logic.
        // This could involve:
        // - Simple moving averages
        // - Exponential smoothing
        // - Time series models (ARIMA, Prophet)
        // - Machine learning models (LSTMs, Transformers) if historical data is rich enough

        let mut predicted_usage = HashMap::new();

        if let Some(latest_snapshot) = self.history.back() {
            // For initial mock, let's assume a slight linear increase based on recent trends
            // In a real scenario, this would be much more sophisticated.
            let growth_factor = 1.0 + (time_horizon_seconds as f64 / 3600.0) * 0.01; // 1% growth per hour
            for (res_type, &used_value) in &latest_snapshot.used_resources {
                predicted_usage.insert(res_type.clone(), used_value * growth_factor);
            }
        } else {
            println!("Warning: No history available for prediction. Returning empty prediction.");
        }

        predicted_usage
    }
}
