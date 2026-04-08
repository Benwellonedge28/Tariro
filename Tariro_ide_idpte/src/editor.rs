// src/editor.rs (Tariro Editor Core Logic)
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub struct TariroEditorCore { active_locks: Mutex<HashMap<String, Uuid>> }
impl TariroEditorCore {
    pub fn new() -> Self { Self { active_locks: Mutex::new(HashMap::new()) } }
}
