/// Module for Predictive Threat Analysis and Trend Forecasting in Rust.
///
/// This module provides conceptual components for analyzing historical threat data
/// and forecasting future threat trends, laying the groundwork for proactive defense.

use std::collections::HashMap;
use rand::Rng;

/// Represents historical threat data for training a predictive model.
#[derive(Debug, Clone)]
pub struct HistoricalThreat {
    pub timestamp: u64,
    pub threat_type: String,
    pub severity: f64, // Normalized severity (0.0 to 1.0)
    pub vector: String, // e.g., "phishing", "malware", "ddos"
    pub region: String,
}

/// A conceptual predictive model (e.g., a simple linear regression or pattern matcher).
#[derive(Debug, Clone)]
pub struct PredictiveModel {
    // In a real scenario, this would hold trained weights, decision tree, etc.
    pub model_id: String,
    pub trained_on_data_points: usize,
    pub confidence: f64,
}

/// The Predictive Threat Analyzer.
pub struct PredictiveAnalyzer {
    model: Option<PredictiveModel>,
    historical_data: Vec<HistoricalThreat>,
}

impl PredictiveAnalyzer {
    pub fn new() -> Self {
        PredictiveAnalyzer {
            model: None,
            historical_data: Vec::new(),
        }
    }

    /// Adds historical threat data for training.
    pub fn add_historical_data(&mut self, data: HistoricalThreat) {
        self.historical_data.push(data);
    }

    /// Conceptually trains a predictive model based on historical data.
    /// In a real system, this would involve complex ML algorithms.
    pub fn train_model(&mut self) -> Result<(), String> {
        if self.historical_data.len() < 10 {
            return Err("Not enough historical data to train model (min 10 required).".to_string());
        }

        println!("  [PredictiveAnalyzer]: Training model with {} data points...", self.historical_data.len());

        // Simulate model training and evaluation
        let mut rng = rand::thread_rng();
        let confidence = rng.gen_range(0.75..0.99);

        self.model = Some(PredictiveModel {
            model_id: "threat_predictor_v1".to_string(),
            trained_on_data_points: self.historical_data.len(),
            confidence,
        });

        println!("  [PredictiveAnalyzer]: Model trained successfully with confidence: {:.2}", confidence);
        Ok(())
    }

    /// Predicts future threats or trends based on the trained model.
    /// This is highly conceptual and simulates predictions.
    pub fn predict_future_threats(&self, forecast_horizon_days: u32) -> Result<Vec<String>, String> {
        let model = self.model.as_ref().ok_or("Model not trained yet.".to_string())?;
        println!("  [PredictiveAnalyzer]: Forecasting for next {} days with model confidence: {:.2}",
                 forecast_horizon_days, model.confidence);

        let mut predictions = Vec::new();
        let mut rng = rand::thread_rng();

        // Simulate common and rare threats based on probability
        if rng.gen_bool(0.7) { predictions.push(format!("Increased \"phishing\" attacks in next {} days.", forecast_horizon_days)); }
        if rng.gen_bool(0.4) { predictions.push(format!("Moderate \"malware\" outbreak likely in {} days.", forecast_horizon_days / 2)); }
        if rng.gen_bool(0.1) { predictions.push(format!("Potential \"zero-day exploit\" targeting {} (low probability).", rng.gen_range(1..100))); }

        if predictions.is_empty() { predictions.push("No significant threat trends predicted.".to_string()); }

        Ok(predictions)
    }

