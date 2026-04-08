pub mod wellbeing_monitor;
pub mod environment_controller;
pub mod digital_twin_simulator;

pub struct WellbeingIntegrationEngine {
    monitor: wellbeing_monitor::WellbeingMonitor,
    env_controller: environment_controller::EnvironmentController,
    twin_simulator: digital_twin_simulator::DigitalTwinSimulator,
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
        // considering well-being urgency, privacy implications, and user-specific overrides from LearningInsights.
        if action.contains("adjust_lighting") && user_prefs.contains("allow_auto_lighting_adjust") && context.contains("optimal_focus") {
            AutonomyLevel::FullAuto
        } else if action.contains("share_health_data") {
            AutonomyLevel::HumanRequired // Always require human for sharing health data
        } else if action.contains("suggest_wellness_activity") {
            AutonomyLevel::SuggestOnly // Suggest wellness activities for review
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
        // Fetches dynamic user preferences from LearningInsights entity, including well-being preferences
        "allow_auto_lighting_adjust, prefer_quiet_for_focus, interested_in_stress_reduction_techniques".to_string()
    }
    pub fn record_wellbeing_insight(&self, user_id: &str, insight: &str) {
        println!("Recording well-being insight for {}: {}", user_id, insight);
        // Persists new well-being learning to LearningInsights entity
    }
}

impl WellbeingIntegrationEngine {
    pub fn new() -> Self {
        WellbeingIntegrationEngine {
            monitor: wellbeing_monitor::WellbeingMonitor::new(),
            env_controller: environment_controller::EnvironmentController::new(),
            twin_simulator: digital_twin_simulator::DigitalTwinSimulator::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn support_wellbeing(&self, user_id: &str, health_data: &str, current_state: &str) -> String {
        println!("Supporting wellbeing for user {} with data: {}", user_id, health_data);
        let user_prefs = self.learning_agent.get_user_preferences(user_id);

        let autonomy_level_env = self.autonomy_agent.determine_autonomy_level(
            "adjust_environment",
            current_state,
            &user_prefs
        );
        let autonomy_level_wellness_suggestion = self.autonomy_agent.determine_autonomy_level(
            "suggest_wellness_activity",
            current_state,
            &user_prefs
        );

        let mut result_messages = Vec::new();

        let insights = self.monitor.analyze_health_data(user_id, health_data);
        result_messages.push(format!("Health insights: '{}'.", insights));

        match autonomy_level_env {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Adjusting environment for wellbeing.");
                self.env_controller.adjust_environment_for_wellbeing(user_id, &insights);
                self.learning_agent.record_wellbeing_insight(user_id, &format!("FullAuto environment adjustment for state '{}'.", current_state));
                result_messages.push("Environment adjusted automatically.".to_string());
            },
            AutonomyLevel::SuggestOnly => {
                result_messages.push(format!("Environment adjustment suggested (Autonomy {:?}).", autonomy_level_env));
            },
            _ => result_messages.push(format!("Environment adjustment blocked by autonomy policy: {:?}.", autonomy_level_env)),
        }

        match autonomy_level_wellness_suggestion {
            AutonomyLevel::FullAuto | AutonomyLevel::SuggestOnly => {
                self.monitor.provide_nudges(user_id, &insights);
                self.learning_agent.record_wellbeing_insight(user_id, &format!("Wellness nudges provided/suggested for state '{}'.", current_state));
                result_messages.push(format!("Wellness nudges provided/suggested (Autonomy {:?}).", autonomy_level_wellness_suggestion));
            },
            _ => result_messages.push(format!("Wellness nudges blocked by autonomy policy: {:?}.", autonomy_level_wellness_suggestion)),
        }

        self.twin_simulator.simulate_scenario(user_id, "rest_optimization");
        result_messages.push("Digital Twin simulation run for rest optimization.".to_string());

        result_messages.join(" ")
    }

    pub fn log_wellbeing_event(&self, user_id: &str, mood: &str, energy: f32) {
        println!("Logging wellbeing for user {}: Mood: {}, Energy: {}", user_id, mood, energy);
        // Call WellbeingLog.create()
    }
}
