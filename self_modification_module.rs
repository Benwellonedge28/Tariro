use crate::omega_news_content_generator_llm::ContentGeneratorLlm;
use serde::{Deserialize, Serialize};

pub struct SelfModificationModule {
    llm_processor: ContentGeneratorLlm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigChangeProposal {
    pub proposed_key: String,
    pub proposed_value: String,
    pub justification: String,
    pub impact_assessment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeRefactorProposal {
    pub original_code: String,
    pub refactored_code: String,
    pub rationale: String,
    pub estimated_improvement: String,
}

impl SelfModificationModule {
    pub fn new() -> Result<Self, String> {
        let llm_processor = ContentGeneratorLlm::new()?;
        Ok(SelfModificationModule { llm_processor })
    }

    pub async fn propose_config_change(&self, current_context: &str, desired_outcome: &str) -> Result<ConfigChangeProposal, String> {
        println!("Proposing config change based on desired outcome: {}", desired_outcome);
        let system_instruction = "You are an AI assistant designed to propose configuration changes for an AGI system. Your goal is to suggest optimal parameter values and provide clear justifications and impact assessments.";
        let prompt = format!(
            "Based on the current system context: '{}', and the desired outcome: '{}', propose a specific configuration change. 
            Identify the `proposed_key`, `proposed_value`, provide a `justification` for why this change is beneficial, and an `impact_assessment` describing its expected effects on the system. 
            Output your response as a JSON object with these fields.",
            current_context, desired_outcome
        );

        let llm_response_json = self.llm_processor.generate_content(&prompt, Some(system_instruction)).await?;
        
        // Attempt to parse JSON response directly
        let proposal: ConfigChangeProposal = serde_json::from_str(&llm_response_json)
            .map_err(|e| format!("Failed to parse LLM's config change proposal JSON: {}. Response: {}", e, llm_response_json))?;
        
        Ok(proposal)
    }

    pub async fn propose_code_refactor(&self, current_code: &str, refactoring_goal: &str, language: &str) -> Result<CodeRefactorProposal, String> {
        println!("Proposing code refactor for {} code with goal: {}", language, refactoring_goal);
        let system_instruction = format!("You are an expert {} programmer and AI assistant specializing in code refactoring. Your task is to analyze the given code, apply the requested refactoring goal, and provide the `refactored_code`, a clear `rationale` for the changes, and an `estimated_improvement` (e.g., '5% performance increase', 'improved readability'). Output your response as a JSON object with these fields.", language);
        let prompt = format!(
            "Refactor the following {} code with the goal: '{}'.\n\nCode:\n```""""{}""""\n```\n\nOutput a JSON object for the refactoring proposal.",
            language, refactoring_goal, current_code
        );

        let llm_response_json = self.llm_processor.generate_content(&prompt, Some(system_instruction)).await?;

        let proposal: CodeRefactorProposal = serde_json::from_str(&llm_response_json)
            .map_err(|e| format!("Failed to parse LLM's code refactor proposal JSON: {}. Response: {}", e, llm_response_json))?;
        
        Ok(proposal)
    }
}
