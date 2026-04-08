pub mod goal_decomposer;
pub mod resource_linker;
pub mod anticipatory_assistant;

pub struct GoalOrchestrationEngine {
    decomposer: goal_decomposer::GoalDecomposer,
    linker: resource_linker::ResourceLinker,
    assistant: anticipatory_assistant::AnticipatoryAssistant,
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
        // considering goal priority, user-specific overrides from LearningInsights.
        if context.contains("high_priority_goal") && user_prefs.contains("allow_full_auto_orchestration") {
            AutonomyLevel::FullAuto
        } else if action.contains("external_resource_allocation") {
            AutonomyLevel::ConfirmOnly // Confirm before allocating external resources
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
        // Fetches dynamic user preferences from LearningInsights entity, including goal management preferences
        "allow_full_auto_orchestration, prefer_detailed_breakdowns".to_string()
    }
    pub fn record_orchestration_insight(&self, user_id: &str, insight: &str) {
        println!("Recording orchestration insight for {}: {}", user_id, insight);
        // Persists new goal management learning to LearningInsights entity
    }
}

impl GoalOrchestrationEngine {
    pub fn new() -> Self {
        GoalOrchestrationEngine {
            decomposer: goal_decomposer::GoalDecomposer::new(),
            linker: resource_linker::ResourceLinker::new(),
            assistant: anticipatory_assistant::AnticipatoryAssistant::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn orchestrate_goal(&self, user_id: &str, goal_title: &str, goal_description: &str, goal_priority: &str) -> Result<Vec<String>, String> {
        println!("Orchestrating goal: {}", goal_title);
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "goal_orchestration",
            &format!("goal_priority:{}", goal_priority),
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Orchestrating goal '{}' automatically.", goal_title);
                let sub_tasks = self.decomposer.decompose_goal(goal_description);
                for task in &sub_tasks {
                    let resources = self.linker.find_relevant_resources(task);
                    println!("- Task: '{}', Suggested Resources: {:?}", task, resources);
                    self.assistant.offer_proactive_assistance(user_id, task, resources.clone());
                }
                self.learning_agent.record_orchestration_insight(user_id, &format!("FullAuto orchestration for goal '{}' resulted in {} sub-tasks.", goal_title, sub_tasks.len()));
                Ok(sub_tasks)
            },
            AutonomyLevel::SuggestOnly => {
                let sub_tasks = self.decomposer.decompose_goal(goal_description);
                let suggestions: Vec<String> = sub_tasks.iter().map(|task| {
                    let resources = self.linker.find_relevant_resources(task);
                    format!("Task: '{}', Suggested Resources: {:?}", task, resources)
                }).collect();
                self.learning_agent.record_orchestration_insight(user_id, &format!("SuggestOnly orchestration for goal '{}' proposed {} tasks.", goal_title, sub_tasks.len()));
                Err(format!("Orchestration for '{}' proposed. Review and confirm: {:?}", goal_title, suggestions))
            },
            AutonomyLevel::ConfirmOnly => {
                Err(format!("Action Blocked: Goal orchestration for '{}' requires explicit confirmation.", goal_title))
            },
            AutonomyLevel::HumanRequired => {
                Err(format!("Action Blocked: Goal orchestration for '{}' requires human intervention.", goal_title))
            },
        }
    }

    pub fn take_automated_initiative(&self, goal_domain: &str) {
        // This would be triggered based on AutonomyPolicy set to 'full_auto'
        println!("Taking automated initiative for goal domain: {}", goal_domain);
        // e.g., identify next task, execute external integration, report progress
    }

    pub fn learn_from_feedback(&self, task_completed: &str, efficiency_score: f32) {
        println!("Learning from feedback for task: {} with efficiency: {}", task_completed, efficiency_score);
        // Update LearningInsights for better future orchestration
    }
}
