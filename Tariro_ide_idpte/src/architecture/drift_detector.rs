// src/architecture/drift_detector.rs

//! Compares the current architectural model against a "desired state" or historical baselines
//! to identify deviations (drifts) and their potential implications.

use super::auditor::ArchitecturalModel;
use std::collections::{HashMap, HashSet};

/// Represents a detected architectural drift.
#[derive(Debug, Clone)]
pub struct ArchitecturalDrift {
    pub drift_type: String, // e.g., "service_added", "dependency_changed", "performance_degradation"
    pub description: String,
    pub severity: DriftSeverity,
    pub affected_components: Vec<String>,
    pub suggested_action: Option<String>,
}

/// Defines the severity of an architectural drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftSeverity {
    Informational,
    Warning,
    Critical,
}

/// The `DriftDetector` identifies discrepancies between architectural models.
pub struct DriftDetector {
    // Can hold rules for what constitutes a drift, thresholds, etc.
}

impl DriftDetector {
    pub fn new() -> Self {
        DriftDetector {}
    }

    /// Detects drifts between a `current_model` and a `baseline_model`.
    pub fn detect_drift(&self, current_model: &ArchitecturalModel, baseline_model: &ArchitecturalModel) -> Vec<ArchitecturalDrift> {
        println!("[DriftDetector] Detecting architectural drifts...");
        let mut drifts = Vec::new();

        // --- Simulate Service Drifts ---
        let current_services: HashSet<_> = current_model.services.keys().collect();
        let baseline_services: HashSet<_> = baseline_model.services.keys().collect();

        // Services added
        for s_name in current_services.difference(&baseline_services) {
            drifts.push(ArchitecturalDrift {
                drift_type: "service_added".to_string(),
                description: format!("New service '{}' detected in current architecture.", s_name),
                severity: DriftSeverity::Informational,
                affected_components: vec![(*s_name).clone()],
                suggested_action: Some("Evaluate if this addition was intentional and update baseline.".to_string()),
            });
        }

        // Services removed
        for s_name in baseline_services.difference(&current_services) {
            drifts.push(ArchitecturalDrift {
                drift_type: "service_removed".to_string(),
                description: format!("Service '{}' present in baseline but missing in current architecture.", s_name),
                severity: DriftSeverity::Warning,
                affected_components: vec![(*s_name).clone()],
                suggested_action: Some("Investigate if service was deprecated or accidentally removed.".to_string()),
            });
        }

        // --- Simulate Performance Drifts ---
        for (s_name, current_service) in &current_model.services {
            if let Some(baseline_service) = baseline_model.services.get(s_name) {
                let current_perf = current_service.performance_metrics.get("response_quality_score").and_then(|s| s.parse::<f64>().ok());
                let baseline_perf = baseline_service.performance_metrics.get("response_quality_score").and_then(|s| s.parse::<f64>().ok());

                if let (Some(c_perf), Some(b_perf)) = (current_perf, baseline_perf) {
                    if c_perf < b_perf * 0.8 { // 20% degradation
                        drifts.push(ArchitecturalDrift {
                            drift_type: "performance_degradation".to_string(),
                            description: format!("Performance degradation detected for service '{}'. Current score: {:.2}, Baseline: {:.2}.", s_name, c_perf, b_perf),
                            severity: DriftSeverity::Critical,
                            affected_components: vec![s_name.clone()],
                            suggested_action: Some("Initiate detailed performance analysis and consider rollback or optimization.".to_string()),
                        });
                    }
                }
            }
        }

        // --- Simulate Health Status Drifts ---
        for (s_name, current_service) in &current_model.services {
            if let Some(baseline_service) = baseline_model.services.get(s_name) {
                if current_service.health_status != baseline_service.health_status && current_service.health_status == "critical" {
                    drifts.push(ArchitecturalDrift {
                        drift_type: "health_status_change".to_string(),
                        description: format!("Health status of service '{}' changed from {} to {}.", s_name, baseline_service.health_status, current_service.health_status),
                        severity: DriftSeverity::Critical,
                        affected_components: vec![s_name.clone()],
                        suggested_action: Some("Alert operations and initiate incident response.".to_string()),
                    });
                }
            }
        }

        println!("[DriftDetector] Detected {} drifts.", drifts.len());
        drifts
    }
}
