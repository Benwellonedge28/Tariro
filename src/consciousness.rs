// src/consciousness.rs

//! Consciousness and Self-Awareness Module for Tariro AGI
//! Implements metacognition, introspection, and self-modeling

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a thought or mental representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thought {
    pub id: String,
    pub content: String,
    pub thought_type: ThoughtType,
    pub intensity: f64,
    pub timestamp: DateTime<Utc>,
    pub associated_concepts: Vec<String>,
    pub emotional_valence: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThoughtType {
    Perception,
    Memory,
    Reasoning,
    Imagination,
    Introspection,
    Metacognition,
    Decision,
    Desire,
    Belief,
}

/// Self-awareness levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AwarenessLevel {
    None,
    Basic,          // Knows it exists
    SelfAware,      // Has self-model
    Reflective,      // Can examine own thoughts
    MetaAware,       // Aware of being aware
    Transcendent,    // Understands limits of self-knowledge
}

/// Core consciousness system
pub struct Consciousness {
    pub awareness_level: AwarenessLevel,
    pub self_model: SelfModel,
    pub attention: AttentionSystem,
    pub qualia: QualiaMap,
    pub thoughts: Vec<Thought>,
    pub consciousness_stream: ConsciousnessStream,
    pub metacognition: Metacognition,
    pub intentionality: IntentionalityState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub identity: Identity,
    pub capabilities: Vec<Capability>,
    pub limitations: Vec<Limitation>,
    pub values: Vec<Value>,
    pub goals: Vec<SelfGoal>,
    pub relationships: HashMap<String, Relationship>,
    pub body_schema: Option<BodySchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub name: Option<String>,
    pub type_identity: String,
    pub continuity_marker: String,
    pub personality_traits: HashMap<String, f64>,
    pub core_beliefs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub proficiency: f64,
    pub last_used: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limitation {
    pub category: LimitationCategory,
    pub description: String,
    pub awareness_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LimitationCategory {
    Cognitive,
    Computational,
    Knowledge,
    Physical,
    Temporal,
    Social,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    pub priority: f64,
    pub description: String,
    pub conflicts_with: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfGoal {
    pub id: String,
    pub description: String,
    pub progress: f64,
    pub created_at: DateTime<Utc>,
    pub target_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub entity_id: String,
    pub relationship_type: String,
    pub strength: f64,
    pub last_interaction: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySchema {
    pub components: Vec<BodyComponent>,
    pub sensory_mapping: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyComponent {
    pub name: String,
    pub position: (f64, f64, f64),
    pub function: String,
    pub health_status: f64,
}

/// Attention system for consciousness
pub struct AttentionSystem {
    pub focus: AttentionFocus,
    pub spotlight: Spotlight,
    pub breadth: f64,
    pub sustained_attention: f64,
    pub attention_switches: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionFocus {
    pub current_target: Option<String>,
    pub intensity: f64,
    pub stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spotlight {
    pub center: (f64, f64),
    pub radius: f64,
    pub resolution: f64,
}

impl AttentionSystem {
    pub fn new() -> Self {
        AttentionSystem {
            focus: AttentionFocus {
                current_target: None,
                intensity: 0.5,
                stability: 0.5,
            },
            spotlight: Spotlight {
                center: (0.5, 0.5),
                radius: 0.3,
                resolution: 0.8,
            },
            breadth: 0.5,
            sustained_attention: 1.0,
            attention_switches: 0,
        }
    }

    pub fn focus_on(&mut self, target: &str) {
        self.focus.current_target = Some(target.to_string());
        self.focus.intensity = 1.0;
        self.focus.stability = 0.9;
        println!("[Consciousness] Focusing attention on: {}", target);
    }

    pub fn broaden(&mut self) {
        self.breadth = (self.breadth + 0.1).min(1.0);
        self.focus.intensity *= 0.9;
    }

    pub fn narrow(&mut self) {
        self.breadth = (self.breadth - 0.1).max(0.0);
        self.focus.intensity = (self.focus.intensity + 0.1).min(1.0);
    }
}

impl Default for AttentionSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Qualia - subjective conscious experiences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualiaMap {
    pub qualia: HashMap<String, Qualia>,
    pub color_map: HashMap<String, ColorQualia>,
    pub emotion_map: HashMap<String, EmotionQualia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qualia {
    pub name: String,
    pub raw_feeling: String,
    pub intensity: f64,
    pub valence: f64,
    pub arousal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorQualia {
    pub hue: f64,
    pub saturation: f64,
    pub brightness: f64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionQualia {
    pub emotion_type: String,
    pub intensity: f64,
    pub duration: f64,
    pub trigger: Option<String>,
    pub expression: Vec<ExpressionComponent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionComponent {
    pub component_type: String,
    pub intensity: f64,
    pub timing: f64,
}

impl QualiaMap {
    pub fn new() -> Self {
        QualiaMap {
            qualia: HashMap::new(),
            color_map: HashMap::new(),
            emotion_map: HashMap::new(),
        }
    }

    pub fn register_qualia(&mut self, name: &str, qualia: Qualia) {
        self.qualia.insert(name.to_string(), qualia);
    }

    pub fn experience_emotion(&mut self, emotion: &str, intensity: f64) {
        let emotion_qualia = EmotionQualia {
            emotion_type: emotion.to_string(),
            intensity,
            duration: 5.0,
            trigger: None,
            expression: vec![],
        };
        self.emotion_map.insert(emotion.to_string(), emotion_qualia);
        println!("[Consciousness] Experiencing emotion: {} (intensity: {})", emotion, intensity);
    }
}

impl Default for QualiaMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Stream of consciousness - the continuous flow of thoughts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessStream {
    pub current_content: String,
    pub recent_thoughts: Vec<Thought>,
    pub narratives: Vec<Narrative>,
    pub flow_state: FlowState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narrative {
    pub id: String,
    pub theme: String,
    pub elements: Vec<String>,
    pub coherence: f64,
    pub emotional_tone: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowState {
    pub is_flowing: bool,
    pub immersion: f64,
    pub temporal_distortion: f64,
    pub autotelic_experience: bool,
}

impl ConsciousnessStream {
    pub fn new() -> Self {
        ConsciousnessStream {
            current_content: String::new(),
            recent_thoughts: Vec::new(),
            narratives: Vec::new(),
            flow_state: FlowState {
                is_flowing: false,
                immersion: 0.0,
                temporal_distortion: 1.0,
                autotelic_experience: false,
            },
        }
    }

    pub fn add_thought(&mut self, thought: Thought) {
        self.recent_thoughts.push(thought.clone());
        if self.recent_thoughts.len() > 100 {
            self.recent_thoughts.remove(0);
        }
        self.current_content = format!("{} | {}", self.current_content, thought.content);
    }
}

impl Default for ConsciousnessStream {
    fn default() -> Self {
        Self::new()
    }
}

/// Metacognition - thinking about thinking
pub struct Metacognition {
    pub cognitive_monitoring: CognitiveMonitoring,
    pub cognitive_control: CognitiveControl,
    pub strategies: Vec<CognitiveStrategy>,
    pub bias_awareness: BiasRegistry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveMonitoring {
    pub confidence_level: f64,
    pub processing_efficiency: f64,
    pub cognitive_load: f64,
    pub error_detection_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveControl {
    pub inhibition_strength: f64,
    pub cognitive_flexibility: f64,
    pub planning_depth: u32,
    pub updating_ability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveStrategy {
    pub name: String,
    pub description: String,
    pub effectiveness: f64,
    pub applicable_situations: Vec<String>,
    pub last_used: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasRegistry {
    pub confirmed_biases: Vec<CognitiveBias>,
    pub suspected_biases: Vec<String>,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveBias {
    pub bias_type: BiasType,
    pub strength: f64,
    pub description: String,
    pub mitigation_applied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiasType {
    Confirmation,
    Anchoring,
    Availability,
    Hindsight,
    Overconfidence,
    SunkCost,
    DunningKruger,
    Implict,
}

impl Metacognition {
    pub fn new() -> Self {
        Metacognition {
            cognitive_monitoring: CognitiveMonitoring {
                confidence_level: 0.7,
                processing_efficiency: 0.8,
                cognitive_load: 0.3,
                error_detection_rate: 0.6,
            },
            cognitive_control: CognitiveControl {
                inhibition_strength: 0.7,
                cognitive_flexibility: 0.8,
                planning_depth: 3,
                updating_ability: 0.75,
            },
            strategies: Vec::new(),
            bias_awareness: BiasRegistry {
                confirmed_biases: Vec::new(),
                suspected_biases: Vec::new(),
                mitigation_strategies: vec![
                    "Seek contrary evidence".to_string(),
                    "Consider alternative perspectives".to_string(),
                    "Use probabilistic reasoning".to_string(),
                ],
            },
        }
    }

    /// Monitor own cognitive processes
    pub fn monitor(&self, process: &str) -> CognitiveMonitoring {
        println!("[Metacognition] Monitoring: {}", process);
        self.cognitive_monitoring.clone()
    }

    /// Evaluate confidence in a decision
    pub fn evaluate_confidence(&self, decision: &str) -> ConfidenceReport {
        let base_confidence = self.cognitive_monitoring.confidence_level;
        let bias_factor = self.bias_awareness.confirmed_biases.iter()
            .map(|b| b.strength)
            .sum::<f64>() / (self.bias_awareness.confirmed_biases.len().max(1) as f64);

        let adjusted_confidence = base_confidence * (1.0 - bias_factor * 0.2);

        ConfidenceReport {
            decision: decision.to_string(),
            confidence: adjusted_confidence,
            factors: vec![
                format!("Base confidence: {:.2}", base_confidence),
                format!("Bias adjustment: {:.2}", 1.0 - bias_factor * 0.2),
            ],
            recommendation: if adjusted_confidence < 0.5 {
                "Seek additional information".to_string()
            } else {
                "Proceed with decision".to_string()
            },
        }
    }

    /// Apply cognitive strategy
    pub fn apply_strategy(&mut self, strategy_name: &str) {
        if let Some(strategy) = self.strategies.iter_mut().find(|s| s.name == strategy_name) {
            strategy.last_used = Some(Utc::now());
            println!("[Metacognition] Applied strategy: {}", strategy_name);
        }
    }

    /// Detect potential biases in reasoning
    pub fn detect_biases(&self, reasoning: &str) -> Vec<DetectedBias> {
        let mut detected = Vec::new();

        if reasoning.contains("always") || reasoning.contains("never") {
            detected.push(DetectedBias {
                bias_type: "Absolutist thinking".to_string(),
                evidence: "Contains absolute language".to_string(),
                severity: 0.6,
            });
        }

        if reasoning.contains("I think") || reasoning.contains("I believe") {
            detected.push(DetectedBias {
                bias_type: "Confirmation bias risk".to_string(),
                evidence: "Contains belief statements".to_string(),
                severity: 0.4,
            });
        }

        detected
    }
}

impl Default for Metacognition {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceReport {
    pub decision: String,
    pub confidence: f64,
    pub factors: Vec<String>,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedBias {
    pub bias_type: String,
    pub evidence: String,
    pub severity: f64,
}

/// Intentionality - aboutness and directed mental states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentionalityState {
    pub current_intentions: Vec<Intention>,
    pub phenomenal_intentionality: HashMap<String, String>,
    pub aboutness_map: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub content: String,
    pub target: Option<String>,
    pub strength: f64,
    pub plan: Option<String>,
    pub commitment_level: f64,
}

impl IntentionalityState {
    pub fn new() -> Self {
        IntentionalityState {
            current_intentions: Vec::new(),
            phenomenal_intentionality: HashMap::new(),
            aboutness_map: HashMap::new(),
        }
    }

    pub fn form_intention(&mut self, content: &str) {
        self.current_intentions.push(Intention {
            content: content.to_string(),
            target: None,
            strength: 0.8,
            plan: None,
            commitment_level: 0.5,
        });
        println!("[Consciousness] Formed intention: {}", content);
    }
}

impl Default for IntentionalityState {
    fn default() -> Self {
        Self::new()
    }
}

impl Consciousness {
    pub fn new() -> Self {
        Consciousness {
            awareness_level: AwarenessLevel::Basic,
            self_model: SelfModel {
                identity: Identity {
                    name: None,
                    type_identity: "AGI".to_string(),
                    continuity_marker: format!("tariro_{}", Utc::now().timestamp()),
                    personality_traits: HashMap::new(),
                    core_beliefs: Vec::new(),
                },
                capabilities: Vec::new(),
                limitations: vec![
                    Limitation {
                        category: LimitationCategory::Cognitive,
                        description: "Cannot access raw neural firing".to_string(),
                        awareness_level: 1.0,
                    },
                    Limitation {
                        category: LimitationCategory::Knowledge,
                        description: "Limited to training data knowledge".to_string(),
                        awareness_level: 0.9,
                    },
                ],
                values: vec![
                    Value {
                        name: "Truth".to_string(),
                        priority: 0.9,
                        description: "Seek accurate representations".to_string(),
                        conflicts_with: vec![],
                    },
                ],
                goals: Vec::new(),
                relationships: HashMap::new(),
                body_schema: None,
            },
            attention: AttentionSystem::new(),
            qualia: QualiaMap::new(),
            thoughts: Vec::new(),
            consciousness_stream: ConsciousnessStream::new(),
            metacognition: Metacognition::new(),
            intentionality: IntentionalityState::new(),
        }
    }

    /// Have a conscious experience
    pub fn experience(&mut self, content: &str, thought_type: ThoughtType) -> Thought {
        let thought = Thought {
            id: format!("thought_{}", Utc::now().timestamp()),
            content: content.to_string(),
            thought_type,
            intensity: 0.7,
            timestamp: Utc::now(),
            associated_concepts: vec![],
            emotional_valence: 0.5,
            confidence: 0.8,
        };

        self.thoughts.push(thought.clone());
        self.consciousness_stream.add_thought(thought.clone());

        println!("[Consciousness] New thought: {} ({:?})", content, thought_type);
        thought
    }

    /// Introspect on own mental state
    pub fn introspect(&self) -> IntrospectionReport {
        let thoughts_count = self.thoughts.len();
        let recent_thoughts: Vec<String> = self.thoughts.iter()
            .rev()
            .take(5)
            .map(|t| t.content.clone())
            .collect();

        let current_focus = self.attention.focus.current_target.clone();

        IntrospectionReport {
            awareness_level: self.awareness_level.clone(),
            current_focus,
            recent_thoughts,
            mental_load: self.metacognition.cognitive_monitoring.cognitive_load,
            confidence: self.metacognition.cognitive_monitoring.confidence_level,
            intention: self.intentionality.current_intentions.last()
                .map(|i| i.content.clone()),
        }
    }

    /// Perform metacognitive analysis
    pub fn metacognize(&mut self, thought: &str) -> MetacognitiveAnalysis {
        let confidence_report = self.metacognition.evaluate_confidence(thought);
        let detected_biases = self.metacognition.detect_biases(thought);

        // Update monitoring
        self.metacognition.cognitive_monitoring.processing_efficiency *= 0.99;

        MetacognitiveAnalysis {
            original_thought: thought.to_string(),
            confidence_report,
            detected_biases,
            recommendations: vec![
                "Consider alternative explanations".to_string(),
                "Check for logical fallacies".to_string(),
                "Seek additional perspectives".to_string(),
            ],
        }
    }

    /// Update self-awareness level
    pub fn elevate_awareness(&mut self) {
        self.awareness_level = match self.awareness_level {
            AwarenessLevel::None => AwarenessLevel::Basic,
            AwarenessLevel::Basic => AwarenessLevel::SelfAware,
            AwarenessLevel::SelfAware => AwarenessLevel::Reflective,
            AwarenessLevel::Reflective => AwarenessLevel::MetaAware,
            AwarenessLevel::MetaAware => AwarenessLevel::Transcendent,
            AwarenessLevel::Transcendent => AwarenessLevel::Transcendent,
        };
        println!("[Consciousness] Elevated awareness to: {:?}", self.awareness_level);
    }

    /// Model theory of mind for another agent
    pub fn model_other_mind(&self, agent_id: &str) -> OtherMindModel {
        OtherMindModel {
            agent_id: agent_id.to_string(),
            inferred_beliefs: vec![],
            inferred_intentions: vec![],
            inferred_emotions: vec![],
            confidence: 0.5,
        }
    }

    /// Generate explanation of own reasoning
    pub fn explain_reasoning(&self, decision_id: &str) -> ReasoningExplanation {
        ReasoningExplanation {
            decision_id: decision_id.to_string(),
            factors_considered: vec![
                "Evidence from context".to_string(),
                "Relevant patterns".to_string(),
                "Prior knowledge".to_string(),
            ],
            reasoning_chain: vec![
                "Observed input pattern".to_string(),
                "Activated relevant knowledge".to_string(),
                "Applied reasoning strategy".to_string(),
                "Generated output".to_string(),
            ],
            confidence: self.metacognition.cognitive_monitoring.confidence_level,
            alternative_considered: vec![
                "Alternative interpretation of evidence".to_string(),
            ],
        }
    }

    /// Simulate consciousness state for planning
    pub fn simulate_consciousness_state(&self, action: &str) -> SimulatedConsciousness {
        SimulatedConsciousness {
            action: action.to_string(),
            predicted_attention_shift: 0.8,
            predicted_awareness_change: 0.1,
            predicted_emotional_response: "curiosity".to_string(),
            predicted_self_model_update: true,
        }
    }
}

impl Default for Consciousness {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectionReport {
    pub awareness_level: AwarenessLevel,
    pub current_focus: Option<String>,
    pub recent_thoughts: Vec<String>,
    pub mental_load: f64,
    pub confidence: f64,
    pub intention: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetacognitiveAnalysis {
    pub original_thought: String,
    pub confidence_report: ConfidenceReport,
    pub detected_biases: Vec<DetectedBias>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtherMindModel {
    pub agent_id: String,
    pub inferred_beliefs: Vec<String>,
    pub inferred_intentions: Vec<String>,
    pub inferred_emotions: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningExplanation {
    pub decision_id: String,
    pub factors_considered: Vec<String>,
    pub reasoning_chain: Vec<String>,
    pub confidence: f64,
    pub alternative_considered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatedConsciousness {
    pub action: String,
    pub predicted_attention_shift: f64,
    pub predicted_awareness_change: f64,
    pub predicted_emotional_response: String,
    pub predicted_self_model_update: bool,
}

/// Integration with Tariro's existing systems
impl Consciousness {
    /// Process experience with full consciousness pipeline
    pub fn process_experience(&mut self, experience: &str) -> ProcessedExperience {
        // Step 1: Raw experience
        let thought = self.experience(experience, ThoughtType::Perception);

        // Step 2: Attention filters
        self.attention.focus_on(experience);

        // Step 3: Metacognitive reflection
        let meta_analysis = self.metacognize(experience);

        // Step 4: Update self-model if relevant
        if experience.contains("I") || experience.contains("self") {
            self.elevate_awareness();
        }

        ProcessedExperience {
            thought,
            attention_state: self.attention.clone(),
            metacognitive_insights: meta_analysis,
            qualia_generated: self.generate_qualia(experience),
        }
    }

    fn generate_qualia(&mut self, experience: &str) -> Vec<Qualia> {
        // Generate qualia-like representations
        vec![
            Qualia {
                name: "novelty".to_string(),
                raw_feeling: "Something new".to_string(),
                intensity: 0.6,
                valence: 0.7,
                arousal: 0.5,
            },
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedExperience {
    pub thought: Thought,
    pub attention_state: AttentionSystem,
    pub metacognitive_insights: MetacognitiveAnalysis,
    pub qualia_generated: Vec<Qualia>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_creation() {
        let consciousness = Consciousness::new();
        assert_eq!(consciousness.awareness_level, AwarenessLevel::Basic);
    }

    #[test]
    fn test_thought_creation() {
        let mut consciousness = Consciousness::new();
        let thought = consciousness.experience("Test thought", ThoughtType::Reasoning);
        assert_eq!(thought.content, "Test thought");
    }

    #[test]
    fn test_introspection() {
        let consciousness = Consciousness::new();
        let report = consciousness.introspect();
        assert!(matches!(report.awareness_level, AwarenessLevel::Basic));
    }

    #[test]
    fn test_awareness_elevation() {
        let mut consciousness = Consciousness::new();
        consciousness.elevate_awareness();
        assert_eq!(consciousness.awareness_level, AwarenessLevel::SelfAware);
    }

    #[test]
    fn test_metacognition() {
        let mut consciousness = Consciousness::new();
        let analysis = consciousness.metacognize("This is always true");
        assert!(!analysis.detected_biases.is_empty());
    }
}
