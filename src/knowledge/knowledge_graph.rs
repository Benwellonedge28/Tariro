//! Tariro Knowledge Graph - First-Class Explicit Knowledge
//!
//! Knowledge is explicitly represented, queried, reasoned over, and updated.
//! This is NOT emergent from token prediction - it is a first-class citizen.

use crate::prelude::*;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for entities
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityId(pub String);

impl EntityId {
    pub fn new(id: impl Into<String>) -> Self {
        EntityId(id.into())
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for relations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelationId(pub String);

impl RelationId {
    pub fn new(id: impl Into<String>) -> Self {
        RelationId(id.into())
    }
}

/// Type of entity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    Concrete,
    Abstract,
    Event,
    Process,
    Property,
    Concept,
    Agent,
    Custom(String),
}

/// An entity in the knowledge graph
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: EntityId,
    pub name: String,
    pub entity_type: EntityType,
    pub attributes: HashMap<Attribute, Value>,
    pub properties: Vec<Property>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

impl Entity {
    pub fn new(name: impl Into<String>, entity_type: EntityType) -> Self {
        let name = name.into();
        Entity {
            id: EntityId::new(name.clone()),
            name,
            entity_type,
            attributes: HashMap::new(),
            properties: Vec::new(),
            created_at: Utc::now(),
            last_modified: Utc::now(),
        }
    }

    pub fn with_attribute(mut self, attr: Attribute, value: Value) -> Self {
        self.attributes.insert(attr, value);
        self
    }

    pub fn get_attr(&self, attr: &Attribute) -> Option<&Value> {
        self.attributes.get(attr)
    }
}

/// Attribute of an entity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute(pub String);

impl Attribute {
    pub fn new(name: impl Into<String>) -> Self {
        Attribute(name.into())
    }
}

/// Value types for attributes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    DateTime(DateTime<Utc>),
    EntityRef(EntityId),
    List(Vec<Value>),
    Custom(String),
}

impl Value {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "String",
            Value::Integer(_) => "Integer",
            Value::Float(_) => "Float",
            Value::Boolean(_) => "Boolean",
            Value::DateTime(_) => "DateTime",
            Value::EntityRef(_) => "EntityRef",
            Value::List(_) => "List",
            Value::Custom(_) => "Custom",
        }
    }
}

/// A property of an entity
#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
    pub confidence: Probability,
}

impl Property {
    pub fn new(name: impl Into<String>, value: Value) -> Self {
        Property {
            name: name.into(),
            value,
            confidence: Probability::certain(),
        }
    }
}

// ============================================================================
// Predicates and Relations
// ============================================================================

/// A predicate represents a type of relation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Predicate {
    // Semantic relations
    IsA,
    PartOf,
    HasProperty,
    LocatedAt,
    SimilarTo,
    RelatedTo,

    // Causal relations
    Causes,
    Enables,
    Prevents,
    Produces,
    Consumes,

    // Temporal relations
    Before,
    After,
    During,
    Overlaps,
    StartsAt,
    EndsAt,

    // Functional relations
    HasCapability,
    HasGoal,
    Believes,
    Desires,

    // Custom predicate
    Custom(String),
}

impl Predicate {
    pub fn name(&self) -> &str {
        match self {
            Predicate::IsA => "is_a",
            Predicate::PartOf => "part_of",
            Predicate::HasProperty => "has_property",
            Predicate::LocatedAt => "located_at",
            Predicate::SimilarTo => "similar_to",
            Predicate::RelatedTo => "related_to",
            Predicate::Causes => "causes",
            Predicate::Enables => "enables",
            Predicate::Prevents => "prevents",
            Predicate::Produces => "produces",
            Predicate::Consumes => "consumes",
            Predicate::Before => "before",
            Predicate::After => "after",
            Predicate::During => "during",
            Predicate::Overlaps => "overlaps",
            Predicate::StartsAt => "starts_at",
            Predicate::EndsAt => "ends_at",
            Predicate::HasCapability => "has_capability",
            Predicate::HasGoal => "has_goal",
            Predicate::Believes => "believes",
            Predicate::Desires => "desires",
            Predicate::Custom(name) => name,
        }
    }

