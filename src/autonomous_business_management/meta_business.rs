use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct MetaBusinessManagement {}

impl MetaBusinessManagement {
    pub fn new() -> Self {
        MetaBusinessManagement {}
    }
}
