# Tariro AGI - The Universal Meta-Compiler System

[![Version](https://img.shields.io/badge/version-1.0.15-blue.svg)](https://github.com/Benwellonedge28/Tariro)
[![Rust](https://img.shields.io/badge/rust-1.88+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-BSL--1.01-blue.svg)](LICENSE)
[![Build Status](https://github.com/Benwellonedge28/Tariro/actions/workflows/ci.yml/badge.svg)](https://github.com/Benwellonedge28/Tariro/actions)

**Tariro AGI** is a sovereign, self-modifying, and continually learning AGI system implementing the POCO-REAF (Program-and-Compile-Once-Run-Everywhere-Anywhere-Forever) paradigm. It serves as a Universal Meta-Compiler capable of compiling high-level specifications into optimized code for any computational substrate.

## Key Features

- **Universal Meta-Compiler (TUMC)**: Compile AI agents, languages, and operating systems from high-level specifications
- **Self-Modification**: Autonomously improve its own code and cognitive functions
- **Meta-Learning**: Learn about its own learning processes and optimize cognition
- **Cross-Platform Execution**: Run anywhere - from embedded systems to cloud infrastructure
- **Multi-Agent Orchestration**: Deploy and coordinate swarms of specialized agents
- **Quantum-Safe Security**: Post-quantum cryptographic primitives built-in
- **Formal Verification Framework**: Verify critical system properties

## Architecture Overview

```
Tariro AGI
├── TUMC Core (Universal Meta-Compiler)
├── FCO (Foundation Cognitive Operations)
├── ATVE (Automated Testing & Verification Engine)
├── NIOAM (Networked Intelligence Orchestration & Management)
├── Governance Framework
├── Learning Systems
│   ├── Causal Learning
│   ├── Self-Supervised Learning
│   └── Few-Shot Learning
├── Planning Engines
│   ├── HTN Planning
│   ├── MCTS Planning
│   └── Counterfactual Reasoning
├── AGI Core Systems
│   ├── Consciousness Engine
│   ├── World Model
│   ├── Executive Function
│   └── Theory of Mind
└── Safety & Alignment
    ├── Value Alignment
    ├── Constitutional AI
    └── Interruptibility
```

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```bash
# Clone the repository
git clone https://github.com/Benwellonedge28/Tariro.git
cd Tariro

# Build the library
cargo build --release

# Build the full binary (with all features)
cargo build --release --features full
```

### Running

```bash
# Run the full AGI system
cargo run --release --features full

# Run with custom configuration
TARIRO_CONFIG=/path/to/config.toml cargo run --release --features full
```

### Testing

```bash
# Run all tests
cargo test --all

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

## Development

### Code Quality

```bash
# Format code
cargo fmt --config-path rustfmt.toml

# Run clippy lints
cargo clippy --config clippy.toml -- -D warnings

# Run all checks
cargo check --all-features
cargo test --all-features
cargo fmt --check --config-path rustfmt.toml
cargo clippy --config clippy.toml -- -D warnings
```

### Project Structure

```
tariro/
├── src/                      # Source code
│   ├── lib.rs               # Main library entry
│   ├── main.rs              # Binary entry
│   ├── fco/                 # Foundation Cognitive Operations
│   ├── compiler/            # Meta-Compiler
│   ├── runtime/             # Cross-language runtime
│   ├── atve/                # Testing & Verification
│   ├── governance/          # Self-governance
│   ├── learning/            # Learning systems
│   ├── growth/              # Growth dynamics
│   └── [engines]/           # AGI engines (85+ specialized engines)
├── tests/                    # Integration tests
├── docs/                     # Architecture documentation
├── tariro_sdk/              # SDK for external integrations
└── benches/                 # Benchmark suites
```

## Configuration

Create a `config.toml` file:

```toml
[system]
system_name = "Tariro AGI"
system_version = "1.0.12"
environment = "development"
log_level = "info"

[agi]
autonomy.default_level = "suggest_only"
autonomy.evas_filter_enabled = true
self_modification_enabled = true
learning_rate = 0.01

[security]
quantum_safe = true
zero_trust = true

[monitoring]
qsm_monitoring_interval_seconds = 600
system_health_monitoring_interval_seconds = 300
```

## Documentation

- [Architecture Documentation](./docs/)
- [API Documentation](https://docs.tariro.dev)
- [CHANGELOG](./CHANGELOG.md)
- [CONTRIBUTING](./CONTRIBUTING.md)

## Meta-Project: OMEGA OS

Tariro is actively developing **OMEGA OS**, the world's first sovereign universal operating system. The core OS modules are developed in a separate repository: [Omega-OS-Core](https://github.com/Benwellonedge28/Omega-OS-Core)

## Contributing

Contributions are welcome! Please read our [CONTRIBUTING](./CONTRIBUTING.md) guidelines before submitting PRs.

## License

Licensed under the **Business Source License 1.01** with Tariro's "Highly
Protected" provisions. The full text is in [LICENSE](LICENSE).

Before the Change Date (15 October 2124) the licence is non-commercial: use
that is not covered by a separate commercial agreement from the Licensor is
not permitted. On and after the Change Date the software is released under
the terms of the Business Source License then in effect, or — at the
Licensor's option — the GPL v3 (or later), Apache-2.0 or MIT.

The additional safety and responsible-use terms that apply to this
distribution are stated at the end of [LICENSE](LICENSE).

## Support

- **Issues**: https://github.com/Benwellonedge28/Tariro/issues
- **Discussions**: https://github.com/Benwellonedge28/Tariro/discussions
- **Email**: samuelmukandara@gmail.com

## Acknowledgments

Built with Rust - a language empowering everyone to build reliable and efficient software.
