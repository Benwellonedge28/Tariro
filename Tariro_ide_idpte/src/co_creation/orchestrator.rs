// Tariro_ide_idpte/src/co_creation/orchestrator.rs

//! Coordinates the entire Human-AGI Co-Creative Synthesis Engine (HACSE).
//! This orchestrator manages creative workflows, integrates learning from human styles,
//! and drives the iterative generation and refinement of artifacts.

use crate::co_creation::creative_models::{CreativePrompt, CreativeArtifact, HumanCreativeStyle, CreativeFeedback, UserPreferences, InteractionModality, CreativeArtifactType};
use crate::co_creation::human_style_learner::HumanStyleLearner;
use crate::co_creation::synthesis_generator::SynthesisGenerator;
use crate::co_creation::feedback_analyzer::FeedbackAnalyzer;
use crate::co_creation::multi_modal_interface::MultiModalInterface;
use crate::agi_meta_goals::{MetaGoalManager, MetaGoalStatus}; // Import MetaGoalManager

use anyhow::Result;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid; // For generating mock artifact IDs
use chrono::Utc; // For timestamps

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
    meta_goal_manager: Arc<Mutex<MetaGoalManager>>, // Added MetaGoalManager
    // Channel for incoming creative prompts
    prompt_rx: mpsc::Receiver<CreativePrompt>,
    prompt_tx: mpsc::Sender<CreativePrompt>,
    current_options: Arc<Mutex<HacseOptions>>,
}

impl HacseOrchestrator {
    pub fn new(options: HacseOptions, meta_goal_manager: Arc<Mutex<MetaGoalManager>>) -> Self {
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
            meta_goal_manager,
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
        let meta_goal_manager_arc = self.meta_goal_manager.clone();

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
                    info!("HACSE: Periodically checking for feedback to process...");
                    let mock_artifact_id = Uuid::new_v4().to_string();
                    let mock_artifact = CreativeArtifact {
                        id: mock_artifact_id.clone(),
                        title: "Mock Implicit Artifact".to_string(),
                        description: None,
                        content: "This is some mock content that was edited by the user.".to_string(),
                        artifact_type: CreativeArtifactType::Text,
                        created_at: Utc::now(),
                        contributors: vec!["Tariro_AGI".to_string(), "samuelmukandara@gmail.com".to_string()],
                        version: 1,
                        tags: vec![],
                        parent_artifact_id: None,
                        associated_styles: vec![],
                        generative_model_used: crate::co_creation::creative_models::GenerativeModelType::TextGenerator,
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

                // Creative Pulse: Proactive suggestions aligned with meta-goals
                if now.timestamp() % options.creative_pulse_interval_seconds as i64 == 0 {
                    info!("HACSE: Generating proactive creative suggestions aligned with meta-goals...");
                    
                    let active_meta_goals = meta_goal_manager_arc.lock().unwrap().get_active_goals().await;
                    let target_meta_goal = active_meta_goals.into_iter().next().unwrap_or_else(|| {
                        warn!("No active meta-goals found, defaulting to generic creative pulse.");
                        crate::agi_meta_goals::MetaGoal {
                            id: "generic-creativity".to_string(),
                            title: "Generic Creative Exploration".to_string(),
                            description: "Explore novel creative avenues.".to_string(),
                            priority: 50,
                            status: MetaGoalStatus::Active,
                            last_updated: Utc::now(),
                            associated_agi_modules: vec![],
                            progress_percentage: 0.0,
                        }
                    });

                    let user_id = multi_modal_interface_arc.get_user_preferences().user_id.clone();
                    let user_style = if let Ok(learner) = style_learner_arc.lock().ok() {
                        learner.get_style(&user_id).await
                    } else { None };

                    let proactive_prompt = CreativePrompt {
                        id: Uuid::new_v4().to_string(),
                        user_id: user_id.clone(),
                        objective: format!("Develop a concept or artifact related to the meta-goal: '{}' ({}). Apply {} style.", 
                                            target_meta_goal.title,
                                            target_meta_goal.description,
                                            user_style.as_ref().map_or("user's preferred".to_string(), |s| s.style_name.clone())),
                        desired_artifact_type: CreativeArtifactType::Text, // Default to text for broader applicability
                        constraints: vec![format!("Align with meta-goal ID: {}", target_meta_goal.id)],
                        reference_artifacts: vec![],
                        desired_style_id: user_style.map(|s| s.id),
                        created_at: Utc::now(),
                    };

                    let generated_suggestion_artifact = if let Ok(generator) = generator_arc.lock() {
                        generator.generate_artifact(&proactive_prompt, user_style.as_ref()).await
                    } else {
                        error!("Failed to lock generator for proactive suggestion.");
                        Err(anyhow::anyhow!("Generator lock failed"))
                    };

                    match generated_suggestion_artifact {
                        Ok(artifact) => {
                            let suggestion_text = format!("New creative idea aligning with meta-goal '{}': '{}'. Content preview: '{}'"),
                                                        target_meta_goal.title, artifact.title, &artifact.content[..50].to_string());
                            multi_modal_interface_arc.present_suggestion(&suggestion_text).await;
                        },
                        Err(e) => error!("HACSE: Failed to generate proactive suggestion: {}", e),
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Small sleep to avoid busy-waiting
            }
        });

        Ok(())
    }
}
