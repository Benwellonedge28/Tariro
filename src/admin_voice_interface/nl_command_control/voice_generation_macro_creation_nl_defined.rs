// src/admin_voice_interface/nl_command_control/voice_generation_macro_creation_nl_defined.rs

use crate::prelude::*;
pub struct VoiceMacroCreator;

impl VoiceMacroCreator {
    pub fn create_macro(&self, macro_name: &str, description: &str) -> String {
        format!("Defined and saved complex voice generation workflow '{}' as a reusable macro using NL: {}.", macro_name, description)
    }
}
