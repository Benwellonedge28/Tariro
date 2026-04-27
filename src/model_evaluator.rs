/// Module for Predictive Model Evaluation and Refinement in Rust.
///
/// This module conceptually outlines a system for continuously evaluating the
/// performance of predictive models. It tracks their accuracy and other metrics,
/// and can trigger recommendations for refinement or retraining.

use std::collections::VecDeque;
use std::fmt;

/// Represents a single prediction outcome with its actual observed value.
#[derive(Debug, Clone, PartialEq)]
pub struct Prediction<T> {
    pub predicted_value: T,
    pub actual_value: T,
    pub timestamp: u64,
}

/// Defines various metrics used for model evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationMetric {
    Accuracy(f32),
    Precision(f32),
    Recall(f32),
    F1Score(f32),
    MAE(f32), // Mean Absolute Error for regression
    RMSE(f32), // Root Mean Squared Error for regression
}

impl fmt::Display for EvaluationMetric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvaluationMetric::Accuracy(val) => write!(f, "Accuracy: {:.2}%", val * 100.0),
            EvaluationMetric::Precision(val) => write!(f, "Precision: {:.2}%", val * 100.0),
            EvaluationMetric::Recall(val) => write!(f, "Recall: {:.2}%", val * 100.0),
            EvaluationMetric::F1Score(val) => write!(f, "F1-Score: {:.2}", val),
            EvaluationMetric::MAE(val) => write!(f, "MAE: {:.2}", val),
            EvaluationMetric::RMSE(val) => write!(f, "RMSE: {:.2}", val),
        }
    }
}

/// The Model Evaluator component.
/// For simplicity, this example focuses on a single type of numerical prediction.
pub struct ModelEvaluator {
    predictions: VecDeque<Prediction<f32>>, // Store recent predictions for rolling evaluation
    max_predictions_to_keep: usize,
}

impl ModelEvaluator {
    pub fn new(max_predictions_to_keep: usize) -> Self {
        ModelEvaluator { predictions: VecDeque::new(), max_predictions_to_keep }
    }

    /// Records a new prediction and its actual outcome.
    pub fn record_prediction(&mut self, predicted_value: f32, actual_value: f32, timestamp: u64) {
        let prediction = Prediction {
            predicted_value,
            actual_value,
            timestamp,
        };
        self.predictions.push_back(prediction);
        if self.predictions.len() > self.max_predictions_to_keep {
            self.predictions.pop_front();
        }
        println!("  [Evaluator]: Recorded prediction (P: {:.2}, A: {:.2}).", predicted_value, actual_value);
    }

    /// Calculates various evaluation metrics for the stored predictions.
    pub fn calculate_metrics(&self) -> Vec<EvaluationMetric> {
        if self.predictions.is_empty() {
            return vec![];
        }

        let mut total_absolute_error = 0.0;
        let mut total_squared_error = 0.0;
        let mut num_correct = 0;
        let total_predictions = self.predictions.len() as f32;

        for p in &self.predictions {
            total_absolute_error += (p.predicted_value - p.actual_value).abs();
            total_squared_error += (p.predicted_value - p.actual_value).powi(2);

            // For simplicity, assume "correct" if within a small epsilon for numerical prediction
            if (p.predicted_value - p.actual_value).abs() < 0.5 {
                num_correct += 1;
            }
        }

        let mae = total_absolute_error / total_predictions;
        let rmse = (total_squared_error / total_predictions).sqrt();
        let accuracy = num_correct as f32 / total_predictions;

        vec![
            EvaluationMetric::MAE(mae),
            EvaluationMetric::RMSE(rmse),
            EvaluationMetric::Accuracy(accuracy),
            // Precision, Recall, F1 would require a classification context with true/false positives/negatives
        ]
    }

