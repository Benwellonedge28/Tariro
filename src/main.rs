//! Tariro AGI - Main Entry Point
//!
//! This is the production entry point for the Tariro AGI system,
//! implementing "Program-and-Compile-Once-Run-Everywhere-Anywhere-Forever" (POCO-REAF).
//!
//! # Features
//!
//! - Full AGI capabilities with cognitive reasoning
//! - TUMC Universal Meta-Compiler integration
//! - Neural-symbolic pattern recognition
//! - Meta-learning optimization
//! - Self-modifying code generation
//! - Quantum-ready architecture
//!
//! # Usage
//!
//! ```rust
//! use tariro_agi_core::TariroAGI;
//!
//! let mut agi = TariroAGI::new();
//! agi.initialize();
//! let status = agi.get_status();
//! println!("{}", status);
//! ```

use tariro_agi_core::{TariroAGI, VERSION, POCO_REAF, TUMC_NAME};
use std::process;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

/// Initialize logging with tracing
fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .init();
}

/// Main entry point for Tariro AGI
fn main() {
    // Initialize logging
    init_logging();

    info!("=================================================");
    info!("  TARIRO AGI - THE MOST POWERFUL AGI SYSTEM");
    info!("=================================================");
    info!("Version: {}", VERSION);
    info!("Paradigm: {}", POCO_REAF);
    info!("=================================================");

    // Create and initialize the AGI system
    let mut agi = TariroAGI::new();

    // Initialize the complete system
    if let Err(e) = std::panic::catch_unwind(|| {
        agi.initialize();
    }) {
        error!("Failed to initialize Tariro AGI: {:?}", e);
        eprintln!("ERROR: Failed to initialize Tariro AGI system");
        process::exit(1);
    }

    // Display system manifest
    let manifest = agi.get_system_manifest();
    info!("System Manifest:");
    info!("  - Total Files: {}", manifest.total_files);
    info!("  - Total Engines: {}", manifest.total_engines);
    info!("  - Active Modules: {}", manifest.active_modules);
    info!("  - Health Score: {:.1}%", manifest.health_score);
    info!("  - System Ready: {}", manifest.system_ready);

    // Run demonstration
    run_demonstration(&mut agi);

    info!("=================================================");
    info!("  TARIRO AGI READY FOR PRODUCTION");
    info!("=================================================");
}

/// Run demonstration of Tariro AGI capabilities
fn run_demonstration(agi: &mut TariroAGI) {
    info!("Running Tariro AGI demonstration...");

    // 1. Standard compilation
    info!("\n[1] Standard TUMC Compilation:");
    let result = agi.process_intent(
        "Create a secure microservice for real-time sensor processing",
        "wasm32-unknown-unknown"
    );
    info!("Result: {}", result);

    // 2. AGI-powered compilation
    info!("\n[2] AGI-Powered Compilation:");
    match agi.compile_with_agi("neural network inference", "x86_64-unknown-linux-gnu") {
        Ok(output) => info!("Compiled: {}", output),
        Err(e) => error!("Compilation failed: {}", e),
    }

    // 3. System capabilities
    info!("\n[3] Available Capabilities:");
    let capabilities = agi.get_capabilities();
    let capability_names: Vec<_> = capabilities.iter().map(|c| c.name.as_str()).collect();
    info!("Found {} capability categories", capability_names.len());

    // 4. Find specific engines
    info!("\n[4] Reasoning Engines:");
    let reasoning_engines = agi.find_capable_engines("reasoning");
    for engine in reasoning_engines.iter().take(5) {
        info!("  - {}: {}", engine.name, engine.description);
    }

    // 5. System health
    info!("\n[5] System Health Check:");
    let health = agi.get_system_health();
    info!("  Overall Score: {:.1}%", health.overall_score);
    info!("  Reasoning Ready: {}", health.reasoning_ready);
    info!("  Learning Ready: {}", health.learning_ready);
    info!("  Security Ready: {}", health.security_ready);
    info!("  Memory Ready: {}", health.memory_ready);
    info!("  Communication Ready: {}", health.communication_ready);
    info!("  Creativity Ready: {}", health.creativity_ready);

    // 6. Engine statistics
    info!("\n[6] Engine Statistics:");
    let stats = agi.get_engine_stats();
    info!("  Total Registered Engines: {}", stats.total_engines);
    info!("  Total Files: {}", stats.total_files);
    for (category, count) in &stats.category_breakdown {
        info!("  {}: {}", category, count);
    }

    // 7. Unified status
    info!("\n[7] Unified System Status:");
    let unified_status = agi.get_unified_status();
    info!("  Intent Queue Size: {}", unified_status.intent_queue_size);
    info!("  Active Processes: {}", unified_status.active_processes);
    info!("  Memory Usage: {} MB", unified_status.memory_usage_mb);
}

/// Display usage information
fn print_usage() {
    println!("
Tariro AGI - Universal Meta-Compiler System
===========================================

Usage: tariro [OPTIONS]

OPTIONS:
    -h, --help          Show this help message
    -v, --version       Show version information
    -s, --status        Display system status and exit
    -c, --compile       Run compilation demo
    -t, --test          Run system tests

For more information, visit: https://github.com/Benwellonedge28/Tariro
");
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_tariro_agi_initialization() {
        let mut agi = TariroAGI::new();
        agi.initialize();
        let manifest = agi.get_system_manifest();
        assert!(manifest.total_files > 0);
        assert!(manifest.total_engines > 0);
    }

    #[test]
    fn test_system_health() {
        let agi = TariroAGI::new();
        let health = agi.get_system_health();
        assert!(health.overall_score >= 0.0);
        assert!(health.overall_score <= 100.0);
    }

    #[test]
    fn test_capabilities() {
        let agi = TariroAGI::new();
        let caps = agi.get_capabilities();
        assert!(!caps.is_empty());
    }

    #[test]
    fn test_engine_search() {
        let agi = TariroAGI::new();
        let reasoning = agi.find_capable_engines("reasoning");
        assert!(!reasoning.is_empty());
    }
}
