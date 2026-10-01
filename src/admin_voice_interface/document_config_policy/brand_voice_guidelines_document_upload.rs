// src/admin_voice_interface/document_config_policy/brand_voice_guidelines_document_upload.rs

use crate::prelude::*;
pub struct BrandVoiceGuidelinesUploader;

impl BrandVoiceGuidelinesUploader {
    pub fn upload_brand_guidelines(&self, document_type: &str) -> String {
        format!("Uploaded extensive brand voice and tone guidelines ({}) for Tariro to learn and emulate in all its vocal interactions.", document_type)
    }
}
