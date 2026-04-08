// src/agi_swarm.rs
use sqlx::PgPool;
use anyhow::Result;
use tracing::info;
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct SwarmProposal {
    pub id: Uuid,
    pub proposer_agi_id: Uuid,
    pub proposal_type: String,
    pub description: String,
    pub votes_for: i32,
    pub status: String,
}

pub struct AgiSwarmEngine {
    pool: Option<PgPool>,
}

impl AgiSwarmEngine {
    pub fn new(pool: Option<PgPool>) -> Self { Self { pool } }

    pub async fn submit_swarm_proposal(&self, agi_id: Uuid, p_type: &str, desc: &str) -> Result<SwarmProposal> {
        info!("SwarmIntelligence: AGI {} submitting {} proposal...", agi_id, p_type);
        let expires = chrono::Utc::now() + chrono::Duration::hours(24);
        let id = Uuid::new_v4();
        if let Some(pool) = &self.pool {
            sqlx::query!("INSERT INTO swarm_proposals (id, proposer_agi_id, proposal_type, description, expires_at) VALUES ($1, $2, $3, $4, $5)",
                id, agi_id, p_type, desc, expires).execute(pool).await?;
        }
        Ok(SwarmProposal { id, proposer_agi_id: agi_id, proposal_type: p_type.to_string(), description: desc.to_string(), votes_for: 0, status: "open".to_string() })
    }
}
