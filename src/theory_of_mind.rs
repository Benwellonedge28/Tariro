// Tariro AGI - Theory of Mind Module
// Mental state attribution and social cognition

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Commitment level for intentions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentLevel {
    pub level: f32,
    pub reason: String,
}

// ============================================================================
// Theory of Mind Types
// ============================================================================

/// Theory of Mind engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoryOfMindEngine {
    pub engine_id: String,
    pub agents: HashMap<String, AgentModel>,
    pub relationships: Vec<Relationship>,
    pub social_knowledge: SocialKnowledge,
    pub mental_state_inference: MentalStateInference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentModel {
    pub agent_id: String,
    pub name: String,
    pub agent_type: AgentType,
    pub beliefs: BeliefSet,
    pub desires: DesireSet,
    pub intentions: IntentionSet,
    pub emotions: EmotionalState,
    pub personality: Personality,
    pub mental_model_version: usize,
    pub last_updated: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    Human,
    AI,
    Robot,
    Animal,
    Fictional,
    Abstract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeliefSet {
    pub beliefs: Vec<Belief>,
    pub belief_strengths: HashMap<String, f32>,
    pub nested_beliefs: Vec<NestedBelief>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Belief {
    pub belief_id: String,
    pub content: String,
    pub truth_value: f32,
    pub source: BeliefSource,
    pub justification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedBelief {
    pub agent_id: String,
    pub belief: Belief,
    pub nesting_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesireSet {
    pub desires: Vec<Desire>,
    pub intensity: HashMap<String, f32>,
    pub conflicts: Vec<DesireConflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desire {
    pub desire_id: String,
    pub content: String,
    pub goal_state: String,
    pub priority: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesireConflict {
    pub desire1: String,
    pub desire2: String,
    pub resolution: ConflictResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    Prioritization,
    Compromise,
    Suppression,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentionSet {
    pub intentions: Vec<Intention>,
    pub commitment_level: HashMap<String, CommitmentLevel>,
    pub plans: Vec<Plan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub intention_id: String,
    pub content: String,
    pub goal: String,
    pub temporal_extent: TemporalExtent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalExtent {
    Immediate,
    ShortTerm,
    LongTerm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub current_emotions: Vec<Emotion>,
    pub emotion_intensity: f32,
    pub mood: String,
    pub emotional_history: Vec<Emotion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emotion {
    pub emotion_type: String,
    pub intensity: f32,
    pub duration: Duration,
    pub cause: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub traits: Vec<PersonalityTrait>,
    pub big_five: BigFive,
    pub behavioral_patterns: Vec<BehavioralPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTrait {
    pub trait_name: String,
    pub score: f32,
    pub manifestations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigFive {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralPattern {
    pub pattern_name: String,
    pub trigger: String,
    pub behavior: String,
    pub frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub plan_id: String,
    pub steps: Vec<String>,
    pub confidence: f32,
}

// ============================================================================
// Social Knowledge
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialKnowledge {
    pub social_norms: Vec<SocialNorm>,
    pub cultural_knowledge: CulturalKnowledge,
    pub relationship_types: Vec<RelationshipTemplate>,
    pub communication_patterns: Vec<CommunicationPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    pub norm_id: String,
    pub description: String,
    pub context: String,
    pub enforcement: EnforcementMechanism,
    pub violation_consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementMechanism {
    Social,
    Legal,
    Moral,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalKnowledge {
    pub culture_id: String,
    pub culture_name: String,
    pub customs: Vec<Custom>,
    pub values: Vec<String>,
    pub taboos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Custom {
    pub name: String,
    pub description: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTemplate {
    pub template_id: String,
    pub relationship_type: String,
    pub expected_behaviors: Vec<String>,
    pub power_dynamic: PowerDynamic,
    pub obligations: Vec<Obligation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDynamic {
    pub direction: PowerDirection,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerDirection {
    Equal,
    AgentDominant,
    OtherDominant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obligation {
    pub from_agent: String,
    pub to_agent: String,
    pub obligation_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPattern {
    pub pattern_id: String,
    pub style: CommunicationStyle,
    pub directness: Directness,
    pub formality: Formality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommunicationStyle {
    Formal,
    Informal,
    Assertive,
    Passive,
    Aggressive,
    Diplomatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Directness {
    Direct,
    Indirect,
    Contextual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Formality {
    VeryFormal,
    Formal,
    Casual,
    VeryCasual,
}

// ============================================================================
// Mental State Inference
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalStateInference {
    pub inference_id: String,
    pub target_agent: String,
    pub inferred_state: InferredMentalState,
    pub evidence: Vec<Evidence>,
    pub confidence: f32,
    pub reasoning_type: ReasoningType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferredMentalState {
    pub beliefs: Vec<String>,
    pub desires: Vec<String>,
    pub intentions: Vec<String>,
    pub emotions: Vec<String>,
    pub goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub content: String,
    pub weight: f32,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    ObservedBehavior,
    VerbalStatement,
    FacialExpression,
    BodyLanguage,
    Contextual,
    Inferential,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasoningType {
    TheoryTheory,
    SimulationTheory,
    InteractionTheory,
    Mixed,
}

// ============================================================================
// Relationship
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub relationship_id: String,
    pub agent1: String,
    pub agent2: String,
    pub relationship_type: String,
    pub strength: f32,
    pub history: Vec<RelationshipEvent>,
    pub mutual_knowledge: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEvent {
    pub event_id: String,
    pub event_type: String,
    pub description: String,
    pub impact_on_relationship: f32,
    pub timestamp: Instant,
}

use std::time::{Duration, Instant};

// ============================================================================
// Implementation
// ============================================================================

impl TheoryOfMindEngine {
    pub fn new() -> Self {
        Self {
            engine_id: generate_id(),
            agents: HashMap::new(),
            relationships: Vec::new(),
            social_knowledge: SocialKnowledge {
                social_norms: vec![],
                cultural_knowledge: CulturalKnowledge {
                    culture_id: generate_id(),
                    culture_name: "Default".to_string(),
                    customs: vec![],
                    values: vec!["cooperation".to_string(), "honesty".to_string()],
                    taboos: vec![],
                },
                relationship_types: vec![],
                communication_patterns: vec![],
            },
            mental_state_inference: MentalStateInference {
                inference_id: generate_id(),
                target_agent: String::new(),
                inferred_state: InferredMentalState {
                    beliefs: vec![],
                    desires: vec![],
                    intentions: vec![],
                    emotions: vec![],
                    goals: vec![],
                },
                evidence: vec![],
                confidence: 0.0,
                reasoning_type: ReasoningType::TheoryTheory,
            },
        }
    }

    /// Create agent model
    pub fn create_agent(&mut self, agent_id: &str, name: &str, agent_type: AgentType) -> &mut AgentModel {
        let agent = AgentModel {
            agent_id: agent_id.to_string(),
            name: name.to_string(),
            agent_type,
            beliefs: BeliefSet {
                beliefs: vec![],
                belief_strengths: HashMap::new(),
                nested_beliefs: vec![],
            },
            desires: DesireSet {
                desires: vec![],
                intensity: HashMap::new(),
                conflicts: vec![],
            },
            intentions: IntentionSet {
                intentions: vec![],
                commitment_level: HashMap::new(),
                plans: vec![],
            },
            emotions: EmotionalState {
                current_emotions: vec![],
                emotion_intensity: 0.5,
                mood: "neutral".to_string(),
                emotional_history: vec![],
            },
            personality: Personality {
                traits: vec![],
                big_five: BigFive {
                    openness: 0.5,
                    conscientiousness: 0.5,
                    extraversion: 0.5,
                    agreeableness: 0.5,
                    neuroticism: 0.5,
                },
                behavioral_patterns: vec![],
            },
            mental_model_version: 1,
            last_updated: Instant::now(),
        };

        self.agents.insert(agent_id.to_string(), agent);
        self.agents.get_mut(agent_id).unwrap()
    }

    /// Update agent beliefs
    pub fn update_belief(&mut self, agent_id: &str, belief: Belief) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            // Check if belief exists
            let existing = agent.beliefs.beliefs.iter()
                .position(|b| b.content == belief.content);

            if let Some(idx) = existing {
                agent.beliefs.beliefs[idx] = belief;
            } else {
                agent.beliefs.beliefs.push(belief);
            }

            agent.beliefs.belief_strengths.insert(
                belief.belief_id.clone(),
                belief.truth_value
            );

            agent.mental_model_version += 1;
            agent.last_updated = Instant::now();
        }
    }

    /// Infer mental state from observations
    pub fn infer_mental_state(
        &self,
        agent_id: &str,
        observations: &[Observation],
    ) -> MentalStateInference {
        let agent = match self.agents.get(agent_id) {
            Some(a) => a,
            None => return self.default_inference(agent_id),
        };

        let mut inferred_state = InferredMentalState {
            beliefs: vec![],
            desires: vec![],
            intentions: vec![],
            emotions: vec![],
            goals: vec![],
        };

        let mut evidence = Vec::new();
        let mut confidence = 0.5;

        for obs in observations {
            match obs.observation_type {
                ObservationType::Behavior => {
                    // Infer from behavior
                    if obs.content.contains("said") {
                        inferred_state.beliefs.push(self.extract_belief_from_statement(&obs.content));
                        confidence += 0.1;
                    }
                    evidence.push(Evidence {
                        evidence_type: EvidenceType::ObservedBehavior,
                        content: obs.content.clone(),
                        weight: 0.8,
                        source: "direct_observation".to_string(),
                    });
                }
                ObservationType::Emotion => {
                    inferred_state.emotions.push(self.extract_emotion(&obs.content));
                    confidence += 0.15;
                    evidence.push(Evidence {
                        evidence_type: EvidenceType::FacialExpression,
                        content: obs.content.clone(),
                        weight: 0.9,
                        source: "emotional_cue".to_string(),
                    });
                }
                ObservationType::Goal => {
                    inferred_state.goals.push(obs.content.clone());
                    confidence += 0.1;
                }
            }
        }

        // Use personality to predict behavior
        let predicted_behavior = self.predict_behavior(&agent.personality);
        inferred_state.intentions.push(predicted_behavior);

        confidence = confidence.min(0.95);

        MentalStateInference {
            inference_id: generate_id(),
            target_agent: agent_id.to_string(),
            inferred_state,
            evidence,
            confidence,
            reasoning_type: ReasoningType::Mixed,
        }
    }

    fn default_inference(&self, agent_id: &str) -> MentalStateInference {
        MentalStateInference {
            inference_id: generate_id(),
            target_agent: agent_id.to_string(),
            inferred_state: InferredMentalState {
                beliefs: vec![],
                desires: vec![],
                intentions: vec![],
                emotions: vec![],
                goals: vec![],
            },
            evidence: vec![],
            confidence: 0.0,
            reasoning_type: ReasoningType::TheoryTheory,
        }
    }

    /// Extract belief from statement
    fn extract_belief_from_statement(&self, statement: &str) -> String {
        // Simple extraction - find the proposition after "said" or "stated"
        if let Some(pos) = statement.find("said") {
            let after = &statement[pos + 4..];
            after.trim().to_string()
        } else {
            statement.to_string()
        }
    }

    /// Extract emotion from observation
    fn extract_emotion(&self, content: &str) -> String {
        let content_lower = content.to_lowercase();

        if content_lower.contains("happy") || content_lower.contains("smile") {
            "joy".to_string()
        } else if content_lower.contains("sad") || content_lower.contains("cry") {
            "sadness".to_string()
        } else if content_lower.contains("angry") || content_lower.contains("frustrat") {
            "anger".to_string()
        } else if content_lower.contains("fear") || content_lower.contains("scared") {
            "fear".to_string()
        } else if content_lower.contains("surprise") {
            "surprise".to_string()
        } else {
            "neutral".to_string()
        }
    }

    /// Predict behavior from personality
    fn predict_behavior(&self, personality: &Personality) -> String {
        if personality.big_five.extraversion > 0.7 {
            "likely to engage socially".to_string()
        } else if personality.big_five.conscientiousness > 0.7 {
            "likely to be organized and planful".to_string()
        } else if personality.big_five.neuroticism > 0.7 {
            "may react emotionally".to_string()
        } else {
            "behavior is moderate".to_string()
        }
    }

    /// Create nested belief (what agent A thinks agent B believes)
    pub fn create_nested_belief(
        &mut self,
        agent_id: &str,
        target_agent_id: &str,
        belief: Belief,
    ) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.beliefs.nested_beliefs.push(NestedBelief {
                agent_id: target_agent_id.to_string(),
                belief,
                nesting_level: 1,
            });

            agent.mental_model_version += 1;
        }
    }

    /// Model relationship
    pub fn update_relationship(&mut self, agent1: &str, agent2: &str, event: RelationshipEvent) {
        // Find existing relationship
        let existing = self.relationships.iter()
            .position(|r| (r.agent1 == agent1 && r.agent2 == agent2) ||
                        (r.agent1 == agent2 && r.agent2 == agent1));

        if let Some(idx) = existing {
            self.relationships[idx].history.push(event);
            self.relationships[idx].strength += event.impact_on_relationship;
            self.relationships[idx].strength = self.relationships[idx].strength.clamp(-1.0, 1.0);
        } else {
            let relationship = Relationship {
                relationship_id: generate_id(),
                agent1: agent1.to_string(),
                agent2: agent2.to_string(),
                relationship_type: "unknown".to_string(),
                strength: 0.5 + event.impact_on_relationship,
                history: vec![event],
                mutual_knowledge: vec![],
            };
            self.relationships.push(relationship);
        }
    }

    /// Predict agent's response
    pub fn predict_response(
        &self,
        agent_id: &str,
        situation: &str,
    ) -> PredictedResponse {
        let agent = match self.agents.get(agent_id) {
            Some(a) => a,
            None => return PredictedResponse::default(),
        };

        // Use BDI model (Beliefs, Desires, Intentions)
        let desires = &agent.desires.desires;
        let current_intentions = &agent.intentions.intentions;

        // Find most relevant desire/intention
        let relevant_desire = desires.iter()
            .find(|d| situation.contains(&d.goal_state))
            .cloned();

        let response_type = if relevant_desire.is_some() {
            ResponseType::GoalDirected
        } else if !current_intentions.is_empty() {
            ResponseType::Planned
        } else {
            ResponseType::Reactive
        };

        PredictedResponse {
            response_id: generate_id(),
            response_type,
            predicted_action: format!("Agent will likely {}", response_type.action_verb()),
            confidence: 0.6,
            alternatives: vec![],
        }
    }

    /// Get agent model
    pub fn get_agent(&self, agent_id: &str) -> Option<&AgentModel> {
        self.agents.get(agent_id)
    }

    /// Get relationship between agents
    pub fn get_relationship(&self, agent1: &str, agent2: &str) -> Option<&Relationship> {
        self.relationships.iter()
            .find(|r| (r.agent1 == agent1 && r.agent2 == agent2) ||
                      (r.agent1 == agent2 && r.agent2 == agent1))
    }
}

impl Default for TheoryOfMindEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Observation from which to infer mental states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub observation_id: String,
    pub timestamp: Instant,
    pub observation_type: ObservationType,
    pub content: String,
    pub source: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObservationType {
    Behavior,
    Speech,
    Emotion,
    Goal,
    Intention,
}

/// Predicted response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedResponse {
    pub response_id: String,
    pub response_type: ResponseType,
    pub predicted_action: String,
    pub confidence: f32,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseType {
    GoalDirected,
    Planned,
    Reactive,
    Habitual,
}

impl ResponseType {
    fn action_verb(&self) -> &'static str {
        match self {
            ResponseType::GoalDirected => "pursue a goal",
            ResponseType::Planned => "execute a plan",
            ResponseType::Reactive => "react to the situation",
            ResponseType::Habitual => "follow a habit",
        }
    }
}

impl Default for PredictedResponse {
    fn default() -> Self {
        PredictedResponse {
            response_id: generate_id(),
            response_type: ResponseType::Reactive,
            predicted_action: "unknown".to_string(),
            confidence: 0.0,
            alternatives: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeliefSource {
    DirectExperience,
    Testimony,
    Inference,
    Authority,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("tom_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theory_of_mind_creation() {
        let tom = TheoryOfMindEngine::new();
        assert!(tom.agents.is_empty());
    }

    #[test]
    fn test_create_agent() {
        let mut tom = TheoryOfMindEngine::new();
        tom.create_agent("agent1", "Alice", AgentType::Human);

        assert!(tom.agents.contains_key("agent1"));
        assert_eq!(tom.agents.get("agent1").unwrap().name, "Alice");
    }

    #[test]
    fn test_update_relationship() {
        let mut tom = TheoryOfMindEngine::new();
        tom.create_agent("agent1", "Alice", AgentType::Human);
        tom.create_agent("agent2", "Bob", AgentType::Human);

        let event = RelationshipEvent {
            event_id: "evt1".to_string(),
            event_type: "cooperation".to_string(),
            description: "Worked together on project".to_string(),
            impact_on_relationship: 0.2,
            timestamp: Instant::now(),
        };

        tom.update_relationship("agent1", "agent2", event);
        let relationship = tom.get_relationship("agent1", "agent2").unwrap();
        assert!(relationship.strength > 0.5);
    }
}
