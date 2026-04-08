// src/chatbot/chat_client.rs
pub struct ChatClient;
impl ChatClient {
    pub async fn send_message(&self, _msg: &str) -> anyhow::Result<()> { Ok(()) }
}
