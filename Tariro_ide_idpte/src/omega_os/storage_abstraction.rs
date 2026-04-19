// Tariro_ide_idpte/src/omega_os/storage_abstraction.rs

//! Provides a unified, abstracted interface to various underlying storage technologies.
//! This module hides the complexities of physical or distributed storage, offering a
//! consistent block-level or object-level access for higher-level OMEGA OS components.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::omega_os::data_models::{DataVolume, DataIntegrityStatus};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};

/// Represents a physical or logical storage unit that the abstraction layer manages.
#[derive(Debug, Clone)]
pub struct StorageUnit {
    pub id: String,
    pub name: String,
    pub total_capacity_bytes: u64,
    pub free_bytes: u64,
    pub type_name: String, // e.g., "NVMe SSD", "Network Share", "Quantum Storage", "Cloud Bucket"
    pub path: String, // Internal path or URI to access storage
    pub healthy: bool,
    pub read_iops: u32,
    pub write_iops: u32,
    pub last_checked: DateTime<Utc>,
}

/// The OMEGA OS Storage Abstraction Layer.
pub struct StorageAbstraction {
    storage_units: Mutex<HashMap<String, StorageUnit>>, // id -> StorageUnit
    event_bus: Arc<EventBus>,
}

impl StorageAbstraction {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        info!("OMEGA OS Storage Abstraction Layer initialized.");
        let mut units = HashMap::new();
        // Simulate a default local storage unit
        let local_storage = StorageUnit {
            id: "local-nvme-001".to_string(),
            name: "Primary Local NVMe SSD".to_string(),
            total_capacity_bytes: 1_000_000_000_000, // 1 TB
            free_bytes: 800_000_000_000, // 800 GB free
            type_name: "NVMe SSD".to_string(),
            path: "/mnt/local_storage".to_string(),
            healthy: true,
            read_iops: 100_000,
            write_iops: 80_000,
            last_checked: Utc::now(),
        };
        units.insert(local_storage.id.clone(), local_storage);
        StorageAbstraction { storage_units: Mutex::new(units), event_bus }
    }

    /// Registers a new physical or logical storage unit.
    pub async fn register_storage_unit(&self, unit: StorageUnit) -> Result<(), String> {
        let mut guard = self.storage_units.lock().unwrap();
        if guard.contains_key(&unit.id) {
            return Err(format!("Storage unit with ID {} already exists.", unit.id));
        }
        info!("StorageAbstraction: Registering storage unit '{}' (ID: {}).", unit.name, unit.id);
        guard.insert(unit.id.clone(), unit.clone());
        self.event_bus.publish(EventEnvelope::new("StorageAbstraction".to_string(), OsEvent::Custom("StorageUnitRegistered".to_string(), unit.id))).await;
        Ok(())
    }

    /// Reads data blocks from a specified storage unit and offset.
    pub async fn read_block(&self, unit_id: &str, offset: u64, size: usize) -> Result<Vec<u8>, String> {
        let guard = self.storage_units.lock().unwrap();
        if let Some(unit) = guard.get(unit_id) {
            if !unit.healthy { return Err(format!("Storage unit {} is unhealthy.", unit_id)); }
            // Simulate reading data
            info!("StorageAbstraction: Reading {} bytes from unit {} at offset {}.", size, unit_id, offset);
            let mock_data = vec![0xAB; size]; // Return mock data
            Ok(mock_data)
        } else {
            Err(format!("Storage unit {} not found.", unit_id))
        }
    }

    /// Writes data blocks to a specified storage unit and offset.
    pub async fn write_block(&self, unit_id: &str, offset: u64, data: &[u8]) -> Result<(), String> {
        let mut guard = self.storage_units.lock().unwrap();
        if let Some(unit) = guard.get_mut(unit_id) {
            if !unit.healthy { return Err(format!("Storage unit {} is unhealthy, cannot write.", unit_id)); }
            // Simulate writing data and updating free space
            info!("StorageAbstraction: Writing {} bytes to unit {} at offset {}.", data.len(), unit_id, offset);
            unit.free_bytes = unit.free_bytes.saturating_sub(data.len() as u64);
            unit.last_checked = Utc::now();
            self.event_bus.publish(EventEnvelope::new("StorageAbstraction".to_string(), OsEvent::Custom("StorageWrite".to_string(), unit_id.to_string()))).await;
            Ok(())
        } else {
            Err(format!("Storage unit {} not found.", unit_id))
        }
    }

    /// Retrieves metrics for a specific storage unit.
    pub async fn get_storage_metrics(&self, unit_id: &str) -> Option<StorageUnit> {
        self.storage_units.lock().unwrap().get(unit_id).cloned()
    }

    /// Registers the StorageAbstraction itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "os-storage-abstraction".to_string(),
            name: "OMEGA OS Storage Abstraction Layer".to_string(),
            description: "Provides a unified interface to various underlying storage technologies.".to_string(),
            service_type: ServiceType::Storage,
            capabilities: vec![
                ServiceCapability { name: "read_block".to_string(), description: "Reads data blocks from storage.".to_string(), parameters: HashMap::new(), output_schema: "bytes".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "write_block".to_string(), description: "Writes data blocks to storage.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["write_storage".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string()],
            managed_by_agi_module: None,
        }
    }
}
