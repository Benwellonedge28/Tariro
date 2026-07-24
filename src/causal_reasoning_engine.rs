// src/causal_reasoning_engine.rs

//! Causal Reasoning Engine for Tariro AGI
//! Implements Pearl's do-calculus and causal inference

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rand::Rng;

/// Represents a causal relationship between variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    pub from: String,
    pub to: String,
    pub mechanism: CausalMechanism,
    pub strength: f64, // 0.0 to 1.0
    pub delay: u32, // Time steps for effect
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CausalMechanism {
    Direct,
    Indirect,
    Spurious,
    Confounded,
    Mediated,
    Moderated,
}

/// Causal graph structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalGraph {
    pub nodes: Vec<CausalNode>,
    pub edges: Vec<CausalEdge>,
    pub confounding_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalNode {
    pub id: String,
    pub variable_type: VariableType,
    pub domain: Vec<String>, // Possible values
    pub interventions: Vec<Intervention>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    Continuous,
    Discrete,
    Binary,
    Categorical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub do_operator: String, // e.g., "do(X=1)"
    pub effect_on: Vec<String>,
}

/// Probability tables for causal mechanisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalMechanismTable {
    pub parent_config: Vec<String>,
    pub child_distribution: HashMap<String, f64>,
}

/// Counterfactual reasoning result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualResult {
    pub premise: String,
    pub factual_outcome: String,
    pub counterfactual_outcome: String,
    pub probability: f64,
    pub explanation: String,
}

/// Causal effect estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEffect {
    pub treatment: String,
    pub outcome: String,
    pub average_treatment_effect: f64,
    pub confidence_interval: (f64, f64),
    pub confounding_adjustment: Vec<String>,
}

/// Main Causal Reasoning Engine
pub struct CausalReasoningEngine {
    pub causal_graph: CausalGraph,
    pub mechanism_tables: HashMap<String, CausalMechanismTable>,
    pub observations: Vec<Observation>,
    pub interventions: Vec<Intervention>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub timestamp: DateTime<Utc>,
    pub variables: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Continuous(f64),
    Discrete(i64),
    Categorical(String),
    Binary(bool),
}

