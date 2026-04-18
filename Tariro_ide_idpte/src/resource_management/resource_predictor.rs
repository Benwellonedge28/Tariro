//! Predicts future resource demands based on historical data and current trends.

use crate::resource_management::resource_models::{ResourceSnapshot, ResourceType};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc, Duration};

const HISTORY_SIZE: usize = 100; // Number of past snapshots to keep for prediction

/// Trait for a learned prediction model.
/// This allows swapping different prediction algorithms (e.g., linear regression, LSTM, ARIMA).
pub trait LearnedPredictionModel: Send + Sync {
    /// Updates the model with a new resource snapshot.
    fn update_with_snapshot(&mut self, snapshot: &ResourceSnapshot);

    /// Predicts future resource usage for a given time horizon.
    /// Returns a map of predicted usage for each resource type.
    fn predict(&self, time_horizon_seconds: u64) -> HashMap<ResourceType, f64>;
}

/// A simple moving average prediction model for demonstration.
pub struct MovingAverageModel {
    history_data: HashMap<ResourceType, VecDeque<(DateTime<Utc>, f64)>>,
    window_size: usize, // Number of data points to consider for the moving average
}

impl MovingAverageModel {
    pub fn new(window_size: usize) -> Self {
        MovingAverageModel {
            history_data: HashMap::new(),
            window_size,
        }
    }
}

impl LearnedPredictionModel for MovingAverageModel {
    fn update_with_snapshot(&mut self, snapshot: &ResourceSnapshot) {
        let now = Utc::now();
        for (res_type, &used_value) in &snapshot.used_resources {
            let entry = self.history_data.entry(res_type.clone()).or_insert_with(|| VecDeque::with_capacity(self.window_size));
            if entry.len() == self.window_size {
                entry.pop_front();
            }
            entry.push_back((now, used_value));
        }
    }

    fn predict(&self, time_horizon_seconds: u64) -> HashMap<ResourceType, f64> {
        let mut predicted_usage = HashMap::new();

        for (res_type, history) in &self.history_data {
            if history.is_empty() {
                continue;
            }

            // Calculate simple moving average as prediction
            let sum: f64 = history.iter().map(|&(_, val)| val).sum();
            let avg = sum / history.len() as f64;

            // For demonstration, assume average usage persists over the horizon
            // Advanced models would extrapolate trends, seasonality, etc.
            predicted_usage.insert(res_type.clone(), avg);
        }

        // In a real advanced system, the prediction would also consider the time_horizon_seconds
        // to project potential growth or decay based on learned trends.
        // For example, if a trend of 1% increase per minute is learned, it would apply that.
        predicted_usage
    }
}

/// ResourcePredictor uses historical data to forecast future resource needs.
pub struct ResourcePredictor {
    history: VecDeque<ResourceSnapshot>,
    prediction_model: Box<dyn LearnedPredictionModel + Send + Sync>, // Use a trait object for flexibility
}

impl ResourcePredictor {
    pub fn new() -> Self {
        println!("ResourcePredictor initialized with MovingAverageModel.");
        ResourcePredictor {
            history: VecDeque::with_capacity(HISTORY_SIZE),
            prediction_model: Box::new(MovingAverageModel::new(10)), // Default to MovingAverageModel with window 10
        }
    }

    /// Feeds a new resource snapshot into the prediction model's history.
    pub fn feed_snapshot(&mut self, snapshot: ResourceSnapshot) {
        // Update the internal history (used for debugging/logging, or if the model needs raw snapshots)
        if self.history.len() == HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(snapshot.clone()); // Clone for history storage

        // Also feed the snapshot to the active prediction model
        self.prediction_model.update_with_snapshot(&snapshot);

        // In a real system, this would also update/retrain any internal prediction models
    }

    /// Predicts future resource usage for a given time horizon (in seconds).
    /// Returns a predicted ResourceSnapshot or a summary of predicted changes.
    pub async fn predict_future_usage(
        &self,
        time_horizon_seconds: u64,
    ) -> HashMap<ResourceType, f64> {
        println!("ResourcePredictor predicting for next {} seconds.", time_horizon_seconds);
        self.prediction_model.predict(time_horizon_seconds)
    }
}
