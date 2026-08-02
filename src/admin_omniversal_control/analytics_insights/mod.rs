// src/admin_omniversal_control/analytics_insights/mod.rs

//! Analytics and Insights Module for Tariro AGI Admin Omniversal Control
//!
//! This module provides comprehensive analytics, reporting, and insights generation
//! capabilities for monitoring and understanding the AGI's operational patterns,
//! performance metrics, decision-making processes, and self-optimization trajectories.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

/// Analytics data point representing a single metric observation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDataPoint {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub metric_name: String,
    pub metric_category: MetricCategory,
    pub value: f64,
    pub unit: String,
    pub tags: HashMap<String, String>,
    pub source_module: String,
    pub confidence: f32,
}

impl AnalyticsDataPoint {
    pub fn new(
        metric_name: String,
        metric_category: MetricCategory,
        value: f64,
        unit: String,
        source_module: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            metric_name,
            metric_category,
            value,
            unit,
            tags: HashMap::new(),
            source_module,
            confidence: 1.0,
        }
    }

    pub fn with_tags(mut self, tags: HashMap<String, String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

/// Categories for metrics tracked by the analytics system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MetricCategory {
    Performance,
    Learning,
    DecisionMaking,
    ResourceUsage,
    Security,
    Autonomy,
    GoalProgress,
    SystemHealth,
    Communication,
    SelfModification,
    Custom(String),
}

impl Default for MetricCategory {
    fn default() -> Self {
        MetricCategory::Performance
    }
}

impl ToString for MetricCategory {
    fn to_string(&self) -> String {
        match self {
            MetricCategory::Performance => "Performance".to_string(),
            MetricCategory::Learning => "Learning".to_string(),
            MetricCategory::DecisionMaking => "DecisionMaking".to_string(),
            MetricCategory::ResourceUsage => "ResourceUsage".to_string(),
            MetricCategory::Security => "Security".to_string(),
            MetricCategory::Autonomy => "Autonomy".to_string(),
            MetricCategory::GoalProgress => "GoalProgress".to_string(),
            MetricCategory::SystemHealth => "SystemHealth".to_string(),
            MetricCategory::Communication => "Communication".to_string(),
            MetricCategory::SelfModification => "SelfModification".to_string(),
            MetricCategory::Custom(s) => s.clone(),
        }
    }
}

/// Aggregated analytics report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub id: String,
    pub generated_at: DateTime<Utc>,
    pub report_type: ReportType,
    pub time_range: TimeRange,
    pub summary_metrics: HashMap<String, f64>,
    pub detailed_breakdown: HashMap<MetricCategory, CategoryMetrics>,
    pub trends: Vec<TrendAnalysis>,
    pub recommendations: Vec<Recommendation>,
    pub anomalies_detected: Vec<AnomalyReport>,
}

impl AnalyticsReport {
    pub fn new(report_type: ReportType, time_range: TimeRange) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            generated_at: Utc::now(),
            report_type,
            time_range,
            summary_metrics: HashMap::new(),
            detailed_breakdown: HashMap::new(),
            trends: Vec::new(),
            recommendations: Vec::new(),
            anomalies_detected: Vec::new(),
        }
    }

    pub fn add_summary_metric(&mut self, key: String, value: f64) {
        self.summary_metrics.insert(key, value);
    }

    pub fn add_category_metrics(&mut self, category: MetricCategory, metrics: CategoryMetrics) {
        self.detailed_breakdown.insert(category, metrics);
    }

    pub fn add_trend(&mut self, trend: TrendAnalysis) {
        self.trends.push(trend);
    }

    pub fn add_recommendation(&mut self, recommendation: Recommendation) {
        self.recommendations.push(recommendation);
    }

    pub fn add_anomaly(&mut self, anomaly: AnomalyReport) {
        self.anomalies_detected.push(anomaly);
    }
}

/// Types of analytics reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    PerformanceSummary,
    LearningProgress,
    DecisionAnalysis,
    SecurityAudit,
    AutonomyReport,
    GoalAchievement,
    ResourceOptimization,
    ComprehensiveReview,
    AnomalyAlert,
    TrendForecast,
}

/// Time range for analytics queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }

    pub fn last_hours(hours: i64) -> Self {
        let end = Utc::now();
        let start = end - Duration::hours(hours);
        Self { start, end }
    }

    pub fn last_days(days: i64) -> Self {
        let end = Utc::now();
        let start = end - Duration::days(days);
        Self { start, end }
    }

    pub fn last_weeks(weeks: i64) -> Self {
        let end = Utc::now();
        let start = end - Duration::weeks(weeks);
        Self { start, end }
    }

    pub fn duration_hours(&self) -> f64 {
        (self.end - self.start).num_minutes() as f64 / 60.0
    }
}

