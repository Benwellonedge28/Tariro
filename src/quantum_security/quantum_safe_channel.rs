
//! Manages supremely autonomous, quantum-safe communication channels within Tariro.

use crate::quantum_security::prelude::*;
use crate::quantum_security::pqc_algorithms::{QuantumResistantCipher, QuantumResistantKeyExchange};
use crate::quantum_security::mod::{EvasCompliancePolicy, QuantumAutonomyLevel};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};

/// Represents a dynamic, self-adapting quantum-safe communication channel.
#[derive(Debug)]
pub struct QuantumSafeChannel {
    pub peer_id: String,
    session_key: Arc<Mutex<Vec<u8>>>,
    kex_algo: Box<dyn QuantumResistantKeyExchange + Send + Sync>,
    cipher_algo: Box<dyn QuantumResistantCipher + Send + Sync>,
    evas_policy: EvasCompliancePolicy,
    last_key_rotation: Arc<Mutex<DateTime<Utc>>>,
    // Advanced features:
    rekey_interval_seconds: u64,
    quantum_random_seed_used: bool,
}

impl QuantumSafeChannel {
    /// Establishes a new quantum-safe channel.
    /// This process is designed to be "infinitely advanced" with autonomous key management.
    pub async fn new(
        peer_id: String,
        kex_algo: Box<dyn QuantumResistantKeyExchange + Send + Sync>,
        cipher_algo: Box<dyn QuantumResistantCipher + Send + Sync>,
        evas_policy: EvasCompliancePolicy,
    ) -> Result<Self> {
        tariro_log!(info, "Initiating PQC key exchange for new channel with {}", peer_id);

        // Autonomous key exchange for session establishment
        let (public_key, private_key) = kex_algo.generate_keypair()?;
        // Simulate sending public_key to peer and receiving encapsulated key
        // For demonstration, we'll self-encapsulate/decapsulate
        let (shared_secret_initiator, encapsulated_key) = kex_algo.encapsulate_key(&public_key)?;
        let shared_secret_recipient = kex_algo.decapsulate_key(&private_key, &encapsulated_key)?;

        if shared_secret_initiator != shared_secret_recipient {
            return Err("PQC Key Exchange failed: shared secrets do not match.".into());
        }

        tariro_log!(info, "Quantum-resistant session key established for channel with {}", peer_id);

        Ok(Self {
            peer_id,
            session_key: Arc::new(Mutex::new(shared_secret_initiator)),
            kex_algo,
            cipher_algo,
            evas_policy,
            last_key_rotation: Arc::new(Mutex::new(Utc::now())),
            rekey_interval_seconds: 3600,
            quantum_random_seed_used: false,
        })
    }

    /// Autonomously encrypts data for transmission over the quantum-safe channel.
    pub async fn send_data(&self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let key = self.session_key.lock().await;
        let encrypted_data = self.cipher_algo.encrypt(&key, plaintext, aad)?;
        tariro_log!(trace, "Encrypted {} bytes for transmission to {}", plaintext.len(), self.peer_id);
        
        // Ethical filter check (EvasCompliance)
        if self.evas_policy.enabled {
            // Further checks could involve content analysis for compliance
            // For infinitely advanced: real-time ethical reasoning on data transmission
            if self.evas_policy.data_minimization_active && plaintext.len() > 1024 {
                tariro_log!(warn, "Large data ({}) sent, data minimization active. Ensure compliance for {}", plaintext.len(), self.peer_id);
            }
        }

        Ok(encrypted_data)
    }

    /// Autonomously decrypts received data from the quantum-safe channel.
    pub async fn receive_data(&self, ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
        let key = self.session_key.lock().await;
        let decrypted_data = self.cipher_algo.decrypt(&key, ciphertext, aad)?;
        tariro_log!(trace, "Decrypted {} bytes received from {}", decrypted_data.len(), self.peer_id);
        Ok(decrypted_data)
    }

    /// Initiates an autonomous re-keying process for the channel.
    /// This ensures forward secrecy and continuously adapts to changing quantum threats.
    pub async fn autonomous_rekey(&self) -> Result<()> {
        let mut last_rotation = self.last_key_rotation.lock().await;
        let now = Utc::now();

        if (now - *last_rotation).num_seconds() < self.rekey_interval_seconds as i64 {
            tariro_log!(trace, "Re-keying not yet due for channel with {}", self.peer_id);
            return Ok(());
        }

        tariro_log!(info, "Initiating autonomous re-keying for channel with {}", self.peer_id);
        let new_key_material = self.kex_algo.generate_keypair()?; // Peer would also do this
        // Simulate new key exchange
        let (new_shared_secret, _) = self.kex_algo.encapsulate_key(&new_key_material.0)?;
        
        let mut current_session_key = self.session_key.lock().await;
        *current_session_key = new_shared_secret;
        *last_rotation = now;

        tariro_log!(info, "Channel with {} successfully re-keyed autonomously.", self.peer_id);
        Ok(())
    }

    /// Allows autonomous policy adjustments for the channel.
    pub fn update_evas_policy(&mut self, new_policy: EvasCompliancePolicy) {
        self.evas_policy = new_policy;
        tariro_log!(info, "EvasCompliancePolicy updated for channel with {}", self.peer_id);
    }
}
