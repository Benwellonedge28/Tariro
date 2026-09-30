//! Tariro Knowledge-Centric AGI System
//!
//! A sovereign AGI architecture based on explicit symbolic knowledge,
//! logical reasoning, and causal inference - NOT token prediction.

pub mod knowledge_graph;
pub mod reasoning;
pub mod logic;
pub mod causal;
pub mod commonsense;
pub mod formal;
pub mod mind;

#[cfg(test)]
mod tests;

pub use knowledge_graph::KnowledgeGraph;
pub use reasoning::ReasoningEngine;
pub use logic::LogicEngine;
pub use causal::CausalReasoningEngine;
pub use commonsense::CommonSenseEngine;
pub use formal::FormalVerifier;
pub use mind::TariroMind;
