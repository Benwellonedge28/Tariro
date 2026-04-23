// src/admin_voice_interface/nl_command_control/voice_generation_status_updates_via_nl.rs

pub struct VoiceStatusUpdater;

impl VoiceStatusUpdater {
    pub fn get_status_update(&self, task_id: &str) -> String {
        format!("Requested real-time updates on the status of voice generation task {}.
", task_id)
    }
}
