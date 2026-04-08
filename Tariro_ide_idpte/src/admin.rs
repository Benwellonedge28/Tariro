// src/admin.rs (Comprehensive Admin Dashboard Backend)
use sqlx::PgPool;
use anyhow::Result;
use tracing::info;
use serde::Serialize;
use uuid::Uuid;

// Integrate with UMC and Deployment engines
use crate::compiler::umc_core::TariroUmc;
use crate::deployment::DeploymentEngine;

#[derive(Serialize)]
pub struct PlatformSetting {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct PlatformUser {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub is_banned: bool,
}

pub struct AdminEngine {
    pool: Option<PgPool>,
}

impl AdminEngine {
    pub fn new(pool: Option<PgPool>) -> Self { Self { pool } }

    pub async fn get_platform_settings(&self) -> Result<Vec<PlatformSetting>> {
        info!("Admin: Fetching platform settings...");
        if let Some(pool) = &self.pool {
            let settings = sqlx::query_as!(PlatformSetting, "SELECT key, value, description FROM admin_settings")
                .fetch_all(pool).await?;
            Ok(settings)
        } else { Ok(vec![]) }
    }

    pub async fn update_setting(&self, key: &str, value: &str) -> Result<()> {
        info!("Admin: Updating setting '{}' to '{}'".to_string(), key, value);
        if let Some(pool) = &self.pool {
            sqlx::query!("UPDATE admin_settings SET value = $1 WHERE key = $2", value, key)
                .execute(pool).await?;
        }
        Ok(())
    }

    pub async fn list_platform_users(&self) -> Result<Vec<PlatformUser>> {
        info!("Admin: Listing all platform users...");
        if let Some(pool) = &self.pool {
            let users = sqlx::query_as!(PlatformUser, "SELECT id, email, role, is_banned FROM users")
                .fetch_all(pool).await?;
            Ok(users)
        } else { Ok(vec![]) }
    }

    pub async fn ban_user(&self, user_id: Uuid, ban: bool) -> Result<()> {
        info!("Admin: Setting ban status for user {} to {}", user_id, ban);
        if let Some(pool) = &self.pool {
            sqlx::query!("UPDATE users SET is_banned = $1 WHERE id = $2", ban, user_id)
                .execute(pool).await?;
        }
        Ok(())
    }
}
