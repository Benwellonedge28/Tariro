pub mod ui_learner;
pub mod workflow_optimizer;
pub mod dynamic_ui_renderer;

pub struct UiAdaptationEngine {
    learner: ui_learner::UiLearner,
    optimizer: workflow_optimizer::WorkflowOptimizer,
    renderer: dynamic_ui_renderer::DynamicUiRenderer,
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
        // considering UI customization policies, current cognitive load, and user-specific overrides from LearningInsights.
        if context.contains("deep_work") && user_prefs.contains("auto_focus_mode") {
            AutonomyLevel::FullAuto
        } else if action.contains("major_layout_change") {
            AutonomyLevel::ConfirmOnly // Confirm major UI changes
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
        // Fetches dynamic user preferences from LearningInsights entity, including UI preferences
        "auto_focus_mode, prefer_dark_theme, prefer_minimal_notifications_during_deep_work".to_string()
    }
    pub fn record_ui_insight(&self, user_id: &str, insight: &str) {
        println!("Recording UI insight for {}: {}", user_id, insight);
        // Persists new UI adaptation learning to LearningInsights entity
    }
}

impl UiAdaptationEngine {
    pub fn new() -> Self {
        UiAdaptationEngine {
            learner: ui_learner::UiLearner::new(),
            optimizer: workflow_optimizer::WorkflowOptimizer::new(),
            renderer: dynamic_ui_renderer::DynamicUiRenderer::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn adapt_ui_for_user(&self, user_id: &str, current_context: &str) -> String {
        println!("Adapting UI for user: {} in context: {}", user_id, current_context);
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "ui_adaptation",
            current_context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Adapting UI automatically for user {}.", user_id);
                let preferences = self.learner.get_user_preferences(user_id);
                let optimized_workflow = self.optimizer.suggest_workflow(user_id, current_context);
                self.renderer.render_adaptive_ui(user_id, &preferences, &optimized_workflow);
                self.learning_agent.record_ui_insight(user_id, &format!("FullAuto UI adaptation applied for context: '{}'.", current_context));
                format!("UI adapted to FullAuto mode for context: {}.", current_context)
            },
            AutonomyLevel::SuggestOnly => {
                let preferences = self.learner.get_user_preferences(user_id);
                let optimized_workflow = self.optimizer.suggest_workflow(user_id, current_context);
                self.learning_agent.record_ui_insight(user_id, &format!("SuggestOnly UI adaptation proposed for context: '{}'.", current_context));
                format!("UI adaptation suggested for context: {}. Proposed: '{}', '{}'. Confirm to apply.", current_context, preferences, optimized_workflow)
            },
            AutonomyLevel::ConfirmOnly => {
                format!("Action Blocked: UI adaptation for context '{}' requires explicit confirmation.", current_context)
            },
            AutonomyLevel::HumanRequired => {
                format!("Action Blocked: UI adaptation for context '{}' requires human intervention.", current_context)
            },
        }
    }

    pub fn record_interaction(&self, user_id: &str, interaction_data: String) {
        self.learner.record_user_interaction(user_id, interaction_data);
    }

    pub fn generate_ui_learning_insight(&self, user_id: &str, insight: String) {
        println!("Generating UI learning insight for {}: {}", user_id, insight);
        // Call LearningInsight.create()
    }
}
