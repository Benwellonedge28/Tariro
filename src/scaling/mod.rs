//! Tariro AGI Scaling Infrastructure
//!
//! This module provides the infrastructure for scaling Tariro AGI
//! from embedded microcontrollers to planet-scale distributed systems.
//!
//! # Architecture
//!
//! The scaling infrastructure is organized into several layers:
//!
//! - **Capability Detection**: Automatic device tier identification
//! - **Resource-Aware Loading**: Lazy module loading based on device capabilities
//! - **Tiered Memory**: Hierarchical memory management (KB → GB → TB)
//! - **Adaptive Computation**: Resource-aware reasoning paths
//! - **Distributed Consensus**: Byzantine fault-tolerant agreement
//! - **Federated Learning**: Cross-device knowledge synchronization
//! - **Edge-Cloud Sync**: Hierarchical data synchronization
//!
//! # Device Tiers
//!
//! | Tier | Memory | Example Devices |
//! |------|--------|-----------------|
//! | Microcontroller | < 64KB | ARM Cortex-M0, ESP32 |
//! | Embedded | 64KB - 16MB | Raspberry Pi Pico, STM32 |
//! | Edge | 16MB - 4GB | Raspberry Pi 4, Desktop |
//! | Cloud | > 4GB | Server, Data Center |
//! | Planet | Distributed | Multi-region, Satellite |
//!
//! # Example
//!
//! ```rust
//! use tariro_agi_core::scaling::{CapabilityProfile, ResourceAwareLoader};
//!
//! // Detect device capabilities
//! let profile = CapabilityProfile::detect();
//! println!("Device tier: {:?}", profile.tier);
//!
//! // Load modules based on resources
//! let loader = ResourceAwareLoader::initialize(profile).unwrap();
//! ```

// ============================================================================
// Core Types and Traits
// ============================================================================
pub mod capability;
pub mod resource_loader;
pub mod tiered_memory;
pub mod adaptive_compute;
pub mod consensus;
pub mod federated;
pub mod edge_sync;

// ============================================================================
// Re-exports for convenience
// ============================================================================
pub use capability::{DeviceTier, CapabilityProfile, StorageType, PowerMode, FeatureFlags};
pub use resource_loader::{ResourceAwareLoader, ModuleDescriptor, ModuleTier, ModuleHandle};
pub use tiered_memory::{TieredMemory, MemoryTier, AccessPriority};
pub use adaptive_compute::{AdaptiveComputation, ComputationPath, PartitionStrategy};
pub use consensus::{DistributedConsensus, ConsensusConfig, ConsensusState, NodeId};
pub use federated::{FederatedProtocol, AggregationStrategy, PrivacyConfig};
pub use edge_sync::{EdgeCloudSync, SyncRole, SyncPolicy};

// ============================================================================
// Constants
// ============================================================================

/// Minimum memory required for essential Tariro kernel (bytes)
pub const ESSENTIAL_MEMORY_MIN: u64 = 8 * 1024; // 8KB

/// Maximum latency for tier-1 (L1) memory access (microseconds)
pub const L1_LATENCY_US: u32 = 1;

/// Maximum latency for tier-2 (L2) memory access (microseconds)
pub const L2_LATENCY_US: u32 = 10;

/// Maximum latency for tier-3 (L3) memory access (milliseconds)
pub const L3_LATENCY_MS: u32 = 100;

/// Default network timeout (seconds)
pub const DEFAULT_NETWORK_TIMEOUT: u64 = 30;

/// Minimum quorum size for consensus (n > 3f)
pub const MIN_CONSENSUS_QUORUM: usize = 4;

/// Version of the scaling infrastructure
pub const SCALING_VERSION: &str = "1.0.0";

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur in the scaling infrastructure
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalingError {
    /// Device does not meet minimum requirements
    InsufficientResources {
        required: u64,
        available: u64,
        resource: String,
    },

    /// Module failed to load
    ModuleLoadFailed {
        module: String,
        reason: String,
    },

    /// Memory allocation failed
    MemoryAllocationFailed {
        requested: u64,
        available: u64,
    },

    /// Consensus protocol error
    ConsensusError(String),

    /// Network communication error
    NetworkError(String),

    /// Synchronization error
    SyncError(String),

    /// Invalid device tier for operation
    InvalidTier {
        operation: String,
        tier: DeviceTier,
    },

    /// Feature not supported on this platform
    FeatureNotSupported(String),
}

