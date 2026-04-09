//! POCO-REAF System: Universal & Self-Managing Runtime Abstraction Layer.
//!
//! This module provides the core components for the Program-and-Compile-Once-Run-Everywhere-Anywhere-Forever (POCO-REAF) system,
//! ensuring platform independence, self-management, and perpetual executability for generated software.

pub mod uee_core;
pub mod self_healing;
pub mod adaptive_optimizer;
pub mod migration_lb; // NEW: Integrate migration_lb module

pub struct PocoReafSystem {
    uee_core_engine: uee_core::UeeCore,
    self_healing_engine: self_healing::SelfHealingEngine,
    adaptive_optimizer_engine: adaptive_optimizer::AdaptiveOptimizerEngine,
    migration_lb_engine: migration_lb::MigrationLoadBalancerEngine, // NEW: MigrationLoadBalancerEngine instance
}

impl PocoReafSystem {
    pub fn new() -> Self {
        PocoReafSystem {
            uee_core_engine: uee_core::UeeCore::new(),
            self_healing_engine: self_healing::SelfHealingEngine::new(),
            adaptive_optimizer_engine: adaptive_optimizer::AdaptiveOptimizerEngine::new(),
            migration_lb_engine: migration_lb::MigrationLoadBalancerEngine::new(), // NEW: Initialize MigrationLoadBalancerEngine
        }
    }

    /// Executes a given artifact within the Universal Execution Environment.
    pub fn execute_artifact(&self, artifact_id: &str, target_platform: &str) -> String {
        println!("POCO-REAF: Executing artifact '{}' on target platform '{}'.", artifact_id, target_platform);
        let uee_status = self.uee_core_engine.run_application(artifact_id, target_platform);
        self.self_healing_engine.monitor_application(artifact_id);
        self.adaptive_optimizer_engine.monitor_and_optimize(artifact_id);
        self.migration_lb_engine.monitor_and_balance(artifact_id); // NEW: Initiate migration/load balancing
        uee_status
    }

    /// Initiates a self-healing process for a given application or service.
    pub fn initiate_self_healing(&self, application_id: &str) -> String {
        println!("POCO-REAF: Initiating self-healing for application '{}'.", application_id);
        self.self_healing_engine.trigger_recovery(application_id)
    }

    /// Deploys an artifact within the POCO-REAF managed environment.
    pub fn deploy_artifact(&self, artifact_id: &str, target_platform: &str, deployment_strategy: &str) -> String {
        println!("POCO-REAF: Deploying artifact '{}' to '{}' with strategy '{}'.", artifact_id, target_platform, deployment_strategy);
        self.uee_core_engine.run_application(artifact_id, target_platform);
        format!("Artifact '{}' deployed to '{}' via POCO-REAF.", artifact_id, target_platform)
    }

    /// Initiates autonomous migration for a specified application.
    pub fn initiate_migration(&self, application_id: &str, target_env: &str) -> String {
        println!("POCO-REAF: Initiating migration for '{}' to '{}'.", application_id, target_env);
        self.migration_lb_engine.migrate_application(application_id, target_env)
    }
}
