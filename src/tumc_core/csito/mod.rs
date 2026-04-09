//! CSITO System: Comprehensive Computer Science & IT Operationalization.
//!
//! This module translates Tariro's vast KnowledgeItems across every domain of Computer Science
//! and IT/ICT into executable, manageable, and optimized systems within the Meta-Compiler framework.

pub mod knowledge_translation;
pub mod monitoring_management;
pub mod deployment_provisioning; // NEW: Integrate deployment_provisioning module

pub struct CsitoSystem {
    knowledge_translation_layer: knowledge_translation::KnowledgeTranslationLayer,
    monitoring_management_integrator: monitoring_management::MonitoringManagementIntegrator,
    deployment_provisioning_engine: deployment_provisioning::DeploymentProvisioningEngine, // NEW: DeploymentProvisioningEngine instance
}

impl CsitoSystem {
    pub fn new() -> Self {
        CsitoSystem {
            knowledge_translation_layer: knowledge_translation::KnowledgeTranslationLayer::new(),
            monitoring_management_integrator: monitoring_management::MonitoringManagementIntegrator::new(),
            deployment_provisioning_engine: deployment_provisioning::DeploymentProvisioningEngine::new(), // NEW: Initialize DeploymentProvisioningEngine
        }
    }

    /// Translates high-level intent into operationalized IT systems.
    pub fn operationalize_intent(&self, intent_description: &str, artifact_id: &str, target_platform: &str) -> String {
        println!("CSITO: Operationalizing intent '{}' for artifact '{}' on '{}'.", intent_description, artifact_id, target_platform);
        let translated_config = self.knowledge_translation_layer.translate_knowledge(intent_description, target_platform);

        // Simulate monitoring initialization for the newly operationalized system
        self.monitoring_management_integrator.initialize_monitoring(artifact_id, target_platform);

        // Deploy and provision the system based on translated config
        let deployment_report = self.deployment_provisioning_engine.deploy_system(artifact_id, &translated_config, target_platform);

        format!("Intent operationalized. Translated config: {}. Artifact '{}' ready for managed IT lifecycle. Deployment: {}", translated_config, artifact_id, deployment_report)
    }

    /// Provides access to domain-specific IT operational knowledge.
    pub fn get_operational_knowledge(&self, topic: &str) -> String {
        println!("CSITO: Retrieving operational knowledge for topic '{}'.", topic);
        self.knowledge_translation_layer.translate_knowledge(topic, "conceptual_config")
    }

    /// Fetches a system's current monitoring status.
    pub fn get_system_status(&self, system_id: &str) -> String {
        println!("CSITO: Fetching monitoring status for system '{}'.", system_id);
        self.monitoring_management_integrator.get_status(system_id)
    }
}
