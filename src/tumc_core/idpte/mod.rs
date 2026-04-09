//! Integrated Development, Preview, Test & Deployment Environment (IDPTE) Core Module.
//! 
//! This module orchestrates the user-facing and backend components of the IDPTE,
//! providing a seamless experience for interacting with the Universal Meta-Compiler (TUMC).

pub mod ide_backend;
pub mod live_preview;
pub mod test_orchestration; // NEW: Integrate the test_orchestration module

pub struct IapteCore {
    ide_backend_engine: ide_backend::IdeBackend,
    live_preview_engine: live_preview::LivePreviewEngine,
    test_orchestration_engine: test_orchestration::TestOrchestrationEngine, // NEW: Test Orchestration Engine instance
}

impl IapteCore {
    pub fn new() -> Self {
        IapteCore {
            ide_backend_engine: ide_backend::IdeBackend::new(),
            live_preview_engine: live_preview::LivePreviewEngine::new(),
            test_orchestration_engine: test_orchestration::TestOrchestrationEngine::new(), // NEW: Initialize Test Orchestration Engine
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
}
