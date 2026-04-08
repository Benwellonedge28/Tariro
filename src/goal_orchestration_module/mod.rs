pub mod goal_decomposer;
pub mod resource_linker;
pub mod anticipatory_assistant;

pub struct GoalOrchestrationEngine {
    decomposer: goal_decomposer::GoalDecomposer,
    linker: resource_linker::ResourceLinker,
    assistant: anticipatory_assistant::AnticipatoryAssistant,
}

impl GoalOrchestrationEngine {
    pub fn new() -> Self {
        GoalOrchestrationEngine {
            decomposer: goal_decomposer::GoalDecomposer::new(),
            linker: resource_linker::ResourceLinker::new(),
            assistant: anticipatory_assistant::AnticipatoryAssistant::new(),
        }
    }

    pub fn orchestrate_goal(&self, goal_title: &str, goal_description: &str) -> Vec<String> {
        println!("Orchestrating goal: {}", goal_title);
        let sub_tasks = self.decomposer.decompose_goal(goal_description);
        for task in &sub_tasks {
            let resources = self.linker.find_relevant_resources(task);
            println!("- Task: '{}', Suggested Resources: {:?}", task, resources);
            self.assistant.offer_proactive_assistance(task, resources);
        }
        sub_tasks
    }

    pub fn take_automated_initiative(&self, goal_domain: &str) {
        // This would be triggered based on AutonomyPolicy set to 'full_auto'
        println!("Taking automated initiative for goal domain: {}", goal_domain);
        // e.g., identify next task, execute external integration, report progress
    }

    // Integration with LearningInsights would refine decomposition and suggestions over time
    pub fn learn_from_feedback(&self, task_completed: &str, efficiency_score: f32) {
        println!("Learning from feedback for task: {} with efficiency: {}", task_completed, efficiency_score);
        // Update LearningInsights for better future orchestration
    }
}
