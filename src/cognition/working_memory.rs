//! Tariro Working Memory
//!
//! The mind's limited-capacity workspace for active reasoning.
//! Based on Baddeley's model of working memory.

use crate::prelude::*;
use std::collections::{HashMap, VecDeque};

// ============================================================================
// Mental Items
// ============================================================================

/// An item in working memory
#[derive(Debug, Clone)]
pub struct MentalItem {
    pub id: ItemId,
    pub content: ItemContent,
    pub activation: f64,
    pub timestamp: DateTime<Utc>,
    pub associations: Vec<Association>,
}

impl MentalItem {
    pub fn new(content: ItemContent) -> Self {
        MentalItem {
            id: ItemId::new(),
            content,
            activation: 1.0,
            timestamp: Utc::now(),
            associations: Vec::new(),
        }
    }

    /// Decay activation over time
    pub fn decay(&mut self, rate: f64) {
        self.activation = (self.activation - rate).max(0.0);
    }

    /// Boost activation
    pub fn boost(&mut self, amount: f64) {
        self.activation = (self.activation + amount).min(1.0);
    }
}

/// Unique item identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(pub String);

impl ItemId {
    pub fn new() -> Self {
        ItemId(format!("item_{}", uuid::Uuid::new_v4()))
    }
}

/// Content of a mental item
#[derive(Debug, Clone)]
pub enum ItemContent {
    /// A concept or idea
    Concept(Concept),
    /// A rule or pattern
    Rule(Rule),
    /// A goal or objective
    Goal(Goal),
    /// A sensory perception
    Perception(Perception),
    /// An action to take
    Action(Action),
    /// A retrieved memory
    Memory(Memory),
}

impl ItemContent {
    pub fn as_str(&self) -> String {
        match self {
            ItemContent::Concept(c) => c.name.clone(),
            ItemContent::Rule(r) => r.description.clone(),
            ItemContent::Goal(g) => g.description.clone(),
            ItemContent::Perception(p) => p.description.clone(),
            ItemContent::Action(a) => a.description.clone(),
            ItemContent::Memory(m) => m.description.clone(),
        }
    }
}

/// Association between items
#[derive(Debug, Clone)]
pub struct Association {
    pub target_id: ItemId,
    pub association_type: AssociationType,
    pub strength: f64,
}

/// Type of association
#[derive(Debug, Clone, Copy)]
pub enum AssociationType {
    /// Causal relationship
    Causal,
    /// Similarity
    Similar,
    /// Part-whole
    PartWhole,
    /// Temporal
    Temporal,
    /// Spatial,
    Spatial,
    /// Logical
    Logical,
}

// ============================================================================
// Content Types
// ============================================================================

/// A concept
#[derive(Debug, Clone)]
pub struct Concept {
    pub name: String,
    pub properties: HashMap<String, PropertyValue>,
    pub category: Option<String>,
}

impl Concept {
    pub fn new(name: impl Into<String>) -> Self {
        Concept {
            name: name.into(),
            properties: HashMap::new(),
            category: None,
        }
    }
}

/// Property value
#[derive(Debug, Clone)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
}

/// A rule
#[derive(Debug, Clone)]
pub struct Rule {
    pub description: String,
    pub antecedents: Vec<String>,
    pub consequent: String,
    pub confidence: f64,
}

/// A goal
#[derive(Debug, Clone)]
pub struct Goal {
    pub id: GoalId,
    pub description: String,
    pub priority: f64,
    pub status: GoalStatus,
    pub subgoals: Vec<GoalId>,
}

impl Goal {
    pub fn new(description: impl Into<String>) -> Self {
        Goal {
            id: GoalId::new(),
            description: description.into(),
            priority: 0.5,
            status: GoalStatus::Active,
            subgoals: Vec::new(),
        }
    }
}

/// Unique goal identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GoalId(pub String);

impl GoalId {
    pub fn new() -> Self {
        GoalId(format!("goal_{}", uuid::Uuid::new_v4()))
    }
}

/// Goal status
#[derive(Debug, Clone, Copy)]
pub enum GoalStatus {
    Active,
    Completed,
    Abandoned,
    Blocked,
}

/// A perception
#[derive(Debug, Clone)]
pub struct Perception {
    pub modality: Modality,
    pub description: String,
    pub features: Vec<String>,
}

/// Sensory modality
#[derive(Debug, Clone, Copy)]
pub enum Modality {
    Visual,
    Auditory,
    Textual,
    Internal,
}

