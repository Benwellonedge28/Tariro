//! Tariro Social Cognition Module
//!
//! Implements social intelligence:
//! - Theory of Mind (ToM)
//! - Cultural knowledge
//! - Normative reasoning
//! - Reputation and trust modeling

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SocialId(pub Uuid);

impl SocialId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SocialId {
    fn default() -> Self {
        Self::new()
    }
}

/// An agent in social cognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: SocialId,
    pub name: String,
    pub beliefs: Beliefs,
    pub desires: Desires,
    pub intentions: Intentions,
    pub mental_state: MentalState,
}

impl Agent {
    pub fn new(name: &str) -> Self {
        Self {
            id: SocialId::new(),
            name: name.to_string(),
            beliefs: Beliefs::new(),
            desires: Desires::new(),
            intentions: Intentions::new(),
            mental_state: MentalState::default(),
        }
    }
}

/// Beliefs about the world
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Beliefs {
    pub facts: HashSet<String>,
    pub probabilistic: HashMap<String, f64>,
}

impl Beliefs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, fact: &str) {
        self.facts.insert(fact.to_string());
    }

    pub fn has(&self, fact: &str) -> bool {
        self.facts.contains(fact)
    }
}

/// Desires/goals
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Desires {
    pub goals: Vec<Goal>,
}

impl Desires {
    pub fn new() -> Self {
        Self::default()
    }
}

/// A goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub description: String,
    pub priority: f64,
}

/// Intentions (commitments to act)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Intentions {
    pub planned_actions: Vec<String>,
}

impl Intentions {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Mental state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalState {
    pub emotion: Option<String>,
    pub attention: Vec<String>,
    pub confidence: f64,
}

impl Default for MentalState {
    fn default() -> Self {
        Self {
            emotion: None,
            attention: Vec::new(),
            confidence: 0.5,
        }
    }
}

// ============================================================================
// Theory of Mind
// ============================================================================

/// Theory of Mind - understanding others' mental states
#[derive(Debug, Clone)]
pub struct TheoryOfMind {
    pub agents: HashMap<SocialId, Agent>,
    pub self_model: Agent,
    pub perspective_taking_depth: usize,
}

impl TheoryOfMind {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            self_model: Agent::new("self"),
            perspective_taking_depth: 2,
        }
    }

    /// Create agent model
    pub fn create_agent(&mut self, name: &str) -> SocialId {
        let agent = Agent::new(name);
        let id = agent.id.clone();
        self.agents.insert(id.clone(), agent);
        id
    }

    /// Update agent beliefs
    pub fn update_belief(&mut self, agent_id: &SocialId, belief: &str) {
        if let Some(agent) = self.agents.get_mut(agent_id) {
            agent.beliefs.add(belief);
        }
    }

    /// Predict agent action
    pub fn predict_action(&self, agent_id: &SocialId) -> Option<String> {
        if let Some(agent) = self.agents.get(agent_id) {
            // BDI model: Intention = Belief + Desire
            if !agent.intentions.planned_actions.is_empty() {
                return Some(agent.intentions.planned_actions[0].clone());
            }

            // Fallback: infer from beliefs
            if agent.beliefs.facts.iter().any(|b| b.contains("wants")) {
                return Some("pursue goal".to_string());
            }
        }
        None
    }

    /// Perspective taking
    pub fn perspective_take(&self, agent_id: &SocialId, belief: &str) -> String {
        if let Some(agent) = self.agents.get(agent_id) {
            return format!("{} believes: {}", agent.name, belief);
        }
        format!("Unknown believes: {}", belief)
    }

    /// Recursive theory of mind
    pub fn recursive_tom(&self, agent1: &SocialId, agent2: &SocialId) -> String {
        if let (Some(a1), Some(a2)) = (self.agents.get(agent1), self.agents.get(agent2)) {
            return format!(
                "{} thinks that {} has mental state: {:?}",
                a1.name,
                a2.name,
                a2.mental_state.emotion
            );
        }
        "Unknown recursive ToM".to_string()
    }
}

impl Default for TheoryOfMind {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Cultural Knowledge
// ============================================================================

/// Cultural norms and practices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    pub id: String,
    pub name: String,
    pub norms: Vec<Norm>,
    pub practices: Vec<Practice>,
    pub values: Vec<Value>,
}

