pub mod brainstorm_engine;
pub mod content_co_creator;
pub mod skill_amplifier;

pub struct CreativeCollaborationEngine {
    brainstorm_eng: brainstorm_engine::BrainstormEngine,
    co_creator: content_co_creator::ContentCoCreator,
    skill_amp: skill_amplifier::SkillAmplifier,
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
        // considering creative phase, sensitivity, and user-specific overrides from LearningInsights.
        if action.contains("generate_ideas") && user_prefs.contains("prefer_diverse_input") {
            AutonomyLevel::FullAuto
        } else if action.contains("refine_concept") && context.contains("sensitive_project") {
            AutonomyLevel::ConfirmOnly // Confirm changes on sensitive creative work
        } else if action.contains("skill_suggestion") {
            AutonomyLevel::SuggestOnly // Always suggest skill improvements for review
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
        // Fetches dynamic user preferences from LearningInsights entity, including creative preferences
        "prefer_diverse_input, prefer_subtle_suggestions, open_to_bold_interventions".to_string()
    }
    pub fn record_creative_insight(&self, user_id: &str, insight: &str) {
        println!("Recording creative insight for {}: {}", user_id, insight);
        // Persists new creative collaboration learning to LearningInsights entity
    }
}

impl CreativeCollaborationEngine {
    pub fn new() -> Self {
        CreativeCollaborationEngine {
            brainstorm_eng: brainstorm_engine::BrainstormEngine::new(),
            co_creator: content_co_creator::ContentCoCreator::new(),
            skill_amp: skill_amplifier::SkillAmplifier::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn start_brainstorm_session(&self, user_id: &str, topic: &str, context: &str) -> Result<Vec<String>, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "generate_ideas",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Generating brainstorm ideas automatically.");
                let ideas = self.brainstorm_eng.generate_ideas(topic);
                self.learning_agent.record_creative_insight(user_id, &format!("FullAuto idea generation for '{}' in context '{}'.", topic, context));
                Ok(ideas)
            },
            AutonomyLevel::SuggestOnly => {
                let ideas = self.brainstorm_eng.generate_ideas(topic);
                self.learning_agent.record_creative_insight(user_id, &format!("SuggestOnly idea generation for '{}' in context '{}'.", topic, context));
                Err(format!("Brainstorm ideas suggested (Autonomy {:?}): {:?}", autonomy_level, ideas))
            },
            _ => {
                Err(format!("Brainstorming for '{}' blocked by autonomy policy: {:?}", topic, autonomy_level))
            }
        }
    }

    pub fn co_create_content(&self, user_id: &str, user_input: &str, context: &str) -> Result<String, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "content_co_creation",
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Co-creating content automatically.");
                let result = self.co_creator.collaborate(user_input, context);
                self.learning_agent.record_creative_insight(user_id, &format!("FullAuto content co-creation in context '{}'.", context));
                Ok(result)
            },
            AutonomyLevel::SuggestOnly => {
                let result = self.co_creator.collaborate(user_input, context);
                self.learning_agent.record_creative_insight(user_id, &format!("SuggestOnly content co-creation in context '{}'.", context));
                Err(format!("Co-created content suggested (Autonomy {:?}): '{}'. Review and refine.", autonomy_level, result))
            },
            _ => {
                Err(format!("Content co-creation blocked by autonomy policy: {:?}", autonomy_level))
            }
        }
    }

    pub fn identify_skill_gap(&self, user_id: &str, task_context: &str) -> Result<Option<String>, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "skill_gap_identification",
            task_context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto | AutonomyLevel::SuggestOnly => {
                let gap = self.skill_amp.analyze_skill_gaps(user_id, task_context);
                self.learning_agent.record_creative_insight(user_id, &format!("Skill gap identified (Autonomy {:?}): {:?}.", autonomy_level, gap));
                Ok(gap)
            },
            _ => {
                Err(format!("Skill gap identification blocked by autonomy policy: {:?}", autonomy_level))
            }
        }
    }

    pub fn integrate_knowledge(&self, knowledge_item_id: &str) {
        println!("Integrating knowledge item: {}", knowledge_item_id);
        // Pull data from KnowledgeItem into collaboration context
    }

    pub fn learn_from_creative_process(&self, process_log: String) {
        println!("Learning from creative process: {}", process_log);
        // Update LearningInsights based on successful or failed creative attempts
    }
}
