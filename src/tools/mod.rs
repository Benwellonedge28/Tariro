//! # Tariro AGI Tools
//!
//! Tools that extend Tariro AGI's capabilities by interacting with
//! external systems like IDEs, terminals, and LSP servers.
//!
//! ## Available Tools
//!
//! - **IDE Tool**: File management, terminal execution, LSP integration

pub mod ide;

pub use ide::{
    IDETool, IDEToolConfig, IDEToolCapabilities, IDEToolResult,
    IDEAction, IDEToolResponse,
};
