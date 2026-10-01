//! Resource-Aware Module Loader
//!
//! This module provides lazy loading of Tariro modules based on
//! device capabilities and available resources.
//!
//! # Features
//!
//! - Lazy module initialization
//! - Memory-aware loading
//! - Priority-based module loading
//! - Conditional compilation support
//! - Module dependency resolution

use crate::scaling::capability::{CapabilityProfile, DeviceTier, FeatureFlags};
use crate::scaling::{ScalingError, ScalingModuleHandle, SCALING_VERSION};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

// ============================================================================
// Module Types
// ============================================================================

/// Module tier requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleTier {
    /// Essential: Always loaded (kernel of intelligence)
    Essential = 0,

    /// Standard: Default for edge/cloud
    Standard = 1,

    /// Extended: Large memory, full capabilities
    Extended = 2,

    /// Experimental: Research features
    Experimental = 3,
}

/// Module descriptor for lazy loading
#[derive(Debug, Clone)]
pub struct ModuleDescriptor {
    /// Module name
    pub name: &'static str,

    /// Module version
    pub version: &'static str,

    /// Minimum device tier required
    pub min_tier: DeviceTier,

    /// Required memory (bytes)
    pub required_memory: u64,

    /// Required memory when loaded (bytes)
    pub memory_footprint: u64,

    /// Required CPU frequency (MHz)
    pub required_cpu_mhz: f32,

    /// Required features
    pub required_features: FeatureFlags,

    /// Is optional
    pub optional: bool,

    /// Load priority (lower = earlier)
    pub load_priority: u8,

    /// Dependencies on other modules
    pub dependencies: &'static [&'static str],

    /// Category
    pub category: ModuleCategory,
}

/// Module category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleCategory {
    /// Core kernel modules
    Kernel,

    /// Knowledge representation
    Knowledge,

    /// Cognitive functions
    Cognition,

    /// Learning systems
    Learning,

    /// Safety and alignment
    Safety,

    /// Network and distribution
    Network,

    /// Storage and persistence
    Storage,

    /// Utilities
    Utility,
}

/// Module initialization function type
pub type ModuleInitFn = fn(&CapabilityProfile) -> Result<ScalingModuleHandle, ScalingError>;

/// Module handle with metadata
#[derive(Debug, Clone)]
pub struct ModuleHandle {
    pub name: String,
    pub version: String,
    pub memory_footprint: u64,
    pub cpu_usage: f32,
    pub loaded: bool,
    pub tier: ModuleTier,
}

/// Module registration entry
struct ModuleEntry {
    descriptor: ModuleDescriptor,
    init_fn: Option<ModuleInitFn>,
    state: ModuleState,
    handle: Option<ScalingModuleHandle>,
}

/// Module loading state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    /// Not yet loaded
    Unloaded,

    /// Loading in progress
    Loading,

    /// Successfully loaded
    Loaded,

    /// Failed to load
    Failed,

    /// Disabled (not available for this tier)
    Disabled,
}

/// Load result
#[derive(Debug, Clone)]
pub struct LoadResult {
    pub module: String,
    pub success: bool,
    pub memory_used: u64,
    pub error: Option<String>,
}

/// Module load summary
#[derive(Debug, Clone, Default)]
pub struct LoadSummary {
    pub total_modules: usize,
    pub loaded_modules: usize,
    pub failed_modules: usize,
    pub skipped_modules: usize,
    pub disabled_modules: usize,
    pub total_memory_used: u64,
    pub results: Vec<LoadResult>,
}

// ============================================================================
// Resource-Aware Loader
// ============================================================================

/// Resource-aware module loader
pub struct ResourceAwareLoader {
    /// Device capability profile
    pub profile: CapabilityProfile,

    /// Registered modules
    modules: HashMap<&'static str, ModuleEntry>,

    /// Currently loaded modules
    loaded_modules: HashMap<&'static str, ScalingModuleHandle>,

    /// Memory budget (bytes)
    memory_budget: u64,

    /// CPU budget (cores as f32)
    cpu_budget: f32,

    /// Memory currently used
    memory_used: u64,

    /// CPU currently used
    cpu_used: f32,

    /// Loading strategy
    strategy: LoadingStrategy,

    /// Strict mode (fail on missing optional)
    strict: bool,
}

