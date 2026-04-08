pub mod service_manager;
pub mod api_connector;
pub mod auth_handler;

pub struct ExternalIntegrationFramework {
    service_mgr: service_manager::ServiceManager,
    api_conn: api_connector::ApiConnector,
    auth_hdlr: auth_handler::AuthHandler,
}

impl ExternalIntegrationFramework {
    pub fn new() -> Self {
        ExternalIntegrationFramework {
            service_mgr: service_manager::ServiceManager::new(),
            api_conn: api_connector::ApiConnector::new(),
            auth_hdlr: auth_handler::AuthHandler::new(),
        }
    }

    pub fn connect_service(&self, service_name: &str, config: String) -> Result<String, String> {
        if self.auth_hdlr.check_permission(service_name, "connect") {
            self.service_mgr.register_service(service_name, config)
        } else {
            Err(format!("Permission denied to connect service: {}", service_name))
        }
    }

    pub fn call_service_api(&self, service_name: &str, endpoint: &str, payload: String) -> Result<String, String> {
        if self.auth_hdlr.check_permission(service_name, "call_api") {
            let service_config = self.service_mgr.get_service_config(service_name)?;
            let auth_token = self.auth_hdlr.get_token(service_name)?;
            self.api_conn.make_request(&service_config, endpoint, &payload, &auth_token)
        } else {
            Err(format!("Permission denied to call API for service: {}", service_name))
        }
    }

    // Example of how AutonomyPolicy would govern this
    pub fn update_autonomy_policy(&self, service_name: &str, policy: &str) {
        println!("Updating AutonomyPolicy for service {}: {}", service_name, policy);
        // This would interact with the AutonomyPolicy entity
    }
}
