//! TUMC Build System
//!
//! Incremental build system for TUMC projects with caching,
//! parallel compilation, and cross-platform support.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Build configuration
#[derive(Debug, Clone)]
pub struct BuildConfig {
    /// Build mode
    pub mode: BuildMode,
    /// Target platform
    pub target: TargetPlatform,
    /// Optimization level
    pub optimization: OptimizationLevel,
    /// Output directory
    pub out_dir: PathBuf,
    /// Build directory
    pub build_dir: PathBuf,
    /// Enable incremental compilation
    pub incremental: bool,
    /// Enable caching
    pub cache: bool,
    /// Number of parallel jobs
    pub jobs: usize,
    /// Enable debug info
    pub debug: bool,
    /// Link time optimization
    pub lto: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            mode: BuildMode::Debug,
            target: TargetPlatform::Native,
            optimization: OptimizationLevel::Standard,
            out_dir: PathBuf::from("target/tumc"),
            build_dir: PathBuf::from("target/build"),
            incremental: true,
            cache: true,
            jobs: num_cpus::get(),
            debug: true,
            lto: false,
        }
    }
}

/// Build mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildMode {
    Debug,
    Release,
    Profile,
}

/// Build target
#[derive(Debug, Clone)]
pub struct BuildTarget {
    /// Target name
    pub name: String,
    /// Target type
    pub kind: TargetKind,
    /// Source files
    pub sources: Vec<PathBuf>,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Compiler flags
    pub flags: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
}

/// Target kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetKind {
    Binary,
    Library,
    Test,
    Benchmark,
    Example,
}

/// Build artifact
#[derive(Debug, Clone)]
pub struct BuildArtifact {
    /// Artifact path
    pub path: PathBuf,
    /// Artifact kind
    pub kind: ArtifactKind,
    /// Compilation time
    pub compile_time: Duration,
    /// File size
    pub size: u64,
    /// Dependencies
    pub dependencies: Vec<PathBuf>,
}

/// Artifact kind
#[derive(Debug, Clone)]
pub enum ArtifactKind {
    Executable,
    Library,
    ObjectFile,
    WASM,
    Bytecode,
}

/// Build unit (single compilation task)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BuildUnit {
    /// Package name
    pub package: String,
    /// Target name
    pub target: String,
    /// Source file
    pub source: PathBuf,
    /// Output file
    pub output: PathBuf,
    /// Fingerprint
    pub fingerprint: u64,
}

impl BuildUnit {
    /// Create new build unit
    pub fn new(package: &str, target: &str, source: PathBuf, output: PathBuf) -> Self {
        Self {
            package: package.to_string(),
            target: target.to_string(),
            source,
            output,
            fingerprint: 0,
        }
    }

    /// Compute fingerprint
    pub fn compute_fingerprint(&mut self) {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.source.hash(&mut hasher);
        self.target.hash(&mut hasher);
        self.fingerprint = hasher.finish();
    }
}

/// Build plan
#[derive(Debug, Clone)]
pub struct BuildPlan {
    /// Units to build
    pub units: Vec<BuildUnit>,
    /// Units to link
    pub link_units: Vec<BuildUnit>,
    /// Total units
    pub total_units: usize,
}

impl BuildPlan {
    /// Create new plan
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
            link_units: Vec::new(),
            total_units: 0,
        }
    }

    /// Check if unit needs rebuilding
    pub fn needs_rebuild(&self, unit: &BuildUnit, cache: &BuildCache) -> bool {
        if !cache.contains(&unit.fingerprint) {
            return true;
        }

        // Check if output exists
        if !unit.output.exists() {
            return true;
        }

        // Check if source is newer than output
        if let (Ok(src_meta), Ok(out_meta)) = (
            std::fs::metadata(&unit.source),
            std::fs::metadata(&unit.output),
        ) {
            if let (Ok(src_time), Ok(out_time)) = (src_meta.modified(), out_meta.modified()) {
                return src_time > out_time;
            }
        }

        false
    }
}

/// Build cache
#[derive(Debug, Clone, Default)]
pub struct BuildCache {
    /// Fingerprint cache
    fingerprints: HashMap<u64, CacheEntry>,
    /// Cache directory
    dir: PathBuf,
}

#[derive(Debug, Clone)]
struct CacheEntry {
    fingerprint: u64,
    output: PathBuf,
    compile_time: Duration,
    exit_code: i32,
}

/// Build context
#[derive(Debug, Clone)]
pub struct BuildContext {
    /// Configuration
    pub config: BuildConfig,
    /// Workspace root
    pub workspace: PathBuf,
    /// Package
    pub package: PackageManifest,
    /// Targets
    pub targets: Vec<BuildTarget>,
    /// Resolved dependencies
    pub dependencies: HashMap<String, PathBuf>,
}

