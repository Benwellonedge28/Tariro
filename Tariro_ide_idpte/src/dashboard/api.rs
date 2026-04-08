// src/dashboard/api.rs
use crate::dashboard::models::UserDashboard;
use chrono::Utc;
use std::collections::HashMap;

pub struct DashboardService;
impl DashboardService {
    pub fn new() -> Self { DashboardService }
    pub fn get_user_dashboard(&self, user_id: &str) -> UserDashboard {
        UserDashboard { user_id: user_id.to_string(), last_updated: Utc::now(), sections: vec![], preferences: HashMap::new() }
    }
}
