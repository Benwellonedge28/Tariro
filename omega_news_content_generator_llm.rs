use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmRequest {
    pub model: String,
    pub messages: Vec<LlmMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LlmResponse {
    pub choices: Vec<LlmChoice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmChoice {
    pub message: LlmMessage,
}

pub struct ContentGeneratorLlm {
    client: Client,
    api_key: String,
    model: String,
}

impl ContentGeneratorLlm {
    pub fn new() -> Result<Self, String> {
        let api_key = env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY not set.".to_string())?;
        Ok(ContentGeneratorLlm {
            client: Client::new(),
            api_key,
            model: "gpt-4-turbo-preview".to_string(), // Or another suitable model
        })
    }

    pub async fn generate_content(&self, prompt: &str, system_instruction: Option<&str>) -> Result<String, String> {
        let mut messages = vec![
            LlmMessage { role: "user".to_string(), content: prompt.to_string() }
        ];

        if let Some(instruction) = system_instruction {
            messages.insert(0, LlmMessage { role: "system".to_string(), content: instruction.to_string() });
        }

        let request_body = LlmRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 1000,
        };

        let res = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send().await.map_err(|e| format!("LLM API request failed: {}", e))?;

        let response_text = res.text().await.map_err(|e| format!("Failed to read LLM API response: {}", e))?;
        let response: LlmResponse = serde_json::from_str(&response_text).map_err(|e| format!("Failed to parse LLM API JSON: {}. Response: {}", e, response_text))?;

        response.choices.into_iter().next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| "No content in LLM response.".to_string())
    }

    pub async fn summarize_text(&self, text: &str) -> Result<String, String> {
        let prompt = format!("Summarize the following text concisely and accurately:\n\n{}", text);
        self.generate_content(&prompt, Some("You are a helpful assistant that summarizes text.")).await
    }

    pub async fn verify_fact(&self, fact: &str, context: &str) -> Result<String, String> {
        let prompt = format!("Evaluate the following fact: '{}' based on the provided context: '{}'. Is the fact supported, contradicted, or not mentioned? Provide a brief explanation.\n\nFact: {}\nContext: {}\nEvaluation:", fact, context, fact, context);
        self.generate_content(&prompt, Some("You are a helpful assistant that evaluates facts against context.")).await
    }
}
