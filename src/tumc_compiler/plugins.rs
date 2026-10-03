//! TUMC Plugin System
//!
//! Extensible plugin architecture for extending TUMC compiler functionality.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Plugin metadata
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: Option<String>,
    /// Plugin author
    pub author: Option<String>,
    /// Dependencies
    pub dependencies: Vec<PluginDependency>,
    /// Entry point
    pub entry: String,
}

/// Plugin dependency
#[derive(Debug, Clone)]
pub struct PluginDependency {
    /// Dependency name
    pub name: String,
    /// Version requirement
    pub version: String,
}

/// Plugin state
#[derive(Debug, Clone)]
pub enum PluginState {
    /// Plugin is loaded
    Loaded,
    /// Plugin is initialized
    Initialized,
    /// Plugin is running
    Running,
    /// Plugin is paused
    Paused,
    /// Plugin is stopped
    Stopped,
    /// Plugin error
    Error(String),
}

/// Plugin trait
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Initialize plugin
    fn initialize(&mut self) -> Result<(), PluginError>;

    /// Shutdown plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;

    /// Get plugin state
    fn state(&self) -> PluginState;

    /// Process compilation event
    fn on_compilation_event(&self, event: &CompilationEvent) -> Result<(), PluginError> {
        let _ = event;
        Ok(())
    }
}

/// Compilation event
#[derive(Debug, Clone)]
pub enum CompilationEvent {
    /// Before compilation
    PreCompilation { source: String },
    /// After compilation
    PostCompilation { result: String },
    /// Error during compilation
    CompilationError { error: String },
    /// Optimization pass
    OptimizationPass { pass: String },
}

/// Plugin error
#[derive(Debug)]
pub enum PluginError {
    LoadFailed(String),
    InitFailed(String),
    NotFound(String),
    DependencyError(String),
    StateError(String),
    SymbolNotFound(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::LoadFailed(s) => write!(f, "Failed to load plugin: {}", s),
            PluginError::InitFailed(s) => write!(f, "Failed to initialize plugin: {}", s),
            PluginError::NotFound(s) => write!(f, "Plugin not found: {}", s),
            PluginError::DependencyError(s) => write!(f, "Dependency error: {}", s),
            PluginError::StateError(s) => write!(f, "State error: {}", s),
            PluginError::SymbolNotFound(s) => write!(f, "Symbol not found: {}", s),
        }
    }
}

impl std::error::Error for PluginError {}

/// Plugin manager
pub struct PluginManager {
    /// Loaded plugins
    plugins: HashMap<String, Arc<dyn Plugin>>,
    /// Plugin paths
    plugin_dirs: Vec<PathBuf>,
    /// Hooks
    hooks: HashMap<String, Vec<Arc<dyn PluginHook>>>,
    /// State
    state: ManagerState,
}

