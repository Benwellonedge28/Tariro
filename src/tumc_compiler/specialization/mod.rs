// Specialization Module - Partial Evaluation and Auto-Differentiation

pub mod partial_eval;
pub mod auto_diff;
pub mod specialization;

use crate::tumc_compiler::ir::{Module, Function};

/// Specialization result
#[derive(Debug, Clone)]
pub struct SpecializationResult {
    pub specialized_module: Module,
    pub residual_module: Module,
    pub statistics: SpecializationStats,
}

#[derive(Debug, Clone)]
pub struct SpecializationStats {
    pub static_evaluations: u64,
    pub dynamic_evaluations: u64,
    pub code_growth_factor: f64,
    pub specialization_ratio: f64,
}

impl Default for SpecializationResult {
    fn default() -> Self {
        Self {
            specialized_module: Module::default(),
            residual_module: Module::default(),
            statistics: SpecializationStats {
                static_evaluations: 0,
                dynamic_evaluations: 0,
                code_growth_factor: 0.0,
                specialization_ratio: 0.0,
            },
        }
    }
}

/// Specialization configuration
#[derive(Debug, Clone)]
pub struct SpecializationConfig {
    pub max_depth: usize,
    pub inline_all: bool,
    pub specialize_loops: bool,
    pub specialize_conditionals: bool,
}

impl Default for SpecializationConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            inline_all: false,
            specialize_loops: true,
            specialize_conditionals: true,
        }
    }
}

/// Main specialization engine
pub struct SpecializationEngine {
    config: SpecializationConfig,
    partial_evaluator: partial_eval::PartialEvaluator,
    differentiator: auto_diff::AutoDifferentiator,
}

impl SpecializationEngine {
    pub fn new(config: SpecializationConfig) -> Self {
        Self {
            config,
            partial_evaluator: partial_eval::PartialEvaluator::default(),
            differentiator: auto_diff::AutoDifferentiator::default(),
        }
    }

    /// Specialize a module with given static arguments
    pub fn specialize_module(
        &mut self,
        module: &Module,
        static_args: &[specialization::StaticValue],
    ) -> SpecializationResult {
        let mut result = SpecializationResult::default();

        // Perform partial evaluation
        for function in &module.functions {
            let specialized = self.partial_evaluator.specialize_function(function, static_args);
            result.specialized_module.functions.push(specialized);
        }

        // Calculate statistics
        result.statistics.specialization_ratio =
            result.specialized_module.functions.len() as f64 / module.functions.len() as f64;

        result
    }

    /// Generate differentiated version
    pub fn differentiate_function(
        &self,
        function: &Function,
        mode: auto_diff::DiffMode,
    ) -> Function {
        self.differentiator.differentiate(function, mode)
    }

    /// Generate gradient function
    pub fn generate_gradient(&self, function: &Function) -> Vec<Function> {
        self.differentiator.generate_gradient(function)
    }
}

impl Default for SpecializationEngine {
    fn default() -> Self {
        Self::new(SpecializationConfig::default())
    }
}
