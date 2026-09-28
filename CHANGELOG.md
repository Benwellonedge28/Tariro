# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.12] - 2024-09-28

### Added
- Production readiness infrastructure with comprehensive CI/CD
- Clippy and rustfmt configuration files for code quality
- Integration test suite (`tests/tariro_lib.rs`)
- Safety audit documentation (no unsafe blocks found)
- Complete TariroError enum with Display and Error implementations
- Utility functions: `generate_id()` and `generate_uuid()`

### Fixed
- Compilation errors (821 → 0 errors)
- Import infrastructure with centralized prelude module
- Borrow checker issues in zero_trust, world_model, causal_reasoning_engine
- Missing async methods in quantum_security
- Trait bound issues with f32 types (Eq, Hash derives)
- chrono API compatibility (TimeDelta vs Duration)
- dyn Trait object Debug/Clone implementations

### Changed
- MSRV updated to 1.70.0
- Enhanced AGIConfig with self-modification settings
- Updated CI workflow to use project-specific configurations
- Improved error handling and documentation

## [1.0.11] - Previous Release

### Added
- TUMC (Tariro Universal Meta-Compiler) core implementation
- 85+ specialized AGI engines
- Consciousness and Theory of Mind engines
- Multi-agent orchestration capabilities
- Quantum-safe security primitives

### Changed
- Major architectural refactoring
- Enhanced cross-platform compilation support
