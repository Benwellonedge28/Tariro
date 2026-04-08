// src/db.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct ProjectTask { pub id: String, pub title: String, pub status: String, pub priority: String, pub progress: f64, pub description: Option<String>, pub due_date: Option<String>, pub linked_file_ids: Option<Vec<String>> }

pub struct DbClient;
impl DbClient {
    pub async fn read_project_task(&self, _query: Option<serde_json::Value>, _limit: Option<i64>, _skip: Option<i64>, _sort: Option<String>, _fields: Option<Vec<String>>) -> Result<Vec<ProjectTask>, String> {
        Ok(vec![])
    }
}
