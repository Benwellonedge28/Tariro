// src/lib.rs

//! Tariro IDE and IDPTE (Intelligent Development Platform & Tooling Engine) integration library.
//! 
//! This library provides the core logic for integrating the Tariro AGI with modern IDEs,
//! facilitating a seamless AI-native development experience.

pub mod ide_integration;
pub mod compiler_server;
pub mod bridge_protocol;
pub mod features; // Declare the features module
pub mod agi_core_modules; // Declare the new AGI Core Modules

// Re-export common types for easier use
pub use ide_integration::{IdeConnector, IdeRequest, IdeResponse, FileUpdate, FileInfo};
pub use compiler_server::{CompilerServer, LanguageDefinition};
pub use bridge_protocol::{BridgeMessage, MessageType};

/// Version of the Tariro IDE integration library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initializes the IDE integration sub-system.
pub async fn initialize() -> Result<(), String> {
    println!("Initializing Tariro IDE/IDPTE Integration v{}...", VERSION);
    // Setup connections, initialize LSP servers, etc.
    Ok(())
}

/// Shuts down the IDE integration sub-system.
pub async fn shutdown() -> Result<(), String> {
    println!("Shutting down Tariro IDE/IDPTE Integration...");
    // Cleanup resources
    Ok(())
}
