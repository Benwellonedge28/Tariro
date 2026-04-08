// src/did.rs (True Decentralized Identity (DID) Integration)
use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct DecentralizedIdentity { pub did: String, pub user_id: Uuid, pub public_key_jwk: serde_json::Value }

pub struct DidEngine { pool: Option<PgPool> }
impl DidEngine {
    pub fn new(pool: Option<PgPool>) -> Self { Self { pool } }
}
