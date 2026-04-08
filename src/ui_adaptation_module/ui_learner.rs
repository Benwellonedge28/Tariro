use std::collections::HashMap;

pub struct UiLearner {
    user_preferences: HashMap<String, String>,
}

impl UiLearner {
    pub fn new() -> Self {
        UiLearner { user_preferences: HashMap::new() }
    }

    pub fn record_user_interaction(&mut self, user_id: &str, interaction_data: String) {
        println!("Recording interaction for user {}: {}", user_id, interaction_data);
        // Analyze interaction_data to update user_preferences
        // Example: if user frequently uses dark mode, store that preference
        self.user_preferences.insert(user_id.to_string(), "dark_mode_preferred".to_string());
    }

    pub fn get_user_preferences(&self, user_id: &str) -> String {
        self.user_preferences.get(user_id).cloned().unwrap_or("default_preferences".to_string())
    }
}
