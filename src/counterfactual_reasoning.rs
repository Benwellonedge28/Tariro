// Tariro AGI - Counterfactual Reasoning Module
// Abductive reasoning about alternative possibilities

use std::collections::{HashMap, HashSet, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Counterfactual Types
// ============================================================================

/// Counterfactual query and reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualReasoning {
    pub reasoning_id: String,
    pub base_fact: Fact,
    pub antecedent: Condition,
    pub consequent: Consequent,
    pub causal_chain: Vec<CausalLink>,
    pub alternative_worlds: Vec<AlternativeWorld>,
    pub plausibility: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub fact_id: String,
    pub proposition: String,
    pub truth_value: bool,
    pub confidence: f32,
    pub source: String,
    pub causal_role: CausalRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub proposition: String,
    pub negation: bool,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub var_type: VariableType,
    pub domain: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariableType {
    Entity,
    Event,
    Property,
    Time,
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequent {
    pub proposition: String,
    pub expected_truth: bool,
    pub certainty: f32,
    pub causal_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub link_id: String,
    pub cause: String,
    pub effect: String,
    pub mechanism: String,
    pub strength: f32,
    pub type_: CausalLinkType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalLinkType {
    Necessary,
    Sufficient,
    NecessaryAndSufficient,
    Contributing,
    Enabling,
    Preventing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeWorld {
    pub world_id: String,
    pub modifications: Vec<Modification>,
    pub predicted_outcome: Outcome,
    pub similarity: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Modification {
    pub original: String,
    pub changed_to: String,
    pub scope: ModificationScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModificationScope {
    Single,
    Chain,
    Background,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub outcome_type: OutcomeType,
    pub description: String,
    pub probability: f32,
    pub causal_explanation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutcomeType {
    Same,
    Different,
    Improved,
    Worsened,
    Undetermined,
}

// ============================================================================
// Causal Graph
// ============================================================================

/// Causal reasoning engine
pub struct CounterfactualEngine {
    // Causal knowledge
    causal_graph: CausalGraph,

    // Counterfactual store
    counterfactuals: VecDeque<CounterfactualReasoning>,

    // Reasoning strategies
    strategies: Vec<ReasoningStrategy>,

    // Configuration
    config: CFConfig,

    // Statistics
    stats: CFStatistics,
}

#[derive(Debug, Clone, Default)]
pub struct CausalGraph {
    nodes: HashMap<String, CausalNode>,
    edges: HashMap<String, Vec<CausalEdge>>,
    reverse_edges: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct CausalNode {
    pub node_id: String,
    pub name: String,
    pub node_type: NodeType,
    pub properties: HashMap<String, String>,
    pub mechanisms: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Event,
    State,
    Action,
    Variable,
    Factor,
}

#[derive(Debug, Clone)]
pub struct CausalEdge {
    pub edge_id: String,
    pub source: String,
    pub target: String,
    pub edge_type: CausalLinkType,
    pub strength: f32,
    pub mechanism: String,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningStrategy {
    Additive,
    Subtractive,
    Replacement,
    Interpolation,
    Extrapolation,
}

#[derive(Debug, Clone)]
pub struct CFConfig {
    pub max_depth: usize,
    pub similarity_threshold: f32,
    pub enable_interpolation: bool,
    pub enable_abduction: bool,
    pub consider_side_effects: bool,
    pub min_plausibility: f32,
}

impl Default for CFConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            similarity_threshold: 0.5,
            enable_interpolation: true,
            enable_abduction: true,
            consider_side_effects: true,
            min_plausibility: 0.1,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CFStatistics {
    pub counterfactuals_generated: usize,
    pub worlds_explored: usize,
    pub causal_links_used: usize,
    pub avg_plausibility: f32,
}

// ============================================================================
// Implementation
// ============================================================================

impl CounterfactualEngine {
    pub fn new() -> Self {
        Self {
            causal_graph: CausalGraph::default(),
            counterfactuals: VecDeque::new(),
            strategies: vec![
                ReasoningStrategy::Additive,
                ReasoningStrategy::Subtractive,
                ReasoningStrategy::Replacement,
            ],
            config: CFConfig::default(),
            stats: CFStatistics::default(),
        }
    }

    /// Add causal knowledge
    pub fn add_causal_knowledge(&mut self, cause: &str, effect: &str, link_type: CausalLinkType, strength: f32) {
        let edge_id = generate_id();

        // Add edge
        let edge = CausalEdge {
            edge_id: edge_id.clone(),
            source: cause.to_string(),
            target: effect.to_string(),
            edge_type: link_type,
            strength,
            mechanism: "unknown".to_string(),
            conditions: vec![],
        };

        self.causal_graph.edges.entry(cause.to_string())
            .or_insert_with(Vec::new)
            .push(edge);

        self.causal_graph.reverse_edges.entry(effect.to_string())
            .or_insert_with(Vec::new)
            .push(cause.to_string());

        // Add nodes if not exist
        self.causal_graph.nodes.entry(cause.to_string())
            .or_insert_with(|| CausalNode {
                node_id: cause.to_string(),
                name: cause.to_string(),
                node_type: NodeType::Event,
                properties: HashMap::new(),
                mechanisms: vec![],
            });

        self.causal_graph.nodes.entry(effect.to_string())
            .or_insert_with(|| CausalNode {
                node_id: effect.to_string(),
                name: effect.to_string(),
                node_type: NodeType::Event,
                properties: HashMap::new(),
                mechanisms: vec![],
            });
    }

    /// Reason about counterfactual
    pub fn reason_counterfactual(&mut self, antecedent: &Condition, consequent: &str) -> CounterfactualReasoning {
        let reasoning_id = generate_id();

        // Find causal path from antecedent to consequent
        let causal_chain = self.find_causal_path(&antecedent.proposition, consequent);

        // Generate alternative worlds
        let alternative_worlds = self.generate_alternatives(&antecedent, consequent, &causal_chain);

        // Calculate plausibility
        let plausibility = self.calculate_plausibility(&causal_chain, &alternative_worlds);

        let counterfactual = CounterfactualReasoning {
            reasoning_id,
            base_fact: Fact {
                fact_id: generate_id(),
                proposition: antecedent.proposition.clone(),
                truth_value: !antecedent.negation,
                confidence: 1.0,
                source: "reasoning".to_string(),
                causal_role: CausalRole::Cause,
            },
            antecedent: antecedent.clone(),
            consequent: Consequent {
                proposition: consequent.to_string(),
                expected_truth: true,
                certainty: plausibility,
                causal_path: causal_chain.iter().map(|l| l.link_id.clone()).collect(),
            },
            causal_chain,
            alternative_worlds,
            plausibility,
        };

        // Store counterfactual
        self.counterfactuals.push_back(counterfactual.clone());
        if self.counterfactuals.len() > 1000 {
            self.counterfactuals.pop_front();
        }

        self.stats.counterfactuals_generated += 1;

        counterfactual
    }

    /// Find causal path between two events
    fn find_causal_path(&self, start: &str, end: &str) -> Vec<CausalLink> {
        let mut path = Vec::new();
        let mut visited = HashSet::new();

        self.find_causal_path_recursive(start, end, &mut path, &mut visited);

        path
    }

    fn find_causal_path_recursive(
        &self,
        current: &str,
        end: &str,
        path: &mut Vec<CausalLink>,
        visited: &mut HashSet<String>,
    ) -> bool {
        if current == end {
            return true;
        }

        visited.insert(current.to_string());

        if let Some(edges) = self.causal_graph.edges.get(current) {
            for edge in edges {
                if !visited.contains(&edge.target) {
                    let link = CausalLink {
                        link_id: edge.edge_id.clone(),
                        cause: edge.source.clone(),
                        effect: edge.target.clone(),
                        mechanism: edge.mechanism.clone(),
                        strength: edge.strength,
                        type_: edge.edge_type,
                    };

                    path.push(link);

                    if self.find_causal_path_recursive(&edge.target, end, path, visited) {
                        return true;
                    }

                    path.pop();
                }
            }
        }

        visited.remove(current);
        false
    }

    /// Generate alternative worlds
    fn generate_alternatives(
        &self,
        antecedent: &Condition,
        consequent: &str,
        causal_chain: &[CausalLink],
    ) -> Vec<AlternativeWorld> {
        let mut worlds = Vec::new();

        // Strategy 1: Minimal change
        worlds.push(self.generate_minimal_change_world(antecedent, consequent));

        // Strategy 2: Side effect consideration
        if self.config.consider_side_effects {
            worlds.push(self.generate_side_effect_world(causal_chain));
        }

        // Strategy 3: Counterfactual comparison
        worlds.push(self.generate_comparison_world(antecedent, consequent, causal_chain));

        // Filter by plausibility
        worlds.retain(|w| w.similarity >= self.config.similarity_threshold);

        self.stats.worlds_explored += worlds.len();

        worlds
    }

    /// Generate minimal change alternative world
    fn generate_minimal_change_world(&self, antecedent: &Condition, consequent: &str) -> AlternativeWorld {
        let modification = if antecedent.negation {
            Modification {
                original: antecedent.proposition.clone(),
                changed_to: format!("NOT {}", antecedent.proposition),
                scope: ModificationScope::Single,
            }
        } else {
            Modification {
                original: format!("NOT {}", antecedent.proposition),
                changed_to: antecedent.proposition.clone(),
                scope: ModificationScope::Single,
            }
        };

        AlternativeWorld {
            world_id: generate_id(),
            modifications: vec![modification],
            predicted_outcome: Outcome {
                outcome_type: OutcomeType::Different,
                description: format!("If {}, then {}", antecedent.proposition, consequent),
                probability: 0.7,
                causal_explanation: "Minimal change to antecedent".to_string(),
            },
            similarity: 0.9,
            description: "Minimal intervention world".to_string(),
        }
    }

    /// Generate world considering side effects
    fn generate_side_effect_world(&self, causal_chain: &[CausalLink]) -> AlternativeWorld {
        let mut modifications = Vec::new();

        for link in causal_chain {
            modifications.push(Modification {
                original: link.cause.clone(),
                changed_to: format!("MODIFIED_{}", link.cause),
                scope: ModificationScope::Chain,
            });
        }

        AlternativeWorld {
            world_id: generate_id(),
            modifications,
            predicted_outcome: Outcome {
                outcome_type: OutcomeType::Undetermined,
                description: "Multiple causal changes".to_string(),
                probability: 0.5,
                causal_explanation: "Chain of side effects".to_string(),
            },
            similarity: 0.6,
            description: "Side effects world".to_string(),
        }
    }

    /// Generate comparison world
    fn generate_comparison_world(
        &self,
        antecedent: &Condition,
        consequent: &str,
        causal_chain: &[CausalLink],
    ) -> AlternativeWorld {
        // Find the key causal link
        let key_link = causal_chain.first();

        AlternativeWorld {
            world_id: generate_id(),
            modifications: vec![],
            predicted_outcome: Outcome {
                outcome_type: OutcomeType::Same,
                description: "Causal chain maintained".to_string(),
                probability: 0.8,
                causal_explanation: key_link
                    .map(|l| format!("Key link: {} -> {}", l.cause, l.effect))
                    .unwrap_or_default(),
            },
            similarity: 0.85,
            description: "Baseline comparison world".to_string(),
        }
    }

    /// Calculate plausibility of counterfactual
    fn calculate_plausibility(&self, causal_chain: &[CausalLink], worlds: &[AlternativeWorld]) -> f32 {
        if causal_chain.is_empty() {
            return 0.0;
        }

        // Calculate causal strength
        let avg_strength: f32 = causal_chain.iter()
            .map(|l| l.strength)
            .sum::<f32>() / causal_chain.len() as f32;

        // Calculate world similarity
        let avg_similarity: f32 = worlds.iter()
            .map(|w| w.similarity)
            .sum::<f32>() / worlds.len().max(1) as f32;

        // Combine factors
        (avg_strength * 0.6 + avg_similarity * 0.4).max(0.0).min(1.0)
    }

    /// Abductive reasoning - infer best explanation
    pub fn abduce(&self, observation: &str) -> Vec<AbductiveExplanation> {
        let mut explanations = Vec::new();

        // Find potential causes
        if let Some(causes) = self.causal_graph.reverse_edges.get(observation) {
            for cause in causes {
                let explanation = AbductiveExplanation {
                    explanation_id: generate_id(),
                    observed_effect: observation.to_string(),
                    hypothesized_cause: cause.clone(),
                    confidence: self.calculate_abduction_confidence(cause, observation),
                    supporting_evidence: self.find_supporting_evidence(cause),
                    alternatives: vec![],
                };
                explanations.push(explanation);
            }
        }

        // Sort by confidence
        explanations.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        explanations
    }

    fn calculate_abduction_confidence(&self, cause: &str, effect: &str) -> f32 {
        if let Some(edges) = self.causal_graph.edges.get(cause) {
            edges.iter()
                .filter(|e| e.target == effect)
                .map(|e| e.strength)
                .fold(0.0f32, |max, val| max.max(val))
                .max(0.5)
        } else {
            0.5
        }
    }

    fn find_supporting_evidence(&self, cause: &str) -> Vec<String> {
        // Find all effects of this cause
        self.causal_graph.edges.get(cause)
            .map(|edges| edges.iter().map(|e| e.target.clone()).collect())
            .unwrap_or_default()
    }

    /// Answer "what if" questions
    pub fn what_if(&mut self, condition: &str, action: &str) -> Vec<Prediction> {
        let antecedent = Condition {
            condition_type: ConditionType::CausalAntecedent,
            proposition: condition.to_string(),
            negation: false,
            variables: vec![],
        };

        let mut predictions = Vec::new();

        // Find causal paths from action
        if let Some(edges) = self.causal_graph.edges.get(action) {
            for edge in edges {
                let path = self.find_causal_path(action, &edge.target);

                let prediction = Prediction {
                    prediction_id: generate_id(),
                    if_condition: condition.to_string(),
                    then_action: action.to_string(),
                    predicted_effect: edge.target.clone(),
                    probability: edge.strength,
                    causal_explanation: format!("{:?} causation: {} -> {}",
                        edge.edge_type, action, edge.target),
                    confidence: edge.strength,
                };

                predictions.push(prediction);
            }
        }

        predictions
    }

    /// Get counterfactual history
    pub fn get_history(&self) -> &[CounterfactualReasoning] {
        &self.counterfactuals
    }

    /// Get statistics
    pub fn get_stats(&self) -> &CFStatistics {
        &self.stats
    }
}

impl Default for CounterfactualEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Abductive explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbductiveExplanation {
    pub explanation_id: String,
    pub observed_effect: String,
    pub hypothesized_cause: String,
    pub confidence: f32,
    pub supporting_evidence: Vec<String>,
    pub alternatives: Vec<String>,
}

/// What-if prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub prediction_id: String,
    pub if_condition: String,
    pub then_action: String,
    pub predicted_effect: String,
    pub probability: f32,
    pub causal_explanation: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalRole {
    Cause,
    Effect,
    Mediating,
    Enabling,
    Preventing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    CausalAntecedent,
    NecessaryCondition,
    SufficientCondition,
    EnablingCondition,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("cf_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_knowledge() {
        let mut engine = CounterfactualEngine::new();

        engine.add_causal_knowledge("rain", "wet_ground", CausalLinkType::Sufficient, 0.9);

        assert!(engine.causal_graph.edges.contains_key("rain"));
    }

    #[test]
    fn test_counterfactual_reasoning() {
        let mut engine = CounterfactualEngine::new();

        engine.add_causal_knowledge("smoking", "cancer", CausalLinkType::Contributing, 0.7);
        engine.add_causal_knowledge("cancer", "death", CausalLinkType::Necessary, 0.8);

        let antecedent = Condition {
            condition_type: ConditionType::CausalAntecedent,
            proposition: "NOT smoking".to_string(),
            negation: true,
            variables: vec![],
        };

        let result = engine.reason_counterfactual(&antecedent, "NO cancer");

        assert!(result.causal_chain.len() >= 1);
    }
}