/// An action
#[derive(Debug, Clone)]
pub struct Action {
    pub description: String,
    pub expected_outcome: String,
    pub prerequisites: Vec<String>,
    pub cost: f64,
}

/// A memory
#[derive(Debug, Clone)]
pub struct Memory {
    pub description: String,
    pub emotional_valence: f64,
    pub importance: f64,
    pub context: String,
}

// ============================================================================
// Context
// ============================================================================

/// Task context
#[derive(Debug, Clone)]
pub struct Context {
    pub task: Option<String>,
    pub domain: Option<String>,
    pub constraints: Vec<String>,
    pub resources: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            task: None,
            domain: None,
            constraints: Vec::new(),
            resources: HashMap::new(),
        }
    }

    pub fn with_task(mut self, task: impl Into<String>) -> Self {
        self.task = Some(task.into());
        self
    }

    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Chunk
// ============================================================================

/// A chunk - bound mental items
#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: ChunkId,
    pub items: Vec<ItemId>,
    pub binding_type: BindingType,
    pub strength: f64,
}

impl Chunk {
    pub fn new(items: impl IntoIterator<Item = ItemId>) -> Self {
        Chunk {
            id: ChunkId::new(),
            items: items.into_iter().collect(),
            binding_type: BindingType::Associative,
            strength: 1.0,
        }
    }

    pub fn with_binding(mut self, binding_type: BindingType) -> Self {
        self.binding_type = binding_type;
        self
    }
}

/// Unique chunk identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChunkId(pub String);

impl ChunkId {
    pub fn new() -> Self {
        ChunkId(format!("chunk_{}", uuid::Uuid::new_v4()))
    }
}

/// How items are bound together
#[derive(Debug, Clone, Copy)]
pub enum BindingType {
    /// Associated together
    Associative,
    /// Part of same object
    Object,
    /// Sequential in time
    Temporal,
    /// Spatial arrangement
    Spatial,
}

// ============================================================================
// Working Memory
// ============================================================================

/// Working memory configuration
#[derive(Debug, Clone)]
pub struct WorkingMemoryConfig {
    /// Maximum number of items
    pub capacity: usize,

    /// Decay rate per second
    pub decay_rate: f64,

    /// Threshold for eviction
    pub eviction_threshold: f64,

    /// Boost amount for recent access
    pub boost_amount: f64,
}

impl Default for WorkingMemoryConfig {
    fn default() -> Self {
        WorkingMemoryConfig {
            capacity: 7, // Miller's magic number
            decay_rate: 0.01,
            eviction_threshold: 0.1,
            boost_amount: 0.2,
        }
    }
}

/// Working memory - limited capacity workspace
pub struct WorkingMemory {
    /// Active items
    items: HashMap<ItemId, MentalItem>,

    /// Item order (most recent last)
    item_order: VecDeque<ItemId>,

    /// Current context
    context: Context,

    /// Bound chunks
    chunks: Vec<Chunk>,

    /// Configuration
    config: WorkingMemoryConfig,

    /// Statistics
    stats: WorkingMemoryStats,
}

impl WorkingMemory {
    /// Create new working memory
    pub fn new() -> Self {
        WorkingMemory {
            items: HashMap::new(),
            item_order: VecDeque::new(),
            context: Context::new(),
            chunks: Vec::new(),
            config: WorkingMemoryConfig::default(),
            stats: WorkingMemoryStats::default(),
        }
    }

    /// Create with configuration
    pub fn with_config(config: WorkingMemoryConfig) -> Self {
        WorkingMemory {
            items: HashMap::new(),
            item_order: VecDeque::new(),
            context: Context::new(),
            chunks: Vec::new(),
            config,
            stats: WorkingMemoryStats::default(),
        }
    }

    /// Add item to working memory
    pub fn store(&mut self, item: MentalItem) {
        self.stats.total_stores += 1;

        // Evict if at capacity
        if self.items.len() >= self.config.capacity {
            self.evict_least_salient();
        }

        // Add item
        let id = item.id.clone();
        self.items.insert(id.clone(), item);
        self.item_order.push_back(id);
    }

    /// Retrieve an item
    pub fn retrieve(&mut self, id: &ItemId) -> Option<MentalItem> {
        self.stats.total_retrievals += 1;

        if let Some(item) = self.items.get_mut(id) {
            // Boost activation
            item.boost(self.config.boost_amount);

            // Move to end of order
            self.item_order.retain(|i| i != id);
            self.item_order.push_back(id.clone());

            return Some(item.clone());
        }

        None
    }

