//! Tariro Memory Architecture
//!
//! Implements the dual-store memory model inspired by cognitive psychology:
//! - **Episodic Memory**: Autobiographical experiences with temporal context
//! - **Semantic Memory**: Generalized world knowledge independent of personal experience
//! - **Procedural Memory**: Learned skills and habits
//!
//! This module provides the memory substrate for Tariro's cognition,
//! enabling both recall of specific experiences and retrieval of abstract knowledge.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for memory items
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MemoryId(pub Uuid);

impl MemoryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MemoryId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MemoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Memory trace strength (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TraceStrength(pub f64);

impl TraceStrength {
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn decay(&self, rate: f64) -> Self {
        Self(self.0 * (1.0 - rate).max(0.0))
    }

    pub fn boost(&self, amount: f64) -> Self {
        Self((self.0 + amount).clamp(0.0, 1.0))
    }
}

impl Default for TraceStrength {
    fn default() -> Self {
        Self(0.5)
    }
}

/// Memory content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContent {
    /// Concrete experience or event
    Episode(Episode),
    /// Abstract knowledge or concept
    Semantic(SemanticKnowledge),
    /// Skill or procedure
    Procedure(Procedure),
}

/// A temporal episode - a specific experience in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    /// Unique identifier
    pub id: MemoryId,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    /// Duration of the episode
    pub duration_ms: Option<u64>,
    /// Location context
    pub location: Option<String>,
    /// Emotional valence (-1.0 to 1.0)
    pub emotional_valence: f64,
    /// Emotional arousal (0.0 to 1.0)
    pub emotional_arousal: f64,
    /// What happened
    pub event: Event,
    /// Associated entities
    pub entities: Vec<MemoryEntity>,
    /// Causal chain
    pub causal_chain: Vec<CausalLink>,
    /// Outcome
    pub outcome: Option<Outcome>,
    /// Importance weight
    pub importance: f64,
    /// Memory trace strength
    pub trace: TraceStrength,
    /// Retrieval count
    pub retrieval_count: u32,
    /// Last retrieval time
    pub last_retrieval: Option<DateTime<Utc>>,
}

impl Episode {
    pub fn new(event: Event) -> Self {
        Self {
            id: MemoryId::new(),
            timestamp: Utc::now(),
            duration_ms: None,
            location: None,
            emotional_valence: 0.0,
            emotional_arousal: 0.0,
            event,
            entities: Vec::new(),
            causal_chain: Vec::new(),
            outcome: None,
            importance: 0.5,
            trace: TraceStrength::default(),
            retrieval_count: 0,
            last_retrieval: None,
        }
    }

    /// Add an entity to the episode
    pub fn with_entity(mut self, entity: MemoryEntity) -> Self {
        self.entities.push(entity);
        self
    }

    /// Add emotional context
    pub fn with_emotion(mut self, valence: f64, arousal: f64) -> Self {
        self.emotional_valence = valence.clamp(-1.0, 1.0);
        self.emotional_arousal = arousal.clamp(0.0, 1.0);
        self
    }

    /// Set importance
    pub fn with_importance(mut self, importance: f64) -> Self {
        self.importance = importance.clamp(0.0, 1.0);
        self
    }

    /// Record retrieval
    pub fn record_retrieval(&mut self) {
        self.retrieval_count += 1;
        self.last_retrieval = Some(Utc::now());
        self.trace = self.trace.boost(0.1);
    }

    /// Apply decay
    pub fn decay(&mut self, rate: f64) {
        self.trace = self.trace.decay(rate);
    }
}

/// An event within an episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// What occurred
    pub description: String,
    /// Type of event
    pub event_type: EventType,
    /// Temporal ordering
    pub sequence_position: usize,
    /// Associated predicates
    pub predicates: Vec<Predicate>,
}

impl Event {
    pub fn new(description: String, event_type: EventType) -> Self {
        Self {
            description,
            event_type,
            sequence_position: 0,
            predicates: Vec::new(),
        }
    }
}

/// Types of events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// Perceptual input
    Perception,
    /// Internal thought
    Thought,
    /// Action taken
    Action,
    /// External action on agent
    Interaction,
    /// Emotional response
    Emotion,
    /// Decision made
    Decision,
    /// Learning occurred
    Learning,
    /// Planning
    Planning,
    /// Goal state change
    GoalChange,
}

