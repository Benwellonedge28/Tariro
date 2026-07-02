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

impl PartialOrd for Infinitism {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Infinitism {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Infinitism::NegativeInfinity, Infinitism::NegativeInfinity) => std::cmp::Ordering::Equal,
            (Infinitism::NegativeInfinity, _) => std::cmp::Ordering::Less,
            (_, Infinitism::NegativeInfinity) => std::cmp::Ordering::Greater,
            (Infinitism::PositiveInfinity, Infinitism::PositiveInfinity) => std::cmp::Ordering::Equal,
            (Infinitism::PositiveInfinity, _) => std::cmp::Ordering::Greater,
            (_, Infinitism::PositiveInfinity) => std::cmp::Ordering::Less,
            (Infinitism::Finite(a), Infinitism::Finite(b)) => a.cmp(b),
        }
    }
}