    /// Recommends refinement based on current metric values.
    pub fn recommend_refinement(&self, metrics: &[EvaluationMetric]) -> Option<String> {
        for metric in metrics {
            match metric {
                EvaluationMetric::MAE(val) => {
                    if *val > 5.0 { return Some(format!("High MAE ({:.2}). Model may need re-calibration or new features.", val)); }
                },
                EvaluationMetric::RMSE(val) => {
                    if *val > 10.0 { return Some(format!("High RMSE ({:.2}). Consider a different model architecture.", val)); }
                },
                EvaluationMetric::Accuracy(val) => {
                    if *val < 0.8 { return Some(format!("Accuracy ({:.2}%) is below target. Investigate data drift or model decay.", val * 100.0)); }
                },
                _ => {},
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_prediction() {
        let mut evaluator = ModelEvaluator::new(3);
        evaluator.record_prediction(10.0, 10.2, 1);
        evaluator.record_prediction(11.0, 10.9, 2);
        assert_eq!(evaluator.predictions.len(), 2);

        evaluator.record_prediction(12.0, 11.5, 3);
        evaluator.record_prediction(13.0, 13.1, 4);
        assert_eq!(evaluator.predictions.len(), 3); // Max predictions reached, oldest removed
        assert_eq!(evaluator.predictions.front().unwrap().timestamp, 2);
    }

    #[test]
    fn test_calculate_metrics() {
        let mut evaluator = ModelEvaluator::new(10);
        evaluator.record_prediction(10.0, 10.0, 1);
        evaluator.record_prediction(11.0, 11.0, 2);
        evaluator.record_prediction(12.0, 11.5, 3);
        evaluator.record_prediction(13.0, 13.0, 4);

        let metrics = evaluator.calculate_metrics();
        let mae = metrics.iter().find_map(|m| if let EvaluationMetric::MAE(v) = m { Some(*v) } else { None }).unwrap();
        let accuracy = metrics.iter().find_map(|m| if let EvaluationMetric::Accuracy(v) = m { Some(*v) } else { None }).unwrap();

        assert!((mae - 0.125).abs() < 0.001);
        assert!((accuracy - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_recommend_refinement() {
        let evaluator = ModelEvaluator::new(10);
        let low_mae_metrics = vec![EvaluationMetric::MAE(1.0)];
        assert!(evaluator.recommend_refinement(&low_mae_metrics).is_none());

        let high_mae_metrics = vec![EvaluationMetric::MAE(5.1)];
        assert!(evaluator.recommend_refinement(&high_mae_metrics).is_some());
        assert!(evaluator.recommend_refinement(&high_mae_metrics).unwrap().contains("High MAE"));

        let low_accuracy_metrics = vec![EvaluationMetric::Accuracy(0.79)];
        assert!(evaluator.recommend_refinement(&low_accuracy_metrics).is_some());
        assert!(evaluator.recommend_refinement(&low_accuracy_metrics).unwrap().contains("Accuracy is below target"));
    }
}

fn main() {
    println!("\n--- Predictive Model Evaluation and Refinement Demonstration ---");

    let mut evaluator = ModelEvaluator::new(100); // Keep last 100 predictions

    // 1. Simulate a series of predictions and actual outcomes
    println!("\n--- Recording Predictions ---");
    evaluator.record_prediction(10.2, 10.0, 1);
    evaluator.record_prediction(15.1, 15.0, 2);
    evaluator.record_prediction(20.5, 21.0, 3);
    evaluator.record_prediction(8.9, 9.0, 4);
    evaluator.record_prediction(12.3, 12.0, 5);
    evaluator.record_prediction(30.0, 35.0, 6); // A larger error
    evaluator.record_prediction(40.1, 40.0, 7);
    evaluator.record_prediction(5.5, 5.0, 8);

    // 2. Calculate evaluation metrics based on recorded predictions
    println!("\n--- Calculating Metrics ---");
    let metrics = evaluator.calculate_metrics();
    for metric in &metrics {
        println!("  {}", metric);
    }

    // 3. Recommend refinement based on the calculated metrics
    println!("\n--- Checking for Refinement Recommendations ---");
    if let Some(recommendation) = evaluator.recommend_refinement(&metrics) {
        println!("  Recommendation: {}", recommendation);
    } else {
        println!("  No immediate refinement recommended. Model performing within acceptable thresholds.");
    }

    // Simulate a scenario where accuracy drops (e.g., due to data drift)
    println!("\n--- Simulating Model Decay/Drift (Lower Accuracy) ---");
    let mut degraded_evaluator = ModelEvaluator::new(10);
    degraded_evaluator.record_prediction(10.0, 10.0, 1);
    degraded_evaluator.record_prediction(11.0, 11.0, 2);
    degraded_evaluator.record_prediction(12.0, 15.0, 3); // Large error
    degraded_evaluator.record_prediction(13.0, 18.0, 4); // Large error
    degraded_evaluator.record_prediction(14.0, 14.1, 5);
    degraded_evaluator.record_prediction(15.0, 15.2, 6);
    degraded_evaluator.record_prediction(16.0, 20.0, 7); // Large error
    degraded_evaluator.record_prediction(17.0, 17.5, 8);

    let degraded_metrics = degraded_evaluator.calculate_metrics();
    for metric in &degraded_metrics {
        println!("  {}", metric);
    }
    if let Some(recommendation) = degraded_evaluator.recommend_refinement(&degraded_metrics) {
        println!("  Recommendation: {}", recommendation);
    } else {
        println!("  No immediate refinement recommended.");
    }

    println!("\nPredictive Model Evaluation and Refinement Demonstration Complete.");
}
