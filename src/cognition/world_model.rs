//! Tariro World Model
//!
//! The mind's internal model of the world - entities, causal relationships,
//! and the ability to simulate counterfactuals.

use crate::prelude::*;
use std::collections::{HashMap, HashSet};

// ============================================================================
// World Entities
// ============================================================================

/// An entity in the world model
#[derive(Debug, Clone)]
pub struct WorldEntity {
    pub id: EntityId,
    pub name: String,
    pub entity_type: WorldEntityType,
    pub properties: HashMap<String, Property>,
    pub location: Option<Location>,
    pub state: WorldState,
}

impl WorldEntity {
    pub fn new(name: impl Into<String>, entity_type: WorldEntityType) -> Self {
        WorldEntity {
            id: EntityId::new(),
            name: name.into(),
            entity_type,
            properties: HashMap::new(),
            location: None,
            state: WorldState::default(),
        }
    }

    pub fn with_property(mut self, key: impl Into<String>, value: Property) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}

/// Type of world entity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorldEntityType {
    Object,
    Agent,
    Process,
    Event,
    Place,
    Abstract,
}

/// A property
#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
    pub confidence: f64,
}

/// Property value
#[derive(Debug, Clone)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<PropertyValue>),
}

/// Location
#[derive(Debug, Clone)]
pub struct Location {
    pub space: Space,
    pub coordinates: Coordinates,
}

/// Space type
#[derive(Debug, Clone, Copy)]
pub enum Space {
    Physical,
    Abstract,
    Social,
    Temporal,
}

/// Coordinates in space
#[derive(Debug, Clone)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Current state of an entity
#[derive(Debug, Clone)]
pub struct WorldState {
    pub active: bool,
    pub properties: HashMap<String, PropertyValue>,
    pub relationships: Vec<Relationship>,
}

impl Default for WorldState {
    fn default() -> Self {
        WorldState {
            active: true,
            properties: HashMap::new(),
            relationships: Vec::new(),
        }
    }
}

/// Relationship to another entity
#[derive(Debug, Clone)]
pub struct Relationship {
    pub target_id: EntityId,
    pub relationship_type: RelationshipType,
}

// ============================================================================
// Causal Graph
// ============================================================================

/// A causal relationship
#[derive(Debug, Clone)]
pub struct CausalRelationship {
    pub cause: EntityId,
    pub effect: EntityId,
    pub mechanism: String,
    pub probability: f64,
    pub temporal_delay: Option<i64>,
}

/// Causal graph for the world
#[derive(Debug, Clone)]
pub struct CausalGraph {
    nodes: HashMap<EntityId, WorldEntity>,
    edges: Vec<CausalRelationship>,
    adjacency: HashMap<EntityId, Vec<EntityId>>,
    reverse_adjacency: HashMap<EntityId, Vec<EntityId>>,
}

impl CausalGraph {
    pub fn new() -> Self {
        CausalGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
            reverse_adjacency: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: WorldEntity) {
        let id = entity.id.clone();
        self.nodes.insert(id.clone(), entity);
        self.adjacency.entry(id).or_default();
        self.reverse_adjacency.entry(id).or_default();
    }

    pub fn add_causal_link(&mut self, relationship: CausalRelationship) {
        let cause_id = relationship.cause.clone();
        let effect_id = relationship.effect.clone();

        self.edges.push(relationship);

        self.adjacency.entry(cause_id.clone())
            .or_default()
            .push(effect_id.clone());

        self.reverse_adjacency.entry(effect_id)
            .or_default()
            .push(cause_id);
    }

