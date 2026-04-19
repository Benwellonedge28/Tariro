// Tariro_ide_idpte/src/co_creation/orchestrator.rs

//! Coordinates the entire Human-AGI Co-Creative Synthesis Engine (HACSE).
//! This orchestrator manages creative workflows, integrates learning from human styles,
//! and drives the iterative generation and refinement of artifacts.

use crate::co_creation::creative_models::{CreativePrompt, CreativeArtifact, HumanCreativeStyle, CreativeFeedback, UserPreferences, InteractionModality};
use crate::co_creation::human_style_learner::HumanStyleLearner;
use crate::co_creation::synthesis_generator::SynthesisGenerator;
use crate::co_creation::feedback_analyzer::FeedbackAnalyzer;
use crate::co_creation::multi_modal_interface::MultiModalInterface;

use anyhow::Result;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};

/// Configuration options for the HACSE Orchestrator.
#[derive(Debug, Clone)]
pub struct HacseOptions {
    pub feedback_processing_interval_seconds: u64,
    pub style_learning_interval_seconds: u64,
    pub creative_pulse_interval_seconds: u64, // For proactive suggestions
}

impl Default for HacseOptions {
    fn default() -> Self {
        HacseOptions {
            feedback_processing_interval_seconds: 10, // Process feedback every 10 seconds
            style_learning_interval_seconds: 60, // Update styles every minute
            creative_pulse_interval_seconds: 300, // Generate proactive ideas every 5 minutes
        }
    }
}

/// The main orchestrator for the Human-AGI Co-Creative Synthesis Engine.
pub struct HacseOrchestrator {
    style_learner: Arc<Mutex<HumanStyleLearner>>,
    generator: Arc<Mutex<SynthesisGenerator>>,
    feedback_analyzer: Arc<FeedbackAnalyzer>,
    multi_modal_interface: Arc<MultiModalInterface>,
    // Channel for incoming creative prompts
    prompt_rx: mpsc::Receiver<CreativePrompt>,
    prompt_tx: mpsc::Sender<CreativePrompt>,
    current_options: Arc<Mutex<HacseOptions>>,
}

impl HacseOrchestrator {
    pub fn new(options: HacseOptions) -> Self {
        let (prompt_tx, prompt_rx) = mpsc::channel(100); // Buffer for 100 pending prompts
        let style_learner_arc = Arc::new(Mutex::new(HumanStyleLearner::new()));
        let generator_arc = Arc::new(Mutex::new(SynthesisGenerator::new()));
        
        // Default user preferences for the multi-modal interface
        let default_user_prefs = UserPreferences {
            user_id: "samuelmukandara@gmail.com".to_string(), // Default to Samuel as the primary user
            preferred_modality: InteractionModality::Cli,
            preferred_output_format: HashMap::new(), // Can be populated later
            verbosity_level: 0.8,
        };

        HacseOrchestrator {
            style_learner: style_learner_arc.clone(),
            generator: generator_arc.clone(),
            feedback_analyzer: Arc::new(FeedbackAnalyzer::new(style_learner_arc, generator_arc.clone())),
            multi_modal_interface: Arc::new(MultiModalInterface::new("samuelmukandara@gmail.com".to_string(), default_user_prefs)),
            prompt_rx,
            prompt_tx,
            current_options: Arc::new(Mutex::new(options)),
        }
    }

    /// Returns a sender to submit new creative prompts to the orchestrator.
    pub fn get_prompt_sender(&self) -> mpsc::Sender<CreativePrompt> {
        self.prompt_tx.clone()
    }

