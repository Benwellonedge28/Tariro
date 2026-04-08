pub mod task_delegator;
pub mod agent_communicator;
pub mod project_overseer;

pub struct DistributedAgencyEngine {
    delegator: task_delegator::TaskDelegator,
    communicator: agent_communicator::AgentCommunicator,
    overseer: project_overseer::ProjectOverseer,
}

impl DistributedAgencyEngine {
    pub fn new() -> Self {
        DistributedAgencyEngine {
            delegator: task_delegator::TaskDelegator::new(),
            communicator: agent_communicator::AgentCommunicator::new(),
            overseer: project_overseer::ProjectOverseer::new(),
        }
    }

    pub fn delegate_task(&self, task_name: &str, capabilities_needed: &str) -> Result<String, String> {
        if self.check_autonomy("delegate_task") {
            let assigned_agent = self.delegator.assign_task(task_name, capabilities_needed);
            Ok(format!("Task '{}' delegated to: {}", task_name, assigned_agent))
        } else {
            Err(format!("Permission denied to delegate task: {}", task_name))
        }
    }

    pub fn coordinate_agents(&self, agents: Vec<String>, common_goal: &str) {
        println!("Coordinating agents {:?} for goal: {}", agents, common_goal);
        self.communicator.broadcast_message(agents, &format!("Achieve: {}", common_goal));
    }

    pub fn get_project_status(&self, project_id: &str) -> String {
        self.overseer.generate_report(project_id)
    }

    fn check_autonomy(&self, action: &str) -> bool {
        // Placeholder for actual AutonomyPolicy check
        println!("Checking AutonomyPolicy for action: {}", action);
        true // Assume allowed for now
    }
}