/// Loading strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingStrategy {
    /// Load only essential modules
    Minimal,

    /// Load essential + requested modules
    Selective,

    /// Load standard modules
    Standard,

    /// Load all modules
    Full,

    /// Load distributed modules
    Distributed,
}

impl Default for LoadingStrategy {
    fn default() -> Self {
        LoadingStrategy::Standard
    }
}

impl ResourceAwareLoader {
    /// Create a new loader with the given profile
    pub fn new(profile: CapabilityProfile) -> Self {
        let strategy = profile.tier.loading_strategy();
        let memory_budget = profile.available_memory();

        Self {
            profile,
            modules: HashMap::new(),
            loaded_modules: HashMap::new(),
            memory_budget,
            cpu_budget: 0.0, // Set based on profile
            memory_used: 0,
            cpu_used: 0.0,
            strategy,
            strict: false,
        }
    }

    /// Initialize with default configuration
    pub fn initialize(profile: CapabilityProfile) -> Result<Self, ScalingError> {
        let mut loader = Self::new(profile);
        loader.register_standard_modules();
        loader.load_modules()?;
        Ok(loader)
    }

    /// Create with a specific strategy
    pub fn with_strategy(profile: CapabilityProfile, strategy: LoadingStrategy) -> Self {
        let mut loader = Self::new(profile);
        loader.strategy = strategy;
        loader.register_standard_modules();
        loader
    }

