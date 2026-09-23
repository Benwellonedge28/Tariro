// Tariro AGI - Working Memory Module
// Active working memory with attention mechanisms

use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Working Memory Types
// ============================================================================

/// Working memory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryItem {
    pub item_id: String,
    pub content: WorkingContent,
    pub representation: RepresentationType,

    // Attention metrics
    pub activation: f32,
    pub attention_weight: f32,
    pub priority: f32,

    // Temporal
    pub added_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub decay_rate: f32,

    // Binding
    pub binding_key: Option<String>,
    pub chunk_id: Option<String>,

    // Source tracking
    pub source_type: SourceType,
    pub source_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkingContent {
    Perceptual(PerceptualContent),
    Conceptual(ConceptualContent),
    Procedural(ProceduralContent),
    Episodic(EpisodicContent),
    Affective(AffectiveContent),
    Visual(String),
    Verbal(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptualContent {
    pub modality: Modality,
    pub raw_data: String,
    pub features: Vec<Feature>,
    pub binding_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualContent {
    pub concept_id: String,
    pub name: String,
    pub properties: HashMap<String, Value>,
    pub relations: Vec<Relation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralContent {
    pub action: String,
    pub parameters: HashMap<String, Value>,
    pub preconditions: Vec<Condition>,
    pub effects: Vec<Effect>,
    pub progress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicContent {
    pub memory_id: String,
    pub summary: String,
    pub key_events: Vec<String>,
    pub emotional_tone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectiveContent {
    pub emotion: String,
    pub intensity: f32,
    pub valence: f32,
    pub arousal: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepresentationType {
    Verbal,
    Visual,
    Spatial,
    Auditory,
    Motor,
    Abstract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modality {
    Visual,
    Auditory,
    Tactile,
    Olfactory,
    Gustatory,
    Proprioceptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub value: Value,
    pub salience: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub relation_type: String,
    pub target: String,
    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<Value>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub attribute: String,
    pub operator: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub attribute: String,
    pub value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    Sensory,
    LongTermMemory,
    Reasoning,
    Planning,
    Communication,
    Introspection,
}

// ============================================================================
// Attention Mechanisms
// ============================================================================

/// Attention mechanism
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionMechanism {
    pub mechanism_type: AttentionType,
    pub query_vector: Vec<f32>,
    pub key_weights: HashMap<String, f32>,
    pub value_weights: HashMap<String, f32>,
    pub output_weights: Vec<f32>,
    pub attention_span: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttentionType {
    Selective,      // Focus on one item
    Sustained,      // Maintain attention
    Divided,        // Multiple attention streams
    Executive,      // Top-down control
    Saliency,       // Bottom-up capture
}

// ============================================================================
// Chunk and Binding
// ============================================================================

/// Memory chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryChunk {
    pub chunk_id: String,
    pub items: Vec<String>,  // Item IDs
    pub binding_key: String,
    pub integrated_content: String,
    pub stability: f32,
    pub formed_at: DateTime<Utc>,
}

// ============================================================================
// Working Memory Store
// ============================================================================

/// Working memory management system
pub struct WorkingMemoryStore {
    // Primary storage
    items: VecDeque<WorkingMemoryItem>,
    item_index: HashMap<String, usize>,

    // Attention
    attention_mechanisms: Vec<AttentionMechanism>,
    current_attention_focus: Option<String>,
    attention_window: Vec<String>,

    // Chunking
    chunks: HashMap<String, MemoryChunk>,

    // Binding
    bindings: HashMap<String, Vec<String>>,
    binding_keys: HashMap<String, String>,

    // Central executive
    central_executive: CentralExecutive,

    // Phonological loop
    phonological_loop: PhonologicalLoop,

    // Visuospatial sketchpad
    visuospatial_sketchpad: VisuospatialStore,

    // Episodic buffer
    episodic_buffer: EpisodicBuffer,

    // Configuration
    config: WorkingMemoryConfig,

    // Statistics
    stats: WorkingMemoryStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralExecutive {
    pub current_goals: Vec<GoalState>,
    pub attention_controller: AttentionState,
    pub switching_cost: Duration,
    pub processing_capacity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalState {
    pub goal_id: String,
    pub description: String,
    pub priority: f32,
    pub progress: f32,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionState {
    pub focused_items: Vec<String>,
    pub distributed_items: Vec<String>,
    pub inhibition_map: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonologicalLoop {
    pub phonological_store: Vec<String>,
    pub articulatory_process: Vec<String>,
    pub rehearsal_rate: f32,
    pub decay_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisuospatialStore {
    pub visual_cache: Vec<VisualObject>,
    pub spatial_map: HashMap<String, Position>,
    pub object_locations: HashMap<String, Position>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualObject {
    pub object_id: String,
    pub features: Vec<VisualFeature>,
    pub position: Position,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualFeature {
    pub feature_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicBuffer {
    pub episodes: Vec<EpisodeSummary>,
    pub integration_mode: IntegrationMode,
    pub capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeSummary {
    pub summary: String,
    pub temporal_marker: DateTime<Utc>,
    pub key_items: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationMode {
    Sequential,
    Thematic,
    Causal,
}

#[derive(Debug, Clone)]
pub struct WorkingMemoryConfig {
    pub capacity: usize,
    pub decay_rate: f32,
    pub rehearsal_benefit: f32,
    pub chunk_size: usize,
    pub attention_switch_cost_ms: u64,
    pub rehearsal_interval_ms: u64,
    pub enable_central_executive: bool,
}

impl Default for WorkingMemoryConfig {
    fn default() -> Self {
        Self {
            capacity: 7,  // Miller's law
            decay_rate: 0.02,
            rehearsal_benefit: 0.1,
            chunk_size: 4,
            attention_switch_cost_ms: 200,
            rehearsal_interval_ms: 2000,
            enable_central_executive: true,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkingMemoryStats {
    pub total_items_added: usize,
    pub items_retrieved: usize,
    pub attention_switches: usize,
    pub chunks_formed: usize,
    pub average_activation: f32,
    pub capacity_used: f32,
}

// ============================================================================
// Priority Types
// ============================================================================

#[derive(Clone)]
struct PrioritizedItem {
    item: WorkingMemoryItem,
    priority: f32,
}

impl PartialOrd for PrioritizedItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for PrioritizedItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Eq for PrioritizedItem {}
impl PartialEq for PrioritizedItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

// ============================================================================
// Implementation
// ============================================================================

impl WorkingMemoryStore {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
            item_index: HashMap::new(),
            attention_mechanisms: vec![
                AttentionMechanism {
                    mechanism_type: AttentionType::Selective,
                    query_vector: vec![],
                    key_weights: HashMap::new(),
                    value_weights: HashMap::new(),
                    output_weights: vec![],
                    attention_span: Duration::from_secs(5),
                },
                AttentionMechanism {
                    mechanism_type: AttentionType::Executive,
                    query_vector: vec![],
                    key_weights: HashMap::new(),
                    value_weights: HashMap::new(),
                    output_weights: vec![],
                    attention_span: Duration::from_secs(10),
                },
            ],
            current_attention_focus: None,
            attention_window: Vec::new(),
            chunks: HashMap::new(),
            bindings: HashMap::new(),
            binding_keys: HashMap::new(),
            central_executive: CentralExecutive {
                current_goals: Vec::new(),
                attention_controller: AttentionState {
                    focused_items: Vec::new(),
                    distributed_items: Vec::new(),
                    inhibition_map: HashMap::new(),
                },
                switching_cost: Duration::from_millis(200),
                processing_capacity: 1.0,
            },
            phonological_loop: PhonologicalLoop {
                phonological_store: Vec::new(),
                articulatory_process: Vec::new(),
                rehearsal_rate: 0.5,
                decay_rate: 0.02,
            },
            visuospatial_sketchpad: VisuospatialStore {
                visual_cache: Vec::new(),
                spatial_map: HashMap::new(),
                object_locations: HashMap::new(),
            },
            episodic_buffer: EpisodicBuffer {
                episodes: Vec::new(),
                integration_mode: IntegrationMode::Causal,
                capacity: 4,
            },
            config: WorkingMemoryConfig::default(),
            stats: WorkingMemoryStats::default(),
        }
    }

    /// Add item to working memory
    pub fn add(&mut self, item: WorkingMemoryItem) -> Result<(), WorkingMemoryError> {
        let item_id = item.item_id.clone();

        // Check capacity
        if self.items.len() >= self.config.capacity {
            self.evict_low_activation()?;
        }

        // Store item
        let index = self.items.len();
        self.items.push_back(item.clone());
        self.item_index.insert(item_id.clone(), index);

        // Update specialized stores
        match &item.content {
            WorkingContent::Verbal(v) => self.handle_verbal(item),
            WorkingContent::Visual(v) => self.handle_visual(item),
            _ => {}
        }

        // Update bindings
        if let Some(binding_key) = &item.binding_key {
            self.bindings.entry(binding_key.clone())
                .or_insert_with(Vec::new)
                .push(item_id.clone());
        }

        // Update attention window
        self.update_attention_window(&item_id);

        // Update central executive
        if self.config.enable_central_executive {
            self.update_executive(&item);
        }

        self.stats.total_items_added += 1;
        self.stats.capacity_used = self.items.len() as f32 / self.config.capacity as f32;

        Ok(())
    }

    /// Handle verbal content
    fn handle_verbal(&mut self, item: WorkingMemoryItem) {
        if let WorkingContent::Verbal(v) = item.content {
            self.phonological_loop.phonological_store.push(v);
        }
    }

    /// Handle visual content
    fn handle_visual(&mut self, item: WorkingMemoryItem) {
        if let WorkingContent::Visual(v) = item.content {
            self.visuospatial_sketchpad.visual_cache.push(VisualObject {
                object_id: item.item_id,
                features: vec![],
                position: Position { x: 0.0, y: 0.0, z: 0.0 },
                active: true,
            });
        }
    }

    /// Update attention window
    fn update_attention_window(&mut self, item_id: &str) {
        self.attention_window.retain(|id| id != item_id);
        self.attention_window.push(item_id.to_string());

        // Maintain window size
        while self.attention_window.len() > self.config.capacity {
            self.attention_window.remove(0);
        }
    }

    /// Update central executive
    fn update_executive(&mut self, item: &WorkingMemoryItem) {
        // Update attention focus if higher priority
        if item.priority > 0.8 {
            self.current_attention_focus = Some(item.item_id.clone());
        }

        // Update goal-related items
        if let Some(goal) = self.central_executive.current_goals.iter_mut().find(|g| g.active) {
            goal.progress += item.activation * 0.01;
        }
    }

    /// Retrieve item by ID
    pub fn get(&mut self, item_id: &str) -> Option<&WorkingMemoryItem> {
        if let Some(&idx) = self.item_index.get(item_id) {
            let item = &mut self.items[idx];
            item.last_accessed = Utc::now();
            item.activation = (item.activation + self.config.rehearsal_benefit).min(1.0);
            self.stats.items_retrieved += 1;
            Some(item)
        } else {
            None
        }
    }

    /// Retrieve items by priority
    pub fn retrieve_by_priority(&self, limit: usize) -> Vec<&WorkingMemoryItem> {
        let mut items: Vec<_> = self.items.iter().collect();

        items.sort_by(|a, b| {
            let a_score = a.priority * a.activation;
            let b_score = b.priority * b.activation;
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        items.into_iter().take(limit).collect()
    }

    /// Retrieve by content type
    pub fn retrieve_by_type(&self, content_type: &str) -> Vec<&WorkingMemoryItem> {
        self.items.iter()
            .filter(|item| {
                match (&item.content, content_type) {
                    (WorkingContent::Perceptual(_), "perceptual") => true,
                    (WorkingContent::Conceptual(_), "conceptual") => true,
                    (WorkingContent::Procedural(_), "procedural") => true,
                    (WorkingContent::Episodic(_), "episodic") => true,
                    (WorkingContent::Affective(_), "affective") => true,
                    _ => false,
                }
            })
            .collect()
    }

    /// Focus attention on item
    pub fn focus_attention(&mut self, item_id: &str) -> Result<(), WorkingMemoryError> {
        if let Some(&idx) = self.item_index.get(item_id) {
            // Update previous focus
            if let Some(prev_focus) = &self.current_attention_focus {
                if prev_focus != item_id {
                    self.stats.attention_switches += 1;
                    // Apply switching cost
                    let item = &mut self.items[idx];
                    item.activation *= 0.9;
                }
            }

            self.current_attention_focus = Some(item_id.to_string());
            self.central_executive.attention_controller.focused_items.push(item_id.to_string());

            Ok(())
        } else {
            Err(WorkingMemoryError::ItemNotFound)
        }
    }

    /// Clear attention focus
    pub fn clear_focus(&mut self) {
        self.current_attention_focus = None;
        self.central_executive.attention_controller.focused_items.clear();
    }

    /// Form chunk from related items
    pub fn form_chunk(&mut self, item_ids: &[String], binding_key: &str) -> Result<String, WorkingMemoryError> {
        if item_ids.len() > self.config.chunk_size {
            return Err(WorkingMemoryError::ChunkTooLarge);
        }

        // Verify all items exist
        for id in item_ids {
            if !self.item_index.contains_key(id) {
                return Err(WorkingMemoryError::ItemNotFound);
            }
        }

        let chunk_id = generate_id();

        // Create chunk
        let chunk = MemoryChunk {
            chunk_id: chunk_id.clone(),
            items: item_ids.to_vec(),
            binding_key: binding_key.to_string(),
            integrated_content: format!("Chunk of {} items", item_ids.len()),
            stability: 0.5,
            formed_at: DateTime<Utc>::now(),
        };

        // Store binding
        self.binding_keys.insert(chunk_id.clone(), binding_key.to_string());

        // Update items
        for id in item_ids {
            if let Some(&idx) = self.item_index.get(id) {
                self.items[idx].chunk_id = Some(chunk_id.clone());
            }
        }

        self.chunks.insert(chunk_id.clone(), chunk);
        self.stats.chunks_formed += 1;

        Ok(chunk_id)
    }

    /// Rehearse item (refresh in memory)
    pub fn rehearse(&mut self, item_id: &str) -> Result<(), WorkingMemoryError> {
        if let Some(&idx) = self.item_index.get(item_id) {
            let item = &mut self.items[idx];
            item.activation = (item.activation + self.config.rehearsal_benefit).min(1.0);
            item.last_accessed = Utc::now();
            Ok(())
        } else {
            Err(WorkingMemoryError::ItemNotFound)
        }
    }

    /// Apply decay to all items
    pub fn apply_decay(&mut self) {
        for item in &mut self.items {
            item.activation *= 1.0 - self.config.decay_rate;
        }

        // Decay phonological store
        self.phonological_loop.phonological_store.retain(|_| {
            rand::random::<f32>() > self.phonological_loop.decay_rate
        });
    }

    /// Remove item
    pub fn remove(&mut self, item_id: &str) -> Result<WorkingMemoryItem, WorkingMemoryError> {
        if let Some(&idx) = self.item_index.get(item_id) {
            let item = self.items.remove(idx).unwrap();

            // Update indices
            self.item_index.remove(item_id);
            for (i, _) in self.items.iter().enumerate() {
                self.item_index.insert(
                    self.items[i].item_id.clone(),
                    i
                );
            }

            // Remove from attention
            self.attention_window.retain(|id| id != item_id);
            if self.current_attention_focus.as_deref() == Some(item_id) {
                self.current_attention_focus = None;
            }

            Ok(item)
        } else {
            Err(WorkingMemoryError::ItemNotFound)
        }
    }

    /// Evict lowest activation item
    fn evict_low_activation(&mut self) -> Result<(), WorkingMemoryError> {
        if self.items.is_empty() {
            return Err(WorkingMemoryError::EmptyMemory);
        }

        let mut lowest_idx = 0;
        let mut lowest_activation = f32::MAX;

        for (idx, item) in self.items.iter().enumerate() {
            let effective_activation = item.activation * item.attention_weight;
            if effective_activation < lowest_activation {
                lowest_activation = effective_activation;
                lowest_idx = idx;
            }
        }

        let item = self.items.remove(lowest_idx);
        self.item_index.remove(&item.item_id);

        // Re-index
        for (i, remaining) in self.items.iter().enumerate() {
            self.item_index.insert(remaining.item_id.clone(), i);
        }

        Ok(())
    }

    /// Get current attention focus
    pub fn get_attention_focus(&self) -> Option<&String> {
        self.current_attention_focus.as_ref()
    }

    /// Get attention window
    pub fn get_attention_window(&self) -> &[String] {
        &self.attention_window
    }

    /// Get statistics
    pub fn get_stats(&self) -> &WorkingMemoryStats {
        &self.stats
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.item_index.clear();
        self.attention_window.clear();
        self.current_attention_focus = None;
        self.chunks.clear();
        self.bindings.clear();
        self.binding_keys.clear();
        self.phonological_loop.phonological_store.clear();
        self.visuospatial_sketchpad.visual_cache.clear();
        self.episodic_buffer.episodes.clear();
    }
}

/// Working memory errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkingMemoryError {
    ItemNotFound,
    CapacityExceeded,
    ChunkTooLarge,
    EmptyMemory,
}

impl std::fmt::Display for WorkingMemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkingMemoryError::ItemNotFound => write!(f, "Item not found in working memory"),
            WorkingMemoryError::CapacityExceeded => write!(f, "Working memory capacity exceeded"),
            WorkingMemoryError::ChunkTooLarge => write!(f, "Chunk exceeds maximum size"),
            WorkingMemoryError::EmptyMemory => write!(f, "Working memory is empty"),
        }
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("wm_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut store = WorkingMemoryStore::new();

        let item = WorkingMemoryItem {
            item_id: generate_id(),
            content: WorkingContent::Conceptual(ConceptualContent {
                concept_id: "test".to_string(),
                name: "Test Concept".to_string(),
                properties: HashMap::new(),
                relations: vec![],
            }),
            representation: RepresentationType::Abstract,
            activation: 0.8,
            attention_weight: 1.0,
            priority: 0.9,
            added_at: DateTime<Utc>::now(),
            last_accessed: DateTime<Utc>::now(),
            decay_rate: 0.01,
            binding_key: None,
            chunk_id: None,
            source_type: SourceType::Reasoning,
            source_id: "test".to_string(),
        };

        assert!(store.add(item).is_ok());
        assert_eq!(store.stats.total_items_added, 1);
    }

    #[test]
    fn test_focus_attention() {
        let mut store = WorkingMemoryStore::new();

        let item = WorkingMemoryItem {
            item_id: generate_id(),
            content: WorkingContent::Conceptual(ConceptualContent {
                concept_id: "focus_test".to_string(),
                name: "Focus Test".to_string(),
                properties: HashMap::new(),
                relations: vec![],
            }),
            representation: RepresentationType::Abstract,
            activation: 0.9,
            attention_weight: 1.0,
            priority: 1.0,
            added_at: DateTime<Utc>::now(),
            last_accessed: DateTime<Utc>::now(),
            decay_rate: 0.01,
            binding_key: None,
            chunk_id: None,
            source_type: SourceType::Reasoning,
            source_id: "test".to_string(),
        };

        let item_id = item.item_id.clone();
        store.add(item).unwrap();

        assert!(store.focus_attention(&item_id).is_ok());
        assert_eq!(store.get_attention_focus(), Some(&item_id));
    }
}
