// src/admin_voice_interface/auditing_explainability/why_this_voice_query_interface.rs

pub struct VoiceQueryInterface;

impl VoiceQueryInterface {
    pub fn query_voice_decision(&self, voice_id: &str) -> String {
        format!("Allowed admins to query why a specific voice characteristic or emotional inflection was generated for output {}.", voice_id)
    }
}