/// Manager state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagerState {
    Uninitialized,
    Initializing,
    Ready,
    Shutdown,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    /// Create new plugin manager
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            plugin_dirs: Vec::new(),
            hooks: HashMap::new(),
            state: ManagerState::Uninitialized,
        }
    }

    /// Add plugin directory
    pub fn add_plugin_dir(&mut self, path: PathBuf) -> &mut Self {
        self.plugin_dirs.push(path);
        self
    }

    /// Load plugin from path
    pub fn load_plugin(&mut self, path: &PathBuf) -> Result<(), PluginError> {
        // In a real implementation, would dynamically load .so/.dll/.dylib
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Create a placeholder plugin for demonstration
        let metadata = PluginMetadata {
            name: name.clone(),
            version: "0.1.0".to_string(),
            description: Some("Loaded plugin".to_string()),
            author: None,
            dependencies: Vec::new(),
            entry: path.to_string_lossy().to_string(),
        };

        let plugin: Arc<dyn Plugin> = Arc::new(DynamicPlugin { metadata });
        self.plugins.insert(name.clone(), plugin);

        Ok(())
    }

    /// Register hook
    pub fn register_hook(&mut self, event: &str, hook: Arc<dyn PluginHook>) -> &mut Self {
        self.hooks
            .entry(event.to_string())
            .or_default()
            .push(hook);
        self
    }

    /// Get plugin
    pub fn get_plugin(&self, name: &str) -> Option<&Arc<dyn Plugin>> {
        self.plugins.get(name)
    }

    /// Get all plugins
    pub fn plugins(&self) -> Vec<&Arc<dyn Plugin>> {
        self.plugins.values().collect()
    }

    /// Initialize all plugins
    pub fn initialize(&mut self) -> Result<(), PluginError> {
        self.state = ManagerState::Initializing;

        for plugin in self.plugins.values_mut() {
            let plugin_ptr = Arc::get_mut(plugin).ok_or_else(|| {
                PluginError::StateError("Could not get mutable plugin reference".to_string())
            })?;

            plugin_ptr.initialize()?;
        }

        self.state = ManagerState::Ready;
        Ok(())
    }

    /// Shutdown all plugins
    pub fn shutdown(&mut self) -> Result<(), PluginError> {
        for plugin in self.plugins.values_mut() {
            let plugin_ptr = Arc::get_mut(plugin).ok_or_else(|| {
                PluginError::StateError("Could not get mutable plugin reference".to_string())
            })?;

            plugin_ptr.shutdown()?;
        }

        self.plugins.clear();
        self.state = ManagerState::Shutdown;
        Ok(())
    }

    /// Emit event to all registered hooks
    pub fn emit_event(&self, event: &CompilationEvent) -> Result<(), PluginError> {
        let event_name = match event {
            CompilationEvent::PreCompilation { .. } => "pre_compilation",
            CompilationEvent::PostCompilation { .. } => "post_compilation",
            CompilationEvent::CompilationError { .. } => "compilation_error",
            CompilationEvent::OptimizationPass { .. } => "optimization_pass",
        };

        if let Some(hooks) = self.hooks.get(event_name) {
            for hook in hooks {
                hook.execute(event)?;
            }
        }

        Ok(())
    }
}

/// Plugin hook
pub trait PluginHook: Send + Sync {
    /// Execute hook
    fn execute(&self, event: &CompilationEvent) -> Result<(), PluginError>;
}

/// Dynamic plugin (wrapper for loaded plugins)
struct DynamicPlugin {
    metadata: PluginMetadata,
    state: PluginState,
}

impl Plugin for DynamicPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        self.state = PluginState::Initialized;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        self.state = PluginState::Stopped;
        Ok(())
    }

    fn state(&self) -> PluginState {
        self.state.clone()
    }
}

/// Plugin compiler pass
pub struct PluginPass {
    /// Pass name
    pub name: String,
    /// Pass function
    pub run: Box<dyn Fn(&str) -> Result<String, PluginError> + Send + Sync>,
}

impl PluginPass {
    /// Create new pass
    pub fn new<F>(name: &str, run: F) -> Self
    where
        F: Fn(&str) -> Result<String, PluginError> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            run: Box::new(run),
        }
    }

    /// Execute pass
    pub fn execute(&self, source: &str) -> Result<String, PluginError> {
        (self.run)(source)
    }
}

/// Plugin extension point
pub trait ExtensionPoint: Send + Sync {
    /// Get extension point name
    fn name(&self) -> &str;

    /// Execute extension
    fn execute(&self, ctx: &ExtensionContext) -> Result<(), PluginError>;
}

/// Extension context
pub struct ExtensionContext {
    /// Compiler state
    pub compiler_state: HashMap<String, Box<dyn std::any::Any>>,
    /// Source code
    pub source: String,
    /// AST (serialized)
    pub ast: Option<Vec<u8>>,
}

impl ExtensionContext {
    /// Create new context
    pub fn new(source: String) -> Self {
        Self {
            compiler_state: HashMap::new(),
            source,
            ast: None,
        }
    }

    /// Get state value
    pub fn get_state<T: 'static>(&self, key: &str) -> Option<&T> {
        self.compiler_state
            .get(key)
            .and_then(|v| v.downcast_ref())
    }

    /// Set state value
    pub fn set_state<T: 'static>(&mut self, key: String, value: T) {
        self.compiler_state.insert(key, Box::new(value));
    }
}

/// Plugin loader configuration
#[derive(Debug, Clone)]
pub struct PluginLoaderConfig {
    /// Plugin directories
    pub plugin_dirs: Vec<PathBuf>,
    /// Plugin extension
    pub extension: String,
    /// Allow list
    pub allow_list: Vec<String>,
    /// Deny list
    pub deny_list: Vec<String>,
    /// Enable sandbox
    pub sandbox_enabled: bool,
}

