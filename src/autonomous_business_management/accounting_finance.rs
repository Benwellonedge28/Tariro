use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Default)]
pub struct AccountingFinanceEngine {}

impl AccountingFinanceEngine {
    pub fn new() -> Self {
        AccountingFinanceEngine {}
    }
}
