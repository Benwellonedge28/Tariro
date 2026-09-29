//! Tariro Cognition Engine
//!
//! The integrated cognitive system combining consciousness, working memory,
//! world model, and motivation into a unified architecture.

use crate::prelude::*;
use crate::cognition::{
    consciousness::{Consciousness, Thought, ThoughtContent, FocusLevel},
    working_memory::{WorkingMemory, MentalItem, ItemContent, Context},
    world_model::{WorldModel, WorldEntity, WorldEntityType},
    motivation::{MotivationSystem, Goal, GoalType, Emotion, ValueAlignment},
};
use std::collections::HashMap;

// ============================================================================
// Cognitive State
// ============================================================================

/// The complete cognitive state
#[derive(Debug, Clone)]
pub struct CognitiveState {
    /// Current cognitive mode
    pub mode: CognitiveMode,

    /// Current focus
    pub focus: String,

    /// Cognitive load (0-1)
    pub load: f64,

    /// Overall readiness
    pub readiness: f64,

    /// Dominant motivation
    pub dominant_drive: Option<String>,

    /// Emotional state summary
    pub emotional_state: String,
}

impl CognitiveState {
    pub fn new() -> Self {
        CognitiveState {
            mode: CognitiveMode::Idle,
            focus: "None".to_string(),
            load: 0.0,
            readiness: 1.0,
            dominant_drive: None,
            emotional_state: "Neutral".to_string(),
        }
    }
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self::new()
    }
}

/// Cognitive processing mode
#[derive(Debug, Clone, Copy)]
pub enum CognitiveMode {
    Idle,
    Perception,
    Reasoning,
    Planning,
    Learning,
    Creating,
    Reflecting,
    Deciding,
}

// ============================================================================
// Cognitive Cycle
// ============================================================================

/// One cycle of cognition
#[derive(Debug, Clone)]
pub struct CognitiveCycle {
    pub cycle_id: u64,
    pub mode: CognitiveMode,
    pub inputs: Vec<CognitiveInput>,
    pub processing_steps: Vec<ProcessingStep>,
    pub outputs: Vec<CognitiveOutput>,
    pub duration_ms: u64,
}

/// Input to a cognitive cycle
#[derive(Debug, Clone)]
pub enum CognitiveInput {
    Perception(String),
    Question(String),
    Goal(Goal),
    Observation(String),
}

/// A step in cognitive processing
#[derive(Debug, Clone)]
pub struct ProcessingStep {
    pub step_number: usize,
    pub step_type: String,
    pub description: String,
    pub duration_ms: u64,
}

/// Output from a cognitive cycle
#[derive(Debug, Clone)]
pub enum CognitiveOutput {
    Decision(String),
    Action(String),
    Answer(String),
    Plan(String),
    Explanation(String),
}

// ============================================================================
// The Complete Cognition Engine
// ============================================================================

/// Configuration for the cognition engine
#[derive(Debug, Clone)]
pub struct CognitionConfig {
    /// Working memory capacity
    pub working_memory_capacity: usize,

    /// Thought stream size
    pub thought_stream_size: usize,

    /// Decay rate for working memory
    pub memory_decay: f64,

    /// Goal selection strategy
    pub goal_strategy: GoalSelectionStrategy,
}

impl Default for CognitionConfig {
    fn default() -> Self {
        CognitionConfig {
            working_memory_capacity: 7,
            thought_stream_size: 1000,
            memory_decay: 0.01,
            goal_strategy: GoalSelectionStrategy::HighestPriority,
        }
    }
}

/// Goal selection strategy
#[derive(Debug, Clone, Copy)]
pub enum GoalSelectionStrategy {
    HighestPriority,
    MostUrgent,
    MostProgress,
}

/// The complete Cognition Engine
pub struct CognitionEngine {
    /// Consciousness - meta-cognition
    consciousness: Consciousness,

