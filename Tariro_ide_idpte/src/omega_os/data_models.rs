// Tariro_ide_idpte/src/omega_os/data_models.rs

//! Defines data models for OMEGA OS persistent storage, volumes, files, and encryption policies.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Represents a logical data volume within OMEGA OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataVolume {
    pub id: String,
    pub name: String,
    pub capacity_bytes: u64,
    pub used_bytes: u64,
    pub mount_point: String,
    pub filesystem_type: String, // e.g., "OmegaFS", "QuantumFS", "Ext4"
    pub encryption_policy: EncryptionPolicy,
    pub integrity_status: DataIntegrityStatus,
    pub created_at: DateTime<Utc>,
    pub last_checked: DateTime<Utc>,
    pub associated_storage_ids: Vec<String>, // IDs of underlying physical/logical storage units
}

/// Represents metadata for a file or data object within OMEGA OS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub owner_id: String,
    pub permissions: String, // e.g., "rwx", "r--"
    pub encryption_status: EncryptionStatus,
    pub integrity_checksum: String,
    pub version: u32,
    pub tags: Vec<String>,
}

/// Defines the encryption policy for a data volume or file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionPolicy {
    Aes256Gcm,
    ChaCha20Poly1305,
    QuantumResistantPqc(String), // e.g., "Kyber-AES", "FrodoKEM-ChaCha"
    NoEncryption,
    Custom(String),
}

/// Current encryption status of a file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionStatus {
    Encrypted,
    Decrypted,
    PartiallyEncrypted,
    Error,
}

/// Status of data integrity checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataIntegrityStatus {
    Verified,
    Compromised,
    Checking,
    Unknown,
}
