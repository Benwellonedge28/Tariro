/// Module for Real-time Anomaly Detection for System Health in Rust.
///
/// This module conceptually outlines a system for monitoring system health metrics
/// and detecting anomalies that could indicate potential failures or performance issues.

use std::collections::VecDeque;
use std::fmt;

/// Represents a single system metric reading.
#[derive(Debug, Clone)]
pub struct SystemMetric {
    pub timestamp: u64,
    pub component: String,
    pub metric_name: String,
    pub value: f64,
}

/// Represents a detected anomaly.
#[derive(Debug, Clone, PartialEq)]
pub struct Anomaly {
    pub timestamp: u64,
    pub component: String,
    pub metric_name: String,
    pub anomalous_value: f64,
    pub baseline_mean: f64,
    pub deviation: f64,
    pub severity: AnomalySeverity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnomalySeverity {
    Informational,
    Warning,
    Critical,
}

impl fmt::Display for AnomalySeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Stores baseline statistics for a metric.
struct MetricBaseline {
    mean: f64,
    std_dev: f64, // Standard deviation
    sample_size: usize,
}

/// The Anomaly Detector.
pub struct AnomalyDetector {
    // A map from (component, metric_name) to its baseline stats.
    baselines: HashMap<(String, String), MetricBaseline>,
    // History of recent values for dynamic baseline updates (conceptual).
    history_buffers: HashMap<(String, String), VecDeque<f64>>,
    history_buffer_capacity: usize,
    threshold_std_dev: f64, // How many standard deviations from mean is an anomaly
}

use std::collections::HashMap;

impl AnomalyDetector {
    pub fn new(history_buffer_capacity: usize, threshold_std_dev: f64) -> Self {
        AnomalyDetector {
            baselines: HashMap::new(),
            history_buffers: HashMap::new(),
            history_buffer_capacity,
            threshold_std_dev,
        }
    }

    /// Conceptually trains a baseline for a given metric.
    /// In a real system, this would involve processing historical data over time.
    pub fn train_baseline(&mut self, initial_metrics: &[SystemMetric]) {
        println!("  [AnomalyDetector]: Training initial baselines with {} metrics.", initial_metrics.len());
        let mut temp_data: HashMap<(String, String), Vec<f64>> = HashMap::new();

        for metric in initial_metrics {
            temp_data.entry((metric.component.clone(), metric.metric_name.clone()))
                .or_default()
                .push(metric.value);
        }

        for ((component, metric_name), values) in temp_data {
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
            let std_dev = variance.sqrt();

            self.baselines.insert(
                (component.clone(), metric_name.clone()),
                MetricBaseline { mean, std_dev, sample_size: values.len() },
            );
            // Initialize history buffer
            self.history_buffers.insert(
                (component, metric_name),
                values.into_iter().collect::<VecDeque<f64>>(),
            );
        }
    }