    pub fn is_causal(&self) -> bool {
        matches!(
            self,
            Predicate::Causes | Predicate::Enables | Predicate::Prevents
            | Predicate::Produces | Predicate::Consumes
        )
    }

    pub fn is_temporal(&self) -> bool {
        matches!(
            self,
            Predicate::Before | Predicate::After | Predicate::During
            | Predicate::Overlaps | Predicate::StartsAt | Predicate::EndsAt
        )
    }
}

/// A relation between entities
#[derive(Debug, Clone)]
pub struct Relation {
    pub id: RelationId,
    pub subject: EntityId,
    pub predicate: Predicate,
    pub object: EntityId,
    pub confidence: Probability,
    pub source: Evidence,
    pub temporal: Option<TimeInterval>,
}

impl Relation {
    pub fn new(subject: EntityId, predicate: Predicate, object: EntityId) -> Self {
        Relation {
            id: RelationId::new(format!("{:?}-{:?}-{:?}", subject, predicate.name(), object)),
            subject,
            predicate,
            object,
            confidence: Probability::certain(),
            source: Evidence::Inferred,
            temporal: None,
        }
    }

    pub fn with_confidence(mut self, prob: Probability) -> Self {
        self.confidence = prob;
        self
    }

    pub fn with_source(mut self, source: Evidence) -> Self {
        self.source = source;
        self
    }

    pub fn with_temporal(mut self, interval: TimeInterval) -> Self {
        self.temporal = Some(interval);
        self
    }
}

/// A fact is the basic unit of knowledge
#[derive(Debug, Clone)]
pub struct Fact {
    pub subject: Entity,
    pub predicate: Predicate,
    pub object: Entity,
    pub confidence: Probability,
    pub source: Evidence,
    pub temporal: Option<TimeInterval>,
}

impl Fact {
    pub fn new(subject: Entity, predicate: Predicate, object: Entity) -> Self {
        Fact {
            subject,
            predicate,
            object,
            confidence: Probability::certain(),
            source: Evidence::Inferred,
            temporal: None,
        }
    }

    pub fn is_causal(&self) -> bool {
        self.predicate.is_causal()
    }
}

// ============================================================================
// Probability and Uncertainty
// ============================================================================

/// Probability with confidence intervals
#[derive(Debug, Clone, Copy)]
pub struct Probability {
    pub value: f64,
    pub lower: f64,
    pub upper: f64,
}

impl Probability {
    pub fn new(value: f64, lower: f64, upper: f64) -> Self {
        Probability { value, lower, upper }
    }

    pub fn certain() -> Self {
        Probability { value: 1.0, lower: 1.0, upper: 1.0 }
    }

    pub fn impossible() -> Self {
        Probability { value: 0.0, lower: 0.0, upper: 0.0 }
    }

    pub fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(0.0, 1.0);
        Probability {
            value: clamped,
            lower: (clamped - 0.1).max(0.0),
            upper: (clamped + 0.1).min(1.0)
        }
    }

    pub fn is_certain(&self) -> bool {
        (self.value - 1.0).abs() < f64::EPSILON
    }

    pub fn is_impossible(&self) -> bool {
        self.value < f64::EPSILON
    }

    pub fn and(&self, other: &Probability) -> Probability {
        let value = self.value * other.value;
        Probability::from_f64(value)
    }

    pub fn or(&self, other: &Probability) -> Probability {
        let value = self.value + other.value - (self.value * other.value);
        Probability::from_f64(value)
    }

    pub fn not(&self) -> Probability {
        Probability::from_f64(1.0 - self.value)
    }
}

