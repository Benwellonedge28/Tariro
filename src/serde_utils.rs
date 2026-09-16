// Tariro AGI - Serde Utilities
// Custom serialization for types that don't implement Serialize/Deserialize natively

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as DeError;
use std::time::{Duration, Instant};

// ============================================================================
// Instant Serialization
// ============================================================================

/// Wrapper for std::time::Instant that implements serde
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerializableInstant {
    /// Nanoseconds since an arbitrary epoch
    pub nanos_since_epoch: u64,
}

impl SerializableInstant {
    pub fn from_instant(instant: &Instant) -> Self {
        // Since Instant doesn't provide direct epoch conversion,
        // we use a reference point approach
        // For serialization, we store a zero-based offset
        Self {
            nanos_since_epoch: instant.elapsed().as_nanos() as u64,
        }
    }

    pub fn to_instant(&self) -> Instant {
        // We can't perfectly reconstruct Instant, so we return a new Instant
        // This is a limitation of Instant's design
        Instant::now() - Duration::from_nanos(self.nanos_since_epoch)
    }
}

impl Serialize for SerializableInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.nanos_since_epoch)
    }
}

impl<'de> Deserialize<'de> for SerializableInstant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let nanos = u64::deserialize(deserializer)?;
        Ok(Self { nanos_since_epoch: nanos })
    }
}

impl From<Instant> for SerializableInstant {
    fn from(instant: Instant) -> Self {
        Self::from_instant(&instant)
    }
}

impl From<SerializableInstant> for Instant {
    fn from(wrapper: SerializableInstant) -> Self {
        wrapper.to_instant()
    }
}

// ============================================================================
// Duration Serialization
// ============================================================================

// Duration already implements Serialize/Deserialize natively via serde,
// so no custom implementation is needed.
