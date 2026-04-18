//! Predicts future resource demands based on historical data and current trends.

use crate::resource_management::resource_models::{ResourceSnapshot, ResourceType, PredictionModelType};
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc, Duration};
use tracing::{info, warn};

const HISTORY_SIZE: usize = 100; // Number of past snapshots to keep for prediction

/// Trait for a learned prediction model.
/// This allows swapping different prediction algorithms (e.g., linear regression, LSTM, ARIMA).
pub trait LearnedPredictionModel: Send + Sync {
    /// Updates the model with a new resource snapshot.
    fn update_with_snapshot(&mut self, snapshot: &ResourceSnapshot);

    /// Predicts future resource usage for a given time horizon.
    /// Returns a map of predicted usage for each resource type.
    fn predict(&self, time_horizon_seconds: u64) -> HashMap<ResourceType, f64>;

    /// Returns the type of this prediction model.
    fn get_type(&self) -> PredictionModelType;

    /// Evaluates the current performance of the model (e.g., mean absolute error).
    /// In a real scenario, this would compare predictions with actual historical data.
    fn evaluate_performance(&self) -> f64;

    /// Triggers retraining or adaptation of the model.
    fn retrain(&mut self, full_history: &VecDeque<ResourceSnapshot>);
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
                predicted_usage.insert(res_type.clone(), 0.0);
                continue;
            }

            let sum: f64 = history.iter().map(|&(_, val)| val).sum();
            let avg = sum / history.len() as f64;

            // For demonstration, assume average usage persists over the horizon
            predicted_usage.insert(res_type.clone(), avg);
        }

        predicted_usage
    }

    fn get_type(&self) -> PredictionModelType {
        PredictionModelType::MovingAverage
    }

    fn evaluate_performance(&self) -> f64 {
        // Mock performance evaluation: higher is better
        0.75
    }

    fn retrain(&mut self, full_history: &VecDeque<ResourceSnapshot>) {
        info!("MovingAverageModel: Retraining with full history (size: {}).", full_history.len());
        // For MovingAverageModel, retraining might just mean re-populating its internal history_data
        // with relevant parts of `full_history` or re-calculating parameters.
        self.history_data.clear();
        for snapshot in full_history.iter().rev().take(self.window_size).cloned() {
            self.update_with_snapshot(&snapshot);
        }
    }
}

// Placeholder for a more advanced prediction model (e.g., ARIMA)
pub struct ArimaModel {
    // Internal ARIMA model state
}

impl ArimaModel {
    pub fn new() -> Self { ArimaModel {} }
}

impl LearnedPredictionModel for ArimaModel {
    fn update_with_snapshot(&mut self, _snapshot: &ResourceSnapshot) {
        // ARIMA model would update its internal state or be retrained periodically
    }
    fn predict(&self, time_horizon_seconds: u64) -> HashMap<ResourceType, f64> {
        // Mock ARIMA prediction
        info!("ARIMA Model: Performing mock prediction for {} seconds.", time_horizon_seconds);
        HashMap::new()
    }
    fn get_type(&self) -> PredictionModelType {
        PredictionModelType::Arima
    }
    fn evaluate_performance(&self) -> f64 {
        // Mock performance
        0.85
    }
    fn retrain(&mut self, full_history: &VecDeque<ResourceSnapshot>) {
        info!("ARIMA Model: Performing complex retraining with full history (size: {}).", full_history.len());
        // In a real implementation, this would involve fitting the ARIMA model
        // to the time series data extracted from `full_history`.
    }
}

/// ResourcePredictor uses historical data to forecast future resource needs.
pub struct ResourcePredictor {
    history: VecDeque<ResourceSnapshot>,
    // A map of active prediction models, one per resource type or a global best model
    prediction_models: HashMap<ResourceType, Box<dyn LearnedPredictionModel + Send + Sync>>,
    active_model_type: PredictionModelType,
    last_model_evaluation: DateTime<Utc>,
    model_evaluation_interval: Duration,
}