    /// Working memory
    working_memory: WorkingMemory,

    /// World model
    world_model: WorldModel,

    /// Motivation system
    motivation: MotivationSystem,

    /// Configuration
    config: CognitionConfig,

    /// Current cognitive state
    state: CognitiveState,

    /// Cognitive cycles
    cycles: VecDeque<CognitiveCycle>,

    /// Statistics
    stats: CognitionStats,
}

impl CognitionEngine {
    /// Create a new cognition engine
    pub fn new() -> Self {
        let config = CognitionConfig::default();

        CognitionEngine {
            consciousness: Consciousness::with_config(crate::cognition::consciousness::ConsciousnessConfig {
                max_stream_size: config.thought_stream_size,
                max_insights: 100,
                insight_threshold: 0.7,
            }),
            working_memory: WorkingMemory::with_config(crate::cognition::working_memory::WorkingMemoryConfig {
                capacity: config.working_memory_capacity,
                decay_rate: config.memory_decay,
                ..Default::default()
            }),
            world_model: WorldModel::new(),
            motivation: MotivationSystem::new(),
            config,
            state: CognitiveState::new(),
            cycles: VecDeque::with_capacity(1000),
            stats: CognitionStats::default(),
        }
    }

    /// Process perception
    pub fn perceive(&mut self, perception: String) {
        self.state.mode = CognitiveMode::Perception;
        self.stats.total_perceptions += 1;

        // Add to working memory
        let item = MentalItem::new(ItemContent::Perception(
            crate::cognition::working_memory::Perception {
                modality: crate::cognition::working_memory::Modality::Internal,
                description: perception.clone(),
                features: vec![],
            }
        ));
        self.working_memory.store(item);

        // Add to consciousness
        self.consciousness.attend(Thought::new(ThoughtContent::Perception(perception)));
    }

    /// Think about something
    pub fn think(&mut self, thought: String) {
        self.state.mode = CognitiveMode::Reasoning;
        self.stats.total_thoughts += 1;

        // Add to working memory
        let item = MentalItem::new(ItemContent::Concept(
            crate::cognition::working_memory::Concept::new(&thought)
        ));
        self.working_memory.store(item);

        // Add to consciousness
        self.consciousness.attend(Thought::new(ThoughtContent::Reasoning(thought)));
    }

    /// Plan to achieve a goal
    pub fn plan(&mut self, goal_description: &str) -> Vec<String> {
        self.state.mode = CognitiveMode::Planning;
        self.stats.total_plans += 1;

        // Create goal
        let goal = Goal::new(goal_description, GoalType::Achievement)
            .with_priority(0.8);

        // Add to motivation system
        self.motivation.add_goal(goal);

        // Generate plan steps
        let plan = vec![
            format!("Step 1: Understand the goal: {}", goal_description),
            "Step 2: Gather relevant information".to_string(),
            "Step 3: Generate possible approaches".to_string(),
            "Step 4: Evaluate approaches".to_string(),
            "Step 5: Execute best approach".to_string(),
            "Step 6: Verify results".to_string(),
        ];

        // Add plan to consciousness
        for step in &plan {
            self.consciousness.attend(Thought::new(ThoughtContent::Reasoning(step.clone())));
        }

        plan
    }

    /// Make a decision
    pub fn decide(&mut self, options: Vec<String>) -> String {
        self.state.mode = CognitiveMode::Deciding;
        self.stats.total_decisions += 1;

        // Evaluate options against values
        let mut scored_options: Vec<(&String, f64)> = options.iter()
            .map(|opt| {
                let alignment = self.motivation.evaluate_action(opt);
                let score = match alignment {
                    ValueAlignment::HighlyAligned => 0.9,
                    ValueAlignment::SomewhatAligned => 0.7,
                    ValueAlignment::Neutral => 0.5,
                    ValueAlignment::Misaligned => 0.3,
                };
                (opt, score)
            })
            .collect();

        // Sort by score
        scored_options.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Select best option
        let decision = scored_options.first()
            .map(|(opt, _)| opt.clone())
            .unwrap_or_else(|| "No decision possible".to_string());

        // Generate emotion
        let emotion = self.motivation.generate_emotion(&format!("Decision: {}", decision));

        // Add to consciousness
        self.consciousness.attend(Thought::new(ThoughtContent::Decision(
            format!("Chose: {} (confidence: {:.2})", decision, emotion.intensity)
        )));

        decision
    }

