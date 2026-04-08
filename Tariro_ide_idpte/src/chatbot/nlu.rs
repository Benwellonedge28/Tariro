// src/chatbot/nlu.rs
pub struct NluEngine;
impl NluEngine {
    pub async fn parse_intent(&self, _msg: &str) -> anyhow::Result<String> { Ok("unknown_intent".to_string()) }
}
