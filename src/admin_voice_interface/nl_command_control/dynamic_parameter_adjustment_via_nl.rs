// src/admin_voice_interface/nl_command_control/dynamic_parameter_adjustment_via_nl.rs

pub struct DynamicParameterAdjuster;

impl DynamicParameterAdjuster {
    pub fn adjust_parameter(&self, parameter: &str, value: &str) -> String {
        format!("Adjusted voice generation parameter '{}' to '{}' using natural language.", parameter, value)
    }
}
