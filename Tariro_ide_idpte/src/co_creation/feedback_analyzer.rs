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
    generator: Arc<Mutex<SynthesisGenerator>>, // Added reference to SynthesisGenerator
}

impl FeedbackAnalyzer {
    pub fn new(style_learner: Arc<Mutex<HumanStyleLearner>>, generator: Arc<Mutex<SynthesisGenerator>>) -> Self {
        info!("Feedback Analyzer initialized.");
        FeedbackAnalyzer { style_learner, generator }
    }

    /// Analyzes explicit feedback provided by a user on a creative artifact.
    pub async fn analyze_explicit_feedback(&self, feedback: &CreativeFeedback, artifact: &CreativeArtifact) -> Result<()> {
        info!("Analyzing explicit feedback for artifact {}: Rating {}, Comments: '{}'"), feedback.artifact_id, feedback.rating, feedback.comments);

        // 1. Perform deeper sentiment analysis on `feedback.comments` (placeholder)
        let sentiment_signal = if feedback.comments.contains("great") || feedback.comments.contains("excellent") { "positive" }
                        else if feedback.comments.contains("bad") || feedback.comments.contains("poor") { "negative" }
                        else if feedback.comments.contains("concise") { "less verbose" } // Example of actionable signal
                        else if feedback.comments.contains("readable") { "more readable" }
                        else { "neutral" };

        info!("Simulated sentiment from feedback: {}", sentiment_signal);

        // 2. Translate feedback into actionable learning for HumanStyleLearner
        if let Ok(mut learner) = self.style_learner.lock() {
            learner.learn_from_artifact(&feedback.user_id, artifact, Some(feedback)).await?;
            info!("HumanStyleLearner updated based on explicit feedback.");
        }

        // 3. Signal SynthesisGenerator for adaptation
        if let Ok(mut generator) = self.generator.lock() {
            // Pass specific signals based on feedback analysis
            generator.adapt_to_feedback(sentiment_signal, artifact.generative_model_used.clone());
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
            // 3. Feed these insights back to the `HumanStyleLearner` and `SynthesisGenerator`
            //    to improve future co-creation iterations.
            if let Ok(mut learner) = self.style_learner.lock() {
                let synthetic_feedback = CreativeFeedback {
                    artifact_id: artifact.id.clone(),
                    user_id: artifact.contributors.iter().find(|c| c != &"Tariro_AGI").unwrap_or(&"unknown").to_string(),
                    rating: 3.5,
                    comments: format!("Implicit feedback: User edited artifact. Diff analysis shows: {}", edits.chars().take(50).collect::<String>()),
                    suggested_changes: vec!["Adapt to user's edit patterns".to_string()],
                    timestamp: Utc::now(),
                    feedback_type: FeedbackType::ImplicitUsage,
                };
                learner.learn_from_artifact(&synthetic_feedback.user_id, artifact, Some(&synthetic_feedback)).await?;
                info!("HumanStyleLearner updated based on implicit feedback (user edits).");
            }
            // Also signal SynthesisGenerator based on implicit feedback
            if let Ok(mut generator) = self.generator.lock() {
                generator.adapt_to_feedback(&format!("user edited: {}", edits.chars().take(20).collect::<String>()), artifact.generative_model_used.clone());
            }
        }
        Ok(())
    }
}