    /// Learn something new
    pub fn learn(&mut self, knowledge: &str) {
        self.state.mode = CognitiveMode::Learning;
        self.stats.total_learning += 1;

        // Add entity to world model
        let entity = WorldEntity::new(
            format!("Learned: {}", knowledge),
            WorldEntityType::Abstract,
        );
        self.world_model.add_entity(entity);

        // Add to working memory
        let item = MentalItem::new(ItemContent::Memory(
            crate::cognition::working_memory::Memory {
                description: knowledge.to_string(),
                emotional_valence: 0.5,
                importance: 0.7,
                context: "Learning".to_string(),
            }
        ));
        self.working_memory.store(item);

        // Satisfy curiosity drive
        if let Some(goal) = self.motivation.get_current_goal() {
            self.motivation.update_goal_progress(&goal.id, 0.2);
        }
    }

    /// Imagine a scenario
    pub fn imagine(&mut self, scenario: &str) -> SimulationResult {
        self.stats.total_imaginations += 1;

        // Add to consciousness
        self.consciousness.attend(Thought::new(ThoughtContent::Imagination(scenario.to_string())));

        // Create simulated action
        let action = crate::cognition::world_model::SimulatedAction {
            target: crate::cognition::world_model::EntityId::new(),
            action_type: "Imagine".to_string(),
            parameters: HashMap::new(),
            initial_state: crate::cognition::world_model::SimulatedState::new(),
        };

        // Simulate
        self.world_model.predict(action)
    }

    /// Reflect on thoughts
    pub fn reflect(&mut self) -> SelfReflection {
        self.state.mode = CognitiveMode::Reflecting;
        self.stats.total_reflections += 1;

        let understanding = self.consciousness.reflect();

        SelfReflection {
            thoughts_examined: self.consciousness.get_recent_thoughts(10).len(),
            metacognitive_state: format!("{:?}", understanding.current_state),
            self_questions: understanding.questions,
            insights: understanding.recent_insights.len(),
        }
    }

    /// Get current cognitive state
    pub fn get_state(&self) -> &CognitiveState {
        &self.state
    }

    /// Update cognitive state
    pub fn update_state(&mut self) {
        // Update load based on working memory
        self.state.load = self.working_memory.capacity_usage();

        // Update readiness
        self.state.readiness = 1.0 - (self.state.load * 0.5);

        // Update focus
        if let Some(thought) = self.consciousness.get_focus() {
            self.state.focus = thought.content.as_str().to_string();
        }

        // Update dominant drive
        if let Some(drive) = self.motivation.get_most_urgent_drive() {
            self.state.dominant_drive = Some(drive.drive_type.name().to_string());
        }

        // Update emotional state
        if let Some(emotion) = self.motivation.get_current_emotion() {
            self.state.emotional_state = format!("{:?}", emotion.emotion_type);
        }

        // Decay drives
        self.motivation.decay_drives();

        // Decay working memory
        self.working_memory.decay();
    }

    /// Get consciousness
    pub fn consciousness(&self) -> &Consciousness {
        &self.consciousness
    }

    /// Get working memory
    pub fn working_memory(&self) -> &WorkingMemory {
        &self.working_memory
    }

    /// Get world model
    pub fn world_model(&self) -> &WorldModel {
        &self.world_model
    }

    /// Get motivation system
    pub fn motivation(&self) -> &MotivationSystem {
        &self.motivation
    }

