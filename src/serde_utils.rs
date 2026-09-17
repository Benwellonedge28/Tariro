// Tariro AGI - Serde Utilities
// Custom serialization for types that don't implement Serialize/Deserialize natively

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// DateTime Serialization (chrono types already implement serde)
// ============================================================================

// Duration already implements Serialize/Deserialize natively via serde,
// so no custom implementation is needed.