    /// Provides insights into historical data to identify key trends.
    pub fn analyze_historical_trends(&self) -> HashMap<String, usize> {
        let mut threat_counts: HashMap<String, usize> = HashMap::new();
        for threat in &self.historical_data {
            *threat_counts.entry(threat.threat_type.clone()).or_default() += 1;
        }
        threat_counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_historical_data() -> Vec<HistoricalThreat> {
        vec![
            HistoricalThreat { timestamp: 1, threat_type: "phishing".to_string(), severity: 0.7, vector: "email".to_string(), region: "EU".to_string() },
            HistoricalThreat { timestamp: 2, threat_type: "malware".to_string(), severity: 0.8, vector: "download".to_string(), region: "NA".to_string() },
            HistoricalThreat { timestamp: 3, threat_type: "phishing".to_string(), severity: 0.6, vector: "email".to_string(), region: "EU".to_string() },
            HistoricalThreat { timestamp: 4, threat_type: "ddos".to_string(), severity: 0.9, vector: "network".to_string(), region: "ASIA".to_string() },
            HistoricalThreat { timestamp: 5, threat_type: "malware".to_string(), severity: 0.75, vector: "usb".to_string(), region: "NA".to_string() },
            HistoricalThreat { timestamp: 6, threat_type: "phishing".to_string(), severity: 0.72, vector: "email".to_string(), region: "EU".to_string() },
            HistoricalThreat { timestamp: 7, threat_type: "ddos".to_string(), severity: 0.88, vector: "network".to_string(), region: "ASIA".to_string() },
            HistoricalThreat { timestamp: 8, threat_type: "phishing".to_string(), severity: 0.65, vector: "email".to_string(), region: "EU".to_string() },
            HistoricalThreat { timestamp: 9, threat_type: "malware".to_string(), severity: 0.81, vector: "exploit".to_string(), region: "AF".to_string() },
            HistoricalThreat { timestamp: 10, threat_type: "phishing".to_string(), severity: 0.71, vector: "email".to_string(), region: "EU".to_string() },
            HistoricalThreat { timestamp: 11, threat_type: "phishing".to_string(), severity: 0.71, vector: "email".to_string(), region: "EU".to_string() },
        ]
    }

    #[test]
    fn test_train_model_success() {
        let mut analyzer = PredictiveAnalyzer::new();
        for data in create_mock_historical_data() { analyzer.add_historical_data(data); }
        let result = analyzer.train_model();
        assert!(result.is_ok());
        assert!(analyzer.model.is_some());
    }

    #[test]
    fn test_train_model_not_enough_data() {
        let mut analyzer = PredictiveAnalyzer::new();
        // Add less than 10 data points
        for data in create_mock_historical_data().into_iter().take(5) { analyzer.add_historical_data(data); }
        let result = analyzer.train_model();
        assert!(result.is_err());
    }

    #[test]
    fn test_predict_future_threats_without_training() {
        let analyzer = PredictiveAnalyzer::new();
        let result = analyzer.predict_future_threats(7);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Model not trained yet.".to_string());
    }

    #[test]
    fn test_analyze_historical_trends() {
        let mut analyzer = PredictiveAnalyzer::new();
        for data in create_mock_historical_data() { analyzer.add_historical_data(data); }
        let trends = analyzer.analyze_historical_trends();
        assert_eq!(trends.get("phishing"), Some(&6));
        assert_eq!(trends.get("malware"), Some(&3));
        assert_eq!(trends.get("ddos"), Some(&2));
    }
}

fn main() {
    println!("\n--- Predictive Threat Analysis and Trend Forecasting Demonstration ---");

    let mut analyzer = PredictiveAnalyzer::new();

    // 1. Add historical threat data
    println!("Adding historical threat data...");
    let historical_data = vec![
        HistoricalThreat { timestamp: 1672531200, threat_type: "phishing".to_string(), severity: 0.7, vector: "email".to_string(), region: "EMEA".to_string() },
        HistoricalThreat { timestamp: 1675209600, threat_type: "malware".to_string(), severity: 0.8, vector: "download".to_string(), region: "APAC".to_string() },
        HistoricalThreat { timestamp: 1677628800, threat_type: "phishing".to_string(), severity: 0.65, vector: "smishing".to_string(), region: "NA".to_string() },
        HistoricalThreat { timestamp: 1680307200, threat_type: "ddos".to_string(), severity: 0.9, vector: "network_flood".to_string(), region: "EMEA".to_string() },
        HistoricalThreat { timestamp: 1682985600, threat_type: "ransomware".to_string(), severity: 0.95, vector: "exploit_kit".to_string(), region: "NA".to_string() },
        HistoricalThreat { timestamp: 1685664000, threat_type: "phishing".to_string(), severity: 0.72, vector: "email".to_string(), region: "EMEA".to_string() },
        HistoricalThreat { timestamp: 1688342400, threat_type: "malware".to_string(), severity: 0.82, vector: "usb".to_string(), region: "APAC".to_string() },
        HistoricalThreat { timestamp: 1691020800, threat_type: "insider_threat".to_string(), severity: 0.5, vector: "data_exfil".to_string(), region: "NA".to_string() },
        HistoricalThreat { timestamp: 1693788800, threat_type: "phishing".to_string(), severity: 0.78, vector: "spear_phishing".to_string(), region: "EMEA".to_string() },
        HistoricalThreat { timestamp: 1696467200, threat_type: "ddos".to_string(), severity: 0.85, vector: "dns_amplification".to_string(), region: "ASIA".to_string() },
        HistoricalThreat { timestamp: 1699145600, threat_type: "phishing".to_string(), severity: 0.70, vector: "vishing".to_string(), region: "NA".to_string() },
        HistoricalThreat { timestamp: 1701824000, threat_type: "ransomware".to_string(), severity: 0.98, vector: "supply_chain".to_string(), region: "GLOBAL".to_string() },
    ];

    for data in historical_data { analyzer.add_historical_data(data); }

    // 2. Analyze historical trends
    let trends = analyzer.analyze_historical_trends();
    println!("\nHistorical Threat Trends: {:?}", trends);

    // 3. Train the predictive model
    match analyzer.train_model() {
        Ok(_) => {
            // 4. Predict future threats
            let forecast_horizon = 30;
            match analyzer.predict_future_threats(forecast_horizon) {
                Ok(predictions) => {
                    println!("\nFuture Threat Predictions for next {} days:", forecast_horizon);
                    for (i, p) in predictions.iter().enumerate() {
                        println!("  {}. {}", i + 1, p);
                    }
                },
                Err(e) => eprintln!("Error predicting threats: {}", e),
            }
        },
        Err(e) => eprintln!("Error training model: {}", e),
    }
}