    /// Register the standard Tariro modules
    pub fn register_standard_modules(&mut self) {
        // Kernel modules (Essential)
        self.register_module(ModuleDescriptor {
            name: "core_kernel",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Microcontroller,
            required_memory: 8 * 1024,
            memory_footprint: 4 * 1024,
            required_cpu_mhz: 1.0,
            required_features: FeatureFlags::EMPTY,
            optional: false,
            load_priority: 0,
            dependencies: &[],
            category: ModuleCategory::Kernel,
        });

        self.register_module(ModuleDescriptor {
            name: "symbolic_reasoning",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Microcontroller,
            required_memory: 16 * 1024,
            memory_footprint: 8 * 1024,
            required_cpu_mhz: 10.0,
            required_features: FeatureFlags::SYMBOLIC,
            optional: false,
            load_priority: 1,
            dependencies: &["core_kernel"],
            category: ModuleCategory::Knowledge,
        });

        // Standard modules
        self.register_module(ModuleDescriptor {
            name: "knowledge_base",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Embedded,
            required_memory: 64 * 1024,
            memory_footprint: 32 * 1024,
            required_cpu_mhz: 20.0,
            required_features: FeatureFlags::EMPTY,
            optional: true,
            load_priority: 2,
            dependencies: &["core_kernel", "symbolic_reasoning"],
            category: ModuleCategory::Knowledge,
        });

        self.register_module(ModuleDescriptor {
            name: "language_processing",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Embedded,
            required_memory: 256 * 1024,
            memory_footprint: 128 * 1024,
            required_cpu_mhz: 50.0,
            required_features: FeatureFlags::LANGUAGE,
            optional: true,
            load_priority: 3,
            dependencies: &["core_kernel", "symbolic_reasoning"],
            category: ModuleCategory::Cognition,
        });

        self.register_module(ModuleDescriptor {
            name: "planning_engine",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Embedded,
            required_memory: 128 * 1024,
            memory_footprint: 64 * 1024,
            required_cpu_mhz: 40.0,
            required_features: FeatureFlags::EMPTY,
            optional: true,
            load_priority: 4,
            dependencies: &["core_kernel", "symbolic_reasoning"],
            category: ModuleCategory::Cognition,
        });

        // Extended modules
        self.register_module(ModuleDescriptor {
            name: "consciousness_engine",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Edge,
            required_memory: 4 * 1024 * 1024,
            memory_footprint: 2 * 1024 * 1024,
            required_cpu_mhz: 200.0,
            required_features: FeatureFlags::CONSCIOUSNESS,
            optional: true,
            load_priority: 5,
            dependencies: &["core_kernel", "symbolic_reasoning", "knowledge_base"],
            category: ModuleCategory::Cognition,
        });

        self.register_module(ModuleDescriptor {
            name: "temporal_reasoning",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Edge,
            required_memory: 512 * 1024,
            memory_footprint: 256 * 1024,
            required_cpu_mhz: 100.0,
            required_features: FeatureFlags::EMPTY,
            optional: true,
            load_priority: 6,
            dependencies: &["core_kernel", "symbolic_reasoning"],
            category: ModuleCategory::Cognition,
        });

        self.register_module(ModuleDescriptor {
            name: "social_cognition",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Edge,
            required_memory: 1 * 1024 * 1024,
            memory_footprint: 512 * 1024,
            required_cpu_mhz: 150.0,
            required_features: FeatureFlags::EMPTY,
            optional: true,
            load_priority: 7,
            dependencies: &["core_kernel", "symbolic_reasoning", "language_processing"],
            category: ModuleCategory::Cognition,
        });

        self.register_module(ModuleDescriptor {
            name: "creativity_engine",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Edge,
            required_memory: 2 * 1024 * 1024,
            memory_footprint: 1 * 1024 * 1024,
            required_cpu_mhz: 180.0,
            required_features: FeatureFlags::EMPTY,
            optional: true,
            load_priority: 8,
            dependencies: &["core_kernel", "symbolic_reasoning", "knowledge_base"],
            category: ModuleCategory::Cognition,
        });

        // Network modules
        self.register_module(ModuleDescriptor {
            name: "network_base",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Embedded,
            required_memory: 64 * 1024,
            memory_footprint: 32 * 1024,
            required_cpu_mhz: 30.0,
            required_features: FeatureFlags::EMPTY,
            optional: true,
            load_priority: 10,
            dependencies: &["core_kernel"],
            category: ModuleCategory::Network,
        });

        self.register_module(ModuleDescriptor {
            name: "distributed_consensus",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Cloud,
            required_memory: 64 * 1024 * 1024,
            memory_footprint: 32 * 1024 * 1024,
            required_cpu_mhz: 500.0,
            required_features: FeatureFlags::DISTRIBUTED,
            optional: true,
            load_priority: 15,
            dependencies: &["core_kernel", "network_base"],
            category: ModuleCategory::Network,
        });

        // Safety modules
        self.register_module(ModuleDescriptor {
            name: "safety_core",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Microcontroller,
            required_memory: 8 * 1024,
            memory_footprint: 4 * 1024,
            required_cpu_mhz: 5.0,
            required_features: FeatureFlags::EMPTY,
            optional: false,
            load_priority: 0,
            dependencies: &["core_kernel"],
            category: ModuleCategory::Safety,
        });

        self.register_module(ModuleDescriptor {
            name: "value_alignment",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Embedded,
            required_memory: 32 * 1024,
            memory_footprint: 16 * 1024,
            required_cpu_mhz: 20.0,
            required_features: FeatureFlags::EMPTY,
            optional: true,
            load_priority: 9,
            dependencies: &["core_kernel", "safety_core"],
            category: ModuleCategory::Safety,
        });

        // Quantum security (Edge+)
        self.register_module(ModuleDescriptor {
            name: "quantum_crypto",
            version: SCALING_VERSION,
            min_tier: DeviceTier::Edge,
            required_memory: 1 * 1024 * 1024,
            memory_footprint: 512 * 1024,
            required_cpu_mhz: 100.0,
            required_features: FeatureFlags::QUANTUM_SAFE,
            optional: true,
            load_priority: 11,
            dependencies: &["core_kernel"],
            category: ModuleCategory::Safety,
        });
    }

    /// Register a module
    pub fn register_module(&mut self, descriptor: ModuleDescriptor) {
        let name = descriptor.name;
        self.modules.insert(
            name,
            ModuleEntry {
                descriptor,
                init_fn: None,
                state: ModuleState::Unloaded,
                handle: None,
            },
        );
    }

    /// Register a module with initialization function
    pub fn register_module_with_init(
        &mut self,
        descriptor: ModuleDescriptor,
        init_fn: ModuleInitFn,
    ) {
        let name = descriptor.name;
        self.modules.insert(
            name,
            ModuleEntry {
                descriptor,
                init_fn: Some(init_fn),
                state: ModuleState::Unloaded,
                handle: None,
            },
        );
    }