impl BuildContext {
    /// Create new context
    pub fn new(workspace: PathBuf, config: BuildConfig) -> Self {
        Self {
            config,
            workspace,
            package: PackageManifest::new("unknown", Version { major: 0, minor: 0, patch: 0, pre: None, build: None }),
            targets: Vec::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Generate build plan
    pub fn plan(&self) -> BuildPlan {
        let mut plan = BuildPlan::new();

        for target in &self.targets {
            for source in &target.sources {
                let output = self.output_path(target, source);
                let mut unit = BuildUnit::new(&self.package.name, &target.name, source.clone(), output);
                unit.compute_fingerprint();
                plan.units.push(unit);
            }
        }

        plan.total_units = plan.units.len();
        plan
    }

    /// Get output path for source
    fn output_path(&self, target: &BuildTarget, source: &Path) -> PathBuf {
        let stem = source.file_stem().unwrap_or_default().to_string_lossy();
        let ext = match self.config.target {
            TargetPlatform::Wasm => "wasm",
            TargetPlatform::Native => {
                if target.kind == TargetKind::Library { "rlib" } else { "o" }
            }
            _ => "o",
        };

        self.config
            .out_dir
            .join(&target.name)
            .join(format!("{}.{}", stem, ext))
    }
}

/// Build driver
pub struct BuildDriver {
    /// Context
    context: BuildContext,
    /// Cache
    cache: BuildCache,
    /// Compilation queue
    queue: Arc<RwLock<Vec<BuildUnit>>>,
    /// Active compilations
    active: Arc<RwLock<HashSet<BuildUnit>>>,
    /// Results
    results: Arc<RwLock<Vec<BuildResult>>>,
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    /// Unit
    pub unit: BuildUnit,
    /// Success
    pub success: bool,
    /// Duration
    pub duration: Duration,
    /// Output
    pub output: Option<PathBuf>,
    /// Error
    pub error: Option<String>,
}

impl BuildDriver {
    /// Create new driver
    pub fn new(context: BuildContext) -> Self {
        let cache_dir = context.config.out_dir.join(".cache");
        std::fs::create_dir_all(&cache_dir).ok();

        Self {
            context,
            cache: BuildCache {
                fingerprints: HashMap::new(),
                dir: cache_dir,
            },
            queue: Arc::new(RwLock::new(Vec::new())),
            active: Arc::new(RwLock::new(HashSet::new())),
            results: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Execute build
    pub fn build(&mut self) -> BuildReport {
        let start = Instant::now();
        let plan = self.context.plan();

        // Filter units that need building
        let mut to_build: Vec<BuildUnit> = plan
            .units
            .into_iter()
            .filter(|u| plan.needs_rebuild(u, &self.cache))
            .collect();

        let total = to_build.len();
        let cached = plan.total_units - total;

        // Build units in parallel
        if self.context.config.jobs > 1 {
            self.build_parallel(&mut to_build);
        } else {
            self.build_sequential(&mut to_build);
        }

        let duration = start.elapsed();

        BuildReport {
            success: true,
            duration,
            total_units: plan.total_units,
            built_units: total,
            cached_units: cached,
            failed_units: 0,
            artifacts: Vec::new(),
        }
    }

    /// Build units sequentially
    fn build_sequential(&mut self, units: &mut [BuildUnit]) {
        for unit in units {
            let result = self.compile_unit(unit);
            if result.success {
                self.cache.insert(&unit.fingerprint, &unit.output);
            }
        }
    }

    /// Build units in parallel
    fn build_parallel(&mut self, units: &mut [BuildUnit]) {
        use std::thread;

        let (tx, rx) = std::sync::mpsc::channel();

        for mut unit in units.drain(..) {
            let tx = tx.clone();
            let context = self.context.clone();
            let cache = Arc::new(RwLock::new(self.cache.clone()));

            thread::spawn(move || {
                let result = Self::compile_unit_internal(&context, &unit);
                if result.success {
                    if let Ok(mut c) = cache.write() {
                        c.insert(&unit.fingerprint, &unit.output);
                    }
                }
                tx.send((unit, result)).ok();
            });
        }

        drop(tx);

        for (unit, result) in rx {
            if result.success {
                if let Ok(mut c) = self.cache.write() {
                    c.insert(&unit.fingerprint, &unit.output);
                }
            }
        }
    }

    /// Compile a single unit
    fn compile_unit(unit: &BuildUnit) -> BuildResult {
        let start = Instant::now();
        let mut compiler = crate::tumc_compiler::Compiler::new(
            crate::tumc_compiler::SourceLanguage::TumcIR,
            crate::tumc_compiler::TargetPlatform::Wasm,
        );

        let source = std::fs::read_to_string(&unit.source).unwrap_or_default();
        let result = compiler.compile(&source);

        BuildResult {
            unit: unit.clone(),
            success: result.success,
            duration: start.elapsed(),
            output: if result.success { Some(unit.output.clone()) } else { None },
            error: if result.success { None } else { Some("Compilation failed".to_string()) },
        }
    }

    /// Internal compile unit
    fn compile_unit_internal(context: &BuildContext, unit: &BuildUnit) -> BuildResult {
        let start = Instant::now();

        // In a real implementation, this would invoke the actual compiler
        let output = unit.output.clone();
        std::fs::create_dir_all(output.parent().unwrap_or(&PathBuf::new())).ok();

        BuildResult {
            unit: unit.clone(),
            success: true,
            duration: start.elapsed(),
            output: Some(output),
            error: None,
        }
    }
}

impl BuildCache {
    /// Check if fingerprint is cached
    fn contains(&self, fingerprint: &u64) -> bool {
        self.fingerprints.contains_key(fingerprint)
    }

    /// Insert into cache
    fn insert(&mut self, fingerprint: &u64, output: &Path) {
        self.fingerprints.insert(
            *fingerprint,
            CacheEntry {
                fingerprint: *fingerprint,
                output: output.to_path_buf(),
                compile_time: Duration::ZERO,
                exit_code: 0,
            },
        );
    }
}

/// Build report
#[derive(Debug, Clone)]
pub struct BuildReport {
    /// Success
    pub success: bool,
    /// Total duration
    pub duration: Duration,
    /// Total units
    pub total_units: usize,
    /// Built units
    pub built_units: usize,
    /// Cached units
    pub cached_units: usize,
    /// Failed units
    pub failed_units: usize,
    /// Artifacts
    pub artifacts: Vec<BuildArtifact>,
}

impl BuildReport {
    /// Format for display
    pub fn format(&self) -> String {
        format!(
            "Build {} in {:?}\n  {} built, {} cached, {} failed\n",
            if self.success { "succeeded" } else { "failed" },
            self.duration,
            self.built_units,
            self.cached_units,
            self.failed_units
        )
    }
}

/// Build script executor
pub struct BuildScript {
    /// Script path
    script: PathBuf,
    /// Environment
    env: HashMap<String, String>,
}

impl BuildScript {
    /// Create new build script
    pub fn new(script: PathBuf) -> Self {
        Self {
            script,
            env: HashMap::new(),
        }
    }

    /// Set environment variable
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    /// Execute script
    pub fn execute(&self) -> Result<ScriptOutput, BuildError> {
        let output = Command::new(&self.script)
            .envs(&self.env)
            .output()
            .map_err(|e| BuildError::ScriptExecution(e.to_string()))?;

        Ok(ScriptOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
}

/// Script output
#[derive(Debug, Clone)]
pub struct ScriptOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// Build error
#[derive(Debug)]
pub enum BuildError {
    CompilationError(String),
    LinkError(String),
    ScriptExecution(String),
    IoError(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompilationError(s) => write!(f, "Compilation error: {}", s),
            Self::LinkError(s) => write!(f, "Link error: {}", s),
            Self::ScriptExecution(s) => write!(f, "Script execution error: {}", s),
            Self::IoError(s) => write!(f, "IO error: {}", s),
        }
    }
}

impl std::error::Error for BuildError {}

/// Watch mode for incremental builds
pub struct Watcher {
    /// Paths to watch
    paths: Vec<PathBuf>,
    /// Debounce duration
    debounce: Duration,
}

impl Watcher {
    /// Create new watcher
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            debounce: Duration::from_millis(500),
        }
    }

    /// Add path to watch
    pub fn watch(mut self, path: impl Into<PathBuf>) -> Self {
        self.paths.push(path.into());
        self
    }

    /// Set debounce duration
    pub fn debounce(mut self, duration: Duration) -> Self {
        self.debounce = duration;
        self
    }

    /// Start watching
    pub fn watch<F>(&self, mut callback: F)
    where
        F: FnMut(Vec<PathBuf>) + Send,
    {
        // In a real implementation, this would use notify or similar
        loop {
            std::thread::sleep(self.debounce);
            let changes = self.check_changes();
            if !changes.is_empty() {
                callback(changes);
            }
        }
    }

    /// Check for changes
    fn check_changes(&self) -> Vec<PathBuf> {
        let mut changes = Vec::new();

        for path in &self.paths {
            if let Ok(meta) = std::fs::metadata(path) {
                if meta.modified().unwrap_or(Instant::now().into()) > Instant::now().into() {
                    changes.push(path.clone());
                }
            }
        }

        changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_unit() {
        let mut unit = BuildUnit::new(
            "test",
            "bin",
            PathBuf::from("src/main.tumc"),
            PathBuf::from("target/main.wasm"),
        );
        unit.compute_fingerprint();
        assert!(unit.fingerprint != 0);
    }

    #[test]
    fn test_build_plan() {
        let plan = BuildPlan::new();
        assert_eq!(plan.total_units, 0);
    }

    #[test]
    fn test_build_report() {
        let report = BuildReport {
            success: true,
            duration: Duration::from_secs(10),
            total_units: 10,
            built_units: 5,
            cached_units: 5,
            failed_units: 0,
            artifacts: Vec::new(),
        };

        assert!(report.success);
    }
}
