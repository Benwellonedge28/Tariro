// src/dashboard/aggregation.rs
use crate::dashboard::models::{UserDashboard, DashboardSection, SectionType};
use chrono::Utc;
use std::collections::HashMap;

pub struct DashboardAggregator;
impl DashboardAggregator {
    pub fn new() -> Self { DashboardAggregator }
    pub fn aggregate_user_data(&self, user_id: &str) -> UserDashboard {
        UserDashboard { user_id: user_id.to_string(), last_updated: Utc::now(), sections: vec![], preferences: HashMap::new() }
    }
}
