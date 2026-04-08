// src/data_fabric.rs (Sovereign Data Fabric)
use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct SemanticDataNode { pub id: Uuid, pub did: String, pub data_type: String, pub content: serde_json::Value, pub created_at: chrono::DateTime<chrono::Utc> }

pub struct DataFabricEngine { pool: Option<PgPool> }
impl DataFabricEngine {
    pub fn new(pool: Option<PgPool>) -> Self { Self { pool } }
}
