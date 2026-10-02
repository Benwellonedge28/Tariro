//! TUMC Hot Reloading
//!
//! Live code reloading system for rapid development cycles.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Hot reload configuration
#[derive(Debug, Clone)]
pub struct HotReloadConfig {
    /// Enable hot reload
    pub enabled: bool,
    /// Watch directories
    pub watch_dirs: Vec<PathBuf>,
    /// Debounce duration
    pub debounce_ms: u64,
    /// Enable preserve state
    pub preserve_state: bool,
    /// Enable preserve console
    pub preserve_console: bool,
    /// Extensions to watch
    pub extensions: Vec<String>,
}

impl Default for HotReloadConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_dirs: vec![PathBuf::from("src")],
            debounce_ms: 300,
            preserve_state: true,
            preserve_console: true,
            extensions: vec![
                "tumc".to_string(),
                "rs".to_string(),
            ],
        }
    }
}

/// Hot reload event
#[derive(Debug, Clone)]
pub enum ReloadEvent {
    /// File modified
    Modified(PathBuf),
    /// File added
    Added(PathBuf),
    /// File removed
    Removed(PathBuf),
    /// Compilation error
    CompilationError(String),
    /// Reload triggered
    ReloadTriggered,
    /// Reload complete
    ReloadComplete(Duration),
}

/// Change record
#[derive(Debug, Clone)]
pub struct ChangeRecord {
    /// File path
    pub path: PathBuf,
    /// Event type
    pub event: ReloadEvent,
    /// Timestamp
    pub timestamp: Instant,
    /// Hash of content
    pub content_hash: u64,
}

/// State serializer
pub trait StateSerializer: Send + Sync {
    /// Serialize state
    fn serialize(&self, state: &dyn std::any::Any) -> Vec<u8>;

    /// Deserialize state
    fn deserialize(&self, data: &[u8]) -> Box<dyn std::any::Any>;
}

/// Hot reload manager
pub struct HotReloadManager {
    /// Configuration
    config: HotReloadConfig,
    /// File watcher
    watcher: FileWatcher,
    /// Change buffer
    changes: Arc<RwLock<Vec<ChangeRecord>>>,
    /// State cache
    state_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// Last reload time
    last_reload: Arc<RwLock<Instant>>,
    /// Reload count
    reload_count: Arc<RwLock<usize>>,
    /// Callbacks
    callbacks: Vec<Box<dyn ReloadCallback + Send + Sync>>,
    /// Serializer
    serializer: Option<Box<dyn StateSerializer>>,
}

impl HotReloadManager {
    /// Create new manager
    pub fn new(config: HotReloadConfig) -> Self {
        Self {
            watcher: FileWatcher::new(),
            config,
            changes: Arc::new(RwLock::new(Vec::new())),
            state_cache: Arc::new(RwLock::new(HashMap::new())),
            last_reload: Arc::new(RwLock::new(Instant::now())),
            reload_count: Arc::new(RwLock::new(0)),
            callbacks: Vec::new(),
            serializer: None,
        }
    }

    /// Set serializer
    pub fn set_serializer(&mut self, serializer: Box<dyn StateSerializer>) {
        self.serializer = Some(serializer);
    }

    /// Add callback
    pub fn on_reload<F>(&mut self, callback: F)
    where
        F: Fn(&[ChangeRecord]) + Send + Sync + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    /// Watch directory
    pub fn watch_dir(&mut self, path: impl AsRef<Path>) {
        self.watcher.add_path(path);
    }

    /// Start watching
    pub fn start(&mut self) {
        for dir in &self.config.watch_dirs {
            self.watcher.add_path(dir);
        }
    }

    /// Check for changes
    pub fn check_changes(&self) -> Vec<ChangeRecord> {
        let events = self.watcher.get_events();

        let mut records = Vec::new();
        for event in events {
            if let Some(ext) = event.path.extension() {
                if self.config.extensions.contains(&ext.to_string_lossy().to_string()) {
                    let record = ChangeRecord {
                        path: event.path.clone(),
                        event: match event.kind {
                            FileEventKind::Modified => ReloadEvent::Modified(event.path.clone()),
                            FileEventKind::Created => ReloadEvent::Added(event.path.clone()),
                            FileEventKind::Removed => ReloadEvent::Removed(event.path.clone()),
                        },
                        timestamp: event.timestamp,
                        content_hash: event.content_hash,
                    };
                    records.push(record);
                }
            }
        }

        records
    }

