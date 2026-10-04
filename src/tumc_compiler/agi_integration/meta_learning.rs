//! # Meta-Learning Optimization Engine
//!
//! Learns from compilation experiences to improve future optimization decisions.

use crate::tumc_compiler::ir::Module;
use crate::tumc_compiler::agi_integration::{AgiInsight, AgiCompilerConfig, PerformanceMetrics};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Optimization decision record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationDecision {
    pub decision_id: String,
    pub context: OptimizationContext,
    pub action_taken: OptimizationAction,
    pub outcome: OptimizationOutcome,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationContext {
    pub module_hash: String,
    pub function_count: usize,
    pub total_instructions: usize,
    pub loop_count: usize,
    pub branch_count: usize,
    pub memory_ops: usize,
    pub target_platform: String,
    pub optimization_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAction {
    pub action_type: String,
    pub target: String,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOutcome {
    pub compilation_time_ms: u64,
    pub output_size_bytes: usize,
    pub runtime_improvement: f64,
    pub memory_improvement: f64,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Meta-learning model for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaModel {
    /// Learned parameters for each optimization type
    pub parameters: HashMap<String, Vec<f64>>,

    /// Performance baselines
    pub baselines: HashMap<String, f64>,

    /// Confidence scores for each optimization
    pub confidence: HashMap<String, f64>,

    /// Learning history
    pub history: Vec<LearningRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecord {
    pub iteration: u64,
    pub loss: f64,
    pub accuracy: f64,
    pub timestamp: u64,
}

impl Default for MetaModel {
    fn default() -> Self {
        let mut parameters = HashMap::new();

        // Initialize parameters for common optimizations
        parameters.insert("inline_threshold".to_string(), vec![1.0, 0.5, 0.3]); // Mean, variance, count
        parameters.insert("unroll_factor".to_string(), vec![4.0, 2.0, 0.3]);
        parameters.insert("vectorize_threshold".to_string(), vec![100.0, 50.0, 0.3]);
        parameters.insert("parallelize_threshold".to_string(), vec![1000.0, 500.0, 0.3]);

        let mut baselines = HashMap::new();
        baselines.insert("baseline_compile_time".to_string(), 1000.0);
        baselines.insert("baseline_output_size".to_string(), 10000.0);

        let mut confidence = HashMap::new();
        confidence.insert("inline".to_string(), 0.5);
        confidence.insert("unroll".to_string(), 0.5);
        confidence.insert("vectorize".to_string(), 0.5);
        confidence.insert("parallelize".to_string(), 0.5);

        Self {
            parameters,
            baselines,
            confidence,
            history: Vec::new(),
        }
    }
}

/// Meta-learning optimizer
pub struct MetaLearningOptimizer {
    learning_rate: f64,
    model: MetaModel,
    decision_history: VecDeque<OptimizationDecision>,
    max_history_size: usize,
    exploration_rate: f64,
    current_iteration: u64,
}

impl MetaLearningOptimizer {
    pub fn new(learning_rate: f64) -> Self {
        Self {
            learning_rate,
            model: MetaModel::default(),
            decision_history: VecDeque::new(),
            max_history_size: 1000,
            exploration_rate: 0.1,
            current_iteration: 0,
        }
    }

    /// Learn from a compilation
    pub fn learn_from_compilation(&mut self, module: &Module, insights: &[AgiInsight]) {
        self.current_iteration += 1;

        // Create context
        let context = self.extract_context(module);

        // Create actions from insights
        for insight in insights {
            let action = OptimizationAction {
                action_type: format!("{:?}", insight.suggestion.transform_type),
                target: insight.suggestion.target_location.clone(),
                parameters: HashMap::new(),
            };

            // Store decision
            let decision = OptimizationDecision {
                decision_id: format!("dec_{}", self.current_iteration),
                context: context.clone(),
                action_taken: action,
                outcome: OptimizationOutcome {
                    compilation_time_ms: 0, // Will be filled later
                    output_size_bytes: 0,
                    runtime_improvement: 0.0,
                    memory_improvement: 0.0,
                    success: false,
                    error_message: None,
                },
                timestamp: current_timestamp(),
            };

            self.add_decision(decision);
        }

        // Update model
        self.update_model();
    }

    /// Update from feedback
    pub fn update_from_feedback(
        &mut self,
        compilation_id: &str,
        success: bool,
        metrics: &PerformanceMetrics,
    ) {
        // Find the decision
        if let Some(decision) = self.decision_history
            .iter_mut()
            .find(|d| d.decision_id == compilation_id)
        {
            decision.outcome = OptimizationOutcome {
                compilation_time_ms: metrics.compilation_time_ms,
                output_size_bytes: metrics.output_size_bytes,
                runtime_improvement: metrics.estimated_execution_time,
                memory_improvement: metrics.memory_usage_bytes as f64,
                success,
                error_message: None,
            };

            // Update model based on outcome
            self.update_model_with_outcome(decision);
        }
    }

    /// Suggest next optimization
    pub fn suggest_optimization(&self, context: &OptimizationContext) -> Option<OptimizationSuggestion> {
        // Exploration vs exploitation
        if rand_float() < self.exploration_rate {
            // Random exploration
            return Some(self.random_suggestion());
        }

        // Exploitation: use learned model
        self.model_based_suggestion(context)
    }

    fn random_suggestion(&self) -> OptimizationSuggestion {
        let types = vec!["inline", "unroll", "vectorize", "parallelize", "specialize"];
        let type_idx = (rand_float() * types.len() as f64) as usize;
        let opt_type = types[type_idx.min(types.len() - 1)];

        OptimizationSuggestion {
            optimization_type: opt_type.to_string(),
            confidence: 0.3,
            expected_improvement: 0.1,
            risk_level: "medium".to_string(),
            parameters: HashMap::new(),
        }
    }

    fn model_based_suggestion(
        &self,
        context: &OptimizationContext,
    ) -> Option<OptimizationSuggestion> {
        let mut best_suggestion: Option<OptimizationSuggestion> = None;
        let mut best_score = f64::MIN;

        for (opt_type, params) in &self.model.parameters {
            let score = self.calculate_score(opt_type, params, context);
            let confidence = *self.model.confidence.get(opt_type).unwrap_or(&0.5);

            if score > best_score && confidence > 0.4 {
                best_score = score;
                best_suggestion = Some(OptimizationSuggestion {
                    optimization_type: opt_type.clone(),
                    confidence,
                    expected_improvement: score,
                    risk_level: if confidence > 0.7 {
                        "low".to_string()
                    } else {
                        "medium".to_string()
                    },
                    parameters: HashMap::new(),
                });
            }
        }

        best_suggestion
    }

    fn calculate_score(
        &self,
        _opt_type: &str,
        params: &[f64],
        context: &OptimizationContext,
    ) -> f64 {
        // Calculate expected improvement based on learned parameters
        let baseline = *self.model.baselines.get("baseline_compile_time").unwrap_or(&1000.0);

        // Simplified scoring: higher params suggest more aggressive optimization
        let mean = params.first().copied().unwrap_or(1.0);

        // Context-dependent scoring
        let context_factor = if context.loop_count > 10 {
            1.5 // More loops = more benefit from optimizations
        } else if context.memory_ops > context.total_instructions / 2 {
            1.2 // High memory usage
        } else {
            1.0
        };

        mean * context_factor / baseline
    }

    fn add_decision(&mut self, decision: OptimizationDecision) {
        if self.decision_history.len() >= self.max_history_size {
            self.decision_history.pop_front();
        }
        self.decision_history.push_back(decision);
    }

    fn update_model(&mut self) {
        // Update based on recent history
        let recent: Vec<_> = self.decision_history.iter().rev().take(10).collect();

        for decision in recent {
            self.update_model_with_outcome(decision);
        }
    }

    fn update_model_with_outcome(&mut self, decision: &OptimizationDecision) {
        let action_type = &decision.action_taken.action_type;

        if let Some(params) = self.model.parameters.get_mut(action_type) {
            if decision.outcome.success {
                // Increase confidence
                let confidence = self.model.confidence.entry(action_type.clone()).or_insert(0.5);
                *confidence = (*confidence * 0.9 + 0.1).min(0.95);

                // Adjust parameters based on improvement
                let improvement = decision.outcome.runtime_improvement;
                if let Some(mean) = params.first_mut() {
                    *mean += self.learning_rate * improvement;
                }
            } else {
                // Decrease confidence
                let confidence = self.model.confidence.entry(action_type.clone()).or_insert(0.5);
                *confidence = (*confidence * 0.9).max(0.1);
            }
        }

        // Record learning
        self.model.history.push(LearningRecord {
            iteration: self.current_iteration,
            loss: if decision.outcome.success { 0.1 } else { 0.9 },
            accuracy: if decision.outcome.success { 1.0 } else { 0.0 },
            timestamp: current_timestamp(),
        });
    }

    fn extract_context(&self, module: &Module) -> OptimizationContext {
        let mut function_count = 0;
        let mut total_instructions = 0;
        let mut loop_count = 0;
        let mut branch_count = 0;
        let mut memory_ops = 0;

        for function in &module.functions {
            function_count += 1;

            for block in &function.body {
                total_instructions += block.instructions.len();

                for inst in &block.instructions {
                    match &inst.op {
                        crate::tumc_compiler::ir::Operation::Loop(_, _, _) => loop_count += 1,
                        crate::tumc_compiler::ir::Operation::Branch(_, _, _) => branch_count += 1,
                        crate::tumc_compiler::ir::Operation::Load(_, _, _, _)
                        | crate::tumc_compiler::ir::Operation::Store(_, _, _) => memory_ops += 1,
                        _ => {}
                    }
                }
            }
        }

        OptimizationContext {
            module_hash: format!("{:x}", total_instructions),
            function_count,
            total_instructions,
            loop_count,
            branch_count,
            memory_ops,
            target_platform: "default".to_string(),
            optimization_level: "O2".to_string(),
        }
    }

    /// Get current model state
    pub fn get_model(&self) -> &MetaModel {
        &self.model
    }

    /// Reset learning
    pub fn reset(&mut self) {
        self.model = MetaModel::default();
        self.decision_history.clear();
        self.current_iteration = 0;
    }
}

impl Default for MetaLearningOptimizer {
    fn default() -> Self {
        Self::new(0.01)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub optimization_type: String,
    pub confidence: f64,
    pub expected_improvement: f64,
    pub risk_level: String,
    pub parameters: HashMap<String, f64>,
}

fn rand_float() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    ((nanos as f64) % 1000.0) / 1000.0
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