    /// Starts the main HACSE orchestration loop.
    pub async fn start_loop(&self) -> Result<()> {
        let options_arc = self.current_options.clone();
        let prompt_tx_clone = self.prompt_tx.clone();
        let mut prompt_rx = self.prompt_rx.take().expect("Prompt receiver already taken");
        let style_learner_arc = self.style_learner.clone();
        let generator_arc = self.generator.clone();
        let feedback_analyzer_arc = self.feedback_analyzer.clone();
        let multi_modal_interface_arc = self.multi_modal_interface.clone();

        info!("HACSE Orchestrator started.");

        tokio::spawn(async move {
            loop {
                let options = options_arc.lock().unwrap().clone();
                let now = chrono::Utc::now();

                // Process incoming prompts
                while let Ok(prompt) = prompt_rx.try_recv() {
                    info!("HACSE: Received new creative prompt for '{}'.", prompt.objective);
                    // Retrieve user's learned style
                    let user_style = if let Ok(learner) = style_learner_arc.lock().ok() {
                        learner.get_style(&prompt.user_id).await
                    } else { None };

                    // Generate artifact
                    let generated_artifact = if let Ok(generator) = generator_arc.lock() {
                        generator.generate_artifact(&prompt, user_style.as_ref()).await
                    } else {
                        error!("Failed to lock generator for artifact generation.");
                        Err(anyhow::anyhow!("Generator lock failed"))
                    };

                    match generated_artifact {
                        Ok(artifact) => {
                            info!("HACSE: Generated artifact '{}'. Content preview: '{}'"), artifact.title, &artifact.content[..20].to_string());
                            // Send artifact to multi-modal interface for presentation/user feedback
                            multi_modal_interface_arc.present_artifact(&artifact).await;

                            // Simulate immediate processing of feedback (if any was just given)
                            if let Some(feedback) = multi_modal_interface_arc.receive_feedback(&artifact.id).await {
                                if let Err(e) = feedback_analyzer_arc.analyze_explicit_feedback(&feedback, &artifact).await {
                                    error!("Error analyzing explicit feedback: {}", e);
                                }
                            }

                        },
                        Err(e) => error!("HACSE: Failed to generate artifact for prompt '{}': {}", prompt.objective, e),
                    }
                }

                // Periodically process feedback (simulated)
                if now.timestamp() % options.feedback_processing_interval_seconds as i64 == 0 {
                    // This would typically involve iterating through a queue of pending feedback items
                    // and artifacts for implicit feedback analysis.
                    info!("HACSE: Periodically checking for feedback to process...");
                    // Simulate implicit feedback for an existing artifact
                    // This would require fetching an artifact from a store
                    // For now, it's a simple placeholder call
                    let mock_artifact_id = Uuid::new_v4().to_string();
                    let mock_artifact = CreativeArtifact {
                        id: mock_artifact_id.clone(),
                        title: "Mock Implicit Artifact".to_string(),
                        description: None,
                        content: "This is some mock content that was edited by the user.".to_string(),
                        artifact_type: creative_models::CreativeArtifactType::Text,
                        created_at: Utc::now(),
                        contributors: vec!["Tariro_AGI".to_string(), "samuelmukandara@gmail.com".to_string()],
                        version: 1,
                        tags: vec![],
                        parent_artifact_id: None,
                        associated_styles: vec![],
                        generative_model_used: creative_models::GenerativeModelType::TextGenerator,
                    };
                    // To avoid endless mocking, only simulate this if feedback processing interval is low
                    if options.feedback_processing_interval_seconds < 30 {
                        if let Err(e) = feedback_analyzer_arc.analyze_implicit_feedback(&mock_artifact, Some("User made minor edits to sentence structure.")).await {
                            error!("Error analyzing implicit feedback: {}", e);
                        }
                    }

                }

                // Periodically update styles
                if now.timestamp() % options.style_learning_interval_seconds as i64 == 0 {
                    info!("HACSE: Periodically updating human creative styles...");
                }

                // Creative Pulse: Proactive suggestions
                if now.timestamp() % options.creative_pulse_interval_seconds as i64 == 0 {
                    info!("HACSE: Generating proactive creative suggestions...");
                    multi_modal_interface_arc.present_suggestion("Consider a new angle for project X, focusing on sustainable quantum computing.").await;
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Small sleep to avoid busy-waiting
            }
        });

        Ok(())
    }
}
