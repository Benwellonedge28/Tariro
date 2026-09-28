# Tariro AGI API Reference

## Core Types

### TariroAGI

The main entry point for the Tariro AGI system.

```rust
use tariro::TariroAGI;

// Create and initialize
let mut agi = TariroAGI::new();
agi.initialize();

// Process intents
let result = agi.process_intent("analyze data", "wasm");

// Get system status
let status = agi.get_status();
```

### AGIConfig

Configuration for the Tariro AGI system.

```rust
use tariro::AGIConfig;

// Default configuration
let config = AGIConfig::default();

// Load from file
let config = AGIConfig::load("config.toml").unwrap();

// Save configuration
config.save("config.toml").unwrap();

// Dynamic settings
config.update_setting("autonomy.level", "high");
let value = config.get_setting("autonomy.level");
```

### TariroError

Main error type for Tariro operations.

```rust
use tariro::TariroError;

let error = TariroError::Configuration("missing setting".to_string());
println!("{}", error); // Configuration error: missing setting
```

Variants:
- `Configuration(String)` - Configuration errors
- `Runtime(String)` - Runtime errors
- `Compilation(String)` - Compilation errors
- `Serialization(String)` - Serialization errors
- `Network(String)` - Network errors
- `Security(String)` - Security errors
- `Unknown(String)` - Unknown errors

### Utility Functions

```rust
use tariro::{generate_id, generate_uuid};

// Generate unique IDs
let id = generate_id();      // UUID v4 string
let uuid = generate_uuid();  // Same as generate_id
```

### Infinitism

Represents values that can be finite, negative infinity, or positive infinity.

```rust
use tariro::Infinitism;

let finite = Infinitism::Finite(42);
let neg_inf = Infinitism::NegativeInfinity;
let pos_inf = Infinitism::PositiveInfinity;

// Comparison
assert!(neg_inf < finite);
assert!(finite < pos_inf);
```

### MukandaraTime

The absolute smallest quantum of temporality.

```rust
use tariro::MukandaraTime; // u128

let timestamp: MukandaraTime = 1234567890;
```

## Prelude

The prelude module provides common types:

```rust
use tariro::prelude::*;

// chrono types
let now: DateTime<Utc> = Utc::now();
let duration = ChronoDuration::seconds(60);
```

## Re-exports

Key types re-exported for convenience:

```rust
use tariro::{
    TariroUniversalMetaCompiler,
    MukandaraState,
    GrowthDynamics,
    TestResult,
    AGIConfig,
};
```

## Module Overview

### Core Framework
- `fco` - Foundation Cognitive Operations
- `runtime` - Cross-language runtime
- `compiler` - Universal Meta-Compiler
- `atve` - Automated Testing & Verification
- `nioam` - Networked Intelligence Orchestration
- `governance` - Self-governance framework
- `learning` - Learning systems
- `growth` - Growth dynamics

### AGI Engines
- 85+ specialized engines including:
  - `consciousness_engine` - Consciousness processing
  - `world_model` - World modeling
  - `executive_function` - Executive control
  - `theory_of_mind` - Social cognition
  - `causal_reasoning_engine` - Causal reasoning
  - `creativity_engine` - Creative processes

### Safety & Alignment
- `value_alignment` - Value alignment
- `constitutional_ai` - Constitutional AI
- `safety_constraints` - Safety constraints
- `interruptibility` - Interruptibility support
- `shutdown_mechanism` - Graceful shutdown

### Security
- `quantum_security` - Quantum-safe security
- `zero_trust` - Zero trust architecture
- `adversarial_defense` - Adversarial defense
- `trusted_execution` - Trusted execution

### Advanced Capabilities
- `htn_planning` - Hierarchical Task Network planning
- `mcts_planning` - Monte Carlo Tree Search planning
- `counterfactual_reasoning` - Counterfactual reasoning
- `causal_learning` - Causal learning
- `self_supervised_learning` - Self-supervised learning
- `few_shot_learning` - Few-shot learning

## Feature Flags

```toml
[dependencies]
tariro = { version = "1.0", features = ["full"] }
```

Available features:
- `std` - Standard library support (default)
- `async-runtime` - Async runtime support (Tokio)
- `full` - All features enabled
- `metrics` - Prometheus metrics
- `profiling` - Tracing-based profiling
- `mongodb` - MongoDB database support
