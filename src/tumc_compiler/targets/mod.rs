// Targets Module - CPU, GPU, WASM, JVM Targets

pub mod cpu;
pub mod gpu;
pub mod wasm;
pub mod vm;
pub mod risc_v;

pub use cpu::*;
pub use gpu::*;
pub use wasm::*;
pub use vm::*;

// ============================================================================
// Target Infrastructure
// ============================================================================

use crate::tumc_compiler::ir::*;

/// Compilation target trait
pub trait Target {
    /// Target name
    fn name(&self) -> &'static str;

    /// Target triple
    fn triple(&self) -> &str;

    /// Data layout
    fn data_layout(&self) -> DataLayout;

    /// Generate code for a function
    fn codegen(&self, function: &Function) -> Result<TargetCode, TargetError>;

    /// Verify generated code
    fn verify(&self, code: &TargetCode) -> bool;
}

/// Target code representation
#[derive(Debug, Clone)]
pub struct TargetCode {
    pub data: Vec<u8>,
    pub relocations: Vec<Relocation>,
    pub debug_info: DebugInfo,
}

#[derive(Debug, Clone)]
pub struct Relocation {
    pub offset: usize,
    pub kind: RelocationKind,
    pub symbol: String,
    pub addend: i64,
}

#[derive(Debug, Clone)]
pub enum RelocationKind {
    Absolute,
    Relative,
    PLT32,
    GOTPCRel,
    Direct,
}

#[derive(Debug, Clone, Default)]
pub struct DebugInfo {
    pub dwarf: Option<Vec<u8>>,
    pub source_map: Option<SourceMap>,
}

#[derive(Debug, Clone)]
pub struct SourceMap {
    pub segments: Vec<SourceSegment>,
}

#[derive(Debug, Clone)]
pub struct SourceSegment {
    pub output_offset: usize,
    pub source_line: u32,
    pub source_column: u32,
}

#[derive(Debug, Clone)]
pub enum TargetError {
    Unsupported(String),
    CodegenFailed(String),
    RelocationFailed(String),
}

/// Target machine configuration
#[derive(Debug, Clone)]
pub struct TargetMachine {
    pub target: String,
    pub cpu: String,
    pub features: Vec<String>,
    pub options: TargetOptions,
}

#[derive(Debug, Clone)]
pub struct TargetOptions {
    pub position_independent: bool,
    pub static_link: bool,
    pub little_endian: bool,
    pub code_model: CodeModel,
    pub optimization_level: OptLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum CodeModel {
    Small,
    Medium,
    Large,
    Kernel,
}

impl Default for TargetOptions {
    fn default() -> Self {
        Self {
            position_independent: false,
            static_link: false,
            little_endian: true,
            code_model: CodeModel::Small,
            optimization_level: OptLevel::O2,
        }
    }
}

/// Target registry
pub struct TargetRegistry {
    targets: HashMap<String, Box<dyn Target>>,
}

impl TargetRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            targets: HashMap::new(),
        };
        registry.register_builtins();
        registry
    }

    fn register_builtins(&mut self) {
        // Register built-in targets
        self.register(X86_64Target::new());
        self.register(Arm64Target::new());
        self.register(WasmTarget::new());
        self.register(JvmTarget::new());
    }

    pub fn register<T: Target + 'static>(&mut self, target: T) {
        self.targets.insert(target.name().to_string(), Box::new(target));
    }

    pub fn get(&self, name: &str) -> Option<&dyn Target> {
        self.targets.get(name).map(|b| b.as_ref())
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.targets.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for TargetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;
