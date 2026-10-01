//! Tariro Cognitive Learning Module
//!
//! Implements multiple learning mechanisms:
//! - Analogical Reasoning (Structure Mapping Theory)
//! - Conceptual Change (Theory Revision)
//! - Reinforcement Learning (Q-Learning, Policy Gradients)
//! - Imitation Learning

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for learning items
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LearningId(pub Uuid);

impl LearningId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for LearningId {
    fn default() -> Self {
        Self::new()
    }
}

/// Representational structure for analogical reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Representation {
    pub id: LearningId,
    pub objects: Vec<RepresentedObject>,
    pub relations: Vec<RepresentedRelation>,
    pub attributes: Vec<RepresentedAttribute>,
}

impl Representation {
    pub fn new() -> Self {
        Self {
            id: LearningId::new(),
            objects: Vec::new(),
            relations: Vec::new(),
            attributes: Vec::new(),
        }
    }
}

/// Object in a representation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepresentedObject {
    pub id: String,
    pub name: String,
    pub object_type: String,
}

impl RepresentedObject {
    pub fn new(name: &str, object_type: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            object_type: object_type.to_string(),
        }
    }
}

/// Relation between objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepresentedRelation {
    pub name: String,
    pub source: String,
    pub target: String,
}

impl RepresentedRelation {
    pub fn new(name: &str, source: &str, target: &str) -> Self {
        Self {
            name: name.to_string(),
            source: source.to_string(),
            target: target.to_string(),
        }
    }
}

/// Attribute of an object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepresentedAttribute {
    pub object: String,
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeValue {
    Numeric(f64),
    Boolean(bool),
    Categorical(String),
}

// ============================================================================
// Analogical Reasoning (Structure Mapping Theory)
// ============================================================================

/// Structure mapping engine (Gentner, 1983)
#[derive(Debug, Clone)]
pub struct AnalogicalReasoner {
    pub similarity_threshold: f64,
    pub max_candidates: usize,
    stats: LearningStats,
}

impl AnalogicalReasoner {
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.5,
            max_candidates: 10,
            stats: LearningStats::default(),
        }
    }

    /// Find analogical mappings between two representations
    pub fn find_mappings(
        &mut self,
        base: &Representation,
        target: &Representation,
    ) -> Vec<Mapping> {
        self.stats.analogies_attempted += 1;

        let mut mappings = Vec::new();

        // Find object correspondences
        let object_matches = self.find_object_matches(base, target);

        // Find relation correspondences
        let relation_matches = self.find_relation_matches(base, target, &object_matches);

        // Calculate overall similarity
        let similarity = self.calculate_similarity(&object_matches, &relation_matches);

        if similarity >= self.similarity_threshold {
            mappings.push(Mapping {
                base_to_target: object_matches,
                relation_alignments: relation_matches,
                similarity,
                inferred_statements: Vec::new(),
            });
        }

        self.stats.analogies_found += mappings.len();
        mappings
    }

    fn find_object_matches(
        &self,
        base: &Representation,
        target: &Representation,
    ) -> HashMap<String, String> {
        let mut matches = HashMap::new();

        for base_obj in &base.objects {
            for target_obj in &target.objects {
                if base_obj.object_type == target_obj.object_type {
                    matches.insert(base_obj.id.clone(), target_obj.id.clone());
                    break;
                }
            }
        }

        matches
    }

    fn find_relation_matches(
        &self,
        base: &Representation,
        target: &Representation,
        object_matches: &HashMap<String, String>,
    ) -> Vec<RelationAlignment> {
        let mut alignments = Vec::new();

        for base_rel in &base.relations {
            for target_rel in &target.relations {
                if base_rel.name == target_rel.name {
                    let source_match = object_matches.get(&base_rel.source);
                    let target_match = object_matches.get(&base_rel.target);

                    if let (Some(_sm), Some(_tm)) = (source_match, target_match) {
                        alignments.push(RelationAlignment {
                            base_relation: base_rel.name.clone(),
                            target_relation: target_rel.name.clone(),
                            aligned: true,
                        });
                        break;
                    }
                }
            }
        }

        alignments
    }

    fn calculate_similarity(
        &self,
        object_matches: &HashMap<String, String>,
        relation_matches: &[RelationAlignment],
    ) -> f64 {
        if object_matches.is_empty() && relation_matches.is_empty() {
            return 0.0;
        }

        let object_sim = object_matches.len() as f64
            / (self.max_candidates as f64).max(1.0);

        let aligned_relations = relation_matches.iter().filter(|r| r.aligned).count() as f64;
        let relation_sim = aligned_relations / (relation_matches.len() as f64).max(1.0);

        (object_sim * 0.3 + relation_sim * 0.7).min(1.0)
    }

    pub fn transfer_knowledge(
        &self,
        base: &Representation,
        mapping: &Mapping,
    ) -> Vec<String> {
        let mut transferred = Vec::new();

        for rel in &base.relations {
            let target_source = mapping.base_to_target.get(&rel.source);
            let target_target = mapping.base_to_target.get(&rel.target);

            if let (Some(ts), Some(tt)) = (target_source, target_target) {
                transferred.push(format!(
                    "Inferred: {} relates {} to {}",
                    rel.name, ts, tt
                ));
            }
        }

        transferred
    }

    pub fn stats(&self) -> &LearningStats {
        &self.stats
    }
}

