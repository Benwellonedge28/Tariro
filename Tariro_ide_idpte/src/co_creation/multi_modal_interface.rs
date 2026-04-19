// Tariro_ide_idpte/src/co_creation/multi_modal_interface.rs

//! Handles multi-modal input and output for the Human-AGI Co-Creative Synthesis Engine (HACSE).
//! This module interfaces with various user interaction systems (GUI, CLI, Voice, Neuralink) to
//! receive creative prompts and present generated artifacts.

use crate::co_creation::creative_models::{CreativePrompt, CreativeArtifact, CreativeFeedback, CreativeArtifactType, UserPreferences, InteractionModality};
use anyhow::Result;
use tracing::{info, warn};
use uuid::Uuid;
use chrono::Utc;

/// The MultiModalInterface manages interactions for co-creation.
pub struct MultiModalInterface {
    // Internally holds connections to various UI/UX components
    user_preferences: UserPreferences,
}

impl MultiModalInterface {
    pub fn new(user_id: String, user_preferences: UserPreferences) -> Self {
        info!("Multi-Modal Interface for HACSE initialized for user {} with preferences {:?}.", user_id, user_preferences);
        MultiModalInterface { user_preferences }
    }

    /// Provides access to the stored user preferences.
    pub fn get_user_preferences(&self) -> UserPreferences {
        self.user_preferences.clone()
    }

    /// Receives a creative prompt from a human user (simulated).
    pub async fn receive_prompt(&self) -> Option<CreativePrompt> {
        // In a real system, this would listen for user input from GUI, voice, etc.
        // For now, it's a placeholder for how prompts would enter the system.
        None // No prompt received in this mock
    }

    /// Presents a generated creative artifact to the human user, adapting to preferences.
    pub async fn present_artifact(&self, artifact: &CreativeArtifact) {
        info!("HACSE Multi-Modal Interface: Presenting generated artifact (Type: {:?} Title: '{}')", artifact.artifact_type, artifact.title);

        let formatted_content = self.format_artifact_for_presentation(artifact).await;

        match self.user_preferences.preferred_modality {
            InteractionModality::Gui => info!("Presenting via GUI: '{}'"), formatted_content),
            InteractionModality::Cli => println!(
                "--- Generated Artifact ---\nTitle: {}\nType: {:?}\nContent:\n{}\n--------------------------",
                artifact.title, artifact.artifact_type, formatted_content
            ),
            InteractionModality::Voice => info!("Synthesizing voice output for: '{}'"), formatted_content.chars().take(100).collect::<String>()),
            InteractionModality::Neuralink => info!("Direct neural stimulation for artifact: '{}'"), formatted_content.chars().take(100).collect::<String>()),
            _ => println!(
                "--- Generated Artifact (Fallback) ---\nTitle: {}\nType: {:?}\nContent:\n{}\n--------------------------",
                artifact.title, artifact.artifact_type, formatted_content
            ),
        }
    }

    /// Receives feedback on a presented artifact from the human user (simulated).
    pub async fn receive_feedback(&self, artifact_id: &str) -> Option<CreativeFeedback> {
        // This would capture user ratings, comments, or implicit interactions
        // based on the preferred modality.
        if Utc::now().second() % 7 == 0 { // Simulate feedback randomly for demo
            info!("MultiModalInterface: Simulating feedback reception for artifact {}.", artifact_id);
            return Some(CreativeFeedback {
                artifact_id: artifact_id.to_string(),
                user_id: self.user_preferences.user_id.clone(),
                rating: 3.0 + (Utc::now().timestamp() % 3) as f64 / 2.0, // Simulate varied ratings
                comments: format!("I {} the output, but please adjust the {}.", 
                                    if Utc::now().timestamp() % 2 == 0 { "liked" } else { "disliked" },
                                    if Utc::now().timestamp() % 3 == 0 { "tone" } else { "structure" }),
                suggested_changes: vec!["Adjust X".to_string()],
                timestamp: Utc::now(),
                feedback_type: FeedbackType::DirectUser,
            });
        }
        None
    }

    /// Presents proactive creative suggestions to the user.
    pub async fn present_suggestion(&self, suggestion: &str) {
        info!("HACSE Multi-Modal Interface: Proactive Suggestion: '{}'"), suggestion);
        match self.user_preferences.preferred_modality {
            InteractionModality::Gui => info!("GUI Notification: {}", suggestion),
            InteractionModality::Cli => println!("SUGGESTION: {}", suggestion),
            InteractionModality::Voice => info!("Voice Alert: {}", suggestion),
            InteractionModality::Neuralink => info!("Neuralink ping: {}", suggestion),
            _ => println!("SUGGESTION (Fallback): {}", suggestion),
        }
    }

    /// Formats the artifact content based on user preferences and artifact type.
    async fn format_artifact_for_presentation(&self, artifact: &CreativeArtifact) -> String {
        let preferred_format = self.user_preferences.preferred_output_format.get(&artifact.artifact_type);
        let verbosity = self.user_preferences.verbosity_level;

        match artifact.artifact_type {
            CreativeArtifactType::Code => {
                let mut formatted = String::new();
                if verbosity > 0.5 { formatted.push_str("```rust\n"); } // Simulate syntax highlighting
                formatted.push_str(&artifact.content);
                if verbosity > 0.5 { formatted.push_str("\n```"); }
                formatted
            },
            CreativeArtifactType::Text => {
                if verbosity > 0.7 && artifact.description.is_some() {
                    format!("{}\n\n{}", artifact.description.as_ref().unwrap(), artifact.content)
                } else {
                    artifact.content.clone()
                }
            },
            _ => artifact.content.clone(), // Fallback for other types
        }
    }
}
