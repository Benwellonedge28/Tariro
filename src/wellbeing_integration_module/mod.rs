pub mod wellbeing_monitor;
pub mod environment_controller;
pub mod digital_twin_simulator;

pub struct WellbeingIntegrationEngine {
    monitor: wellbeing_monitor::WellbeingMonitor,
    env_controller: environment_controller::EnvironmentController,
    twin_simulator: digital_twin_simulator::DigitalTwinSimulator,
}

impl WellbeingIntegrationEngine {
    pub fn new() -> Self {
        WellbeingIntegrationEngine {
            monitor: wellbeing_monitor::WellbeingMonitor::new(),
            env_controller: environment_controller::EnvironmentController::new(),
            twin_simulator: digital_twin_simulator::DigitalTwinSimulator::new(),
        }
    }

    pub fn support_wellbeing(&self, user_id: &str, health_data: &str) -> String {
        println!("Supporting wellbeing for user {} with data: {}", user_id, health_data);
        let insights = self.monitor.analyze_health_data(user_id, health_data);
        self.env_controller.adjust_environment_for_wellbeing(user_id, &insights);
        self.twin_simulator.simulate_scenario(user_id, "rest_optimization");
        format!("Wellbeing support applied. Insights: {}", insights)
    }

    // Integration with WellbeingLog for historical data and recommendations
    pub fn log_wellbeing_event(&self, user_id: &str, mood: &str, energy: f32) {
        println!("Logging wellbeing for user {}: Mood: {}, Energy: {}", user_id, mood, energy);
        // Call WellbeingLog.create()
    }
}
