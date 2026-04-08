pub mod scheduler_optimizer;
pub mod info_filter;
pub mod digital_hygiene_manager;

pub struct ResourceOptimizer {
    scheduler: scheduler_optimizer::SchedulerOptimizer,
    filter: info_filter::InfoFilter,
    hygiene_mgr: digital_hygiene_manager::DigitalHygieneManager,
}

impl ResourceOptimizer {
    pub fn new() -> Self {
        ResourceOptimizer {
            scheduler: scheduler_optimizer::SchedulerOptimizer::new(),
            filter: info_filter::InfoFilter::new(),
            hygiene_mgr: digital_hygiene_manager::DigitalHygieneManager::new(),
        }
    }

    pub fn optimize_schedule(&self, user_id: &str, current_tasks: Vec<String>) -> String {
        self.scheduler.dynamic_reschedule(user_id, current_tasks)
    }

    pub fn filter_incoming_info(&self, user_id: &str, info_stream: String) -> String {
        self.filter.process_stream(user_id, info_stream)
    }

    pub fn manage_digital_hygiene(&self, user_id: &str, path: &str) {
        self.hygiene_mgr.clean_digital_space(user_id, path)
    }

    // Integrates with LearningInsights and potentially external sensor data
    pub fn update_with_cognitive_state(&self, user_id: &str, state: &str) {
        println!("Updating resource optimizer with cognitive state for user {}: {}", user_id, state);
        // Adjust optimization strategies based on user's cognitive load/energy
    }
}
