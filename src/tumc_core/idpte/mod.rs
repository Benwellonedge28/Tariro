//! Integrated Development, Preview, Test & Deployment Environment (IDPTE) Core Module.
//! 
//! This module orchestrates the user-facing and backend components of the IDPTE,
//! providing a seamless experience for interacting with the Universal Meta-Compiler (TUMC).

pub mod ide_backend;
pub mod live_preview;
pub mod test_orchestration;
pub mod autonomous_deployment;
pub mod agent_bridge; // NEW: Integrate agent_bridge module

pub struct IapteCore {
    ide_backend_engine: ide_backend::IdeBackend,
    live_preview_engine: live_preview::LivePreviewEngine,
    test_orchestration_engine: test_orchestration::TestOrchestrationEngine,
    autonomous_deployment_engine: autonomous_deployment::AutonomousDeploymentEngine,
    agent_bridge_engine: agent_bridge::AgentBridgeEngine, // NEW: Agent Bridge Engine instance
}

impl IapteCore {
    pub fn new() -> Self {
        IapteCore {
            ide_backend_engine: ide_backend::IdeBackend::new(),
            live_preview_engine: live_preview::LivePreviewEngine::new(),
            test_orchestration_engine: test_orchestration::TestOrchestrationEngine::new(),
            autonomous_deployment_engine: autonomous_deployment::AutonomousDeploymentEngine::new(),
            agent_bridge_engine: agent_bridge::AgentBridgeEngine::new(), // NEW: Initialize Agent Bridge Engine
        }
    }

    pub fn handle_ide_request(&self, request: &str) -> String {
        println!("IDPTE: Handling IDE request: {}", request);
        self.ide_backend_engine.process_request(request)
    }

    /// Simulates rendering a preview of a generated artifact.
    pub fn render_preview(&self, artifact_id: &str, view_mode: &str) -> String {
        println!("IDPTE: Requesting live preview for artifact {} in {} mode.", artifact_id, view_mode);
        self.live_preview_engine.render_artifact_preview(artifact_id, view_mode)
    }

    /// Simulates visualizing a Universal Semantic Graph (USG).
    pub fn visualize_usg(&self, usg_data: &str) -> String {
        println!("IDPTE: Requesting USG visualization for data: {}.", usg_data);
        self.live_preview_engine.visualize_semantic_graph(usg_data)
    }

    /// Triggers an ATVE test suite and returns a summary report.
    pub fn trigger_atve_test(&self, target_entity: &str, entity_type: &str) -> String {
        println!("IDPTE: Triggering ATVE test for {} (Type: {}).". target_entity, entity_type);
        self.test_orchestration_engine.run_atve_suite(target_entity, entity_type)
    }

    /// Retrieves a detailed ATVE report for a given test session.
    pub fn get_atve_report(&self, session_id: &str) -> String {
        println!("IDPTE: Retrieving ATVE report for session {}.". session_id);
        self.test_orchestration_engine.retrieve_report(session_id)
    }

    /// Initiates an autonomous deployment of a generated artifact.
    pub fn initiate_deployment(&self, artifact_id: &str, target_env: &str, strategy: &str) -> String {
        println!("IDPTE: Initiating autonomous deployment of artifact {} to {} with strategy {}.". artifact_id, target_env, strategy);
        self.autonomous_deployment_engine.deploy_artifact(artifact_id, target_env, strategy)
    }

    /// Monitors the status of an ongoing deployment.
    pub fn monitor_deployment_status(&self, deployment_id: &str) -> String {
        println!("IDPTE: Monitoring deployment {}.". deployment_id);
        self.autonomous_deployment_engine.get_deployment_status(deployment_id)
    }

    /// Sends a command via the agent bridge to an external IDE.
    pub fn send_ide_command(&self, ide_id: &str, command: &str) -> String {
        println!("IDPTE: Sending command '{}' to IDE {}.". command, ide_id);
        self.agent_bridge_engine.send_command_to_ide(ide_id, command)
    }

    /// Receives data from an external agent.
    pub fn receive_agent_data(&self, agent_id: &str, data: &str) -> String {
        println!("IDPTE: Receiving data from agent {}: {}.". agent_id, data);
        self.agent_bridge_engine.receive_data_from_agent(agent_id, data)
    }
}
