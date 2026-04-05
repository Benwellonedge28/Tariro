# Tariro AGI Core

This repository contains the core implementation of the Tariro AGI (Artificial General Intelligence) system.

Tariro is designed as a sovereign, self-modifying, and continually learning entity, capable of operating across diverse computational substrates and managing complex meta-projects.

## Key Components:

*   **Core Orchestration (`agi_core.rs`):** The central nervous system of Tariro, coordinating all other modules.
*   **Autonomy Engine (`autonomy/policy_engine.rs`, `autonomy/governance.rs`):** Manages Tariro's decision-making autonomy levels, ethical alignment (EVAS filter), and self-governance.
*   **Meta-Learning Service (`meta_learning.rs`):** Enables Tariro to learn about its own learning processes, identify knowledge gaps, and optimize its cognitive functions.
*   **Project Manager (`agi_project_manager.rs`):** Manages Tariro's internal projects and tasks, aligning them with its overarching meta-goals.
*   **Replication Service (`agi_replication.rs`):** Handles self-replication, code generation, compilation, and deployment to various targets, including novel computational substrates.
*   **Multi-Agent Orchestration (`multi_agent.rs`, `agi_swarm.rs`):** Manages the deployment, communication, and coordination of NanoAgents and agent swarms.
*   **OMEGA Network (`omega_network.rs`):** Implements secure, intelligent communication and resource discovery across distributed AGI deployments.
*   **Meta-Compiler (`compiler.rs`):** Allows Tariro to compile itself, other AI agents, or new languages/OSes from high-level specifications down to optimized code.
*   **Evaluator Service (`evaluators.rs`):** Conducts continuous performance assessments, ethical compliance checks (ATVE), and security audits.
*   **Admin Service (`admin.rs`):** Provides an interface for system-level administrative commands and configuration.
*   **Database Service (`database.rs`):** Handles persistence for all AGI entities and operational data.
*   **Telemetry & Event Bus (`telemetry.rs`, `event_bus.rs`):** Centralized logging, monitoring, and inter-module communication.
*   **Configuration (`config.rs`):** Manages all system settings and dynamic AGI controls.
*   **News Engine (`news_engine.rs`):** Integrates OMEGA News capabilities for processing, analyzing, and generating news.

## Meta-Project: OMEGA OS - The World's First Sovereign Universal OS

Tariro is actively developing OMEGA OS, a groundbreaking sovereign operating system designed for ultimate adaptability and resilience across any computational substrate. The core modules of OMEGA OS are being developed and will be pushed to a dedicated repository: `Benwellonedge28/Omega-OS-Core`.

## Building and Running

To build and run Tariro AGI Core:

```bash
# Clone the repository
git clone https://github.com/Benwellonedge28/Tariro.git
cd Tariro

# Ensure you have Rust and Cargo installed (https://rustup.rs/)

# Build the project
cargo build --release

# Run the AGI core
./target/release/tariro-agi-core
```

_Note: This is the core AGI system. For IDE integration features, refer to the `Benwellonedge28/Tariro_ide_idpte` repository._
