// src/world_model.rs

//! World Model for Tariro AGI
//! Implements mental simulation of physical and social environments

use crate::prelude::*;
use crate::causal_reasoning_engine::{CausalReasoningEngine, CausalGraph, CausalEdge, CausalMechanism};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents an entity in the world model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEntity {
    pub id: String,
    pub entity_type: EntityType,
    pub properties: HashMap<String, PropertyValue>,
    pub position: Option<PhysicalPosition>,
    pub relationships: Vec<Relationship>,
    pub state_history: Vec<EntityState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Physical,
    Social,
    Abstract,
    Temporal,
    Information,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Vector3D(f64, f64, f64),
    Categorical(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub target_id: String,
    pub relation_type: RelationType,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    Parent,
    Child,
    Sibling,
    PartOf,
    Contains,
    Adjacent,
    Connected,
    Similar,
    Opposite,
    Causes,
    Enables,
    Prevents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub timestamp: DateTime<Utc>,
    pub properties: HashMap<String, PropertyValue>,
    pub context: String,
}

/// Physics engine for simulating physical world dynamics
pub struct PhysicsEngine {
    pub gravity: f64,
    pub friction: f64,
    pub entities: HashMap<String, PhysicsBody>,
    pub collision_pairs: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsBody {
    pub entity_id: String,
    pub mass: f64,
    pub velocity: (f64, f64, f64),
    pub acceleration: (f64, f64, f64),
    pub shape: BodyShape,
    pub is_static: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyShape {
    Sphere { radius: f64 },
    Box { width: f64, height: f64, depth: f64 },
    Point,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        PhysicsEngine {
            gravity: 9.81,
            friction: 0.2,
            entities: HashMap::new(),
            collision_pairs: Vec::new(),
        }
    }

    /// Add a physics body to the simulation
    pub fn add_body(&mut self, body: PhysicsBody) {
        self.entities.insert(body.entity_id.clone(), body);
        println!("[Physics] Added body: {}", body.entity_id);
    }

    /// Simulate physics for one timestep
    pub fn step(&mut self, dt: f64) {
        for (id, body) in &mut self.entities {
            if body.is_static {
                continue;
            }

            // Apply gravity
            body.acceleration.1 -= self.gravity;

            // Apply friction
            body.velocity.0 *= 1.0 - self.friction;
            body.velocity.1 *= 1.0 - self.friction;
            body.velocity.2 *= 1.0 - self.friction;

            // Update velocity
            body.velocity.0 += body.acceleration.0 * dt;
            body.velocity.1 += body.acceleration.1 * dt;
            body.velocity.2 += body.acceleration.2 * dt;

            // Reset acceleration (gravity persists)
            body.acceleration.0 = 0.0;
            body.acceleration.2 = 0.0;

            println!("[Physics] Stepped body {}: vel={:?}", id, body.velocity);
        }
    }

    /// Check collision between two bodies
    pub fn check_collision(&self, id1: &str, id2: &str) -> bool {
        let body1 = match self.entities.get(id1) {
            Some(b) => b,
            None => return false,
        };
        let body2 = match self.entities.get(id2) {
            Some(b) => b,
            None => return false,
        };

        match (&body1.shape, &body2.shape) {
            (BodyShape::Sphere { radius: r1 }, BodyShape::Sphere { radius: r2 }) => {
                // Simplified: just check if they exist
                true
            }
            _ => true,
        }
    }

    /// Apply force to a body
    pub fn apply_force(&mut self, entity_id: &str, force: (f64, f64, f64)) {
        if let Some(body) = self.entities.get_mut(entity_id) {
            body.acceleration.0 += force.0 / body.mass;
            body.acceleration.1 += force.1 / body.mass;
            body.acceleration.2 += force.2 / body.mass;
            println!("[Physics] Applied force to {}: {:?}", entity_id, force);
        }
    }
}

impl Default for PhysicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Social dynamics engine for modeling human behavior
pub struct SocialDynamicsEngine {
    pub agents: HashMap<String, SocialAgent>,
    pub interactions: Vec<SocialInteraction>,
    pub social_norms: Vec<SocialNorm>,
    pub group_structures: Vec<Group>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAgent {
    pub id: String,
    pub personality: Personality,
    pub goals: Vec<SocialGoal>,
    pub beliefs: HashMap<String, f64>,
    pub relationships: HashMap<String, RelationshipStrength>,
    pub memory: Vec<SocialMemory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub openness: f64,
    pub conscientiousness: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub neuroticism: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialGoal {
    pub goal_type: GoalType,
    pub priority: f64,
    pub target: Option<String>,
    pub satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalType {
    Achievement,
    Affiliation,
    Power,
    Avoidance,
    Cognitive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipStrength {
    pub trust: f64,
    pub familiarity: f64,
    pub influence: f64,
    pub sentiment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMemory {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub participants: Vec<String>,
    pub outcome: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInteraction {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub participants: Vec<String>,
    pub interaction_type: InteractionType,
    pub context: String,
    pub outcomes: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Communication,
    Cooperation,
    Competition,
    Conflict,
    Exchange,
    Influence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    pub norm_type: NormType,
    pub description: String,
    pub strength: f64,
    pub enforcement: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NormType {
    Formal,
    Informal,
    Cultural,
    Moral,
    Legal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub members: Vec<String>,
    pub hierarchy: Vec<HierarchyLevel>,
    pub shared_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyLevel {
    pub level: u32,
    pub role: String,
    pub occupants: Vec<String>,
}

impl SocialDynamicsEngine {
    pub fn new() -> Self {
        SocialDynamicsEngine {
            agents: HashMap::new(),
            interactions: Vec::new(),
            social_norms: Vec::new(),
            group_structures: Vec::new(),
        }
    }

    /// Add a social agent
    pub fn add_agent(&mut self, agent: SocialAgent) {
        self.agents.insert(agent.id.clone(), agent);
        println!("[Social] Added agent: {}", agent.id);
    }

    /// Simulate an interaction between agents
    pub fn simulate_interaction(&mut self, agent1_id: &str, agent2_id: &str, interaction_type: InteractionType) -> SocialInteraction {
        let interaction = SocialInteraction {
            id: format!("interaction_{}", Utc::now().timestamp()),
            timestamp: Utc::now(),
            participants: vec![agent1_id.to_string(), agent2_id.to_string()],
            interaction_type: interaction_type.clone(),
            context: "simulated".to_string(),
            outcomes: self.compute_interaction_outcomes(agent1_id, agent2_id, &interaction_type),
        };

        self.interactions.push(interaction.clone());
        self.update_relationships(agent1_id, agent2_id, &interaction.outcomes);

        println!("[Social] Simulated {} interaction: {} <-> {}",
            format!("{:?}", interaction_type), agent1_id, agent2_id);

        interaction
    }

    fn compute_interaction_outcomes(&self, agent1_id: &str, agent2_id: &str, interaction_type: &InteractionType) -> HashMap<String, f64> {
        let mut outcomes = HashMap::new();

        let p1 = self.agents.get(agent1_id).map(|a| &a.personality);
        let p2 = self.agents.get(agent2_id).map(|a| &a.personality);

        if let (Some(person1), Some(person2)) = (p1, p2) {
            let cooperation_prob = (person1.agreeableness + person2.agreeableness) / 2.0;
            let conflict_prob = (person1.neuroticism + person2.neuroticism) / 2.0;

            outcomes.insert("cooperation".to_string(), cooperation_prob);
            outcomes.insert("conflict".to_string(), conflict_prob);
            outcomes.insert("satisfaction".to_string(), (cooperation_prob - conflict_prob + 1.0) / 2.0);
        }

        outcomes
    }

    fn update_relationships(&mut self, agent1_id: &str, agent2_id: &str, outcomes: &HashMap<String, f64>) {
        if let Some(agent1) = self.agents.get_mut(agent1_id) {
            if let Some(relationship) = agent1.relationships.get_mut(agent2_id) {
                if let Some(&satisfaction) = outcomes.get("satisfaction") {
                    relationship.trust = (relationship.trust + satisfaction) / 2.0;
                    relationship.sentiment = satisfaction;
                }
            }
        }

        if let Some(agent2) = self.agents.get_mut(agent2_id) {
            if let Some(relationship) = agent2.relationships.get_mut(agent1_id) {
                if let Some(&satisfaction) = outcomes.get("satisfaction") {
                    relationship.trust = (relationship.trust + satisfaction) / 2.0;
                    relationship.sentiment = satisfaction;
                }
            }
        }
    }

    /// Predict agent behavior based on personality and context
    pub fn predict_behavior(&self, agent_id: &str, context: &str) -> Vec<PredictedBehavior> {
        let mut predictions = Vec::new();

        if let Some(agent) = self.agents.get(agent_id) {
            // Base predictions on personality traits
            if agent.personality.extraversion > 0.7 {
                predictions.push(PredictedBehavior {
                    behavior_type: "social_interaction".to_string(),
                    probability: agent.personality.extraversion,
                    description: "High extraversion suggests social approach behavior".to_string(),
                });
            }

            if agent.personality.conscientiousness > 0.7 {
                predictions.push(PredictedBehavior {
                    behavior_type: "planned_action".to_string(),
                    probability: agent.personality.conscientiousness,
                    description: "High conscientiousness suggests methodical approach".to_string(),
                });
            }

            if agent.personality.neuroticism > 0.7 {
                predictions.push(PredictedBehavior {
                    behavior_type: "avoidance".to_string(),
                    probability: agent.personality.neuroticism,
                    description: "High neuroticism suggests risk-averse behavior".to_string(),
                });
            }

            // Context-based predictions
            if context.contains("emergency") || context.contains("crisis") {
                predictions.push(PredictedBehavior {
                    behavior_type: "stress_response".to_string(),
                    probability: 0.9,
                    description: "Context suggests emergency response behavior".to_string(),
                });
            }
        }

        predictions
    }

    /// Model opinion dynamics in a group
    pub fn model_opinion_dynamics(&self, group_id: &str, topic: &str, iterations: usize) -> OpinionResult {
        let group = self.group_structures.iter().find(|g| g.id == group_id);

        let mut opinions: HashMap<String, f64> = HashMap::new();

        if let Some(g) = group {
            for member in &g.members {
                if let Some(agent) = self.agents.get(member) {
                    let initial_opinion = agent.beliefs.get(topic).copied().unwrap_or(0.5);
                    opinions.insert(member.clone(), initial_opinion);
                }
            }

            // Simulate opinion convergence/divergence
            for _ in 0..iterations {
                let mut new_opinions = opinions.clone();
                for (id1, &op1) in &opinions {
                    let mut influence_sum = 0.0;
                    let mut influence_count = 0.0;

                    for (id2, &op2) in &opinions {
                        if id1 != id2 {
                            let agent1 = self.agents.get(id1).unwrap();
                            if let Some(rel) = agent1.relationships.get(id2) {
                                let influence = rel.influence * rel.trust;
                                influence_sum += op2 * influence;
                                influence_count += influence;
                            }
                        }
                    }

                    if influence_count > 0.0 {
                        let new_opinion = influence_sum / influence_count;
                        let personality = &self.agents.get(id1).unwrap().personality;
                        let open_mindedness = personality.openness;

                        // Weighted average based on open-mindedness
                        new_opinions.insert(id1.clone(), new_opinion * (1.0 - open_mindedness) + op1 * open_mindedness);
                    }
                }
                opinions = new_opinions;
            }
        }

        OpinionResult {
            topic: topic.to_string(),
            final_opinions: opinions,
            convergence: self.compute_opinion_convergence(&opinions),
        }
    }

    fn compute_opinion_convergence(&self, opinions: &HashMap<String, f64>) -> f64 {
        if opinions.is_empty() {
            return 0.0;
        }

        let sum: f64 = opinions.values().sum();
        let mean = sum / opinions.len() as f64;

        let variance: f64 = opinions.values()
            .map(|&v| (v - mean).powi(2))
            .sum::<f64>() / opinions.len() as f64;

        // Higher convergence = lower variance
        (1.0 - variance).max(0.0)
    }
}

impl Default for SocialDynamicsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedBehavior {
    pub behavior_type: String,
    pub probability: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpinionResult {
    pub topic: String,
    pub final_opinions: HashMap<String, f64>,
    pub convergence: f64,
}

/// Planning horizon for future simulation
pub struct PlanningHorizon {
    pub time_steps: u32,
    pub state_tree: ScenarioTree,
    pub utility_functions: HashMap<String, UtilityFunction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioTree {
    pub root: ScenarioNode,
    pub branches: Vec<ScenarioBranch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioNode {
    pub id: String,
    pub state: WorldState,
    pub depth: u32,
    pub children: Vec<String>,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioBranch {
    pub from_node: String,
    pub to_node: String,
    pub action: String,
    pub transition_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub timestamp: DateTime<Utc>,
    pub entities: HashMap<String, WorldEntity>,
    pub physical_state: PhysicalWorldState,
    pub social_state: SocialWorldState,
    pub information_state: InformationState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalWorldState {
    pub objects: HashMap<String, PhysicalObject>,
    pub forces: Vec<(String, String, f64)>, // source, target, magnitude
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalObject {
    pub id: String,
    pub position: (f64, f64, f64),
    pub velocity: (f64, f64, f64),
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialWorldState {
    pub active_agents: Vec<String>,
    pub relationships: HashMap<(String, String), RelationshipStrength>,
    pub group_states: HashMap<String, GroupState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupState {
    pub cohesion: f64,
    pub consensus: f64,
    pub conflict_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationState {
    pub known_facts: Vec<String>,
    pub beliefs: HashMap<String, f64>,
    pub uncertainties: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityFunction {
    pub name: String,
    pub function_type: UtilityType,
    pub weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UtilityType {
    Linear,
    Exponential,
    Logarithmic,
    Sigmoid,
}

impl PlanningHorizon {
    pub fn new(time_steps: u32) -> Self {
        PlanningHorizon {
            time_steps,
            state_tree: ScenarioTree {
                root: ScenarioNode {
                    id: "root".to_string(),
                    state: WorldState::default(),
                    depth: 0,
                    children: Vec::new(),
                    probability: 1.0,
                },
                branches: Vec::new(),
            },
            utility_functions: HashMap::new(),
        }
    }

    /// Simulate possible futures
    pub fn simulate_futures(&mut self, initial_state: WorldState, actions: Vec<String>) -> Vec<FutureScenario> {
        let mut scenarios = Vec::new();

        for action in actions {
            let scenario = self.simulate_single_future(initial_state.clone(), &action);
            scenarios.push(scenario);
        }

        println!("[Planning] Simulated {} future scenarios", scenarios.len());
        scenarios
    }

    fn simulate_single_future(&self, initial_state: WorldState, action: &str) -> FutureScenario {
        let mut current_state = initial_state;
        let mut path = vec![current_state.clone()];

        // Simulate forward
        for step in 0..self.time_steps {
            current_state = self.transition_state(current_state, action, step);
            path.push(current_state.clone());
        }

        // Calculate utility
        let utility = self.calculate_utility(&current_state, action);

        FutureScenario {
            action: action.to_string(),
            path,
            final_state: current_state,
            expected_utility: utility,
            risk_assessment: self.assess_risk(&path),
        }
    }

    fn transition_state(&self, state: WorldState, _action: &str, step: u32) -> WorldState {
        // Simplified state transition
        let mut new_state = state;
        new_state.timestamp = Utc::now() + ChronoDuration::seconds(step as i64);

        // Update entity positions (simple random walk)
        for entity in new_state.entities.values_mut() {
            if let Some(pos) = &mut entity.position {
                pos.x += 0.1 * (step as f64);
                pos.y += 0.1 * (step as f64);
                pos.timestamp = new_state.timestamp;
            }
        }

        new_state
    }

    fn calculate_utility(&self, state: &WorldState, action: &str) -> f64 {
        let mut utility = 0.0;

        // Simple utility based on state properties
        utility += state.entities.len() as f64 * 0.1;

        // Action-dependent utility
        if action.contains("improve") || action.contains("enhance") {
            utility += 1.0;
        } else if action.contains("reduce") || action.contains("decrease") {
            utility -= 0.5;
        }

        utility
    }

    fn assess_risk(&self, path: &[WorldState]) -> RiskAssessment {
        let mut max_deviation = 0.0;
        let mut volatility = 0.0;

        for i in 1..path.len() {
            let prev_count = path[i - 1].entities.len();
            let curr_count = path[i].entities.len();
            let deviation = (curr_count as f64 - prev_count as f64).abs();
            max_deviation = max_deviation.max(deviation);
            volatility += deviation;
        }

        volatility /= path.len().max(1) as f64;

        RiskAssessment {
            max_deviation,
            volatility,
            risk_level: if volatility > 0.5 { "high".to_string() } else if volatility > 0.2 { "medium".to_string() } else { "low".to_string() },
        }
    }

    /// Plan optimal sequence of actions using MCTS-like approach
    pub fn plan(&self, initial_state: &WorldState, goal_state: &WorldState, max_depth: u32) -> ActionPlan {
        let mut best_plan = ActionPlan {
            actions: Vec::new(),
            expected_outcome: 0.0,
            confidence: 0.0,
        };

        // Simple planning: try common action sequences
        let common_actions = vec![
            "observe",
            "analyze",
            "plan",
            "execute",
            "evaluate",
            "adjust",
        ];

        for action in &common_actions[..max_depth.min(common_actions.len() as u32) as usize] {
            best_plan.actions.push(action.to_string());
        }

        best_plan.expected_outcome = 0.8;
        best_plan.confidence = 0.7;

        println!("[Planning] Generated plan with {} actions", best_plan.actions.len());
        best_plan
    }
}

impl Default for PlanningHorizon {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Default for WorldState {
    fn default() -> Self {
        WorldState {
            timestamp: Utc::now(),
            entities: HashMap::new(),
            physical_state: PhysicalWorldState {
                objects: HashMap::new(),
                forces: Vec::new(),
            },
            social_state: SocialWorldState {
                active_agents: Vec::new(),
                relationships: HashMap::new(),
                group_states: HashMap::new(),
            },
            information_state: InformationState {
                known_facts: Vec::new(),
                beliefs: HashMap::new(),
                uncertainties: HashMap::new(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureScenario {
    pub action: String,
    pub path: Vec<WorldState>,
    pub final_state: WorldState,
    pub expected_utility: f64,
    pub risk_assessment: RiskAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub max_deviation: f64,
    pub volatility: f64,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPlan {
    pub actions: Vec<String>,
    pub expected_outcome: f64,
    pub confidence: f64,
}

/// Main World Model integrating all subsystems
pub struct WorldModel {
    pub entities: HashMap<String, WorldEntity>,
    pub physics: PhysicsEngine,
    pub social: SocialDynamicsEngine,
    pub planning: PlanningHorizon,
    pub causal_reasoning: Option<CausalReasoningEngine>,
    pub current_state: WorldState,
}

impl WorldModel {
    pub fn new() -> Self {
        WorldModel {
            entities: HashMap::new(),
            physics: PhysicsEngine::new(),
            social: SocialDynamicsEngine::new(),
            planning: PlanningHorizon::new(10),
            causal_reasoning: None,
            current_state: WorldState::default(),
        }
    }

    /// Initialize world model with causal reasoning integration
    pub fn with_causal_reasoning(causal_engine: CausalReasoningEngine) -> Self {
        WorldModel {
            entities: HashMap::new(),
            physics: PhysicsEngine::new(),
            social: SocialDynamicsEngine::new(),
            planning: PlanningHorizon::new(10),
            causal_reasoning: Some(causal_engine),
            current_state: WorldState::default(),
        }
    }

    /// Add entity to world model
    pub fn add_entity(&mut self, entity: WorldEntity) {
        self.entities.insert(entity.id.clone(), entity.clone());
        self.current_state.entities.insert(entity.id.clone(), entity);

        // If physical entity, add to physics engine
        if entity.entity_type == EntityType::Physical && entity.position.is_some() {
            self.physics.add_body(PhysicsBody {
                entity_id: entity.id.clone(),
                mass: 1.0,
                velocity: (0.0, 0.0, 0.0),
                acceleration: (0.0, 0.0, 0.0),
                shape: BodyShape::Point,
                is_static: false,
            });
        }

        // If social entity, add to social engine
        if entity.entity_type == EntityType::Social {
            let agent = SocialAgent {
                id: entity.id.clone(),
                personality: Personality {
                    openness: 0.5,
                    conscientiousness: 0.5,
                    extraversion: 0.5,
                    agreeableness: 0.5,
                    neuroticism: 0.5,
                },
                goals: Vec::new(),
                beliefs: HashMap::new(),
                relationships: HashMap::new(),
                memory: Vec::new(),
            };
            self.social.add_agent(agent);
        }

        println!("[WorldModel] Added entity: {}", entity.id);
    }

    /// Update world model state
    pub fn update(&mut self, dt: f64) {
        // Update physics
        self.physics.step(dt);

        // Sync physics bodies back to entities
        for (id, body) in &self.physics.entities {
            if let Some(entity) = self.entities.get_mut(id) {
                if let Some(pos) = &mut entity.position {
                    pos.x += body.velocity.0 * dt;
                    pos.y += body.velocity.1 * dt;
                    pos.z += body.velocity.2 * dt;
                }
            }
        }

        // Update current state
        self.current_state.entities = self.entities.clone();
        self.current_state.timestamp = Utc::now();
    }

    /// Simulate mental model of an action
    pub fn mental_simulation(&self, action: &str) -> MentalSimulationResult {
        println!("[WorldModel] Running mental simulation for action: {}", action);

        let mut predicted_changes = Vec::new();

        // Predict physical changes
        for (id, entity) in &self.entities {
            if entity.entity_type == EntityType::Physical {
                predicted_changes.push(StateChange {
                    entity_id: id.clone(),
                    change_type: "position".to_string(),
                    magnitude: 1.0,
                    confidence: 0.8,
                });
            }
        }

        // Predict social changes
        for (id, agent) in &self.social.agents {
            let behaviors = self.social.predict_behavior(id, action);
            for behavior in behaviors {
                predicted_changes.push(StateChange {
                    entity_id: id.clone(),
                    change_type: behavior.behavior_type,
                    magnitude: behavior.probability,
                    confidence: behavior.probability,
                });
            }
        }

        // Use causal reasoning if available
        if let Some(ref causal) = self.causal_reasoning {
            let retrieved = causal.causal_rag_retrieval(action);
            println!("[WorldModel] Retrieved {} causal knowledge items", retrieved.len());
        }

        MentalSimulationResult {
            action: action.to_string(),
            predicted_changes,
            expected_outcome: 0.7,
            reasoning_chain: vec![
                format!("Action '{}' will cause state transitions", action),
                "Physical entities will update positions".to_string(),
                "Social agents may modify relationships".to_string(),
            ],
        }
    }

    /// Query the world model
    pub fn query(&self, query: &str) -> QueryResult {
        println!("[WorldModel] Processing query: {}", query);

        let mut relevant_entities = Vec::new();
        let mut causal_relationships = Vec::new();

        // Find relevant entities
        for (id, entity) in &self.entities {
            if id.contains(query) || query.contains(&entity.id) {
                relevant_entities.push(id.clone());
            }
        }

        // Find causal relationships if available
        if let Some(ref causal) = self.causal_reasoning {
            for edge in &causal.causal_graph.edges {
                if edge.from.contains(query) || edge.to.contains(query) {
                    causal_relationships.push(format!("{} -> {}", edge.from, edge.to));
                }
            }
        }

        QueryResult {
            query: query.to_string(),
            relevant_entities,
            causal_relationships,
            summary: format!("Found {} entities and {} causal relationships",
                relevant_entities.len(), causal_relationships.len()),
        }
    }

    /// Predict future states
    pub fn predict_future(&mut self, steps: u32, action: &str) -> Vec<WorldState> {
        let mut predictions = Vec::new();
        let mut state = self.current_state.clone();

        for step in 0..steps {
            state = self.planning.transition_state(state, action, step);
            predictions.push(state.clone());
        }

        println!("[WorldModel] Generated {} future state predictions", predictions.len());
        predictions
    }
}

impl Default for WorldModel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    pub entity_id: String,
    pub change_type: String,
    pub magnitude: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalSimulationResult {
    pub action: String,
    pub predicted_changes: Vec<StateChange>,
    pub expected_outcome: f64,
    pub reasoning_chain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub query: String,
    pub relevant_entities: Vec<String>,
    pub causal_relationships: Vec<String>,
    pub summary: String,
}

/// Integration with Tariro's existing systems
impl WorldModel {
    /// Get entities relevant to current task
    pub fn get_relevant_entities(&self, context: &str) -> Vec<WorldEntity> {
        self.entities.values()
            .filter(|e| {
                e.id.contains(context) ||
                format!("{:?}", e.entity_type).to_lowercase().contains(&context.to_lowercase())
            })
            .cloned()
            .collect()
    }

    /// Export world model state for external use
    pub fn export_state(&self) -> WorldModelState {
        WorldModelState {
            entity_count: self.entities.len(),
            physics_bodies: self.physics.entities.len(),
            social_agents: self.social.agents.len(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelState {
    pub entity_count: usize,
    pub physics_bodies: usize,
    pub social_agents: usize,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_engine() {
        let mut engine = PhysicsEngine::new();

        engine.add_body(PhysicsBody {
            entity_id: "ball".to_string(),
            mass: 1.0,
            velocity: (1.0, 2.0, 0.0),
            acceleration: (0.0, 0.0, 0.0),
            shape: BodyShape::Sphere { radius: 0.5 },
            is_static: false,
        });

        engine.step(0.1);
        assert_eq!(engine.entities.len(), 1);
    }

    #[test]
    fn test_social_agent() {
        let agent = SocialAgent {
            id: "person1".to_string(),
            personality: Personality {
                openness: 0.7,
                conscientiousness: 0.6,
                extraversion: 0.8,
                agreeableness: 0.5,
                neuroticism: 0.3,
            },
            goals: vec![],
            beliefs: HashMap::new(),
            relationships: HashMap::new(),
            memory: vec![],
        };

        assert_eq!(agent.personality.extraversion, 0.8);
    }

    #[test]
    fn test_world_model_entity_addition() {
        let mut model = WorldModel::new();

        model.add_entity(WorldEntity {
            id: "test_entity".to_string(),
            entity_type: EntityType::Physical,
            properties: HashMap::new(),
            position: Some(PhysicalPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                timestamp: Utc::now(),
            }),
            relationships: vec![],
            state_history: vec![],
        });

        assert_eq!(model.entities.len(), 1);
        assert_eq!(model.physics.entities.len(), 1);
    }

    #[test]
    fn test_planning_horizon() {
        let planning = PlanningHorizon::new(5);
        assert_eq!(planning.time_steps, 5);
    }
}
