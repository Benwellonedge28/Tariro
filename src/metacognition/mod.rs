//! Tariro Enhanced Metacognition Module
//!
//! Implements advanced self-awareness:
//! - Self-model calibration
//! - Bias detection
//! - Cognitive style adaptation
//! - Learning to learn

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MetaId(pub Uuid);

impl MetaId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MetaId {
    fn default() -> Self {
        Self::new()
    }
}

/// Self-model representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub capabilities: HashMap<String, Capability>,
    pub limitations: HashMap<String, Limitation>,
    pub biases: Vec<Bias>,
    pub metacognitive_knowledge: MetacognitiveKnowledge,
    pub calibration: CalibrationState,
}

impl SelfModel {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
            limitations: HashMap::new(),
            biases: Vec::new(),
            metacognitive_knowledge: MetacognitiveKnowledge::new(),
            calibration: CalibrationState::default(),
        }
    }

    /// Assess capability
    pub fn assess_capability(&self, name: &str) -> f64 {
        self.capabilities
            .get(name)
            .map(|c| c.level)
            .unwrap_or(0.5)
    }
}

impl Default for SelfModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Capability representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub level: f64,
    pub confidence: f64,
    pub examples: Vec<String>,
}

/// Limitation representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limitation {
    pub name: String,
    pub severity: f64,
    pub description: String,
}

/// Cognitive bias
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bias {
    pub name: String,
    pub bias_type: BiasType,
    pub strength: f64,
    pub detected: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiasType {
    Confirmation,
    Availability,
    Anchoring,
    Overconfidence,
    SunkCost,
    DunningKruger,
    Hindsight,
    Representativeness,
}

/// Metacognitive knowledge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetacognitiveKnowledge {
    pub task_knowledge: HashMap<String, TaskKnowledge>,
    pub strategy_knowledge: Vec<Strategy>,
    pub self_knowledge: HashMap<String, f64>,
}

impl MetacognitiveKnowledge {
    pub fn new() -> Self {
        Self {
            task_knowledge: HashMap::new(),
            strategy_knowledge: Vec::new(),
            self_knowledge: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskKnowledge {
    pub task_type: String,
    pub difficulty: f64,
    pub success_rate: f64,
    pub estimated_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub name: String,
    pub effectiveness: f64,
    pub conditions: Vec<String>,
}

/// Calibration state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationState {
    pub bias: f64,  // Difference between confidence and accuracy
    pub last_calibration: chrono::DateTime<Utc>,
}

impl Default for CalibrationState {
    fn default() -> Self {
        Self {
            bias: 0.0,
            last_calibration: Utc::now(),
        }
    }
}

// ============================================================================
// Self-Model Calibration
// ============================================================================

/// Self-model calibration engine
#[derive(Debug, Clone)]
pub struct SelfModelCalibrator {
    pub performance_history: VecDeque<PerformanceRecord>,
    pub calibration_threshold: f64,
}

impl SelfModelCalibrator {
    pub fn new() -> Self {
        Self {
            performance_history: VecDeque::new(),
            calibration_threshold: 0.1,
        }
    }

    /// Record performance
    pub fn record(&mut self, task: &str, confidence: f64, success: bool) {
        self.performance_history.push_back(PerformanceRecord {
            task: task.to_string(),
            confidence,
            success,
            timestamp: Utc::now(),
        });

        // Keep only recent history
        if self.performance_history.len() > 100 {
            self.performance_history.pop_front();
        }
    }

    /// Calculate calibration bias
    pub fn calculate_bias(&self) -> f64 {
        if self.performance_history.is_empty() {
            return 0.0;
        }

        let mut total_diff = 0.0;
        for record in &self.performance_history {
            let predicted = if record.confidence > 0.5 { true } else { false };
            let diff = if predicted == record.success { 0.0 } else { 1.0 };
            total_diff += diff.abs();
        }

        total_diff / self.performance_history.len() as f64
    }

    /// Check if calibration is needed
    pub fn needs_calibration(&self) -> bool {
        self.calculate_bias() > self.calibration_threshold
    }

    /// Update self-model based on calibration
    pub fn update_self_model(&self, model: &mut SelfModel) {
        model.calibration.bias = self.calculate_bias();
        model.calibration.last_calibration = Utc::now();
    }
}

impl Default for SelfModelCalibrator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecord {
    pub task: String,
    pub confidence: f64,
    pub success: bool,
    pub timestamp: chrono::DateTime<Utc>,
}

// ============================================================================
// Bias Detection
// ============================================================================

/// Bias detection engine
#[derive(Debug, Clone)]
pub struct BiasDetector {
    pub known_biases: HashMap<BiasType, BiasPattern>,
    pub detected_biases: Vec<DetectedBias>,
}

impl BiasDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            known_biases: HashMap::new(),
            detected_biases: Vec::new(),
        };
        detector.initialize_patterns();
        detector
    }

    fn initialize_patterns(&mut self) {
        // Confirmation bias pattern
        self.known_biases.insert(
            BiasType::Confirmation,
            BiasPattern {
                bias_type: BiasType::Confirmation,
                indicators: vec!["seeks supporting evidence".to_string()],
                detection_rule: "When evidence contrary to belief is ignored".to_string(),
            },
        );

        // Availability bias pattern
        self.known_biases.insert(
            BiasType::Availability,
            BiasPattern {
                bias_type: BiasType::Availability,
                indicators: vec!["recency effect".to_string()],
                detection_rule: "When recent events are overestimated".to_string(),
            },
        );

        // Anchoring bias
        self.known_biases.insert(
            BiasType::Anchoring,
            BiasPattern {
                bias_type: BiasType::Anchoring,
                indicators: vec!["depends heavily on initial value".to_string()],
                detection_rule: "When initial information dominates".to_string(),
            },
        );
    }

    /// Detect biases in reasoning
    pub fn detect(&self, reasoning: &ReasoningTrace) -> Vec<DetectedBias> {
        let mut detected = Vec::new();

        for bias in &reasoning.potential_biases {
            if let Some(pattern) = self.known_biases.get(&bias.bias_type) {
                if reasoning.evidence.iter().any(|e| pattern.indicators.iter().any(|i| e.contains(i))) {
                    detected.push(DetectedBias {
                        bias_type: bias.bias_type,
                        strength: bias.strength,
                        evidence: reasoning.evidence.clone(),
                        suggestion: format!("Consider alternative perspectives"),
                    });
                }
            }
        }

        detected
    }

    /// Check for overconfidence
    pub fn check_overconfidence(&self, confidence: f64, success_rate: f64) -> Option<DetectedBias> {
        let diff = confidence - success_rate;
        if diff > 0.2 {
            Some(DetectedBias {
                bias_type: BiasType::Overconfidence,
                strength: diff,
                evidence: vec![format!("Confidence {} vs success rate {}", confidence, success_rate)],
                suggestion: "Reduce confidence estimate".to_string(),
            })
        } else {
            None
        }
    }
}

