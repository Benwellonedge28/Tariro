// Tariro_ide_idpte/src/forecasting/forecasting_models.rs

//! Defines data models for Tariro's Predictive Modeling & Forecasting module,
//! including prediction models, forecasts, and evaluation reports.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

use crate::cognitive_arch::learning_models::VerificationStatus; // For verification statuses

/// Represents a predictive model used for forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionModel {
    pub id: String,
    pub name: String,
    pub model_type: String, // e.g., "recurrent_neural_network", "bayesian_inference", "time_series_regression"
    pub target_variable: String, // What this model predicts
    pub input_features_summary: Vec<String>,
    pub training_data_integrity: VerificationStatus, // New: Integrity of training data (from QR-CTFO)
    pub ethical_biases_status: VerificationStatus, // New: Ethical bias assessment (from ALEC-A)
    pub last_trained: DateTime<Utc>,
    pub performance_metrics: HashMap<String, f64>, // e.g., "mae": 0.15, "r2_score": 0.9
}

/// Represents a generated forecast for a future state or event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forecast {
    pub id: String,
    pub model_id: String,
    pub predicted_value: f64,
    pub prediction_interval: (f64, f64), // Lower and upper bound
    pub confidence: f64, // 0.0 to 1.0
    pub horizon: String, // e.g., "24_hours", "1_week"
    pub ethical_review_status: VerificationStatus, // New: Ethical review of the forecast's implications
    pub security_review_status: VerificationStatus, // New: Security review of the forecast's methodology
    pub generated_at: DateTime<Utc>,
}

/// Evaluation of a forecast's accuracy and robustness.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionEvaluation {
    pub id: String,
    pub forecast_id: String,
    pub actual_outcome: f64,
    pub error_metric: f64, // e.g., absolute error, squared error
    pub robustness_score: f64, // 0.0 to 1.0 (resilience to noise/adversarial input)
    pub integrity_verified: VerificationStatus, // New: Integrity of the evaluation process itself
    pub evaluated_at: DateTime<Utc>,
}

/// A report detailing the ethical implications of a forecast or derived intervention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalForecastReport {
    pub id: String,
    pub forecast_id: String,
    pub summary_of_implications: String,
    pub identified_biases: Vec<String>,
    pub potential_societal_impact: String,
    pub recommended_interventions: Vec<String>,
    pub generated_at: DateTime<Utc>,
}
