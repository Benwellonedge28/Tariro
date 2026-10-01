// src/admin_voice_interface/document_config_policy/voice_performance_metrics_thresholds.rs

use crate::prelude::*;
pub struct PerformanceMetricsThresholds;

impl PerformanceMetricsThresholds {
    pub fn define_thresholds(&self, metric: &str) -> String {
        format!("Defined thresholds for voice performance metric '{}' in a document, triggering alerts if exceeded.", metric)
    }
}
