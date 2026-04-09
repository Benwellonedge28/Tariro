//! Integrated Development, Preview, Test & Deployment Environment (IDPTE) Core Module.
//! 
//! This module orchestrates the user-facing and backend components of the IDPTE,
//! providing a seamless experience for interacting with the Universal Meta-Compiler (TUMC).

pub mod ide_backend;

pub struct IapteCore {
    ide_backend_engine: ide_backend::IdeBackend,
}

impl IapteCore {
    pub fn new() -> Self {
        IapteCore {
            ide_backend_engine: ide_backend::IdeBackend::new(),
        }
    }

    pub fn handle_ide_request(&self, request: &str) -> String {
        println!("IDPTE: Handling IDE request: {}", request);
        self.ide_backend_engine.process_request(request)
    }
}
