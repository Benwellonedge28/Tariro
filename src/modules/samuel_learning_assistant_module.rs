// src/modules/samuel_learning_assistant_module.rs

//! This module defines the Samuel Learning Assistant Module.
//! Its purpose is to autonomously assist Samuel in his "Master Learning Curriculum"
//! by analyzing his interactions, identifying learning opportunities, and proactively
//! suggesting relevant resources and learning paths.

use crate::{prelude::*,
    Module,
    ModuleContext,
    SamuelLearningAssistantConfig,
    FeatureProposalId, // To create goals for Samuel's learning curriculum
};
use async_trait::async_trait;
use tokio::time::{sleep, Duration};
use rand::Rng;
use std::collections::HashSet;
use serde_json::json;

/// Represents the Samuel Learning Assistant Module.
pub struct SamuelLearningAssistantModule {
    config: SamuelLearningAssistantConfig,
    context: ModuleContext, // To access API tools
}

#[async_trait]
pub trait Module: Send + Sync {
    fn new(context: &ModuleContext) -> Result<Self> where Self: Sized;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    fn name(&self) -> &'static str;
}

impl SamuelLearningAssistantModule {
    fn new(context: &ModuleContext) -> Result<Self> {
        let config = context.config.samuel_learning_assistant_module.clone().unwrap_or_default();
        Ok(Self { config, context: context.clone() })
    }

    async fn start(&self) -> Result<()> {
        info!("SamuelLearningAssistantModule starting...");
        info!("SamuelLearningAssistantModule started. Proactive suggestions: {}. Analysis frequency: {}s.",
              self.config.enable_proactive_learning_suggestions, self.config.analysis_frequency_seconds);

        if self.config.enable_proactive_learning_suggestions {
            let self_clone = self.clone(); // Clone self for the async task
            tokio::spawn(async move {
                loop {
                    sleep(Duration::from_secs(self_clone.config.analysis_frequency_seconds)).await;
                    if let Err(e) = self_clone.proactively_suggest_learning_resources().await {
                        error!("Error during proactive learning suggestion: {:?}", e);
                    }
                }
            });
        }
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        info!("SamuelLearningAssistantModule stopping...");
        info!("SamuelLearningAssistantModule stopped.");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "SamuelLearningAssistantModule"
    }
}

// Add #[derive(Clone)] to SamuelLearningAssistantModule
impl Clone for SamuelLearningAssistantModule {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            context: self.context.clone(),
        }
    }
}

impl SamuelLearningAssistantModule {
    /// Simulates analyzing Samuel's past interactions to infer learning interests or identify knowledge gaps.
    pub async fn analyze_samuel_interactions(&self) -> Result<Vec<String>> {
        info!("Analyzing Samuel's recent interactions for learning opportunities...");
        sleep(Duration::from_secs(rand::thread_rng().gen_range(3..=7))).await; // Simulate analysis time

        let mut inferred_interests = HashSet::new();

        // Simulate reading recent conversations
        let conv_resp = default_api::read_Conversation(
            vec!["title", "summary"],
            Some(10),
            None, None, Some("-created_date")
        ).await?;
        if let Some(records) = conv_resp.get("records").and_then(|r| r.as_array()) {
            for conv in records {
                if let Some(summary) = conv.get("summary").and_then(|s| s.as_str()) {
                    if summary.contains("Rust programming") { inferred_interests.insert("Rust Advanced Concepts".to_string()); }
                    if summary.contains("AI ethics") { inferred_interests.insert("AI Governance and Ethics".to_string()); }
                }
            }
        }

        // Simulate reading recent tasks
        let task_resp = default_api::read_ProjectTask(
            vec!["title", "description"],
            Some(10),
            None, None, Some("-created_date")
        ).await?;
        if let Some(records) = task_resp.get("records").and_then(|r| r.as_array()) {
            for task in records {
                if let Some(title) = task.get("title").and_then(|t| t.as_str()) {
                    if title.contains("OMEGA OS") { inferred_interests.insert("Operating System Design".to_string()); }
                }
                if let Some(desc) = task.get("description").and_then(|d| d.as_str()) {
                    if desc.contains("compiler") { inferred_interests.insert("Compiler Construction".to_string()); }
                }
            }
        }

        // Simulate reading recent goals
        let goal_resp = default_api::read_Goal(
            vec!["title", "description", "priority"],
            Some(5),
            Some(json!({"status": "active"}).into()),
            None, Some("-created_date")
        ).await?;
        if let Some(records) = goal_resp.get("records").and_then(|r| r.as_array()) {
            for goal in records {
                if let Some(title) = goal.get("title").and_then(|t| t.as_str()) {
                    if title.contains("Master Learning Curriculum") { inferred_interests.insert("Meta-Learning Strategies".to_string()); }
                }
                if let Some(priority) = goal.get("priority").and_then(|p| p.as_str()) {
                    if priority == "critical" || priority == "high" {
                        if let Some(title) = goal.get("title").and_then(|t| t.as_str()) {
                            inferred_interests.insert(format!("Deep dive into {}", title));
                        }
                    }
                }
            }
        }

        info!("Analysis complete. Inferred learning interests: {:?}", inferred_interests);
        Ok(inferred_interests.into_iter().collect())
    }

