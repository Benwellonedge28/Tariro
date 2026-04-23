// src/admin_voice_interface/document_config_policy/voice_profile_definition_via_yaml_json.rs

pub struct VoiceProfileDefiner;

impl VoiceProfileDefiner {
    pub fn define_voice_profile(&self, profile_name: &str, format: &str) -> String {
        format!("Defined comprehensive voice profile '{}' using {} structured data format.", profile_name, format)
    }
}
