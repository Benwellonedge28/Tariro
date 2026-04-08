pub mod scheduler_optimizer;
pub mod info_filter;
pub mod digital_hygiene_manager;

pub struct ResourceOptimizer {
    scheduler: scheduler_optimizer::SchedulerOptimizer,
    filter: info_filter::InfoFilter,
    hygiene_mgr: digital_hygiene_manager::DigitalHygieneManager,
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
        // considering current cognitive load, task priority, and user-specific overrides from LearningInsights.
        if action.contains("reschedule") && user_prefs.contains("allow_dynamic_scheduling") && context.contains("low_focus") {
            AutonomyLevel::FullAuto
        } else if action.contains("filter_notification") && user_prefs.contains("prefer_strict_filtering") {
            AutonomyLevel::FullAuto
        } else if action.contains("digital_cleanup") && context.contains("high_cognitive_load") {
            AutonomyLevel::SuggestOnly // Suggest cleanup during high load
        } else {
            AutonomyLevel::ConfirmOnly // Default to confirming for significant changes
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
        // Fetches dynamic user preferences from LearningInsights entity, including resource optimization preferences
        "allow_dynamic_scheduling, prefer_strict_filtering, avoid_interruption_during_focus_mode".to_string()
    }
    pub fn record_resource_insight(&self, user_id: &str, insight: &str) {
        println!("Recording resource optimization insight for {}: {}", user_id, insight);
        // Persists new resource management learning to LearningInsights entity
    }
}

impl ResourceOptimizer {
    pub fn new() -> Self {
        ResourceOptimizer {
            scheduler: scheduler_optimizer::SchedulerOptimizer::new(),
            filter: info_filter::InfoFilter::new(),
            hygiene_mgr: digital_hygiene_manager::DigitalHygieneManager::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn optimize_schedule(&self, user_id: &str, current_tasks: Vec<String>, current_cognitive_load: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "reschedule_tasks",
            current_cognitive_load,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Dynamically optimizing schedule.");
                let result = self.scheduler.dynamic_reschedule(user_id, current_tasks.clone());
                self.learning_agent.record_resource_insight(user_id, &format!("FullAuto schedule optimization. Cognitive load: '{}'.", current_cognitive_load));
                result
            },
            AutonomyLevel::SuggestOnly => {
                let suggested_schedule = self.scheduler.dynamic_reschedule(user_id, current_tasks.clone());
                self.learning_agent.record_resource_insight(user_id, &format!("SuggestOnly schedule optimization. Cognitive load: '{}'.", current_cognitive_load));
                format!("Schedule optimization suggested (Autonomy {:?}): '{}'. Confirm to apply.", autonomy_level, suggested_schedule)
            },
            _ => {
                format!("Schedule optimization blocked by autonomy policy: {:?}", autonomy_level)
            }
        }
    }

    pub fn filter_incoming_info(&self, user_id: &str, info_stream: String, current_context: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "filter_notification",
            current_context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Filtering incoming information.");
                let result = self.filter.process_stream(user_id, info_stream.clone());
                self.learning_agent.record_resource_insight(user_id, &format!("FullAuto info filtering in context '{}'.", current_context));
                result
            },
            AutonomyLevel::SuggestOnly => {
                let filtered_suggestion = self.filter.process_stream(user_id, info_stream.clone());
                self.learning_agent.record_resource_insight(user_id, &format!("SuggestOnly info filtering in context '{}'.", current_context));
                format!("Information filtering suggested (Autonomy {:?}): '{}'. Review before seeing all.", autonomy_level, filtered_suggestion)
            },
            _ => {
                format!("Information filtering blocked by autonomy policy: {:?}", autonomy_level)
            }
        }
    }

    pub fn manage_digital_hygiene(&self, user_id: &str, path: &str, context: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "digital_cleanup",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Managing digital hygiene.");
                self.hygiene_mgr.clean_digital_space(user_id, path);
                self.learning_agent.record_resource_insight(user_id, &format!("FullAuto digital hygiene management in context '{}'.", context));
                "Digital hygiene managed automatically.".to_string()
            },
            AutonomyLevel::SuggestOnly => {
                self.hygiene_mgr.flag_digital_clutter(user_id, path);
                self.learning_agent.record_resource_insight(user_id, &format!("SuggestOnly digital hygiene management in context '{}'.", context));
                format!("Digital hygiene suggestions provided (Autonomy {:?}). Review recommended cleanup.", autonomy_level)
            },
            _ => {
                format!("Digital hygiene management blocked by autonomy policy: {:?}", autonomy_level)
            }
        }
    }

    pub fn update_with_cognitive_state(&self, user_id: &str, state: &str) {
        println!("Updating resource optimizer with cognitive state for user {}: {}", user_id, state);
        // Adjust optimization strategies based on user's cognitive load/energy
    }
}
