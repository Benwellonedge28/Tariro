// Tariro AGI - Serde Utilities
// Custom serialization for types that don't implement Serialize/Deserialize natively
//
// Note: chrono DateTime and std::time::Duration already implement Serialize/Deserialize
// natively via serde, so no custom implementations are needed here.

use crate::prelude::*;
