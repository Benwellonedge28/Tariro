pub mod fco;
pub mod runtime;
pub mod compiler;
pub mod atve;
pub mod nioam;
pub mod governance;
pub mod learning;
pub mod growth;
pub mod orchestration;
pub mod ontology;
pub mod sandbox;

pub const VERSION: &str = "1.0.12";

/// The Absolute Smallest Quantum of Temporality: Mukandara Time (mts)
pub type MukandaraTime = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Infinitism {
    NegativeInfinity,
    Finite(i128),
    PositiveInfinity,
}