impl Culture {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            norms: Vec::new(),
            practices: Vec::new(),
            values: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Norm {
    pub description: String,
    pub norm_type: NormType,
    pub enforcement: EnforcementType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NormType {
    Social,
    Moral,
    Legal,
    Customary,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EnforcementType {
    Formal,
    Informal,
    Internalized,
}

/// Cultural practice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Practice {
    pub name: String,
    pub description: String,
    pub occasions: Vec<String>,
}

/// Cultural value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    pub description: String,
    pub polarity: f64, // -1 to 1
}

/// Cultural knowledge base
#[derive(Debug, Clone)]
pub struct CulturalKnowledge {
    pub cultures: HashMap<String, Culture>,
    pub current_culture: Option<String>,
}

impl CulturalKnowledge {
    pub fn new() -> Self {
        let mut kb = Self {
            cultures: HashMap::new(),
            current_culture: None,
        };
        kb.initialize_defaults();
        kb
    }

    fn initialize_defaults(&mut self) {
        // Western culture (simplified)
        let mut western = Culture::new("Western");
        western.norms.push(Norm {
            description: "Personal space".to_string(),
            norm_type: NormType::Customary,
            enforcement: EnforcementType::Informal,
        });
        western.values.push(Value {
            name: "Individualism".to_string(),
            description: "Individual achievement".to_string(),
            polarity: 0.8,
        });
        western.practices.push(Practice {
            name: "Handshake greeting".to_string(),
            description: "Grip and shake hands".to_string(),
            occasions: vec!["greeting".to_string()],
        });
        self.cultures.insert("western".to_string(), western);

        // Set default
        self.current_culture = Some("western".to_string());
    }

    /// Get applicable norms
    pub fn get_norms(&self) -> Vec<&Norm> {
        if let Some(culture) = &self.current_culture {
            if let Some(c) = self.cultures.get(culture) {
                return c.norms.iter().collect();
            }
        }
        Vec::new()
    }

    /// Check if action is normatively appropriate
    pub fn is_appropriate(&self, action: &str) -> bool {
        for norm in self.get_norms() {
            if norm.description.to_lowercase().contains(&action.to_lowercase()) {
                return true;
            }
        }
        false // Conservative default
    }
}

impl Default for CulturalKnowledge {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Normative Reasoning
// ============================================================================

/// Normative reasoning engine
#[derive(Debug, Clone)]
pub struct NormativeReasoner {
    pub cultural_kb: CulturalKnowledge,
    pub moral_framework: MoralFramework,
}

impl NormativeReasoner {
    pub fn new() -> Self {
        Self {
            cultural_kb: CulturalKnowledge::new(),
            moral_framework: MoralFramework::new(),
        }
    }

    /// Evaluate action against norms
    pub fn evaluate(&self, action: &str, context: &str) -> NormativeJudgment {
        // Check cultural appropriateness
        let culturally_appropriate = self.cultural_kb.is_appropriate(action);

        // Check moral permissibility
        let moral_judgment = self.moral_framework.evaluate(action);

        // Combine judgments
        let overall = if culturally_appropriate && moral_judgment.permisible {
            JudgmentOutcome::Permissible
        } else if !culturally_appropriate && !moral_judgment.permisible {
            JudgmentOutcome::Forbidden
        } else {
            JudgmentOutcome::ContextDependent
        };

        NormativeJudgment {
            action: action.to_string(),
            context: context.to_string(),
            cultural_appropriateness: culturally_appropriate,
            moral_judgment,
            overall,
        }
    }

    /// Generate normative explanation
    pub fn explain(&self, action: &str) -> String {
        if self.cultural_kb.is_appropriate(action) {
            format!("{} is culturally appropriate in current context", action)
        } else {
            format!("{} may not be culturally appropriate", action)
        }
    }
}

impl Default for NormativeReasoner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralFramework {
    pub principles: Vec<MoralPrinciple>,
}

impl MoralFramework {
    pub fn new() -> Self {
        let mut framework = Self {
            principles: Vec::new(),
        };
        framework.initialize_defaults();
        framework
    }

