use crate::prelude::*;
pub mod communication_adapter;
pub mod social_planner;
pub mod conflict_mediator;

pub struct SocialIntelligenceEngine {
    comm_adapter: communication_adapter::CommunicationAdapter,
    social_plan: social_planner::SocialPlanner,
    conflict_med: conflict_mediator::ConflictMediator,
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
        // considering social context, communication sensitivity, and user-specific overrides from LearningInsights.
        if action.contains("refine_casual_message") && user_prefs.contains("allow_auto_minor_refinement") {
            AutonomyLevel::FullAuto
        } else if action.contains("draft_sensitive_comm") {
            AutonomyLevel::ConfirmOnly // Always confirm sensitive communication drafts
        } else if action.contains("social_planning_suggestion") {
            AutonomyLevel::SuggestOnly // Always suggest social planning options
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
        // Fetches dynamic user preferences from LearningInsights entity, including communication and social preferences
        "allow_auto_minor_refinement, prefer_proactive_social_reminders, avoid_overly_formal_tone".to_string()
    }
    pub fn record_social_insight(&self, user_id: &str, insight: &str) {
        println!("Recording social intelligence insight for {}: {}", user_id, insight);
        // Persists new social intelligence learning to LearningInsights entity
    }
}

impl SocialIntelligenceEngine {
    pub fn new() -> Self {
        SocialIntelligenceEngine {
            comm_adapter: communication_adapter::CommunicationAdapter::new(),
            social_plan: social_planner::SocialPlanner::new(),
            conflict_med: conflict_mediator::ConflictMediator::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn adapt_communication_for_context(&self, user_id: &str, draft: &str, context: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "refine_message",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Adapting communication style automatically.");
                let adapted_draft = self.comm_adapter.adapt_style(user_id, draft, context);
                self.learning_agent.record_social_insight(user_id, &format!("FullAuto communication adaptation in context '{}'.", context));
                adapted_draft
            },
            AutonomyLevel::SuggestOnly => {
                let adapted_draft = self.comm_adapter.adapt_style(user_id, draft, context);
                self.learning_agent.record_social_insight(user_id, &format!("SuggestOnly communication adaptation in context '{}'.", context));
                format!("Communication adaptation suggested (Autonomy {:?}): '{}'. Review before sending.", autonomy_level, adapted_draft)
            },
            _ => {
                format!("Communication adaptation blocked by autonomy policy: {:?}", autonomy_level)
            }
        }
    }

    pub fn assist_social_planning(&self, user_id: &str, event_details: &str, context: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "social_planning_suggestion",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto | AutonomyLevel::SuggestOnly => {
                let plan = self.social_plan.plan_event(user_id, event_details);
                self.learning_agent.record_social_insight(user_id, &format!("Social planning assistance (Autonomy {:?}): '{}'.", autonomy_level, event_details));
                plan
            },
            _ => {
                format!("Social planning assistance blocked by autonomy policy: {:?}", autonomy_level)
            }
        }
    }

    pub fn get_conflict_resolution_advice(&self, user_id: &str, conflict_context: &str, sensitivity_level: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "conflict_advice",
            sensitivity_level,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto | AutonomyLevel::SuggestOnly => {
                let advice = self.conflict_med.mediate_informational(user_id, conflict_context.clone());
                self.learning_agent.record_social_insight(user_id, &format!("Conflict resolution advice (Autonomy {:?}): '{}'.", autonomy_level, conflict_context));
                advice
            },
            _ => {
                format!("Conflict resolution advice blocked by autonomy policy: {:?}", autonomy_level)
            }
        }
    }

    pub fn learn_social_preferences(&self, user_id: &str, feedback: String) {
        println!("Learning social preferences for user {}: {}", user_id, feedback);
        // Update LearningInsights with social interaction patterns
    }
}