    /// Load modules based on current strategy and resources
    pub fn load_modules(&mut self) -> Result<LoadSummary, ScalingError> {
        let mut summary = LoadSummary::default();

        // First, resolve dependencies
        self.resolve_dependencies()?;

        // Get modules sorted by priority
        let mut sorted: Vec<_> = self.modules.values_mut().collect();
        sorted.sort_by_key(|m| m.descriptor.load_priority);

        for entry in sorted {
            summary.total_modules += 1;

            // Check if module can be loaded
            let can_load = self.can_load_module(&entry.descriptor);

            if !can_load.0 {
                if entry.descriptor.optional {
                    entry.state = ModuleState::Disabled;
                    summary.skipped_modules += 1;
                } else {
                    entry.state = ModuleState::Failed;
                    summary.failed_modules += 1;
                    summary.results.push(LoadResult {
                        module: entry.descriptor.name.to_string(),
                        success: false,
                        memory_used: 0,
                        error: Some(can_load.1),
                    });

                    if self.strict {
                        return Err(ScalingError::ModuleLoadFailed {
                            module: entry.descriptor.name.to_string(),
                            reason: can_load.1,
                        });
                    }
                }
                continue;
            }

            // Load the module
            entry.state = ModuleState::Loading;

            let handle = self.load_module_internal(entry)?;

            if let Some(h) = handle {
                entry.state = ModuleState::Loaded;
                entry.handle = Some(h.clone());
                self.loaded_modules.insert(entry.descriptor.name, h.clone());
                self.memory_used += h.memory_footprint;
                self.cpu_used += h.cpu_usage;

                summary.loaded_modules += 1;
                summary.total_memory_used += h.memory_footprint;
                summary.results.push(LoadResult {
                    module: entry.descriptor.name.to_string(),
                    success: true,
                    memory_used: h.memory_footprint,
                    error: None,
                });
            } else {
                entry.state = ModuleState::Failed;
                summary.failed_modules += 1;
                summary.results.push(LoadResult {
                    module: entry.descriptor.name.to_string(),
                    success: false,
                    memory_used: 0,
                    error: Some("Load returned None".to_string()),
                });
            }
        }

        Ok(summary)
    }

    /// Check if a module can be loaded
    fn can_load_module(&self, desc: &ModuleDescriptor) -> (bool, String) {
        // Check tier requirement
        if self.profile.tier < desc.min_tier {
            return (
                false,
                format!(
                    "Device tier {:?} below required {:?}",
                    self.profile.tier, desc.min_tier
                ),
            );
        }

        // Check memory
        if self.memory_budget - self.memory_used < desc.required_memory {
            return (
                false,
                format!(
                    "Insufficient memory: need {} bytes, {} available",
                    desc.required_memory,
                    self.memory_budget - self.memory_used
                ),
            );
        }

        // Check features
        if !self.profile.features.has(desc.required_features) {
            return (
                false,
                format!(
                    "Missing required features: {:?}",
                    desc.required_features
                ),
            );
        }

        // Check strategy
        let tier_level = self.profile.tier as u8;
        let required_level = match desc.category {
            ModuleCategory::Kernel => 0,
            ModuleCategory::Safety => 1,
            ModuleCategory::Knowledge => 2,
            ModuleCategory::Cognition if desc.load_priority <= 5 => 2,
            ModuleCategory::Cognition => 3,
            ModuleCategory::Learning => 3,
            ModuleCategory::Network => 2,
            ModuleCategory::Storage => 2,
            ModuleCategory::Utility => 4,
        };

        if tier_level < required_level {
            return (
                false,
                format!(
                    "Device tier {:?} not sufficient for {} category",
                    self.profile.tier, desc.category
                ),
            );
        }

        (true, String::new())
    }

    /// Load a single module internally
    fn load_module_internal(&mut self, entry: &mut ModuleEntry) -> Result<Option<ScalingModuleHandle>, ScalingError> {
        // If we have an init function, use it
        if let Some(init_fn) = entry.init_fn {
            match init_fn(&self.profile) {
                Ok(handle) => Ok(Some(handle)),
                Err(e) => Err(e),
            }
        } else {
            // Create a default handle
            Ok(Some(ScalingModuleHandle {
                name: entry.descriptor.name.to_string(),
                memory_footprint: entry.descriptor.memory_footprint,
                cpu_usage: entry.descriptor.required_cpu_mhz,
                version: entry.descriptor.version.to_string(),
                loaded_at_ms: 0, // Would be timestamp
            }))
        }
    }

