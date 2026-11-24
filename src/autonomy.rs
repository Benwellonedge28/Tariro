// src/autonomy.rs

//! Manages Tariro AGI's autonomy policies and decision-making processes.

pub mod governance;
pub mod policy_engine;

// Re-export common types for easier access
pub use governance::{AutonomyChangeEvent, GovernanceAction, GovernanceService};
pub use policy_engine::{AutonomyLevel, AutonomyPolicy, PolicyDecision};