impl PartialOrd for Probability {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for Probability {
    fn eq(&self, other: &Self) -> bool {
        (self.value - other.value).abs() < f64::EPSILON
    }
}

/// Source of knowledge
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evidence {
    /// Direct observation
    Observation,
    /// Logical inference
    Inferred,
    /// From text/documents
    Document(String),
    /// From user input
    UserInput,
    /// Default assumption
    Assumption,
    /// Learned from data
    Learned,
}

/// Time interval for temporal knowledge
#[derive(Debug, Clone)]
pub struct TimeInterval {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
}

impl TimeInterval {
    pub fn new(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Self {
        let duration_seconds = match (start, end) {
            (Some(s), Some(e)) => Some((e - s).num_seconds()),
            _ => None,
        };
        TimeInterval { start, end, duration_seconds }
    }

    pub fn contains(&self, time: DateTime<Utc>) -> bool {
        let after_start = self.start.map_or(true, |s| time >= s);
        let before_end = self.end.map_or(true, |e| time <= e);
        after_start && before_end
    }

    pub fn overlaps(&self, other: &TimeInterval) -> bool {
        // Check if intervals overlap
        let self_start = self.start;
        let self_end = self.end;
        let other_start = other.start;
        let other_end = other.end;

        // Simplified: if both have start times, check ordering
        match (self_end, other_start) {
            (Some(se), Some(os)) if se <= os => return false,
            (None, _) => return false,
            _ => {}
        }

        match (other_end, self_start) {
            (Some(oe), Some(ss)) if oe <= ss => return false,
            (None, _) => return false,
            _ => {}
        }

        true
    }
}

// ============================================================================
// Main Knowledge Graph
// ============================================================================

/// The Tariro Knowledge Graph - explicit, first-class knowledge representation
pub struct KnowledgeGraph {
    /// Core entity storage
    entities: HashMap<EntityId, Entity>,

    /// Core relation storage
    relations: HashMap<RelationId, Relation>,

    /// Index: subject -> relations
    subject_index: HashMap<EntityId, Vec<RelationId>>,

    /// Index: predicate -> relations
    predicate_index: HashMap<Predicate, Vec<RelationId>>,

    /// Index: object -> relations
    object_index: HashMap<EntityId, Vec<RelationId>>,

    /// Causal edges for causal reasoning
    causal_edges: Vec<(EntityId, EntityId)>,

    /// Type hierarchy
    type_hierarchy: HashMap<EntityType, Vec<EntityType>>,

    /// Statistics
    stats: KnowledgeStats,
}

/// Statistics about the knowledge graph
#[derive(Debug, Clone, Default)]
pub struct KnowledgeStats {
    pub total_entities: usize,
    pub total_relations: usize,
    pub entity_types: HashMap<EntityType, usize>,
    pub predicate_counts: HashMap<String, usize>,
}

impl KnowledgeGraph {
    /// Create a new empty knowledge graph
    pub fn new() -> Self {
        let mut kg = KnowledgeGraph {
            entities: HashMap::new(),
            relations: HashMap::new(),
            subject_index: HashMap::new(),
            predicate_index: HashMap::new(),
            object_index: HashMap::new(),
            causal_edges: Vec::new(),
            type_hierarchy: HashMap::new(),
            stats: KnowledgeStats::default(),
        };

        // Initialize type hierarchy
        kg.init_type_hierarchy();
        kg
    }

    fn init_type_hierarchy(&mut self) {
        // Define basic type hierarchy
        self.type_hierarchy.insert(
            EntityType::Concrete,
            vec![EntityType::Agent, EntityType::Event],
        );
        self.type_hierarchy.insert(
            EntityType::Abstract,
            vec![EntityType::Concept, EntityType::Property],
        );
    }

    /// Assert a fact into the knowledge graph
    pub fn assert_fact(&mut self, fact: Fact) -> Result<(), KnowledgeError> {
        // Clone entities to insert
        let subject = fact.subject.clone();
        let object = fact.object.clone();
        let predicate = fact.predicate.clone();

        // Validate
        self.validate_fact(&fact)?;

        // Insert entities
        self.entities.insert(subject.id.clone(), subject);
        self.entities.insert(object.id.clone(), object);

        // Create relation
        let relation = Relation {
            id: RelationId::new(format!("{}-{:?}-{}", fact.subject.id, predicate, fact.object.id)),
            subject: fact.subject.id,
            predicate: predicate.clone(),
            object: fact.object.id,
            confidence: fact.confidence,
            source: fact.source,
            temporal: fact.temporal,
        };

        // Insert relation
        let relation_id = relation.id.clone();
        self.relations.insert(relation_id.clone(), relation);

        // Update indexes
        self.subject_index.entry(fact.subject.id).or_default().push(relation_id.clone());
        self.predicate_index.entry(predicate.clone()).or_default().push(relation_id.clone());
        self.object_index.entry(fact.object.id).or_default().push(relation_id.clone());

        // Track causal edges
        if predicate.is_causal() {
            self.causal_edges.push((fact.subject.id, fact.object.id));
        }

        // Update stats
        self.update_stats();

        Ok(())
    }

