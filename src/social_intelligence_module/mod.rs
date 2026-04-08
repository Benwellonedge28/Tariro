pub mod communication_adapter;
pub mod social_planner;
pub mod conflict_mediator;

pub struct SocialIntelligenceEngine {
    comm_adapter: communication_adapter::CommunicationAdapter,
    social_plan: social_planner::SocialPlanner,
    conflict_med: conflict_mediator::ConflictMediator,
}

impl SocialIntelligenceEngine {
    pub fn new() -> Self {
        SocialIntelligenceEngine {
            comm_adapter: communication_adapter::CommunicationAdapter::new(),
            social_plan: social_planner::SocialPlanner::new(),
            conflict_med: conflict_mediator::ConflictMediator::new(),
        }
    }

    pub fn adapt_communication_for_context(&self, user_id: &str, draft: &str, context: &str) -> String {
        self.comm_adapter.adapt_style(user_id, draft, context)
    }

    pub fn assist_social_planning(&self, user_id: &str, event_details: &str) -> String {
        self.social_plan.plan_event(user_id, event_details)
    }

    pub fn get_conflict_resolution_advice(&self, user_id: &str, conflict_context: &str) -> String {
        self.conflict_med.mediate_informational(user_id, conflict_context)
    }

    // Integration with LearningInsights to understand user's social dynamics
    pub fn learn_social_preferences(&self, user_id: &str, feedback: String) {
        println!("Learning social preferences for user {}: {}", user_id, feedback);
        // Update LearningInsights with social interaction patterns
    }
}