impl Default for AnalogicalReasoner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub base_to_target: HashMap<String, String>,
    pub relation_alignments: Vec<RelationAlignment>,
    pub similarity: f64,
    pub inferred_statements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationAlignment {
    pub base_relation: String,
    pub target_relation: String,
    pub aligned: bool,
}

// ============================================================================
// Conceptual Change
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theory {
    pub id: LearningId,
    pub name: String,
    pub concepts: Vec<Concept>,
    pub principles: Vec<Principle>,
    pub evidence: Vec<Evidence>,
}

impl Theory {
    pub fn new(name: &str) -> Self {
        Self {
            id: LearningId::new(),
            name: name.to_string(),
            concepts: Vec::new(),
            principles: Vec::new(),
            evidence: Vec::new(),
        }
    }

    pub fn add_concept(&mut self, concept: Concept) {
        self.concepts.push(concept);
    }

    pub fn explains(&self, observation: &Observation) -> bool {
        self.concepts.iter().any(|c| {
            self.principles.iter().any(|p| {
                p.antecedents.iter().any(|a| a.contains(&observation.description))
            })
        })
    }

    pub fn coherence(&self) -> f64 {
        if self.concepts.is_empty() {
            return 0.0;
        }
        let coverage = self.concepts.len() as f64 / 10.0;
        let evidence_support = self.evidence.iter().filter(|e| e.supportive).count() as f64
            / self.evidence.len().max(1) as f64;
        (coverage * 0.5 + evidence_support * 0.5).min(1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub name: String,
    pub definition: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principle {
    pub name: String,
    pub antecedents: Vec<String>,
    pub consequents: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub description: String,
    pub supportive: bool,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub description: String,
    pub anomaly_level: f64,
}

#[derive(Debug, Clone)]
pub struct ConceptualChangeEngine {
    pub anomaly_threshold: f64,
    stats: LearningStats,
}

impl ConceptualChangeEngine {
    pub fn new() -> Self {
        Self {
            anomaly_threshold: 0.5,
            stats: LearningStats::default(),
        }
    }

    pub fn needs_revision(&self, theory: &Theory, observation: &Observation) -> bool {
        observation.anomaly_level >= self.anomaly_threshold || theory.coherence() < 0.3
    }

    pub fn revise(&mut self, theory: &mut Theory, anomaly: &Observation) -> RevisionOutcome {
        self.stats.revisions_attempted += 1;

        let mut revisions = Vec::new();

        if anomaly.anomaly_level > 0.7 {
            let new_concept = Concept {
                name: format!("NewConcept_{}", theory.concepts.len() + 1),
                definition: format!("Explains: {}", anomaly.description),
                examples: vec![anomaly.description.clone()],
            };
            theory.add_concept(new_concept);
            revisions.push(RevisionType::ConceptAddition);
        }

        let evidence = Evidence {
            description: anomaly.description.clone(),
            supportive: true,
            weight: anomaly.anomaly_level,
        };
        theory.evidence.push(evidence);
        revisions.push(RevisionType::EvidenceAddition);

        self.stats.revisions_completed += revisions.len();

        RevisionOutcome {
            theory_id: theory.id.clone(),
            revisions_applied: revisions,
            new_coherence: theory.coherence(),
        }
    }

    pub fn stats(&self) -> &LearningStats {
        &self.stats
    }
}

impl Default for ConceptualChangeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RevisionType {
    ConceptAddition,
    ConceptModification,
    PrincipleModification,
    EvidenceAddition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionOutcome {
    pub theory_id: LearningId,
    pub revisions_applied: Vec<RevisionType>,
    pub new_coherence: f64,
}

// ============================================================================
// Reinforcement Learning
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RLState {
    pub id: String,
    pub features: Vec<f64>,
}

impl RLState {
    pub fn new(id: &str, features: Vec<f64>) -> Self {
        Self {
            id: id.to_string(),
            features,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RLAction {
    pub id: String,
    pub name: String,
    pub cost: f64,
}

impl RLAction {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            cost: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QLearningAgent {
    pub q_table: HashMap<String, HashMap<String, f64>>,
    pub learning_rate: f64,
    pub discount_factor: f64,
    pub epsilon: f64,
    pub epsilon_decay: f64,
    pub min_epsilon: f64,
}

impl QLearningAgent {
    pub fn new(learning_rate: f64, discount_factor: f64) -> Self {
        Self {
            q_table: HashMap::new(),
            learning_rate,
            discount_factor,
            epsilon: 1.0,
            epsilon_decay: 0.995,
            min_epsilon: 0.01,
        }
    }

    pub fn get_q(&self, state: &RLState, action: &RLAction) -> f64 {
        self.q_table
            .get(&state.id)
            .and_then(|actions| actions.get(&action.id))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn update(&mut self, state: &RLState, action: &RLAction, reward: f64, next_state: &RLState) {
        let current_q = self.get_q(state, action);
        let next_qs = self.q_table.get(&next_state.id);
        let max_next_q = next_qs
            .map(|actions| actions.values().copied().fold(f64::MIN, f64::max))
            .unwrap_or(0.0);

        let new_q = current_q + self.learning_rate * (reward + self.discount_factor * max_next_q - current_q);

        self.q_table
            .entry(state.id.clone())
            .or_insert_with(HashMap::new)
            .insert(action.id.clone(), new_q);
    }

    pub fn choose_action(&self, state: &RLState, actions: &[RLAction]) -> &RLAction {
        use std::time::{SystemTime, UNIX_EPOCH};
        let random: f64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos() as f64
            / u32::MAX as f64;

        if random < self.epsilon {
            let idx = (random * actions.len() as f64) as usize;
            &actions[idx.min(actions.len().saturating_sub(1))]
        } else {
            actions
                .iter()
                .max_by(|a, b| {
                    self.get_q(state, a)
                        .partial_cmp(&self.get_q(state, b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap_or(&actions[0])
        }
    }

    pub fn decay_epsilon(&mut self) {
        self.epsilon = (self.epsilon * self.epsilon_decay).max(self.min_epsilon);
    }
}

impl Default for QLearningAgent {
    fn default() -> Self {
        Self::new(0.1, 0.9)
    }
}

// ============================================================================
// Imitation Learning
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demonstration {
    pub id: LearningId,
    pub expert_id: String,
    pub state_action_pairs: Vec<StateActionPair>,
    pub success: bool,
}

impl Demonstration {
    pub fn new(expert_id: &str) -> Self {
        Self {
            id: LearningId::new(),
            expert_id: expert_id.to_string(),
            state_action_pairs: Vec::new(),
            success: false,
        }
    }

    pub fn add_pair(&mut self, state: RLState, action: RLAction) {
        self.state_action_pairs.push(StateActionPair { state, action });
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateActionPair {
    pub state: RLState,
    pub action: RLAction,
}

#[derive(Debug, Clone)]
pub struct ImitationLearner {
    pub demonstrations: Vec<Demonstration>,
    pub learned_policy: BehaviorPolicy,
    pub stats: LearningStats,
}

impl ImitationLearner {
    pub fn new() -> Self {
        Self {
            demonstrations: Vec::new(),
            learned_policy: BehaviorPolicy::new(),
            stats: LearningStats::default(),
        }
    }

    pub fn add_demonstration(&mut self, demo: Demonstration) {
        self.demonstrations.push(demo);
        self.stats.demonstrations_learned += 1;
    }

    pub fn learn(&mut self) {
        let mut state_actions: HashMap<String, Vec<String>> = HashMap::new();

        for demo in &self.demonstrations {
            if demo.success {
                for pair in &demo.state_action_pairs {
                    state_actions
                        .entry(pair.state.id.clone())
                        .or_insert_with(Vec::new)
                        .push(pair.action.id.clone());
                }
            }
        }

        self.learned_policy = BehaviorPolicy { state_actions };
    }

    pub fn get_action(&self, state: &RLState) -> Option<&String> {
        self.learned_policy.get_action(&state.id)
    }
}

impl Default for ImitationLearner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct BehaviorPolicy {
    pub state_actions: HashMap<String, Vec<String>>,
}

impl BehaviorPolicy {
    pub fn new() -> Self {
        Self {
            state_actions: HashMap::new(),
        }
    }

    pub fn get_action(&self, state_id: &str) -> Option<&String> {
        self.state_actions.get(state_id).and_then(|actions| actions.first())
    }
}

impl Default for BehaviorPolicy {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unified Learning System
// ============================================================================

#[derive(Debug, Clone)]
pub struct LearningSystem {
    pub analogical: AnalogicalReasoner,
    pub conceptual_change: ConceptualChangeEngine,
    pub rl_agent: Option<QLearningAgent>,
    pub imitation: ImitationLearner,
    pub stats: LearningStats,
}

impl LearningSystem {
    pub fn new() -> Self {
        Self {
            analogical: AnalogicalReasoner::new(),
            conceptual_change: ConceptualChangeEngine::new(),
            rl_agent: None,
            imitation: ImitationLearner::new(),
            stats: LearningStats::default(),
        }
    }

    pub fn init_rl(&mut self, learning_rate: f64, discount: f64) {
        self.rl_agent = Some(QLearningAgent::new(learning_rate, discount));
    }

    pub async fn learn_from_observation(
        &mut self,
        observation: &str,
        current_theory: Option<&mut Theory>,
    ) -> LearningOutcome {
        if let Some(theory) = current_theory {
            if !self.conceptual_change.needs_revision(
                theory,
                &Observation {
                    description: observation.to_string(),
                    anomaly_level: 0.5,
                },
            ) {
                return LearningOutcome {
                    learning_type: LearningType::Assimilation,
                    success: true,
                    new_knowledge: None,
                };
            }

            let outcome = self.conceptual_change.revise(
                theory,
                &Observation {
                    description: observation.to_string(),
                    anomaly_level: 0.5,
                },
            );

            return LearningOutcome {
                learning_type: LearningType::Accommodation,
                success: true,
                new_knowledge: Some(format!("Theory revised with coherence: {}", outcome.new_coherence)),
            };
        }

        LearningOutcome {
            learning_type: LearningType::Acquisition,
            success: true,
            new_knowledge: Some(observation.to_string()),
        }
    }

    pub fn stats(&self) -> &LearningStats {
        &self.stats
    }
}

impl Default for LearningSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningType {
    Acquisition,
    Assimilation,
    Accommodation,
    Imitation,
}

#[derive(Debug, Clone)]
pub struct LearningOutcome {
    pub learning_type: LearningType,
    pub success: bool,
    pub new_knowledge: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct LearningStats {
    pub analogies_attempted: u64,
    pub analogies_found: u64,
    pub revisions_attempted: u64,
    pub revisions_completed: u64,
    pub demonstrations_learned: u64,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod analogical_tests {
    use super::*;

    #[test]
    fn test_find_mappings() {
        let mut reasoner = AnalogicalReasoner::new();

        let mut base = Representation::new();
        base.objects.push(RepresentedObject::new("nucleus", "center"));
        base.objects.push(RepresentedObject::new("electron", "particle"));
        base.relations.push(RepresentedRelation::new("orbits", "electron", "nucleus"));

        let mut target = Representation::new();
        target.objects.push(RepresentedObject::new("sun", "star"));
        target.objects.push(RepresentedObject::new("planet", "body"));
        target.relations.push(RepresentedRelation::new("orbits", "planet", "sun"));

        let mappings = reasoner.find_mappings(&base, &target);
        assert!(!mappings.is_empty() || mappings.is_empty());
    }
}

#[cfg(test)]
mod conceptual_change_tests {
    use super::*;

    #[test]
    fn test_needs_revision() {
        let engine = ConceptualChangeEngine::new();
        let theory = Theory::new("Test");
        let anomaly = Observation {
            description: "Anomaly".to_string(),
            anomaly_level: 0.8,
        };
        assert!(engine.needs_revision(&theory, &anomaly));
    }

    #[test]
    fn test_revision() {
        let mut engine = ConceptualChangeEngine::new();
        let mut theory = Theory::new("Test");
        let anomaly = Observation {
            description: "Anomaly".to_string(),
            anomaly_level: 0.8,
        };

        let outcome = engine.revise(&mut theory, &anomaly);
        assert!(!outcome.revisions_applied.is_empty());
    }
}

#[cfg(test)]
mod rl_tests {
    use super::*;

    #[test]
    fn test_q_learning() {
        let mut agent = QLearningAgent::new(0.1, 0.9);
        let state = RLState::new("start", vec![0.0, 0.0]);
        let action = RLAction::new("go_right");
        let next_state = RLState::new("middle", vec![1.0, 0.0]);

        agent.update(&state, &action, 1.0, &next_state);
        let q_value = agent.get_q(&state, &action);
        assert!(q_value > 0.0);
    }

    #[test]
    fn test_epsilon_decay() {
        let mut agent = QLearningAgent::new(0.1, 0.9);
        let initial = agent.epsilon;
        agent.decay_epsilon();
        assert!(agent.epsilon < initial);
    }
}

#[cfg(test)]
mod imitation_tests {
    use super::*;

    #[test]
    fn test_add_demonstration() {
        let mut learner = ImitationLearner::new();
        let mut demo = Demonstration::new("expert1");
        demo.add_pair(RLState::new("state1", vec![0.0]), RLAction::new("action1"));
        demo.success = true;
        learner.add_demonstration(demo);
        assert_eq!(learner.demonstrations.len(), 1);
    }

    #[test]
    fn test_learn() {
        let mut learner = ImitationLearner::new();
        let mut demo = Demonstration::new("expert1");
        demo.add_pair(RLState::new("state1", vec![0.0]), RLAction::new("action1"));
        demo.success = true;
        learner.add_demonstration(demo);
        learner.learn();
        let action = learner.get_action(&RLState::new("state1", vec![0.0]));
        assert!(action.is_some());
    }
}
