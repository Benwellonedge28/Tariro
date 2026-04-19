// Tariro_ide_idpte/src/omega_os/data_manager.rs

//! Manages persistent data storage, including volumes, file systems, and dynamic encryption.
//! This module provides a high-level interface for reading, writing, and managing files
//! and data objects within the OMEGA OS, ensuring integrity and security.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::omega_os::data_models::{DataVolume, FileMetadata, EncryptionPolicy, EncryptionStatus, DataIntegrityStatus};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};

use crate::omega_os::storage_abstraction::StorageAbstraction; // Import StorageAbstraction
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // Import QR-CTFO Orchestrator
use crate::crypto_fabric::communication_adapter::CommunicationChannel; // For QR-CTFO crypto

/// The OMEGA OS Data Manager.
pub struct DataManager {
    volumes: Mutex<HashMap<String, DataVolume>>, // id -> DataVolume
    files: Mutex<HashMap<String, FileMetadata>>, // id -> FileMetadata (or path -> FileMetadata)
    storage_abstraction: Arc<StorageAbstraction>,
    qr_ctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    event_bus: Arc<EventBus>,
}

impl DataManager {
    pub fn new(
        storage_abstraction: Arc<StorageAbstraction>,
        qr_ctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        info!("OMEGA OS Data Manager initialized.");
        DataManager {
            volumes: Mutex::new(HashMap::new()),
            files: Mutex::new(HashMap::new()),
            storage_abstraction,
            qr_ctfo_orchestrator,
            event_bus,
        }
    }

    /// Creates a new logical data volume.
    pub async fn create_volume(
        &self,
        name: &str,
        capacity_bytes: u64,
        filesystem_type: &str,
        encryption_policy: EncryptionPolicy,
        associated_storage_unit_ids: Vec<String>,
    ) -> Result<String, String> {
        let volume_id = format!("vol-{}", Uuid::new_v4());
        info!("DataManager: Creating new volume '{}' (ID: {}).", name, volume_id);

        // Simulate allocating space on underlying storage units
        // In a real system, this would involve complex storage pool management.
        for unit_id in &associated_storage_unit_ids {
            if self.storage_abstraction.get_storage_metrics(unit_id).await.is_none() {
                return Err(format!("Associated storage unit {} not found.", unit_id));
            }
            // Simulate reservation/allocation of space
        }

        let new_volume = DataVolume {
            id: volume_id.clone(),
            name: name.to_string(),
            capacity_bytes,
            used_bytes: 0,
            mount_point: format!("/volumes/{}", name),
            filesystem_type: filesystem_type.to_string(),
            encryption_policy: encryption_policy.clone(),
            integrity_status: DataIntegrityStatus::Verified,
            created_at: Utc::now(),
            last_checked: Utc::now(),
            associated_storage_ids: associated_storage_unit_ids,
        };

        self.volumes.lock().unwrap().insert(volume_id.clone(), new_volume.clone());
        self.event_bus.publish(EventEnvelope::new("DataManager".to_string(), OsEvent::Custom("DataVolumeCreated".to_string(), volume_id.clone()))).await;
        Ok(volume_id)
    }

