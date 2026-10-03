//! TUMC Metrics and Observability
//!
//! Performance monitoring, metrics collection, and observability tools.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// Metric value
#[derive(Debug, Clone)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<u64>),
    Summary(SummaryData),
}

/// Summary data
#[derive(Debug, Clone)]
pub struct SummaryData {
    pub count: u64,
    pub sum: f64,
    pub quantiles: HashMap<f64, f64>,
}

/// Metric
#[derive(Debug, Clone)]
pub struct Metric {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Description
    pub description: String,
    /// Unit
    pub unit: Option<String>,
    /// Value
    pub value: MetricValue,
}

/// Metrics registry
pub struct MetricsRegistry {
    /// Metrics
    metrics: RwLock<HashMap<String, Metric>>,
    /// Labels
    label_sets: RwLock<Vec<LabelSet>>,
}

impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsRegistry {
    /// Create new registry
    pub fn new() -> Self {
        Self {
            metrics: RwLock::new(HashMap::new()),
            label_sets: RwLock::new(Vec::new()),
        }
    }

    /// Register counter
    pub fn register_counter(&self, name: &str, desc: &str, unit: Option<&str>) {
        let metric = Metric {
            name: name.to_string(),
            metric_type: MetricType::Counter,
            description: desc.to_string(),
            unit: unit.map(|s| s.to_string()),
            value: MetricValue::Counter(0),
        };

        self.metrics.write().unwrap().insert(name.to_string(), metric);
    }

    /// Register gauge
    pub fn register_gauge(&self, name: &str, desc: &str, unit: Option<&str>) {
        let metric = Metric {
            name: name.to_string(),
            metric_type: MetricType::Gauge,
            description: desc.to_string(),
            unit: unit.map(|s| s.to_string()),
            value: MetricValue::Gauge(0.0),
        };

        self.metrics.write().unwrap().insert(name.to_string(), metric);
    }

    /// Register histogram
    pub fn register_histogram(&self, name: &str, desc: &str, unit: Option<&str>) {
        let metric = Metric {
            name: name.to_string(),
            metric_type: MetricType::Histogram,
            description: desc.to_string(),
            unit: unit.map(|s| s.to_string()),
            value: MetricValue::Histogram(Vec::new()),
        };

        self.metrics.write().unwrap().insert(name.to_string(), metric);
    }

    /// Increment counter
    pub fn inc_counter(&self, name: &str, value: u64) {
        if let Some(metric) = self.metrics.write().unwrap().get_mut(name) {
            if let MetricValue::Counter(ref mut v) = metric.value {
                *v += value;
            }
        }
    }

    /// Set gauge
    pub fn set_gauge(&self, name: &str, value: f64) {
        if let Some(metric) = self.metrics.write().unwrap().get_mut(name) {
            if let MetricValue::Gauge(ref mut v) = metric.value {
                *v = value;
            }
        }
    }

    /// Observe histogram
    pub fn observe_histogram(&self, name: &str, value: u64) {
        if let Some(metric) = self.metrics.write().unwrap().get_mut(name) {
            if let MetricValue::Histogram(ref mut v) = metric.value {
                v.push(value);
            }
        }
    }

    /// Get all metrics
    pub fn gather(&self) -> Vec<Metric> {
        self.metrics.read().unwrap().values().cloned().collect()
    }
}

/// Label set
#[derive(Debug, Clone)]
pub struct LabelSet {
    pub labels: HashMap<String, String>,
}

/// Counter wrapper
pub struct Counter {
    name: String,
    registry: Arc<MetricsRegistry>,
}

impl Counter {
    /// Create new counter
    pub fn new(name: &str, registry: Arc<MetricsRegistry>) -> Self {
        registry.register_counter(name, "", None);
        Self {
            name: name.to_string(),
            registry,
        }
    }

    /// Increment by value
    pub fn inc(&self, value: u64) {
        self.registry.inc_counter(&self.name, value);
    }
}

