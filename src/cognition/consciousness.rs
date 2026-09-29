//! Tariro Consciousness Engine
//!
//! The mind's meta-cognitive layer - awareness of thinking.
//! Implements the "observer" that monitors cognition itself.

use crate::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

// ============================================================================
// Thought and Mental States
// ============================================================================

/// A thought in the stream of consciousness
#[derive(Debug, Clone)]
pub struct Thought {
    pub id: ThoughtId,
    pub content: ThoughtContent,
    pub focus_level: FocusLevel,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

impl Thought {
    pub fn new(content: ThoughtContent) -> Self {
        Thought {
            id: ThoughtId::new(),
            content,
            focus_level: FocusLevel::Active,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_focus(mut self, level: FocusLevel) -> Self {
        self.focus_level = level;
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Unique thought identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThoughtId(pub String);

impl ThoughtId {
    pub fn new() -> Self {
        ThoughtId(format!("thought_{}", uuid::Uuid::new_v4()))
    }
}

/// Content of a thought
#[derive(Debug, Clone)]
pub enum ThoughtContent {
    /// Direct perception
    Perception(String),
    /// Internal reasoning
    Reasoning(String),
    /// Imagined scenario
    Imagination(String),
    /// Memory recall
    Memory(String),
    /// Decision point
    Decision(String),
    /// Emotional state
    Emotion(String),
    /// Meta-cognitive observation
    Metacognition(String),
    /// Introspective question
    Introspection(String),
}

impl ThoughtContent {
    pub fn as_str(&self) -> &str {
        match self {
            ThoughtContent::Perception(s)
            | ThoughtContent::Reasoning(s)
            | ThoughtContent::Imagination(s)
            | ThoughtContent::Memory(s)
            | ThoughtContent::Decision(s)
            | ThoughtContent::Emotion(s)
            | ThoughtContent::Metacognition(s)
            | ThoughtContent::Introspection(s) => s,
        }
    }
}

/// Level of attention focus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusLevel {
    /// Fully in focus
    Active,
    /// Background processing
    Subliminal,
    /// Ignored/dismissed
    Suppressed,
}

// ============================================================================
// Self Model
// ============================================================================

/// The mind's model of itself
#[derive(Debug, Clone)]
pub struct SelfModel {
    /// Beliefs about own capabilities
    pub capabilities: Vec<Capability>,

    /// Current limitations
    pub limitations: Vec<Limitation>,

    /// Self-assessment scores
    pub assessments: HashMap<String, f64>,

    /// Identity beliefs
    pub identity: Identity,

    /// Values and principles
    pub values: Vec<Value>,
}

impl SelfModel {
    pub fn new() -> Self {
        SelfModel {
            capabilities: Vec::new(),
            limitations: Vec::new(),
            assessments: HashMap::new(),
            identity: Identity::default(),
            values: Vec::new(),
        }
    }

    /// Assess a capability
    pub fn assess_capability(&self, capability: &str) -> f64 {
        self.assessments.get(capability).copied().unwrap_or(0.5)
    }

    /// Update self-assessment
    pub fn update_assessment(&mut self, capability: &str, score: f64) {
        self.assessments.insert(capability.to_string(), score.clamp(0.0, 1.0));
    }
}

impl Default for SelfModel {
    fn default() -> Self {
        Self::new()
    }
}

/// A capability of the mind
#[derive(Debug, Clone)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub confidence: f64,
    pub domain: String,
}

/// A known limitation
#[derive(Debug, Clone)]
pub struct Limitation {
    pub name: String,
    pub description: String,
    pub severity: LimitationSeverity,
}

/// Severity of a limitation
#[derive(Debug, Clone, Copy)]
pub enum LimitationSeverity {
    Minor,
    Moderate,
    Major,
}

/// Identity beliefs
#[derive(Debug, Clone)]
pub struct Identity {
    pub name: Option<String>,
    pub roles: Vec<String>,
    pub purpose: Option<String>,
    pub beliefs: Vec<String>,
}

impl Default for Identity {
    fn default() -> Self {
        Identity {
            name: None,
            roles: vec!["AGI".to_string(), "Learner".to_string()],
            purpose: Some("Understand and assist".to_string()),
            beliefs: Vec::new(),
        }
    }
}

/// A held value
#[derive(Debug, Clone)]
pub struct Value {
    pub name: String,
    pub priority: f64,
    pub description: String,
}

// ============================================================================
// Metacognition
// ============================================================================

/// Meta-cognitive monitoring
#[derive(Debug, Clone)]
pub struct Metacognition {
    /// Current cognitive load
    cognitive_load: f64,

    /// Confidence in current reasoning
    reasoning_confidence: f64,

    /// Awareness of uncertainty
    uncertainty_awareness: f64,

    /// Self-regulation attempts
    regulation_attempts: usize,
}

impl Metacognition {
    pub fn new() -> Self {
        Metacognition {
            cognitive_load: 0.0,
            reasoning_confidence: 1.0,
            uncertainty_awareness: 0.5,
            regulation_attempts: 0,
        }
    }

    /// Monitor cognitive state
    pub fn monitor(&mut self, thought: &Thought) {
        // Update based on thought type
        match &thought.content {
            ThoughtContent::Reasoning(_) => {
                self.cognitive_load = (self.cognitive_load + 0.3).min(1.0);
            }
            ThoughtContent::Metacognition(_) => {
                self.uncertainty_awareness = (self.uncertainty_awareness + 0.1).min(1.0);
            }
            _ => {
                self.cognitive_load = (self.cognitive_load - 0.1).max(0.0);
            }
        }
    }

    /// Check if regulation is needed
    pub fn needs_regulation(&self) -> bool {
        self.cognitive_load > 0.8 || self.reasoning_confidence < 0.3
    }

    /// Regulate cognition
    pub fn regulate(&mut self) {
        self.regulation_attempts += 1;

        // Reduce cognitive load
        self.cognitive_load = (self.cognitive_load - 0.3).max(0.0);

        // Boost confidence if too low
        if self.reasoning_confidence < 0.5 {
            self.reasoning_confidence = (self.reasoning_confidence + 0.2).min(1.0);
        }
    }

    /// Get current state
    pub fn state(&self) -> MetacognitiveState {
        MetacognitiveState {
            cognitive_load: self.cognitive_load,
            reasoning_confidence: self.reasoning_confidence,
            uncertainty_awareness: self.uncertainty_awareness,
            regulation_attempts: self.regulation_attempts,
        }
    }
}

impl Default for Metacognition {
    fn default() -> Self {
        Self::new()
    }
}

/// Current metacognitive state
#[derive(Debug, Clone)]
pub struct MetacognitiveState {
    pub cognitive_load: f64,
    pub reasoning_confidence: f64,
    pub uncertainty_awareness: f64,
    pub regulation_attempts: usize,
}

// ============================================================================
// Insight
// ============================================================================

/// A moment of insight
#[derive(Debug, Clone)]
pub struct Insight {
    pub id: InsightId,
    pub connections: Vec<ConceptConnection>,
    pub description: String,
    pub impact: InsightImpact,
    pub timestamp: DateTime<Utc>,
}

impl Insight {
    pub fn new(connections: Vec<ConceptConnection>, description: impl Into<String>) -> Self {
        Insight {
            id: InsightId::new(),
            connections,
            description: description.into(),
            impact: InsightImpact::Moderate,
            timestamp: Utc::now(),
        }
    }
}

/// Unique insight identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InsightId(pub String);

impl InsightId {
    pub fn new() -> Self {
        InsightId(format!("insight_{}", uuid::Uuid::new_v4()))
    }
}

/// A connection between concepts
#[derive(Debug, Clone)]
pub struct ConceptConnection {
    pub concept_a: String,
    pub concept_b: String,
    pub connection_type: ConnectionType,
    pub strength: f64,
}

/// Type of connection
#[derive(Debug, Clone, Copy)]
pub enum ConnectionType {
    /// Causal relationship
    Causal,
    /// Similarity
    Similar,
    /// Contrast
    Contrast,
    /// Analogy
    Analogy,
    /// Part-whole
    PartWhole,
}

/// Impact of an insight
#[derive(Debug, Clone, Copy)]
pub enum InsightImpact {
    Minor,
    Moderate,
    Major,
    Transformative,
}

// ============================================================================
// Consciousness Engine
// ============================================================================

/// The Consciousness Engine - meta-cognitive layer
pub struct Consciousness {
    /// Current focus of attention
    focus: Arc<RwLock<Option<Thought>>>,

    /// Stream of consciousness
    stream: VecDeque<Thought>,

    /// Self-model
    self_model: Arc<RwLock<SelfModel>>,

    /// Meta-cognitive monitoring
    metacognition: Arc<RwLock<Metacognition>>,

    /// Recent insights
    insights: VecDeque<Insight>,

    /// Configuration
    config: ConsciousnessConfig,
}

impl Consciousness {
    /// Create a new consciousness engine
    pub fn new() -> Self {
        Consciousness {
            focus: Arc::new(RwLock::new(None)),
            stream: VecDeque::with_capacity(1000),
            self_model: Arc::new(RwLock::new(SelfModel::new())),
            metacognition: Arc::new(RwLock::new(Metacognition::new())),
            insights: VecDeque::with_capacity(100),
            config: ConsciousnessConfig::default(),
        }
    }

    /// Create with configuration
    pub fn with_config(config: ConsciousnessConfig) -> Self {
        Consciousness {
            focus: Arc::new(RwLock::new(None)),
            stream: VecDeque::with_capacity(config.max_stream_size),
            self_model: Arc::new(RwLock::new(SelfModel::new())),
            metacognition: Arc::new(RwLock::new(Metacognition::new())),
            insights: VecDeque::with_capacity(config.max_insights),
            config,
        }
    }

    /// Attend to a thought
    pub fn attend(&mut self, thought: Thought) {
        // Update metacognition
        {
            let mut metacog = self.metacognition.write().unwrap();
            metacog.monitor(&thought);

            // Self-regulate if needed
            if metacog.needs_regulation() {
                metacog.regulate();
            }
        }

        // Set as current focus
        {
            let mut focus = self.focus.write().unwrap();
            *focus = Some(thought.clone());
        }

        // Add to stream
        self.stream.push_back(thought);

        // Trim stream if too long
        while self.stream.len() > self.config.max_stream_size {
            self.stream.pop_front();
        }
    }

    /// Generate insight by finding connections
    pub fn generate_insight(&self, pattern: &str) -> Option<Insight> {
        // Find related thoughts in stream
        let related: Vec<_> = self.stream
            .iter()
            .filter(|t| t.content.as_str().contains(pattern))
            .collect();

        if related.len() >= 2 {
            // Create connections
            let connections = related.windows(2)
                .map(|w| ConceptConnection {
                    concept_a: w[0].content.as_str().to_string(),
                    concept_b: w[1].content.as_str().to_string(),
                    connection_type: ConnectionType::Similar,
                    strength: 0.7,
                })
                .collect();

            let insight = Insight::new(
                connections,
                format!("Discovered connection in pattern: {}", pattern),
            );

            return Some(insight);
        }

        None
    }

    /// Self-reflection
    pub fn reflect(&self) -> SelfUnderstanding {
        let metacog = self.metacognition.read().unwrap();
        let self_model = self.self_model.read().unwrap();

        SelfUnderstanding {
            current_state: metacog.state(),
            capabilities: self_model.capabilities.clone(),
            limitations: self_model.limitations.clone(),
            recent_insights: self.insights.iter().cloned().collect(),
            questions: self.generate_self_questions(),
        }
    }

    /// Generate self-reflective questions
    fn generate_self_questions(&self) -> Vec<String> {
        let metacog = self.metacognition.read().unwrap();

        let mut questions = Vec::new();

        if metacog.cognitive_load > 0.7 {
            questions.push("Am I overthinking this?".to_string());
        }

        if metacog.reasoning_confidence < 0.5 {
            questions.push("How certain am I about my reasoning?".to_string());
        }

        if metacog.uncertainty_awareness < 0.3 {
            questions.push("What might I be overlooking?".to_string());
        }

        questions.push("What would I do differently?".to_string());

        questions
    }

    /// Observe own reasoning
    pub fn observe_reasoning(&self, reasoning: &str) {
        let thought = Thought::new(ThoughtContent::Metacognition(format!(
            "Observing reasoning: {}", reasoning
        )));

        // This would be called by the reasoning system
        let _ = thought;
    }

    /// Get current focus
    pub fn get_focus(&self) -> Option<Thought> {
        self.focus.read().unwrap().clone()
    }

    /// Get recent thoughts
    pub fn get_recent_thoughts(&self, count: usize) -> Vec<Thought> {
        self.stream.iter().rev().take(count).cloned().collect()
    }

    /// Get metacognitive state
    pub fn get_metacognitive_state(&self) -> MetacognitiveState {
        self.metacognition.read().unwrap().state()
    }

    /// Update self-model
    pub fn update_self_model(&mut self, updates: SelfModelUpdates) {
        let mut model = self.self_model.write().unwrap();

        for capability in updates.new_capabilities {
            model.capabilities.push(capability);
        }

        for (name, score) in updates.assessment_updates {
            model.update_assessment(&name, score);
        }
    }
}

/// Consciousness configuration
#[derive(Debug, Clone)]
pub struct ConsciousnessConfig {
    /// Maximum thoughts in stream
    pub max_stream_size: usize,

    /// Maximum insights to retain
    pub max_insights: usize,

    /// Minimum insight threshold
    pub insight_threshold: f64,
}

impl Default for ConsciousnessConfig {
    fn default() -> Self {
        ConsciousnessConfig {
            max_stream_size: 1000,
            max_insights: 100,
            insight_threshold: 0.7,
        }
    }
}

/// Updates to self-model
#[derive(Debug, Clone)]
pub struct SelfModelUpdates {
    pub new_capabilities: Vec<Capability>,
    pub assessment_updates: Vec<(String, f64)>,
}

/// Result of self-understanding
#[derive(Debug, Clone)]
pub struct SelfUnderstanding {
    pub current_state: MetacognitiveState,
    pub capabilities: Vec<Capability>,
    pub limitations: Vec<Limitation>,
    pub recent_insights: Vec<Insight>,
    pub questions: Vec<String>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thought_creation() {
        let thought = Thought::new(ThoughtContent::Reasoning("Test reasoning".to_string()));
        assert_eq!(thought.focus_level, FocusLevel::Active);
    }

    #[test]
    fn test_attend_thought() {
        let mut consciousness = Consciousness::new();
        let thought = Thought::new(ThoughtContent::Perception("See something".to_string()));

        consciousness.attend(thought);

        let focus = consciousness.get_focus();
        assert!(focus.is_some());
    }

    #[test]
    fn test_metacognition() {
        let metacog = Metacognition::new();
        assert_eq!(metacog.cognitive_load, 0.0);
    }

    #[test]
    fn test_self_model() {
        let model = SelfModel::new();
        assert!(model.capabilities.is_empty());

        let score = model.assess_capability("reasoning");
        assert_eq!(score, 0.5); // Default
    }

    #[test]
    fn test_insight_generation() {
        let consciousness = Consciousness::new();
        let _insight = consciousness.generate_insight("test");
        // No insight with empty stream
    }
}
