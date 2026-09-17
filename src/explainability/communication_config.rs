use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Defines the available communication styles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Concise,
    Verbose,
    Empathetic,
    Direct,
    Pedagogical, // Explanatory
}

// Represents a user's preference for communication style in different contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCommunicationConfig {
    pub user_id: String,
    pub default_style: CommunicationStyle,
    pub context_specific_styles: HashMap<String, CommunicationStyle>, // e.g., "security_alert": Direct
    pub last_updated: DateTime<Utc>,
}

// Placeholder for RAG to provide communication best practices and examples
pub struct RagCommunicationPractices {
    style_guides: HashMap<String, String>, // e.g., "formal_tone_guide": "Rules for formal communication"
    example_phrases: HashMap<CommunicationStyle, Vec<String>>, // Examples for each style
}

impl RagCommunicationPractices {
    pub fn new() -> Self {
        let mut style_guides = HashMap::new();
        style_guides.insert("formal_tone_guide".to_string(), "Use complete sentences, avoid slang, maintain objective tone.".to_string());
        style_guides.insert("concise_writing_tips".to_string(), "Get straight to the point, eliminate filler words.".to_string());

        let mut example_phrases = HashMap::new();
        example_phrases.insert(CommunicationStyle::Formal, vec!["Proceed with the scheduled deployment.".to_string(), "It is imperative that...".to_string()]);
        example_phrases.insert(CommunicationStyle::Concise, vec!["Deploy now.".to_string(), "Crucial: ...".to_string()]);
        example_phrases.insert(CommunicationStyle::Empathetic, vec!["I understand this is challenging, but...".to_string(), "I appreciate your patience.".to_string()]);

        RagCommunicationPractices { style_guides, example_phrases }
    }

    pub fn get_style_guide(&self, guide_name: &str) -> Option<&String> {
        self.style_guides.get(guide_name)
    }

    pub fn get_example_phrases(&self, style: &CommunicationStyle) -> Option<&Vec<String>> {
        self.example_phrases.get(style)
    }
}


pub struct CommunicationStyleManager {
    rag_practices: Arc<RagCommunicationPractices>,
    user_configurations: Mutex<HashMap<String, UserCommunicationConfig>>, // key: user_id
}

impl CommunicationStyleManager {
    pub fn new(rag_practices: Arc<RagCommunicationPractices>) -> Self {
        CommunicationStyleManager {
            rag_practices,
            user_configurations: Mutex::new(HashMap::new()),
        }
    }

    // Sets or updates the communication configuration for a user
    pub fn set_user_configuration(&self, config: UserCommunicationConfig) {
        println!("Setting communication configuration for user: {}", config.user_id);
        self.user_configurations.lock().unwrap().insert(config.user_id.clone(), config);
    }

    // Retrieves the current configuration for a user, or a default if none set
    pub fn get_user_configuration(&self, user_id: &str) -> UserCommunicationConfig {
        self.user_configurations.lock().unwrap()
            .get(user_id)
            .cloned()
            .unwrap_or_else(|| {
                println!("No specific configuration found for user {}. Using default.", user_id);
                UserCommunicationConfig {
                    user_id: user_id.to_string(),
                    default_style: CommunicationStyle::Direct, // Default style
                    context_specific_styles: HashMap::new(),
                    last_updated: Utc::now(),
                }
            })
    }

    // Adapts Tariro's response text based on the configured communication style
    pub fn adapt_response(&self, user_id: &str, raw_response: &str, context: &str) -> String {
        let config = self.get_user_configuration(user_id);
        let effective_style = config.context_specific_styles.get(context).unwrap_or(&config.default_style);
        println!("Adapting response for user {} with style: {:?}", user_id, effective_style);

        let mut adapted_response = raw_response.to_string();

        match effective_style {
            CommunicationStyle::Formal => {
                // Simulate making the response more formal
                adapted_response = format!("Regarding your query, it is important to note that {}", adapted_response);
                if let Some(guide) = self.rag_practices.get_style_guide("formal_tone_guide") {
                    println!("Applied formal style based on guide: {}", guide);
                }
            }
            CommunicationStyle::Concise => {
                // Simulate making the response more concise (very simplified)
                adapted_response = raw_response.split_whitespace().take(10).collect::<Vec<&str>>().join(" ") + "...";
                if let Some(examples) = self.rag_practices.get_example_phrases(&CommunicationStyle::Concise) {
                    println!("Concise examples: {:?}", examples);
                }
            }
            CommunicationStyle::Empathetic => {
                // Simulate adding empathetic language
                adapted_response = format!("I understand this situation is important. {}", adapted_response);
            }
            CommunicationStyle::Direct => {
                // Direct style might mean no additional embellishment, just the raw response
            }
            CommunicationStyle::Pedagogical => {
                // Simulate adding more explanation
                adapted_response = format!("To clarify, {} This means...", adapted_response);
            }
            CommunicationStyle::Verbose => {
                // Simulate making the response more verbose
                adapted_response = format!("Allow me to elaborate. It appears that {}. Therefore, it is important to consider...", adapted_response);
            }
        }
        adapted_response
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication_style_manager() {
        let rag_practices = Arc::new(RagCommunicationPractices::new());
        let manager = CommunicationStyleManager::new(rag_practices);
        let user_id = "samuel_mukandara".to_string();
        let raw_text = "The system is currently operating at 90% capacity.";

        // Test default Direct style
        let adapted_direct = manager.adapt_response(&user_id, raw_text, "general_info");
        println!("Direct: {}", adapted_direct);
        assert_eq!(adapted_direct, raw_text);

        // Configure for Formal style
        let config_formal = UserCommunicationConfig {
            user_id: user_id.clone(),
            default_style: CommunicationStyle::Formal,
            context_specific_styles: HashMap::new(),
            last_updated: Utc::now(),
        };
        manager.set_user_configuration(config_formal);
        let adapted_formal = manager.adapt_response(&user_id, raw_text, "report");
        println!("Formal: {}", adapted_formal);
        assert!(adapted_formal.starts_with("Regarding your query, it is important to note that"));

        // Configure for Empathetic style for 'critical_alert' context
        let mut context_styles = HashMap::new();
        context_styles.insert("critical_alert".to_string(), CommunicationStyle::Empathetic);
        let config_empathetic = UserCommunicationConfig {
            user_id: user_id.clone(),
            default_style: CommunicationStyle::Concise, // Default is concise
            context_specific_levels: context_styles,
            last_updated: Utc::now(),
        };
        manager.set_user_configuration(config_empathetic);
        let adapted_empathetic = manager.adapt_response(&user_id, raw_text, "critical_alert");
        println!("Empathetic: {}", adapted_empathetic);
        assert!(adapted_empathetic.starts_with("I understand this situation is important."));

        // Test that default Concise style is used for other contexts
        let adapted_concise = manager.adapt_response(&user_id, raw_text, "general_status");
        println!("Concise: {}", adapted_concise);
        assert!(adapted_concise.ends_with("..."));
    }
}
*/