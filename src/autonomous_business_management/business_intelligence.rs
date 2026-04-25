use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct BusinessIntelligenceEngine {}

impl BusinessIntelligenceEngine {
    pub fn new() -> Self {
        BusinessIntelligenceEngine {}
    }
}
