// src/autonomy.rs

//! Manages Tariro AGI's autonomy policies and decision-making processes.

use crate::prelude::*;
pub mod policy_engine;
pub mod governance;

// Re-export common types for easier access
pub use policy_engine::{AutonomyPolicy, PolicyDecision};
pub use governance::{GovernanceService, AutonomyChangeEvent, GovernanceAction};
