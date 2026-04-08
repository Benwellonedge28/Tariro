// src/chatbot/basic_chatbot.rs
use tracing::info;
use anyhow::Result;
use crate::chatbot::nlu::NluEngine;

pub struct BasicChatbot {
    nlu: NluEngine,
}

impl BasicChatbot {
    pub fn new() -> Self {
        Self { nlu: NluEngine }
    }

    pub async fn handle_message(&self, message: &str) -> Result<String> {
        info!("BasicChatbot: Handling message: {}", message);
        let intent = self.nlu.parse_intent(message).await?;
        Ok(format!("Detected intent: {:?}", intent))
    }
}
