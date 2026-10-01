// Tariro AGI - Semantic Memory Module
// Knowledge graph-based semantic memory system

use crate::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::cmp::Ordering;

// ============================================================================
// Semantic Memory Types
// ============================================================================

/// A semantic memory representing factual knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemory {
    pub concept_id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub category: ConceptCategory,

    // Properties
    pub attributes: HashMap<String, Attribute>,
    pub relations: Vec<ConceptRelation>,

    // Knowledge
    pub facts: Vec<Fact>,
    pub rules: Vec<InferenceRule>,

    // Confidence and provenance
    pub confidence: f32,
    pub source: String,
    pub evidence: Vec<Evidence>,
    pub learned_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,

    // Activation
    pub activation_level: f32,
    pub access_frequency: usize,
    pub spread_activation: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
    pub value_type: ValueType,
    pub confidence: f32,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
    ConceptRef(String),
    Structured(HashMap<String, String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValueType {
    String,
    Number,
    Boolean,
    List,
    Concept,
    Structured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    pub relation_type: RelationType,
    pub target_id: String,
    pub target_name: String,
    pub strength: f32,
    pub bidirectional: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    IsA,           // Inheritance
    PartOf,        // Composition
    HasProperty,   // Attribution
    Causes,        // Causation
    Enables,       // Capability
    Opposite,      // Antonymy
    Similar,       // Similarity
    Before,        // Temporal
    LocatedIn,     // Spatial
    ProducedBy,     // Production
    MemberOf,       // Membership
    Associated,     // General association
    Correlates,    // Statistical correlation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub fact_id: String,
    pub statement: String,
    pub confidence: f32,
    pub source: String,
    pub verified: bool,
    pub contradictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRule {
    pub rule_id: String,
    pub name: String,
    pub antecedents: Vec<Condition>,
    pub consequent: Effect,
    pub confidence: f32,
    pub applicability: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub concept_id: String,
    pub relation: RelationType,
    pub value: String,
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub concept_id: String,
    pub relation: RelationType,
    pub value: String,
    pub certainty: Certainty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Certainty {
    Definite,
    Probable,
    Possible,
    Uncertain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_id: String,
    pub evidence_type: EvidenceType,
    pub content: String,
    pub weight: f32,
    pub source: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    Direct,
    Inferred,
    Testimony,
    Statistical,
    Logical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConceptCategory {
    Entity,
    Event,
    Action,
    Property,
    Relation,
    Abstract,
    Process,
    Theory,
    Person,
    Place,
    Organization,
    Concept,
}

// ============================================================================
// Knowledge Graph
// ============================================================================

/// Semantic memory knowledge graph
pub struct SemanticMemoryStore {
    // Core storage
    concepts: HashMap<String, SemanticMemory>,

    // Indices
    name_index: HashMap<String, String>,
    category_index: HashMap<ConceptCategory, Vec<String>>,
    relation_index: HashMap<String, Vec<(String, String)>>,  // (source, target)

    // Graph structure
    adjacency_list: HashMap<String, HashSet<String>>,
    reverse_adjacency: HashMap<String, HashSet<String>>,

    // Spreading activation
    activation_cache: HashMap<String, f32>,

    // Configuration
    config: SemanticConfig,

    // Statistics
    stats: SemanticStats,
}

#[derive(Debug, Clone)]
pub struct SemanticConfig {
    pub max_concepts: usize,
    pub default_confidence: f32,
    pub spreading_iterations: usize,
    pub activation_threshold: f32,
    pub decay_rate: f32,
    pub novelty_bonus: f32,
}

impl Default for SemanticConfig {
    fn default() -> Self {
        Self {
            max_concepts: 500000,
            default_confidence: 0.5,
            spreading_iterations: 3,
            activation_threshold: 0.1,
            decay_rate: 0.95,
            novelty_bonus: 0.2,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SemanticStats {
    pub total_concepts: usize,
    pub total_relations: usize,
    pub total_facts: usize,
    pub categories: HashMap<String, usize>,
    pub average_confidence: f32,
}

// ============================================================================
// Priority Queue Types
// ============================================================================

#[derive(Clone)]
struct PrioritizedConcept {
    concept: SemanticMemory,
    priority: f32,
}

impl Ord for PrioritizedConcept {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.partial_cmp(&other.priority).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PrioritizedConcept {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PrioritizedConcept {}
impl PartialEq for PrioritizedConcept {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

// ============================================================================
// Implementation
// ============================================================================

impl SemanticMemoryStore {
    pub fn new() -> Self {
        Self {
            concepts: HashMap::new(),
            name_index: HashMap::new(),
            category_index: HashMap::new(),
            relation_index: HashMap::new(),
            adjacency_list: HashMap::new(),
            reverse_adjacency: HashMap::new(),
            activation_cache: HashMap::new(),
            config: SemanticConfig::default(),
            stats: SemanticStats::default(),
        }
    }

    /// Store a new concept
    pub fn store_concept(&mut self, concept: SemanticMemory) -> String {
        let concept_id = concept.concept_id.clone();

        // Check capacity
        if self.concepts.len() >= self.config.max_concepts {
            self.evict_low_activation();
        }

        // Store concept
        self.concepts.insert(concept_id.clone(), concept.clone());

        // Update name index
        self.name_index.insert(concept.name.clone(), concept_id.clone());
        for alias in &concept.aliases {
            self.name_index.insert(alias.clone(), concept_id.clone());
        }

        // Update category index
        self.category_index.entry(concept.category)
            .or_insert_with(Vec::new)
            .push(concept_id.clone());

        // Update relation indices
        self.update_relation_indices(&concept);

        // Update adjacency lists
        self.update_adjacency_lists(&concept);

        // Update statistics
        self.stats.total_concepts += 1;
        *self.stats.categories.entry(format!("{:?}", concept.category)).or_insert(0) += 1;

        concept_id
    }

    /// Update relation indices
    fn update_relation_indices(&mut self, concept: &SemanticMemory) {
        for relation in &concept.relations {
            let key = format!("{:?}_{}", relation.relation_type, relation.target_id);

            self.relation_index.entry(key)
                .or_insert_with(Vec::new)
                .push((concept.concept_id.clone(), relation.target_id.clone()));

            if relation.bidirectional {
                let rev_key = format!("{:?}_{}", relation.relation_type, concept.concept_id);
                self.relation_index.entry(rev_key)
                    .or_insert_with(Vec::new)
                    .push((relation.target_id.clone(), concept.concept_id.clone()));
            }
        }
    }

    /// Update adjacency lists
    fn update_adjacency_lists(&mut self, concept: &SemanticMemory) {
        self.adjacency_list.entry(concept.concept_id.clone())
            .or_insert_with(HashSet::new);

        for relation in &concept.relations {
            self.adjacency_list.entry(concept.concept_id.clone())
                .or_default()
                .insert(relation.target_id.clone());

            self.reverse_adjacency.entry(relation.target_id.clone())
                .or_insert_with(HashSet::new)
                .insert(concept.concept_id.clone());
        }

        self.stats.total_relations += concept.relations.len();
    }

    /// Retrieve concept by ID
    pub fn get_concept(&mut self, concept_id: &str) -> Option<&SemanticMemory> {
        // Boost activation on access
        if let Some(concept) = self.concepts.get_mut(concept_id) {
            concept.access_frequency += 1;
            concept.activation_level = (concept.activation_level + 0.1).min(1.0);
        }

        self.concepts.get(concept_id)
    }

    /// Retrieve concept by name
    pub fn get_by_name(&self, name: &str) -> Option<&SemanticMemory> {
        self.name_index.get(name)
            .and_then(|id| self.concepts.get(id))
    }

    /// Query concepts by category
    pub fn get_by_category(&self, category: ConceptCategory) -> Vec<&SemanticMemory> {
        self.category_index.get(&category)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.concepts.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Retrieve concepts using spreading activation
    pub fn retrieve(&mut self, query: &ConceptQuery) -> Vec<SemanticMemory> {
        // Initialize activation
        let mut activation: HashMap<String, f32> = HashMap::new();

        // Seed with query concepts
        for seed_id in &query.seed_concepts {
            if let Some(concept) = self.concepts.get(seed_id) {
                activation.insert(seed_id.clone(), 1.0);
            }
        }

        // Also seed by name matches
        if let Some(ref name_pattern) = query.name_pattern {
            for (name, id) in &self.name_index {
                if name.to_lowercase().contains(&name_pattern.to_lowercase()) {
                    activation.insert(id.clone(), 0.8);
                }
            }
        }

        // Spreading activation iterations
        for _ in 0..self.config.spreading_iterations {
            let new_activation: HashMap<String, f32> = HashMap::new();

            for (concept_id, current_activation) in &activation {
                if let Some(concept) = self.concepts.get(concept_id) {
                    // Propagate through relations
                    for relation in &concept.relations {
                        let propagation = current_activation * relation.strength * 0.5;

                        let current = new_activation.entry(relation.target_id.clone())
                            .or_insert(0.0);
                        *current = (*current + propagation).min(1.0);

                        // Bidirectional
                        if relation.bidirectional {
                            let rev = new_activation.entry(concept_id.clone())
                                .or_insert(0.0);
                            *rev = (*rev + propagation).min(1.0);
                        }
                    }

                    // Propagate through reverse relations
                    if let Some(predecessors) = self.reverse_adjacency.get(concept_id) {
                        for pred_id in predecessors {
                            if let Some(pred) = self.concepts.get(pred_id) {
                                // Find relation strength
                                let strength = pred.relations.iter()
                                    .find(|r| r.target_id == *concept_id)
                                    .map(|r| r.strength)
                                    .unwrap_or(0.5);

                                let propagation = current_activation * strength * 0.3;
                                let current = new_activation.entry(pred_id.clone())
                                    .or_insert(0.0);
                                *current = (*current + propagation).min(1.0);
                            }
                        }
                    }
                }
            }

            // Merge new activation
            for (id, act) in new_activation {
                let current = activation.entry(id).or_insert(0.0);
                *current = (*current + act * 0.5).min(1.0);
            }
        }

        // Build result heap
        let mut results: BinaryHeap<PrioritizedConcept> = BinaryHeap::new();

        for (concept_id, activation_level) in activation {
            if activation_level >= self.config.activation_threshold {
                if let Some(concept) = self.concepts.get(&concept_id) {
                    // Calculate final score
                    let mut score = activation_level * 0.4;
                    score += concept.confidence * 0.3;
                    score += (concept.access_frequency as f32 / 100.0).min(0.3);

                    // Apply filters
                    if let Some(ref category) = query.category_filter {
                        if concept.category != *category {
                            continue;
                        }
                    }

                    if let Some(min_confidence) = query.min_confidence {
                        if concept.confidence < min_confidence {
                            continue;
                        }
                    }

                    results.push(PrioritizedConcept {
                        concept: concept.clone(),
                        priority: score,
                    });
                }
            }
        }

        // Extract top results
        let limit = query.limit.unwrap_or(10);
        let mut output = Vec::new();

        while let Some(pc) = results.pop() {
            if output.len() >= limit {
                break;
            }
            output.push(pc.concept);
        }

        output
    }

    /// Add relation between concepts
    pub fn add_relation(&mut self, source_id: &str, target_id: &str,
                        relation_type: RelationType, strength: f32) -> bool {
        if let Some(source) = self.concepts.get_mut(source_id) {
            // Check if relation exists
            if source.relations.iter().any(|r| r.target_id == target_id && r.relation_type == relation_type) {
                return false;
            }

            let target_name = self.concepts.get(target_id)
                .map(|c| c.name.clone())
                .unwrap_or_default();

            source.relations.push(ConceptRelation {
                relation_type,
                target_id: target_id.to_string(),
                target_name,
                strength,
                bidirectional: false,
            });

            // Update indices
            self.update_relation_indices(source);
            self.update_adjacency_lists(source);

            // Create reverse relation
            if let Some(target) = self.concepts.get_mut(target_id) {
                let reverse_type = Self::get_reverse_relation(relation_type);
                target.relations.push(ConceptRelation {
                    relation_type: reverse_type,
                    target_id: source_id.to_string(),
                    target_name: source.name.clone(),
                    strength,
                    bidirectional: false,
                });
                self.update_relation_indices(target);
                self.update_adjacency_lists(target);
            }

            self.stats.total_relations += 2;
            true
        } else {
            false
        }
    }

    /// Get reverse relation type
    fn get_reverse_relation(relation: RelationType) -> RelationType {
        match relation {
            RelationType::IsA => RelationType::PartOf,
            RelationType::PartOf => RelationType::IsA,
            RelationType::HasProperty => RelationType::ProducedBy,
            RelationType::Causes => RelationType::Before,
            RelationType::Enables => RelationType::MemberOf,
            RelationType::Opposite => RelationType::Opposite,
            RelationType::Similar => RelationType::Similar,
            RelationType::Before => RelationType::Causes,
            RelationType::LocatedIn => RelationType::PartOf,
            RelationType::ProducedBy => RelationType::HasProperty,
            RelationType::MemberOf => RelationType::Enables,
            RelationType::Associated => RelationType::Associated,
            RelationType::Correlates => RelationType::Correlates,
        }
    }

    /// Perform inference
    pub fn infer(&self, concept_id: &str, target_relation: RelationType) -> Vec<InferenceResult> {
        let mut results = Vec::new();

        if let Some(concept) = self.concepts.get(concept_id) {
            // Direct relations
            for relation in &concept.relations {
                if relation.relation_type == target_relation {
                    results.push(InferenceResult {
                        target_id: relation.target_id.clone(),
                        confidence: relation.strength * concept.confidence,
                        path: vec![concept.concept_id.clone(), relation.target_id.clone()],
                    });
                }
            }

            // Multi-hop inference through rules
            for rule in &concept.rules {
                if self.matches_rule(rule, concept) {
                    results.push(InferenceResult {
                        target_id: rule.consequent.concept_id.clone(),
                        confidence: rule.confidence * concept.confidence,
                        path: vec![concept.concept_id.clone()],
                    });
                }
            }
        }

        results
    }

    /// Check if inference rule matches
    fn matches_rule(&self, rule: &InferenceRule, concept: &SemanticMemory) -> bool {
        for condition in &rule.antecedents {
            if condition.concept_id == concept.concept_id {
                continue;
            }

            if let Some(condition_concept) = self.concepts.get(&condition.concept_id) {
                let has_relation = condition_concept.relations.iter()
                    .any(|r| r.relation_type == condition.relation && r.target_id == condition.value);

                if !has_relation {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Get concept categories
    pub fn get_categories(&self) -> Vec<ConceptCategory> {
        self.category_index.keys().cloned().collect()
    }

    /// Get statistics
    pub fn get_stats(&self) -> &SemanticStats {
        &self.stats
    }

    /// Evict low activation concepts
    fn evict_low_activation(&mut self) {
        let mut min_concept: Option<String> = None;
        let mut min_activation = f32::MAX;

        for (id, concept) in &self.concepts {
            let effective_activation = concept.activation_level *
                (1.0 - concept.access_frequency as f32 / 1000.0);

            if effective_activation < min_activation {
                min_activation = effective_activation;
                min_concept = Some(id.clone());
            }
        }

        if let Some(id) = min_concept {
            self.remove_concept(&id);
        }
    }

    /// Remove a concept
    fn remove_concept(&mut self, concept_id: &str) {
        if let Some(removed) = self.concepts.remove(concept_id) {
            // Remove from name index
            self.name_index.remove(&removed.name);
            for alias in &removed.aliases {
                self.name_index.remove(alias);
            }

            // Remove from category index
            if let Some(ids) = self.category_index.get_mut(&removed.category) {
                ids.retain(|id| id != concept_id);
            }

            // Update adjacency
            self.adjacency_list.remove(concept_id);
            if let Some(predecessors) = self.reverse_adjacency.remove(concept_id) {
                for pred_id in predecessors {
                    if let Some(pred) = self.concepts.get_mut(&pred_id) {
                        pred.relations.retain(|r| r.target_id != concept_id);
                    }
                    self.adjacency_list.get_mut(&pred_id)
                        .map(|set| set.remove(concept_id));
                }
            }

            self.stats.total_concepts -= 1;
        }
    }

    /// Merge duplicate concepts
    pub fn merge_concepts(&mut self, source_id: &str, target_id: &str) -> bool {
        if let (Some(source), Some(target)) = (
            self.concepts.get(source_id),
            self.concepts.get_mut(target_id)
        ) {
            // Merge attributes
            for (key, attr) in &source.attributes {
                if !target.attributes.contains_key(key) {
                    target.attributes.insert(key.clone(), attr.clone());
                }
            }

            // Merge relations (avoiding duplicates)
            for relation in &source.relations {
                if !target.relations.iter().any(|r| r.target_id == relation.target_id) {
                    target.relations.push(relation.clone());
                }
            }

            // Merge facts
            for fact in &source.facts {
                if !target.facts.iter().any(|f| f.statement == fact.statement) {
                    target.facts.push(fact.clone());
                }
            }

            // Update confidence
            target.confidence = (target.confidence + source.confidence) / 2.0;

            // Remove source
            self.remove_concept(source_id);

            true
        } else {
            false
        }
    }
}

/// Query for semantic retrieval
#[derive(Debug, Clone, Default)]
pub struct ConceptQuery {
    pub seed_concepts: Vec<String>,
    pub name_pattern: Option<String>,
    pub category_filter: Option<ConceptCategory>,
    pub min_confidence: Option<f32>,
    pub required_relations: Vec<RelationType>,
    pub limit: Option<usize>,
}

/// Inference result
#[derive(Debug, Clone)]
pub struct InferenceResult {
    pub target_id: String,
    pub confidence: f32,
    pub path: Vec<String>,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("sem_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concept_storage() {
        let mut store = SemanticMemoryStore::new();

        let concept = SemanticMemory {
            concept_id: generate_id(),
            name: "Machine Learning".to_string(),
            aliases: vec!["ML".to_string(), "Statistical Learning".to_string()],
            category: ConceptCategory::Theory,
            attributes: HashMap::new(),
            relations: vec![],
            facts: vec![],
            rules: vec![],
            confidence: 0.9,
            source: "textbook".to_string(),
            evidence: vec![],
            learned_at: Utc::now(),
            last_updated: Utc::now(),
            activation_level: 1.0,
            access_frequency: 0,
            spread_activation: 0.0,
        };

        let id = store.store_concept(concept);
        assert!(!id.is_empty());
        assert_eq!(store.stats.total_concepts, 1);
    }

    #[test]
    fn test_retrieval_by_name() {
        let mut store = SemanticMemoryStore::new();

        let concept = SemanticMemory {
            concept_id: generate_id(),
            name: "Neural Network".to_string(),
            aliases: vec!["NN".to_string()],
            category: ConceptCategory::Theory,
            attributes: HashMap::new(),
            relations: vec![],
            facts: vec![],
            rules: vec![],
            confidence: 0.95,
            source: "research".to_string(),
            evidence: vec![],
            learned_at: Utc::now(),
            last_updated: Utc::now(),
            activation_level: 1.0,
            access_frequency: 0,
            spread_activation: 0.0,
        };

        store.store_concept(concept);

        let retrieved = store.get_by_name("Neural Network");
        assert!(retrieved.is_some());
    }
}
