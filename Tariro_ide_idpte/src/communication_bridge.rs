// Tariro_ide_idpte/src/communication_bridge.rs

//! Handles secure communication between Tariro AGI and external components, like the IDE bridge.
//! This module uses the QR-CTFO's CommunicationAdapter to ensure all messages are quantum-resistant secure.

use crate::bridge_protocol::{BridgeMessage, MessageType};
use crate::crypto_fabric::communication_adapter::{CommunicationAdapter, CommunicationChannel};
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;

/// The CommunicationBridge provides methods to send and receive secure messages.
pub struct CommunicationBridge {
    adapter: Arc<CommunicationAdapter>,
    channel_id: CommunicationChannel,
}

impl CommunicationBridge {
    /// Creates a new CommunicationBridge instance for a specific channel.
    pub async fn new(adapter: Arc<CommunicationAdapter>, channel: CommunicationChannel) -> Result<Self> {
        adapter.register_channel(channel.clone()).await
            .map_err(|e| anyhow::anyhow!("Failed to register communication channel: {}", e))?;
        Ok(CommunicationBridge {
            adapter,
            channel_id: channel,
        })
    }

    /// Sends a message, encrypting its payload using the CommunicationAdapter.
    pub async fn send_secure_message(&self, message_type: MessageType, payload: Value) -> Result<()> {
        let payload_bytes = serde_json::to_vec(&payload)?;
        let encrypted_bytes = self.adapter.encrypt_data(&self.channel_id, &payload_bytes).await
            .map_err(|e| anyhow::anyhow!("Failed to encrypt message: {}", e))?;

        let secure_message = BridgeMessage::new_encrypted_message(
            encrypted_bytes,
            message_type,
            format!("{:?}", self.channel_id),
        );

        // Here, the secure_message would be sent over the actual transport layer (e.g., WebSocket, TCP)
        println!("CommunicationBridge: Sent secure message of type {:?} from channel {:?}", message_type, self.channel_id);
        // In a real system, this would push `secure_message` to a network sender.
        Ok(())
    }

    /// Receives and decrypts a message's payload using the CommunicationAdapter.
    pub async fn receive_secure_message(&self, incoming_message: BridgeMessage) -> Result<BridgeMessage> {
        if !incoming_message.is_encrypted {
            println!("CommunicationBridge: Received unencrypted message, processing as is.");
            return Ok(incoming_message);
        }

        if let Some(encrypted_payload) = incoming_message.encrypted_payload {
            let decrypted_bytes = self.adapter.decrypt_data(&self.channel_id, &encrypted_payload).await
                .map_err(|e| anyhow::anyhow!("Failed to decrypt message: {}", e))?;
            let original_payload: Value = serde_json::from_slice(&decrypted_bytes)?;

            println!("CommunicationBridge: Received and decrypted message of type {:?} for channel {:?}", incoming_message.message_type, self.channel_id);

            Ok(BridgeMessage {
                payload: Some(original_payload),
                encrypted_payload: None,
                is_encrypted: false,
                ..incoming_message
            })
        } else {
            Err(anyhow::anyhow!("Encrypted message received without encrypted_payload."))
        }
    }
}