    pub fn get_causes(&self, entity: &EntityId) -> Vec<&WorldEntity> {
        self.reverse_adjacency.get(entity)
            .map(|ids| ids.iter().filter_map(|id| self.nodes.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn get_effects(&self, entity: &EntityId) -> Vec<&WorldEntity> {
        self.adjacency.get(entity)
            .map(|ids| ids.iter().filter_map(|id| self.nodes.get(id)).collect())
            .unwrap_or_default()
    }

    pub fn find_causal_chain(&self, start: &EntityId, end: &EntityId) -> Option<Vec<EntityId>> {
        use std::collections::VecDeque;

        let mut queue: VecDeque<(EntityId, Vec<EntityId>)> = VecDeque::new();
        let mut visited: HashSet<EntityId> = HashSet::new();

        queue.push_back((start.clone(), vec![start.clone()]));
        visited.insert(start.clone());

        while let Some((current, path)) = queue.pop_front() {
            if current == *end {
                return Some(path);
            }

            if let Some(effects) = self.adjacency.get(&current) {
                for effect in effects {
                    if !visited.contains(effect) {
                        visited.insert(effect.clone());
                        let mut new_path = path.clone();
                        new_path.push(effect.clone());
                        queue.push_back((effect.clone(), new_path));
                    }
                }
            }
        }

        None
    }
}

impl Default for CausalGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Imagination Engine
// ============================================================================

/// Imagination engine for counterfactual reasoning
pub struct ImaginationEngine {
    /// Maximum simulation depth
    max_depth: usize,

    /// Simulation statistics
    stats: ImaginationStats,
}

impl ImaginationEngine {
    pub fn new() -> Self {
        ImaginationEngine {
            max_depth: 10,
            stats: ImaginationStats::default(),
        }
    }

    /// Simulate an action and predict consequences
    pub fn simulate_action(
        &self,
        graph: &mut CausalGraph,
        action: &SimulatedAction,
    ) -> SimulationResult {
        self.stats.total_simulations += 1;

        let mut current_state = action.initial_state.clone();
        let mut predicted_effects = Vec::new();
        let mut visited = HashSet::new();

        // Trace causal chain
        self.trace_effects(
            graph,
            &action.target,
            &mut current_state,
            &mut predicted_effects,
            &mut visited,
            0,
        );

        SimulationResult {
            action: action.clone(),
            predicted_effects,
            final_state: current_state,
            confidence: self.calculate_confidence(&predicted_effects),
        }
    }

    fn trace_effects(
        &self,
        graph: &CausalGraph,
        entity: &EntityId,
        state: &mut SimulatedState,
        effects: &mut Vec<PredictedEffect>,
        visited: &mut HashSet<EntityId>,
        depth: usize,
    ) {
        if depth >= self.max_depth || visited.contains(entity) {
            return;
        }

        visited.insert(entity.clone());

        // Find direct effects
        for effect_entity in graph.get_effects(entity) {
            effects.push(PredictedEffect {
                entity: effect_entity.id.clone(),
                effect_type: "caused_by".to_string(),
                description: format!("{} is affected by {}", effect_entity.name, entity.id.0),
                probability: 0.8,
                confidence: 0.9,
            });

            // Recursively trace
            self.trace_effects(
                graph,
                &effect_entity.id,
                state,
                effects,
                visited,
                depth + 1,
            );
        }
    }

    fn calculate_confidence(&self, effects: &[PredictedEffect]) -> f64 {
        if effects.is_empty() {
            return 0.5;
        }

        let avg_confidence: f64 = effects.iter()
            .map(|e| e.confidence)
            .sum::<f64>() / effects.len() as f64;

        avg_confidence * (1.0 - (effects.len() as f64 * 0.01)).max(0.5)
    }

    /// Counterfactual: what if X had been different?
    pub fn counterfactual(
        &self,
        graph: &mut CausalGraph,
        scenario: &CounterfactualScenario,
    ) -> CounterfactualResult {
        self.stats.counterfactual_simulations += 1;

        // Apply hypothetical change
        let mut hypothetical_state = scenario.actual_state.clone();

        // Find affected entities
        let mut affected = Vec::new();

        if let Some(change) = &scenario.hypothetical_change {
            hypothetical_state.apply_change(change);

            // Find entities affected by the change
            affected.push(change.target.clone());

            // Recursively find downstream effects
            self.find_affected_entities(graph, &change.target, &mut affected, &mut HashSet::new());
        }

        CounterfactualResult {
            scenario: scenario.clone(),
            affected_entities: affected,
            differences: vec![], // Would calculate actual differences
            conclusion: "Counterfactual analysis complete".to_string(),
            confidence: 0.7,
        }
    }

    fn find_affected_entities(
        &self,
        graph: &CausalGraph,
        entity: &EntityId,
        affected: &mut Vec<EntityId>,
        visited: &mut HashSet<EntityId>,
    ) {
        if visited.contains(entity) {
            return;
        }
        visited.insert(entity.clone());

        for effect_entity in graph.get_effects(entity) {
            affected.push(effect_entity.id.clone());
            self.find_affected_entities(graph, &effect_entity.id, affected, visited);
        }
    }

    pub fn stats(&self) -> &ImaginationStats {
        &self.stats
    }
}

impl Default for ImaginationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Imagination statistics
#[derive(Debug, Clone, Default)]
pub struct ImaginationStats {
    pub total_simulations: usize,
    pub counterfactual_simulations: usize,
}

// ============================================================================
// Simulation Types
// ============================================================================

/// An action to simulate
#[derive(Debug, Clone)]
pub struct SimulatedAction {
    pub target: EntityId,
    pub action_type: String,
    pub parameters: HashMap<String, PropertyValue>,
    pub initial_state: SimulatedState,
}

/// State for simulation
#[derive(Debug, Clone)]
pub struct SimulatedState {
    pub entity_states: HashMap<EntityId, HashMap<String, PropertyValue>>,
}

impl SimulatedState {
    pub fn new() -> Self {
        SimulatedState {
            entity_states: HashMap::new(),
        }
    }

    pub fn set_state(&mut self, entity: EntityId, key: String, value: PropertyValue) {
        self.entity_states
            .entry(entity)
            .or_default()
            .insert(key, value);
    }

    pub fn get_state(&self, entity: &EntityId, key: &str) -> Option<&PropertyValue> {
        self.entity_states.get(entity)
            .and_then(|states| states.get(key))
    }

    pub fn apply_change(&mut self, change: &HypotheticalChange) {
        self.set_state(change.target.clone(), change.property.clone(), change.new_value.clone());
    }
}

impl Default for SimulatedState {
    fn default() -> Self {
        Self::new()
    }
}

/// A predicted effect
#[derive(Debug, Clone)]
pub struct PredictedEffect {
    pub entity: EntityId,
    pub effect_type: String,
    pub description: String,
    pub probability: f64,
    pub confidence: f64,
}

/// Result of simulation
#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub action: SimulatedAction,
    pub predicted_effects: Vec<PredictedEffect>,
    pub final_state: SimulatedState,
    pub confidence: f64,
}

/// Counterfactual scenario
#[derive(Debug, Clone)]
pub struct CounterfactualScenario {
    pub description: String,
    pub actual_state: SimulatedState,
    pub hypothetical_change: Option<HypotheticalChange>,
}

/// Hypothetical change
#[derive(Debug, Clone)]
pub struct HypotheticalChange {
    pub target: EntityId,
    pub property: String,
    pub original_value: PropertyValue,
    pub new_value: PropertyValue,
}

/// Result of counterfactual analysis
#[derive(Debug, Clone)]
pub struct CounterfactualResult {
    pub scenario: CounterfactualScenario,
    pub affected_entities: Vec<EntityId>,
    pub differences: Vec<StateDifference>,
    pub conclusion: String,
    pub confidence: f64,
}

/// A state difference
#[derive(Debug, Clone)]
pub struct StateDifference {
    pub entity: EntityId,
    pub property: String,
    pub before: PropertyValue,
    pub after: PropertyValue,
}

// ============================================================================
// Social Cognition
// ============================================================================

/// Theory of mind
pub struct TheoryOfMind {
    /// Models of other agents
    agent_models: HashMap<EntityId, AgentModel>,
}

impl TheoryOfMind {
    pub fn new() -> Self {
        TheoryOfMind {
            agent_models: HashMap::new(),
        }
    }

    /// Create a model of an agent
    pub fn create_agent_model(&mut self, agent: WorldEntity) {
        let model = AgentModel {
            id: agent.id.clone(),
            beliefs: HashMap::new(),
            desires: Vec::new(),
            intentions: Vec::new(),
            capabilities: Vec::new(),
        };
        self.agent_models.insert(agent.id.clone(), model);
    }

    /// Update beliefs about an agent
    pub fn update_beliefs(&mut self, agent_id: &EntityId, beliefs: HashMap<String, PropertyValue>) {
        if let Some(model) = self.agent_models.get_mut(agent_id) {
            model.beliefs = beliefs;
        }
    }

    /// Predict what an agent will do
    pub fn predict_action(&self, agent_id: &EntityId) -> Option<String> {
        self.agent_models.get(agent_id).map(|model| {
            // Simple prediction based on desires and beliefs
            if !model.desires.is_empty() {
                format!("Agent {} will pursue: {}", agent_id.0, model.desires[0])
            } else {
                "Unknown intention".to_string()
            }
        })
    }

    /// Understand another agent's perspective
    pub fn perspective_taking(&self, agent_id: &EntityId, topic: &str) -> String {
        self.agent_models.get(agent_id)
            .map(|model| {
                format!(
                    "From {}'s perspective on {}: {}",
                    agent_id.0,
                    topic,
                    model.beliefs.get(topic)
                        .map(|v| format!("{:?}", v))
                        .unwrap_or_else(|| "Unknown".to_string())
                )
            })
            .unwrap_or_else(|| "Agent model not found".to_string())
    }
}

impl Default for TheoryOfMind {
    fn default() -> Self {
        Self::new()
    }
}

/// Model of an agent
#[derive(Debug, Clone)]
pub struct AgentModel {
    pub id: EntityId,
    pub beliefs: HashMap<String, PropertyValue>,
    pub desires: Vec<String>,
    pub intentions: Vec<String>,
    pub capabilities: Vec<String>,
}

// ============================================================================
// World Model
// ============================================================================

/// The complete world model
pub struct WorldModel {
    /// Entities in the world
    entities: HashMap<EntityId, WorldEntity>,

    /// Causal relationships
    causal_graph: CausalGraph,

    /// Imagination engine
    imagination: ImaginationEngine,

    /// Theory of mind
    theory_of_mind: TheoryOfMind,

    /// Statistics
    stats: WorldModelStats,
}

impl WorldModel {
    pub fn new() -> Self {
        WorldModel {
            entities: HashMap::new(),
            causal_graph: CausalGraph::new(),
            imagination: ImaginationEngine::new(),
            theory_of_mind: TheoryOfMind::new(),
            stats: WorldModelStats::default(),
        }
    }

    /// Add an entity to the world
    pub fn add_entity(&mut self, entity: WorldEntity) {
        self.entities.insert(entity.id.clone(), entity.clone());
        self.causal_graph.add_entity(entity);
        self.stats.entities_added += 1;

        // Create theory of mind model for agents
        if entity.entity_type == WorldEntityType::Agent {
            self.theory_of_mind.create_agent_model(entity);
        }
    }

    /// Add a causal relationship
    pub fn add_causal_relation(&mut self, relationship: CausalRelationship) {
        self.causal_graph.add_causal_link(relationship);
        self.stats.causal_relations_added += 1;
    }

    /// Get an entity
    pub fn get_entity(&self, id: &EntityId) -> Option<&WorldEntity> {
        self.entities.get(id)
    }

    /// Find entities matching a pattern
    pub fn find_entities(&self, pattern: &str) -> Vec<&WorldEntity> {
        self.entities.values()
            .filter(|e| e.name.to_lowercase().contains(&pattern.to_lowercase()))
            .collect()
    }

    /// Predict what will happen if action is taken
    pub fn predict(&mut self, action: SimulatedAction) -> SimulationResult {
        self.imagination.simulate_action(&mut self.causal_graph, &action)
    }

    /// Counterfactual: what if things were different?
    pub fn what_if(&mut self, scenario: CounterfactualScenario) -> CounterfactualResult {
        self.imagination.counterfactual(&mut self.causal_graph, &scenario)
    }

    /// Get causal predecessors (causes)
    pub fn get_causes(&self, entity: &EntityId) -> Vec<&WorldEntity> {
        self.causal_graph.get_causes(entity)
    }

    /// Get causal successors (effects)
    pub fn get_effects(&self, entity: &EntityId) -> Vec<&WorldEntity> {
        self.causal_graph.get_effects(entity)
    }

    /// Find causal chain between entities
    pub fn find_causal_chain(&self, from: &EntityId, to: &EntityId) -> Option<Vec<EntityId>> {
        self.causal_graph.find_causal_chain(from, to)
    }

    /// Understand another agent's perspective
    pub fn understand_agent(&self, agent_id: &EntityId, topic: &str) -> String {
        self.theory_of_mind.perspective_taking(agent_id, topic)
    }

    /// Get statistics
    pub fn stats(&self) -> &WorldModelStats {
        &self.stats
    }
}

impl Default for WorldModel {
    fn default() -> Self {
        Self::new()
    }
}

/// World model statistics
#[derive(Debug, Clone, Default)]
pub struct WorldModelStats {
    pub entities_added: usize,
    pub causal_relations_added: usize,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity = WorldEntity::new("Test Object", WorldEntityType::Object);
        assert_eq!(entity.name, "Test Object");
    }

    #[test]
    fn test_causal_graph() {
        let mut graph = CausalGraph::new();

        let entity1 = WorldEntity::new("Fire", WorldEntityType::Process);
        let entity2 = WorldEntity::new("Smoke", WorldEntityType::Event);

        graph.add_entity(entity1.clone());
        graph.add_entity(entity2.clone());

        graph.add_causal_link(CausalRelationship {
            cause: entity1.id.clone(),
            effect: entity2.id.clone(),
            mechanism: "Combustion produces smoke".to_string(),
            probability: 0.9,
            temporal_delay: Some(10),
        });

        let effects = graph.get_effects(&entity1.id);
        assert_eq!(effects.len(), 1);
    }

    #[test]
    fn test_world_model() {
        let mut model = WorldModel::new();

        let fire = WorldEntity::new("Fire", WorldEntityType::Process);
        let smoke = WorldEntity::new("Smoke", WorldEntityType::Event);

        model.add_entity(fire.clone());
        model.add_entity(smoke.clone());

        assert_eq!(model.entities.len(), 2);
    }

    #[test]
    fn test_simulation() {
        let mut model = WorldModel::new();
        let entity = WorldEntity::new("Test", WorldEntityType::Object);
        model.add_entity(entity.clone());

        let action = SimulatedAction {
            target: entity.id.clone(),
            action_type: "Activate".to_string(),
            parameters: HashMap::new(),
            initial_state: SimulatedState::new(),
        };

        let result = model.predict(action);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_theory_of_mind() {
        let mut tom = TheoryOfMind::new();

        let agent = WorldEntity::new("Alice", WorldEntityType::Agent);
        tom.create_agent_model(agent);

        let prediction = tom.predict_action(&agent.id);
        assert!(prediction.is_some());
    }
}
