pub mod solution_architect;
pub mod prototype_designer;
pub mod micro_agent_manager;

pub struct CoCreationEngine {
    architect: solution_architect::SolutionArchitect,
    designer: prototype_designer::PrototypeDesigner,
    agent_mgr: micro_agent_manager::MicroAgentManager,
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
        // considering problem complexity, potential impact, and user-specific overrides from LearningInsights.
        if action.contains("generate_simple_script") && user_prefs.contains("auto_generate_low_impact_tools") {
            AutonomyLevel::FullAuto
        } else if action.contains("deploy_critical_system") {
            AutonomyLevel::ConfirmOnly // Confirm deployment of critical systems
        } else if action.contains("propose_architecture") {
            AutonomyLevel::SuggestOnly // Suggest architectures for review
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
        // Fetches dynamic user preferences from LearningInsights entity, including co-creation preferences
        "auto_generate_low_impact_tools, prefer_iterative_prototyping, always_review_critical_deployments".to_string()
    }
    pub fn record_co_creation_insight(&self, user_id: &str, insight: &str) {
        println!("Recording co-creation insight for {}: {}", user_id, insight);
        // Persists new co-creation learning to LearningInsights entity
    }
}

impl CoCreationEngine {
    pub fn new() -> Self {
        CoCreationEngine {
            architect: solution_architect::SolutionArchitect::new(),
            designer: prototype_designer::PrototypeDesigner::new(),
            agent_mgr: micro_agent_manager::MicroAgentManager::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn co_create_tool(&self, user_id: &str, problem_statement: &str, tool_impact: &str) -> Result<String, String> {
        println!("Starting co-creation for user {} on problem: {}", user_id, problem_statement);
        let user_prefs = self.learning_agent.get_user_preferences(user_id);

        let autonomy_level_architect = self.autonomy_agent.determine_autonomy_level(
            "propose_architecture",
            tool_impact,
            &user_prefs
        );
        let autonomy_level_deploy = self.autonomy_agent.determine_autonomy_level(
            "deploy_tool",
            tool_impact,
            &user_prefs
        );

        let mut result_messages = Vec::new();

        let architecture = match autonomy_level_architect {
            AutonomyLevel::FullAuto => {
                let arch = self.architect.propose_architecture(problem_statement);
                self.learning_agent.record_co_creation_insight(user_id, &format!("FullAuto architecture proposal for '{}'.", problem_statement));
                result_messages.push(format!("Architecture proposed: '{}'.", arch));
                arch
            },
            _ => {
                let arch = self.architect.propose_architecture(problem_statement);
                result_messages.push(format!("Architecture suggested (Autonomy {:?}): '{}'. Review required.", autonomy_level_architect, arch));
                return Err(result_messages.join(" "));
            }
        };

        let prototype = self.designer.design_prototype(user_id, &architecture);
        result_messages.push(format!("Prototype designed: '{}'.", prototype));

        let deployment_status = match autonomy_level_deploy {
            AutonomyLevel::FullAuto => {
                let status = self.agent_mgr.deploy_micro_agent(user_id, &prototype);
                self.learning_agent.record_co_creation_insight(user_id, &format!("FullAuto tool deployment for '{}'.", problem_statement));
                status
            },
            _ => {
                return Err(format!("{}. Tool deployment suggested (Autonomy {:?}). Confirm to proceed.", result_messages.join(" "), autonomy_level_deploy));
            }
        };

        Ok(format!("Co-creation complete: {}. Deployment: '{}'.", result_messages.join(" "), deployment_status))
    }

    pub fn integrate_domain_knowledge(&self, knowledge_area: &str) {
        println!("Integrating domain knowledge for co-creation: {}", knowledge_area);
        // Pull relevant KnowledgeItems for problem-solving
    }
}
