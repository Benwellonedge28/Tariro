// src/bridge_protocol.rs

//! Defines the communication protocol between the Tariro AGI and the IDE Bridge.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Represents a message sent across the bridge.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeMessage {
    pub id: String,
    pub message_type: MessageType,
    pub payload: Option<serde_json::Value>, // Original payload, if unencrypted
    pub encrypted_payload: Option<Vec<u8>>, // Encrypted payload bytes
    pub is_encrypted: bool,
    pub timestamp: DateTime<Utc>,
    pub source_channel: String, // To identify which channel this message came from
}

/// Distinguishes between different types of bridge messages.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageType {
    Request,
    Response,
    Notification,
    Error,
    // Add more specific message types as needed for IDE communication
    IdeCommand,
    IdeEvent,
}

impl BridgeMessage {
    pub fn new_request(payload: serde_json::Value, source_channel: String) -> Self {
        BridgeMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::Request,
            payload: Some(payload),
            encrypted_payload: None,
            is_encrypted: false,
            timestamp: Utc::now(),
            source_channel,
        }
    }

    pub fn new_encrypted_message(encrypted_payload: Vec<u8>, message_type: MessageType, source_channel: String) -> Self {
        BridgeMessage {
            id: Uuid::new_v4().to_string(),
            message_type,
            payload: None,
            encrypted_payload: Some(encrypted_payload),
            is_encrypted: true,
            timestamp: Utc::now(),
            source_channel,
        }
    }
}
