use crate::prelude::*;
pub mod core_engine;

#[derive(Default)]
pub struct ForestOfTreesEngine {
    pub core_engine: core_engine::CoreEngine,
}

impl ForestOfTreesEngine {
    pub fn new() -> Self {
        ForestOfTreesEngine {
            core_engine: core_engine::CoreEngine::new(),
        }
    }
}