impl Default for PluginLoaderConfig {
    fn default() -> Self {
        Self {
            plugin_dirs: Vec::new(),
            extension: if cfg!(target_os = "windows") {
                "dll".to_string()
            } else if cfg!(target_os = "macos") {
                "dylib".to_string()
            } else {
                "so".to_string()
            },
            allow_list: Vec::new(),
            deny_list: Vec::new(),
            sandbox_enabled: true,
        }
    }
}

/// Plugin sandbox
pub struct PluginSandbox {
    /// Allowed operations
    allowed_ops: HashSet<AllowedOp>,
    /// Memory limit
    memory_limit: usize,
    /// CPU time limit
    cpu_limit_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllowedOp {
    ReadFile,
    WriteFile,
    Network,
    SpawnProcess,
    LoadLibrary,
}

impl Default for PluginSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginSandbox {
    /// Create new sandbox
    pub fn new() -> Self {
        Self {
            allowed_ops: HashSet::from([
                AllowedOp::ReadFile,
                AllowedOp::WriteFile,
            ]),
            memory_limit: 128 * 1024 * 1024, // 128MB
            cpu_limit_secs: 30,
        }
    }

    /// Allow operation
    pub fn allow(&mut self, op: AllowedOp) -> &mut Self {
        self.allowed_ops.insert(op);
        self
    }

    /// Deny operation
    pub fn deny(&mut self, op: AllowedOp) -> &mut Self {
        self.allowed_ops.remove(&op);
        self
    }

    /// Check if operation allowed
    pub fn is_allowed(&self, op: AllowedOp) -> bool {
        self.allowed_ops.contains(&op)
    }

    /// Set memory limit
    pub fn memory_limit(mut self, bytes: usize) -> Self {
        self.memory_limit = bytes;
        self
    }

    /// Set CPU limit
    pub fn cpu_limit(mut self, secs: u64) -> Self {
        self.cpu_limit_secs = secs;
        self
    }
}

/// Plugin discovery
pub struct PluginDiscovery {
    /// Search paths
    search_paths: Vec<PathBuf>,
    /// Cache
    cache: HashMap<String, PluginMetadata>,
}

impl PluginDiscovery {
    /// Create new discovery
    pub fn new() -> Self {
        Self {
            search_paths: Vec::new(),
            cache: HashMap::new(),
        }
    }

    /// Add search path
    pub fn add_path(&mut self, path: PathBuf) -> &mut Self {
        self.search_paths.push(path);
        self
    }

    /// Discover plugins
    pub fn discover(&mut self) -> Result<Vec<PluginMetadata>, PluginError> {
        let mut found = Vec::new();

        for path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    // Would check if valid plugin file
                    let metadata = PluginMetadata {
                        name: entry_path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        version: "0.1.0".to_string(),
                        description: None,
                        author: None,
                        dependencies: Vec::new(),
                        entry: entry_path.to_string_lossy().to_string(),
                    };
                    self.cache.insert(metadata.name.clone(), metadata.clone());
                    found.push(metadata);
                }
            }
        }

        Ok(found)
    }

    /// Get cached metadata
    pub fn get_cached(&self, name: &str) -> Option<&PluginMetadata> {
        self.cache.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager() {
        let mut manager = PluginManager::new();
        assert_eq!(manager.state, ManagerState::Uninitialized);
    }

    fn test_extension_point() {
        struct TestExtension;
        impl ExtensionPoint for TestExtension {
            fn name(&self) -> &str {
                "test"
            }

            fn execute(&self, _ctx: &ExtensionContext) -> Result<(), PluginError> {
                Ok(())
            }
        }

        let ext = TestExtension;
        assert_eq!(ext.name(), "test");
    }

    #[test]
    fn test_plugin_sandbox() {
        let mut sandbox = PluginSandbox::new();
        assert!(sandbox.is_allowed(AllowedOp::ReadFile));
        sandbox.deny(AllowedOp::ReadFile);
        assert!(!sandbox.is_allowed(AllowedOp::ReadFile));
    }

    #[test]
    fn test_plugin_pass() {
        let pass = PluginPass::new("test", |source| Ok(source.to_string()));
        let result = pass.execute("hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }
}
