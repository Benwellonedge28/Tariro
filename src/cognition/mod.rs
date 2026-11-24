//! Tariro Cognition - The Mind Architecture
//!
//! This module implements the core cognitive architecture of Tariro AGI,
//! including consciousness, working memory, world model, and motivation.

pub mod cognition_engine;
pub mod consciousness;
pub mod motivation;
pub mod working_memory;
pub mod world_model;

#[cfg(test)]
mod tests;

pub use cognition_engine::CognitionEngine;
pub use consciousness::Consciousness;
pub use motivation::MotivationSystem;
pub use working_memory::WorkingMemory;
pub use world_model::WorldModel;
