// Tariro_ide_idpte/src/omega_os/secure_channel_manager.rs

//! Manages secure communication channels within OMEGA OS, leveraging QR-CTFO for cryptographic primitives.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::omega_os::network_models::{SecureChannelInfo, SecureChannelStatus};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};

use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // Import QR-CTFO Orchestrator
use crate::crypto_fabric::communication_adapter::CommunicationChannel; // For QR-CTFO

/// The OMEGA OS Secure Channel Manager.
pub struct SecureChannelManager {
    active_channels: Mutex<HashMap<String, SecureChannelInfo>>, // id -> SecureChannelInfo
    qr_ctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, // Reference to QR-CTFO for crypto ops
    event_bus: Arc<EventBus>,
}

impl SecureChannelManager {
    pub fn new(qr_ctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, event_bus: Arc<EventBus>) -> Self {
        info!("OMEGA OS Secure Channel Manager initialized.");
        SecureChannelManager {
            active_channels: Mutex::new(HashMap::new()),
            qr_ctfo_orchestrator,
            event_bus,
        }
    }

    /// Establishes a new secure communication channel to a peer.
    pub async fn establish_secure_channel(
        &self,
        peer_id: &str,
        channel_type: CommunicationChannel,
        interface_id: Option<String>,
    ) -> Result<String, String> {
        let channel_id = format!("sec-chan-{}", Uuid::new_v4());
        info!("SecureChannelManager: Establishing secure channel {} to {}.", channel_id, peer_id);

        // Leverage QR-CTFO to set up the underlying secure communication
        let qr_ctfo_guard = self.qr_ctfo_orchestrator.lock().unwrap();
        match qr_ctfo_guard.comm_adapter.register_channel(channel_type.clone()).await {
            Ok(_) => {
                let mut active_channels_guard = self.active_channels.lock().unwrap();
                let new_channel = SecureChannelInfo {
                    id: channel_id.clone(),
                    peer_id: peer_id.to_string(),
                    established_at: Utc::now(),
                    last_used: Utc::now(),
                    current_pqc_alg: "Dynamically Negotiated PQC".to_string(), // From QR-CTFO
                    status: SecureChannelStatus::Active,
                    data_encrypted_bytes: 0,
                    data_decrypted_bytes: 0,
                    associated_network_interface_id: interface_id,
                };
                active_channels_guard.insert(channel_id.clone(), new_channel.clone());
                info!("SecureChannelManager: Secure channel {} established to {}.", channel_id, peer_id);
                self.event_bus.publish(EventEnvelope::new("SecureChannelManager".to_string(), OsEvent::Custom("SecureChannelEstablished".to_string(), channel_id.clone()))).await;
                Ok(channel_id)
            },
            Err(e) => {
                error!("SecureChannelManager: Failed to establish secure channel via QR-CTFO: {}", e);
                self.event_bus.publish(EventEnvelope::new("SecureChannelManager".to_string(), OsEvent::Custom("SecureChannelError".to_string(), format!("Failed to establish channel to {}: {}", peer_id, e)))).await;
                Err(format!("Failed to establish secure channel: {}", e))
            }
        }
    }

    /// Sends data securely over an established channel.
    pub async fn send_secure_data(&self, channel_id: &str, data: &[u8]) -> Result<u64, String> {
        let mut active_channels_guard = self.active_channels.lock().unwrap();
        if let Some(channel) = active_channels_guard.get_mut(channel_id) {
            if channel.status == SecureChannelStatus::Active {
                let qr_ctfo_guard = self.qr_ctfo_orchestrator.lock().unwrap();
                // For simplicity, we need to know the CommunicationChannel type here. Mock it.
                let comm_channel = CommunicationChannel::Other(channel.peer_id.clone()); 
                match qr_ctfo_guard.comm_adapter.encrypt_data(&comm_channel, data).await {
                    Ok(encrypted_data) => {
                        channel.data_encrypted_bytes += encrypted_data.len() as u64;
                        channel.last_used = Utc::now();
                        info!("SecureChannelManager: Sent {} bytes securely on channel {}. (Encrypted: {} bytes)", data.len(), channel_id, encrypted_data.len());
                        // Simulate sending over network
                        self.event_bus.publish(EventEnvelope::new("SecureChannelManager".to_string(), OsEvent::Custom("SecureDataSent".to_string(), format!("channel: {}, bytes: {}", channel_id, data.len())))).await;
                        Ok(data.len() as u64)
                    },
                    Err(e) => Err(format!("Failed to send secure data: {}", e)),
                }
            } else {
                Err(format!("Secure channel {} is not active.", channel_id))
            }
        } else {
            Err(format!("Secure channel {} not found.", channel_id))
        }
    }

