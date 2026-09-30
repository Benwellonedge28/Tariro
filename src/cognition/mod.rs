//! Tariro Cognition - The Mind Architecture
//!
//! This module implements the core cognitive architecture of Tariro AGI,
//! including consciousness, working memory, world model, and motivation.

pub mod consciousness;
pub mod working_memory;
pub mod world_model;
pub mod motivation;
pub mod cognition_engine;

#[cfg(test)]
mod tests;

pub use consciousness::Consciousness;
pub use working_memory::WorkingMemory;
pub use world_model::WorldModel;
pub use motivation::MotivationSystem;
pub use cognition_engine::CognitionEngine;
