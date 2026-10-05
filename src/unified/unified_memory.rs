//! # Unified Memory
//!
//! A unified memory system that integrates episodic, semantic, and procedural memory
//! for the TUMC-AGI system.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;

/// Memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub key: String,
    pub value: MemoryValue,
    pub entry_type: MemoryType,
    pub timestamp: u64,
    pub importance: f64,
    pub access_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryValue {
    String(String),
    Int(i64),
    Float(f64),
    Code(String),
    Insight(String),
    Pattern(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryType {
    Episodic,
    Semantic,
    Procedural,
    Working,
}

/// Unified memory system
pub struct UnifiedMemory {
    /// Maximum memory size in entries
    max_size: usize,

    /// Episodic memory (experiences)
    episodic: VecDeque<MemoryEntry>,

    /// Semantic memory (facts/concepts)
    semantic: HashMap<String, MemoryEntry>,

    /// Procedural memory (skills/procedures)
    procedural: HashMap<String, MemoryEntry>,

    /// Working memory (active processing)
    working: HashMap<String, MemoryEntry>,

    /// Index for fast lookup
    index: HashMap<String, Vec<String>>,
}

impl UnifiedMemory {
    /// Create a new unified memory
    pub fn new(max_size_mb: usize) -> Self {
        // Roughly 1KB per entry estimate
        let max_entries = max_size_mb * 1024;

        Self {
            max_size: max_entries,
            episodic: VecDeque::new(),
            semantic: HashMap::new(),
            procedural: HashMap::new(),
            working: HashMap::new(),
            index: HashMap::new(),
        }
    }

    /// Initialize the memory system
    pub fn initialize(&mut self) {
        println!("Initializing Unified Memory...");
        println!("  - Max entries: {}", self.max_size);
        println!("  - Episodic memory: enabled");
        println!("  - Semantic memory: enabled");
        println!("  - Procedural memory: enabled");
    }

    /// Store an intent and its result
    pub fn store_intent(&mut self, intent: &str, result: &str, insights: &[crate::tumc_compiler::agi_integration::AgiInsight]) {
        // Store in episodic memory
        let entry = MemoryEntry {
            key: format!("intent_{}", current_timestamp()),
            value: MemoryValue::Code(format!("{} -> {}", intent, result)),
            entry_type: MemoryType::Episodic,
            timestamp: current_timestamp(),
            importance: 0.8,
            access_count: 0,
        };

        self.add_episodic(entry);

        // Store insights in semantic memory
        for (i, insight) in insights.iter().enumerate() {
            let key = format!("insight_{}", i);
            self.semantic.insert(
                key.clone(),
                MemoryEntry {
                    key,
                    value: MemoryValue::Insight(insight.description.clone()),
                    entry_type: MemoryType::Semantic,
                    timestamp: current_timestamp(),
                    importance: insight.confidence,
                    access_count: 0,
                },
            );
        }

        self.reindex();
    }

    /// Add to episodic memory
    fn add_episodic(&mut self, entry: MemoryEntry) {
        if self.episodic.len() >= self.max_size {
            self.episodic.pop_front();
        }
        self.episodic.push_back(entry);
    }

    /// Store a fact in semantic memory
    pub fn store_fact(&mut self, key: &str, value: &str) {
        self.semantic.insert(
            key.to_string(),
            MemoryEntry {
                key: key.to_string(),
                value: MemoryValue::String(value.to_string()),
                entry_type: MemoryType::Semantic,
                timestamp: current_timestamp(),
                importance: 0.7,
                access_count: 0,
            },
        );
        self.reindex();
    }

    /// Store a procedure in procedural memory
    pub fn store_procedure(&mut self, name: &str, code: &str) {
        self.procedural.insert(
            name.to_string(),
            MemoryEntry {
                key: name.to_string(),
                value: MemoryValue::Code(code.to_string()),
                entry_type: MemoryType::Procedural,
                timestamp: current_timestamp(),
                importance: 0.9,
                access_count: 0,
            },
        );
        self.reindex();
    }

    /// Retrieve from memory
    pub fn retrieve(&mut self, key: &str) -> Option<&MemoryEntry> {
        // Check working memory first
        if let Some(entry) = self.working.get(key) {
            return Some(entry);
        }

        // Check semantic memory
        if let Some(entry) = self.semantic.get(key) {
            return Some(entry);
        }

        // Check procedural memory
        if let Some(entry) = self.procedural.get(key) {
            return Some(entry);
        }

        // Search episodic memory
        for entry in &self.episodic {
            if entry.key == key {
                return Some(entry);
            }
        }

        None
    }

    /// Search memory by pattern
    pub fn search(&self, pattern: &str) -> Vec<&MemoryEntry> {
        let mut results = Vec::new();
        let pattern_lower = pattern.to_lowercase();

        // Search semantic
        for entry in self.semantic.values() {
            if match &entry.value {
                MemoryValue::String(s) => s.to_lowercase().contains(&pattern_lower),
                MemoryValue::Insight(s) => s.to_lowercase().contains(&pattern_lower),
                _ => false,
            } {
                results.push(entry);
            }
        }

        // Search procedural
        for entry in self.procedural.values() {
            if match &entry.value {
                MemoryValue::Code(s) => s.to_lowercase().contains(&pattern_lower),
                _ => false,
            } {
                results.push(entry);
            }
        }

        // Search episodic
        for entry in &self.episodic {
            if match &entry.value {
                MemoryValue::Code(s) => s.to_lowercase().contains(&pattern_lower),
                _ => false,
            } {
                results.push(entry);
            }
        }

        results
    }

    /// Get recent memories
    pub fn get_recent(&self, count: usize) -> Vec<&MemoryEntry> {
        self.episodic
            .iter()
            .rev()
            .take(count)
            .collect()
    }

    /// Get memories by type
    pub fn get_by_type(&self, memory_type: MemoryType) -> Vec<&MemoryEntry> {
        match memory_type {
            MemoryType::Episodic => self.episodic.iter().collect(),
            MemoryType::Semantic => self.semantic.values().collect(),
            MemoryType::Procedural => self.procedural.values().collect(),
            MemoryType::Working => self.working.values().collect(),
        }
    }

    /// Reindex memory for fast search
    fn reindex(&mut self) {
        self.index.clear();

        // Index semantic memory
        for entry in self.semantic.values() {
            let words = extract_words(&entry.value);
            for word in words {
                self.index.entry(word).or_default().push(entry.key.clone());
            }
        }

        // Index procedural memory
        for entry in self.procedural.values() {
            let words = extract_words(&entry.value);
            for word in words {
                self.index.entry(word).or_default().push(entry.key.clone());
            }
        }
    }

    /// Finalize the memory system
    pub fn finalize(&mut self) {
        let episodic_count = self.episodic.len();
        let semantic_count = self.semantic.len();
        let procedural_count = self.procedural.len();

        println!("Memory Finalization:");
        println!("  - Episodic entries: {}", episodic_count);
        println!("  - Semantic entries: {}", semantic_count);
        println!("  - Procedural entries: {}", procedural_count);

        self.episodic.clear();
        self.semantic.clear();
        self.procedural.clear();
        self.working.clear();
        self.index.clear();
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            episodic_count: self.episodic.len(),
            semantic_count: self.semantic.len(),
            procedural_count: self.procedural.len(),
            working_count: self.working.len(),
            index_size: self.index.len(),
            total_entries: self.episodic.len() + self.semantic.len() + self.procedural.len(),
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub episodic_count: usize,
    pub semantic_count: usize,
    pub procedural_count: usize,
    pub working_count: usize,
    pub index_size: usize,
    pub total_entries: usize,
}

/// Extract words from a value for indexing
fn extract_words(value: &MemoryValue) -> Vec<String> {
    let text = match value {
        MemoryValue::String(s) => s.clone(),
        MemoryValue::Code(s) => s.clone(),
        MemoryValue::Insight(s) => s.clone(),
        MemoryValue::Pattern(s) => s.clone(),
        _ => return Vec::new(),
    };

    text.split_whitespace()
        .map(|s| s.to_lowercase())
        .filter(|s| s.len() > 2)
        .collect()
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl Default for UnifiedMemory {
    fn default() -> Self {
        Self::new(1024)
    }
}
