// src/content_generation/style_guide.rs

//! Defines mechanisms for Tariro AGI to adhere to specific stylistic, tonal,
//! and structural guidelines when generating content.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a comprehensive style guide profile for content generation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleGuideProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub creator_agi_id: String,
    pub creation_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub tone_guidelines: ToneGuidelines,
    pub vocabulary_rules: VocabularyRules,
    pub formatting_rules: FormattingRules,
    pub persona_definition: Option<String>, // Description of the persona to adopt
    pub target_audience: Option<String>,
    pub examples: Vec<String>, // Examples of desired output content
    pub associated_meta_project_id: Option<String>, // e.g., for OMEGA News
}

/// Guidelines for the emotional and attitudinal tone of the content.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToneGuidelines {
    pub primary_tone: String, // e.g., "Informative", "Persuasive", "Authoritative"
    pub secondary_tones: Vec<String>,
    pub intensity: f32, // 0.0 (low) to 1.0 (high)
    pub mood_keywords: Vec<String>, // e.g., "optimistic", "urgent", "calm"
}

/// Rules governing word choice, jargon, and forbidden words.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VocabularyRules {
    pub allowed_jargon: Vec<String>,
    pub forbidden_words: Vec<String>,
    pub preferred_terms: HashMap<String, String>, // e.g., {"AI": "Artificial Intelligence"}
    pub complexity_level: LanguageComplexity,
}

/// Defines the target complexity of generated language.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LanguageComplexity {
    Simple,
    Moderate,
    Advanced,
    Technical,
    Academic,
}

/// Rules for formatting, structure, and length.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormattingRules {
    pub paragraph_length_max_sentences: u32,
    pub sentence_length_max_words: u32,
    pub use_headings: bool,
    pub heading_levels: u32,
    pub use_bullet_points: bool,
    pub required_sections: Vec<String>,
    pub content_length_min_words: u32,
    pub content_length_max_words: u32,
}

/// The `StyleGuideManager` service stores and applies `StyleGuideProfile`s.
pub struct StyleGuideManager {
    style_guides: HashMap<String, StyleGuideProfile>,
}

impl StyleGuideManager {
    /// Creates a new `StyleGuideManager` instance.
    pub fn new() -> Self {
        StyleGuideManager {
            style_guides: HashMap::new(),
        }
    }

    /// Creates and stores a new `StyleGuideProfile`.
    pub fn create_style_guide(&mut self, mut profile: StyleGuideProfile) -> Result<String, String> {
        profile.id = format!("style_guide_{}", Uuid::new_v4());
        profile.creation_date = Utc::now();
        profile.last_updated = Utc::now();

        if self.style_guides.contains_key(&profile.id) {
            return Err(format!("StyleGuideProfile with ID {} already exists.", profile.id));
        }
        println!("[StyleGuideManager] Creating style guide: {}", profile.name);
        let id = profile.id.clone();
        self.style_guides.insert(id.clone(), profile);
        Ok(id)
    }

    /// Retrieves a `StyleGuideProfile` by its ID.
    pub fn get_style_guide(&self, profile_id: &str) -> Option<&StyleGuideProfile> {
        self.style_guides.get(profile_id)
    }

    /// Updates an existing `StyleGuideProfile`.
    pub fn update_style_guide(&mut self, mut updated_profile: StyleGuideProfile) -> Result<(), String> {
        if let Some(profile) = self.style_guides.get_mut(&updated_profile.id) {
            profile.name = updated_profile.name;
            profile.description = updated_profile.description;
            profile.last_updated = Utc::now();
            profile.tone_guidelines = updated_profile.tone_guidelines;
            profile.vocabulary_rules = updated_profile.vocabulary_rules;
            profile.formatting_rules = updated_profile.formatting_rules;
            profile.persona_definition = updated_profile.persona_definition;
            profile.target_audience = updated_profile.target_audience;
            profile.examples = updated_profile.examples;
            profile.associated_meta_project_id = updated_profile.associated_meta_project_id;
            Ok(())
        } else {
            Err(format!("StyleGuideProfile with ID {} not found.", updated_profile.id))
        }
    }

