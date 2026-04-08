// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Serialization error: {0}")] SerializationError(#[from] serde_json::Error),
    #[error("I/O error: {0}")] IoError(#[from] std::io::Error),
}