impl std::fmt::Display for ScalingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalingError::InsufficientResources { required, available, resource } => {
                write!(
                    f,
                    "Insufficient {}: required {} bytes, available {} bytes",
                    resource, required, available
                )
            }
            ScalingError::ModuleLoadFailed { module, reason } => {
                write!(f, "Failed to load module '{}': {}", module, reason)
            }
            ScalingError::MemoryAllocationFailed { requested, available } => {
                write!(
                    f,
                    "Memory allocation failed: requested {} bytes, {} available",
                    requested, available
                )
            }
            ScalingError::ConsensusError(msg) => write!(f, "Consensus error: {}", msg),
            ScalingError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ScalingError::SyncError(msg) => write!(f, "Sync error: {}", msg),
            ScalingError::InvalidTier { operation, tier } => {
                write!(f, "Invalid tier {:?} for operation '{}'", tier, operation)
            }
            ScalingError::FeatureNotSupported(feature) => {
                write!(f, "Feature not supported: {}", feature)
            }
        }
    }
}

impl std::error::Error for ScalingError {}

// ============================================================================
// Utility Functions
// ============================================================================

/// Calculate the optimal number of threads for a given workload
pub fn optimal_thread_count(cores: usize, workload_factor: f32) -> usize {
    let base = (cores as f32 * workload_factor).ceil() as usize;
    base.max(1).min(cores * 2)
}

/// Estimate memory overhead for a given number of modules
pub fn estimate_module_overhead(module_count: usize) -> u64 {
    // Base overhead per module: 1KB
    // Plus metadata overhead: 256 bytes per module
    (module_count as u64 * 1024) + (module_count as u64 * 256)
}

/// Calculate the minimum memory needed for a specific tier
pub fn minimum_memory_for_tier(tier: DeviceTier) -> u64 {
    match tier {
        DeviceTier::Microcontroller => 8 * 1024,       // 8KB
        DeviceTier::Embedded => 64 * 1024,              // 64KB
        DeviceTier::Edge => 16 * 1024 * 1024,          // 16MB
        DeviceTier::Cloud => 512 * 1024 * 1024,        // 512MB
        DeviceTier::Planet => 1024 * 1024 * 1024,      // 1GB
    }
}

/// Check if a capability profile meets minimum requirements
pub fn meets_requirements(profile: &CapabilityProfile, min_tier: DeviceTier) -> bool {
    profile.tier >= min_tier && profile.memory_max >= minimum_memory_for_tier(min_tier)
}

// ============================================================================
// Scaling Statistics
// ============================================================================

/// Statistics about the scaling system
#[derive(Debug, Clone, Default)]
pub struct ScalingStats {
    /// Number of modules loaded
    pub modules_loaded: usize,

    /// Total memory used (bytes)
    pub memory_used: u64,

    /// Peak memory usage (bytes)
    pub memory_peak: u64,

    /// Number of consensus rounds
    pub consensus_rounds: u64,

    /// Number of federated learning rounds
    pub federated_rounds: u64,

    /// Number of sync operations
    pub sync_operations: u64,

    /// Current device tier
    pub current_tier: DeviceTier,

    /// Uptime in milliseconds
    pub uptime_ms: u64,
}

impl ScalingStats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a module load
    pub fn record_module_load(&mut self, memory: u64) {
        self.modules_loaded += 1;
        self.memory_used += memory;
        self.memory_peak = self.memory_peak.max(self.memory_used);
    }

    /// Record a consensus round
    pub fn record_consensus_round(&mut self) {
        self.consensus_rounds += 1;
    }

    /// Record a federated learning round
    pub fn record_federated_round(&mut self) {
        self.federated_rounds += 1;
    }

    /// Record a sync operation
    pub fn record_sync_operation(&mut self) {
        self.sync_operations += 1;
    }

    /// Get memory utilization percentage
    pub fn memory_utilization(&self, total: u64) -> f32 {
        if total == 0 {
            0.0
        } else {
            (self.memory_used as f32 / total as f32) * 100.0
        }
    }
}

// ============================================================================
// Module Handle
// ============================================================================

/// Handle to a loaded module
#[derive(Debug, Clone)]
pub struct ScalingModuleHandle {
    pub name: String,
    pub memory_footprint: u64,
    pub cpu_usage: f32,
    pub version: String,
    pub loaded_at_ms: u64,
}

impl ScalingModuleHandle {
    pub fn memory_footprint(&self) -> u64 {
        self.memory_footprint
    }

    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for the scaling system
#[derive(Debug, Clone)]
pub struct ScalingConfig {
    /// Target device tier (auto-detected if None)
    pub target_tier: Option<DeviceTier>,

    /// Maximum memory to use (bytes)
    pub memory_limit: Option<u64>,

    /// Maximum CPU cores to use
    pub cpu_limit: Option<usize>,

    /// Enable automatic tier selection
    pub auto_tier: bool,

    /// Enable resource monitoring
    pub enable_monitoring: bool,

    /// Enable adaptive computation
    pub enable_adaptive_compute: bool,

    /// Consensus configuration
    pub consensus: Option<ConsensusConfig>,