/// Metrics for a specific category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryMetrics {
    pub total_observations: u64,
    pub average_value: f64,
    pub min_value: f64,
    pub max_value: f64,
    pub standard_deviation: f64,
    pub percentile_25: f64,
    pub percentile_50: f64,
    pub percentile_75: f64,
    pub percentile_95: f64,
    pub trend_direction: TrendDirection,
    pub health_score: f32,
}

impl Default for CategoryMetrics {
    fn default() -> Self {
        Self {
            total_observations: 0,
            average_value: 0.0,
            min_value: 0.0,
            max_value: 0.0,
            standard_deviation: 0.0,
            percentile_25: 0.0,
            percentile_50: 0.0,
            percentile_75: 0.0,
            percentile_95: 0.0,
            trend_direction: TrendDirection::Stable,
            health_score: 1.0,
        }
    }
}

/// Direction of a trend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
    Unknown,
}

/// Trend analysis for a metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub metric_name: String,
    pub category: MetricCategory,
    pub direction: TrendDirection,
    pub slope: f64,
    pub r_squared: f64,
    pub prediction_confidence: f32,
    pub forecast_value: Option<f64>,
    pub forecast_timeframe: Option<String>,
    pub significant_changes: Vec<SignificantChange>,
}

impl TrendAnalysis {
    pub fn new(metric_name: String, category: MetricCategory) -> Self {
        Self {
            metric_name,
            category,
            direction: TrendDirection::Unknown,
            slope: 0.0,
            r_squared: 0.0,
            prediction_confidence: 0.0,
            forecast_value: None,
            forecast_timeframe: None,
            significant_changes: Vec::new(),
        }
    }

    pub fn with_regression(mut self, slope: f64, r_squared: f64) -> Self {
        self.slope = slope;
        self.r_squared = r_squared;
        self.direction = if slope.abs() < 0.01 {
            TrendDirection::Stable
        } else if slope > 0.0 {
            TrendDirection::Increasing
        } else {
            TrendDirection::Decreasing
        };
        self
    }
}

/// Significant change in a metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificantChange {
    pub timestamp: DateTime<Utc>,
    pub change_type: ChangeType,
    pub magnitude: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Spike,
    Drop,
    GradualIncrease,
    GradualDecrease,
    StepChange,
    Oscillation,
}

/// Recommendation generated from analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub priority: RecommendationPriority,
    pub category: MetricCategory,
    pub title: String,
    pub description: String,
    pub rationale: String,
    pub expected_impact: String,
    pub implementation_difficulty: ImplementationDifficulty,
    pub estimated_benefit: f32,
    pub related_metrics: Vec<String>,
    pub action_steps: Vec<String>,
}