    /// Reads data from a specified file path.
    pub async fn read_file(&self, file_path: &str) -> Result<Vec<u8>, String> {
        // Find file metadata
        let files_guard = self.files.lock().unwrap();
        let file_meta = files_guard.values().find(|fm| fm.path == file_path).cloned()
            .ok_or_else(|| format!("File not found: {}.", file_path))?;
        drop(files_guard);

        info!("DataManager: Reading file '{}' (ID: {}).", file_meta.name, file_meta.id);

        // Simulate reading from storage abstraction (block by block)
        let raw_data = self.storage_abstraction.read_block("local-nvme-001", 0, file_meta.size_bytes as usize).await?;

        // Decrypt data using QR-CTFO if necessary
        if file_meta.encryption_status == EncryptionStatus::Encrypted {
            let qr_ctfo_guard = self.qr_ctfo_orchestrator.lock().unwrap();
            let comm_channel_mock = CommunicationChannel::Other(file_meta.id.clone()); // Mock channel for crypto context
            match qr_ctfo_guard.comm_adapter.decrypt_data(&comm_channel_mock, &raw_data).await {
                Ok(decrypted) => {
                    info!("DataManager: File '{}' decrypted.", file_meta.name);
                    self.event_bus.publish(EventEnvelope::new("DataManager".to_string(), OsEvent::Custom("FileReadDecrypted".to_string(), file_meta.id.clone()))).await;
                    Ok(decrypted)
                },
                Err(e) => Err(format!("Failed to decrypt file {}: {}", file_meta.name, e)),
            }
        } else {
            self.event_bus.publish(EventEnvelope::new("DataManager".to_string(), OsEvent::Custom("FileRead".to_string(), file_meta.id.clone()))).await;
            Ok(raw_data)
        }
    }

    /// Writes data to a specified file path, encrypting it if required by policy.
    pub async fn write_file(&self, file_path: &str, data: &[u8], encryption_policy: EncryptionPolicy) -> Result<String, String> {
        let file_id = format!("file-{}", Uuid::new_v4());
        info!("DataManager: Writing new file '{}' (ID: {}).", file_path, file_id);

        let mut data_to_write = data.to_vec();
        let encryption_status = match encryption_policy {
            EncryptionPolicy::NoEncryption => EncryptionStatus::Decrypted,
            _ => {
                let qr_ctfo_guard = self.qr_ctfo_orchestrator.lock().unwrap();
                let comm_channel_mock = CommunicationChannel::Other(file_id.clone()); // Mock channel for crypto context
                match qr_ctfo_guard.comm_adapter.encrypt_data(&comm_channel_mock, data).await {
                    Ok(encrypted) => {
                        data_to_write = encrypted;
                        EncryptionStatus::Encrypted
                    },
                    Err(e) => { 
                        error!("DataManager: Failed to encrypt data for file {}: {}", file_path, e);
                        return Err(format!("Encryption failed: {}", e));
                    },
                }
            }
        };

        // Simulate writing to storage abstraction
        self.storage_abstraction.write_block("local-nvme-001", 0, &data_to_write).await?;

        let new_file_meta = FileMetadata {
            id: file_id.clone(),
            name: file_path.split('/').last().unwrap_or("unnamed").to_string(),
            path: file_path.to_string(),
            size_bytes: data.len() as u64,
            created_at: Utc::now(),
            last_modified: Utc::now(),
            owner_id: "system".to_string(), // Placeholder
            permissions: "rwx".to_string(),
            encryption_status,
            integrity_checksum: "mock-checksum".to_string(),
            version: 1,
            tags: vec![],
        };

        self.files.lock().unwrap().insert(file_id.clone(), new_file_meta.clone());
        self.event_bus.publish(EventEnvelope::new("DataManager".to_string(), OsEvent::Custom("FileWritten".to_string(), file_id.clone()))).await;
        Ok(file_id)
    }

    /// Registers the DataManager itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "os-data-manager".to_string(),
            name: "OMEGA OS Data Manager".to_string(),
            description: "Manages persistent data storage, volumes, and files with encryption.".to_string(),
            service_type: ServiceType::Storage,
            capabilities: vec![
                ServiceCapability { name: "create_volume".to_string(), description: "Creates a new logical data volume.".to_string(), parameters: HashMap::new(), output_schema: "VolumeId".to_string(), requires_permissions: vec!["root".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "read_file".to_string(), description: "Reads data from a file.".to_string(), parameters: HashMap::new(), output_schema: "bytes".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "write_file".to_string(), description: "Writes data to a file.".to_string(), parameters: HashMap::new(), output_schema: "FileId".to_string(), requires_permissions: vec!["write_storage".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "os-storage-abstraction".to_string(), "qr-ctfo".to_string()],
            managed_by_agi_module: None,
        }
    }
}
