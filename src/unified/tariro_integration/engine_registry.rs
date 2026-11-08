//! Engine Registry Module
//! Provides centralized registration and management of all Tariro AGI engines.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Engine category for organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EngineCategory {
    /// Core AGI engines
    Core,
    /// Cognitive processing engines
    Cognitive,
    /// Learning and adaptation engines
    Learning,
    /// Reasoning engines
    Reasoning,
    /// Perception engines
    Perception,
    /// Action engines
    Action,
    /// Meta-cognitive engines
    MetaCognitive,
    /// Memory systems
    Memory,
    /// Language processing
    Language,
    /// Creative systems
    Creative,
    /// Ethical systems
    Ethical,
    /// Tool use engines
    Tool,
    /// Compiler and code generation
    Compiler,
    /// Optimization engines
    Optimization,
    /// Verification systems
    Verification,
    /// Other engines
    Other,
}

/// Engine metadata for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMetadata {
    /// Unique engine identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Engine category
    pub category: EngineCategory,
    /// Engine version
    pub version: String,
    /// Brief description
    pub description: String,
    /// Capabilities provided by this engine
    pub capabilities: Vec<String>,
    /// Dependencies on other engines
    pub dependencies: Vec<String>,
    /// Resource requirements (estimated)
    pub memory_estimate: usize,
    /// Whether engine is enabled by default
    pub enabled_by_default: bool,
    /// Priority for initialization (higher = earlier)
    pub init_priority: u32,
}

/// Engine execution statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EngineStatistics {
    /// Number of times engine was invoked
    pub invocation_count: u64,
    /// Total execution time
    pub total_execution_time: Duration,
    /// Number of successful executions
    pub success_count: u64,
    /// Number of failed executions
    pub failure_count: u64,
    /// Average execution time per invocation
    pub avg_execution_time: Duration,
    /// Peak memory usage observed
    pub peak_memory_usage: usize,
    /// Last invocation timestamp
    pub last_invocation: Option<Instant>,
    /// Timestamps of recent invocations
    pub recent_invocations: Vec<Instant>,
}

impl EngineStatistics {
    /// Create new statistics tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Record an invocation
    pub fn record_invocation(&mut self, duration: Duration, success: bool, memory_used: usize) {
        self.invocation_count += 1;
        self.total_execution_time += duration;
        self.last_invocation = Some(Instant::now());

        if success {
            self.success_count += 1;
        } else {
            self.failure_count += 1;
        }

        // Update average
        self.avg_execution_time = self.total_execution_time / self.invocation_count;

        // Update peak memory
        if memory_used > self.peak_memory_usage {
            self.peak_memory_usage = memory_used;
        }

        // Keep recent invocations (last 100)
        self.recent_invocations.push(self.last_invocation.unwrap());
        if self.recent_invocations.len() > 100 {
            self.recent_invocations.remove(0);
        }
    }

    /// Get success rate
    pub fn success_rate(&self) -> f64 {
        if self.invocation_count == 0 {
            return 0.0;
        }
        self.success_count as f64 / self.invocation_count as f64
    }
}

/// Engine registration entry
struct EngineEntry {
    metadata: EngineMetadata,
    statistics: EngineStatistics,
    enabled: bool,
    engine: Option<Arc<dyn std::any::Any + Send + Sync>>,
}

/// Engine Registry - Central management for all engines
pub struct EngineRegistry {
    /// Registered engines
    engines: RwLock<HashMap<String, EngineEntry>>,
    /// Engines by category
    categories: RwLock<HashMap<EngineCategory, HashSet<String>>>,
    /// Capability index
    capability_index: RwLock<HashMap<String, HashSet<String>>>,
}

impl EngineRegistry {
    /// Create new engine registry
    pub fn new() -> Self {
        Self {
            engines: RwLock::new(HashMap::new()),
            categories: RwLock::new(HashMap::new()),
            capability_index: RwLock::new(HashMap::new()),
        }
    }

    /// Register an engine
    pub fn register(&self, metadata: EngineMetadata) -> Result<(), EngineRegistryError> {
        let mut engines = self.engines.write().map_err(|_| EngineRegistryError::LockError)?;
        let mut categories = self.categories.write().map_err(|_| EngineRegistryError::LockError)?;
        let mut capability_index = self.capability_index.write().map_err(|_| EngineRegistryError::LockError)?;

        // Check for duplicate
        if engines.contains_key(&metadata.id) {
            return Err(EngineRegistryError::AlreadyRegistered(metadata.id));
        }

        // Create entry
        let entry = EngineEntry {
            metadata: metadata.clone(),
            statistics: EngineStatistics::new(),
            enabled: metadata.enabled_by_default,
            engine: None,
        };

        // Register in main index
        engines.insert(metadata.id.clone(), entry);

        // Register in category index
        categories
            .entry(metadata.category.clone())
            .or_default()
            .insert(metadata.id.clone());

        // Register in capability index
        for capability in &metadata.capabilities {
            capability_index
                .entry(capability.clone())
                .or_default()
                .insert(metadata.id.clone());
        }

        Ok(())
    }

