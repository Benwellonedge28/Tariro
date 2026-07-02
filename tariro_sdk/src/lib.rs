//! Tariro SDK - Shared entities and types for Tariro AGI System
//!
//! This crate contains all the core data structures, entities, and types
//! used throughout the Tariro AGI system.

pub mod entities;

pub use entities::*;

/// Result type alias for Tariro operations
pub type Result<T> = std::result::Result<T, TariroError>;

/// Tariro error types
#[derive(Debug, thiserror::Error)]
pub enum TariroError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("AGI error: {0}")]
    AGI(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal error: {0}")]
    Internal(String),
}