    /// Assert an entity
    pub fn assert_entity(&mut self, entity: Entity) -> Result<(), KnowledgeError> {
        let id = entity.id.clone();

        if self.entities.contains_key(&id) {
            return Err(KnowledgeError::EntityExists(id));
        }

        self.entities.insert(id, entity);
        self.stats.total_entities += 1;

        Ok(())
    }

    /// Assert a relation directly
    pub fn assert_relation(&mut self, relation: Relation) -> Result<(), KnowledgeError> {
        // Ensure entities exist
        if !self.entities.contains_key(&relation.subject) {
            return Err(KnowledgeError::EntityNotFound(relation.subject));
        }
        if !self.entities.contains_key(&relation.object) {
            return Err(KnowledgeError::EntityNotFound(relation.object));
        }

        let id = relation.id.clone();
        self.relations.insert(id.clone(), relation);

        // Update indexes
        self.subject_index.entry(relation.subject).or_default().push(id.clone());
        self.predicate_index.entry(relation.predicate.clone()).or_default().push(id.clone());
        self.object_index.entry(relation.object).or_default().push(id);

        // Track causal edges
        if relation.predicate.is_causal() {
            self.causal_edges.push((relation.subject, relation.object));
        }

        self.stats.total_relations += 1;

        Ok(())
    }

