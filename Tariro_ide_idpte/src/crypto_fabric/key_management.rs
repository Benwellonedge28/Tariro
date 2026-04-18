// Tariro_ide_idpte/src/crypto_fabric/key_management.rs

//! Manages the lifecycle of Quantum-Resistant (QR) cryptographic keys.
//! This includes generation, secure storage, rotation, and revocation of QR keys.

use crate::crypto_fabric::qr_algorithms::{QrKeyExchangeAlg, QrSignatureAlg, QrAlgorithmType};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use anyhow::Result;
use async_trait::async_trait;
use tracing::{info, warn, error};

/// Represents a QR public/private key pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrKeyPair {
    pub id: String,
    pub algorithm_type: QrAlgorithmType, // Store the full algorithm type
    pub public_key: Vec<u8>,
    #[serde(skip_serializing)] // Don't serialize private key for security
    pub private_key: Vec<u8>,
    pub generated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub usage: QrKeyUsage,
}

/// Specifies the intended use of a QR key pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QrKeyUsage {
    KeyExchange,
    DigitalSignature,
    Encryption,
    RootOfTrust,
    Other(String),
}

/// Trait for abstracting secure key storage mechanisms.
#[async_trait]
pub trait SecureKeyStorage: Send + Sync {
    /// Stores a key pair securely.
    async fn store_key(&self, key_pair: &QrKeyPair) -> Result<(), String>;
    /// Retrieves a key pair by its ID.
    async fn retrieve_key(&self, id: &str) -> Result<Option<QrKeyPair>, String>;
    /// Deletes a key pair by its ID.
    async fn delete_key(&self, id: &str) -> Result<(), String>;
    /// Lists all stored key IDs.
    async fn list_key_ids(&self) -> Result<Vec<String>, String>;
}

/// Mock implementation of SecureKeyStorage using in-memory Vec for demonstration.
/// In a real system, this would be an interface to an HSM, TPM, or encrypted file system.
pub struct MockSecureMemoryStorage {
    storage: Mutex<HashMap<String, QrKeyPair>>,
}

impl MockSecureMemoryStorage {
    pub fn new() -> Self {
        MockSecureMemoryStorage { storage: Mutex::new(HashMap::new()) }
    }
}

#[async_trait]
impl SecureKeyStorage for MockSecureMemoryStorage {
    async fn store_key(&self, key_pair: &QrKeyPair) -> Result<(), String> {
        info!("MockSecureMemoryStorage: Storing key {}.", key_pair.id);
        self.storage.lock().unwrap().insert(key_pair.id.clone(), key_pair.clone());
        Ok(())
    }

    async fn retrieve_key(&self, id: &str) -> Result<Option<QrKeyPair>, String> {
        info!("MockSecureMemoryStorage: Retrieving key {}.", id);
        Ok(self.storage.lock().unwrap().get(id).cloned())
    }

    async fn delete_key(&self, id: &str) -> Result<(), String> {
        info!("MockSecureMemoryStorage: Deleting key {}.", id);
        self.storage.lock().unwrap().remove(id);
        Ok(())
    }

    async fn list_key_ids(&self) -> Result<Vec<String>, String> {
        info!("MockSecureMemoryStorage: Listing all key IDs.");
        Ok(self.storage.lock().unwrap().keys().cloned().collect())
    }
}

/// Manages the secure generation, storage, and retrieval of QR keys.
pub struct QrKeyManager {
    secure_storage: Box<dyn SecureKeyStorage + Send + Sync>,
}

impl QrKeyManager {
    pub fn new() -> Self {
        println!("QR Key Manager initialized with MockSecureMemoryStorage.");
        QrKeyManager { secure_storage: Box::new(MockSecureMemoryStorage::new()) }
    }

    /// Generates a new QR key pair for a specified algorithm type and usage.
    pub async fn generate_key_pair(
        &mut self,
        alg_type: QrAlgorithmType,
        usage: QrKeyUsage,
        duration_days: i64,
    ) -> Result<QrKeyPair, String> {
        let id = format!("qr-key-{}", Uuid::new_v4());
        let generated_at = Utc::now();
        let expires_at = generated_at + ChronoDuration::days(duration_days);

        // Mock key generation - replace with actual crypto library calls
        info!("Generating QR key pair for {:?} (Mock)", alg_type);
        let public_key = vec![0x01; 64]; // Mock public key bytes
        let private_key = vec![0x02; 128]; // Mock private key bytes

        let key_pair = QrKeyPair {
            id: id.clone(),
            algorithm_type: alg_type.clone(),
            public_key,
            private_key,
            generated_at,
            expires_at,
            usage,
        };

        self.secure_storage.store_key(&key_pair).await?;
        Ok(key_pair)
    }

    /// Retrieves an active key pair by ID or usage.
    pub async fn get_active_key_pair(&self, usage: &QrKeyUsage) -> Option<QrKeyPair> {
        // In a real system, this would query the storage more intelligently (e.g., by usage type)
        // For mock: iterate through all stored keys
        if let Ok(key_ids) = self.secure_storage.list_key_ids().await {
            for id in key_ids {
                if let Ok(Some(key_pair)) = self.secure_storage.retrieve_key(&id).await {
                    if &key_pair.usage == usage && key_pair.expires_at > Utc::now() {
                        return Some(key_pair);
                    }
                }
            }
        }
        None
    }

    /// Rotates a key pair, generating a new one and marking the old for deprecation.
    pub async fn rotate_key_pair(&mut self, usage: QrKeyUsage, alg_type: QrAlgorithmType, duration_days: i64) -> Result<QrKeyPair, String> {
        info!("Rotating QR key pair for {:?}...", usage);
        // In a real scenario, this would safely deprecate the old key after a transition period
        if let Some(old_key) = self.get_active_key_pair(&usage).await {
            info!("Revoking old key: {}", old_key.id);
            self.secure_storage.delete_key(&old_key.id).await?;
        }
        self.generate_key_pair(alg_type, usage, duration_days).await
    }

    /// Revokes a key pair, rendering it unusable.
    pub async fn revoke_key_pair(&mut self, id: &str) -> Result<(), String> {
        info!("Revoking key pair {}", id);
        self.secure_storage.delete_key(id).await
    }
}
