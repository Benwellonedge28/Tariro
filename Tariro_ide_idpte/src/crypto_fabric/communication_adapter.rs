// Tariro_ide_idpte/src/crypto_fabric/communication_adapter.rs

//! Adapts existing communication channels and protocols to utilize Quantum-Resistant (QR) cryptography.
//! This module provides an abstraction layer for integrating QR algorithms without disrupting
//! the core communication logic of OMEGA OS components.

use crate::crypto_fabric::qr_algorithms::{QrKeyExchangeAlg, QrSignatureAlg, QrAlgorithmInfo, QrAlgorithmType};
use crate::crypto_fabric::key_management::QrKeyPair;
use crate::crypto_fabric::trust_policy_engine::{TrustPolicyEngine, TrustLevel};

use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, warn, error};

/// Represents a secure communication channel that needs PQC adaptation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommunicationChannel {
    NetworkInterface(String), // e.g., "eth0", "wifi0"
    InterProcess(String), // e.g., "agi_core_to_sorae_channel"
    CloudService(String), // e.g., "omega_cloud_storage"
    Other(String),
}

/// Represents the PQC state of a communication channel.
#[derive(Debug, Clone)]
pub struct PqcChannelState {
    pub negotiated_kex_alg: QrKeyExchangeAlg,
    pub negotiated_sign_alg: QrSignatureAlg,
    pub current_session_key: Option<Vec<u8>>, // Symmetric key derived from KEM
    pub channel_id: String,
    pub is_secure: bool,
}

impl CommunicationAdapter {
    pub fn new(policy_engine: Arc<Mutex<TrustPolicyEngine>>) -> Self {
        info!("Communication Adapter initialized.");
        CommunicationAdapter {
            active_channels: Arc::new(Mutex::new(HashMap::new())),
            policy_engine,
        }
    }

    /// Registers a new communication channel for PQC adaptation.
    pub async fn register_channel(&self, channel: CommunicationChannel) -> Result<(), String> {
        info!("Registering communication channel: {:?}", channel);
        let mut channels = self.active_channels.lock().await;

        if channels.contains_key(&channel) {
            warn!("Channel {:?} already registered.", channel);
            return Err(format!("Channel {:?} already registered.", channel));
        }

        // Dynamic PQC Protocol Negotiation
        let context_str = format!("{:?}", channel);
        let policy_engine_guard = self.policy_engine.lock().unwrap();
        let recommended_algs = policy_engine_guard.get_recommended_algorithms(&context_str).await;
        drop(policy_engine_guard); // Release lock as soon as possible

        let negotiated_kex_alg = recommended_algs.iter()
            .find_map(|alg_type| if let QrAlgorithmType::KeyExchange(kex) = alg_type { Some(kex.clone()) } else { None })
            .unwrap_or_else(|| { warn!("No recommended KEX alg found for {:?}, defaulting to Kyber.", channel); QrKeyExchangeAlg::Kyber });

        let negotiated_sign_alg = recommended_algs.iter()
            .find_map(|alg_type| if let QrAlgorithmType::DigitalSignature(sig) = alg_type { Some(sig.clone()) } else { None })
            .unwrap_or_else(|| { warn!("No recommended Sign alg found for {:?}, defaulting to Dilithium.", channel); QrSignatureAlg::Dilithium });

        let default_state = PqcChannelState {
            negotiated_kex_alg,
            negotiated_sign_alg,
            current_session_key: None,
            channel_id: format!("chan-{}", Uuid::new_v4()),
            is_secure: false, // Initially not secure until setup
        };
        channels.insert(channel.clone(), default_state);
        info!("Channel {:?} registered. Now setting up PQC.", channel);
        self.setup_pqc_for_channel(channel).await?; // Initial setup
        Ok(())
    }

