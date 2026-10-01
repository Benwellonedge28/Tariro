use crate::prelude::*;
pub mod task_delegator;
pub mod agent_communicator;
pub mod project_overseer;

pub struct DistributedAgencyEngine {
    delegator: task_delegator::TaskDelegator,
    communicator: agent_communicator::AgentCommunicator,
    overseer: project_overseer::ProjectOverseer,
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
        // considering task complexity, agent trustworthiness, and user-specific overrides from LearningInsights.
        if action.contains("delegate_routine_task") && user_prefs.contains("auto_delegate_low_risk_tasks") {
            AutonomyLevel::FullAuto
        } else if action.contains("delegate_sensitive_task") {
            AutonomyLevel::ConfirmOnly // Confirm delegation of sensitive tasks
        } else if action.contains("coordinate_agents") && context.contains("urgent_project") {
            AutonomyLevel::FullAuto // Auto-coordinate for urgent projects
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
        // Fetches dynamic user preferences from LearningInsights entity, including delegation preferences
        "auto_delegate_low_risk_tasks, prefer_oversight_on_critical_projects, trust_codegen_agent_x".to_string()
    }
    pub fn record_delegation_insight(&self, user_id: &str, insight: &str) {
        println!("Recording delegation insight for {}: {}", user_id, insight);
        // Persists new delegation learning to LearningInsights entity
    }
}

impl DistributedAgencyEngine {
    pub fn new() -> Self {
        DistributedAgencyEngine {
            delegator: task_delegator::TaskDelegator::new(),
            communicator: agent_communicator::AgentCommunicator::new(),
            overseer: project_overseer::ProjectOverseer::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn delegate_task(&self, user_id: &str, task_name: &str, capabilities_needed: &str, task_complexity: &str) -> Result<String, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "delegate_task",
            task_complexity,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Delegating task '{}' automatically.", task_name);
                let assigned_agent = self.delegator.assign_task(task_name, capabilities_needed);
                self.learning_agent.record_delegation_insight(user_id, &format!("FullAuto delegation of '{}' to '{}'.", task_name, assigned_agent));
                Ok(format!("Task '{}' delegated to: {}", task_name, assigned_agent))
            },
            AutonomyLevel::ConfirmOnly => {
                let assigned_agent = self.delegator.assign_task(task_name, capabilities_needed);
                Err(format!("Delegation of '{}' to '{}' suggested (Autonomy {:?}). Requires confirmation.", task_name, assigned_agent, autonomy_level))
            },
            _ => {
                Err(format!("Task delegation for '{}' blocked by autonomy policy: {:?}", task_name, autonomy_level))
            }
        }
    }

    pub fn coordinate_agents(&self, user_id: &str, agents: Vec<String>, common_goal: &str, urgency: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "coordinate_agents",
            urgency,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Coordinating agents {:?} automatically for goal: {}", agents, common_goal);
                self.communicator.broadcast_message(agents.clone(), &format!("Achieve: {}", common_goal));
                self.learning_agent.record_delegation_insight(user_id, &format!("FullAuto agent coordination for goal '{}' (Urgency: {}).", common_goal, urgency));
                format!("Agents coordinated in FullAuto mode for goal '{}'.", common_goal)
            },
            _ => {
                format!("Agent coordination for goal '{}' blocked by autonomy policy: {:?}", common_goal, autonomy_level)
            }
        }
    }

    pub fn get_project_status(&self, project_id: &str) -> String {
        self.overseer.generate_report(project_id)
    }
}