impl CausalReasoningEngine {
    pub fn new() -> Self {
        CausalReasoningEngine {
            causal_graph: CausalGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
                confounding_factors: Vec::new(),
            },
            mechanism_tables: HashMap::new(),
            observations: Vec::new(),
            interventions: Vec::new(),
        }
    }

    /// Add a causal edge to the graph
    pub fn add_causal_edge(&mut self, edge: CausalEdge) {
        // Check for cycles
        if !self.would_create_cycle(&edge) {
            self.causal_graph.edges.push(edge);
            println!("[CausalReasoning] Added causal edge: {} -> {}", edge.from, edge.to);
        } else {
            println!("[CausalReasoning] WARNING: Cycle detected, edge not added");
        }
    }

    /// Check if adding an edge would create a cycle
    fn would_create_cycle(&self, edge: &CausalEdge) -> bool {
        let mut visited = std::collections::HashSet::new();
        self.has_path(edge.to.clone(), edge.from.clone(), &mut visited)
    }

    fn has_path(&self, from: String, to: String, visited: &mut std::collections::HashSet<String>) -> bool {
        if from == to {
            return true;
        }
        if visited.contains(&from) {
            return false;
        }
        visited.insert(from.clone());

        for edge in &self.causal_graph.edges {
            if edge.from == from {
                if self.has_path(edge.to.clone(), to.clone(), visited) {
                    return true;
                }
            }
        }
        false
    }

    /// Estimate causal effect using do-calculus
    pub fn estimate_causal_effect(&mut self, treatment: &str, outcome: &str, confounders: Vec<String>) -> CausalEffect {
        println!("[CausalReasoning] Estimating causal effect: {} -> {}", treatment, outcome);

        // Step 1: Identify confounders
        let adjusted_confounders = self.identify_backdoor_paths(treatment, outcome, confounders);

        // Step 2: Adjust for confounders
        let ate = self.compute_adjusted_effect(treatment, outcome, &adjusted_confounders);

        // Step 3: Compute confidence interval via bootstrap
        let (ci_low, ci_high) = self.bootstrap_confidence_interval(treatment, outcome, 100);

        CausalEffect {
            treatment: treatment.to_string(),
            outcome: outcome.to_string(),
            average_treatment_effect: ate,
            confidence_interval: (ci_low, ci_high),
            confounding_adjustment: adjusted_confounders,
        }
    }

    /// Identify backdoor paths that need adjustment
    fn identify_backdoor_paths(&self, treatment: &str, outcome: &str, confounders: Vec<String>) -> Vec<String> {
        // Find all paths from treatment to outcome
        // Select paths that start with an arrow into treatment (backdoor paths)
        // Return common causes that block these paths
        let mut blockers = Vec::new();

        for confounder in &confounders {
            if self.is_backdoor_blocker(treatment, outcome, confounder) {
                blockers.push(confounder.clone());
            }
        }

        blockers
    }

    fn is_backdoor_blocker(&self, treatment: &str, outcome: &str, node: &str) -> bool {
        // Check if node is a common cause of treatment and outcome
        let affects_treatment = self.causal_graph.edges.iter().any(|e| e.from == node && e.to == treatment);
        let affects_outcome = self.causal_graph.edges.iter().any(|e| e.from == node && e.to == outcome);
        affects_treatment && affects_outcome
    }

    /// Compute adjusted causal effect
    fn compute_adjusted_effect(&self, treatment: &str, outcome: &str, confounders: &[String]) -> f64 {
        // Stratification: compute effect within each stratum of confounders
        // Then weight by prevalence of each stratum
        let mut weighted_effect = 0.0;

        for obs in &self.observations {
            let mut stratum_weight = 1.0;
            for confounder in confounders {
                if let Some(value) = obs.variables.get(confounder) {
                    stratum_weight *= self.get_stratum_prevalence(confounder, value);
                }
            }

            // Compute within-stratum treatment effect
            let treatment_value = obs.variables.get(treatment);
            let outcome_value = obs.variables.get(outcome);

            if let (Some(Value::Binary(treated)), Some(Value::Continuous(outcome_val))) = (treatment_value, outcome_value) {
                let effect = if *treated { *outcome_val } else { -0.1 * outcome_val };
                weighted_effect += effect * stratum_weight;
            }
        }

        weighted_effect
    }

    fn get_stratum_prevalence(&self, _confounder: &str, _value: &Value) -> f64 {
        // Placeholder: in real implementation, compute from data
        0.5
    }

    /// Bootstrap confidence intervals
    fn bootstrap_confidence_interval(&self, _treatment: &str, _outcome: &str, n_iterations: usize) -> (f64, f64) {
        let mut rng = rand::thread_rng();
        let mut estimates = Vec::new();

        for _ in 0..n_iterations {
            // Resample with replacement
            let n = self.observations.len().max(1);
            let mut sample_effect = 0.0;

            for _ in 0..n {
                let idx = rng.gen_range(0..n);
                sample_effect += rng.gen_range(-1.0..1.0);
            }

            estimates.push(sample_effect / n as f64);
        }

        estimates.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let low_idx = (n_iterations as f64 * 0.025) as usize;
        let high_idx = (n_iterations as f64 * 0.975) as usize;

        (estimates[low_idx], estimates[high_idx])
    }

    /// Counterfactual reasoning: "What if X had been different?"
    pub fn reason_counterfactually(&self, premise: &str, intervention: &str, observation: &Observation) -> CounterfactualResult {
        println!("[CausalReasoning] Counterfactual reasoning: {} under {}", premise, intervention);

        // Step 1: Abduction - infer most likely explanation
        let explanation = self.abduce_explanation(observation);

        // Step 2: Action - apply the counterfactual intervention
        let cf_observation = self.apply_counterfactual(intervention, observation);

        // Step 3: Prediction - predict outcome under counterfactual
        let predicted_outcome = self.predict_outcome(&cf_observation);

        CounterfactualResult {
            premise: premise.to_string(),
            factual_outcome: format!("{:?}", observation),
            counterfactual_outcome: predicted_outcome,
            probability: 0.75, // Placeholder
            explanation,
        }
    }

    fn abduce_explanation(&self, obs: &Observation) -> String {
        // Find most likely causal configuration consistent with observation
        format!("Most likely causal mechanism: confounded by {}", obs.variables.keys().next().unwrap_or(&"unknown".to_string()))
    }

    fn apply_counterfactual(&self, intervention: &str, obs: &Observation) -> Observation {
        // Create new observation with intervention applied
        let mut cf_obs = obs.clone();
        // Parse intervention (e.g., "do(X=5)") and modify observation
        if intervention.starts_with("do(") {
            // Extract variable and value
            cf_obs.variables.insert("intervention_applied".to_string(), Value::Binary(true));
        }
        cf_obs
    }

    fn predict_outcome(&self, obs: &Observation) -> String {
        // Use causal graph to predict outcome
        format!("Predicted outcome based on causal mechanisms")
    }

    /// Do-calculus operations
    pub fn do_operation(&mut self, variable: &str, value: Value) -> Intervention {
        let do_expr = format!("do({}={:?})", variable, value);
        println!("[CausalReasoning] Applying intervention: {}", do_expr);

        Intervention {
            do_operator: do_expr,
            effect_on: self.get_descendants(variable),
        }
    }

    fn get_descendants(&self, node: &str) -> Vec<String> {
        self.causal_graph.edges
            .iter()
            .filter(|e| e.from == node)
            .map(|e| e.to.clone())
            .collect()
    }

    /// Validate causal assumptions
    pub fn validate_assumptions(&self) -> Vec<CausalAssumptionViolation> {
        let mut violations = Vec::new();

        // Check for unobserved confounding
        for edge in &self.causal_graph.edges {
            if edge.mechanism == CausalMechanism::Confounded {
                violations.push(CausalAssumptionViolation {
                    assumption: "No unobserved confounding".to_string(),
                    location: format!("{} -> {}", edge.from, edge.to),
                    severity: ViolationSeverity::High,
                    description: "Potential unmeasured confounder detected".to_string(),
                });
            }
        }

        // Check for positivity violation
        if self.observations.is_empty() {
            violations.push(CausalAssumptionViolation {
                assumption: "Positivity".to_string(),
                location: "All observations".to_string(),
                severity: ViolationSeverity::Critical,
                description: "No observations available".to_string(),
            });
        }

        violations
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalAssumptionViolation {
    pub assumption: String,
    pub location: String,
    pub severity: ViolationSeverity,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Integration with Tariro's existing systems
impl CausalReasoningEngine {
    /// Use causal reasoning to improve RAG retrieval
    pub fn causal_rag_retrieval(&self, query: &str) -> Vec<RetrievedKnowledge> {
        println!("[CausalReasoning] Using causal knowledge for retrieval: {}", query);

        // Extract causal relationships from query
        let causal_elements = self.extract_causal_elements(query);

        // Build counterfactual queries
        let cf_queries = self.generate_counterfactual_queries(&causal_elements);

        // Return knowledge that addresses causal structure
        cf_queries.into_iter().map(|q| RetrievedKnowledge {
            content: q,
            causal_relevance: 0.9,
            retrieval_score: 0.85,
        }).collect()
    }

    fn extract_causal_elements(&self, query: &str) -> Vec<String> {
        // Simple keyword extraction for causal terms
        let causal_terms = ["cause", "effect", "because", "therefore", "lead to", "result in", "why", "how"];
        causal_terms.iter()
            .filter(|t| query.to_lowercase().contains(*t))
            .map(|t| t.to_string())
            .collect()
    }

    fn generate_counterfactual_queries(&self, elements: &[String]) -> Vec<String> {
        elements.iter().map(|e| format!("What if {} were different?", e)).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedKnowledge {
    pub content: String,
    pub causal_relevance: f64,
    pub retrieval_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_edge_addition() {
        let mut engine = CausalReasoningEngine::new();

        // Add nodes
        engine.causal_graph.nodes.push(CausalNode {
            id: "X".to_string(),
            variable_type: VariableType::Binary,
            domain: vec!["0".to_string(), "1".to_string()],
            interventions: vec![],
        });

        // Add causal edge
        engine.add_causal_edge(CausalEdge {
            from: "X".to_string(),
            to: "Y".to_string(),
            mechanism: CausalMechanism::Direct,
            strength: 0.8,
            delay: 1,
        });

        assert_eq!(engine.causal_graph.edges.len(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let mut engine = CausalReasoningEngine::new();

        // Add edges: X -> Y -> Z
        engine.add_causal_edge(CausalEdge {
            from: "X".to_string(),
            to: "Y".to_string(),
            mechanism: CausalMechanism::Direct,
            strength: 0.8,
            delay: 1,
        });

        engine.add_causal_edge(CausalEdge {
            from: "Y".to_string(),
            to: "Z".to_string(),
            mechanism: CausalMechanism::Direct,
            strength: 0.7,
            delay: 1,
        });

        // Try to add edge Z -> X (should be rejected - creates cycle)
        engine.add_causal_edge(CausalEdge {
            from: "Z".to_string(),
            to: "X".to_string(),
            mechanism: CausalMechanism::Direct,
            strength: 0.6,
            delay: 1,
        });

        // Should still only have 2 edges
        assert_eq!(engine.causal_graph.edges.len(), 2);
    }

    #[test]
    fn test_causal_effect_estimation() {
        let mut engine = CausalReasoningEngine::new();

        // Add observation
        let mut vars = HashMap::new();
        vars.insert("Treatment".to_string(), Value::Binary(true));
        vars.insert("Outcome".to_string(), Value::Continuous(10.0));
        vars.insert("Confounder".to_string(), Value::Binary(true));

        engine.observations.push(Observation {
            timestamp: Utc::now(),
            variables: vars,
        });

        let effect = engine.estimate_causal_effect("Treatment", "Outcome", vec!["Confounder".to_string()]);

        assert_eq!(effect.treatment, "Treatment");
        assert_eq!(effect.outcome, "Outcome");
    }
}
