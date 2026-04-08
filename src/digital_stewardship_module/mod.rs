pub mod data_governor;
pub mod digital_nudge_system;
pub mod privacy_guardian;

pub struct DigitalStewardshipEngine {
    governor: data_governor::DataGovernor,
    nudge_sys: digital_nudge_system::DigitalNudgeSystem,
    privacy_guard: privacy_guardian::PrivacyGuardian,
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
        // considering policy criticality, data sensitivity, and user-specific overrides from LearningInsights.
        if action.contains("auto_data_retention") && user_prefs.contains("allow_auto_data_management") && context.contains("non_critical_data") {
            AutonomyLevel::FullAuto
        } else if action.contains("access_sensitive_data") {
            AutonomyLevel::HumanRequired // Always require human for accessing sensitive data
        } else if action.contains("privacy_nudge") && user_prefs.contains("prefer_gentle_nudges") {
            AutonomyLevel::SuggestOnly // Suggest nudges gently for review
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
        // Fetches dynamic user preferences from LearningInsights entity, including stewardship preferences
        "allow_auto_data_management, prefer_strong_privacy_defaults, responsive_to_nudges".to_string()
    }
    pub fn record_stewardship_insight(&self, user_id: &str, insight: &str) {
        println!("Recording digital stewardship insight for {}: {}", user_id, insight);
        // Persists new digital stewardship learning to LearningInsights entity
    }
}

impl DigitalStewardshipEngine {
    pub fn new() -> Self {
        DigitalStewardshipEngine {
            governor: data_governor::DataGovernor::new(),
            nudge_sys: digital_nudge_system::DigitalNudgeSystem::new(),
            privacy_guard: privacy_guardian::PrivacyGuardian::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn enforce_personal_policies(&self, user_id: &str, current_activity: &str, policy_criticality: &str) -> String {
        println!("Enforcing personal policies for user {} during activity: {}", user_id, current_activity);
        let user_prefs = self.learning_agent.get_user_preferences(user_id);

        let autonomy_level_governor = self.autonomy_agent.determine_autonomy_level(
            "auto_data_retention",
            policy_criticality,
            &user_prefs
        );
        let autonomy_level_nudge = self.autonomy_agent.determine_autonomy_level(
            "privacy_nudge",
            current_activity,
            &user_prefs
        );
        let autonomy_level_privacy_monitor = self.autonomy_agent.determine_autonomy_level(
            "monitor_app_permissions",
            "background_task",
            &user_prefs
        );

        let mut result_messages = Vec::new();

        match autonomy_level_governor {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Applying data governance policies.");
                self.governor.apply_data_policies(user_id);
                self.learning_agent.record_stewardship_insight(user_id, &format!("FullAuto data governance applied. Criticality: '{}'.", policy_criticality));
                result_messages.push("Data governance applied automatically.".to_string());
            },
            _ => result_messages.push(format!("Data governance ({:?}) skipped or suggested.", autonomy_level_governor)),
        }

        match autonomy_level_nudge {
            AutonomyLevel::FullAuto | AutonomyLevel::SuggestOnly => {
                self.nudge_sys.provide_contextual_nudge(user_id, current_activity);
                self.learning_agent.record_stewardship_insight(user_id, &format!("Digital nudge provided/suggested for activity '{}'.", current_activity));
                result_messages.push(format!("Digital nudges provided/suggested (Autonomy {:?}).", autonomy_level_nudge));
            },
            _ => result_messages.push(format!("Digital nudges blocked by autonomy policy: {:?}.", autonomy_level_nudge)),
        }

        match autonomy_level_privacy_monitor {
            AutonomyLevel::FullAuto => {
                self.privacy_guard.monitor_app_permissions(user_id);
                self.learning_agent.record_stewardship_insight(user_id, "FullAuto privacy monitoring.");
                result_messages.push("App permissions monitored automatically.".to_string());
            },
            _ => result_messages.push(format!("App permissions monitoring ({:?}) skipped or suggested.", autonomy_level_privacy_monitor)),
        }

        result_messages.join(" ")
    }

    pub fn review_policy_compliance(&self, user_id: &str) -> String {
        println!("Reviewing policy compliance for user: {}", user_id);
        // Check against user's defined AutonomyPolicy and E.V.A.S. rules
        "Compliance check complete: All data governance policies are being followed. (Simulated)".to_string()
    }
}
