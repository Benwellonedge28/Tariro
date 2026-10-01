use crate::prelude::*;
pub mod service_manager;
pub mod api_connector;
pub mod auth_handler;

pub struct ExternalIntegrationFramework {
    service_mgr: service_manager::ServiceManager,
    api_conn: api_connector::ApiConnector,
    auth_hdlr: auth_handler::AuthHandler,
    autonomy_agent: AutonomyAgent,
    learning_agent: LearningAgent,
}

// Re-using conceptual structs for Autonomy and Learning from TariroContextManager
pub struct AutonomyAgent;
impl AutonomyAgent {
    pub fn new() -> Self { AutonomyAgent {} }
    pub fn determine_autonomy_level(&self, action: &str, context: &str, user_prefs: &str) -> AutonomyLevel {
        println!("Determining autonomy for '{}' in context '{}' with prefs '{}'", action, context, user_prefs);
        // In a real system, this would query the AutonomyPolicy entity,
        // considering service-specific policies, data sensitivity, and user-specific overrides from LearningInsights.
        if action.contains("sensitive_data") && !user_prefs.contains("allow_external_data_sharing") {
            AutonomyLevel::HumanRequired // Always require human for sensitive data if not explicitly allowed
        } else if context.contains("emergency") {
            AutonomyLevel::FullAuto
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
        // Fetches dynamic user preferences from LearningInsights entity, including service integration preferences
        "allow_external_data_sharing, prefer_fast_response_over_privacy_for_non_sensitive_data".to_string()
    }
    pub fn record_integration_insight(&self, user_id: &str, insight: &str) {
        println!("Recording integration insight for {}: {}", user_id, insight);
        // Persists new integration learning to LearningInsights entity
    }
}

impl ExternalIntegrationFramework {
    pub fn new() -> Self {
        ExternalIntegrationFramework {
            service_mgr: service_manager::ServiceManager::new(),
            api_conn: api_connector::ApiConnector::new(),
            auth_hdlr: auth_handler::AuthHandler::new(),
            autonomy_agent: AutonomyAgent::new(),
            learning_agent: LearningAgent::new(),
        }
    }

    pub fn connect_service(&self, user_id: &str, service_name: &str, config: String, context: &str) -> Result<String, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            &format!("connect_{}", service_name),
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Connecting service {} automatically.", service_name);
                let result = self.service_mgr.register_service(service_name, config);
                self.learning_agent.record_integration_insight(user_id, &format!("Attempted connection for {} with FullAuto autonomy: {:?}", service_name, result));
                result
            },
            AutonomyLevel::SuggestOnly => {
                println!("SuggestOnly: Proposing service connection for {}. Awaiting confirmation.", service_name);
                Err(format!("Suggestion: Connect service {}. Confirm to proceed.", service_name))
            },
            AutonomyLevel::ConfirmOnly => {
                println!("ConfirmOnly: Requiring explicit user confirmation for service {}.", service_name);
                Err(format!("Action Blocked: Connect service {}. Requires confirmation.", service_name))
            },
            AutonomyLevel::HumanRequired => {
                println!("HumanRequired: Manual intervention needed for service {}.", service_name);
                Err(format!("Action Blocked: Connect service {}. Human review required.", service_name))
            },
        }
    }

    pub fn call_service_api(&self, user_id: &str, service_name: &str, endpoint: &str, payload: String, context: &str) -> Result<String, String> {
        let user_prefs = self.learning_agent.get_user_preferences(user_id);
        let autonomy_level = self.autonomy_agent.determine_autonomy_level(
            &format!("call_{}_api", service_name),
            context,
            &user_prefs
        );

        match autonomy_level {
            AutonomyLevel::FullAuto => {
                println!("FullAuto: Calling API for {} automatically.", service_name);
                let service_config = self.service_mgr.get_service_config(service_name)?;
                let auth_token = self.auth_hdlr.get_token(service_name)?;
                let result = self.api_conn.make_request(&service_config, endpoint, &payload, &auth_token);
                self.learning_agent.record_integration_insight(user_id, &format!("API call to {} with FullAuto autonomy: {:?}", service_name, result));
                result
            },
            _ => {
                println!("API call for {} blocked by autonomy policy.", service_name);
                Err(format!("Action Blocked: API call for {}. Autonomy level: {:?}", service_name, autonomy_level))
            },
        }
    }

    pub fn update_autonomy_policy(&self, service_name: &str, policy: &str) {
        println!("Updating AutonomyPolicy for service {}: {}", service_name, policy);
        // This would interact with the AutonomyPolicy entity
    }
}