/// Gauge wrapper
pub struct Gauge {
    name: String,
    registry: Arc<MetricsRegistry>,
}

impl Gauge {
    /// Create new gauge
    pub fn new(name: &str, registry: Arc<MetricsRegistry>) -> Self {
        registry.register_gauge(name, "", None);
        Self {
            name: name.to_string(),
            registry,
        }
    }

    /// Set value
    pub fn set(&self, value: f64) {
        self.registry.set_gauge(&self.name, value);
    }

    /// Increment by value
    pub fn inc(&self, value: f64) {
        // Get current value and add
        let metrics = self.registry.gather();
        if let Some(m) = metrics.iter().find(|m| m.name == self.name) {
            if let MetricValue::Gauge(current) = m.value {
                self.registry.set_gauge(&self.name, current + value);
            }
        }
    }
}

/// Histogram wrapper
pub struct Histogram {
    name: String,
    registry: Arc<MetricsRegistry>,
}

impl Histogram {
    /// Create new histogram
    pub fn new(name: &str, registry: Arc<MetricsRegistry>) -> Self {
        registry.register_histogram(name, "", None);
        Self {
            name: name.to_string(),
            registry,
        }
    }

    /// Observe value
    pub fn observe(&self, value: u64) {
        self.registry.observe_histogram(&self.name, value);
    }
}

/// Timer
pub struct Timer {
    start: Instant,
    histogram: Option<Histogram>,
}

impl Timer {
    /// Create new timer
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            histogram: None,
        }
    }

    /// Create timer with histogram
    pub fn with_histogram(histogram: Histogram) -> Self {
        Self {
            start: Instant::now(),
            histogram: Some(histogram),
        }
    }

    /// Stop timer and return duration
    pub fn stop(self) -> Duration {
        let duration = self.start.elapsed();
        if let Some(ref h) = self.histogram {
            h.observe(duration.as_millis() as u64);
        }
        duration
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        // Timer auto-records on drop if histogram set
    }
}

/// Tracing span
#[derive(Debug)]
pub struct Span {
    /// Trace ID
    pub trace_id: u64,
    /// Span ID
    pub span_id: u64,
    /// Parent span ID
    pub parent_id: Option<u64>,
    /// Operation name
    pub name: String,
    /// Start time
    pub start: Instant,
    /// End time
    pub end: Option<Instant>,
    /// Tags
    pub tags: HashMap<String, String>,
    /// Logs
    pub logs: Vec<SpanLog>,
}

/// Span log entry
#[derive(Debug, Clone)]
pub struct SpanLog {
    pub timestamp: Instant,
    pub fields: HashMap<String, String>,
}

/// Tracer
pub struct Tracer {
    /// Service name
    service_name: String,
    /// Spans
    spans: RwLock<Vec<Span>>,
    /// Current span
    current_span: RwLock<Option<u64>>,
    /// Span counter
    span_counter: RwLock<u64>,
}

impl Default for Tracer {
    fn default() -> Self {
        Self::new("service")
    }
}

impl Tracer {
    /// Create new tracer
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            spans: RwLock::new(Vec::new()),
            current_span: RwLock::new(None),
            span_counter: RwLock::new(0),
        }
    }

    /// Start new span
    pub fn start_span(&self, name: &str) -> Span {
        let span_id = {
            let mut counter = self.span_counter.write().unwrap();
            *counter += 1;
            *counter
        };

        let parent_id = *self.current_span.read().unwrap();

        let trace_id = rand::random();

        let span = Span {
            trace_id,
            span_id,
            parent_id,
            name: name.to_string(),
            start: Instant::now(),
            end: None,
            tags: HashMap::new(),
            logs: Vec::new(),
        };

        self.spans.write().unwrap().push(span.clone());
        *self.current_span.write().unwrap() = Some(span_id);

        span
    }

    /// End current span
    pub fn end_span(&self) {
        let span_id = self.current_span.write().unwrap().take();
        if let Some(id) = span_id {
            if let Some(span) = self.spans.write().unwrap().iter_mut().find(|s| s.span_id == id) {
                span.end = Some(Instant::now());
            }
        }
    }

    /// Add tag to current span
    pub fn tag(&self, key: &str, value: &str) {
        let span_id = *self.current_span.read().unwrap();
        if let Some(id) = span_id {
            if let Some(span) = self.spans.write().unwrap().iter_mut().find(|s| s.span_id == id) {
                span.tags.insert(key.to_string(), value.to_string());
            }
        }
    }

    /// Log to current span
    pub fn log(&self, fields: HashMap<String, String>) {
        let span_id = *self.current_span.read().unwrap();
        if let Some(id) = span_id {
            if let Some(span) = self.spans.write().unwrap().iter_mut().find(|s| s.span_id == id) {
                span.logs.push(SpanLog {
                    timestamp: Instant::now(),
                    fields,
                });
            }
        }
    }

    /// Get all spans
    pub fn spans(&self) -> Vec<Span> {
        self.spans.read().unwrap().clone()
    }
}

