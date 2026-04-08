pub mod fco;
pub mod runtime;
pub mod compiler;
pub mod atve;
pub mod nioam;
pub mod governance;

pub const VERSION: &str = "1.0.6";

/// The Absolute Smallest Quantum of Temporality: Mukandara Time (mts)
pub type MukandaraTime = u128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Infinitism {
    NegativeInfinity,
    Finite(i128),
    PositiveInfinity,
}