impl EventType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EventType::Perception => "perception",
            EventType::Thought => "thought",
            EventType::Action => "action",
            EventType::Interaction => "interaction",
            EventType::Emotion => "emotion",
            EventType::Decision => "decision",
            EventType::Learning => "learning",
            EventType::Planning => "planning",
            EventType::GoalChange => "goal_change",
        }
    }
}

/// Predicate for describing events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Predicate {
    pub name: String,
    pub arguments: Vec<String>,
}

impl Predicate {
    pub fn new(name: &str, arguments: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            arguments,
        }
    }
}

/// Entity referenced in memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntity {
    pub id: String,
    pub name: String,
    pub entity_type: String,
    pub role: EntityRole,
}

impl MemoryEntity {
    pub fn new(name: &str, entity_type: &str, role: EntityRole) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            entity_type: entity_type.to_string(),
            role,
        }
    }
}

/// Role of an entity in an episode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityRole {
    /// Agent itself
    Self_,
    /// Actor performing action
    Actor,
    /// Object being acted upon
    Object,
    /// Tool or means
    Tool,
    /// Location
    Location,
    /// Other participant
    Participant,
    /// Witness
    Witness,
}

/// Causal link in episode chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub cause: String,
    pub effect: String,
    pub mechanism: String,
    pub confidence: f64,
}

impl CausalLink {
    pub fn new(cause: &str, effect: &str, mechanism: &str) -> Self {
        Self {
            cause: cause.to_string(),
            effect: effect.to_string(),
            mechanism: mechanism.to_string(),
            confidence: 0.5,
        }
    }
}

/// Outcome of an episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub description: String,
    pub success: bool,
    pub expected: bool,
    pub deviation: Option<String>,
}

/// Semantic knowledge - abstract world knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticKnowledge {
    pub id: MemoryId,
    /// The concept or fact
    pub concept: String,
    /// Category
    pub category: KnowledgeCategory,
    /// Definition or description
    pub description: String,
    /// Associated predicates
    pub predicates: Vec<Predicate>,
    /// Links to other semantic memories
    pub associations: Vec<Association>,
    /// Source of knowledge
    pub source: KnowledgeSource,
    /// Confidence in truth
    pub confidence: f64,
    /// Last updated
    pub updated: DateTime<Utc>,
    /// Memory trace
    pub trace: TraceStrength,
}

impl SemanticKnowledge {
    pub fn new(concept: &str, category: KnowledgeCategory, description: &str) -> Self {
        Self {
            id: MemoryId::new(),
            concept: concept.to_string(),
            category,
            description: description.to_string(),
            predicates: Vec::new(),
            associations: Vec::new(),
            source: KnowledgeSource::Derived,
            confidence: 0.5,
            updated: Utc::now(),
            trace: TraceStrength::default(),
        }
    }

    /// Add an association
    pub fn with_association(mut self, association: Association) -> Self {
        self.associations.push(association);
        self
    }
}

/// Knowledge categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeCategory {
    /// Physical world knowledge
    Physical,
    /// Social/interpersonal knowledge
    Social,
    /// Mental/psychological knowledge
    Mental,
    /// Causal relationships
    Causal,
    /// Procedural knowledge
    Procedural,
    /// Linguistic knowledge
    Linguistic,
    /// Mathematical knowledge
    Mathematical,
    /// Scientific knowledge
    Scientific,
    /// Cultural knowledge
    Cultural,
    /// Other abstract knowledge
    Abstract,
}

impl KnowledgeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            KnowledgeCategory::Physical => "physical",
            KnowledgeCategory::Social => "social",
            KnowledgeCategory::Mental => "mental",
            KnowledgeCategory::Causal => "causal",
            KnowledgeCategory::Procedural => "procedural",
            KnowledgeCategory::Linguistic => "linguistic",
            KnowledgeCategory::Mathematical => "mathematical",
            KnowledgeCategory::Scientific => "scientific",
            KnowledgeCategory::Cultural => "cultural",
            KnowledgeCategory::Abstract => "abstract",
        }
    }
}

/// Source of knowledge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeSource {
    /// Direct experience
    Experience,
    /// Learned from others
    Taught,
    /// Derived through reasoning
    Derived,
    /// Read or observed
    Observed,
    /// Intuitive belief
    Intuitive,
}

/// Association between semantic memories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    pub target_id: MemoryId,
    pub association_type: AssociationType,
    pub strength: f64,
}

