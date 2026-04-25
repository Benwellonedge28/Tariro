use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct RecordKeepingSystem {}

impl RecordKeepingSystem {
    pub fn new() -> Self {
        RecordKeepingSystem {}
    }
}
