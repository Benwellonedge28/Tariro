// src/admin_voice_interface/document_config_policy/bulk_voice_persona_import_export.rs

pub struct BulkVoicePersonaManager;

impl BulkVoicePersonaManager {
    pub fn manage_bulk_personas(&self, action: &str, format: &str) -> String {
        format!("{} hundreds of voice personas via {} format.", action, format)
    }
}