    /// Find item by content
    pub fn find(&self, content: &str) -> Option<&MentalItem> {
        self.items.values()
            .find(|item| item.content.as_str().contains(content))
    }

    /// Attend to an item (increase activation)
    pub fn attend(&mut self, id: &ItemId) {
        if let Some(item) = self.items.get_mut(id) {
            item.boost(self.config.boost_amount * 2.0);
        }
    }

    /// Bind items together into a chunk
    pub fn bind(&mut self, ids: Vec<ItemId>) -> Chunk {
        let chunk = Chunk::new(ids.clone());
        self.chunks.push(chunk.clone());
        self.stats.total_chunks += 1;

        // Boost associated items
        for id in &ids {
            if let Some(item) = self.items.get_mut(id) {
                item.boost(0.1);
            }
        }

        chunk
    }

    /// Decay all items
    pub fn decay(&mut self) {
        for item in self.items.values_mut() {
            item.decay(self.config.decay_rate);
        }
    }

    /// Evict least salient item
    fn evict_least_salient(&mut self) {
        // Find item with lowest activation
        let to_evict = self.items.iter()
            .min_by(|a, b| a.1.activation.partial_cmp(&b.1.activation).unwrap())
            .map(|(id, _)| id.clone());

        if let Some(id) = to_evict {
            self.stats.evictions += 1;
            self.items.remove(&id);
            self.item_order.retain(|i| i != &id);
        }
    }

    /// Clear working memory
    pub fn clear(&mut self) {
        self.items.clear();
        self.item_order.clear();
        self.chunks.clear();
    }

    /// Set context
    pub fn set_context(&mut self, context: Context) {
        self.context = context;
    }

    /// Get all items sorted by activation
    pub fn get_active_items(&self) -> Vec<&MentalItem> {
        let mut items: Vec<_> = self.items.values().collect();
        items.sort_by(|a, b| b.activation.partial_cmp(&a.activation).unwrap());
        items
    }

    /// Get item count
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get capacity usage
    pub fn capacity_usage(&self) -> f64 {
        self.items.len() as f64 / self.config.capacity as f64
    }

    /// Get statistics
    pub fn stats(&self) -> &WorkingMemoryStats {
        &self.stats
    }
}

impl Default for WorkingMemory {
    fn default() -> Self {
        Self::new()
    }
}

/// Working memory statistics
#[derive(Debug, Clone, Default)]
pub struct WorkingMemoryStats {
    pub total_stores: usize,
    pub total_retrievals: usize,
    pub evictions: usize,
    pub total_chunks: usize,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_memory_store() {
        let mut wm = WorkingMemory::new();
        let item = MentalItem::new(ItemContent::Concept(Concept::new("Test")));

        wm.store(item);

        assert_eq!(wm.len(), 1);
    }

    #[test]
    fn test_working_memory_retrieve() {
        let mut wm = WorkingMemory::new();
        let item = MentalItem::new(ItemContent::Concept(Concept::new("Test")));

        let id = item.id.clone();
        wm.store(item);

        let retrieved = wm.retrieve(&id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_working_memory_eviction() {
        let mut wm = WorkingMemory::with_config(WorkingMemoryConfig {
            capacity: 2,
            ..Default::default()
        });

        for i in 0..5 {
            let item = MentalItem::new(ItemContent::Concept(Concept::new(format!("Item {}", i))));
            wm.store(item);
        }

        assert_eq!(wm.len(), 2);
    }

    #[test]
    fn test_binding() {
        let mut wm = WorkingMemory::new();

        let item1 = MentalItem::new(ItemContent::Concept(Concept::new("A")));
        let item2 = MentalItem::new(ItemContent::Concept(Concept::new("B")));

        let id1 = item1.id.clone();
        let id2 = item2.id.clone();

        wm.store(item1);
        wm.store(item2);

        let chunk = wm.bind(vec![id1, id2]);
        assert_eq!(chunk.items.len(), 2);
    }

    #[test]
    fn test_activation_decay() {
        let mut item = MentalItem::new(ItemContent::Concept(Concept::new("Test")));
        assert_eq!(item.activation, 1.0);

        item.decay(0.3);
        assert_eq!(item.activation, 0.7);
    }
}
