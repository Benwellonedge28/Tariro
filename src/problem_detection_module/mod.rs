use crate::prelude::*;
pub mod health_monitor;
pub mod data_integrity_checker;
pub mod troubleshooter;

pub struct ProblemDetectionEngine {
    monitor: health_monitor::HealthMonitor,
    data_checker: data_integrity_checker::DataIntegrityChecker,
    trouble_shooter: troubleshooter::Troubleshooter,
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
        // considering problem severity, impact, and user-specific overrides from LearningInsights.
        if action.contains("auto_fix") && user_prefs.contains("allow_auto_fix_minor") && context.contains("minor_issue") {
            AutonomyLevel::FullAuto
        } else if action.contains("system_change") && context.contains("critical_system") {
            AutonomyLevel::ConfirmOnly // Confirm changes on critical systems
        } else if action.contains("suggest_troubleshoot") {
            AutonomyLevel::SuggestOnly // Always suggest troubleshooting steps first
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
        // Fetches dynamic user preferences from LearningInsights entity, including problem resolution preferences
        "allow_auto_fix_minor, prefer_diagnosis_first, always_confirm_major_changes".to_string()
    }
    pub fn record_problem_insight(&self, user_id: &str, insight: &str) {
        println!("Recording problem detection insight for {}: {}", user_id, insight);
        // Persists new problem detection learning to LearningInsights entity
    }
}

impl ProblemDetectionEngine {
    pub fn new() -> Self {
        ProblemDetectionEngine {
            monitor: health_monitor::HealthMonitor::new(),
            data_checker: data_integrity_checker::DataIntegrityChecker::new(),
            trouble_shooter: troubleshooter::Troubleshooter::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn start_monitoring(&self, user_id: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "proactive_monitoring",
            "background_task",
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Starting proactive problem monitoring.");
                self.monitor.monitor_devices(user_id);
                self.data_checker.check_data_integrity(user_id);
                self.learning_agent.record_problem_insight(user_id, "FullAuto monitoring initiated.");
                "Proactive monitoring started in FullAuto mode.".to_string()
            },
            _ => {
                self.learning_agent.record_problem_insight(user_id, &format!("Monitoring initiated with autonomy {:?}", autonomy_level));
                format!("Monitoring started with autonomy {:?}. Some actions might require confirmation.", autonomy_level)
            }
        }
    }

    pub fn diagnose_issue(&self, user_id: &str, problem_description: &str, severity: &str) -> String {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            "diagnose_and_fix",
            severity,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Diagnosing and attempting to fix issue automatically.");
                let solution = self.trouble_shooter.diagnose_and_solve(user_id, problem_description);
                self.learning_agent.record_problem_insight(user_id, &format!("FullAuto diagnosis and fix for issue: '{}'.", problem_description));
                solution
            },
            AutonomyLevel::SuggestOnly => {
                let solution = self.trouble_shooter.diagnose_and_solve(user_id, problem_description);
                self.learning_agent.record_problem_insight(user_id, &format!("SuggestOnly diagnosis for issue: '{}'.", problem_description));
                format!("Diagnosis and suggested solution (Autonomy {:?}): '{}'. Confirm to apply.", autonomy_level, solution)
            },
            _ => {
                format!("Problem diagnosis and fix for '{}' blocked by autonomy policy: {:?}", problem_description, autonomy_level)
            }
        }
    }

    pub fn log_system_health_event(&self, component: &str, status: &str, details: String) {
        println!("Logging system health: Component: {}, Status: {}, Details: {}", component, status, details);
        // Call SystemHealthLog.create()
    }
}