impl Association {
    pub fn new(target_id: MemoryId, association_type: AssociationType) -> Self {
        Self {
            target_id,
            association_type,
            strength: 0.5,
        }
    }
}

/// Types of associations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssociationType {
    /// Is-a relationship
    IsA,
    /// Part-of relationship
    PartOf,
    /// Similarity
    Similar,
    /// Causally related
    Causes,
    /// Enabled by
    Enables,
    /// Contradicts
    Contradicts,
    /// Associated with
    Associated,
}

/// A learned procedure/skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub id: MemoryId,
    /// Name of the procedure
    pub name: String,
    /// Domain where applicable
    pub domain: String,
    /// Steps in the procedure
    pub steps: Vec<ProcedureStep>,
    /// Preconditions
    pub preconditions: Vec<String>,
    /// Postconditions (expected outcomes)
    pub postconditions: Vec<String>,
    /// Success rate
    pub success_rate: f64,
    /// Times practiced
    pub practice_count: u32,
    /// Automation level (0=conscious, 1=automatic)
    pub automation_level: f64,
}

impl Procedure {
    pub fn new(name: &str, domain: &str) -> Self {
        Self {
            id: MemoryId::new(),
            name: name.to_string(),
            domain: domain.to_string(),
            steps: Vec::new(),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            success_rate: 0.0,
            practice_count: 0,
            automation_level: 0.0,
        }
    }

    /// Add a step
    pub fn with_step(mut self, step: ProcedureStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Record successful practice
    pub fn record_practice(&mut self, success: bool) {
        self.practice_count += 1;
        // Update success rate with exponential moving average
        let alpha = 0.2;
        if success {
            self.success_rate = alpha * 1.0 + (1.0 - alpha) * self.success_rate;
        } else {
            self.success_rate = (1.0 - alpha) * self.success_rate;
        }
        // Increase automation with practice
        self.automation_level = (self.automation_level + 0.05).min(1.0);
    }
}

/// A step in a procedure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureStep {
    pub order: usize,
    pub description: String,
    pub action: String,
    pub conditions: Vec<String>,
    pub duration_ms: Option<u64>,
}

// ============================================================================
// Episodic Memory
// ============================================================================

/// Episodic memory store - autobiographical experiences
#[derive(Debug, Clone)]
pub struct EpisodicMemory {
    /// Chronological episodes
    episodes: Vec<Episode>,
    /// Index by timestamp
    time_index: HashMap<DateTime<Utc>, Vec<MemoryId>>,
    /// Index by entity
    entity_index: HashMap<String, Vec<MemoryId>>,
    /// Index by event type
    type_index: HashMap<EventType, Vec<MemoryId>>,
    /// Index by location
    location_index: HashMap<String, Vec<MemoryId>>,
    /// Maximum episodes to retain
    capacity: usize,
    /// Statistics
    stats: MemoryStats,
}

