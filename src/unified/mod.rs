//! # TUMC-AGI Unified Orchestrator
//!
//! This is the central integration layer that unifies all TUMC compiler modules
//! with AGI cognitive systems under the TariroAGI entry point.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────┐
//! │                        TARIRO AGI                                   │
//! │                    (Main Entry Point)                                │
//! ├─────────────────────────────────────────────────────────────────────┤
//! │  ┌──────────────────────────────────────────────────────────────┐   │
//! │  │            TUMC-AGI UNIFIED ORCHESTRATOR                    │   │
//! │  │  ┌─────────────┐ ┌─────────────┐ ┌────────────────────────┐│   │
//! │  │  │ Cognitive   │ │  Meta-      │ │    Universal          ││   │
//! │  │  │ Compiler   │ │  Learning   │ │    Compiler          ││   │
//! │  │  │ Engine     │ │  Engine     │ │    Engine           ││   │
//! │  │  └─────────────┘ └─────────────┘ └────────────────────────┘│   │
//! │  ├──────────────────────────────────────────────────────────────┤   │
//! │  │  ┌─────────────┐ ┌─────────────┐ ┌────────────────────────┐│   │
//! │  │  │   AGI      │ │  Planning   │ │      Learning          ││   │
//! │  │  │   Core     │ │  Engine    │ │      Systems          ││   │
//! │  │  └─────────────┘ └─────────────┘ └────────────────────────┘│   │
//! │  ├──────────────────────────────────────────────────────────────┤   │
//! │  │  ┌──────────────────────────────────────────────────────┐ │   │
//! │  │  │              TUMC COMPILER MODULES                   │ │   │
//! │  │  │  IR | Optimization | Analysis | Type System | GC     │ │   │
//! │  │  │  Runtime | Targets | Tooling | Verification | Spec   │ │   │
//! │  │  └──────────────────────────────────────────────────────┘ │   │
//! │  └──────────────────────────────────────────────────────────────┘   │
//! └─────────────────────────────────────────────────────────────────────┘
//! ```

pub mod orchestrator;
pub mod cognitive_core;
pub mod compiler_integration;
pub mod unified_memory;
pub mod engine_registry;
pub mod tariro_integration;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

// Re-export main types
pub use orchestrator::TariroUnifiedOrchestrator;
pub use cognitive_core::CognitiveCore;
pub use compiler_integration::CompilerIntegration;
pub use unified_memory::UnifiedMemory;

// Re-export integration system types
pub use tariro_integration::{
    TariroIntegration,
    EngineRegistry,
    EngineMetadata,
    EngineCategory,
    EngineStatistics,
    SystemStatus as IntegrationSystemStatus,
    SystemHealth,
    CapabilityProfile,
    CapabilityLevel,
};
pub use engine_registry::{EngineMetadata, EngineCategory, EngineStatistics};

use crate::tumc_compiler::ir::Module;
use crate::tumc_compiler::agi_integration::{AgiInsight, AgiCompilerConfig};
use crate::tumc_compiler::optimization::OptLevel;

/// Unified TUMC-AGI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedConfig {
    /// Enable cognitive compiler
    pub enable_cognitive_compiler: bool,

    /// Enable meta-learning
    pub enable_meta_learning: bool,

    /// Enable AGI reasoning
    pub enable_agi_reasoning: bool,

    /// Enable self-modification
    pub enable_self_modification: bool,

    /// Default optimization level
    pub default_optimization: String,

    /// Target platform
    pub target_platform: String,

    /// Maximum reasoning depth
    pub max_reasoning_depth: usize,

    /// Memory size for unified memory (MB)
    pub memory_size_mb: usize,
}

impl Default for UnifiedConfig {
    fn default() -> Self {
        Self {
            enable_cognitive_compiler: true,
            enable_meta_learning: true,
            enable_agi_reasoning: true,
            enable_self_modification: true,
            default_optimization: "O3".to_string(),
            target_platform: "wasm".to_string(),
            max_reasoning_depth: 10,
            memory_size_mb: 1024,
        }
    }
}

/// System status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub version: String,
    pub uptime_mts: u128,
    pub components: HashMap<String, ComponentStatus>,
    pub metrics: SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    pub name: String,
    pub initialized: bool,
    pub healthy: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub compilations_completed: u64,
    pub insights_generated: u64,
    pub optimizations_applied: u64,
    pub memory_usage_mb: usize,
    pub active_agents: usize,
}

/// Intent processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentResult {
    pub success: bool,
    pub output: String,
    pub insights: Vec<AgiInsight>,
    pub metrics: ProcessingMetrics,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingMetrics {
    pub compilation_time_ms: u64,
    pub reasoning_steps: usize,
    pub optimizations_applied: usize,
    pub confidence: f64,
}

/// Create a new unified TUMC-AGI system
pub fn create_tariro_unified(config: UnifiedConfig) -> TariroUnifiedOrchestrator {
    TariroUnifiedOrchestrator::new(config)
}
