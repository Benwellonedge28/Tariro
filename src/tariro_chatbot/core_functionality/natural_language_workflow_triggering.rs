// src/tariro_chatbot/core_functionality/natural_language_workflow_triggering.rs

pub struct NaturalLanguageWorkflowTriggering;

impl NaturalLanguageWorkflowTriggering {
    pub fn trigger_workflow(&self, workflow_name: &str) -> String {
        format!("Natural Language Workflow Triggering: Initiated complex automated workflow '{}' directly through chat.", workflow_name)
    }
}