impl EpisodicMemory {
    pub fn new() -> Self {
        Self {
            episodes: Vec::new(),
            time_index: HashMap::new(),
            entity_index: HashMap::new(),
            type_index: HashMap::new(),
            location_index: HashMap::new(),
            capacity: 10_000,
            stats: MemoryStats::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut mem = Self::new();
        mem.capacity = capacity;
        mem
    }

    /// Store a new episode
    pub fn store(&mut self, mut episode: Episode) -> Result<MemoryId, MemoryError> {
        // Calculate initial trace based on importance and emotion
        let base_trace = episode.importance * 0.5;
        let emotion_boost = (episode.emotional_arousal * episode.emotional_valence.abs()) * 0.3;
        episode.trace = TraceStrength::new(base_trace + emotion_boost);

        let id = episode.id.clone();

        // Add to indices
        self.time_index
            .entry(episode.timestamp)
            .or_default()
            .push(id.clone());

        for entity in &episode.entities {
            self.entity_index
                .entry(entity.name.clone())
                .or_default()
                .push(id.clone());
        }

        self.type_index
            .entry(episode.event.event_type)
            .or_default()
            .push(id.clone());

        if let Some(ref location) = episode.location {
            self.location_index
                .entry(location.clone())
                .or_default()
                .push(id.clone());
        }

        // Add episode
        self.episodes.push(episode);
        self.stats.episodes_stored += 1;
        self.stats.total_episodes += 1;

        // Evict old episodes if over capacity
        if self.episodes.len() > self.capacity {
            self.evict_oldest();
        }

        Ok(id)
    }

    /// Retrieve episode by ID
    pub fn retrieve(&mut self, id: &MemoryId) -> Option<&Episode> {
        self.episodes.iter().find(|e| &e.id == id).map(|e| {
            // Update retrieval metrics
            if let Some(ep) = self.episodes.iter_mut().find(|ep| &ep.id == id) {
                ep.record_retrieval();
                self.stats.episodes_retrieved += 1;
            }
            e
        })
    }

    /// Retrieve episode by ID (mutable)
    pub fn retrieve_mut(&mut self, id: &MemoryId) -> Option<&mut Episode> {
        self.episodes.iter_mut().find(|e| &e.id == id).map(|e| {
            e.record_retrieval();
            self.stats.episodes_retrieved += 1;
            e
        })
    }

    /// Retrieve by temporal context
    pub fn retrieve_temporal(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<&Episode> {
        self.episodes
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }

    /// Retrieve by entity
    pub fn retrieve_by_entity(&self, entity_name: &str) -> Vec<&Episode> {
        self.entity_index
            .get(entity_name)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.episodes.iter().find(|e| &e.id == id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retrieve by event type
    pub fn retrieve_by_type(&self, event_type: EventType) -> Vec<&Episode> {
        self.type_index
            .get(&event_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.episodes.iter().find(|e| &e.id == id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retrieve similar episodes (same event type + entities)
    pub fn retrieve_similar(&self, query: &Episode) -> Vec<(&Episode, f64)> {
        let mut similarities: Vec<(&Episode, f64)> = self
            .episodes
            .iter()
            .filter(|e| e.id != query.id)
            .map(|e| {
                let mut score = 0.0;

                // Same event type
                if e.event.event_type == query.event.event_type {
                    score += 0.3;
                }

                // Entity overlap
                let query_entities: Vec<_> = query.entities.iter().map(|e| &e.name).collect();
                let e_entities: Vec<_> = e.entities.iter().map(|e| &e.name).collect();
                let overlap: Vec<_> = query_entities
                    .iter()
                    .filter(|q| e_entities.contains(q))
                    .collect();
                if !overlap.is_empty() {
                    score += 0.4 * (overlap.len() as f64 / query_entities.len().max(1) as f64);
                }

                // Similar emotion
                let emotion_diff =
                    (e.emotional_valence - query.emotional_valence).abs()
                    + (e.emotional_arousal - query.emotional_arousal).abs();
                score += 0.3 * (1.0 - emotion_diff.min(1.0));

                (e, score)
            })
            .filter(|(_, score)| *score > 0.3)
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities
    }

    /// Retrieve by temporal proximity to now
    pub fn retrieve_recent(&self, count: usize) -> Vec<&Episode> {
        let mut episodes: Vec<_> = self.episodes.iter().collect();
        episodes.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        episodes.into_iter().take(count).collect()
    }

    /// Search episodes by description
    pub fn search(&self, query: &str) -> Vec<&Episode> {
        let query_lower = query.to_lowercase();
        self.episodes
            .iter()
            .filter(|e| {
                e.event.description.to_lowercase().contains(&query_lower)
                    || e.outcome
                        .as_ref()
                        .map(|o| o.description.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .collect()
    }

    /// Apply decay to all episodes
    pub fn decay_all(&mut self, rate: f64) {
        for episode in &mut self.episodes {
            episode.decay(ray);
        }
        self.stats.decay_operations += 1;
    }

    /// Get recent episodes within time window
    pub fn get_recent_window(&self, window_hours: i64) -> Vec<&Episode> {
        let cutoff = Utc::now() - chrono::Duration::hours(window_hours);
        self.episodes
            .iter()
            .filter(|e| e.timestamp > cutoff)
            .collect()
    }

    /// Get statistics
    pub fn stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Get episode count
    pub fn len(&self) -> usize {
        self.episodes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.episodes.is_empty()
    }

    /// Evict oldest episode
    fn evict_oldest(&mut self) {
        if let Some(oldest) = self.episodes.first() {
            let id = &oldest.id;

            // Remove from indices
            self.time_index.retain(|_, ids| {
                ids.retain(|i| i != id);
                !ids.is_empty()
            });

            for entity in &oldest.entities {
                self.entity_index.retain(|_, ids| {
                    ids.retain(|i| i != id);
                    !ids.is_empty()
                });
            }

            self.type_index.retain(|_, ids| {
                ids.retain(|i| i != id);
                !ids.is_empty()
            });

            if let Some(ref location) = oldest.location {
                self.location_index.retain(|_, ids| {
                    ids.retain(|i| i != id);
                    !ids.is_empty()
                });
            }

            self.episodes.remove(0);
            self.stats.episodes_evicted += 1;
        }
    }

    /// Consolidate episodic to semantic memories
    pub fn consolidate(&self, threshold: f64) -> Vec<SemanticKnowledge> {
        let mut semantic_knowledge = Vec::new();

        // Group similar episodes
        let mut episode_groups: HashMap<String, Vec<&Episode>> = HashMap::new();

        for episode in &self.episodes {
            let key = format!(
                "{}:{:?}",
                episode.event.event_type.as_str(),
                episode.event.description.split_whitespace().take(3).collect::<String>()
            );
            episode_groups.entry(key).or_default().push(episode);
        }

        // Convert repeated patterns to semantic knowledge
        for (key, episodes) in episode_groups {
            if episodes.len() >= 2 {
                let avg_importance: f64 =
                    episodes.iter().map(|e| e.importance).sum::<f64>() / episodes.len() as f64;

                if avg_importance >= threshold {
                    let first = episodes[0];
                    let knowledge = SemanticKnowledge::new(
                        &key,
                        KnowledgeCategory::Procedural,
                        &format!(
                            "Pattern: {} (observed {} times)",
                            first.event.description, episodes.len()
                        ),
                    );
                    semantic_knowledge.push(knowledge);
                }
            }
        }

        semantic_knowledge
    }
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Semantic Memory
// ============================================================================

/// Semantic memory store - abstract world knowledge
#[derive(Debug, Clone)]
pub struct SemanticMemory {
    /// Knowledge base
    knowledge: HashMap<MemoryId, SemanticKnowledge>,
    /// Concept name index
    name_index: HashMap<String, MemoryId>,
    /// Category index
    category_index: HashMap<KnowledgeCategory, Vec<MemoryId>>,
    /// Association graph
    association_graph: HashMap<MemoryId, Vec<Association>>,
    /// Statistics
    stats: MemoryStats,
}

impl SemanticMemory {
    pub fn new() -> Self {
        Self {
            knowledge: HashMap::new(),
            name_index: HashMap::new(),
            category_index: HashMap::new(),
            association_graph: HashMap::new(),
            stats: MemoryStats::default(),
        }
    }

    /// Store new knowledge
    pub fn store(&mut self, mut knowledge: SemanticKnowledge) -> Result<MemoryId, MemoryError> {
        let id = knowledge.id.clone();

        // Check for existing with same concept name
        if let Some(existing_id) = self.name_index.get(&knowledge.concept) {
            // Update existing instead
            if let Some(existing) = self.knowledge.get_mut(existing_id) {
                // Merge associations
                for assoc in knowledge.associations.clone() {
                    if !existing.associations.contains(&assoc) {
                        existing.associations.push(assoc);
                    }
                }
                // Update if new knowledge has higher confidence
                if knowledge.confidence > existing.confidence {
                    existing.description = knowledge.description.clone();
                    existing.confidence = knowledge.confidence;
                }
                existing.updated = Utc::now();
                self.stats.knowledge_updated += 1;
                return Ok(existing_id.clone());
            }
        }

        // Store new
        let cat_id = knowledge.id.clone();
        self.knowledge.insert(id.clone(), knowledge);

        // Update indices
        self.name_index
            .entry(knowledge.concept.clone())
            .or_insert_with(|| id.clone());

        self.category_index
            .entry(knowledge.category)
            .or_default()
            .push(cat_id);

        self.association_graph
            .entry(id.clone())
            .or_default();

        self.stats.knowledge_stored += 1;
        self.stats.total_knowledge += 1;

        Ok(id)
    }

    /// Retrieve knowledge by ID
    pub fn retrieve(&self, id: &MemoryId) -> Option<&SemanticKnowledge> {
        self.knowledge.get(id)
    }

    /// Retrieve by concept name
    pub fn retrieve_by_name(&self, name: &str) -> Option<&SemanticKnowledge> {
        self.name_index.get(name).and_then(|id| self.knowledge.get(id))
    }

    /// Retrieve by category
    pub fn retrieve_by_category(&self, category: KnowledgeCategory) -> Vec<&SemanticKnowledge> {
        self.category_index
            .get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.knowledge.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retrieve associated knowledge
    pub fn get_associations(&self, id: &MemoryId) -> Vec<&SemanticKnowledge> {
        self.association_graph
            .get(id)
            .map(|assocs| {
                assocs
                    .iter()
                    .filter_map(|a| self.knowledge.get(&a.target_id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Add association between two knowledge items
    pub fn add_association(&mut self, source: &MemoryId, target: &MemoryId, assoc_type: AssociationType) {
        if let Some(assocs) = self.association_graph.get_mut(source) {
            if !assocs.iter().any(|a| &a.target_id == target) {
                assocs.push(Association::new(target.clone(), assoc_type));
                self.stats.associations_added += 1;
            }
        }
    }

    /// Search knowledge
    pub fn search(&self, query: &str) -> Vec<&SemanticKnowledge> {
        let query_lower = query.to_lowercase();
        self.knowledge
            .values()
            .filter(|k| {
                k.concept.to_lowercase().contains(&query_lower)
                    || k.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Retrieve related through transitive closure
    pub fn get_related(&self, id: &MemoryId, depth: usize) -> Vec<&SemanticKnowledge> {
        let mut visited = std::collections::HashSet::new();
        let mut result = Vec::new();
        self.traverse_associations(id, depth, &mut visited, &mut result);
        result
    }

    fn traverse_associations(
        &self,
        id: &MemoryId,
        depth: usize,
        visited: &mut std::collections::HashSet<MemoryId>,
        result: &mut Vec<&SemanticKnowledge>,
    ) {
        if depth == 0 || visited.contains(id) {
            return;
        }

        visited.insert(id.clone());

        if let Some(knowledge) = self.knowledge.get(id) {
            result.push(knowledge);
        }

        if let Some(assocs) = self.association_graph.get(id) {
            for assoc in assocs {
                self.traverse_associations(&assoc.target_id, depth - 1, visited, result);
            }
        }
    }

    /// Get statistics
    pub fn stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Get knowledge count
    pub fn len(&self) -> usize {
        self.knowledge.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.knowledge.is_empty()
    }

    /// Build common sense knowledge base
    pub fn initialize_commonsense(&mut self) {
        let common_sense_knowledge = vec![
            (
                "Fire",
                KnowledgeCategory::Physical,
                "Fire is hot and produces heat. It requires fuel, oxygen, and heat to burn.",
            ),
            (
                "Water",
                KnowledgeCategory::Physical,
                "Water is a liquid at room temperature. It is essential for life and can extinguish fire.",
            ),
            (
                "Human",
                KnowledgeCategory::Social,
                "Humans are social beings that communicate through language. They have needs like food, water, and shelter.",
            ),
            (
                "Time",
                KnowledgeCategory::Abstract,
                "Time flows in one direction (forward). Events in the past cannot be changed.",
            ),
            (
                "Causality",
                KnowledgeCategory::Causal,
                "Every effect has a cause. Causes precede their effects temporally.",
            ),
            (
                "Object Persistence",
                KnowledgeCategory::Physical,
                "Objects continue to exist even when not observed. They don't disappear spontaneously.",
            ),
            (
                "Social Norms",
                KnowledgeCategory::Social,
                "People generally expect polite behavior, honesty, and reciprocity in social interactions.",
            ),
        ];

        for (concept, category, description) in common_sense_knowledge {
            let knowledge = SemanticKnowledge::new(concept, category, description)
                .with_association(Association::new(MemoryId::new(), AssociationType::Associated));
            let _ = self.store(knowledge);
        }
    }
}

impl Default for SemanticMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Procedural Memory
// ============================================================================

/// Procedural memory store - learned skills
#[derive(Debug, Clone)]
pub struct ProceduralMemory {
    /// Procedures by ID
    procedures: HashMap<MemoryId, Procedure>,
    /// Procedures by domain
    domain_index: HashMap<String, Vec<MemoryId>>,
    /// Procedures by name
    name_index: HashMap<String, MemoryId>,
    /// Statistics
    stats: MemoryStats,
}

impl ProceduralMemory {
    pub fn new() -> Self {
        Self {
            procedures: HashMap::new(),
            domain_index: HashMap::new(),
            name_index: HashMap::new(),
            stats: MemoryStats::default(),
        }
    }

    /// Store a procedure
    pub fn store(&mut self, mut procedure: Procedure) -> Result<MemoryId, MemoryError> {
        let id = procedure.id.clone();

        // Check for existing with same name
        if let Some(existing_id) = self.name_index.get(&procedure.name) {
            // Update existing
            if let Some(existing) = self.procedures.get_mut(existing_id) {
                existing.practice_count = procedure.practice_count;
                existing.success_rate = procedure.success_rate;
                existing.automation_level = procedure.automation_level;
                self.stats.procedures_updated += 1;
                return Ok(existing_id.clone());
            }
        }

        // Store new
        self.procedures.insert(id.clone(), procedure);
        self.name_index
            .entry(procedure.name.clone())
            .or_insert_with(|| id.clone());
        self.domain_index
            .entry(procedure.domain.clone())
            .or_default()
            .push(id.clone());

        self.stats.procedures_stored += 1;
        self.stats.total_procedures += 1;

        Ok(id)
    }

    /// Retrieve procedure
    pub fn retrieve(&self, id: &MemoryId) -> Option<&Procedure> {
        self.procedures.get(id)
    }

    /// Retrieve by name
    pub fn retrieve_by_name(&self, name: &str) -> Option<&Procedure> {
        self.name_index.get(name).and_then(|id| self.procedures.get(id))
    }

    /// Retrieve by domain
    pub fn retrieve_by_domain(&self, domain: &str) -> Vec<&Procedure> {
        self.domain_index
            .get(domain)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.procedures.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get most practiced procedures
    pub fn get_expert_procedures(&self) -> Vec<&Procedure> {
        let mut procs: Vec<_> = self.procedures.values().collect();
        procs.sort_by(|a, b| {
            b.automation_level
                .partial_cmp(&a.automation_level)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        procs.into_iter().take(10).collect()
    }

    /// Practice a procedure
    pub fn practice(&mut self, id: &MemoryId, success: bool) -> Result<(), MemoryError> {
        if let Some(proc) = self.procedures.get_mut(id) {
            proc.record_practice(success);
            self.stats.practice_sessions += 1;
            Ok(())
        } else {
            Err(MemoryError::NotFound(id.clone()))
        }
    }

    /// Get statistics
    pub fn stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Get procedure count
    pub fn len(&self) -> usize {
        self.procedures.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.procedures.is_empty()
    }
}

impl Default for ProceduralMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Memory System
// ============================================================================

/// Unified memory system
#[derive(Debug, Clone)]
pub struct MemorySystem {
    pub episodic: EpisodicMemory,
    pub semantic: SemanticMemory,
    pub procedural: ProceduralMemory,
    config: MemoryConfig,
    stats: SystemStats,
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            episodic: EpisodicMemory::new(),
            semantic: SemanticMemory::new(),
            procedural: ProceduralMemory::new(),
            config: MemoryConfig::default(),
            stats: SystemStats::default(),
        }
    }

    pub fn with_config(config: MemoryConfig) -> Self {
        Self {
            episodic: EpisodicMemory::with_capacity(config.episodic_capacity),
            semantic: SemanticMemory::new(),
            procedural: ProceduralMemory::new(),
            config,
            stats: SystemStats::default(),
        }
    }

    /// Initialize memory with common sense
    pub fn initialize(&mut self) {
        self.semantic.initialize_commonsense();
        self.stats.initializations += 1;
    }

    /// Store episode
    pub fn store_episode(&mut self, episode: Episode) -> Result<MemoryId, MemoryError> {
        self.episodic.store(episode)
    }

    /// Store semantic knowledge
    pub fn store_knowledge(&mut self, knowledge: SemanticKnowledge) -> Result<MemoryId, MemoryError> {
        self.semantic.store(knowledge)
    }

    /// Store procedure
    pub fn store_procedure(&mut self, procedure: Procedure) -> Result<MemoryId, MemoryError> {
        self.procedural.store(procedure)
    }

    /// Retrieve episode
    pub fn recall_episode(&mut self, id: &MemoryId) -> Option<Episode> {
        self.episodic.retrieve(id).cloned()
    }

    /// Retrieve knowledge
    pub fn recall_knowledge(&self, id: &MemoryId) -> Option<SemanticKnowledge> {
        self.semantic.retrieve(id).cloned()
    }

    /// Retrieve procedure
    pub fn recall_procedure(&self, id: &MemoryId) -> Option<Procedure> {
        self.procedural.retrieve(id).cloned()
    }

    /// Search across all memory types
    pub fn search(&self, query: &str) -> MemorySearchResult {
        MemorySearchResult {
            episodes: self.episodic.search(query),
            knowledge: self.semantic.search(query),
            procedures: Vec::new(), // Search by name would need separate implementation
        }
    }

    /// Get recent episodes
    pub fn get_recent_experiences(&self, count: usize) -> Vec<&Episode> {
        self.episodic.retrieve_recent(count)
    }

    /// Get related knowledge
    pub fn get_related_knowledge(&self, id: &MemoryId) -> Vec<&SemanticKnowledge> {
        self.semantic.get_related(id, 2)
    }

    /// Consolidate memories (episodic -> semantic)
    pub fn consolidate(&mut self) {
        let threshold = self.config.consolidation_threshold;
        let new_knowledge = self.episodic.consolidate(threshold);

        for knowledge in new_knowledge {
            if let Err(e) = self.semantic.store(knowledge) {
                eprintln!("Failed to consolidate: {:?}", e);
            }
        }

        self.stats.consolidations += 1;
    }

    /// Apply decay to all memories
    pub fn decay(&mut self) {
        self.episodic.decay_all(self.config.decay_rate);
        self.stats.decay_operations += 1;
    }

    /// Get comprehensive stats
    pub fn stats(&self) -> MemorySystemStats {
        MemorySystemStats {
            episodic: self.episodic.stats().clone(),
            semantic: self.semantic.stats().clone(),
            procedural: self.procedural.stats().clone(),
            system: self.stats.clone(),
        }
    }
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Memory system configuration
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub episodic_capacity: usize,
    pub semantic_capacity: usize,
    pub procedural_capacity: usize,
    pub decay_rate: f64,
    pub consolidation_threshold: f64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            episodic_capacity: 10_000,
            semantic_capacity: 100_000,
            procedural_capacity: 1_000,
            decay_rate: 0.01,
            consolidation_threshold: 0.7,
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone, Default)]
pub struct MemoryStats {
    pub total_episodes: usize,
    pub total_knowledge: usize,
    pub total_procedures: usize,
    pub episodes_stored: usize,
    pub episodes_retrieved: usize,
    pub episodes_evicted: usize,
    pub knowledge_stored: usize,
    pub knowledge_updated: usize,
    pub associations_added: usize,
    pub procedures_stored: usize,
    pub procedures_updated: usize,
    pub practice_sessions: usize,
    pub decay_operations: usize,
    pub consolidations: usize,
}

impl Clone for MemoryStats {
    fn clone(&self) -> Self {
        Self {
            total_episodes: self.total_episodes,
            total_knowledge: self.total_knowledge,
            total_procedures: self.total_procedures,
            episodes_stored: self.episodes_stored,
            episodes_retrieved: self.episodes_retrieved,
            episodes_evicted: self.episodes_evicted,
            knowledge_stored: self.knowledge_stored,
            knowledge_updated: self.knowledge_updated,
            associations_added: self.associations_added,
            procedures_stored: self.procedures_stored,
            procedures_updated: self.procedures_updated,
            practice_sessions: self.practice_sessions,
            decay_operations: self.decay_operations,
            consolidations: self.consolidations,
        }
    }
}

/// System-level statistics
#[derive(Debug, Clone, Default)]
pub struct SystemStats {
    pub initializations: usize,
    pub retrievals: usize,
    pub stores: usize,
}

/// Memory search result
#[derive(Debug, Clone)]
pub struct MemorySearchResult {
    pub episodes: Vec<&'static Episode>,
    pub knowledge: Vec<&'static SemanticKnowledge>,
    pub procedures: Vec<&'static Procedure>,
}

impl MemorySearchResult {
    pub fn is_empty(&self) -> bool {
        self.episodes.is_empty() && self.knowledge.is_empty() && self.procedures.is_empty()
    }

    pub fn total(&self) -> usize {
        self.episodes.len() + self.knowledge.len() + self.procedures.len()
    }
}

/// Complete memory system statistics
#[derive(Debug, Clone)]
pub struct MemorySystemStats {
    pub episodic: MemoryStats,
    pub semantic: MemoryStats,
    pub procedural: MemoryStats,
    pub system: SystemStats,
}

/// Memory errors
#[derive(Debug, Clone)]
pub enum MemoryError {
    NotFound(MemoryId),
    CapacityExceeded,
    InvalidOperation(String),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::NotFound(id) => write!(f, "Memory not found: {}", id),
            MemoryError::CapacityExceeded => write!(f, "Memory capacity exceeded"),
            MemoryError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

impl std::error::Error for MemoryError {}