    /// Query: Get all relations for a subject
    pub fn get_subject_relations(&self, subject: &EntityId) -> Vec<&Relation> {
        self.subject_index
            .get(subject)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.relations.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Query: Get all relations with a predicate
    pub fn get_predicate_relations(&self, predicate: &Predicate) -> Vec<&Relation> {
        self.predicate_index
            .get(predicate)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.relations.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Query: Get all relations to an object
    pub fn get_object_relations(&self, object: &EntityId) -> Vec<&Relation> {
        self.object_index
            .get(object)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.relations.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Query: Find all entities of a type
    pub fn get_entities_by_type(&self, entity_type: &EntityType) -> Vec<&Entity> {
        self.entities
            .values()
            .filter(|e| &e.entity_type == entity_type)
            .collect()
    }

    /// Query: Get a specific entity
    pub fn get_entity(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id)
    }

    /// Query: Check if a relation exists
    pub fn has_relation(&self, subject: &EntityId, predicate: &Predicate, object: &EntityId) -> bool {
        self.relations.values().any(|r|
            r.subject == *subject && r.predicate == *predicate && r.object == *object
        )
    }

    /// Query: Get all causal predecessors (causes)
    pub fn get_causes(&self, effect: &EntityId) -> Vec<&Entity> {
        self.causal_edges
            .iter()
            .filter(|(_, o)| *o == *effect)
            .filter_map(|(s, _)| self.entities.get(s))
            .collect()
    }

    /// Query: Get all causal successors (effects)
    pub fn get_effects(&self, cause: &EntityId) -> Vec<&Entity> {
        self.causal_edges
            .iter()
            .filter(|(c, _)| *c == *cause)
            .filter_map(|(_, o)| self.entities.get(o))
            .collect()
    }

    /// Query: Find path between two entities
    pub fn find_path(&self, from: &EntityId, to: &EntityId) -> Option<Vec<EntityId>> {
        // BFS to find shortest path
        use std::collections::VecDeque;

        let mut queue: VecDeque<(EntityId, Vec<EntityId>)> = VecDeque::new();
        let mut visited: HashSet<EntityId> = HashSet::new();

        queue.push_back((from.clone(), vec![from.clone()]));
        visited.insert(from.clone());

        while let Some((current, path)) = queue.pop_front() {
            if current == *to {
                return Some(path);
            }

            // Get all related entities
            for rel in self.get_subject_relations(&current) {
                if !visited.contains(&rel.object) {
                    visited.insert(rel.object.clone());
                    let mut new_path = path.clone();
                    new_path.push(rel.object.clone());
                    queue.push_back((rel.object.clone(), new_path));
                }
            }
        }

        None
    }

    /// Retract (remove) a fact
    pub fn retract_fact(&mut self, subject: &EntityId, predicate: &Predicate, object: &EntityId) -> bool {
        // Find the relation
        let relation_id = self.relations
            .iter()
            .find(|(_, r)| r.subject == *subject && r.predicate == *predicate && r.object == *object)
            .map(|(id, _)| id.clone());

        if let Some(id) = relation_id {
            // Remove from indexes
            if let Some(ids) = self.subject_index.get_mut(subject) {
                ids.retain(|i| i != &id);
            }
            if let Some(ids) = self.predicate_index.get_mut(predicate) {
                ids.retain(|i| i != &id);
            }
            if let Some(ids) = self.object_index.get_mut(object) {
                ids.retain(|i| i != &id);
            }

            // Remove causal edge if applicable
            if predicate.is_causal() {
                self.causal_edges.retain(|(c, e)| !(c == subject && e == object));
            }

            // Remove relation
            self.relations.remove(&id);
            self.stats.total_relations = self.relations.len();

            true
        } else {
            false
        }
    }

    /// Update a fact's confidence
    pub fn update_confidence(&mut self, subject: &EntityId, predicate: &Predicate,
                            object: &EntityId, new_confidence: Probability) -> bool {
        let relation_id = self.relations
            .iter()
            .find(|(_, r)| r.subject == *subject && r.predicate == *predicate && r.object == *object)
            .map(|(id, _)| id.clone());

        if let Some(id) = relation_id {
            if let Some(relation) = self.relations.get_mut(&id) {
                relation.confidence = new_confidence;
                return true;
            }
        }

        false
    }

    /// Validate a fact before inserting
    fn validate_fact(&self, fact: &Fact) -> Result<(), KnowledgeError> {
        // Check confidence is valid
        if fact.confidence.value < 0.0 || fact.confidence.value > 1.0 {
            return Err(KnowledgeError::InvalidConfidence(fact.confidence.value));
        }

        // Check temporal consistency
        if let Some(temporal) = &fact.temporal {
            if let (Some(start), Some(end)) = (temporal.start, temporal.end) {
                if start > end {
                    return Err(KnowledgeError::InvalidTemporal(
                        "Start is after end".to_string()
                    ));
                }
            }
        }

        Ok(())
    }

    fn update_stats(&mut self) {
        self.stats.total_entities = self.entities.len();
        self.stats.total_relations = self.relations.len();

        // Count entity types
        self.stats.entity_types.clear();
        for entity in self.entities.values() {
            *self.stats.entity_types.entry(entity.entity_type.clone()).or_insert(0) += 1;
        }

        // Count predicates
        self.stats.predicate_counts.clear();
        for rel in self.relations.values() {
            *self.stats.predicate_counts.entry(rel.predicate.name().to_string()).or_insert(0) += 1;
        }
    }

    /// Get statistics
    pub fn stats(&self) -> &KnowledgeStats {
        &self.stats
    }

    /// Export to JSON for serialization
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Import from JSON
    pub fn from_json(json: &str) -> Result<Self, KnowledgeError> {
        serde_json::from_str(json)
            .map_err(|e| KnowledgeError::ParseError(e.to_string()))
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone)]
pub enum KnowledgeError {
    EntityExists(EntityId),
    EntityNotFound(EntityId),
    RelationExists(RelationId),
    RelationNotFound(RelationId),
    InvalidConfidence(f64),
    InvalidTemporal(String),
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for KnowledgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnowledgeError::EntityExists(id) => write!(f, "Entity already exists: {}", id),
            KnowledgeError::EntityNotFound(id) => write!(f, "Entity not found: {}", id),
            KnowledgeError::RelationExists(id) => write!(f, "Relation already exists: {}", id),
            KnowledgeError::RelationNotFound(id) => write!(f, "Relation not found: {}", id),
            KnowledgeError::InvalidConfidence(v) => write!(f, "Invalid confidence value: {}", v),
            KnowledgeError::InvalidTemporal(msg) => write!(f, "Invalid temporal: {}", msg),
            KnowledgeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            KnowledgeError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for KnowledgeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
        let entity = Entity::new("John", EntityType::Agent);
        assert_eq!(entity.name, "John");
        assert_eq!(entity.entity_type, EntityType::Agent);
    }

    #[test]
    fn test_create_relation() {
        let subject = Entity::new("Socrates", EntityType::Agent);
        let object = Entity::new("Human", EntityType::Concept);
        let relation = Relation::new(subject.id.clone(), Predicate::IsA, object.id.clone());

        assert_eq!(relation.subject, subject.id);
        assert_eq!(relation.object, object.id);
    }

    #[test]
    fn test_knowledge_graph_insert() {
        let mut kg = KnowledgeGraph::new();

        let subject = Entity::new("Bird", EntityType::Concept);
        let object = Entity::new("Animal", EntityType::Concept);

        let fact = Fact::new(subject, Predicate::IsA, object);
        let result = kg.assert_fact(fact);

        assert!(result.is_ok());
        assert_eq!(kg.stats().total_entities, 2);
        assert_eq!(kg.stats().total_relations, 1);
    }

    #[test]
    fn test_query_relations() {
        let mut kg = KnowledgeGraph::new();

        let socrates = Entity::new("Socrates", EntityType::Agent);
        let human = Entity::new("Human", EntityType::Concept);
        let mortal = Entity::new("Mortal", EntityType::Concept);

        kg.assert_fact(Fact::new(socrates.clone(), Predicate::IsA, human.clone())).unwrap();
        kg.assert_fact(Fact::new(human.clone(), Predicate::IsA, mortal.clone())).unwrap();

        // Query
        let human_relations = kg.get_subject_relations(&human.id);
        assert_eq!(human_relations.len(), 1);

        // Check causal (IsA is not causal)
        let causes = kg.get_causes(&human.id);
        assert!(causes.is_empty());
    }

    #[test]
    fn test_causal_relations() {
        let mut kg = KnowledgeGraph::new();

        let fire = Entity::new("Fire", EntityType::Concrete);
        let smoke = Entity::new("Smoke", EntityType::Event);

        let fact = Fact::new(fire.clone(), Predicate::Causes, smoke.clone());
        kg.assert_fact(fact).unwrap();

        let effects = kg.get_effects(&fire.id);
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].name, "Smoke");