    /// Get statistics
    pub fn stats(&self) -> &CognitionStats {
        &self.stats
    }
}

impl Default for CognitionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of reflection
#[derive(Debug, Clone)]
pub struct SelfReflection {
    pub thoughts_examined: usize,
    pub metacognitive_state: String,
    pub self_questions: Vec<String>,
    pub insights: usize,
}

/// Result of imagination/simulation
#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub scenario: String,
    pub predicted_outcomes: Vec<String>,
    pub confidence: f64,
}

/// Cognition statistics
#[derive(Debug, Clone, Default)]
pub struct CognitionStats {
    pub total_perceptions: usize,
    pub total_thoughts: usize,
    pub total_plans: usize,
    pub total_decisions: usize,
    pub total_learning: usize,
    pub total_imaginations: usize,
    pub total_reflections: usize,
}

// ============================================================================
// Integration with Knowledge
// ============================================================================

impl CognitionEngine {
    /// Think using knowledge
    pub fn reason_with_knowledge(&mut self, question: &str, knowledge_graph: &crate::knowledge::knowledge_graph::KnowledgeGraph) -> String {
        self.state.mode = CognitiveMode::Reasoning;
        self.stats.total_thoughts += 1;

        // Add question to consciousness
        self.consciousness.attend(Thought::new(ThoughtContent::Introspection(
            format!("Question: {}", question)
        )));

        // Query knowledge graph
        let query = crate::knowledge::reasoning::Query::new();
        let results = crate::knowledge::reasoning::ReasoningEngine::new(
            std::sync::Arc::new(std::sync::RwLock::new(knowledge_graph.clone()))
        ).deduce(&query);

        // Format answer
        let answer = if results.deductions.is_empty() {
            format!("Based on my knowledge, I don't have specific information about: {}", question)
        } else {
            format!("From my knowledge: Found {} relevant facts", results.deductions.len())
        };

        // Add answer to consciousness
        self.consciousness.attend(Thought::new(ThoughtContent::Reasoning(answer.clone())));

        answer
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognition_engine_creation() {
        let engine = CognitionEngine::new();
        assert_eq!(engine.state.mode, CognitiveMode::Idle);
    }

    #[test]
    fn test_perception() {
        let mut engine = CognitionEngine::new();
        engine.perceive("See a red ball".to_string());

        assert_eq!(engine.state.mode, CognitiveMode::Perception);
        assert!(engine.working_memory.len() > 0);
    }

    #[test]
    fn test_thinking() {
        let mut engine = CognitionEngine::new();
        engine.think("What is the meaning of life?".to_string());

        assert_eq!(engine.state.mode, CognitiveMode::Reasoning);
    }

    #[test]
    fn test_planning() {
        let mut engine = CognitionEngine::new();
        let plan = engine.plan("Learn to cook");

        assert!(!plan.is_empty());
        assert!(plan.len() >= 6);
    }

    #[test]
    fn test_decision() {
        let mut engine = CognitionEngine::new();
        let options = vec![
            "Do nothing".to_string(),
            "Study for exam".to_string(),
            "Go to party".to_string(),
        ];

        let decision = engine.decide(options);
        assert!(!decision.is_empty());
    }

    #[test]
    fn test_learning() {
        let mut engine = CognitionEngine::new();
        engine.learn("Rust is a systems programming language");

        assert!(engine.world_model.entities.len() > 0);
    }

    #[test]
    fn test_reflection() {
        let mut engine = CognitionEngine::new();
        engine.think("Test thought".to_string());

        let reflection = engine.reflect();
        assert!(reflection.thoughts_examined > 0);
    }

    #[test]
    fn test_state_update() {
        let mut engine = CognitionEngine::new();
        engine.perceive("Test".to_string());
        engine.update_state();

        assert!(engine.state.load > 0.0);
    }
}