    /// Sets up the Post-Quantum Cryptography for a specific channel.
    pub async fn setup_pqc_for_channel(&self, channel: CommunicationChannel) -> Result<(), String> {
        let mut channels = self.active_channels.lock().await;
        if let Some(state) = channels.get_mut(&channel) {
            info!("Setting up PQC for channel {:?} using KEX: {:?} and Sign: {:?}", channel, state.negotiated_kex_alg, state.negotiated_sign_alg);
            // In a real system:
            // 1. Exchange public keys (e.g., via a secure handshake with the other endpoint).
            // 2. Perform a PQC Key Encapsulation Mechanism (KEM) to establish a shared secret.
            // 3. Use the shared secret to derive a symmetric encryption key.
            // 4. Update channel state to 'is_secure = true'.
            state.is_secure = true;
            state.current_session_key = Some(vec![0x03; 32]); // Mock session key

            // After setup, evaluate compliance with the policy engine
            let context_str = format!("{:?}", channel);
            let policy_engine_guard = self.policy_engine.lock().unwrap();
            let used_algs = vec![
                QrAlgorithmType::KeyExchange(state.negotiated_kex_alg.clone()),
                QrAlgorithmType::DigitalSignature(state.negotiated_sign_alg.clone()),
            ];
            if let Err(e) = policy_engine_guard.evaluate_compliance(&context_str, &used_algs).await {
                error!("PQC Setup for channel {:?} failed compliance check: {}", channel, e);
                state.is_secure = false; // Mark as not secure if compliance fails
                return Err(format!("Compliance check failed: {}", e));
            }
            Ok(())
        } else {
            Err(format!("Channel {:?} not registered.", channel))
        }
    }

    /// Updates the PQC keys for all active channels (e.g., after key rotation).
    pub async fn update_pqc_keys(&self) {
        info!("CommunicationAdapter: Updating PQC keys for all active channels...");
        let channels_to_update: Vec<CommunicationChannel> = {
            let channels = self.active_channels.lock().await;
            channels.keys().cloned().collect()
        };

        for channel in channels_to_update {
            if let Err(e) = self.setup_pqc_for_channel(channel.clone()).await {
                error!("Failed to update PQC keys for channel: {:?} - {}", channel, e);
            }
        }
    }

    /// Encrypts data using the channel's current PQC session key.
    pub async fn encrypt_data(&self, channel: &CommunicationChannel, data: &[u8]) -> Result<Vec<u8>, String> {
        let channels = self.active_channels.lock().await;
        if let Some(state) = channels.get(channel) {
            if state.is_secure {
                if let Some(key) = &state.current_session_key {
                    info!("Encrypting data for {:?} using PQC (Mock)", channel);
                    // Mock encryption: In a real system, use AES-256 with key
                    let mut encrypted_data = data.to_vec();
                    encrypted_data.extend_from_slice(key);
                    Ok(encrypted_data) // Simple mock
                } else {
                    Err("Channel secure but no session key established.".to_string())
                }
            } else {
                Err("Channel is not secure.".to_string())
            }
        } else {
            Err(format!("Channel {:?} not registered.", channel))
        }
    }

    /// Decrypts data using the channel's current PQC session key.
    pub async fn decrypt_data(&self, channel: &CommunicationChannel, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
        let channels = self.active_channels.lock().await;
        if let Some(state) = channels.get(channel) {
            if state.is_secure {
                if let Some(key) = &state.current_session_key {
                    info!("Decrypting data for {:?} using PQC (Mock)", channel);
                    // Mock decryption: In a real system, use AES-256 with key
                    if encrypted_data.ends_with(key) {
                        Ok(encrypted_data[..encrypted_data.len() - key.len()].to_vec())
                    } else {
                        Err("Decryption failed: Key mismatch or invalid data.".to_string())
                    }
                } else {
                    Err("Channel secure but no session key established.".to_string())
                }
            } else {
                Err("Channel is not secure.".to_string())
            }
        } else {
            Err(format!("Channel {:?} not registered.", channel))
        }
    }
}
