// Tariro_ide_idpte/src/co_creation/human_style_learner.rs

//! Learns and models human creative styles and preferences based on their artifacts and feedback.

use crate::co_creation::creative_models::{CreativeArtifact, HumanCreativeStyle, CreativeFeedback};
use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

/// The HumanStyleLearner observes human creative output and feedback to build style profiles.
pub struct HumanStyleLearner {
    style_database: HashMap<String, HumanCreativeStyle>, // user_id -> style_id -> HumanCreativeStyle
}

impl HumanStyleLearner {
    pub fn new() -> Self {
        println!("Human Style Learner initialized.");
        HumanStyleLearner { style_database: HashMap::new() }
    }

    /// Learns or updates a human's creative style based on a new artifact and optional feedback.
    pub async fn learn_from_artifact(
        &mut self,
        user_id: &str,
        artifact: &CreativeArtifact,
        feedback: Option<&CreativeFeedback>,
    ) -> Result<HumanCreativeStyle, String> {
        println!("Learning from artifact {} for user {}", artifact.id, user_id);

        // In a real implementation, this would involve:
        // 1. Feature extraction from the artifact (e.g., code metrics, linguistic patterns, visual elements).
        // 2. Comparing with existing style profiles for the user.
        // 3. Updating style parameters based on new features and feedback.
        // 4. Using machine learning (e.g., clustering, deep learning for style transfer) to model the style.

        let style_id = format!("style-{}", user_id);
        let mut current_style = self.style_database.entry(style_id.clone()).or_insert_with(|| HumanCreativeStyle {
            id: style_id.clone(),
            user_id: user_id.to_string(),
            style_name: "default_style".to_string(),
            characteristics: HashMap::new(),
            learned_from_artifacts: Vec::new(),
            last_updated_at: Utc::now(),
            confidence: 0.5,
        }).clone(); // Clone to modify and then insert

        // Mock update based on artifact type (placeholder for real feature extraction)
        match artifact.artifact_type {
            CreativeArtifactType::Code => {
                current_style.characteristics.insert("code_lang".to_string(), "Rust".to_string());
                current_style.characteristics.insert("code_indentation".to_string(), "4_spaces".to_string());
            },
            CreativeArtifactType::Text => {
                current_style.characteristics.insert("text_tone".to_string(), "formal".to_string());
                current_style.characteristics.insert("text_verbosity".to_string(), "medium".to_string());
            },
            _ => {},
        }

        if let Some(fb) = feedback {
            if fb.rating > 4.0 { // Positive feedback increases confidence
                current_style.confidence = (current_style.confidence + 0.1).min(1.0);
            } else if fb.rating < 2.0 { // Negative feedback decreases confidence
                current_style.confidence = (current_style.confidence - 0.1).max(0.0);
            }
            current_style.characteristics.insert("last_feedback_sentiment".to_string(), fb.rating.to_string());
        }

        current_style.learned_from_artifacts.push(artifact.id.clone());
        current_style.last_updated_at = Utc::now();
        current_style.style_name = format!("{}'s {} style", user_id, current_style.artifact_type_to_string(&artifact.artifact_type));

        self.style_database.insert(style_id, current_style.clone());
        Ok(current_style)
    }

    /// Retrieves a human's learned style.
    pub async fn get_style(&self, user_id: &str) -> Option<HumanCreativeStyle> {
        self.style_database.get(&format!("style-{}", user_id)).cloned()
    }
}

// Helper to get a string representation for style name generation
impl HumanCreativeStyle {
    fn artifact_type_to_string(&self, art_type: &CreativeArtifactType) -> String {
        match art_type {
            CreativeArtifactType::Code => "coding".to_string(),
            CreativeArtifactType::Text => "writing".to_string(),
            CreativeArtifactType::Image => "visual".to_string(),
            CreativeArtifactType::Audio => "audio".to_string(),
            CreativeArtifactType::Design => "design".to_string(),
            _ => "creative".to_string(),
        }
    }
}
