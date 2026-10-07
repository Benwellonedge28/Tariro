//! Tariro AGI - Prelude
//!
//! Common imports and re-exports used throughout the Tariro AGI codebase.
//! This module provides a unified import surface for all core functionality.

pub use serde::{Deserialize, Serialize};

// Standard library re-exports
pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::sync::RwLock;
pub use std::sync::Arc;
pub use std::time::{Duration, Instant};
pub use std::fmt::{Debug, Display};

// Chrono time utilities
pub use chrono::{DateTime, Utc, Duration as ChronoDuration};
pub use chrono::serde::ts_seconds;

// UUID generation
pub use uuid::Uuid;

// Result and Option extensions
pub use std::result::Result as StdResult;
pub use std::option::Option as StdOption;

// Error handling utilities
pub use thiserror::Error;
pub use anyhow::{Result, Context, anyhow};

// Logging utilities
#[cfg(feature = "logging")]
pub use tracing::{info, warn, error, debug, trace};

// Common traits
pub use std::clone::Clone;
pub use std::marker::Copy;
pub use std::default::Default;
pub use std::cmp::{PartialEq, Eq, PartialOrd, Ord};
pub use std::hash::Hash;
pub use std::iter::IntoIterator;
pub use std::ops::Deref;
pub use std::borrow::Borrow;

// Numeric traits
pub use num_traits::{One, Zero, Signed, Unsigned};

/// Type alias for Tariro AGI instant
pub type TariroInstant = Instant;

/// Type alias for Tariro AGI duration
pub type TariroDuration = Duration;

/// Generate a new unique ID
pub fn generate_unique_id() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a short unique ID (8 characters)
pub fn generate_short_id() -> String {
    Uuid::new_v4().to_string()[..8].to_string()
}