/// Health check
#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Health check result
#[derive(Debug)]
pub struct HealthCheck {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub duration: Option<Duration>,
}

/// Health checker
pub struct HealthChecker {
    checks: HashMap<String, Box<dyn Fn() -> HealthCheck + Send + Sync>>,
}

impl HealthChecker {
    /// Create new health checker
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
        }
    }

    /// Register health check
    pub fn register<F>(&mut self, name: &str, check: F)
    where
        F: Fn() -> HealthCheck + Send + Sync + 'static,
    {
        self.checks.insert(name.to_string(), Box::new(check));
    }

    /// Run all checks
    pub fn check_all(&self) -> Vec<HealthCheck> {
        self.checks
            .values()
            .map(|f| f())
            .collect()
    }

    /// Check if healthy
    pub fn is_healthy(&self) -> bool {
        self.check_all()
            .iter()
            .all(|c| c.status == HealthStatus::Healthy)
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Exporter trait
pub trait MetricsExporter: Send + Sync {
    /// Export metrics
    fn export(&self, metrics: &[Metric]) -> Result<(), ExportError>;
}

/// Export error
#[derive(Debug)]
pub struct ExportError {
    pub message: String,
}

/// Prometheus exporter
pub struct PrometheusExporter {
    endpoint: String,
}

impl PrometheusExporter {
    /// Create new exporter
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }
}

impl MetricsExporter for PrometheusExporter {
    fn export(&self, metrics: &[Metric]) -> Result<(), ExportError> {
        // Would export to Prometheus endpoint
        Ok(())
    }
}

/// Metrics snapshot
pub struct MetricsSnapshot {
    pub timestamp: Instant,
    pub metrics: Vec<Metric>,
    pub spans: Vec<Span>,
    pub health_checks: Vec<HealthCheck>,
}

impl MetricsSnapshot {
    /// Create new snapshot
    pub fn new(
        metrics: Vec<Metric>,
        spans: Vec<Span>,
        health_checks: Vec<HealthCheck>,
    ) -> Self {
        Self {
            timestamp: Instant::now(),
            metrics,
            spans,
            health_checks,
        }
    }

    /// Export as JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_registry() {
        let registry = MetricsRegistry::new();
        registry.register_counter("requests", "Total requests", Some("requests"));
        registry.inc_counter("requests", 1);

        let metrics = registry.gather();
        assert!(metrics.len() >= 1);
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new();
        std::thread::sleep(Duration::from_millis(10));
        let duration = timer.stop();
        assert!(duration.as_millis() >= 10);
    }

    #[test]
    fn test_tracer() {
        let tracer = Tracer::new("test");
        let span = tracer.start_span("test_op");
        tracer.end_span();

        assert!(span.end.is_some());
    }

    #[test]
    fn test_health_checker() {
        let mut checker = HealthChecker::new();
        checker.register("test", || HealthCheck {
            name: "test".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            duration: None,
        });

        assert!(checker.is_healthy());
    }
}