impl ResourcePredictor {
    pub fn new() -> Self {
        info!("ResourcePredictor initialized.");
        let mut models: HashMap<ResourceType, Box<dyn LearnedPredictionModel + Send + Sync>> = HashMap::new();
        // Initialize with default models for common resource types
        models.insert(ResourceType::Cpu, Box::new(MovingAverageModel::new(HISTORY_SIZE)));
        models.insert(ResourceType::Memory, Box::new(MovingAverageModel::new(HISTORY_SIZE)));
        models.insert(ResourceType::NetworkIo, Box::new(MovingAverageModel::new(HISTORY_SIZE)));
        models.insert(ResourceType::Storage, Box::new(ArimaModel::new())); // Example: use ARIMA for Storage

        ResourcePredictor {
            history: VecDeque::with_capacity(HISTORY_SIZE),
            prediction_models: models,
            active_model_type: PredictionModelType::MovingAverage, // Default starting model
            last_model_evaluation: Utc::now(),
            model_evaluation_interval: Duration::minutes(5), // Evaluate models every 5 minutes
        }
    }

    /// Feeds a new resource snapshot into the prediction model's history.
    pub fn feed_snapshot(&mut self, snapshot: ResourceSnapshot) {
        // Update the internal history
        if self.history.len() == HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(snapshot.clone());

        // Feed the snapshot to all active prediction models for their internal updates
        for (_, model) in self.prediction_models.iter_mut() {
            model.update_with_snapshot(&snapshot);
        }
    }

    /// Predicts future resource usage for a given time horizon (in seconds).
    /// This now iterates through models or uses a selected one.
    pub async fn predict_future_usage(
        &self,
        time_horizon_seconds: u64,
    ) -> HashMap<ResourceType, f64> {
        // For simplicity, we'll aggregate predictions from all models, or pick a 'best' one.
        // A more advanced system would use ensemble methods or a meta-predictor.
        let mut aggregated_predictions = HashMap::new();

        for (res_type, model) in &self.prediction_models {
            let prediction = model.predict(time_horizon_seconds);
            // For each resource type, take the prediction from the model that handles it
            if let Some(&val) = prediction.get(res_type) {
                 aggregated_predictions.insert(res_type.clone(), val);
            }
        }
        info!("ResourcePredictor aggregating predictions.");
        aggregated_predictions
    }

    /// Periodically evaluates prediction models and selects/retrains the best ones.
    pub async fn evaluate_and_select_models(&mut self) {
        if Utc::now().signed_duration_since(self.last_model_evaluation) < self.model_evaluation_interval {
            return; // Not time to evaluate yet
        }
        self.last_model_evaluation = Utc::now();

        info!("ResourcePredictor: Evaluating and selecting prediction models.");
        let mut best_performance = 0.0;
        let mut best_model_type = PredictionModelType::MovingAverage;

        // Evaluate existing models and retrain if necessary
        for (res_type, model) in self.prediction_models.iter_mut() {
            let performance = model.evaluate_performance();
            info!("Model {:?} for {:?} performance: {:.2}", model.get_type(), res_type, performance);

            if performance > best_performance {
                best_performance = performance;
                best_model_type = model.get_type();
            }

            // Heuristic: If performance is below a threshold, retrain the model
            if performance < 0.7 && self.history.len() > 10 {
                warn!("Model {:?} for {:?} performing poorly, initiating retraining.", model.get_type(), res_type);
                model.retrain(&self.history);
            }
        }

        // Dynamic model selection: If a new model type (e.g., ARIMA) performs better, switch or introduce it.
        // This part would be more complex, involving dynamically loading/compiling new model implementations.
        if best_model_type != self.active_model_type {
            info!("ResourcePredictor: Switching active prediction model to {:?}", best_model_type);
            self.active_model_type = best_model_type;
            // In a real system, this would involve instantiating and integrating the new model
        }
    }
}
