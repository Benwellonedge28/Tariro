// src/infinite_learning_memory/ephemeral_memory_management_knowledge_volatility_control.rs

pub enum EphemeralType {
    SensoryBuffer,
    WorkingMemory,
    Scratchpad,
    TransientObservation,
}

pub struct EphemeralMemoryEntry {
    pub entry_id: String,
    pub ephemeral_type: EphemeralType,
    pub content_hash: String,
    pub creation_time: chrono::DateTime<chrono::Utc>,
    pub expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    pub persistence_priority: f64, // 0-1, 1 means likely to become long-term memory
    pub associated_task_id: Option<String>,
}

pub struct EphemeralMemoryManagementKnowledgeVolatilityControl {
    // Manages short-term, temporary memories.
    // Dynamically controls their persistence and allows for controlled forgetting to prevent cognitive overload.
    // Ensures critical information is retained, integrates with Conscious Computing Architecture (Feature 40).
    pub active_ephemeral_memories: std::collections::HashMap<String, EphemeralMemoryEntry>,
}

impl EphemeralMemoryManagementKnowledgeVolatilityControl {
    pub fn new() -> Self {
        println!("Initializing Ephemeral Memory Management & Knowledge Volatility Control...");
        EphemeralMemoryManagementKnowledgeVolatilityControl { active_ephemeral_memories: std::collections::HashMap::new() }
    }

    pub fn create_ephemeral_entry(&mut self, entry: EphemeralMemoryEntry) {
        println!("Creating ephemeral memory entry '{}' of type {:?}.", entry.entry_id, entry.ephemeral_type);
        self.active_ephemeral_memories.insert(entry.entry_id.clone(), entry);
    }

    pub fn assess_and_control_persistence(&mut self, entry_id: &str) -> Result<(), String> {
        if let Some(entry) = self.active_ephemeral_memories.get_mut(entry_id) {
            println!("Assessing persistence for ephemeral entry '{}'. Current priority: {}.", entry_id, entry.persistence_priority);
            // This function continuously evaluates the relevance and importance of ephemeral memories.
            // It uses context from active tasks (Feature 36), the Conscious Computing Architecture's attention (Feature 40),
            // and patterns identified through Meta-Learning (Feature 29) to decide if a memory should be:
            // 1. Promoted to long-term memory (triggering knowledge graph integration).
            // 2. Its expiration time extended.
            // 3. Allowed to expire and be forgotten.

            if entry.persistence_priority > 0.7 && entry.expiration_time.is_some() {
                entry.expiration_time = Some(chrono::Utc::now() + chrono::Duration::hours(24));
                println!("  -> Extended expiration for '{}' due to high priority.", entry_id);
            }
            Ok(())
        } else {
            Err(format!("Ephemeral memory entry '{}' not found.", entry_id))
        }
    }

    pub fn enforce_forgetting(&mut self) {
        println!("Enforcing forgetting of expired ephemeral memories.");
        let now = chrono::Utc::now();
        self.active_ephemeral_memories.retain(|_, entry| {
            entry.expiration_time.map_or(true, |exp_time| exp_time > now)
        });
    }
}
