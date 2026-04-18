// Tariro_ide_idpte/src/co_creation/human_style_learner.rs

//! Learns and models human creative styles and preferences based on their artifacts and feedback.

use crate::co_creation::creative_models::{CreativeArtifact, HumanCreativeStyle, CreativeFeedback, CreativeArtifactType};
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use uuid::Uuid;
use anyhow::Result;
use tracing::{info, warn};

/// The HumanStyleLearner observes human creative output and feedback to build style profiles.
pub struct HumanStyleLearner {
    style_database: HashMap<String, HumanCreativeStyle>, // user_id -> style_id -> HumanCreativeStyle
    // In a real system, this would also include machine learning models for feature extraction
    // and iterative style profile refinement.
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
    ) -> Result<HumanCreativeStyle> {
        info!("Learning from artifact {} for user {}", artifact.id, user_id);

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

        // 1. Feature Extraction (Placeholder for actual ML/NLP models)
        // This part would use specialized models to extract stylistic features from the artifact.
        let mut extracted_features = HashMap::new();
        match artifact.artifact_type {
            CreativeArtifactType::Code => {
                extracted_features.insert("indentation_style".to_string(), "tabs".to_string());
                extracted_features.insert("code_verbosity".to_string(), "high".to_string());
            },
            CreativeArtifactType::Text => {
                extracted_features.insert("tone".to_string(), "formal".to_string());
                extracted_features.insert("sentence_complexity".to_string(), "medium".to_string());
            },
            _ => {},
        }
        current_style.characteristics.extend(extracted_features);

        // 2. Feedback Analysis (Placeholder for sentiment analysis)
        if let Some(fb) = feedback {
            info!("Processing feedback for artifact {}: Rating {}, Comments: '{}'"), artifact.id, fb.rating, fb.comments);
            // In a real system:
            // - Perform sentiment analysis on `fb.comments` (e.g., using a pre-trained NLP model).
            // - Adjust style characteristics based on explicit suggestions in `fb.suggested_changes`.

            if fb.rating > 4.0 { // Positive feedback
                current_style.confidence = (current_style.confidence + 0.1).min(1.0);
                current_style.characteristics.insert("last_feedback_sentiment".to_string(), "positive".to_string());
            } else if fb.rating < 2.0 { // Negative feedback
                current_style.confidence = (current_style.confidence - 0.1).max(0.0);
                current_style.characteristics.insert("last_feedback_sentiment".to_string(), "negative".to_string());
            } else {
                current_style.characteristics.insert("last_feedback_sentiment".to_string(), "neutral".to_string());
            }
            // Incorporate suggested changes into style if patterns emerge
            if fb.suggested_changes.iter().any(|s| s.contains("concise")) {
                current_style.characteristics.insert("text_verbosity".to_string(), "low".to_string());
            }
        }

        // 3. Iterative Refinement of Style Profile
        // This would involve blending new features with existing style characteristics,
        // potentially using weighted averages or more complex neural network updates.

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
            CreativeArtifactType::Documentation => "documentation".to_string(),
            CreativeArtifactType::Other(s) => format!("{}", s),
        }
    }
}
