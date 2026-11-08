//! # Explainability Module for Tariro AGI
//!
//! This module provides comprehensive explainability capabilities for the Tariro AGI system,
//! enabling transparent reasoning, decision tracing, and interpretable outputs.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Represents a single reasoning step in an explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Unique identifier for this step
    pub id: String,
    /// The reasoning operation performed
    pub operation: String,
    /// Inputs to this reasoning step
    pub inputs: Vec<String>,
    /// Outputs from this reasoning step
    pub outputs: Vec<String>,
    /// Confidence in this step's correctness
    pub confidence: f64,
    /// Optional evidence supporting this step
    pub evidence: Vec<String>,
}

/// Complete explanation of a reasoning process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    /// Unique explanation identifier
    pub id: String,
    /// Summary of what was explained
    pub summary: String,
    /// Detailed reasoning steps
    pub steps: Vec<ReasoningStep>,
    /// Entities mentioned in the explanation
    pub entities: HashSet<String>,
    /// Confidence score for the overall explanation
    pub confidence: f64,
    /// Human-readable format
    pub human_readable: String,
}

impl Explanation {
    /// Create new explanation
    pub fn new(id: String, summary: String) -> Self {
        Self {
            id,
            summary,
            steps: Vec::new(),
            entities: HashSet::new(),
            confidence: 1.0,
            human_readable: String::new(),
        }
    }

    /// Add reasoning step
    pub fn add_step(&mut self, step: ReasoningStep) {
        // Update confidence as geometric mean
        if !self.steps.is_empty() {
            self.confidence = (self.confidence * step.confidence).sqrt();
        } else {
            self.confidence = step.confidence;
        }

        // Track entities
        for input in &step.inputs {
            self.entities.insert(input.clone());
        }
        for output in &step.outputs {
            self.entities.insert(output.clone());
        }

        self.steps.push(step);
    }

    /// Generate human-readable summary
    pub fn to_human_readable(&self) -> String {
        let mut output = format!("Explanation: {}\n\n", self.summary);

        for (i, step) in self.steps.iter().enumerate() {
            output.push_str(&format!("Step {}: {}\n", i + 1, step.operation));
            if !step.inputs.is_empty() {
                output.push_str(&format!("  Inputs: {}\n", step.inputs.join(", ")));
            }
            if !step.outputs.is_empty() {
                output.push_str(&format!("  Outputs: {}\n", step.outputs.join(", ")));
            }
            if !step.evidence.is_empty() {
                output.push_str(&format!("  Evidence: {}\n", step.evidence.join(", ")));
            }
            output.push_str(&format!("  Confidence: {:.1}%\n\n", step.confidence * 100.0));
        }

        output.push_str(&format!("Overall Confidence: {:.1}%\n", self.confidence * 100.0));

        output
    }
}

/// Explainability level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplainabilityLevel {
    /// No explanation provided
    None,
    /// Brief summary only
    Brief,
    /// Standard explanation with reasoning steps
    Standard,
    /// Detailed explanation with evidence
    Detailed,
    /// Full transparency with all reasoning trace
    Full,
}

impl Default for ExplainabilityLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Explanation request configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationRequest {
    /// Target to explain (query, action, decision)
    pub target: String,
    /// Desired explanation level
    pub level: ExplainabilityLevel,
    /// Optional constraints on explanation
    pub constraints: ExplanationConstraints,
    /// Format preference
    pub format: ExplanationFormat,
}

impl Default for ExplanationRequest {
    fn default() -> Self {
        Self {
            target: String::new(),
            level: ExplainabilityLevel::default(),
            constraints: ExplanationConstraints::default(),
            format: ExplanationFormat::Text,
        }
    }
}

/// Constraints on explanation generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationConstraints {
    /// Maximum length in tokens
    pub max_length: Option<usize>,
    /// Required entities to include
    pub required_entities: Vec<String>,
    /// Excluded reasoning paths
    pub excluded_paths: Vec<String>,
    /// Audience level (affects vocabulary)
    pub audience: AudienceLevel,
}

