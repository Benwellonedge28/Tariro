//! AGI Control and Category entities

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// AGI Control levels (for policies)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AGIControl {
    SuggestOnly,
    SuggestWithApproval,
    AutonomousWithApproval,
    FullyAutonomous,
}

impl Default for AGIControl {
    fn default() -> Self {
        AGIControl::SuggestOnly
    }
}

/// AGI Control Setting (for dynamic configuration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AGIControlSetting {
    pub id: Option<String>,
    pub control_key: String,
    pub value: String,
    pub category: Option<String>,
    pub label: String,
    pub description: Option<String>,
    pub last_modified_by: Option<String>,
}

impl AGIControlSetting {
    pub fn new(control_key: String, value: String, label: String) -> Self {
        Self {
            id: None,
            control_key,
            value,
            category: None,
            label,
            description: None,
            last_modified_by: None,
        }
    }
}

/// AGI Category types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AGICategory {
    Productivity,
    Research,
    Development,
    Communication,
    Analysis,
    Creative,
    Administrative,
    Security,
    Learning,
    Custom(String),
}

impl Default for AGICategory {
    fn default() -> Self {
        AGICategory::Productivity
    }
}

/// Autonomy Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyPolicy {
    pub id: String,
    pub name: String,
    pub control_level: String,
    pub category: AGICategory,
    pub rules: Vec<String>,
    pub restrictions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for AutonomyPolicy {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Policy".to_string(),
            control_level: "suggest_only".to_string(),
            category: AGICategory::default(),
            rules: Vec::new(),
            restrictions: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Capability Status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilityStatus {
    Enabled,
    Disabled,
    Learning,
    Beta,
    Deprecated,
}

impl Default for CapabilityStatus {
    fn default() -> Self {
        CapabilityStatus::Enabled
    }
}

/// Workflow Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRule {
    pub id: String,
    pub name: String,
    pub trigger: String,
    pub actions: Vec<String>,
    pub conditions: Vec<String>,
    pub enabled: bool,
}

impl Default for WorkflowRule {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::new(),
            trigger: String::new(),
            actions: Vec::new(),
            conditions: Vec::new(),
            enabled: true,
        }
    }
}

/// Service Price
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePrice {
    pub service_id: String,
    pub name: String,
    pub price_per_call: f64,
    pub currency: String,
    pub tier: String,
}

impl Default for ServicePrice {
    fn default() -> Self {
        Self {
            service_id: String::new(),
            name: String::new(),
            price_per_call: 0.0,
            currency: "USD".to_string(),
            tier: "standard".to_string(),
        }
    }
}