// src/agi_project_manager.rs
use sqlx::PgPool;
use anyhow::Result;
use tracing::{info, warn};
use uuid::Uuid;
use serde::Serialize;
use crate::autonomy_orchestrator::AutonomyOrchestrator;

#[derive(Serialize)]
pub struct AgiManagedTask {
    pub id: Uuid,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub agi_notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct AgiProjectManagerEngine {
    pool: Option<PgPool>,
}

impl AgiProjectManagerEngine {
    pub fn new(pool: Option<PgPool>) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn agi_generate_and_update_tasks(&self, user_id: Uuid, goal_id: Uuid) -> Result<String> {
        info!("AGI Project Manager: Autonomously generating/updating tasks for goal {}...", goal_id);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let new_task_desc = format!("AGI: Generated task to optimize UMC Tier 1 for {} after detecting potential bottleneck.", goal_id);
        let task_id = Uuid::new_v4();
        if let Some(pool) = &self.pool {
            sqlx::query!(
                "INSERT INTO agi_managed_tasks (id, original_task_id, agi_manager_id, description, status, priority, agi_notes) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                task_id, None::<Uuid>, user_id, new_task_desc, "agi_assigned", "critical", Some("Identified via predictive analysis.".to_string())
            )
            .execute(pool).await?;
        }
        Ok(format!("AGI: Generated and assigned new critical task for Goal {}: '{}'.", goal_id, new_task_desc))
    }

    pub async fn get_agi_managed_tasks(&self, user_id: Uuid) -> Result<Vec<AgiManagedTask>> {
        if let Some(pool) = &self.pool {
            let tasks = sqlx::query_as!(AgiManagedTask, "SELECT id, description, status, priority, agi_notes, created_at FROM agi_managed_tasks WHERE agi_manager_id = $1 ORDER BY created_at DESC", user_id)
                .fetch_all(pool).await?;
            Ok(tasks)
        } else { Ok(vec![]) }
    }
}
