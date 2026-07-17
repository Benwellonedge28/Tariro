// src/qrfs.rs

//! Tariro's Quantum-Resistant File System (QRFS) module.
//! This module provides an abstraction layer for storing and retrieving files
//! with autonomous post-quantum cryptography (PQC) protection.

use crate::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::{self, File};
use uuid::Uuid;
use crate::quantum_security::mod::QuantumSecurityManager;
use crate::quantum_security::mod::prelude::*; 
use crate::config::AGIConfig;
use crate::telemetry::{TelemetryLogger, TelemetryEvent};

/// Metadata for a quantum-resistant file, stored alongside encrypted content.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QrfsMetadata {
    pub file_id: String,
    pub original_filename: String,
    pub mime_type: String,
    pub encrypted_size_bytes: u64,
    pub encryption_algo_name: String, 
    pub creation_date: DateTime<Utc>,
    pub last_modified_date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    // For advanced features: PQC signature, data fragmentation info, version history
}

/// The Quantum-Resistant File System service.
/// It provides methods for autonomously storing, retrieving, and managing PQC-secured files.
pub struct QrfsService {
    config: Arc<AGIConfig>,
    telemetry_logger: TelemetryLogger,
    qsm: Arc<Mutex<QuantumSecurityManager>>,
    storage_base_path: String, 
}

impl QrfsService {
    /// Creates a new `QrfsService` instance.
    pub fn new(
        config: Arc<AGIConfig>,
        telemetry_logger: TelemetryLogger,
        qsm: Arc<Mutex<QuantumSecurityManager>>,
        storage_base_path: String,
    ) -> Self {
        QrfsService {
            config,
            telemetry_logger,
            qsm,
            storage_base_path,
        }
    }

    /// Initializes the QRFS service, ensuring the storage path exists.
    pub async fn initialize(&self) -> Result<()> {
        info!("[QRFS Service] Initializing...");
        fs::create_dir_all(&self.storage_base_path).await
            .map_err(|e| format!("Failed to create QRFS storage path '{}': {}", self.storage_base_path, e))?;
        info!("[QRFS Service] Storage path '{}' ensured.", self.storage_base_path);
        self.telemetry_logger.log_event(TelemetryEvent::ServiceInitialized { service_name: "QrfsService".to_string() }).await;
        Ok(())
    }

    /// Autonomously stores a file with PQC encryption.
    /// Returns the QrfsMetadata, which effectively acts as a handle to the encrypted file.
    pub async fn store_file(
        &self,
        original_filename: String,
        mime_type: String,
        plaintext_content: Vec<u8>,
        tags: Vec<String>,
        summary: Option<String>,
    ) -> Result<QrfsMetadata> {
        let file_id = Uuid::new_v4().to_string();
        let qsm_locked = self.qsm.lock().await;

        info!("[QRFS Service] Autonomously encrypting and storing file: '{}' (ID: {})", original_filename, file_id);

        let encrypted_content = qsm_locked.protect_data(
            &plaintext_content,
            &format!("QRFS_file_{}", file_id),
        ).await?;

        let storage_path = format!("{}/{}.enc", self.storage_base_path, file_id);
        let mut file = File::create(&storage_path).await
            .map_err(|e| format!("Failed to create encrypted file at '{}': {}", storage_path, e))?;
        file.write_all(&encrypted_content).await
            .map_err(|e| format!("Failed to write encrypted content to '{}': {}", storage_path, e))?;

        let metadata = QrfsMetadata {
            file_id: file_id.clone(),
            original_filename,
            mime_type,
            encrypted_size_bytes: encrypted_content.len() as u64,
            encryption_algo_name: qsm_locked.active_cipher.name().to_string(), 
            creation_date: Utc::now(),
            last_modified_date: Utc::now(),
            tags,
            summary,
        };

        // Store metadata alongside the file (or in a database). For simplicity, let's also write it to disk.
        let metadata_path = format!("{}/{}.meta", self.storage_base_path, file_id);
        fs::write(&metadata_path, serde_json::to_string(&metadata)?)
            .await
            .map_err(|e| format!("Failed to write QRFS metadata to '{}': {}", metadata_path, e))?;


        info!("[QRFS Service] File '{}' (ID: {}) stored and PQC-encrypted successfully.", metadata.original_filename, file_id);
        self.telemetry_logger.log_event(TelemetryEvent::FileStored {
            file_id: file_id,
            filename: metadata.original_filename.clone(),
            size_bytes: plaintext_content.len() as u64,
            encrypted: true,
            pqc_algo: metadata.encryption_algo_name.clone(),
        }).await;

        Ok(metadata)
    }