    /// Detects anomalies for a new incoming system metric.
    pub fn detect_anomaly(&mut self, metric: SystemMetric) -> Option<Anomaly> {
        let key = (metric.component.clone(), metric.metric_name.clone());

        // Update history buffer and re-calculate baseline dynamically (conceptual)
        let history = self.history_buffers.entry(key.clone()).or_insert_with(VecDeque::new);
        history.push_back(metric.value);
        if history.len() > self.history_buffer_capacity {
            history.pop_front();
        }

        let new_mean = history.iter().sum::<f64>() / history.len() as f64;
        let new_variance = history.iter().map(|&x| (x - new_mean).powi(2)).sum::<f64>() / history.len() as f64;
        let new_std_dev = new_variance.sqrt();
        let current_baseline = self.baselines.entry(key.clone()).or_insert(MetricBaseline { mean: new_mean, std_dev: new_std_dev, sample_size: history.len() });

        // For simplicity, we'll use the dynamically updated mean/std_dev for detection
        // In a more advanced system, there would be a separation between training and detection baselines
        current_baseline.mean = new_mean;
        current_baseline.std_dev = new_std_dev;
        current_baseline.sample_size = history.len();

        // Anomaly detection logic: check deviation from mean based on standard deviation
        let deviation = (metric.value - current_baseline.mean).abs();
        if deviation > self.threshold_std_dev * current_baseline.std_dev {
            let severity = if deviation > 3.0 * current_baseline.std_dev {
                AnomalySeverity::Critical
            } else if deviation > 2.0 * current_baseline.std_dev {
                AnomalySeverity::Warning
            } else {
                AnomalySeverity::Informational
            };
            println!("  [AnomalyDetector]: ANOMALY DETECTED for {} {}! Value: {:.2}, Baseline Mean: {:.2}, Deviation: {:.2}",
                     metric.component, metric.metric_name, metric.value, current_baseline.mean, deviation);
            Some(Anomaly {
                timestamp: metric.timestamp,
                component: metric.component,
                metric_name: metric.metric_name,
                anomalous_value: metric.value,
                baseline_mean: current_baseline.mean,
                deviation,
                severity,
                message: format!("{} {} deviated significantly from baseline.", metric.component, metric.metric_name),
            })
        } else {
            println!("  [AnomalyDetector]: {} {} is normal. Value: {:.2}, Baseline Mean: {:.2}",
                     metric.component, metric.metric_name, metric.value, current_baseline.mean);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_baseline() {
        let metrics = vec![
            SystemMetric { timestamp: 1, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 50.0 },
            SystemMetric { timestamp: 2, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 55.0 },
            SystemMetric { timestamp: 3, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 45.0 },
        ];
        let mut detector = AnomalyDetector::new(10, 2.0);
        detector.train_baseline(&metrics);
        let baseline = detector.baselines.get(&("CPU".to_string(), "usage".to_string())).unwrap();
        assert!((baseline.mean - 50.0).abs() < 0.001);
        assert!((baseline.std_dev - 4.082).abs() < 0.001);
    }

    #[test]
    fn test_detect_anomaly_critical() {
        let initial_metrics = vec![
            SystemMetric { timestamp: 1, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 50.0 },
            SystemMetric { timestamp: 2, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 55.0 },
            SystemMetric { timestamp: 3, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 45.0 },
        ];
        let mut detector = AnomalyDetector::new(10, 2.0);
        detector.train_baseline(&initial_metrics);

        let anomalous_metric = SystemMetric { timestamp: 4, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 100.0 };
        let anomaly = detector.detect_anomaly(anomalous_metric);
        assert!(anomaly.is_some());
        assert_eq!(anomaly.unwrap().severity, AnomalySeverity::Critical);
    }

    #[test]
    fn test_detect_anomaly_normal() {
        let initial_metrics = vec![
            SystemMetric { timestamp: 1, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 50.0 },
            SystemMetric { timestamp: 2, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 55.0 },
            SystemMetric { timestamp: 3, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 45.0 },
        ];
        let mut detector = AnomalyDetector::new(10, 2.0);
        detector.train_baseline(&initial_metrics);

        let normal_metric = SystemMetric { timestamp: 4, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 52.0 };
        let anomaly = detector.detect_anomaly(normal_metric);
        assert!(anomaly.is_none());
    }
}

fn main() {
    println!("\n--- Real-time Anomaly Detection for System Health Demonstration ---");

    // 1. Initialize detector with a history buffer size and anomaly threshold (in std deviations)
    let mut detector = AnomalyDetector::new(5, 2.0);

    // 2. Provide initial metrics to train the baseline (historical data)
    let initial_metrics = vec![
        SystemMetric { timestamp: 1, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 30.0 },
        SystemMetric { timestamp: 2, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 32.0 },
        SystemMetric { timestamp: 3, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 29.0 },
        SystemMetric { timestamp: 4, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 31.0 },
        SystemMetric { timestamp: 5, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 33.0 },
        SystemMetric { timestamp: 1, component: "Memory".to_string(), metric_name: "free_gb".to_string(), value: 16.0 },
        SystemMetric { timestamp: 2, component: "Memory".to_string(), metric_name: "free_gb".to_string(), value: 15.5 },
        SystemMetric { timestamp: 3, component: "Memory".to_string(), metric_name: "free_gb".to_string(), value: 16.2 },
    ];
    detector.train_baseline(&initial_metrics);

    println!("\n--- Monitoring Live Metrics ---");
    // 3. Simulate incoming live metrics
    let live_metrics = vec![
        // Normal CPU
        SystemMetric { timestamp: 6, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 30.5 },
        // Normal Memory
        SystemMetric { timestamp: 4, component: "Memory".to_string(), metric_name: "free_gb".to_string(), value: 15.8 },
        // CPU Warning anomaly (2.x std dev)
        SystemMetric { timestamp: 7, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 50.0 },
        // CPU Critical anomaly (3.x std dev)
        SystemMetric { timestamp: 8, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 65.0 },
        // Memory Critical anomaly (very low free_gb)
        SystemMetric { timestamp: 5, component: "Memory".to_string(), metric_name: "free_gb".to_string(), value: 5.0 },
        // Normal CPU again
        SystemMetric { timestamp: 9, component: "CPU".to_string(), metric_name: "usage".to_string(), value: 31.0 },
    ];

    for metric in live_metrics {
        if let Some(anomaly) = detector.detect_anomaly(metric.clone()) {
            println!("!!! ALERT !!! Severity: {}, Component: {}, Metric: {}, Value: {:.2}",
                     anomaly.severity, anomaly.component, anomaly.metric_name, anomaly.anomalous_value);
        }
        std::thread::sleep(std::time::Duration::from_millis(100)); // Simulate real-time delay
    }

    println!("\nReal-time Anomaly Detection Demonstration Complete.");
}