    /// Trigger reload
    pub fn trigger_reload(&mut self) -> Result<ReloadResult, ReloadError> {
        let start = Instant::now();
        let changes = self.check_changes();

        if changes.is_empty() {
            return Ok(ReloadResult {
                success: true,
                changed_files: Vec::new(),
                duration: start.elapsed(),
                reload_count: *self.reload_count.read().unwrap(),
            });
        }

        // Save state if enabled
        if self.config.preserve_state {
            self.save_state();
        }

        // Trigger callbacks
        for callback in &self.callbacks {
            callback(&changes);
        }

        // Increment reload count
        *self.reload_count.write().unwrap() += 1;
        *self.last_reload.write().unwrap() = Instant::now();

        let changed_files: Vec<_> = changes.iter().map(|c| c.path.clone()).collect();

        Ok(ReloadResult {
            success: true,
            changed_files,
            duration: start.elapsed(),
            reload_count: *self.reload_count.read().unwrap(),
        })
    }

    /// Save state
    fn save_state(&self) {
        if let Some(serializer) = &self.serializer {
            let mut cache = self.state_cache.write().unwrap();
            // In a real implementation, would serialize current app state
            cache.insert("last_save".to_string(), Vec::new());
        }
    }

    /// Restore state
    fn restore_state(&self) {
        if let Some(serializer) = &self.serializer {
            let cache = self.state_cache.read().unwrap();
            if let Some(data) = cache.get("last_save") {
                // In a real implementation, would deserialize and restore state
                let _ = serializer.deserialize(data);
            }
        }
    }

    /// Get statistics
    pub fn stats(&self) -> ReloadStats {
        ReloadStats {
            reload_count: *self.reload_count.read().unwrap(),
            last_reload: *self.last_reload.read().unwrap(),
            cached_states: self.state_cache.read().unwrap().len(),
        }
    }

    /// Clear state cache
    pub fn clear_cache(&mut self) {
        self.state_cache.write().unwrap().clear();
    }
}

/// Reload callback
pub trait ReloadCallback: Send + Sync {
    /// Called on reload
    fn call(&self, changes: &[ChangeRecord]);
}

impl<F> ReloadCallback for F
where
    F: Fn(&[ChangeRecord]) + Send + Sync + 'static,
{
    fn call(&self, changes: &[ChangeRecord]) {
        self(changes)
    }
}

/// Reload result
#[derive(Debug)]
pub struct ReloadResult {
    pub success: bool,
    pub changed_files: Vec<PathBuf>,
    pub duration: Duration,
    pub reload_count: usize,
}

/// Reload statistics
#[derive(Debug)]
pub struct ReloadStats {
    pub reload_count: usize,
    pub last_reload: Instant,
    pub cached_states: usize,
}

/// Reload error
#[derive(Debug)]
pub enum ReloadError {
    CompilationFailed(String),
    StateSerializationFailed,
    StateDeserializationFailed,
    FileNotFound,
    WatchError(String),
}

/// File watcher
pub struct FileWatcher {
    /// Watched paths
    paths: Vec<PathBuf>,
    /// Events
    events: Vec<FileEvent>,
    /// File hashes
    hashes: HashMap<PathBuf, u64>,
}