    /// Autonomously retrieves and decrypts a file using its metadata.
    pub async fn retrieve_file(&self, metadata: &QrfsMetadata) -> Result<Vec<u8>> {
        let storage_path = format!("{}/{}.enc", self.storage_base_path, metadata.file_id);
        let mut file = File::open(&storage_path).await
            .map_err(|e| format!("Failed to open encrypted file at '{}': {}", storage_path, e))?;
        let mut encrypted_content = Vec::new();
        file.read_to_end(&mut encrypted_content).await
            .map_err(|e| format!("Failed to read encrypted content from '{}': {}", storage_path, e))?;

        let qsm_locked = self.qsm.lock().await;

        info!("[QRFS Service] Autonomously decrypting file: '{}' (ID: {})", metadata.original_filename, metadata.file_id);
        let plaintext_content = qsm_locked.unprotect_data(
            &encrypted_content,
            &format!("QRFS_file_{}", metadata.file_id),
        ).await?;

        info!("[QRFS Service] File '{}' (ID: {}) decrypted successfully.", metadata.original_filename, metadata.file_id);
        self.telemetry_logger.log_event(TelemetryEvent::FileRetrieved {
            file_id: metadata.file_id.clone(),
            filename: metadata.original_filename.clone(),
            size_bytes: plaintext_content.len() as u64,
            decrypted: true,
        }).await;

        Ok(plaintext_content)
    }

    /// Retrieves metadata for a stored file.
    pub async fn get_file_metadata(&self, file_id: &str) -> Result<QrfsMetadata> {
        let metadata_path = format!("{}/{}.meta", self.storage_base_path, file_id);
        let metadata_str = fs::read_to_string(&metadata_path).await
            .map_err(|e| format!("Failed to read QRFS metadata for file '{}': {}", file_id, e))?;
        serde_json::from_str(&metadata_str)
            .map_err(|e| format!("Failed to parse QRFS metadata for file '{}': {}", file_id, e).into())
    }

    /// Deletes a quantum-resistant file and its metadata.
    pub async fn delete_file(&self, file_id: &str) -> Result<()> {
        info!("[QRFS Service] Deleting QRFS file (ID: {}).");
        let storage_path = format!("{}/{}.enc", self.storage_base_path, file_id);
        let metadata_path = format!("{}/{}.meta", self.storage_base_path, file_id);

        fs::remove_file(&storage_path).await
            .map_err(|e| format!("Failed to delete encrypted file '{}': {}", storage_path, e))?;
        fs::remove_file(&metadata_path).await
            .map_err(|e| format!("Failed to delete metadata file '{}': {}", metadata_path, e))?;

        info!("[QRFS Service] QRFS file (ID: {}) deleted successfully.", file_id);
        self.telemetry_logger.log_event(TelemetryEvent::FileDeleted {
            file_id: file_id.to_string(),
        }).await;
        Ok(())
    }

    // For infinitely advanced:
    // - Autonomous file integrity checks (PQC signatures)
    // - Self-healing data redundancy across different storage mediums (QFSN - Quantum File Storage Network)
    // - Versioning with PQC-secured deltas
}
