// Tooling Module - LSP, Debug, Profiler, and Development Tools

pub mod lsp;
pub mod debug;
pub mod profiler;

use crate::tumc_compiler::ir::Module;

pub use lsp::LspServer;
pub use debug::{Debugger, Breakpoint, Watchpoint};
pub use profiler::{Profiler, ProfileData, PerformanceCounter};

/// Tooling configuration
#[derive(Debug, Clone)]
pub struct ToolingConfig {
    pub enable_lsp: bool,
    pub enable_debugger: bool,
    pub enable_profiler: bool,
    pub lsp_port: u16,
    pub debug_port: u16,
}

impl Default for ToolingConfig {
    fn default() -> Self {
        Self {
            enable_lsp: true,
            enable_debugger: true,
            enable_profiler: true,
            lsp_port: 9507,
            debug_port: 9508,
        }
    }
}

/// Tooling manager
pub struct ToolingManager {
    pub config: ToolingConfig,
    lsp: Option<LspServer>,
    debugger: Option<Debugger>,
    profiler: Option<Profiler>,
}

impl ToolingManager {
    pub fn new(config: ToolingConfig) -> Self {
        Self {
            config,
            lsp: None,
            debugger: None,
            profiler: None,
        }
    }

    pub fn start(&mut self, module: &Module) {
        if self.config.enable_lsp {
            self.lsp = Some(LspServer::new(self.config.lsp_port));
        }
        if self.config.enable_debugger {
            self.debugger = Some(Debugger::new(self.config.debug_port));
        }
        if self.config.enable_profiler {
            self.profiler = Some(Profiler::new());
        }
    }

    pub fn stop(&mut self) {
        self.lsp = None;
        self.debugger = None;
        self.profiler = None;
    }
}

impl Default for ToolingManager {
    fn default() -> Self {
        Self::new(ToolingConfig::default())
    }
}
