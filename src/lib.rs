// src/lib.rs

//! Tariro: The Self-Evolving AGI Core.
//! This crate contains the foundational components and modules for the Tariro AGI system.

// Re-export common utilities and traits for modules
pub use anyhow::{Result, Error};
pub use async_trait::async_trait;
pub use log::{info, warn, error, debug};
pub use uuid;
pub use chrono;

// Define the core Module trait that all Tariro modules must implement.
// This ensures a consistent interface for module lifecycle management.
#[async_trait]
pub trait Module: Send + Sync {
    fn new(context: &ModuleContext) -> Result<Self> where Self: Sized;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    fn name(&self) -> &'static str;
}

// Defines the context provided to each module during initialization.
// This context can contain shared resources, configuration, and access to other modules.
pub struct ModuleContext {
    // Configuration data for the entire system
    pub config: AppConfig,
    // Database connections, external API clients, other shared resources
    // pub db_client: DbClient,
    // pub event_bus: EventBus,
}

impl ModuleContext {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
}

// Placeholder for application-wide configuration
pub struct AppConfig {
    pub log_level: String,
    pub database_url: String,
    // Other global settings
}

// Placeholder for FeatureProposalId (from the data model)
pub struct FeatureProposalId(pub String);

// Declare the modules within this crate
pub mod modules;

