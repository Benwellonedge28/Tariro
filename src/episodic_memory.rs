// Tariro AGI - Episodic Memory Module
// Long-term memory for experiences and events

use std::collections::{HashMap, VecDeque, BinaryHeap, HashSet};
use std::time::{Duration, Instant};
use std::cmp::Ordering;

// ============================================================================
// Episodic Memory Types
// ============================================================================

/// An episodic memory representing a specific experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub memory_id: String,
    pub event_id: String,
    pub timestamp: Instant,
    pub duration: Option<Duration>,

    // Content
    pub narrative: String,
    pub events: Vec<EpisodeEvent>,
    pub entities: Vec<MemoryEntity>,
    pub locations: Vec<MemoryLocation>,

    // Context
    pub emotional_context: EmotionalContext,
    pub sensory_context: SensoryContext,
    pub spatial_context: SpatialContext,

    // Metadata
    pub importance: f32,
    pub vividness: f32,
    pub accessibility: f32,
    pub valence: f32,  // Positive/negative
    pub arousal: f32,  // Emotional intensity

    // Associations
    pub associated_memories: Vec<String>,
    pub causal_links: Vec<CausalLink>,

    // Retrieval metadata
    pub access_count: usize,
    pub last_accessed: Option<Instant>,
    pub decay_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeEvent {
    pub event_id: String,
    pub start_time: Duration,
    pub end_time: Duration,
    pub event_type: EventType,
    pub description: String,
    pub participants: Vec<String>,
    pub location: Option<String>,
    pub emotional_impact: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    Action,
    Perception,
    Thought,
    Emotion,
    Communication,
    Planning,
    Decision,
    Outcome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntity {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub name: String,
    pub role: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Self_,
    Person,
    Object,
    Place,
    Abstract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLocation {
    pub location_id: String,
    pub name: String,
    pub spatial_relation: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalContext {
    pub primary_emotion: String,
    pub emotion_intensity: f32,
    pub emotional_states: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryContext {
    pub visual: Option<String>,
    pub auditory: Option<String>,
    pub proprioceptive: Option<String>,
    pub modalities: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialContext {
    pub current_location: String,
    pub spatial_relations: Vec<SpatialRelation>,
    pub navigation_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialRelation {
    pub entity: String,
    pub relation: String,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub cause_memory_id: String,
    pub effect_memory_id: String,
    pub causal_mechanism: String,
    pub strength: f32,
}

// ============================================================================
// Episodic Memory Store
// ============================================================================

/// Long-term episodic memory storage and retrieval
pub struct EpisodicMemoryStore {
    // Primary storage
    memories: VecDeque<EpisodicMemory>,
    memory_index: HashMap<String, usize>,

    // Organization structures
    timelines: HashMap<String, Timeline>,
    theme_clusters: HashMap<String, Vec<String>>,
    emotional_clusters: HashMap<String, Vec<String>>,

    // Temporal organization
    recent_memories: VecDeque<String>,
    consolidated_memories: HashSet<String>,

    // Spacing effect tracking
    repetition_schedule: HashMap<String, RepetitionEntry>,

    // Configuration
    config: EpisodicConfig,

    // Statistics
    stats: MemoryStats,
}

#[derive(Debug, Clone)]
pub struct Timeline {
    pub timeline_id: String,
    pub name: String,
    pub memory_ids: Vec<String>,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
}

#[derive(Debug, Clone)]
pub struct RepetitionEntry {
    pub memory_id: String,
    pub next_review: Instant,
    pub interval: Duration,
    pub ease_factor: f32,
    pub repetitions: usize,
}

#[derive(Debug, Clone)]
pub struct EpisodicConfig {
    pub max_memories: usize,
    pub consolidation_threshold: usize,
    pub decay_rate: f32,
    pub importance_boost_on_access: f32,
    pub enable_spacing_effect: bool,
    pub emotional_clustering: bool,
    pub max_clusters: usize,
}

impl Default for EpisodicConfig {
    fn default() -> Self {
        Self {
            max_memories: 100000,
            consolidation_threshold: 10,
            decay_rate: 0.01,
            importance_boost_on_access: 0.1,
            enable_spacing_effect: true,
            emotional_clustering: true,
            max_clusters: 1000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memories: usize,
    pub total_events: usize,
    pub total_entities: usize,
    pub average_importance: f32,
    pub memories_by_emotion: HashMap<String, usize>,
    pub consolidation_count: usize,
}

// ============================================================================
// Priority Queue for Memory Retrieval
// ============================================================================

/// Memory with priority for retrieval
#[derive(Clone)]
struct PrioritizedMemory {
    memory: EpisodicMemory,
    priority: f32,
}

impl Ord for PrioritizedMemory {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.partial_cmp(&other.priority).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PrioritizedMemory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PrioritizedMemory {}
impl PartialEq for PrioritizedMemory {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

// ============================================================================
// Implementation
// ============================================================================

impl EpisodicMemoryStore {
    pub fn new() -> Self {
        Self {
            memories: VecDeque::new(),
            memory_index: HashMap::new(),
            timelines: HashMap::new(),
            theme_clusters: HashMap::new(),
            emotional_clusters: HashMap::new(),
            recent_memories: VecDeque::new(),
            consolidated_memories: HashSet::new(),
            repetition_schedule: HashMap::new(),
            config: EpisodicConfig::default(),
            stats: MemoryStats {
                total_memories: 0,
                total_events: 0,
                total_entities: 0,
                average_importance: 0.5,
                memories_by_emotion: HashMap::new(),
                consolidation_count: 0,
            },
        }
    }

    /// Store a new episodic memory
    pub fn store(&mut self, memory: EpisodicMemory) {
        let memory_id = memory.memory_id.clone();

        // Remove oldest if at capacity
        if self.memories.len() >= self.config.max_memories {
            self.evict_oldest();
        }

        // Store memory
        let index = self.memories.len();
        self.memories.push_back(memory.clone());
        self.memory_index.insert(memory_id.clone(), index);

        // Update recent memories
        self.recent_memories.push_back(memory_id.clone());
        if self.recent_memories.len() > 100 {
            self.recent_memories.pop_front();
        }

        // Update theme clusters
        self.update_theme_clusters(&memory);

        // Update emotional clusters
        if self.config.emotional_clustering {
            self.update_emotional_clusters(&memory);
        }

        // Schedule for spacing effect review
        if self.config.enable_spacing_effect {
            self.schedule_review(&memory);
        }

        // Update statistics
        self.update_stats(&memory);
    }

    /// Retrieve memories based on query
    pub fn retrieve(&mut self, query: &MemoryQuery) -> Vec<EpisodicMemory> {
        let mut candidates: BinaryHeap<PrioritizedMemory> = BinaryHeap::new();

        // Score each memory
        for memory in &self.memories {
            let score = self.score_memory(memory, query);
            if score > 0.0 {
                candidates.push(PrioritizedMemory {
                    memory: memory.clone(),
                    priority: score,
                });
            }
        }

        // Extract top results
        let mut results = Vec::new();
        let limit = query.limit.unwrap_or(10);

        while let Some(pm) = candidates.pop() {
            if results.len() >= limit {
                break;
            }

            // Boost accessibility and record access
            let mut memory = pm.memory;
            memory.access_count += 1;
            memory.last_accessed = Some(Instant::now());
            memory.accessibility = (memory.accessibility + self.config.importance_boost_on_access).min(1.0);

            results.push(memory);
        }

        results
    }

    /// Score a memory for retrieval relevance
    fn score_memory(&self, memory: &EpisodicMemory, query: &MemoryQuery) -> f32 {
        let mut score = 0.0;

        // Importance weight
        score += memory.importance * 0.3;

        // Recency weight
        let age_hours = memory.timestamp.elapsed().as_secs_f32() / 3600.0;
        let recency = (1.0 / (1.0 + age_hours / 24.0)).min(1.0);
        score += recency * 0.2;

        // Accessibility weight
        score += memory.accessibility * 0.1;

        // Vividness weight
        score += memory.vividness * 0.1;

        // Emotional relevance
        if let Some(ref emotion) = query.emotional_filter {
            if memory.emotional_context.primary_emotion == *emotion {
                score += 0.2;
            }
        }

        // Content matching
        if let Some(ref keywords) = query.keywords {
            for keyword in keywords {
                if memory.narrative.to_lowercase().contains(&keyword.to_lowercase()) {
                    score += 0.1;
                }
            }
        }

        // Entity matching
        if let Some(ref entities) = query.entity_filter {
            for entity in entities {
                if memory.entities.iter().any(|e| e.name == *entity || e.entity_id == *entity) {
                    score += 0.15;
                }
            }
        }

        // Temporal proximity
        if let Some(time_range) = &query.temporal_range {
            let within = memory.timestamp >= time_range.start &&
                        memory.timestamp <= time_range.end;
            if within {
                score += 0.2;
            }
        }

        // Causal chain inclusion
        if query.include_cause_chain {
            score += (memory.causal_links.len() as f32 * 0.05).min(0.2);
        }

        // Apply decay
        score *= 1.0 - (memory.decay_factor * memory.access_count as f32 * 0.01);

        score
    }

    /// Search memories by narrative content
    pub fn search_narrative(&self, pattern: &str, limit: usize) -> Vec<&EpisodicMemory> {
        let pattern_lower = pattern.to_lowercase();

        let mut results: Vec<_> = self.memories.iter()
            .filter(|m| m.narrative.to_lowercase().contains(&pattern_lower))
            .collect();

        results.sort_by(|a, b| {
            let a_matches = a.narrative.to_lowercase().matches(&pattern_lower).count();
            let b_matches = b.narrative.to_lowercase().matches(&pattern_lower).count();
            b_matches.cmp(&a_matches)
        });

        results.into_iter().take(limit).collect()
    }

    /// Get memories from a specific time period
    pub fn get_temporal_slice(&self, start: Instant, end: Instant) -> Vec<&EpisodicMemory> {
        self.memories.iter()
            .filter(|m| m.timestamp >= start && m.timestamp <= end)
            .collect()
    }

    /// Get memories involving specific entities
    pub fn get_memories_with_entities(&self, entities: &[String]) -> Vec<&EpisodicMemory> {
        self.memories.iter()
            .filter(|m| {
                entities.iter().any(|e| {
                    m.entities.iter().any(|mem| &mem.name == e || &mem.entity_id == e)
                })
            })
            .collect()
    }

    /// Get autobiographical timeline
    pub fn get_timeline(&self, timeline_id: &str) -> Option<&Timeline> {
        self.timelines.get(timeline_id)
    }

    /// Create a new timeline from memories
    pub fn create_timeline(&mut self, timeline_id: &str, name: &str, memory_ids: Vec<String>) {
        let memories: Vec<_> = memory_ids.iter()
            .filter_map(|id| self.memory_index.get(id))
            .map(|&idx| self.memories[idx].timestamp)
            .collect();

        let start = memories.iter().min().copied();
        let end = memories.iter().max().copied();

        self.timelines.insert(timeline_id.to_string(), Timeline {
            timeline_id: timeline_id.to_string(),
            name: name.to_string(),
            memory_ids,
            start_time: start.unwrap_or_else(Instant::now),
            end_time: end,
        });
    }

    /// Consolidate memory into more abstract form
    pub fn consolidate(&mut self, memory_id: &str) -> bool {
        if let Some(&idx) = self.memory_index.get(memory_id) {
            let memory = &mut self.memories[idx];

            // Mark as consolidated
            self.consolidated_memories.insert(memory_id.to_string());

            // Increase abstraction
            memory.vividness *= 0.9;
            memory.decay_factor *= 0.5;

            // Extract key patterns
            self.extract_patterns(memory);

            self.stats.consolidation_count += 1;
            true
        } else {
            false
        }
    }

    /// Extract and store patterns from consolidated memory
    fn extract_patterns(&self, memory: &EpisodicMemory) {
        // Theme-based clustering
        let themes = self.extract_themes(memory);
        for theme in themes {
            let cluster_key = theme.to_lowercase();
            self.theme_clusters.entry(cluster_key)
                .or_insert_with(Vec::new)
                .push(memory.memory_id.clone());
        }
    }

    /// Extract themes from memory narrative
    fn extract_themes(&self, memory: &EpisodicMemory) -> Vec<String> {
        let mut themes = Vec::new();
        let narrative = &memory.narrative.to_lowercase();

        // Simple keyword-based theme extraction
        let theme_keywords = [
            ("work", vec!["work", "job", "project", "meeting", "deadline"]),
            ("social", vec!["friend", "family", "party", "conversation", "talk"]),
            ("learning", vec!["learn", "study", "read", "understand", "discover"]),
            ("problem", vec!["problem", "issue", "challenge", "difficult", "conflict"]),
            ("success", vec!["success", "achieved", "won", "accomplished", "happy"]),
        ];

        for (theme, keywords) in theme_keywords {
            if keywords.iter().any(|k| narrative.contains(k)) {
                themes.push(theme.to_string());
            }
        }

        themes
    }

    /// Update theme clusters for a memory
    fn update_theme_clusters(&mut self, memory: &EpisodicMemory) {
        let themes = self.extract_themes(memory);

        for theme in themes {
            let key = theme.to_lowercase();
            self.theme_clusters.entry(key)
                .or_insert_with(Vec::new)
                .push(memory.memory_id.clone());

            // Limit cluster size
            if let Some(ids) = self.theme_clusters.get_mut(&key) {
                if ids.len() > self.config.max_clusters {
                    ids.remove(0);
                }
            }
        }
    }

    /// Update emotional clusters
    fn update_emotional_clusters(&mut self, memory: &EpisodicMemory) {
        let emotion = memory.emotional_context.primary_emotion.clone();
        self.emotional_clusters.entry(emotion)
            .or_insert_with(Vec::new)
            .push(memory.memory_id.clone());
    }

    /// Schedule memory for spaced repetition review
    fn schedule_review(&mut self, memory: &EpisodicMemory) {
        let entry = RepetitionEntry {
            memory_id: memory.memory_id.clone(),
            next_review: Instant::now() + Duration::from_secs(3600), // 1 hour
            interval: Duration::from_secs(3600),
            ease_factor: 2.5,
            repetitions: 0,
        };

        self.repetition_schedule.insert(memory.memory_id.clone(), entry);
    }

    /// Get memories due for review
    pub fn get_review_memories(&self) -> Vec<&EpisodicMemory> {
        let now = Instant::now();

        self.repetition_schedule.iter()
            .filter(|(_, entry)| entry.next_review <= now)
            .filter_map(|(id, _)| {
                self.memory_index.get(id).map(|&idx| &self.memories[idx])
            })
            .collect()
    }

    /// Update stats after storing a memory
    fn update_stats(&mut self, memory: &EpisodicMemory) {
        self.stats.total_memories += 1;
        self.stats.total_events += memory.events.len();
        self.stats.total_entities += memory.entities.len();

        *self.stats.memories_by_emotion
            .entry(memory.emotional_context.primary_emotion.clone())
            .or_insert(0) += 1;

        // Update average importance
        let total = self.stats.total_memories as f32;
        let current_avg = self.stats.average_importance;
        self.stats.average_importance = current_avg + (memory.importance - current_avg) / total;
    }

    /// Evict oldest memory based on decay
    fn evict_oldest(&mut self) {
        // Find memory with lowest combined score
        let mut lowest_idx = 0;
        let mut lowest_score = f32::MAX;

        for (idx, memory) in self.memories.iter().enumerate() {
            let score = memory.importance * memory.accessibility *
                       (1.0 - memory.decay_factor * self.stats.total_memories as f32 * 0.001);

            if score < lowest_score {
                lowest_score = score;
                lowest_idx = idx;
            }
        }

        // Remove memory
        if let Some(removed) = self.memories.remove(lowest_idx) {
            self.memory_index.remove(&removed.memory_id);
            self.consolidated_memories.remove(&removed.memory_id);
            self.repetition_schedule.remove(&removed.memory_id);
        }

        // Re-index
        for (idx, memory) in self.memories.iter().enumerate() {
            self.memory_index.insert(memory.memory_id.clone(), idx);
        }
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Replay a memory sequence
    pub fn replay_sequence(&self, memory_ids: &[String]) -> Vec<EpisodicMemory> {
        memory_ids.iter()
            .filter_map(|id| self.memory_index.get(id))
            .map(|&idx| self.memories[idx].clone())
            .collect()
    }

    /// Form associative link between memories
    pub fn form_association(&mut self, memory_id1: &str, memory_id2: &str, strength: f32) -> bool {
        let get_idx = |store: &Self, id: &str| store.memory_index.get(id).copied();

        if let (Some(idx1), Some(idx2)) = (get_idx(self, memory_id1), get_idx(self, memory_id2)) {
            let mem1 = &mut self.memories[idx1];
            if !mem1.associated_memories.contains(&memory_id2.to_string()) {
                mem1.associated_memories.push(memory_id2.to_string());
            }

            let mem2 = &mut self.memories[idx2];
            if !mem2.associated_memories.contains(&memory_id1.to_string()) {
                mem2.associated_memories.push(memory_id1.to_string());
            }

            true
        } else {
            false
        }
    }
}

/// Query for memory retrieval
#[derive(Debug, Clone, Default)]
pub struct MemoryQuery {
    pub keywords: Option<Vec<String>>,
    pub entity_filter: Option<Vec<String>>,
    pub emotional_filter: Option<String>,
    pub temporal_range: Option<TemporalRange>,
    pub importance_threshold: Option<f32>,
    pub include_cause_chain: bool,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct TemporalRange {
    pub start: Instant,
    pub end: Instant,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("epmem_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_storage() {
        let mut store = EpisodicMemoryStore::new();

        let memory = EpisodicMemory {
            memory_id: generate_id(),
            event_id: "evt1".to_string(),
            timestamp: Instant::now(),
            duration: Some(Duration::from_secs(3600)),
            narrative: "Had a productive meeting about the project".to_string(),
            events: vec![],
            entities: vec![
                MemoryEntity {
                    entity_id: "self".to_string(),
                    entity_type: EntityType::Self_,
                    name: "Self".to_string(),
                    role: "participant".to_string(),
                    properties: HashMap::new(),
                }
            ],
            locations: vec![],
            emotional_context: EmotionalContext {
                primary_emotion: "satisfied".to_string(),
                emotion_intensity: 0.7,
                emotional_states: HashMap::new(),
            },
            sensory_context: SensoryContext {
                visual: None,
                auditory: None,
                proprioceptive: None,
                modalities: HashMap::new(),
            },
            spatial_context: SpatialContext {
                current_location: "office".to_string(),
                spatial_relations: vec![],
                navigation_path: vec![],
            },
            importance: 0.8,
            vividness: 0.9,
            accessibility: 1.0,
            valence: 0.6,
            arousal: 0.5,
            associated_memories: vec![],
            causal_links: vec![],
            access_count: 0,
            last_accessed: None,
            decay_factor: 0.01,
        };

        store.store(memory);
        assert_eq!(store.stats.total_memories, 1);
    }

    #[test]
    fn test_memory_retrieval() {
        let mut store = EpisodicMemoryStore::new();

        // Store test memories
        let memory = EpisodicMemory {
            memory_id: generate_id(),
            event_id: "evt1".to_string(),
            timestamp: Instant::now(),
            duration: Some(Duration::from_secs(3600)),
            narrative: "Learned about machine learning".to_string(),
            events: vec![],
            entities: vec![
                MemoryEntity {
                    entity_id: "ml".to_string(),
                    entity_type: EntityType::Abstract,
                    name: "ML".to_string(),
                    role: "topic".to_string(),
                    properties: HashMap::new(),
                }
            ],
            locations: vec![],
            emotional_context: EmotionalContext {
                primary_emotion: "curious".to_string(),
                emotion_intensity: 0.8,
                emotional_states: HashMap::new(),
            },
            sensory_context: SensoryContext {
                visual: None,
                auditory: None,
                proprioceptive: None,
                modalities: HashMap::new(),
            },
            spatial_context: SpatialContext {
                current_location: "study".to_string(),
                spatial_relations: vec![],
                navigation_path: vec![],
            },
            importance: 0.9,
            vividness: 0.8,
            accessibility: 1.0,
            valence: 0.7,
            arousal: 0.6,
            associated_memories: vec![],
            causal_links: vec![],
            access_count: 0,
            last_accessed: None,
            decay_factor: 0.01,
        };

        store.store(memory);

        let query = MemoryQuery {
            keywords: Some(vec!["learning".to_string()]),
            entity_filter: None,
            emotional_filter: None,
            temporal_range: None,
            importance_threshold: None,
            include_cause_chain: false,
            limit: Some(5),
        };

        let results = store.retrieve(&query);
        assert!(!results.is_empty());
    }
}
