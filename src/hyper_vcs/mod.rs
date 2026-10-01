use crate::prelude::*;
pub mod core_vcs;

#[derive(Default)]
pub struct HyperVcsEngine {
    pub core_vcs: core_vcs::CoreVcs,
}

impl HyperVcsEngine {
    pub fn new() -> Self {
        HyperVcsEngine {
            core_vcs: core_vcs::CoreVcs::new(),
        }
    }
}