    /// Get engine metadata
    pub fn get_metadata(&self, id: &str) -> Option<EngineMetadata> {
        self.engines
            .read()
            .ok()
            .and_then(|engines| engines.get(id).map(|e| e.metadata.clone()))
    }

    /// Get all engines in a category
    pub fn get_by_category(&self, category: EngineCategory) -> Vec<EngineMetadata> {
        let engines = self.engines.read().ok();
        let categories = self.categories.read().ok();

        match (engines, categories) {
            (Some(engines), Some(categories)) => {
                categories
                    .get(&category)
                    .map(|ids| {
                        ids.iter()
                            .filter_map(|id| engines.get(id).map(|e| e.metadata.clone()))
                            .collect()
                    })
                    .unwrap_or_default()
            }
            _ => Vec::new(),
        }
    }

    /// Get engines with capability
    pub fn get_by_capability(&self, capability: &str) -> Vec<EngineMetadata> {
        let engines = self.engines.read().ok();
        let capability_index = self.capability_index.read().ok();

        match (engines, capability_index) {
            (Some(engines), Some(capability_index)) => {
                capability_index
                    .get(capability)
                    .map(|ids| {
                        ids.iter()
                            .filter_map(|id| engines.get(id).map(|e| e.metadata.clone()))
                            .collect()
                    })
                    .unwrap_or_default()
            }
            _ => Vec::new(),
        }
    }

    /// Get engine statistics
    pub fn get_statistics(&self, id: &str) -> Option<EngineStatistics> {
        self.engines
            .read()
            .ok()
            .and_then(|engines| engines.get(id).map(|e| e.statistics.clone()))
    }

    /// List all registered engine IDs
    pub fn list_engines(&self) -> Vec<String> {
        self.engines
            .read()
            .ok()
            .map(|engines| engines.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Enable an engine
    pub fn enable(&self, id: &str) -> Result<(), EngineRegistryError> {
        let mut engines = self.engines.write().map_err(|_| EngineRegistryError::LockError)?;

        match engines.get_mut(id) {
            Some(entry) => {
                entry.enabled = true;
                Ok(())
            }
            None => Err(EngineRegistryError::NotFound(id.to_string())),
        }
    }

    /// Disable an engine
    pub fn disable(&self, id: &str) -> Result<(), EngineRegistryError> {
        let mut engines = self.engines.write().map_err(|_| EngineRegistryError::LockError)?;

        match engines.get_mut(id) {
            Some(entry) => {
                entry.enabled = false;
                Ok(())
            }
            None => Err(EngineRegistryError::NotFound(id.to_string())),
        }
    }

    /// Check if engine is enabled
    pub fn is_enabled(&self, id: &str) -> bool {
        self.engines
            .read()
            .ok()
            .and_then(|engines| engines.get(id).map(|e| e.enabled))
            .unwrap_or(false)
    }

    /// Record engine execution
    pub fn record_execution(
        &self,
        id: &str,
        duration: Duration,
        success: bool,
        memory_used: usize,
    ) -> Result<(), EngineRegistryError> {
        let mut engines = self.engines.write().map_err(|_| EngineRegistryError::LockError)?;

        match engines.get_mut(id) {
            Some(entry) => {
                entry.statistics.record_invocation(duration, success, memory_used);
                Ok(())
            }
            None => Err(EngineRegistryError::NotFound(id.to_string())),
        }
    }
}

impl Default for EngineRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Engine registry errors
#[derive(Debug, Clone)]
pub enum EngineRegistryError {
    /// Engine already registered
    AlreadyRegistered(String),
    /// Engine not found
    NotFound(String),
    /// Lock acquisition failed
    LockError,
    /// Configuration error
    ConfigurationError(String),
}

impl std::fmt::Display for EngineRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineRegistryError::AlreadyRegistered(id) => {
                write!(f, "Engine '{}' is already registered", id)
            }
            EngineRegistryError::NotFound(id) => {
                write!(f, "Engine '{}' not found", id)
            }
            EngineRegistryError::LockError => {
                write!(f, "Failed to acquire lock on engine registry")
            }
            EngineRegistryError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
        }
    }
}

impl std::error::Error for EngineRegistryError {}
