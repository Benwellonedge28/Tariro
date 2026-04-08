pub mod health_monitor;
pub mod data_integrity_checker;
pub mod troubleshooter;

pub struct ProblemDetectionEngine {
    monitor: health_monitor::HealthMonitor,
    data_checker: data_integrity_checker::DataIntegrityChecker,
    trouble_shooter: troubleshooter::Troubleshooter,
}

impl ProblemDetectionEngine {
    pub fn new() -> Self {
        ProblemDetectionEngine {
            monitor: health_monitor::HealthMonitor::new(),
            data_checker: data_integrity_checker::DataIntegrityChecker::new(),
            trouble_shooter: troubleshooter::Troubleshooter::new(),
        }
    }

    pub fn start_monitoring(&self, user_id: &str) {
        println!("Starting proactive problem monitoring for user: {}", user_id);
        self.monitor.monitor_devices(user_id);
        self.data_checker.check_data_integrity(user_id);
    }

    pub fn diagnose_issue(&self, user_id: &str, problem_description: &str) -> String {
        self.trouble_shooter.diagnose_and_solve(user_id, problem_description)
    }

    // Integration with SystemHealthLog and SMACS
    pub fn log_system_health_event(&self, component: &str, status: &str, details: String) {
        println!("Logging system health: Component: {}, Status: {}, Details: {}", component, status, details);
        // Call SystemHealthLog.create()
    }
}
