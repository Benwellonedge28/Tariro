use crate::prelude::*;
pub mod performance_analyst;
pub mod knowledge_curator;
pub mod learning_path_generator;

pub struct DevelopmentAssistant {
    perf_analyst: performance_analyst::PerformanceAnalyst,
    knl_curator: knowledge_curator::KnowledgeCurator,
    lp_generator: learning_path_generator::LearningPathGenerator,
    autonomy_agent: AutonomyAgent,
    learning_agent: LearningAgent,
}

// Re-using conceptual structs for Autonomy and Learning
pub struct AutonomyAgent;
impl AutonomyAgent {
    pub fn new() -> Self { AutonomyAgent {} }
    pub fn determine_autonomy_level(&self, action: &str, context: &str, user_prefs: &str) -> AutonomyLevel {
        println!("Determining autonomy for '{}' in context '{}' with prefs '{}'", action, context, user_prefs);
        // In a real system, this would query the AutonomyPolicy entity,
        // considering feedback sensitivity, learning style preferences, and user-specific overrides from LearningInsights.
        if action.contains("performance_feedback") && user_prefs.contains("prefer_direct_feedback") {
            AutonomyLevel::FullAuto
        } else if action.contains("learning_path_change") && context.contains("critical_skill_acquisition") {
            AutonomyLevel::ConfirmOnly // Confirm major changes to critical learning paths
        } else if action.contains("content_curation") {
            AutonomyLevel::SuggestOnly // Always suggest curated content for review
        } else {
            AutonomyLevel::SuggestOnly
        }
    }
}

pub enum AutonomyLevel {
    FullAuto,
    SuggestOnly,
    ConfirmOnly,
    HumanRequired,
}

pub struct LearningAgent;
impl LearningAgent {
    pub fn new() -> Self { LearningAgent {} }
    pub fn get_user_preferences(&self, user_id: &str) -> String {
        println!("Retrieving user preferences for: {}", user_id);
        // Fetches dynamic user preferences from LearningInsights entity, including learning preferences
        "prefer_direct_feedback, visual_learner, fast_paced_learning".to_string()
    }
    pub fn record_development_insight(&self, user_id: &str, insight: &str) {
        println!("Recording development insight for {}: {}", user_id, insight);
        // Persists new development assistance learning to LearningInsights entity
    }
}

impl DevelopmentAssistant {
    pub fn new() -> Self {
        DevelopmentAssistant {
            perf_analyst: performance_analyst::PerformanceAnalyst::new(),
            knl_curator: knowledge_curator::KnowledgeCurator::new(),
            lp_generator: learning_path_generator::LearningPathGenerator::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn analyze_and_feedback(&self, user_id: &str, context: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "performance_feedback",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Providing direct performance feedback.");
                let feedback = self.perf_analyst.analyze_performance(user_id);
                self.learning_agent.record_development_insight(user_id, &format!("FullAuto feedback provided in context '{}'.", context));
                feedback
            },
            AutonomyLevel::SuggestOnly => {
                let feedback = self.perf_analyst.analyze_performance(user_id);
                self.learning_agent.record_development_insight(user_id, &format!("SuggestOnly feedback proposed in context '{}'.", context));
                format!("Feedback suggested (Autonomy {:?}): '{}'. Review before accepting.", autonomy_level, feedback)
            },
            _ => {
                format!("Performance feedback blocked by autonomy policy: {:?}", autonomy_level)
            }
        }
    }

    pub fn curate_knowledge(&self, user_id: &str, topic: &str, context: &str) -> Result<Vec<String>, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "content_curation",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Curating knowledge automatically.");
                let curated = self.knl_curator.get_personalized_content(user_id, topic);
                self.learning_agent.record_development_insight(user_id, &format!("FullAuto knowledge curation for '{}' in context '{}'.", topic, context));
                Ok(curated)
            },
            AutonomyLevel::SuggestOnly => {
                let curated = self.knl_curator.get_personalized_content(user_id, topic);
                self.learning_agent.record_development_insight(user_id, &format!("SuggestOnly knowledge curation for '{}' in context '{}'.", topic, context));
                Err(format!("Curated knowledge suggested (Autonomy {:?}): {:?}. Review and accept.", autonomy_level, curated))
            },
            _ => {
                Err(format!("Knowledge curation for '{}' blocked by autonomy policy: {:?}", autonomy_level))
            }
        }
    }

    pub fn generate_learning_path(&self, user_id: &str, goal: &str, context: &str) -> Result<String, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "learning_path_generation",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Generating learning path automatically.");
                let path = self.lp_generator.create_path(user_id, goal);
                self.learning_agent.record_development_insight(user_id, &format!("FullAuto learning path generated for '{}' in context '{}'.", goal, context));
                Ok(path)
            },
            AutonomyLevel::ConfirmOnly => {
                let path = self.lp_generator.create_path(user_id, goal);
                Err(format!("Learning path for '{}' suggested (Autonomy {:?}): '{}'. Requires confirmation.", goal, autonomy_level, path))
            },
            _ => {
                Err(format!("Learning path generation for '{}' blocked by autonomy policy: {:?}", autonomy_level))
            }
        }
    }

    pub fn ingest_user_data(&self, user_id: &str, data_type: &str, data: String) {
        println!("Ingesting {} data for user {}: {}", data_type, user_id, data);
        // This would involve reading from PerformanceLogs, ManagedFiles, etc.
    }

    pub fn update_learning_insights(&self, user_id: &str, insight: String) {
        println!("Updating LearningInsights for user {}: {}", user_id, insight);
        // Call LearningInsight.create()
    }
}
