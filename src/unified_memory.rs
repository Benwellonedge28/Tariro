// src/unified_memory.rs

//! Unified Memory Architecture for Tariro AGI
//! Integrates working memory, episodic memory, semantic memory, and procedural memory
//! into a cohesive memory system with attention-based retrieval

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Represents the unified memory system integrating all memory types
pub struct UnifiedMemory {
    pub working_memory: WorkingMemory,
    pub episodic_memory: EpisodicMemory,
    pub semantic_memory: SemanticMemory,
    pub procedural_memory: ProceduralMemory,
    pub attention_gate: AttentionGate,
    pub consolidation_scheduler: ConsolidationScheduler,
    pub memory_binding: MemoryBinding,
}

/// Working memory - short-term active information processing
pub struct WorkingMemory {
    pub capacity: usize,
    pub items: VecDeque<WorkingMemoryItem>,
    pub activation_levels: HashMap<String, f64>,
    pub focus_of_attention: Option<String>,
    pub binding_window: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryItem {
    pub id: String,
    pub content: MemoryContent,
    pub activation: f64,
    pub decay_rate: f64,
    pub source: MemorySource,
    pub associations: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub last_accessed: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContent {
    Perception(String),
    Thought(String),
    Goal(String),
    Plan(Vec<String>),
    Rule(String),
    Concept(String),
    Episode(MemoryEpisode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemorySource {
    Sensory,
    Reasoning,
    Learning,
    Dreaming,
    Introspection,
}

/// Episodic memory - autobiographical experiences
pub struct EpisodicMemory {
    pub episodes: Vec<Episode>,
    pub timeline_index: HashMap<DateTime<Utc>, Vec<String>>,
    pub emotion_index: HashMap<String, Vec<String>>,
    pub context_index: HashMap<String, Vec<String>>,
    pub autoassociative_network: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub location: Option<String>,
    pub participants: Vec<String>,
    pub events: Vec<EpisodeEvent>,
    pub emotions: Vec<EmotionTag>,
    pub significance: f64,
    pub retrievability: f64,
    pub consolidation_level: ConsolidationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeEvent {
    pub timestamp_offset: Duration,
    pub event_type: String,
    pub description: String,
    pub emotional_impact: f64,
    pub entities_involved: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionTag {
    pub emotion: String,
    pub intensity: f64,
    pub valence: f64,
}

/// Semantic memory - factual knowledge and concepts
pub struct SemanticMemory {
    pub concept_graph: ConceptGraph,
    pub knowledge_base: KnowledgeBase,
    pub ontological_structure: Ontology,
    pub schema_store: HashMap<String, Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptGraph {
    pub nodes: HashMap<String, ConceptNode>,
    pub edges: Vec<ConceptEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: String,
    pub name: String,
    pub definition: String,
    pub properties: HashMap<String, PropertyValue>,
    pub examples: Vec<String>,
    pub abstractions: Vec<String>,
    pub concreteness: f64,
    pub activation_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptEdge {
    pub from: String,
    pub to: String,
    pub relation: ConceptRelation,
    pub strength: f64,
    pub bidirectional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptRelation {
    IsA,
    HasA,
    PartOf,
    Causes,
    Enables,
    Contradicts,
    SimilarTo,
    RelatedTo,
    Precedes,
    Follows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub facts: HashMap<String, Fact>,
    pub rules: Vec<InferenceRule>,
    pub theories: Vec<Theory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub id: String,
    pub statement: String,
    pub confidence: f64,
    pub source: String,
    pub evidence: Vec<Evidence>,
    pub contradicted_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theory {
    pub id: String,
    pub name: String,
    pub axioms: Vec<String>,
    pub predictions: Vec<String>,
    pub supporting_evidence: Vec<String>,
    pub scope: String,
}

/// Procedural memory - skills and how-to knowledge
pub struct ProceduralMemory {
    pub skills: HashMap<String, Skill>,
    pub action_sequences: HashMap<String, ActionSequence>,
    pub habit_formation: HashMap<String, HabitStrength>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub steps: Vec<SkillStep>,
    pub conditions: Vec<Precondition>,
    pub practiced_count: u32,
    pub automaticity: f64,
    pub transferability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStep {
    pub step_number: u32,
    pub action: String,
    pub expected_outcome: String,
    pub feedback_cue: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSequence {
    pub id: String,
    pub trigger: String,
    pub steps: Vec<String>,
    pub success_rate: f64,
    pub context_requirements: Vec<String>,
}

/// Attention gate - controls what enters and persists in memory
pub struct AttentionGate {
    pub saliency_threshold: f64,
    pub novelty_weight: f64,
    pub relevance_weight: f64,
    pub emotional_weight: f64,
    pub patterns: Vec<AttentionPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionPattern {
    pub name: String,
    pub cue_type: String,
    pub target_content: String,
    pub priority_boost: f64,
}

/// Consolidation scheduler - manages memory consolidation
pub struct ConsolidationScheduler {
    pub sleep_cycle_simulation: bool,
    pub rehearsal_queue: Vec<RehearsalItem>,
    pub consolidation_threshold: f64,
    pub replay_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RehearsalItem {
    pub memory_id: String,
    pub priority: f64,
    pub next_review: DateTime<Utc>,
    pub repetition_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsolidationLevel {
    Labile,      // Easily modified
    Transitional, // Being consolidated
    Stabilized,   // Long-term memory
}

/// Memory binding - connects different memory types
pub struct MemoryBinding {
    pub cross_references: HashMap<String, Vec<MemoryLink>>,
    pub binding_signals: Vec<BindingSignal>,
    pub temporal_sequences: Vec<TemporalSequence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLink {
    pub from_id: String,
    pub from_type: MemoryType,
    pub to_id: String,
    pub to_type: MemoryType,
    pub link_type: LinkType,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Working,
    Episodic,
    Semantic,
    Procedural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkType {
    Association,
    Sequence,
    Causation,
    Similarity,
    Contrast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingSignal {
    pub signal_type: String,
    pub timestamp: DateTime<Utc>,
    pub linked_memory_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalSequence {
    pub id: String,
    pub events: Vec<String>,
    pub temporal_span: Duration,
    pub coherence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyValue {
    pub value_type: String,
    pub value: String,
    pub certainty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub slots: Vec<SchemaSlot>,
    pub default_values: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaSlot {
    pub name: String,
    pub value_type: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ontology {
    pub root_concepts: Vec<String>,
    pub hierarchies: HashMap<String, Vec<String>>,
    pub domain_boundaries: Vec<String>,
}

impl UnifiedMemory {
    pub fn new(capacity: usize) -> Self {
        UnifiedMemory {
            working_memory: WorkingMemory::new(capacity),
            episodic_memory: EpisodicMemory::new(),
            semantic_memory: SemanticMemory::new(),
            procedural_memory: ProceduralMemory::new(),
            attention_gate: AttentionGate::new(),
            consolidation_scheduler: ConsolidationScheduler::new(),
            memory_binding: MemoryBinding::new(),
        }
    }

    /// Process new input through attention gate
    pub fn process_input(&mut self, content: MemoryContent, source: MemorySource) -> Option<String> {
        let saliency = self.calculate_saliency(&content);

        if saliency < self.attention_gate.saliency_threshold {
            println!("[Memory] Content filtered by attention gate (saliency: {:.2})", saliency);
            return None;
        }

        // Create working memory item
        let item = WorkingMemoryItem {
            id: format!("wm_{}", Utc::now().timestamp_nanos()),
            content,
            activation: saliency,
            decay_rate: 0.1,
            source,
            associations: Vec::new(),
            timestamp: Utc::now(),
            last_accessed: Instant::now(),
        };

        let item_id = item.id.clone();
        self.working_memory.add(item);

        // Trigger episodic encoding if significant
        if saliency > 0.8 {
            self.encode_episode(&item_id, saliency);
        }

        // Update semantic memory if new concept
        self.integrate_semantic(&item_id);

        println!("[Memory] Processed input with saliency {:.2}", saliency);
        Some(item_id)
    }

    fn calculate_saliency(&self, content: &MemoryContent) -> f64 {
        let novelty = self.estimate_novelty(content);
        let relevance = self.estimate_relevance(content);
        let emotional = self.estimate_emotional_impact(content);

        novelty * self.attention_gate.novelty_weight
            + relevance * self.attention_gate.relevance_weight
            + emotional * self.attention_gate.emotional_weight
    }

    fn estimate_novelty(&self, content: &MemoryContent) -> f64 {
        // Check if similar content exists in memory
        match content {
            MemoryContent::Concept(name) => {
                if self.semantic_memory.concept_graph.nodes.contains_key(name) {
                    0.2
                } else {
                    0.9
                }
            }
            _ => 0.5 + rand_simple() * 0.3,
        }
    }

    fn estimate_relevance(&self, content: &MemoryContent) -> f64 {
        // Relevance based on current goals and focus
        let base = 0.5;
        if self.working_memory.focus_of_attention.is_some() {
            base + 0.3
        } else {
            base
        }
    }

    fn estimate_emotional_impact(&self, content: &MemoryContent) -> f64 {
        // Simplified emotional impact estimation
        match content {
            MemoryContent::Thought(t) => {
                if t.contains("!") || t.contains("urgent") || t.contains("important") {
                    0.8
                } else {
                    0.3
                }
            }
            _ => 0.4,
        }
    }

    fn encode_episode(&mut self, item_id: &str, significance: f64) {
        let episode = Episode {
            id: format!("ep_{}", Utc::now().timestamp()),
            start_time: Utc::now(),
            end_time: Utc::now(),
            location: None,
            participants: vec![],
            events: vec![EpisodeEvent {
                timestamp_offset: Duration::ZERO,
                event_type: "memory_encoding".to_string(),
                description: format!("Encoded: {}", item_id),
                emotional_impact: significance,
                entities_involved: vec![item_id.to_string()],
            }],
            emotions: vec![],
            significance,
            retrievability: significance,
            consolidation_level: ConsolidationLevel::Labile,
        };

        self.episodic_memory.add_episode(episode);
        println!("[Memory] Episode encoded with significance {:.2}", significance);
    }

    fn integrate_semantic(&mut self, _item_id: &str) {
        // Integrate new information into semantic memory
        // This would involve concept formation and schema updating
    }

    /// Retrieve from memory based on query
    pub fn retrieve(&self, query: &str, memory_types: Vec<MemoryType>) -> Vec<RetrievalResult> {
        let mut results = Vec::new();

        for mem_type in memory_types {
            match mem_type {
                MemoryType::Working => {
                    for item in &self.working_memory.items {
                        if format!("{:?}", item.content).contains(query) {
                            results.push(RetrievalResult {
                                memory_id: item.id.clone(),
                                memory_type: MemoryType::Working,
                                content: format!("{:?}", item.content),
                                relevance: item.activation,
                                context: "active".to_string(),
                            });
                        }
                    }
                }
                MemoryType::Episodic => {
                    for episode in &self.episodic_memory.episodes {
                        if episode.events.iter().any(|e| e.description.contains(query)) {
                            results.push(RetrievalResult {
                                memory_id: episode.id.clone(),
                                memory_type: MemoryType::Episodic,
                                content: format!("Episode: {}", episode.id),
                                relevance: episode.significance,
                                context: episode.start_time.to_rfc3339(),
                            });
                        }
                    }
                }
                MemoryType::Semantic => {
                    for (id, concept) in &self.semantic_memory.concept_graph.nodes {
                        if concept.name.contains(query) || concept.definition.contains(query) {
                            results.push(RetrievalResult {
                                memory_id: id.clone(),
                                memory_type: MemoryType::Semantic,
                                content: concept.definition.clone(),
                                relevance: 0.7,
                                context: "knowledge".to_string(),
                            });
                        }
                    }
                }
                MemoryType::Procedural => {
                    for (id, skill) in &self.procedural_memory.skills {
                        if skill.name.contains(query) {
                            results.push(RetrievalResult {
                                memory_id: id.clone(),
                                memory_type: MemoryType::Procedural,
                                content: skill.name.clone(),
                                relevance: skill.automaticity,
                                context: "skill".to_string(),
                            });
                        }
                    }
                }
            }
        }

        // Sort by relevance
        results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
        results
    }

    /// Consolidate memories during rest/sleep simulation
    pub fn consolidate(&mut self) -> ConsolidationReport {
        let mut report = ConsolidationReport::default();

        // Consolidate episodic to semantic
        for episode in &mut self.episodic_memory.episodes {
            if episode.consolidation_level == ConsolidationLevel::Labile {
                if episode.retrievability > 0.7 && episode.significance > 0.5 {
                    episode.consolidation_level = ConsolidationLevel::Stabilized;
                    report.episodes_consolidated += 1;
                }
            }
        }

        // Replay important memories
        for item in &self.working_memory.items {
            if item.activation > 0.8 {
                report.replayed_memories.push(item.id.clone());
            }
        }

        // Update retrievability scores
        for episode in &mut self.episodic_memory.episodes {
            episode.retrievability *= 0.99; // Slow decay
        }

        report.successful = true;
        println!("[Memory] Consolidation complete: {} episodes, {} replays",
                 report.episodes_consolidated, report.replayed_memories.len());
        report
    }

    /// Update focus of attention
    pub fn focus_on(&mut self, memory_id: &str) {
        self.working_memory.focus_of_attention = Some(memory_id.to_string());

        // Boost activation of associated items
        if let Some(item) = self.working_memory.items.iter_mut()
            .find(|i| i.id == memory_id)
        {
            item.activation = 1.0;
            item.last_accessed = Instant::now();
        }

        // Update bindings
        for link in &self.memory_binding.cross_references.get(memory_id).cloned().unwrap_or_default() {
            if let Some(item) = self.working_memory.items.iter_mut()
                .find(|i| i.id == link.to_id)
            {
                item.activation = (item.activation + 0.2).min(1.0);
            }
        }

        println!("[Memory] Focus shifted to: {}", memory_id);
    }

    /// Create memory binding
    pub fn bind(&mut self, from_id: &str, from_type: MemoryType, to_id: &str, to_type: MemoryType, link_type: LinkType) {
        let link = MemoryLink {
            from_id: from_id.to_string(),
            from_type,
            to_id: to_id.to_string(),
            to_type,
            link_type,
            strength: 0.5,
        };

        self.memory_binding.cross_references
            .entry(from_id.to_string())
            .or_insert_with(Vec::new)
            .push(link.clone());

        // Bidirectional if strong association
        if matches!(link_type, LinkType::Association | LinkType::Similarity) {
            let reverse_link = MemoryLink {
                from_id: to_id.to_string(),
                from_type: to_type,
                to_id: from_id.to_string(),
                to_type: from_type,
                link_type,
                strength: 0.5,
            };
            self.memory_binding.cross_references
                .entry(to_id.to_string())
                .or_insert_with(Vec::new)
                .push(reverse_link);
        }

        println!("[Memory] Created binding: {} -> {}", from_id, to_id);
    }

    /// Learn a new skill
    pub fn learn_skill(&mut self, name: &str, domain: &str, steps: Vec<SkillStep>) -> String {
        let skill_id = format!("skill_{}", Utc::now().timestamp());

        let skill = Skill {
            id: skill_id.clone(),
            name: name.to_string(),
            domain: domain.to_string(),
            steps,
            conditions: Vec::new(),
            practiced_count: 0,
            automaticity: 0.1,
            transferability: 0.3,
        };

        self.procedural_memory.skills.insert(skill_id.clone(), skill);
        println!("[Memory] Learned new skill: {} in {}", name, domain);
        skill_id
    }

    /// Practice skill to increase automaticity
    pub fn practice_skill(&mut self, skill_id: &str) {
        if let Some(skill) = self.procedural_memory.skills.get_mut(skill_id) {
            skill.practiced_count += 1;
            skill.automaticity = (skill.automaticity + 0.1).min(1.0);
            println!("[Memory] Practiced skill {}: automaticity now {:.2}", skill.name, skill.automaticity);
        }
    }
}

impl WorkingMemory {
    pub fn new(capacity: usize) -> Self {
        WorkingMemory {
            capacity,
            items: VecDeque::new(),
            activation_levels: HashMap::new(),
            focus_of_attention: None,
            binding_window: Duration::from_secs(30),
        }
    }

    pub fn add(&mut self, item: WorkingMemoryItem) {
        if self.items.len() >= self.capacity {
            // Remove lowest activation item
            if let Some(pos) = self.items.iter()
                .position(|i| i.activation == self.items.iter().map(|x| x.activation).fold(f64::INFINITY, f64::min))
            {
                self.items.remove(pos);
            }
        }
        self.items.push_back(item);
    }

    pub fn decay(&mut self, elapsed: Duration) {
        for item in &mut self.items {
            let seconds = elapsed.as_secs_f64();
            item.activation *= (-item.decay_rate * seconds).exp();
            item.activation = item.activation.max(0.01);
        }
    }
}

impl EpisodicMemory {
    pub fn new() -> Self {
        EpisodicMemory {
            episodes: Vec::new(),
            timeline_index: HashMap::new(),
            emotion_index: HashMap::new(),
            context_index: HashMap::new(),
            autoassociative_network: HashMap::new(),
        }
    }

    pub fn add_episode(&mut self, episode: Episode) {
        let episode_id = episode.id.clone();
        self.episodes.push(episode);

        // Index by time
        self.timeline_index
            .entry(episode.start_time)
            .or_insert_with(Vec::new)
            .push(episode_id.clone());
    }
}

impl SemanticMemory {
    pub fn new() -> Self {
        SemanticMemory {
            concept_graph: ConceptGraph {
                nodes: HashMap::new(),
                edges: Vec::new(),
            },
            knowledge_base: KnowledgeBase {
                facts: HashMap::new(),
                rules: Vec::new(),
                theories: Vec::new(),
            },
            ontological_structure: Ontology {
                root_concepts: vec!["Entity".to_string(), "Event".to_string(), "Concept".to_string()],
                hierarchies: HashMap::new(),
                domain_boundaries: Vec::new(),
            },
            schema_store: HashMap::new(),
        }
    }

    pub fn add_concept(&mut self, name: &str, definition: &str) -> String {
        let id = format!("concept_{}", Utc::now().timestamp());
        self.concept_graph.nodes.insert(id.clone(), ConceptNode {
            id: id.clone(),
            name: name.to_string(),
            definition: definition.to_string(),
            properties: HashMap::new(),
            examples: Vec::new(),
            abstractions: Vec::new(),
            concreteness: 0.5,
            activation_threshold: 0.3,
        });
        id
    }
}

impl ProceduralMemory {
    pub fn new() -> Self {
        ProceduralMemory {
            skills: HashMap::new(),
            action_sequences: HashMap::new(),
            habit_formation: HashMap::new(),
        }
    }
}

impl AttentionGate {
    pub fn new() -> Self {
        AttentionGate {
            saliency_threshold: 0.3,
            novelty_weight: 0.4,
            relevance_weight: 0.4,
            emotional_weight: 0.2,
            patterns: Vec::new(),
        }
    }
}

impl ConsolidationScheduler {
    pub fn new() -> Self {
        ConsolidationScheduler {
            sleep_cycle_simulation: true,
            rehearsal_queue: Vec::new(),
            consolidation_threshold: 0.7,
            replay_probability: 0.3,
        }
    }
}

impl MemoryBinding {
    pub fn new() -> Self {
        MemoryBinding {
            cross_references: HashMap::new(),
            binding_signals: Vec::new(),
            temporal_sequences: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievalResult {
    pub memory_id: String,
    pub memory_type: MemoryType,
    pub content: String,
    pub relevance: f64,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationReport {
    pub episodes_consolidated: usize,
    pub replayed_memories: Vec<String>,
    pub successful: bool,
}

impl Default for ConsolidationReport {
    fn default() -> Self {
        ConsolidationReport {
            episodes_consolidated: 0,
            replayed_memories: Vec::new(),
            successful: false,
        }
    }
}

impl Default for UnifiedMemory {
    fn default() -> Self {
        Self::new(7) // Miller's magic number
    }
}

fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 / u32::MAX as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_creation() {
        let memory = UnifiedMemory::new(7);
        assert_eq!(memory.working_memory.capacity, 7);
    }

    #[test]
    fn test_input_processing() {
        let mut memory = UnifiedMemory::new(7);
        let result = memory.process_input(
            MemoryContent::Thought("Important discovery".to_string()),
            MemorySource::Reasoning
        );
        assert!(result.is_some());
    }

    #[test]
    fn test_retrieval() {
        let memory = UnifiedMemory::new(7);
        let results = memory.retrieve("test", vec![MemoryType::Working]);
        assert!(results.is_empty());
    }

    #[test]
    fn test_episodic_encoding() {
        let mut memory = UnifiedMemory::new(7);
        memory.process_input(
            MemoryContent::Thought("Significant event".to_string()),
            MemorySource::Reasoning
        );
        assert!(!memory.episodic_memory.episodes.is_empty());
    }
}
