// src/admin_voice_interface/nl_command_control/voice_model_selection_via_nl.rs

pub struct VoiceModelSelector;

impl VoiceModelSelector {
    pub fn select_voice_model(&self, model_description: &str) -> String {
        format!("Chose specific underlying voice generation model: {}.", model_description)
    }
}
