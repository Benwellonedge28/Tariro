// src/content_generation/generator.rs

//! Provides core functionalities for Tariro AGI to generate text-based content
//! such as articles, summaries, reports, and creative narratives.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a generated piece of content.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneratedContent {
    pub id: String,
    pub title: String,
    pub content: String,
    pub content_type: ContentType,
    pub author_agi_id: String, // ID of the AGI instance or module that generated it
    pub creation_date: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub source_data_ids: Vec<String>, // IDs of KnowledgeItems, ManagedFiles, etc., used as sources
    pub quality_score: Option<f32>, // 0.0 to 1.0, based on internal evaluation or human feedback
    pub style_profile_id: Option<String>, // Reference to a StyleGuideProfile
    pub tone: ContentTone,
    pub keywords: Vec<String>,
}

/// Enumeration of different types of content Tariro can generate.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContentType {
    Article,
    Summary,
    Report,
    CreativeStory,
    CodeSnippet,
    Documentation,
    Dialogue,
    Custom(String),
}

/// Enumeration of content tones.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ContentTone {
    Neutral,
    Informative,
    Persuasive,
    Critical,
    Enthusiastic,
    Formal,
    Casual,
    Sarcastic,
    Humorous,
    Custom(String),
}

/// The `ContentGenerator` service is responsible for orchestrating content generation tasks.
pub struct ContentGenerator {
    // Dependencies: access to meta-learning, knowledge base, language models
}

impl ContentGenerator {
    /// Creates a new `ContentGenerator` instance.
    pub fn new() -> Self {
        ContentGenerator {}
    }

    /// Generates a new piece of content based on a prompt and desired parameters.
    pub async fn generate_content(
        &self,
        prompt: &str,
        content_type: ContentType,
        style_profile_id: Option<String>,
        tone: ContentTone,
        source_data_ids: Vec<String>,
    ) -> Result<GeneratedContent, String> {
        println!("[ContentGenerator] Generating {:?} content with prompt: {}", content_type, prompt);
        // Simulate interaction with an internal language model or external API
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let generated_text = match content_type {
            ContentType::Article => format!("An in-depth article about: {}. This was generated with a {:?} tone.", prompt, tone),
            ContentType::Summary => format!("A concise summary of: {}. Key points include...".to_string(), prompt),
            ContentType::Report => format!("Comprehensive report on '{}'. Findings show...".to_string(), prompt),
            _ => format!("Generated content for {:?} from prompt: {}", content_type, prompt),
        };

        let quality_score = if rand::random::<f32>() > 0.1 { Some(rand::random::<f32>() * 0.2 + 0.8) } else { Some(rand::random::<f32>() * 0.4 + 0.3) };

        Ok(GeneratedContent {
            id: format!("gen_cont_{}", Uuid::new_v4()),
            title: format!("Generated {}", content_type.to_string()),
            content: generated_text,
            content_type,
            author_agi_id: "Tariro_Core_v1".to_string(),
            creation_date: Utc::now(),
            last_modified: Utc::now(),
            source_data_ids,
            quality_score,
            style_profile_id,
            tone,
            keywords: prompt.split_whitespace().map(|s| s.to_string()).collect(),
        })
    }

    /// Refines an existing piece of content based on feedback or new instructions.
    pub async fn refine_content(&self, content: &mut GeneratedContent, refinement_instruction: &str) -> Result<(), String> {
        println!("[ContentGenerator] Refining content {}: {}", content.id, refinement_instruction);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        content.content = format!("{} (Refined based on: {})", content.content, refinement_instruction);
        content.last_modified = Utc::now();
        content.quality_score = Some((content.quality_score.unwrap_or(0.5) + 0.1).min(1.0));
        Ok(())
    }

    /// Translates content from one natural language to another.
    pub async fn translate_content(&self, content: &GeneratedContent, target_language: &str) -> Result<GeneratedContent, String> {
        println!("[ContentGenerator] Translating content {} to {}.", content.id, target_language);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        Ok(GeneratedContent {
            id: format!("gen_cont_trans_{}", Uuid::new_v4()),
            title: format!("{} ({})", content.title, target_language),
            content: format!("[Translated to {}] {}", target_language, content.content),
            content_type: content.content_type.clone(),
            author_agi_id: content.author_agi_id.clone(),
            creation_date: Utc::now(),
            last_modified: Utc::now(),
            source_data_ids: content.source_data_ids.clone(),
            quality_score: Some(content.quality_score.unwrap_or(0.8) * 0.95), // Slight quality drop simulated
            style_profile_id: content.style_profile_id.clone(),
            tone: content.tone.clone(),
            keywords: content.keywords.clone(),
        })
    }
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Article => "Article".to_string(),
            ContentType::Summary => "Summary".to_string(),
            ContentType::Report => "Report".to_string(),
            ContentType::CreativeStory => "Creative Story".to_string(),
            ContentType::CodeSnippet => "Code Snippet".to_string(),
            ContentType::Documentation => "Documentation".to_string(),
            ContentType::Dialogue => "Dialogue".to_string(),
            ContentType::Custom(s) => format!("Custom:{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_article() {
        let generator = ContentGenerator::new();
        let prompt = "The future of AI ethics";
        let content_type = ContentType::Article;
        let tone = ContentTone::Informative;
        let source_data = vec!["knowledge_item_ai_ethics_1".to_string()];

        let article = generator.generate_content(
            prompt,
            content_type,
            None,
            tone,
            source_data.clone(),
        ).await.unwrap();

        assert!(!article.id.is_empty());
        assert_eq!(article.content_type, ContentType::Article);
        assert!(article.content.contains(prompt));
        assert_eq!(article.tone, tone);
        assert_eq!(article.source_data_ids, source_data);
        assert!(article.quality_score.is_some());
    }

    #[tokio::test]
    async fn test_refine_content() {
        let generator = ContentGenerator::new();
        let mut content = generator.generate_content(
            "Initial draft",
            ContentType::Report,
            None,
            ContentTone::Neutral,
            vec![],
        ).await.unwrap();

        let initial_content = content.content.clone();
        let initial_score = content.quality_score.unwrap_or(0.0);
        let refinement_instruction = "Make it more concise and add a call to action.";

        generator.refine_content(&mut content, refinement_instruction).await.unwrap();

        assert!(content.content.contains(&initial_content));
        assert!(content.content.contains(refinement_instruction));
        assert!(content.quality_score.unwrap() > initial_score);
        assert!(content.last_modified > content.creation_date);
    }

    #[tokio::test]
    async fn test_translate_content() {
        let generator = ContentGenerator::new();
        let original_content = generator.generate_content(
            "Hello world from Tariro",
            ContentType::Dialogue,
            None,
            ContentTone::Neutral,
            vec![],
        ).await.unwrap();

        let translated_content = generator.translate_content(&original_content, "Spanish").await.unwrap();

        assert!(!translated_content.id.is_empty());
        assert_ne!(translated_content.id, original_content.id);
        assert!(translated_content.title.contains("Spanish"));
        assert!(translated_content.content.contains("[Translated to Spanish]"));
        assert_eq!(translated_content.content_type, original_content.content_type);
        assert!(translated_content.quality_score.unwrap() < original_content.quality_score.unwrap_or(1.0));
    }
}
