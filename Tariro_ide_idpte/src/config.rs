// src/config.rs (IDPTE specific)
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IdpteConfig {
    pub core: CoreConfig,
    pub security: SecurityConfig,
    // ... many more fields from the source Turn
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CoreConfig { pub data_dir: PathBuf, pub log_level: String, pub enable_auto_updates: bool }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig { pub enable_ai_threat_modeling: bool, pub supply_chain_scan_frequency_hours: u64, pub constitutional_compiler_strictness: f32 }

impl IdpteConfig {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let config_str = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&config_str)?;
        Ok(config)
    }
}
