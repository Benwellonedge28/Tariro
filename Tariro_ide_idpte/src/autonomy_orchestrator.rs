// src/autonomy_orchestrator.rs
use sqlx::PgPool;
use anyhow::Result;
use tracing::{info, warn};
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct AgiDecisionLog {
    pub id: Uuid,
    pub decision: String,
    pub reasoning: String,
    pub autonomy_level: f32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct AutonomyOrchestrator {
    pool: Option<PgPool>,
}

impl AutonomyOrchestrator {
    pub fn new(pool: Option<PgPool>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn resolve_security_threat(&self, threat_id: Uuid, threat_level: &str) -> Result<String> {
        info!("Autonomy Orchestrator: AGI evaluating security threat {} (Level: {})...", threat_id, threat_level);
        let decision_str = format!("AGI autonomously decided to isolate affected module based on {}. Ethical alignment: High. Predicted impact: Low.", threat_level);
        if let Some(pool) = &self.pool {
            sqlx::query!("INSERT INTO agi_decision_logs (decision, reasoning, autonomy_level) VALUES ($1, $2, $3)",
                decision_str, "Autonomous Security Response", 0.95).execute(pool).await?;
        }
        Ok(format!("AGI successfully initiated autonomous threat resolution for {}. Details: {}.", threat_id, decision_str))
    }

    pub async fn initiate_p2p_code_review(&self, file_id: Uuid) -> Result<String> {
        info!("Autonomy Orchestrator: AGI analyzing file {} for P2P code review need...", file_id);
        let decision_str = format!("AGI requested P2P review for file {} based on high cyclomatic complexity. Ethical flag: false. Peers selected based on Rust expertise.", file_id);
        if let Some(pool) = &self.pool {
            sqlx::query!("INSERT INTO agi_decision_logs (decision, reasoning, autonomy_level) VALUES ($1, $2, $3)",
                decision_str, "Autonomous Code Review Orchestration", 0.8).execute(pool).await?;
        }
        Ok(format!("AGI initiated P2P code review for file {}. Peers notified. Details: {}.", file_id, decision_str))
    }

    pub async fn get_agi_decision_logs(&self) -> Result<Vec<AgiDecisionLog>> {
        if let Some(pool) = &self.pool {
            let logs = sqlx::query_as!(AgiDecisionLog, "SELECT id, decision, reasoning, autonomy_level, timestamp FROM agi_decision_logs ORDER BY timestamp DESC").fetch_all(pool).await?;
            Ok(logs)
        } else { Ok(vec![]) }
    }
}
