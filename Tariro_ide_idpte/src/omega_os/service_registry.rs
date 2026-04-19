// Tariro_ide_idpte/src/omega_os/service_registry.rs

//! Manages the registration, discovery, and lifecycle of all OMEGA OS internal services.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::Utc;
use tracing::{info, warn, error};

use crate::omega_os::service_models::{OsService, ServiceCapability, ServiceStatus, ServiceType};

/// The ServiceRegistry acts as a central directory for all OMEGA OS components.
pub struct ServiceRegistry {
    services: Mutex<HashMap<String, OsService>>, // service_id -> OsService
}

impl ServiceRegistry {
    pub fn new() -> Self {
        info!("OMEGA OS Service Registry initialized.");
        ServiceRegistry { services: Mutex::new(HashMap::new()) }
    }

    /// Registers a new service with the registry.
    pub async fn register_service(&self, mut service: OsService) -> Result<(), String> {
        let mut services_guard = self.services.lock().unwrap();
        if services_guard.contains_key(&service.id) {
            return Err(format!("Service with ID {} already registered.", service.id));
        }
        service.registered_at = Utc::now();
        service.last_heartbeat = Utc::now();
        services_guard.insert(service.id.clone(), service);
        info!("Service '{}' (ID: {}) registered.", services_guard[&service.id].name, service.id);
        Ok(())
    }

    /// Discovers services based on type, capability, or other criteria.
    pub async fn discover_services(
        &self,
        service_type: Option<&ServiceType>,
        capability_name: Option<&str>,
    ) -> Vec<OsService> {
        let services_guard = self.services.lock().unwrap();
        services_guard.values()
            .filter(|service| {
                let mut matches = true;
                if let Some(st) = service_type { matches &= &service.service_type == st; }
                if let Some(cn) = capability_name { matches &= service.capabilities.iter().any(|cap| cap.name == cn); }
                matches
            })
            .cloned()
            .collect()
    }

    /// Updates the status or heartbeat of an existing service.
    pub async fn update_service_status(&self, service_id: &str, new_status: Option<ServiceStatus>) -> Result<(), String> {
        let mut services_guard = self.services.lock().unwrap();
        if let Some(service) = services_guard.get_mut(service_id) {
            service.last_heartbeat = Utc::now();
            if let Some(status) = new_status { service.status = status; }
            info!("Service '{}' (ID: {}) status updated to {:?}.", service.name, service.id, service.status);
            Ok(())
        } else {
            Err(format!("Service with ID {} not found.", service_id))
        }
    }

    /// Removes a service from the registry.
    pub async fn unregister_service(&self, service_id: &str) -> Result<(), String> {
        let mut services_guard = self.services.lock().unwrap();
        if services_guard.remove(service_id).is_some() {
            info!("Service ID {} unregistered.", service_id);
            Ok(())
        } else {
            Err(format!("Service with ID {} not found.", service_id))
        }
    }
}
