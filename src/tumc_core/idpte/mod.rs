//! Integrated Development, Preview, Test & Deployment Environment (IDPTE) Core Module.
//! 
//! This module orchestrates the user-facing and backend components of the IDPTE,
//! providing a seamless experience for interacting with the Universal Meta-Compiler (TUMC).

pub mod ide_backend;
pub mod live_preview; // NEW: Integrate the live_preview module

pub struct IapteCore {
    ide_backend_engine: ide_backend::IdeBackend,
    live_preview_engine: live_preview::LivePreviewEngine, // NEW: Live Preview Engine instance
}

impl IapteCore {
    pub fn new() -> Self {
        IapteCore {
            ide_backend_engine: ide_backend::IdeBackend::new(),
            live_preview_engine: live_preview::LivePreviewEngine::new(), // NEW: Initialize Live Preview Engine
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
}
