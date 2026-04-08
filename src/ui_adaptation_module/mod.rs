pub mod ui_learner;
pub mod workflow_optimizer;
pub mod dynamic_ui_renderer;

pub struct UiAdaptationEngine {
    learner: ui_learner::UiLearner,
    optimizer: workflow_optimizer::WorkflowOptimizer,
    renderer: dynamic_ui_renderer::DynamicUiRenderer,
}

impl UiAdaptationEngine {
    pub fn new() -> Self {
        UiAdaptationEngine {
            learner: ui_learner::UiLearner::new(),
            optimizer: workflow_optimizer::WorkflowOptimizer::new(),
            renderer: dynamic_ui_renderer::DynamicUiRenderer::new(),
        }
    }

    pub fn adapt_ui_for_user(&self, user_id: &str, current_context: &str) {
        println!("Adapting UI for user: {} in context: {}", user_id, current_context);
        let preferences = self.learner.get_user_preferences(user_id);
        let optimized_workflow = self.optimizer.suggest_workflow(user_id, current_context);
        self.renderer.render_adaptive_ui(user_id, &preferences, &optimized_workflow);
    }

    pub fn record_interaction(&self, user_id: &str, interaction_data: String) {
        self.learner.record_user_interaction(user_id, interaction_data);
    }

    // This module would feed into LearningInsights
    pub fn generate_ui_learning_insight(&self, user_id: &str, insight: String) {
        println!("Generating UI learning insight for {}: {}", user_id, insight);
        // Call LearningInsight.create()
    }
}