impl Recommendation {
    pub fn new(priority: RecommendationPriority, category: MetricCategory, title: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            priority,
            category,
            title,
            description: String::new(),
            rationale: String::new(),
            expected_impact: String::new(),
            implementation_difficulty: ImplementationDifficulty::Medium,
            estimated_benefit: 0.0,
            related_metrics: Vec::new(),
            action_steps: Vec::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_rationale(mut self, rationale: String) -> Self {
        self.rationale = rationale;
        self
    }

    pub fn with_action_steps(mut self, steps: Vec<String>) -> Self {
        self.action_steps = steps;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RecommendationPriority {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationDifficulty {
    Trivial,
    Easy,
    Medium,
    Difficult,
    Complex,
}

/// Anomaly report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyReport {
    pub id: String,
    pub detected_at: DateTime<Utc>,
    pub metric_name: String,
    pub category: MetricCategory,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub description: String,
    pub expected_value: f64,
    pub actual_value: f64,
    pub deviation_magnitude: f64,
    pub possible_causes: Vec<String>,
    pub recommended_actions: Vec<String>,
}

impl AnomalyReport {
    pub fn new(
        metric_name: String,
        category: MetricCategory,
        anomaly_type: AnomalyType,
        severity: AnomalySeverity,
        expected_value: f64,
        actual_value: f64,
    ) -> Self {
        let deviation_magnitude = if expected_value != 0.0 {
            ((actual_value - expected_value) / expected_value).abs()
        } else {
            actual_value.abs()
        };

        Self {
            id: Uuid::new_v4().to_string(),
            detected_at: Utc::now(),
            metric_name,
            category,
            anomaly_type,
            severity,
            description: String::new(),
            expected_value,
            actual_value,
            deviation_magnitude,
            possible_causes: Vec::new(),
            recommended_actions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    Spike,
    Drop,
    SuddenChange,
    GradualDrift,
    PatternDeviation,
    ThresholdViolation,
    CyclicPatternBreak,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum AnomalySeverity {
    Critical,
    High,
    Medium,
    Low,
    Warning,
}

/// Main analytics service
pub struct AnalyticsService {
    data_points: Arc<RwLock<Vec<AnalyticsDataPoint>>>,
    max_data_points: usize,
}

impl AnalyticsService {
    pub fn new(max_data_points: usize) -> Self {
        Self {
            data_points: Arc::new(RwLock::new(Vec::with_capacity(max_data_points))),
            max_data_points,
        }
    }

    pub async fn record_metric(&self, data_point: AnalyticsDataPoint) {
        let mut points = self.data_points.write().await;

        // Evict oldest if at capacity
        if points.len() >= self.max_data_points {
            points.remove(0);
        }

        points.push(data_point);
    }

    pub async fn get_metrics_in_range(
        &self,
        metric_name: &str,
        time_range: &TimeRange,
    ) -> Vec<AnalyticsDataPoint> {
        let points = self.data_points.read().await;
        points
            .iter()
            .filter(|dp| {
                dp.metric_name == metric_name
                    && dp.timestamp >= time_range.start
                    && dp.timestamp <= time_range.end
            })
            .cloned()
            .collect()
    }

    pub async fn get_all_metrics_in_range(
        &self,
        time_range: &TimeRange,
    ) -> Vec<AnalyticsDataPoint> {
        let points = self.data_points.read().await;
        points
            .iter()
            .filter(|dp| {
                dp.timestamp >= time_range.start && dp.timestamp <= time_range.end
            })
            .cloned()
            .collect()
    }

    pub async fn generate_report(&self, report_type: ReportType, time_range: TimeRange) -> AnalyticsReport {
        let metrics = self.get_all_metrics_in_range(&time_range).await;
        let mut report = AnalyticsReport::new(report_type, time_range.clone());

        // Calculate summary statistics
        if !metrics.is_empty() {
            let total_values: f64 = metrics.iter().map(|dp| dp.value).sum();
            let count = metrics.len() as f64;
            report.add_summary_metric("total_observations".to_string(), count);
            report.add_summary_metric("average_value".to_string(), total_values / count);
            report.add_summary_metric("unique_metrics".to_string(),
                metrics.iter().map(|dp| dp.metric_name.clone()).collect::<std::collections::HashSet<_>>().len() as f64);
        }

        // Group by category and calculate metrics
        let mut category_groups: HashMap<MetricCategory, Vec<&AnalyticsDataPoint>> = HashMap::new();
        for dp in &metrics {
            category_groups.entry(dp.metric_category.clone()).or_default().push(dp);
        }

        for (category, dps) in category_groups {
            let values: Vec<f64> = dps.iter().map(|dp| dp.value).collect();
            if !values.is_empty() {
                let mut cat_metrics = CategoryMetrics::default();
                cat_metrics.total_observations = values.len() as u64;
                cat_metrics.average_value = values.iter().sum::<f64>() / values.len() as f64;
                cat_metrics.min_value = *values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
                cat_metrics.max_value = *values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);

                // Calculate percentiles
                let mut sorted = values.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let n = sorted.len();
                cat_metrics.percentile_25 = sorted[(n as f64 * 0.25) as usize].min(sorted[n-1]);
                cat_metrics.percentile_50 = sorted[(n as f64 * 0.50) as usize].min(sorted[n-1]);
                cat_metrics.percentile_75 = sorted[(n as f64 * 0.75) as usize].min(sorted[n-1]);
                cat_metrics.percentile_95 = sorted[(n as f64 * 0.95) as usize].min(sorted[n-1]);

                // Calculate standard deviation
                let mean = cat_metrics.average_value;
                let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
                cat_metrics.standard_deviation = variance.sqrt();

                // Calculate health score based on variance
                let cv = if mean != 0.0 { cat_metrics.standard_deviation / mean.abs() } else { 0.0 };
                cat_metrics.health_score = (1.0 - cv.min(1.0)) as f32;

                report.add_category_metrics(category, cat_metrics);
            }
        }

        // Detect anomalies
        let anomalies = self.detect_anomalies(&metrics).await;
        for anomaly in anomalies {
            report.add_anomaly(anomaly);
        }

        // Generate recommendations
        let recommendations = self.generate_recommendations(&report).await;
        for rec in recommendations {
            report.add_recommendation(rec);
        }

        report
    }

    async fn detect_anomalies(&self, metrics: &[AnalyticsDataPoint]) -> Vec<AnomalyReport> {
        let mut anomalies = Vec::new();

        // Group by metric name
        let mut metric_groups: HashMap<String, Vec<&AnalyticsDataPoint>> = HashMap::new();
        for dp in metrics {
            metric_groups.entry(dp.metric_name.clone()).or_default().push(dp);
        }

        for (metric_name, dps) in metric_groups {
            if dps.len() < 3 {
                continue;
            }

            let values: Vec<f64> = dps.iter().map(|dp| dp.value).collect();
            let mean = values.iter().sum::<f64>() / values.len() as f64;
            let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
            let std_dev = variance.sqrt();

            // Check for anomalies (values more than 2 standard deviations from mean)
            for dp in &values {
                let deviation = (dp - mean).abs();
                if deviation > 2.0 * std_dev && std_dev > 0.0 {
                    anomalies.push(AnomalyReport::new(
                        metric_name.clone(),
                        dps[0].metric_category.clone(),
                        if dp > &mean { AnomalyType::Spike } else { AnomalyType::Drop },
                        if deviation > 3.0 * std_dev { AnomalySeverity::High } else { AnomalySeverity::Medium },
                        mean,
                        *dp,
                    ));
                }
            }
        }

        anomalies
    }

    async fn generate_recommendations(&self, report: &AnalyticsReport) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        // Check health scores and generate recommendations
        for (category, metrics) in &report.detailed_breakdown {
            if metrics.health_score < 0.5 {
                let mut rec = Recommendation::new(
                    if metrics.health_score < 0.3 { RecommendationPriority::High } else { RecommendationPriority::Medium },
                    category.clone(),
                    format!("Improve {} stability", category.to_string()),
                );
                rec.description = format!(
                    "The {} category shows high variability with a health score of {:.2}. Consider investigating root causes.",
                    category.to_string(),
                    metrics.health_score
                );
                rec.rationale = "Low health scores indicate unstable performance that may impact AGI reliability.".to_string();
                rec.action_steps = vec![
                    "Analyze recent changes in this category".to_string(),
                    "Review external factors affecting stability".to_string(),
                    "Consider implementing additional monitoring".to_string(),
                ];
                recommendations.push(rec);
            }
        }

        // Check for anomalies
        if !report.anomalies_detected.is_empty() {
            let critical_anomalies: Vec<_> = report.anomalies_detected
                .iter()
                .filter(|a| a.severity == AnomalySeverity::Critical || a.severity == AnomalySeverity::High)
                .collect();

            if !critical_anomalies.is_empty() {
                let mut rec = Recommendation::new(
                    RecommendationPriority::Critical,
                    MetricCategory::SystemHealth,
                    "Investigate critical anomalies detected".to_string(),
                );
                rec.description = format!(
                    "{} critical/high anomalies were detected requiring immediate attention.",
                    critical_anomalies.len()
                );
                rec.rationale = "Unaddressed anomalies can lead to system degradation or failure.".to_string();
                rec.action_steps = vec![
                    "Review anomaly details in the report".to_string(),
                    "Identify common causes across anomalies".to_string(),
                    "Implement corrective actions".to_string(),
                    "Monitor for recurrence".to_string(),
                ];
                recommendations.push(rec);
            }
        }

        recommendations
    }

    pub async fn get_insights(&self) -> Vec<AnalyticsInsight> {
        let points = self.data_points.read().await;
        let mut insights = Vec::new();

        // Generate insights based on recent data
        if points.len() >= 10 {
            // Calculate aggregate statistics
            let avg_value = points.iter().map(|dp| dp.value).sum::<f64>() / points.len() as f64;
            let max_metric = points.iter().max_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

            if let Some(max_dp) = max_metric {
                insights.push(AnalyticsInsight {
                    insight_type: InsightType::PeakDetected,
                    title: "Peak metric value detected".to_string(),
                    description: format!(
                        "Peak value {:.2} detected for metric '{}' at {}",
                        max_dp.value,
                        max_dp.metric_name,
                        max_dp.timestamp
                    ),
                    confidence: 0.85,
                    related_metrics: vec![max_dp.metric_name.clone()],
                });
            }

            // Analyze trend
            let recent_points: Vec<_> = points.iter().rev().take(5).collect();
            if recent_points.len() >= 3 {
                let first_half_avg: f64 = recent_points[..recent_points.len()/2]
                    .iter().map(|dp| dp.value).sum::<f64>() / (recent_points.len()/2) as f64;
                let second_half_avg: f64 = recent_points[recent_points.len()/2..]
                    .iter().map(|dp| dp.value).sum::<f64>() / (recent_points.len() - recent_points.len()/2) as f64;

                if (second_half_avg - first_half_avg).abs() > avg_value * 0.2 {
                    insights.push(AnalyticsInsight {
                        insight_type: InsightType::TrendShift,
                        title: "Significant trend shift detected".to_string(),
                        description: format!(
                            "Metric values shifted by {:.2}% from recent baseline",
                            ((second_half_avg - first_half_avg) / avg_value * 100.0).abs()
                        ),
                        confidence: 0.75,
                        related_metrics: recent_points.iter().map(|dp| dp.metric_name.clone()).collect(),
                    });
                }
            }
        }

        insights
    }

    pub async fn clear_old_data(&self, older_than: Duration) {
        let cutoff = Utc::now() - older_than;
        let mut points = self.data_points.write().await;
        points.retain(|dp| dp.timestamp > cutoff);
    }

    pub async fn get_statistics(&self) -> AnalyticsStatistics {
        let points = self.data_points.read().await;
        AnalyticsStatistics {
            total_data_points: points.len(),
            max_capacity: self.max_data_points,
            storage_utilization: points.len() as f64 / self.max_data_points as f64,
            unique_metrics: points.iter().map(|dp| dp.metric_name.clone()).collect::<std::collections::HashSet<_>>().len(),
            oldest_data_point: points.iter().min_by(|a, b| a.timestamp.cmp(&b.timestamp)).map(|dp| dp.timestamp),
            newest_data_point: points.iter().max_by(|a, b| a.timestamp.cmp(&b.timestamp)).map(|dp| dp.timestamp),
        }
    }
}

/// Analytics insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsInsight {
    pub insight_type: InsightType,
    pub title: String,
    pub description: String,
    pub confidence: f32,
    pub related_metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    PatternDetected,
    TrendShift,
    PeakDetected,
    AnomalyResolved,
    ImprovementObserved,
    OptimizationOpportunity,
}

/// Analytics statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsStatistics {
    pub total_data_points: usize,
    pub max_capacity: usize,
    pub storage_utilization: f64,
    pub unique_metrics: usize,
    pub oldest_data_point: Option<DateTime<Utc>>,
    pub newest_data_point: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_and_retrieve_metrics() {
        let service = AnalyticsService::new(100);

        let dp = AnalyticsDataPoint::new(
            "test_metric".to_string(),
            MetricCategory::Performance,
            42.0,
            "ops/s".to_string(),
            "test_module".to_string(),
        );

        service.record_metric(dp).await;

        let time_range = TimeRange::last_hours(1);
        let retrieved = service.get_metrics_in_range("test_metric", &time_range).await;

        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].value, 42.0);
    }