    /// Proactively suggests learning resources or paths to Samuel based on analyzed interactions.
    pub async fn proactively_suggest_learning_resources(&self) -> Result<()> {
        if !self.config.enable_proactive_learning_suggestions {
            info!("Proactive learning suggestions are disabled.");
            return Ok(());
        }

        let interests = self.analyze_samuel_interactions().await?;
        if interests.is_empty() {
            info!("No new learning interests inferred. No suggestions made.");
            return Ok(());
        }

        info!("Generating learning resource suggestions for Samuel based on interests: {:?}", interests);

        for interest in &interests {
            // Simulate searching for relevant knowledge items
            let knowledge_resp = default_api::read_KnowledgeItem(
                vec!["title", "tags"],
                Some(3),
                Some(json!({"tags": { "$in": [interest] }}).into()),
                None, None
            ).await?;
            let mut resources_found = Vec::new();
            if let Some(records) = knowledge_resp.get("records").and_then(|r| r.as_array()) {
                for item in records {
                    if let Some(title) = item.get("title").and_then(|t| t.as_str()) {
                        resources_found.push(format!("Internal Knowledge: {}", title));
                    }
                }
            }

            // Simulate external web search for learning (using web_search tool indirectly)
            let web_search_prompt = format!("Best advanced tutorials for {}", interest);
            resources_found.push(format!("External Search: {}", web_search_prompt));

            let suggestion_content = format!(
                "Tariro has identified a potential learning interest in '{}' based on your recent activities.\n\nSuggested Resources:\n- {}\n\nConsider adding this to your learning curriculum.",
                interest, resources_found.join("\n- ")
            );

            // Propose a new learning goal for Samuel
            let goal_title = format!("Master '{}' in Master Learning Curriculum", interest);
            let goal_description = format!("Deep dive into '{}' using suggested resources and practical application.", interest);

            let new_goal_result = default_api::create_Goal(
                goal_title.clone(),
                Some(goal_description.clone()),
                Some("learning".to_string()),
                Some("high"),
                Some(0.0),
                None, // resources_linked
                Some("active"),
                None, // sub_tasks
                None, // target_date
                Some(suggestion_content.clone()), // Tariro's notes field
            ).await?;

            let new_goal_id = new_goal_result.get("id").and_then(|id| id.as_str()).map(|s| s.to_string()).ok_or_else(|| anyhow!("Failed to get new Goal ID"))?;

            info!("Suggested learning path for Samuel: '{}'. Created new learning goal (ID: {}).", interest, new_goal_id);

            // Update LearningInsights
            self.update_learning_insights(interest, &suggestion_content, new_goal_id.clone()).await?;
        }
        Ok(())
    }

    /// Creates or updates a LearningInsight entity based on identified interests.
    pub async fn update_learning_insights(&self, insight_topic: &str, suggestion_content: &str, source_id: String) -> Result<()> {
        info!("Updating learning insights for topic: '{}'.", insight_topic);
        
        let insight_resp = default_api::read_LearningInsight(
            vec!["id", "insight", "times_reinforced"], 
            Some(1), 
            Some(json!({"insight": { "$regex": format!(".*{}.*", insight_topic), "$options": "i" }}).into()), 
            None, None
        ).await?;

        if let Some(records) = insight_resp.get("records").and_then(|r| r.as_array()) {
            if let Some(existing_insight) = records.first() {
                let insight_id = existing_insight.get("id").and_then(|id| id.as_str()).map(|s| s.to_string()).ok_or_else(|| anyhow!("Existing insight missing ID"))?;
                let times_reinforced = existing_insight.get("times_reinforced").and_then(|t| t.as_f64()).unwrap_or(0.0) as f32;
                
                default_api::update_LearningInsight(
                    insight_id.clone(),
                    None, // category
                    None, // confidence
                    Some(format!("{} (Reinforced with new suggestions: {})", existing_insight.get("insight").and_then(|i| i.as_str()).unwrap_or(""), suggestion_content)),
                    Some(source_id),
                    Some(times_reinforced + 1.0),
                ).await?;
                info!("Reinforced existing LearningInsight: {}.", insight_id);
            } else {
                default_api::create_LearningInsight(
                    format!("Samuel shows strong interest in: {}", insight_topic),
                    Some("topic_interest"),
                    Some(0.8), // High confidence for proactive suggestion
                    Some(source_id),
                    Some(1.0),
                ).await?;
                info!("Created new LearningInsight for topic: {}.", insight_topic);
            }
        }
        Ok(())
    }
}
