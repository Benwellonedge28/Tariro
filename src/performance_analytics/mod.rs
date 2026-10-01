use crate::prelude::*;
pub mod metrics;

#[derive(Default)]
pub struct PerformanceAnalyticsEngine {
    pub metrics: metrics::Metrics,
}

impl PerformanceAnalyticsEngine {
    pub fn new() -> Self {
        PerformanceAnalyticsEngine {
            metrics: metrics::Metrics::new(),
        }
    }
}