impl Default for ExplanationConstraints {
    fn default() -> Self {
        Self {
            max_length: Some(1000),
            required_entities: Vec::new(),
            excluded_paths: Vec::new(),
            audience: AudienceLevel::Technical,
        }
    }
}

/// Target audience level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudienceLevel {
    /// General audience without technical knowledge
    General,
    /// User with basic technical understanding
    Basic,
    /// Technical user with domain knowledge
    Technical,
    /// Expert with deep domain expertise
    Expert,
}

impl Default for AudienceLevel {
    fn default() -> Self {
        Self::Technical
    }
}

/// Output format for explanations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplanationFormat {
    Text,
    Structured,
    Json,
    Markdown,
}

impl Default for ExplanationFormat {
    fn default() -> Self {
        Self::Text
    }
}

/// Explainability system
pub struct ExplainabilitySystem {
    /// Trace storage
    traces: HashMap<String, Vec<ReasoningStep>>,
    /// Cached explanations
    cache: HashMap<String, Explanation>,
    /// Configuration
    config: ExplainabilityConfig,
}

#[derive(Debug, Clone)]
pub struct ExplainabilityConfig {
    pub enable_tracing: bool,
    pub enable_caching: bool,
    pub max_cache_size: usize,
    pub default_level: ExplainabilityLevel,
}

impl Default for ExplainabilityConfig {
    fn default() -> Self {
        Self {
            enable_tracing: true,
            enable_caching: true,
            max_cache_size: 1000,
            default_level: ExplainabilityLevel::Standard,
        }
    }
}

impl ExplainabilitySystem {
    /// Create new explainability system
    pub fn new(config: ExplainabilityConfig) -> Self {
        Self {
            traces: HashMap::new(),
            cache: HashMap::new(),
            config,
        }
    }

    /// Start a new reasoning trace
    pub fn start_trace(&mut self, trace_id: String) {
        if self.config.enable_tracing {
            self.traces.insert(trace_id, Vec::new());
        }
    }

    /// Add step to current trace
    pub fn add_trace_step(&mut self, trace_id: &str, step: ReasoningStep) {
        if self.config.enable_tracing {
            if let Some(trace) = self.traces.get_mut(trace_id) {
                trace.push(step);
            }
        }
    }

    /// End trace and generate explanation
    pub fn end_trace(&mut self, trace_id: String, summary: String) -> Explanation {
        let steps = self.traces.remove(&trace_id).unwrap_or_default();

        let mut explanation = Explanation::new(trace_id.clone(), summary);

        for step in steps {
            explanation.add_step(step);
        }

        // Cache if enabled
        if self.config.enable_caching {
            if self.cache.len() >= self.config.max_cache_size {
                // Remove oldest entry
                if let Some(oldest) = self.cache.keys().next().cloned() {
                    self.cache.remove(&oldest);
                }
            }
            self.cache.insert(trace_id, explanation.clone());
        }

        explanation
    }

    /// Get cached explanation
    pub fn get_cached(&self, id: &str) -> Option<&Explanation> {
        self.cache.get(id)
    }

    /// Generate explanation from request
    pub fn explain(&mut self, request: ExplanationRequest) -> Explanation {
        // Check cache first
        if let Some(cached) = self.cache.get(&request.target) {
            return cached.clone();
        }

        // For now, return empty explanation
        // Full implementation would invoke reasoning trace
        let mut explanation = Explanation::new(
            format!("exp_{}", uuid::Uuid::new_v4()),
            request.target.clone(),
        );

        explanation.human_readable = explanation.to_human_readable();
        explanation
    }
}

impl Default for ExplainabilitySystem {
    fn default() -> Self {
        Self::new(ExplainabilityConfig::default())
    }
}

/// Feature importance explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureImportance {
    /// Feature name
    pub feature: String,
    /// Importance score
    pub importance: f64,
    /// Confidence interval
    pub confidence: (f64, f64),
}

/// Attribution explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    /// What is being attributed
    pub target: String,
    /// Contributing factors with weights
    pub factors: Vec<(String, f64)>,
    /// Overall attribution score
    pub score: f64,
}

impl fmt::Display for Explanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_human_readable())
    }
}