    #[tokio::test]
    async fn test_report_generation() {
        let service = AnalyticsService::new(100);

        for i in 0..10 {
            let dp = AnalyticsDataPoint::new(
                "performance_metric".to_string(),
                MetricCategory::Performance,
                100.0 + i as f64,
                "ms".to_string(),
                "test".to_string(),
            );
            service.record_metric(dp).await;
        }

        let report = service.generate_report(
            ReportType::PerformanceSummary,
            TimeRange::last_hours(1),
        ).await;

        assert!(report.summary_metrics.contains_key("total_observations"));
        assert!(report.detailed_breakdown.contains_key(&MetricCategory::Performance));
    }

    #[tokio::test]
    async fn test_anomaly_detection() {
        let service = AnalyticsService::new(100);

        // Add normal values
        for i in 0..5 {
            let dp = AnalyticsDataPoint::new(
                "test_metric".to_string(),
                MetricCategory::Performance,
                100.0,
                "units".to_string(),
                "test".to_string(),
            );
            service.record_metric(dp).await;
        }

        // Add anomaly
        let dp = AnalyticsDataPoint::new(
            "test_metric".to_string(),
            MetricCategory::Performance,
            1000.0, // Way outside normal range
            "units".to_string(),
            "test".to_string(),
        );
        service.record_metric(dp).await;

        let report = service.generate_report(
            ReportType::AnomalyAlert,
            TimeRange::last_hours(1),
        ).await;

        // Anomaly should be detected
        assert!(!report.anomalies_detected.is_empty());
    }
}