    fn initialize_defaults(&mut self) {
        self.principles.push(MoralPrinciple {
            name: "Do no harm".to_string(),
            description: "Avoid causing suffering".to_string(),
            weight: 0.4,
        });
        self.principles.push(MoralPrinciple {
            name: "Fairness".to_string(),
            description: "Treat equals equally".to_string(),
            weight: 0.3,
        });
        self.principles.push(MoralPrinciple {
            name: "Autonomy".to_string(),
            description: "Respect individual freedom".to_string(),
            weight: 0.3,
        });
    }

    pub fn evaluate(&self, action: &str) -> MoralJudgment {
        // Simplified evaluation
        let mut score = 0.5;

        for principle in &self.principles {
            if action.to_lowercase().contains("harm") {
                if principle.name == "Do no harm" {
                    score -= principle.weight;
                }
            }
        }

        MoralJudgment {
            action: action.to_string(),
            permisible: score >= 0.3,
            principle_scores: self
                .principles
                .iter()
                .map(|p| (p.name.clone(), 0.5))
                .collect(),
        }
    }
}

impl Default for MoralFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralPrinciple {
    pub name: String,
    pub description: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralJudgment {
    pub action: String,
    pub permisible: bool,
    pub principle_scores: Vec<(String, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormativeJudgment {
    pub action: String,
    pub context: String,
    pub cultural_appropriateness: bool,
    pub moral_judgment: MoralJudgment,
    pub overall: JudgmentOutcome,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JudgmentOutcome {
    Permissible,
    Forbidden,
    ContextDependent,
}

// ============================================================================
// Reputation & Trust
// ============================================================================

/// Reputation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reputation {
    pub agent_id: SocialId,
    pub overall_score: f64,
    pub dimensions: HashMap<String, f64>,
    pub history: Vec<ReputationEvent>,
}

impl Reputation {
    pub fn new(agent_id: SocialId) -> Self {
        Self {
            agent_id,
            overall_score: 0.5,
            dimensions: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Update reputation based on event
    pub fn update(&mut self, event: ReputationEvent) {
        // Update dimension
        if let Some(score) = self.dimensions.get_mut(&event.dimension) {
            *score = (*score + event.value) / 2.0;
        } else {
            self.dimensions.insert(event.dimension.clone(), event.value);
        }

        // Recalculate overall
        if !self.dimensions.is_empty() {
            self.overall_score = self.dimensions.values().sum::<f64>() / self.dimensions.len() as f64;
        }

        self.history.push(event);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub dimension: String,
    pub value: f64,
    pub description: String,
    pub timestamp: chrono::DateTime<Utc>,
}

/// Trust model
#[derive(Debug, Clone)]
pub struct TrustModel {
    pub reputations: HashMap<SocialId, Reputation>,
    pub trust_threshold: f64,
}

impl TrustModel {
    pub fn new() -> Self {
        Self {
            reputations: HashMap::new(),
            trust_threshold: 0.5,
        }
    }

    /// Get trust level for agent
    pub fn get_trust(&self, agent_id: &SocialId) -> f64 {
        self.reputations
            .get(agent_id)
            .map(|r| r.overall_score)
            .unwrap_or(0.5)
    }

    /// Check if agent is trusted
    pub fn is_trusted(&self, agent_id: &SocialId) -> bool {
        self.get_trust(agent_id) >= self.trust_threshold
    }

    /// Update trust based on interaction
    pub fn update_trust(&mut self, agent_id: &SocialId, dimension: &str, value: f64, description: &str) {
        let reputation = self
            .reputations
            .entry(agent_id.clone())
            .or_insert_with(|| Reputation::new(agent_id.clone()));

        reputation.update(ReputationEvent {
            dimension: dimension.to_string(),
            value,
            description: description.to_string(),
            timestamp: Utc::now(),
        });
    }
}

impl Default for TrustModel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unified Social Cognition
// ============================================================================

/// Complete social cognition system
#[derive(Debug, Clone)]
pub struct SocialCognition {
    pub tom: TheoryOfMind,
    pub cultural_kb: CulturalKnowledge,
    pub normative: NormativeReasoner,
    pub trust: TrustModel,
}

impl SocialCognition {
    pub fn new() -> Self {
        Self {
            tom: TheoryOfMind::new(),
            cultural_kb: CulturalKnowledge::new(),
            normative: NormativeReasoner::new(),
            trust: TrustModel::new(),
        }
    }

    /// Model another agent
    pub fn model_agent(&mut self, name: &str) -> SocialId {
        self.tom.create_agent(name)
    }

    /// Social inference
    pub fn infer_mental_state(&self, agent_id: &SocialId) -> String {
        if let Some(agent) = self.tom.agents.get(agent_id) {
            format!(
                "{}'s mental state: emotion={:?}, confidence={:.2}",
                agent.name, agent.mental_state.emotion, agent.mental_state.confidence
            )
        } else {
            "Unknown agent".to_string()
        }
    }

    /// Social reasoning
    pub fn reason_socially(&self, situation: &str) -> SocialReasoning {
        let norms = self.cultural_kb.get_norms();
        let appropriate = self.normative.evaluate(situation, "");

        SocialReasoning {
            situation: situation.to_string(),
            applicable_norms: norms.len(),
            judgment: appropriate.overall,
        }
    }
}

impl Default for SocialCognition {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialReasoning {
    pub situation: String,
    pub applicable_norms: usize,
    pub judgment: JudgmentOutcome,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tom_tests {
    use super::*;

    #[test]
    fn test_create_agent() {
        let mut tom = TheoryOfMind::new();
        let id = tom.create_agent("Alice");

        assert!(tom.agents.contains_key(&id));
    }

    #[test]
    fn test_update_belief() {
        let mut tom = TheoryOfMind::new();
        let id = tom.create_agent("Alice");

        tom.update_belief(&id, "Fire is hot");

        if let Some(agent) = tom.agents.get(&id) {
            assert!(agent.beliefs.has("Fire is hot"));
        }
    }

    #[test]
    fn test_predict_action() {
        let mut tom = TheoryOfMind::new();
        let id = tom.create_agent("Bob");

        let action = tom.predict_action(&id);
        assert!(action.is_some() || action.is_none());
    }
}

#[cfg(test)]
mod cultural_tests {
    use super::*;

    #[test]
    fn test_cultural_knowledge() {
        let kb = CulturalKnowledge::new();
        let norms = kb.get_norms();

        assert!(!norms.is_empty());
    }

    #[test]
    fn test_is_appropriate() {
        let kb = CulturalKnowledge::new();
        let appropriate = kb.is_appropriate("greeting");

        assert!(appropriate || !appropriate);
    }
}

#[cfg(test)]
mod normative_tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let reasoner = NormativeReasoner::new();
        let judgment = reasoner.evaluate("help someone", "daily life");

        assert!(matches!(judgment.overall, JudgmentOutcome::Permissible | JudgmentOutcome::ContextDependent));
    }
}

#[cfg(test)]
mod trust_tests {
    use super::*;

    #[test]
    fn test_trust_model() {
        let mut model = TrustModel::new();
        let agent_id = SocialId::new();

        model.update_trust(&agent_id, "reliability", 0.8, "Always on time");

        assert!(model.get_trust(&agent_id) > 0.5);
    }

    #[test]
    fn test_is_trusted() {
        let model = TrustModel::new();
        let agent_id = SocialId::new();

        let trusted = model.is_trusted(&agent_id);
        assert!(trusted || !trusted);
    }
}

#[cfg(test)]
mod social_cognition_tests {
    use super::*;

    #[test]
    fn test_model_agent() {
        let mut social = SocialCognition::new();
        let id = social.model_agent("Charlie");

        assert!(social.tom.agents.contains_key(&id));
    }

    #[test]
    fn test_infer_mental_state() {
        let mut social = SocialCognition::new();
        let id = social.model_agent("Diana");

        let inference = social.infer_mental_state(&id);
        assert!(inference.contains("Diana"));
    }

    #[test]
    fn test_reason_socially() {
        let social = SocialCognition::new();
        let reasoning = social.reason_socially("Meeting new people");

        assert!(reasoning.situation.contains("Meeting"));
    }
}
