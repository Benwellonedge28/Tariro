use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Defines the available levels of explainability detail
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExplainabilityLevel {
    HighLevelSummary,
    KeyInsights,
    ReasoningSteps,
    DataSources,
    TechnicalBreakdown, // Deepest level, e.g., code snippets, algorithm traces
}

// Represents a user's preference for explainability granularity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExplainabilityPreference {
    pub user_id: String,
    pub default_level: ExplainabilityLevel,
    pub context_specific_levels: HashMap<String, ExplainabilityLevel>, // e.g., "security_alert": TechnicalBreakdown
    pub last_updated: DateTime<Utc>,
}

// Data structure holding raw, un-filtered explainability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawExplainabilityData {
    pub decision_id: String,
    pub high_level_summary: String,
    pub key_insights: Vec<String>,
    pub all_reasoning_steps: Vec<String>, // Detailed steps
    pub all_data_sources: HashMap<String, String>, // data_id -> content snippet
    pub technical_details: HashMap<String, String>, // e.g., "algorithm_trace": "...", "code_snippet": "..."
    pub confidence_score: f64,
}

pub struct ExplainabilityGranularityManager {
    user_preferences: Mutex<HashMap<String, UserExplainabilityPreference>>, // key: user_id
}

impl ExplainabilityGranularityManager {
    pub fn new() -> Self {
        ExplainabilityGranularityManager {
            user_preferences: Mutex::new(HashMap::new()),
        }
    }

    // Sets or updates the explainability preference for a user
    pub fn set_user_preference(&self, preference: UserExplainabilityPreference) {
        println!("Setting explainability preference for user: {}", preference.user_id);
        self.user_preferences.lock().unwrap().insert(preference.user_id.clone(), preference);
    }

    // Retrieves the current preference for a user, or a default if none set
    pub fn get_user_preference(&self, user_id: &str) -> UserExplainabilityPreference {
        self.user_preferences.lock().unwrap()
            .get(user_id)
            .cloned()
            .unwrap_or_else(|| {
                println!("No specific preference found for user {}. Using default.", user_id);
                UserExplainabilityPreference {
                    user_id: user_id.to_string(),
                    default_level: ExplainabilityLevel::KeyInsights, // Default level
                    context_specific_levels: HashMap::new(),
                    last_updated: Utc::now(),
                }
            })
    }

    // Filters raw explainability data based on the specified granularity level
    pub fn filter_explainability_data(&self, user_id: &str, raw_data: RawExplainabilityData, context: &str) -> HashMap<String, String> {
        println!("Filtering explainability data for user {} at context '{}'", user_id, context);
        let user_pref = self.get_user_preference(user_id);

        let effective_level = user_pref.context_specific_levels.get(context).unwrap_or(&user_pref.default_level);
        println!("Effective explainability level: {:?}", effective_level);

        let mut filtered_output = HashMap::new();
        filtered_output.insert("decision_id".to_string(), raw_data.decision_id.clone());
        filtered_output.insert("confidence_score".to_string(), raw_data.confidence_score.to_string());


        match effective_level {
            ExplainabilityLevel::HighLevelSummary => {
                filtered_output.insert("summary".to_string(), raw_data.high_level_summary);
            }
            ExplainabilityLevel::KeyInsights => {
                filtered_output.insert("summary".to_string(), raw_data.high_level_summary);
                filtered_output.insert("key_insights".to_string(), raw_data.key_insights.join("\n"));
            }
            ExplainabilityLevel::ReasoningSteps => {
                filtered_output.insert("summary".to_string(), raw_data.high_level_summary);
                filtered_output.insert("key_insights".to_string(), raw_data.key_insights.join("\n"));
                filtered_output.insert("reasoning_steps".to_string(), raw_data.all_reasoning_steps.join("\n"));
            }
            ExplainabilityLevel::DataSources => {
                filtered_output.insert("summary".to_string(), raw_data.high_level_summary);
                filtered_output.insert("key_insights".to_string(), raw_data.key_insights.join("\n"));
                filtered_output.insert("reasoning_steps".to_string(), raw_data.all_reasoning_steps.join("\n"));
                filtered_output.insert("data_sources".to_string(), format!("{:?}", raw_data.all_data_sources));
            }
            ExplainabilityLevel::TechnicalBreakdown => {
                filtered_output.insert("summary".to_string(), raw_data.high_level_summary);
                filtered_output.insert("key_insights".to_string(), raw_data.key_insights.join("\n"));
                filtered_output.insert("reasoning_steps".to_string(), raw_data.all_reasoning_steps.join("\n"));
                filtered_output.insert("data_sources".to_string(), format!("{:?}", raw_data.all_data_sources));
                filtered_output.insert("technical_details".to_string(), format!("{:?}", raw_data.technical_details));
            }
        }
        filtered_output
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_granularity_manager() {
        let manager = ExplainabilityGranularityManager::new();
        let user_id = "samuel_mukandara".to_string();

        let raw_data = RawExplainabilityData {
            decision_id: "decision_XYZ".to_string(),
            high_level_summary: "Tariro decided to optimize resource allocation.".to_string(),
            key_insights: vec!["Identified 15% energy waste.".to_string(), "No performance degradation expected.".to_string()],
            all_reasoning_steps: vec!["Analyzed power consumption logs.".to_string(), "Cross-referenced with performance metrics.".to_string(), "Simulated impact of resource reduction.".to_string()],
            all_data_sources: HashMap::from([("log_file_id_1".to_string(), "cpu_watts=80, usage=60%".to_string())]),
            technical_details: HashMap::from([("simd_optimizer_output".to_string(), "vectorized_matrix_result_ABC".to_string())]),
            confidence_score: 0.95,
        };

        // Test HighLevelSummary
        let pref_summary = UserExplainabilityPreference {
            user_id: user_id.clone(),
            default_level: ExplainabilityLevel::HighLevelSummary,
            context_specific_levels: HashMap::new(),
            last_updated: Utc::now(),
        };
        manager.set_user_preference(pref_summary);
        let output_summary = manager.filter_explainability_data(&user_id, raw_data.clone(), "general_query");
        assert!(output_summary.contains_key("summary"));
        assert!(!output_summary.contains_key("reasoning_steps"));

        // Test TechnicalBreakdown
        let mut context_levels = HashMap::new();
        context_levels.insert("critical_incident".to_string(), ExplainabilityLevel::TechnicalBreakdown);
        let pref_tech = UserExplainabilityPreference {
            user_id: user_id.clone(),
            default_level: ExplainabilityLevel::KeyInsights, // Default different from context-specific
            context_specific_levels: context_levels,
            last_updated: Utc::now(),
        };
        manager.set_user_preference(pref_tech);
        let output_tech = manager.filter_explainability_data(&user_id, raw_data.clone(), "critical_incident");
        assert!(output_tech.contains_key("technical_details"));
        assert!(output_tech.contains_key("reasoning_steps"));
        assert!(output_tech.contains_key("data_sources"));

        // Test default KeyInsights
        let output_default = manager.filter_explainability_data(&user_id, raw_data.clone(), "normal_operation");
        assert!(output_default.contains_key("key_insights"));
        assert!(!output_default.contains_key("technical_details"));
    }
}
*/