        let causes = kg.get_causes(&smoke.id);
        assert_eq!(causes.len(), 1);
        assert_eq!(causes[0].name, "Fire");
    }

    #[test]
    fn test_path_finding() {
        let mut kg = KnowledgeGraph::new();

        let a = Entity::new("A", EntityType::Concept);
        let b = Entity::new("B", EntityType::Concept);
        let c = Entity::new("C", EntityType::Concept);
        let d = Entity::new("D", EntityType::Concept);

        kg.assert_fact(Fact::new(a.clone(), Predicate::RelatedTo, b.clone())).unwrap();
        kg.assert_fact(Fact::new(b.clone(), Predicate::RelatedTo, c.clone())).unwrap();
        kg.assert_fact(Fact::new(c.clone(), Predicate::RelatedTo, d.clone())).unwrap();

        let path = kg.find_path(&a.id, &d.id);
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 4);
    }

    #[test]
    fn test_probability() {
        let p = Probability::from_f64(0.8);
        assert!((p.value - 0.8).abs() < 0.01);

        let p2 = p.and(&Probability::from_f64(0.5));
        assert!((p2.value - 0.4).abs() < 0.01);

        let p3 = p.or(&Probability::from_f64(0.5));
        assert!((p3.value - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_retract() {
        let mut kg = KnowledgeGraph::new();

        let a = Entity::new("A", EntityType::Concept);
        let b = Entity::new("B", EntityType::Concept);

        kg.assert_fact(Fact::new(a.clone(), Predicate::RelatedTo, b.clone())).unwrap();
        assert_eq!(kg.stats().total_relations, 1);

        let removed = kg.retract_fact(&a.id, &Predicate::RelatedTo, &b.id);
        assert!(removed);
        assert_eq!(kg.stats().total_relations, 0);
    }
}
