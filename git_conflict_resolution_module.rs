use crate::omega_news_content_generator_llm::ContentGeneratorLlm;
use serde::{Deserialize, Serialize};

pub struct GitConflictResolutionModule {
    llm_analyzer: ContentGeneratorLlm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConflictResolution {
    pub proposed_content: String,
    pub rationale: String,
    pub detected_conflicts: Vec<String>,
}

impl GitConflictResolutionModule {
    pub fn new() -> Result<Self, String> {
        let llm_analyzer = ContentGeneratorLlm::new()?;
        Ok(GitConflictResolutionModule { llm_analyzer })
    }

    pub async fn resolve_conflict(&self, conflicting_file_content: &str, file_path: &str) -> Result<GitConflictResolution, String> {
        println!("AGI is attempting to resolve Git conflict in: {}", file_path);

        let system_instruction = "You are an expert Git conflict resolution AI. Analyze the provided conflicting file content, identify the different versions, intelligently merge them to create a coherent and correct final version, and provide a clear rationale for your choices. Also, list the key conflicting sections you identified.";
        let prompt = format!(
            "A Git conflict has occurred in the file '{}'. Below is the conflicting content. 
            Your task is to resolve this conflict by producing a single, merged version of the file. 
            Prioritize changes that maintain logical correctness and integrate both sides where appropriate. 
            If a clear choice must be made, favor the 'ours' version (the current branch's changes) unless the 'theirs' version (incoming changes) is clearly superior or necessary for compilation/logic. 
            After providing the `proposed_content`, include a brief `rationale` for your merge strategy and list `detected_conflicts` as specific conflicting blocks.\n\nConflicting Content:\n```\n{}
```\n\nOutput your response as a JSON object with fields: 
`proposed_content` (String), `rationale` (String), `detected_conflicts` (Vec<String>).

NOTE: Ensure the `proposed_content` is ready to be directly written to the file. It should NOT contain Git conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`).",
            file_path, conflicting_file_content
        );

        let llm_response_json = self.llm_analyzer.generate_content(&prompt, Some(system_instruction)).await?;

        let resolution: GitConflictResolution = serde_json::from_str(&llm_response_json)
            .map_err(|e| format!("Failed to parse LLM's Git conflict resolution JSON: {}. Response: {}", e, llm_response_json))?;
        
        Ok(resolution)
    }
}