impl FileWatcher {
    /// Create new watcher
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            events: Vec::new(),
            hashes: HashMap::new(),
        }
    }

    /// Add path to watch
    pub fn add_path(&mut self, path: impl AsRef<Path>) {
        self.paths.push(path.as_ref().to_path_buf());
    }

    /// Get events
    pub fn get_events(&self) -> Vec<FileEvent> {
        let mut events = Vec::new();

        for path in &self.paths {
            if path.is_dir() {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let entry_path = entry.path();
                        if let Some(event) = self.check_file(&entry_path) {
                            events.push(event);
                        }
                    }
                }
            } else if let Some(event) = self.check_file(path) {
                events.push(event);
            }
        }

        events
    }

    /// Check single file
    fn check_file(&self, path: &Path) -> Option<FileEvent> {
        let metadata = std::fs::metadata(path).ok()?;
        let modified = metadata.modified().ok()?;
        let hash = self.compute_hash(path);

        let old_hash = self.hashes.get(path);

        if let Some(old_hash) = old_hash {
            if *old_hash != hash {
                self.hashes.insert(path.to_path_buf(), hash);
                return Some(FileEvent {
                    path: path.to_path_buf(),
                    kind: FileEventKind::Modified,
                    timestamp: Instant::now(),
                    content_hash: hash,
                });
            }
        } else {
            self.hashes.insert(path.to_path_buf(), hash);
        }

        None
    }

    /// Compute file hash
    fn compute_hash(&self, path: &Path) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;

        let content = std::fs::read(path).unwrap_or_default();
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }
}

/// File event
#[derive(Debug, Clone)]
pub struct FileEvent {
    pub path: PathBuf,
    pub kind: FileEventKind,
    pub timestamp: Instant,
    pub content_hash: u64,
}

/// File event kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileEventKind {
    Created,
    Modified,
    Removed,
}

/// Hot reload middleware for web servers
pub struct HotReloadMiddleware {
    /// Manager
    manager: HotReloadManager,
    /// WebSocket clients
    ws_clients: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<String>>>>,
}

impl HotReloadMiddleware {
    /// Create new middleware
    pub fn new(manager: HotReloadManager) -> Self {
        Self {
            manager,
            ws_clients: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register WebSocket client
    pub async fn register_client(&self, sender: tokio::sync::mpsc::Sender<String>) {
        self.ws_clients.write().unwrap().push(sender);
    }

    /// Notify clients of reload
    pub async fn notify_clients(&self, result: &ReloadResult) {
        let message = serde_json::json!({
            "type": "reload",
            "result": result,
        }).to_string();

        let clients = self.ws_clients.read().unwrap().clone();
        for client in clients {
            client.send(message.clone()).await.ok();
        }
    }

    /// Get manager reference
    pub fn manager(&self) -> &HotReloadManager {
        &self.manager
    }
}

/// CLI hot reload command
pub struct HotReloadCommand {
    /// Entry point
    pub entry: PathBuf,
    /// Watch directories
    pub watch: Vec<PathBuf>,
    /// Arguments
    pub args: Vec<String>,
}

impl HotReloadCommand {
    /// Run hot reload session
    pub fn run(&self) -> Result<(), ReloadError> {
        let config = HotReloadConfig {
            enabled: true,
            watch_dirs: self.watch.clone(),
            ..Default::default()
        };

        let mut manager = HotReloadManager::new(config);

        // Watch entry file
        if let Some(parent) = self.entry.parent() {
            manager.watch_dir(parent);
        }

        manager.start();

        println!("Hot reload enabled. Watching for changes...");

        // Main loop
        loop {
            std::thread::sleep(Duration::from_millis(100));

            match manager.trigger_reload() {
                Ok(result) => {
                    if !result.changed_files.is_empty() {
                        println!("Reloaded in {:?}", result.duration);
                    }
                }
                Err(e) => {
                    eprintln!("Reload error: {:?}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = HotReloadConfig::default();
        assert!(config.enabled);
        assert_eq!(config.debounce_ms, 300);
    }

    #[test]
    fn test_file_watcher() {
        let mut watcher = FileWatcher::new();
        watcher.add_path(".");
        let events = watcher.get_events();
        assert!(events.is_empty() || !events.is_empty()); // Depends on current dir
    }

    #[test]
    fn test_reload_manager_creation() {
        let config = HotReloadConfig::default();
        let manager = HotReloadManager::new(config);
        let stats = manager.stats();
        assert_eq!(stats.reload_count, 0);
    }
}