    /// Federated learning configuration
    pub federated: Option<FederatedConfig>,
}

impl Default for ScalingConfig {
    fn default() -> Self {
        Self {
            target_tier: None,
            memory_limit: None,
            cpu_limit: None,
            auto_tier: true,
            enable_monitoring: true,
            enable_adaptive_compute: true,
            consensus: None,
            federated: None,
        }
    }
}

impl ScalingConfig {
    /// Create a configuration for microcontrollers
    pub fn for_microcontroller() -> Self {
        Self {
            target_tier: Some(DeviceTier::Microcontroller),
            memory_limit: Some(64 * 1024),
            cpu_limit: Some(1),
            auto_tier: false,
            enable_monitoring: false,
            enable_adaptive_compute: false,
            consensus: None,
            federated: None,
        }
    }

    /// Create a configuration for embedded devices
    pub fn for_embedded() -> Self {
        Self {
            target_tier: Some(DeviceTier::Embedded),
            memory_limit: Some(16 * 1024 * 1024),
            cpu_limit: Some(4),
            auto_tier: false,
            enable_monitoring: true,
            enable_adaptive_compute: true,
            consensus: None,
            federated: None,
        }
    }

    /// Create a configuration for cloud deployment
    pub fn for_cloud() -> Self {
        Self {
            target_tier: Some(DeviceTier::Cloud),
            memory_limit: None,
            cpu_limit: None,
            auto_tier: false,
            enable_monitoring: true,
            enable_adaptive_compute: true,
            consensus: Some(ConsensusConfig::default()),
            federated: Some(FederatedConfig::default()),
        }
    }
}

/// Configuration for federated learning
#[derive(Debug, Clone)]
pub struct FederatedConfig {
    /// Minimum peers required for aggregation
    pub min_peers: usize,

    /// Aggregation strategy
    pub aggregation: AggregationStrategy,

    /// Privacy configuration
    pub privacy: PrivacyConfig,

    /// Sync frequency in seconds
    pub sync_frequency_s: u32,
}

impl Default for FederatedConfig {
    fn default() -> Self {
        Self {
            min_peers: 3,
            aggregation: AggregationStrategy::FedAvg,
            privacy: PrivacyConfig::default(),
            sync_frequency_s: 300,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_tier_ordering() {
        assert!(DeviceTier::Microcontroller < DeviceTier::Embedded);
        assert!(DeviceTier::Embedded < DeviceTier::Edge);
        assert!(DeviceTier::Edge < DeviceTier::Cloud);
        assert!(DeviceTier::Cloud < DeviceTier::Planet);
    }

    #[test]
    fn test_minimum_memory_for_tier() {
        assert_eq!(minimum_memory_for_tier(DeviceTier::Microcontroller), 8 * 1024);
        assert_eq!(minimum_memory_for_tier(DeviceTier::Embedded), 64 * 1024);
        assert_eq!(minimum_memory_for_tier(DeviceTier::Edge), 16 * 1024 * 1024);
        assert_eq!(minimum_memory_for_tier(DeviceTier::Cloud), 512 * 1024 * 1024);
        assert_eq!(minimum_memory_for_tier(DeviceTier::Planet), 1024 * 1024 * 1024);
    }

    #[test]
    fn test_scaling_stats() {
        let mut stats = ScalingStats::new();
        assert_eq!(stats.modules_loaded, 0);

        stats.record_module_load(1024);
        assert_eq!(stats.modules_loaded, 1);
        assert_eq!(stats.memory_used, 1024);

        stats.record_consensus_round();
        assert_eq!(stats.consensus_rounds, 1);

        stats.record_federated_round();
        assert_eq!(stats.federated_rounds, 1);
    }

    #[test]
    fn test_optimal_thread_count() {
        assert_eq!(optimal_thread_count(4, 1.0), 4);
        assert_eq!(optimal_thread_count(4, 0.5), 2);
        assert_eq!(optimal_thread_count(4, 2.0), 8);
        assert_eq!(optimal_thread_count(1, 1.0), 1);
    }

    #[test]
    fn test_scaling_config_presets() {
        let micro_config = ScalingConfig::for_microcontroller();
        assert_eq!(micro_config.target_tier, Some(DeviceTier::Microcontroller));
        assert!(micro_config.memory_limit.is_some());

        let cloud_config = ScalingConfig::for_cloud();
        assert_eq!(cloud_config.target_tier, Some(DeviceTier::Cloud));
        assert!(cloud_config.consensus.is_some());
    }

    #[test]
    fn test_estimate_module_overhead() {
        assert_eq!(estimate_module_overhead(0), 0);
        assert_eq!(estimate_module_overhead(1), 1024 + 256);
        assert_eq!(estimate_module_overhead(10), 10 * 1024 + 10 * 256);
    }
}
