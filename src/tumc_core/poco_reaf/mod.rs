//! POCO-REAF System: Universal & Self-Managing Runtime Abstraction Layer.
//!
//! This module provides the core components for the Program-and-Compile-Once-Run-Everywhere-Anywhere-Forever (POCO-REAF) system,
//! ensuring platform independence, self-management, and perpetual executability for generated software.

pub mod uee_core;
pub mod self_healing; // NEW: Integrate self_healing module

pub struct PocoReafSystem {
    uee_core_engine: uee_core::UeeCore,
    self_healing_engine: self_healing::SelfHealingEngine, // NEW: SelfHealingEngine instance
}

impl PocoReafSystem {
    pub fn new() -> Self {
        PocoReafSystem {
            uee_core_engine: uee_core::UeeCore::new(),
            self_healing_engine: self_healing::SelfHealingEngine::new(), // NEW: Initialize SelfHealingEngine
        }
    }

    /// Executes a given artifact within the Universal Execution Environment.
    pub fn execute_artifact(&self, artifact_id: &str, target_platform: &str) -> String {
        println!("POCO-REAF: Executing artifact '{}' on target platform '{}'.", artifact_id, target_platform);
        let uee_status = self.uee_core_engine.run_application(artifact_id, target_platform);
        // After execution, self-healing might monitor
        self.self_healing_engine.monitor_application(artifact_id);
        uee_status
    }

    /// Initiates a self-healing process for a given application or service.
    pub fn initiate_self_healing(&self, application_id: &str) -> String {
        println!("POCO-REAF: Initiating self-healing for application '{}'.", application_id);
        self.self_healing_engine.trigger_recovery(application_id)
    }
}
