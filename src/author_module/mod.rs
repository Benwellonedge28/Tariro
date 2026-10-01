use crate::prelude::*;
pub mod content_generation;

#[derive(Default)]
pub struct AuthorModuleEngine {
    pub content_generation: content_generation::ContentGeneration,
}

impl AuthorModuleEngine {
    pub fn new() -> Self {
        AuthorModuleEngine {
            content_generation: content_generation::ContentGeneration::new(),
        }
    }
}