impl Default for BiasDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasPattern {
    pub bias_type: BiasType,
    pub indicators: Vec<String>,
    pub detection_rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedBias {
    pub bias_type: BiasType,
    pub strength: f64,
    pub evidence: Vec<String>,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningTrace {
    pub evidence: Vec<String>,
    pub conclusion: String,
    pub confidence: f64,
    pub potential_biases: Vec<Bias>,
}

// ============================================================================
// Cognitive Style Adaptation
// ============================================================================

/// Cognitive style representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveStyle {
    pub learning_mode: LearningMode,
    pub reasoning_style: ReasoningStyle,
    pub processing_preference: ProcessingPreference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LearningMode {
    Visual,
    Auditory,
    Kinesthetic,
    Analytical,
    Holistic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReasoningStyle {
    Deductive,
    Inductive,
    Abductive,
    Analogical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingPreference {
    Sequential,
    Global,
    Detail,
    Overview,
}

/// Style adapter
#[derive(Debug, Clone)]
pub struct CognitiveStyleAdapter {
    pub current_style: CognitiveStyle,
    pub style_adjustments: Vec<StyleAdjustment>,
}

impl CognitiveStyleAdapter {
    pub fn new() -> Self {
        Self {
            current_style: CognitiveStyle {
                learning_mode: LearningMode::Analytical,
                reasoning_style: ReasoningStyle::Deductive,
                processing_preference: ProcessingPreference::Sequential,
            },
            style_adjustments: Vec::new(),
        }
    }

    /// Adapt to task
    pub fn adapt(&mut self, task_type: &str) {
        match task_type {
            "visual" => {
                self.current_style.learning_mode = LearningMode::Visual;
            }
            "logical" => {
                self.current_style.reasoning_style = ReasoningStyle::Deductive;
            }
            "creative" => {
                self.current_style.reasoning_style = ReasoningStyle::Abductive;
                self.current_style.processing_preference = ProcessingPreference::Global;
            }
            _ => {}
        }
    }

    /// Get adapted strategy
    pub fn get_strategy(&self, task: &str) -> String {
        match self.current_style.reasoning_style {
            ReasoningStyle::Deductive => format!("Apply systematic deduction to {}", task),
            ReasoningStyle::Inductive => format!("Find patterns in {}", task),
            ReasoningStyle::Abductive => format!("Generate explanations for {}", task),
            ReasoningStyle::Analogical => format!("Find similar cases to {}", task),
        }
    }
}

impl Default for CognitiveStyleAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleAdjustment {
    pub dimension: String,
    pub old_value: String,
    pub new_value: String,
    pub reason: String,
    pub timestamp: chrono::DateTime<Utc>,
}

// ============================================================================
// Meta-Learning
// ============================================================================

/// Meta-learning engine (learning to learn)
#[derive(Debug, Clone)]
pub struct MetaLearning {
    pub learning_history: VecDeque<MetaLearningRecord>,
    pub strategy_effectiveness: HashMap<String, f64>,
    pub optimal_learning_rate: f64,
}

impl MetaLearning {
    pub fn new() -> Self {
        Self {
            learning_history: VecDeque::new(),
            strategy_effectiveness: HashMap::new(),
            optimal_learning_rate: 0.1,
        }
    }

    /// Record learning episode
    pub fn record_episode(&mut self, task: &str, strategy: &str, success: bool) {
        self.learning_history.push_back(MetaLearningRecord {
            task: task.to_string(),
            strategy: strategy.to_string(),
            success,
            timestamp: Utc::now(),
        });

        // Update effectiveness
        let current = self.strategy_effectiveness.entry(strategy.to_string()).or_insert(0.5);
        if success {
            *current = (*current * 0.9 + 0.1).min(1.0);
        } else {
            *current = (*current * 0.9).max(0.1);
        }

        if self.learning_history.len() > 100 {
            self.learning_history.pop_front();
        }
    }

    /// Get best strategy for task type
    pub fn get_best_strategy(&self, task_type: &str) -> Option<String> {
        self.learning_history
            .iter()
            .filter(|r| r.task.contains(task_type))
            .max_by(|a, b| {
                let eff_a = self.strategy_effectiveness.get(&a.strategy).unwrap_or(&0.5);
                let eff_b = self.strategy_effectiveness.get(&b.strategy).unwrap_or(&0.5);
                eff_a.partial_cmp(eff_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|r| r.strategy.clone())
    }

    /// Update learning rate based on performance
    pub fn adjust_learning_rate(&mut self) {
        let recent: Vec<_> = self.learning_history.iter().rev().take(20).collect();
        if recent.len() < 5 {
            return;
        }

        let success_rate = recent.iter().filter(|r| r.success).count() as f64 / recent.len() as f64;

        // Increase rate if struggling, decrease if succeeding easily
        if success_rate < 0.3 {
            self.optimal_learning_rate = (self.optimal_learning_rate * 1.2).min(1.0);
        } else if success_rate > 0.8 {
            self.optimal_learning_rate = (self.optimal_learning_rate * 0.9).max(0.01);
        }
    }
}

impl Default for MetaLearning {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearningRecord {
    pub task: String,
    pub strategy: String,
    pub success: bool,
    pub timestamp: chrono::DateTime<Utc>,
}

// ============================================================================
// Unified Metacognition System
// ============================================================================

/// Complete metacognition system
#[derive(Debug, Clone)]
pub struct MetacognitionSystem {
    pub self_model: SelfModel,
    pub calibrator: SelfModelCalibrator,
    pub bias_detector: BiasDetector,
    pub style_adapter: CognitiveStyleAdapter,
    pub meta_learning: MetaLearning,
}

impl MetacognitionSystem {
    pub fn new() -> Self {
        Self {
            self_model: SelfModel::new(),
            calibrator: SelfModelCalibrator::new(),
            bias_detector: BiasDetector::new(),
            style_adapter: CognitiveStyleAdapter::new(),
            meta_learning: MetaLearning::new(),
        }
    }

    /// Monitor thinking process
    pub fn monitor(&mut self, thought: &str, confidence: f64) {
        // Check for biases
        let trace = ReasoningTrace {
            evidence: vec![thought.to_string()],
            conclusion: thought.to_string(),
            confidence,
            potential_biases: self.self_model.biases.clone(),
        };

        let detected = self.bias_detector.detect(&trace);
        for bias in detected {
            println!("Detected bias: {:?} - {}", bias.bias_type, bias.suggestion);
        }
    }

    /// Evaluate reasoning
    pub fn evaluate_reasoning(&mut self, reasoning: &str, success: bool) {
        self.calibrator.record(reasoning, 0.7, success);
        self.meta_learning.record_episode(reasoning, "current", success);
    }

    /// Get self-awareness report
    pub fn get_self_awareness_report(&self) -> String {
        let calibration = self.calibrator.calculate_bias();
        format!(
            "Self-Awareness Report:\n\
            - Calibration bias: {:.2}\n\
            - Capabilities: {}\n\
            - Active biases: {}\n\
            - Learning rate: {:.3}",
            calibration,
            self.self_model.capabilities.len(),
            self.self_model.biases.len(),
            self.meta_learning.optimal_learning_rate
        )
    }
}

impl Default for MetacognitionSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod self_model_tests {
    use super::*;

    #[test]
    fn test_assess_capability() {
        let mut model = SelfModel::new();
        model.capabilities.insert(
            "reasoning".to_string(),
            Capability {
                name: "reasoning".to_string(),
                level: 0.8,
                confidence: 0.9,
                examples: vec![],
            },
        );

        assert_eq!(model.assess_capability("reasoning"), 0.8);
        assert_eq!(model.assess_capability("unknown"), 0.5); // Default
    }
}

#[cfg(test)]
mod calibrator_tests {
    use super::*;

    #[test]
    fn test_record() {
        let mut calibrator = SelfModelCalibrator::new();

        calibrator.record("task1", 0.8, true);
        calibrator.record("task2", 0.6, false);

        assert_eq!(calibrator.performance_history.len(), 2);
    }

    #[test]
    fn test_calculate_bias() {
        let mut calibrator = SelfModelCalibrator::new();

        calibrator.record("task1", 0.9, true);
        calibrator.record("task2", 0.2, false);

        let bias = calibrator.calculate_bias();
        assert!(bias >= 0.0 && bias <= 1.0);
    }

    #[test]
    fn test_needs_calibration() {
        let mut calibrator = SelfModelCalibrator::new();
        calibrator.calibration_threshold = 0.5;

        // Add some wrong predictions
        for _ in 0..5 {
            calibrator.record("task", 0.9, false);
        }

        assert!(calibrator.needs_calibration());
    }
}

#[cfg(test)]
mod bias_detector_tests {
    use super::*;

    #[test]
    fn test_check_overconfidence() {
        let detector = BiasDetector::new();

        let bias = detector.check_overconfidence(0.9, 0.5);
        assert!(bias.is_some());
        assert_eq!(bias.unwrap().bias_type, BiasType::Overconfidence);
    }

    #[test]
    fn test_check_not_overconfident() {
        let detector = BiasDetector::new();

        let bias = detector.check_overconfidence(0.7, 0.65);
        assert!(bias.is_none());
    }
}

#[cfg(test)]
mod style_adapter_tests {
    use super::*;

    #[test]
    fn test_adapt() {
        let mut adapter = CognitiveStyleAdapter::new();

        adapter.adapt("logical");
        assert_eq!(adapter.current_style.reasoning_style, ReasoningStyle::Deductive);
    }

    #[test]
    fn test_get_strategy() {
        let adapter = CognitiveStyleAdapter::new();

        let strategy = adapter.get_strategy("problem");
        assert!(strategy.contains("problem"));
    }
}

#[cfg(test)]
mod meta_learning_tests {
    use super::*;

    #[test]
    fn test_record_episode() {
        let mut meta = MetaLearning::new();

        meta.record_episode("math", "practice", true);
        meta.record_episode("math", "practice", false);

        assert_eq!(meta.learning_history.len(), 2);
    }

    #[test]
    fn test_get_best_strategy() {
        let mut meta = MetaLearning::new();

        meta.record_episode("logic", "deduction", true);
        meta.record_episode("logic", "deduction", true);
        meta.record_episode("logic", "induction", false);

        let best = meta.get_best_strategy("logic");
        assert!(best.is_some());
        assert_eq!(best.unwrap(), "deduction");
    }
}

#[cfg(test)]
mod metacognition_system_tests {
    use super::*;

    #[test]
    fn test_monitor() {
        let mut system = MetacognitionSystem::new();
        system.monitor("Testing thought", 0.7);
        // No assertion needed - just ensure no panic
    }

    #[test]
    fn test_evaluate_reasoning() {
        let mut system = MetacognitionSystem::new();
        system.evaluate_reasoning("Test reasoning", true);
        assert!(!system.meta_learning.learning_history.is_empty());
    }

    #[test]
    fn test_self_awareness_report() {
        let system = MetacognitionSystem::new();
        let report = system.get_self_awareness_report();
        assert!(report.contains("Self-Awareness Report"));
    }
}
