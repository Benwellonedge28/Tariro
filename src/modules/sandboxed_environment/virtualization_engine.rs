// src/modules/sandboxed_environment/virtualization_engine.rs

//! This module defines the Virtualization Engine component of the Sandboxed Environment Module.
//! Its primary responsibility is to create and manage isolated, full-stack replicas of Tariro's
//! current production environment for safe testing.

use crate::{prelude::*,
    Module,
    ModuleContext,
    FeatureProposalId,
    VirtualizationEngineConfig, // Import from lib.rs
};
use async_trait::async_trait;
use std::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;

/// Represents the Virtualization Engine responsible for managing sandbox instances.
pub struct VirtualizationEngine {
    config: VirtualizationEngineConfig,
    // In a real implementation, this would be a client to a container/VM orchestration system
    // e.g., `docker_cli: DockerClient` or `kubernetes_client: KubeClient`
}

// VirtualizationEngineConfig is now defined in lib.rs, removed from here.

#[async_trait]
impl Module for VirtualizationEngine {
    fn new(context: &ModuleContext) -> Result<Self> {
        // Load configuration from context. If not found, use defaults.
        let config = context.config.virtualization_engine.clone().unwrap_or_default();
        Ok(Self { config })
    }

    async fn start(&self) -> Result<()> {
        info!("VirtualizationEngine starting...");
        // Validate Docker or other virtualization dependencies are available
        match Command::new("docker").arg("info").output() {
            Ok(output) if output.status.success() => {
                info!("Docker daemon is running and accessible.");
            }
            _ => {
                error!("Docker daemon not found or not running. VirtualizationEngine cannot start.");
                return Err(anyhow!("Docker daemon not available."));
            }
        }
        info!("VirtualizationEngine started.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        info!("VirtualizationEngine stopping...");
        // No specific stop logic for a client, active sandboxes would be handled by deprovision_sandbox
        info!("VirtualizationEngine stopped.");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "VirtualizationEngine"
    }
}

impl VirtualizationEngine {
    /// Provisions a new isolated sandbox environment using Docker.
    /// This involves creating a replica of the Tariro production environment.
    pub async fn provision_sandbox(&self, feature_proposal_id: &FeatureProposalId) -> Result<String> {
        let sandbox_id = format!("tariro-sandbox-{}-{}", feature_proposal_id.0, uuid::Uuid::new_v4().to_string().splitn(2, '-').next().unwrap_or("unknown"));
        info!("Provisioning sandbox '{}' for feature proposal: {}", sandbox_id, feature_proposal_id.0);

        // 1. Create an isolated Docker network for the sandbox
        let network_name = format!("{}-net", sandbox_id);
        let network_create_output = TokioCommand::new("docker")
            .args(&["network", "create", "--driver", "bridge", &network_name])
            .output()
            .await?;

        if !network_create_output.status.success() {
            error!("Failed to create Docker network {}: {}", network_name, String::from_utf8_lossy(&network_create_output.stderr));
            return Err(anyhow!("Failed to create sandbox network."));
        }
        info!("Created Docker network: {}", network_name);

        // 2. Run the sandbox container
        let container_name = format!("{}-container", sandbox_id);
        let mut cmd = TokioCommand::new("docker");
        cmd.args(&["run", "-d",
            "--name", &container_name,
            "--network", &network_name,
            "--hostname", &sandbox_id,
            "--cpus", &self.config.resource_limits_cpu,
            "--memory", &self.config.resource_limits_memory,
            &self.config.sandbox_base_image,
            // Command to keep the container running and Tariro AGI core active
            "sh", "-c", "while true; do sleep 10; done"
        ]);

        info!("Running docker command: {:?}", cmd);
        let container_run_output = cmd.output().await?;

        if !container_run_output.status.success() {
            error!("Failed to run sandbox container {}: {}", container_name, String::from_utf8_lossy(&container_run_output.stderr));
            // Attempt to clean up the network if container creation failed
            let _ = TokioCommand::new("docker").args(&["network", "rm", &network_name]).output().await;
            return Err(anyhow!("Failed to run sandbox container."));
        }
        let container_id = String::from_utf8_lossy(&container_run_output.stdout).trim().to_string();
        info!("Sandbox container '{}' (ID: {}) provisioned successfully.", container_name, container_id);

        Ok(sandbox_id)
    }

    /// De-provisions an existing sandbox environment, releasing its resources.
    pub async fn deprovision_sandbox(&self, sandbox_id: &str) -> Result<()> {
        info!("De-provisioning sandbox: {}", sandbox_id);

        let container_name = format!("{}-container", sandbox_id);
        let network_name = format!("{}-net", sandbox_id);

        // 1. Stop and remove the container
        let stop_output = TokioCommand::new("docker").args(&["rm", "-f", &container_name]).output().await?;
        if !stop_output.status.success() {
            warn!("Failed to stop/remove container {}: {}", container_name, String::from_utf8_lossy(&stop_output.stderr));
            // Don't fail entirely if container already gone, but log it
        } else {
            info!("Container {} stopped and removed.", container_name);
        }

        // 2. Remove the network
        let network_rm_output = TokioCommand::new("docker").args(&["network", "rm", &network_name]).output().await?;
        if !network_rm_output.status.success() {
            error!("Failed to remove Docker network {}: {}", network_name, String::from_utf8_lossy(&network_rm_output.stderr));
            return Err(anyhow!("Failed to remove sandbox network."));
        }
        info!("Network {} removed.", network_name);

        info!("Sandbox {} de-provisioned successfully.", sandbox_id);
        Ok(())
    }

    /// Snapshots the current state of the production environment for consistent sandbox initialization.
    /// This could involve creating a Docker image from the running production Tariro instance
    /// or saving configuration files and data. For this example, we'll focus on the image.
    pub async fn snapshot_production_environment(&self) -> Result<String> {
        info!("Taking snapshot of production environment...");
        
        // In a real scenario, we'd get the actual production container ID.
        // For now, let's assume a 'tariro-production' container exists.
        let prod_container_name = "tariro-production"; // This needs to be dynamically determined
        let snapshot_tag = format!("tariro/production-snapshot-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));

        // Commit the production container to a new image
        let commit_output = TokioCommand::new("docker")
            .args(&["commit", prod_container_name, &snapshot_tag])
            .output()
            .await?;

        if !commit_output.status.success() {
            error!("Failed to commit production container {}: {}", prod_container_name, String::from_utf8_lossy(&commit_output.stderr));
            return Err(anyhow!("Failed to snapshot production environment."));
        }
        info!("Production environment snapshot image '{}' created.", snapshot_tag);
        
        Ok(snapshot_tag)
    }
}