    /// Resolve module dependencies
    fn resolve_dependencies(&self) -> Result<(), ScalingError> {
        // Check for circular dependencies
        for (name, entry) in &self.modules {
            let mut visited = HashSet::new();
            if self.has_circular_dependency(name, entry, &mut visited) {
                return Err(ScalingError::ModuleLoadFailed {
                    module: name.to_string(),
                    reason: "Circular dependency detected".to_string(),
                });
            }
        }
        Ok(())
    }

    /// Check for circular dependencies
    fn has_circular_dependency<'a>(
        &self,
        module: &'a str,
        entry: &'a ModuleEntry,
        visited: &mut HashSet<&'a str>,
    ) -> bool {
        if visited.contains(module) {
            return true;
        }
        visited.insert(module);

        for dep in entry.dependencies {
            if let Some(dep_entry) = self.modules.get(dep) {
                if self.has_circular_dependency(dep, dep_entry, visited) {
                    return true;
                }
            }
        }

        visited.remove(module);
        false
    }

    /// Get a loaded module handle
    pub fn get_module(&self, name: &str) -> Option<&ScalingModuleHandle> {
        self.loaded_modules.get(name)
    }

    /// Check if a module is loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded_modules.contains_key(name)
    }

    /// Get all loaded modules
    pub fn loaded_modules(&self) -> Vec<&ScalingModuleHandle> {
        self.loaded_modules.values().collect()
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> MemoryStats {
        MemoryStats {
            budget: self.memory_budget,
            used: self.memory_used,
            available: self.memory_budget - self.memory_used,
            utilization_pct: if self.memory_budget > 0 {
                (self.memory_used as f32 / self.memory_budget as f32) * 100.0
            } else {
                0.0
            },
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub budget: u64,
    pub used: u64,
    pub available: u64,
    pub utilization_pct: f32,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_creation() {
        let profile = CapabilityProfile::for_embedded();
        let loader = ResourceAwareLoader::new(profile);

        assert_eq!(loader.loaded_modules.len(), 0);
        assert_eq!(loader.memory_used, 0);
    }

    #[test]
    fn test_module_registration() {
        let profile = CapabilityProfile::for_microcontroller();
        let mut loader = ResourceAwareLoader::new(profile);

        loader.register_module(ModuleDescriptor {
            name: "test_module",
            version: "1.0.0",
            min_tier: DeviceTier::Microcontroller,
            required_memory: 1024,
            memory_footprint: 512,
            required_cpu_mhz: 10.0,
            required_features: FeatureFlags::EMPTY,
            optional: false,
            load_priority: 0,
            dependencies: &[],
            category: ModuleCategory::Kernel,
        });

        assert!(loader.modules.contains_key("test_module"));
    }

    #[test]
    fn test_standard_modules() {
        let profile = CapabilityProfile::for_edge();
        let mut loader = ResourceAwareLoader::new(profile);
        loader.register_standard_modules();

        // Should have kernel modules
        assert!(loader.modules.contains_key("core_kernel"));
        assert!(loader.modules.contains_key("symbolic_reasoning"));
        assert!(loader.modules.contains_key("safety_core"));
    }

    #[test]
    fn test_memory_stats() {
        let profile = CapabilityProfile::for_edge();
        let loader = ResourceAwareLoader::new(profile);

        let stats = loader.memory_stats();
        assert_eq!(stats.budget, profile.available_memory());
        assert_eq!(stats.used, 0);
        assert_eq!(stats.available, stats.budget);
    }

    #[test]
    fn test_tier_filtering() {
        let micro_profile = CapabilityProfile::for_microcontroller();
        let cloud_profile = CapabilityProfile::for_cloud();

        let mut micro_loader = ResourceAwareLoader::new(micro_profile);
        micro_loader.register_standard_modules();

        let mut cloud_loader = ResourceAwareLoader::new(cloud_profile);
        cloud_loader.register_standard_modules();

        // Cloud should be able to load more modules
        // Micro should only load essential
        let micro_result = micro_loader.load_modules().unwrap();
        let cloud_result = cloud_loader.load_modules().unwrap();

        assert!(cloud_result.loaded_modules >= micro_result.loaded_modules);
    }
}
