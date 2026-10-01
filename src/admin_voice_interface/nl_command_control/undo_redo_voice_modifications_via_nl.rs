// src/admin_voice_interface/nl_command_control/undo_redo_voice_modifications_via_nl.rs

use crate::prelude::*;
pub struct VoiceModificationHistory;

impl VoiceModificationHistory {
    pub fn undo_redo_modification(&self, action: &str) -> String {
        format!("Reverted or reapplied previous voice generation changes with natural language command: {}.", action)
    }
}
