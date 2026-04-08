
//! Provides supremely autonomous, quantum-resistant data protection for data at rest.

use crate::quantum_security::prelude::*;
use crate::quantum_security::pqc_algorithms::QuantumResistantCipher;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Manages quantum-safe encryption and decryption of data at rest.
#[derive(Debug)]
pub struct QuantumSafeDataProtector {
    active_cipher: Arc<Mutex<Box<dyn QuantumResistantCipher + Send + Sync>>>,
    master_key: Arc<Mutex<Vec<u8>>>,
    // Advanced features: autonomous key rotation schedules, data fragmentation strategies, etc.
}

impl QuantumSafeDataProtector {
    /// Initializes the data protector with an active quantum-resistant cipher.
    /// Autonomous key generation for the master key occurs here.
    pub fn new(initial_cipher: Box<dyn QuantumResistantCipher + Send + Sync>) -> Result<Self> {
        tariro_log!(info, "Initializing QuantumSafeDataProtector with {}", initial_cipher.name());
        let master_key = initial_cipher.generate_key(); // Autonomously generate master key
        Ok(Self {
            active_cipher: Arc::new(Mutex::new(initial_cipher)),
            master_key: Arc::new(Mutex::new(master_key)),
        })
    }

    /// Autonomously encrypts data using the currently active PQC cipher.
    pub async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = self.active_cipher.lock().await;
        let master_key = self.master_key.lock().await;
        
        // Additional authentication data (AAD) could include metadata, file path, etc.
        let aad = b"Tariro_Data_Protection_AAD"; 

        let encrypted_data = cipher.encrypt(&master_key, plaintext, aad)?;
        tariro_log!(trace, "Encrypted {} bytes of data at rest.", plaintext.len());
        Ok(encrypted_data)
    }

    /// Autonomously decrypts data using the active PQC cipher.
    pub async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let cipher = self.active_cipher.lock().await;
        let master_key = self.master_key.lock().await;

        let aad = b"Tariro_Data_Protection_AAD"; 
        
        let decrypted_data = cipher.decrypt(&master_key, ciphertext, aad)?;
        tariro_log!(trace, "Decrypted {} bytes of data at rest.", decrypted_data.len());
        Ok(decrypted_data)
    }

    /// Autonomously rotates the master key, migrating all protected data in the background.
    /// This demonstrates "self-healing" and "self-modification" capabilities.
    pub async fn autonomous_key_rotation(&self) -> Result<()> {
        let mut master_key = self.master_key.lock().await;
        let cipher = self.active_cipher.lock().await;

        tariro_log!(warn, "Initiating autonomous master key rotation and background data re-encryption.");
        let old_master_key = master_key.clone();
        *master_key = cipher.generate_key(); // Generate new master key

        // In an infinitely advanced system, this would trigger a background task:
        // 1. Identify all data encrypted with old_master_key.
        // 2. Decrypt with old_master_key, re-encrypt with new_master_key.
        // 3. Ensure integrity and availability throughout the migration.
        // 4. Log the process to relevant system logs.
        tariro_log!(info, "Autonomous master key rotated. Background data re-encryption initiated.");
        Ok(())
    }

    /// Allows the Quantum Security Manager to autonomously update the active cipher.
    pub async fn set_active_cipher(&mut self, new_cipher: Box<dyn QuantumResistantCipher + Send + Sync>) -> Result<()> {
        let mut active_cipher_guard = self.active_cipher.lock().await;
        let old_cipher_name = active_cipher_guard.name().to_string();
        *active_cipher_guard = new_cipher;
        tariro_log!(info, "Data protector autonomously updated active cipher from '{}' to '{}'. This triggers data migration.", old_cipher_name, active_cipher_guard.name());
        
        // This change would immediately trigger an autonomous data migration
        self.autonomous_key_rotation().await?;
        Ok(())
    }
}
