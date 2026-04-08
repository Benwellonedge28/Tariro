// src/bridge_protocol.rs

//! Defines the communication protocol between the Tariro AGI and the IDE Bridge.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a message sent across the bridge.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BridgeMessage {
    pub id: String,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Distinguishes between different types of bridge messages.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageType {
    Request,
    Response,
    Notification,
    Error,
}

impl BridgeMessage {
    pub fn new_request(payload: serde_json::Value) -> Self {
        BridgeMessage {
            id: Uuid::new_v4().to_string(),
            message_type: MessageType::Request,
            payload,
            timestamp: chrono::Utc::now(),
        }
    }
}
