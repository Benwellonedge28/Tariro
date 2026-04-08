// src/chatbot/learning.rs
pub struct ChatLearning;
impl ChatLearning {
    pub async fn learn_from_chat(&self, _msg: &str) -> anyhow::Result<()> { Ok(()) }
}