    /// Receives and decrypts data from an established channel.
    pub async fn receive_secure_data(&self, channel_id: &str, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
        let mut active_channels_guard = self.active_channels.lock().unwrap();
        if let Some(channel) = active_channels_guard.get_mut(channel_id) {
            if channel.status == SecureChannelStatus::Active {
                let qr_ctfo_guard = self.qr_ctfo_orchestrator.lock().unwrap();
                let comm_channel = CommunicationChannel::Other(channel.peer_id.clone()); 
                match qr_ctfo_guard.comm_adapter.decrypt_data(&comm_channel, encrypted_data).await {
                    Ok(decrypted_data) => {
                        channel.data_decrypted_bytes += encrypted_data.len() as u64;
                        channel.last_used = Utc::now();
                        info!("SecureChannelManager: Received {} bytes securely on channel {}. (Decrypted: {} bytes)", encrypted_data.len(), channel_id, decrypted_data.len());
                        self.event_bus.publish(EventEnvelope::new("SecureChannelManager".to_string(), OsEvent::Custom("SecureDataReceived".to_string(), format!("channel: {}, bytes: {}", channel_id, decrypted_data.len())))).await;
                        Ok(decrypted_data)
                    },
                    Err(e) => Err(format!("Failed to receive secure data: {}", e)),
                }
            } else {
                Err(format!("Secure channel {} is not active.", channel_id))
            }
        } else {
            Err(format!("Secure channel {} not found.", channel_id))
        }
    }

    /// Closes an established secure channel.
    pub async fn close_secure_channel(&self, channel_id: &str) -> Result<(), String> {
        let mut active_channels_guard = self.active_channels.lock().unwrap();
        if let Some(channel) = active_channels_guard.get_mut(channel_id) {
            channel.status = SecureChannelStatus::Closed;
            info!("SecureChannelManager: Secure channel {} closed.", channel_id);
            self.event_bus.publish(EventEnvelope::new("SecureChannelManager".to_string(), OsEvent::Custom("SecureChannelClosed".to_string(), channel_id.to_string()))).await;
            // In a real system, would also unregister from QR-CTFO
            active_channels_guard.remove(channel_id);
            Ok(())
        } else {
            Err(format!("Secure channel {} not found.", channel_id))
        }
    }

    /// Registers the SecureChannelManager itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "os-secure-channel-manager".to_string(),
            name: "OMEGA OS Secure Channel Manager".to_string(),
            description: "Manages secure communication channels using Quantum-Resistant Cryptography.".to_string(),
            service_type: ServiceType::Security,
            capabilities: vec![
                ServiceCapability { name: "establish_secure_channel".to_string(), description: "Establishes a secure channel.".to_string(), parameters: HashMap::new(), output_schema: "SecureChannelId".to_string(), requires_permissions: vec![], requires_ethical_review: true },
                ServiceCapability { name: "send_secure_data".to_string(), description: "Sends encrypted data over a secure channel.".to_string(), parameters: HashMap::new(), output_schema: "bytes_sent".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "receive_secure_data".to_string(), description: "Receives and decrypts data from a secure channel.".to_string(), parameters: HashMap::new(), output_schema: "decrypted_data".to_string(), requires_permissions: vec![], requires_ethical_review: false },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "qr-ctfo".to_string()],
            managed_by_agi_module: Some("QR-CTFO".to_string()),
        }
    }
}
