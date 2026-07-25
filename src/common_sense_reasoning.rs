// src/common_sense_reasoning.rs

//! Common Sense Reasoning Engine for Tariro AGI
//! Implements everyday reasoning about physical, social, and temporal concepts

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Main common sense reasoning engine
pub struct CommonSenseReasoning {
    pub physics_knowledge: PhysicsKnowledge,
    pub social_knowledge: SocialKnowledge,
    pub temporal_knowledge: TemporalKnowledge,
    pub naive_physics: NaivePhysics,
    pub naive_psychology: NaivePsychology,
    pub reasoning_chains: Vec<ReasoningChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsKnowledge {
    pub objects: HashMap<String, PhysicalObjectSchema>,
    pub substances: HashMap<String, SubstanceSchema>,
    pub spatial_relations: Vec<SpatialRelation>,
    pub causal_physics: Vec<CausalPhysicsRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalObjectSchema {
    pub name: String,
    pub typical_size: SizeRange,
    pub typical_weight: f64,
    pub states: Vec<String>,
    pub affordances: Vec<String>,
    pub can_contain: bool,
    pub is_container: bool,
    pub is_rigid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeRange {
    pub min_cm: f64,
    pub max_cm: f64,
    pub typical: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceSchema {
    pub name: String,
    pub states: Vec<SubstanceState>,
    pub properties: HashMap<String, PropertySpec>,
    pub interactions: Vec<SubstanceInteraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstanceState {
    Solid,
    Liquid,
    Gas,
    Plasma,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySpec {
    pub name: String,
    pub typical_value: f64,
    pub unit: String,
    pub range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstanceInteraction {
    pub with: String,
    pub result: String,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialRelation {
    pub relation_type: SpatialRelationType,
    pub examples: Vec<String>,
    pub inference_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpatialRelationType {
    Above,
    Below,
    Inside,
    Beside,
    Behind,
    InFront,
    OnTop,
    Connected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalPhysicsRule {
    pub condition: String,
    pub effect: String,
    pub certainty: f64,
    pub counter_examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialKnowledge {
    pub agent_stereotypes: HashMap<String, AgentSchema>,
    pub social_norms: Vec<SocialNorm>,
    pub institution_schemas: HashMap<String, InstitutionSchema>,
    pub social_roles: HashMap<String, SocialRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSchema {
    pub name: String,
    pub typical_goals: Vec<String>,
    pub typical_behaviors: Vec<String>,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    pub domain: String,
    pub norm_type: NormCategory,
    pub description: String,
    pub violation_consequences: Vec<String>,
    pub enforcement: EnforcementType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NormCategory {
    Customary,
    Moral,
    Legal,
    Procedural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnforcementType {
    Formal,
    Informal,
    SelfEnforced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionSchema {
    pub name: String,
    pub purpose: String,
    pub roles: Vec<String>,
    pub rules: Vec<String>,
    pub typical_processes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialRole {
    pub name: String,
    pub associated_agents: Vec<String>,
    pub responsibilities: Vec<String>,
    pub expected_behaviors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalKnowledge {
    pub time_units: HashMap<String, TimeUnit>,
    pub event_durations: HashMap<String, DurationSpec>,
    pub sequence_patterns: Vec<SequencePattern>,
    pub causality_rules: Vec<TemporalCausalityRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeUnit {
    pub name: String,
    pub seconds: f64,
    pub subdivisions: Vec<String>,
    pub super_units: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationSpec {
    pub event_type: String,
    pub typical_duration: f64,
    pub unit: String,
    pub variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencePattern {
    pub pattern_type: String,
    pub steps: Vec<String>,
    pub examples: Vec<String>,
    pub flexibility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalCausalityRule {
    pub cause_type: String,
    pub effect_type: String,
    pub typical_delay: f64,
    pub delay_variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaivePhysics {
    pub object_permanence: bool,
    pub gravity_knowledge: GravityKnowledge,
    pub support_knowledge: Vec<SupportRule>,
    pub shape_knowledge: HashMap<String, ShapeProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityKnowledge {
    pub direction: (f64, f64, f64),
    pub strength: f64,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportRule {
    pub condition: String,
    pub stability: f64,
    pub toppling_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeProperty {
    pub shape_type: String,
    pub rolling: bool,
    pub stacking: bool,
    pub fitting: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaivePsychology {
    pub emotion_inference: bool,
    pub desire_attribution: bool,
    pub belief_attribution: bool,
    pub intention_attribution: bool,
    pub personality_types: HashMap<String, PersonalitySchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalitySchema {
    pub name: String,
    pub typical_behaviors: Vec<String>,
    pub typical_reactions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningChain {
    pub id: String,
    pub premise: String,
    pub steps: Vec<ReasoningStep>,
    pub conclusion: String,
    pub confidence: f64,
    pub reasoning_type: ReasoningType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_number: u32,
    pub rule_used: String,
    pub premise_used: Vec<String>,
    pub intermediate_result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReasoningType {
    Deductive,
    Inductive,
    Abductive,
    Analogical,
    Default,
}

impl CommonSenseReasoning {
    pub fn new() -> Self {
        CommonSenseReasoning {
            physics_knowledge: PhysicsKnowledge::default(),
            social_knowledge: SocialKnowledge::default(),
            temporal_knowledge: TemporalKnowledge::default(),
            naive_physics: NaivePhysics::default(),
            naive_psychology: NaivePsychology::default(),
            reasoning_chains: Vec::new(),
        }
    }

    /// Reason about physical causality
    pub fn reason_physical(&self, scenario: &str) -> PhysicalReasoningResult {
        println!("[CommonSense] Reasoning about physical scenario: {}", scenario);

        let mut predictions = Vec::new();

        // Check for gravity effects
        if scenario.contains("drop") || scenario.contains("fall") {
            predictions.push(Prediction {
                prediction: "Object will fall downward".to_string(),
                confidence: 0.95,
                rule_used: "gravity".to_string(),
            });
        }

        // Check for support effects
        if scenario.contains("remove") && scenario.contains("support") {
            predictions.push(Prediction {
                prediction: "Object above may fall".to_string(),
                confidence: 0.8,
                rule_used: "support_removal".to_string(),
            });
        }

        // Check for containment
        if scenario.contains("pour") || scenario.contains("liquid") {
            predictions.push(Prediction {
                prediction: "Liquid will flow to lowest point".to_string(),
                confidence: 0.9,
                rule_used: "liquid_physics".to_string(),
            });
        }

        PhysicalReasoningResult {
            scenario: scenario.to_string(),
            predictions,
            explanations: vec![
                "Based on naive physics principles".to_string(),
            ],
            confidence: predictions.iter().map(|p| p.confidence).sum::<f64>() / predictions.len().max(1) as f64,
        }
    }

    /// Reason about social situations
    pub fn reason_social(&self, scenario: &str) -> SocialReasoningResult {
        println!("[CommonSense] Reasoning about social scenario: {}", scenario);

        let mut inferences = Vec::new();

        // Infer emotions from context
        if scenario.contains("lost") || scenario.contains("failed") {
            inferences.push(SocialInference {
                inferred_state: "Negative emotion likely".to_string(),
                target: "person".to_string(),
                confidence: 0.7,
                reasoning: "Based on negative outcome".to_string(),
            });
        }

        // Infer intentions
        if scenario.contains("try") || scenario.contains("attempt") {
            inferences.push(SocialInference {
                inferred_state: "Goal-directed behavior".to_string(),
                target: "agent".to_string(),
                confidence: 0.8,
                reasoning: "Attempt implies intention".to_string(),
            });
        }

        // Infer social norms
        if scenario.contains("help") || scenario.contains("share") {
            inferences.push(SocialInference {
                inferred_state: "Positive social norm activated".to_string(),
                target: "interaction".to_string(),
                confidence: 0.85,
                reasoning: "Helping behavior is normatively positive".to_string(),
            });
        }

        SocialReasoningResult {
            scenario: scenario.to_string(),
            inferences,
            theory_of_mind: self.infer_mental_state(scenario),
            social_predictions: Vec::new(),
        }
    }

    fn infer_mental_state(&self, scenario: &str) -> MentalStateInference {
        let mut beliefs = Vec::new();
        let mut desires = Vec::new();
        let mut emotions = Vec::new();

        if scenario.contains("know") || scenario.contains("see") {
            beliefs.push("Has perceptual access to mentioned information".to_string());
        }

        if scenario.contains("want") || scenario.contains("need") || scenario.contains("desire") {
            desires.push("Has stated or implied goal".to_string());
        }

        if scenario.contains("happy") || scenario.contains("sad") || scenario.contains("angry") {
            emotions.push("Expressing or experiencing emotion".to_string());
        }

        MentalStateInference {
            beliefs,
            desires,
            intentions: Vec::new(),
            emotions,
            confidence: 0.6,
        }
    }

    /// Reason about time and sequences
    pub fn reason_temporal(&self, event1: &str, event2: &str, relation: &str) -> TemporalReasoningResult {
        println!("[CommonSense] Temporal reasoning: {} {} {}", event1, relation, event2);

        let (temporal_order, confidence) = match relation {
            "before" => ("E1 happens earlier than E2".to_string(), 0.9),
            "after" => ("E2 happens earlier than E1".to_string(), 0.9),
            "during" => ("Events overlap in time".to_string(), 0.7),
            "causes" => ("E1 is likely cause of E2".to_string(), 0.8),
            _ => ("Unknown temporal relation".to_string(), 0.3),
        };

        TemporalReasoningResult {
            event1: event1.to_string(),
            event2: event2.to_string(),
            relation: relation.to_string(),
            temporal_order,
            implied_causality: if relation == "causes" { Some(0.8) } else { None },
            duration_inference: self.infer_duration(event1, event2),
            confidence,
        }
    }

    fn infer_duration(&self, event1: &str, event2: &str) -> Option<String> {
        // Check for typical duration patterns
        let quick_events = ["blink", "think", "notice", "see"];
        let long_events = ["build", "learn", "travel", "work"];

        for event in [event1, event2] {
            for quick in &quick_events {
                if event.contains(quick) {
                    return Some("Very short (seconds)".to_string());
                }
            }
            for long in &long_events {
                if event.contains(long) {
                    return Some("Extended (minutes to hours)".to_string());
                }
            }
        }
        None
    }

    /// Make analogical reasoning
    pub fn reason_analogical(&self, source: &str, target: &str) -> AnalogicalReasoningResult {
        println!("[CommonSense] Analogical reasoning: {} -> {}", source, target);

        let mappings = self.find_structural_mappings(source, target);
        let analogical_strength = if mappings.len() > 3 { 0.8 } else { 0.4 };

        AnalogicalReasoningResult {
            source: source.to_string(),
            target: target.to_string(),
            mappings,
            inferred_knowledge: format!("Based on {} structural mappings", mappings.len()),
            analogical_strength,
            novelty: 0.5,
        }
    }

    fn find_structural_mappings(&self, source: &str, _target: &str) -> Vec<Mapping> {
        // Simplified mapping finding
        vec![
            Mapping {
                source_element: "A".to_string(),
                target_element: "X".to_string(),
                relation: "corresponds_to".to_string(),
                confidence: 0.7,
            },
            Mapping {
                source_element: "B".to_string(),
                target_element: "Y".to_string(),
                relation: "corresponds_to".to_string(),
                confidence: 0.6,
            },
        ]
    }

    /// Perform default reasoning (closed-world assumption with exceptions)
    pub fn reason_default(&self, statement: &str) -> DefaultReasoningResult {
        println!("[CommonSense] Default reasoning for: {}", statement);

        let (typical_truth, exceptions) = self.check_defaults(statement);

        DefaultReasoningResult {
            statement: statement.to_string(),
            default_truth: typical_truth,
            exceptions,
            reasoning_type: if exceptions.is_empty() {
                "Presumed true by default".to_string()
            } else {
                " defeasible - exceptions exist".to_string()
            },
        }
    }

    fn check_defaults(&self, statement: &str) -> (bool, Vec<String>) {
        // Check common defaults
        let defaults = [
            ("bird", "flies", true),
            ("fish", "swims", true),
            ("liquid", "flows", true),
            ("solid", "maintains_shape", true),
        ];

        for (category, property, typical) in &defaults {
            if statement.contains(category) && statement.contains(property) {
                return (*typical, vec![]);
            }
        }

        (true, Vec::new())
    }

    /// Answer common sense questions
    pub fn answer_question(&self, question: &str) -> AnswerResult {
        println!("[CommonSense] Answering: {}", question);

        let question_lower = question.to_lowercase();

        // Physics questions
        if question_lower.contains("why") && (question_lower.contains("fall") || question_lower.contains("drop")) {
            return AnswerResult {
                question: question.to_string(),
                answer: "Because of gravity pulling objects toward the ground".to_string(),
                confidence: 0.95,
                domain: "physics".to_string(),
            };
        }

        // Social questions
        if question_lower.contains("why") && question_lower.contains("sad") {
            return AnswerResult {
                question: question.to_string(),
                answer: "People feel sad when they experience loss, failure, or disappointment".to_string(),
                confidence: 0.8,
                domain: "social".to_string(),
            };
        }

        // Time questions
        if question_lower.contains("how long") {
            return AnswerResult {
                question: question.to_string(),
                answer: "Duration depends on specific activity and context".to_string(),
                confidence: 0.5,
                domain: "temporal".to_string(),
            };
        }

        AnswerResult {
            question: question.to_string(),
            answer: "I don't have enough context to answer definitively".to_string(),
            confidence: 0.3,
            domain: "general".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalReasoningResult {
    pub scenario: String,
    pub predictions: Vec<Prediction>,
    pub explanations: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub prediction: String,
    pub confidence: f64,
    pub rule_used: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialReasoningResult {
    pub scenario: String,
    pub inferences: Vec<SocialInference>,
    pub theory_of_mind: MentalStateInference,
    pub social_predictions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialInference {
    pub inferred_state: String,
    pub target: String,
    pub confidence: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalStateInference {
    pub beliefs: Vec<String>,
    pub desires: Vec<String>,
    pub intentions: Vec<String>,
    pub emotions: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalReasoningResult {
    pub event1: String,
    pub event2: String,
    pub relation: String,
    pub temporal_order: String,
    pub implied_causality: Option<f64>,
    pub duration_inference: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogicalReasoningResult {
    pub source: String,
    pub target: String,
    pub mappings: Vec<Mapping>,
    pub inferred_knowledge: String,
    pub analogical_strength: f64,
    pub novelty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub source_element: String,
    pub target_element: String,
    pub relation: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultReasoningResult {
    pub statement: String,
    pub default_truth: bool,
    pub exceptions: Vec<String>,
    pub reasoning_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerResult {
    pub question: String,
    pub answer: String,
    pub confidence: f64,
    pub domain: String,
}

impl Default for PhysicsKnowledge {
    fn default() -> Self {
        let mut objects = HashMap::new();
        objects.insert("chair".to_string(), PhysicalObjectSchema {
            name: "chair".to_string(),
            typical_size: SizeRange { min_cm: 30.0, max_cm: 100.0, typical: 50.0 },
            typical_weight: 5.0,
            states: vec!["sitting_on".to_string(), "stacked".to_string()],
            affordances: vec!["sit".to_string(), "move".to_string()],
            can_contain: false,
            is_container: false,
            is_rigid: true,
        });
        objects.insert("cup".to_string(), PhysicalObjectSchema {
            name: "cup".to_string(),
            typical_size: SizeRange { min_cm: 5.0, max_cm: 20.0, typical: 10.0 },
            typical_weight: 0.2,
            states: vec!["empty".to_string(), "filled".to_string()],
            affordances: vec!["drink_from".to_string(), "hold".to_string()],
            can_contain: true,
            is_container: true,
            is_rigid: true,
        });

        PhysicsKnowledge {
            objects,
            substances: HashMap::new(),
            spatial_relations: vec![
                SpatialRelation {
                    relation_type: SpatialRelationType::Above,
                    examples: vec!["sky above ground".to_string()],
                    inference_rules: vec!["Above implies not touching".to_string()],
                },
            ],
            causal_physics: vec![
                CausalPhysicsRule {
                    condition: "Object without support".to_string(),
                    effect: "Object falls".to_string(),
                    certainty: 0.95,
                    counter_examples: vec!["Floating objects".to_string(), "Helicopter".to_string()],
                },
            ],
        }
    }
}

impl Default for SocialKnowledge {
    fn default() -> Self {
        SocialKnowledge {
            agent_stereotypes: HashMap::new(),
            social_norms: vec![
                SocialNorm {
                    domain: "general".to_string(),
                    norm_type: NormCategory::Moral,
                    description: "Help others in need".to_string(),
                    violation_consequences: vec!["Social disapproval".to_string()],
                    enforcement: EnforcementType::Informal,
                },
            ],
            institution_schemas: HashMap::new(),
            social_roles: HashMap::new(),
        }
    }
}

impl Default for TemporalKnowledge {
    fn default() -> Self {
        let mut time_units = HashMap::new();
        time_units.insert("second".to_string(), TimeUnit {
            name: "second".to_string(),
            seconds: 1.0,
            subdivisions: vec!["millisecond".to_string(), "microsecond".to_string()],
            super_units: vec!["minute".to_string()],
        });
        time_units.insert("minute".to_string(), TimeUnit {
            name: "minute".to_string(),
            seconds: 60.0,
            subdivisions: vec!["second".to_string()],
            super_units: vec!["hour".to_string()],
        });

        TemporalKnowledge {
            time_units,
            event_durations: HashMap::new(),
            sequence_patterns: Vec::new(),
            causality_rules: vec![
                TemporalCausalityRule {
                    cause_type: "action".to_string(),
                    effect_type: "result".to_string(),
                    typical_delay: 1.0,
                    delay_variance: 0.5,
                },
            ],
        }
    }
}

impl Default for NaivePhysics {
    fn default() -> Self {
        let mut shape_knowledge = HashMap::new();
        shape_knowledge.insert("sphere".to_string(), ShapeProperty {
            shape_type: "sphere".to_string(),
            rolling: true,
            stacking: false,
            fitting: vec!["container_round".to_string()],
        });
        shape_knowledge.insert("cube".to_string(), ShapeProperty {
            shape_type: "cube".to_string(),
            rolling: false,
            stacking: true,
            fitting: vec!["container_square".to_string()],
        });

        NaivePhysics {
            object_permanence: true,
            gravity_knowledge: GravityKnowledge {
                direction: (0.0, -1.0, 0.0),
                strength: 1.0,
                exceptions: vec!["underwater".to_string(), "floating".to_string()],
            },
            support_knowledge: vec![
                SupportRule {
                    condition: "Object on flat surface".to_string(),
                    stability: 0.9,
                    toppling_threshold: 45.0,
                },
            ],
            shape_knowledge,
        }
    }
}

impl Default for NaivePsychology {
    fn default() -> Self {
        let mut personality_types = HashMap::new();
        personality_types.insert("introvert".to_string(), PersonalitySchema {
            name: "introvert".to_string(),
            typical_behaviors: vec!["prefers solitude".to_string(), "thinks before speaking".to_string()],
            typical_reactions: HashMap::new(),
        });

        NaivePsychology {
            emotion_inference: true,
            desire_attribution: true,
            belief_attribution: true,
            intention_attribution: true,
            personality_types,
        }
    }
}

impl Default for CommonSenseReasoning {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_reasoning() {
        let csr = CommonSenseReasoning::new();
        let result = csr.reason_physical("Drop ball from height");
        assert!(!result.predictions.is_empty());
    }

    #[test]
    fn test_social_reasoning() {
        let csr = CommonSenseReasoning::new();
        let result = csr.reason_social("Person lost their job");
        assert!(!result.inferences.is_empty());
    }

    #[test]
    fn test_temporal_reasoning() {
        let csr = CommonSenseReasoning::new();
        let result = csr.reason_temporal("rain", "wet_ground", "causes");
        assert!(result.implied_causality.is_some());
    }

    #[test]
    fn test_question_answering() {
        let csr = CommonSenseReasoning::new();
        let result = csr.answer_question("Why do things fall?");
        assert!(result.confidence > 0.5);
    }
}
