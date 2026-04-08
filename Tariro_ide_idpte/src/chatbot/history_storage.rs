// src/chatbot/history_storage.rs
pub struct HistoryStorage;
impl HistoryStorage {
    pub async fn save_message(&self, _msg: &str) -> anyhow::Result<()> { Ok(()) }
}