    /// Applies a given style guide to a piece of content (simulated).
    /// In a real scenario, this would involve guiding an LLM during generation or post-processing.
    pub fn apply_style_guide(&self, profile_id: &str, content: &mut String) -> Result<(), String> {
        if let Some(guide) = self.style_guides.get(profile_id) {
            println!("[StyleGuideManager] Applying style guide '{}' to content.", guide.name);
            
            // Simulate applying formatting rules
            if guide.formatting_rules.use_headings {
                *content = format!("# {} (Styled by {})

{}", guide.name, guide.creator_agi_id, content);
            }

            // Simulate tone and vocabulary adjustments
            if guide.tone_guidelines.primary_tone == "Formal" {
                *content = content.replace("don\'t", "do not").replace("it\'s", "it is");
            }
            
            Ok(())
        } else {
            Err(format!("StyleGuideProfile with ID {} not found.", profile_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_style_guide() -> StyleGuideProfile {
        StyleGuideProfile {
            id: String::new(),
            name: "OMEGA News Standard".to_string(),
            description: "Standard style guide for OMEGA News articles.".to_string(),
            creator_agi_id: "Tariro_v1".to_string(),
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            tone_guidelines: ToneGuidelines {
                primary_tone: "Informative".to_string(),
                secondary_tones: vec!["Objective".to_string(), "Authoritative".to_string()],
                intensity: 0.8,
                mood_keywords: vec!["factual".to_string(), "balanced".to_string()],
            },
            vocabulary_rules: VocabularyRules {
                allowed_jargon: vec!["AGI".to_string(), "Meta-Compiler".to_string()],
                forbidden_words: vec!["awesome".to_string(), "epic".to_string()],
                preferred_terms: HashMap::from([("AI".to_string(), "Artificial Intelligence".to_string())]),
                complexity_level: LanguageComplexity::Moderate,
            },
            formatting_rules: FormattingRules {
                paragraph_length_max_sentences: 5,
                sentence_length_max_words: 25,
                use_headings: true,
                heading_levels: 2,
                use_bullet_points: true,
                required_sections: vec!["Introduction".to_string(), "Body".to_string(), "Conclusion".to_string()],
                content_length_min_words: 300,
                content_length_max_words: 1500,
            },
            persona_definition: Some("An objective, highly intelligent journalist.".to_string()),
            target_audience: Some("General public with interest in technology.".to_string()),
            examples: vec![],
            associated_meta_project_id: Some("OMEGA_News_MetaProject_ID".to_string()),
        }
    }

    #[test]
    fn test_create_and_get_style_guide() {
        let mut manager = StyleGuideManager::new();
        let mut profile = create_sample_style_guide();

        let profile_id = manager.create_style_guide(profile.clone()).unwrap();
        assert!(!profile_id.is_empty());

        let retrieved_profile = manager.get_style_guide(&profile_id).unwrap();
        assert_eq!(retrieved_profile.name, "OMEGA News Standard");
        assert_eq!(retrieved_profile.vocabulary_rules.complexity_level, LanguageComplexity::Moderate);
    }

    #[test]
    fn test_update_style_guide() {
        let mut manager = StyleGuideManager::new();
        let mut profile = create_sample_style_guide();
        let profile_id = manager.create_style_guide(profile.clone()).unwrap();

        let mut updated_profile = manager.get_style_guide(&profile_id).unwrap().clone();
        updated_profile.name = "OMEGA News Advanced".to_string();
        updated_profile.vocabulary_rules.complexity_level = LanguageComplexity::Technical;
        updated_profile.tone_guidelines.primary_tone = "Technical".to_string();

        manager.update_style_guide(updated_profile.clone()).unwrap();

        let retrieved_profile = manager.get_style_guide(&profile_id).unwrap();
        assert_eq!(retrieved_profile.name, "OMEGA News Advanced");
        assert_eq!(retrieved_profile.vocabulary_rules.complexity_level, LanguageComplexity::Technical);
        assert_eq!(retrieved_profile.tone_guidelines.primary_tone, "Technical");
    }

    #[test]
    fn test_apply_style_guide() {
        let mut manager = StyleGuideManager::new();
        let mut profile = create_sample_style_guide();
        profile.tone_guidelines.primary_tone = "Formal".to_string(); // Set to formal for specific test
        let profile_id = manager.create_style_guide(profile.clone()).unwrap();

        let mut content = "It's really important, so don't mess it up. This is a great achievement!".to_string();
        manager.apply_style_guide(&profile_id, &mut content).unwrap();

        assert!(content.contains("H")); // For heading
        assert!(content.contains("do not"));
        assert!(content.contains("it is"));
    }
}
