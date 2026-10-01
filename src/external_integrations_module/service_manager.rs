use crate::prelude::*;
use std::collections::HashMap;

pub struct ServiceManager {
    service_configs: HashMap<String, String>,
}

impl ServiceManager {
    pub fn new() -> Self {
        ServiceManager {
            service_configs: HashMap::new(),
        }
    }

    pub fn register_service(&mut self, name: &str, config: String) -> Result<String, String> {
        if self.service_configs.contains_key(name) {
            return Err(format!("Service {} already registered.", name));
        }
        self.service_configs.insert(name.to_string(), config);
        Ok(format!("Service {} registered successfully.", name))
    }

    pub fn get_service_config(&self, name: &str) -> Result<String, String> {
        self.service_configs.get(name).cloned().ok_or(format!("Service {} not found.", name))
    }

    pub fn unregister_service(&mut self, name: &str) -> Result<String, String> {
        if self.service_configs.remove(name).is_some() {
            Ok(format!("Service {} unregistered.", name))
        } else {
            Err(format!("Service {} not found.", name))
        }
    }
}
