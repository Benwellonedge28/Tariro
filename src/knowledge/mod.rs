//! Tariro Knowledge-Centric AGI System
//!
//! A sovereign AGI architecture based on explicit symbolic knowledge,
//! logical reasoning, and causal inference - NOT token prediction.

pub mod causal;
pub mod commonsense;
pub mod formal;
pub mod knowledge_graph;
pub mod logic;
pub mod mind;
pub mod reasoning;

#[cfg(test)]
mod tests;

pub use causal::CausalReasoningEngine;
pub use commonsense::CommonSenseEngine;
pub use formal::FormalVerifier;
pub use knowledge_graph::KnowledgeGraph;
pub use logic::LogicEngine;
pub use mind::TariroMind;
pub use reasoning::ReasoningEngine;
