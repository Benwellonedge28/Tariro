// Tariro_ide_idpte/src/co_creation/feedback_analyzer.rs

//! Processes human feedback on creative artifacts to refine Tariro's generative processes.

use crate::co_creation::creative_models::{CreativeFeedback, CreativeArtifact, FeedbackType};
use crate::co_creation::human_style_learner::HumanStyleLearner;
use crate::co_creation::synthesis_generator::SynthesisGenerator;
use anyhow::Result;
use std::sync::{Arc, Mutex};
use tracing::{info, warn};

/// The FeedbackAnalyzer processes explicit and implicit feedback.
pub struct FeedbackAnalyzer {
    style_learner: Arc<Mutex<HumanStyleLearner>>,
    // In a real system, this could also hold a reference to the SynthesisGenerator
    // or a mechanism to signal it for adjustments based on feedback.
}

impl FeedbackAnalyzer {
    pub fn new(style_learner: Arc<Mutex<HumanStyleLearner>>) -> Self {
        info!("Feedback Analyzer initialized.");
        FeedbackAnalyzer { style_learner }
    }

    /// Analyzes explicit feedback provided by a user on a creative artifact.
    pub async fn analyze_explicit_feedback(&self, feedback: &CreativeFeedback, artifact: &CreativeArtifact) -> Result<()> {
        info!("Analyzing explicit feedback for artifact {}: Rating {}, Comments: '{}'"), feedback.artifact_id, feedback.rating, feedback.comments);

        // 1. Perform deeper sentiment analysis on `feedback.comments` (placeholder)
        let sentiment = if feedback.comments.contains("great") || feedback.comments.contains("excellent") { "positive" }
                        else if feedback.comments.contains("bad") || feedback.comments.contains("poor") { "negative" }
                        else { "neutral" };

        info!("Simulated sentiment from feedback: {}", sentiment);

        // 2. Translate feedback into actionable learning for HumanStyleLearner
        if let Ok(mut learner) = self.style_learner.lock() {
            // Here, we re-feed the artifact with the feedback to update the style profile
            // The `learn_from_artifact` method in HumanStyleLearner is designed to handle this.
            learner.learn_from_artifact(&feedback.user_id, artifact, Some(feedback)).await?;
            info!("HumanStyleLearner updated based on explicit feedback.");
        }

        // 3. Potentially signal SynthesisGenerator for adjustment (e.g., if negative feedback is consistent)
        // This would involve a more direct channel or a shared state for performance metrics.
        if feedback.rating < 3.0 {
            warn!("Negative feedback detected for artifact {}. SynthesisGenerator might need adjustment.", artifact.id);
        }

        Ok(())
    }

    /// Analyzes implicit feedback (e.g., user edits, usage patterns) on a creative artifact.
    pub async fn analyze_implicit_feedback(&self, artifact: &CreativeArtifact, user_edits: Option<&str>) -> Result<()> {
        info!("Analyzing implicit feedback for artifact {}", artifact.id);

        if let Some(edits) = user_edits {
            info!("User made edits to content. Comparing original with edits...");
            // In a real system:
            // 1. Diff the original artifact content with `user_edits` to identify specific changes.
            // 2. Extract patterns from these changes (e.g., preference for shorter sentences, specific code refactorings).
            // 3. Feed these insights back to the `HumanStyleLearner` to refine the user's style profile.
            if let Ok(mut learner) = self.style_learner.lock() {
                // Create a synthetic feedback based on edits for the learner
                let synthetic_feedback = CreativeFeedback {
                    artifact_id: artifact.id.clone(),
                    user_id: artifact.contributors.iter().find(|c| c != &"Tariro_AGI").unwrap_or(&"unknown").to_string(), // Assume first non-Tariro contributor is user
                    rating: 3.5, // Neutral rating, as edits are implicit
                    comments: format!("Implicit feedback: User edited artifact. Diff analysis shows: {}", edits.chars().take(50).collect::<String>()),
                    suggested_changes: vec!["Adapt to user's edit patterns".to_string()],
                    timestamp: Utc::now(),
                    feedback_type: FeedbackType::ImplicitUsage,
                };
                learner.learn_from_artifact(&synthetic_feedback.user_id, artifact, Some(&synthetic_feedback)).await?;
                info!("HumanStyleLearner updated based on implicit feedback (user edits).");
            }
        }
        Ok(())
    }
}
