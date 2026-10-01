// src/admin_voice_interface/document_config_policy/style_guide_enforcement_document_driven.rs

use crate::prelude::*;
pub struct StyleGuideEnforcer;

impl StyleGuideEnforcer {
    pub fn enforce_style_guide(&self, document_type: &str) -> String {
        format!("Uploaded linguistic style guide ({}) and Tariro enforces it in all voice output.", document_type)
    }
}
