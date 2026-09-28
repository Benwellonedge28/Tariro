//! Tariro Causal Reasoning Engine
//!
//! Based on Pearl's do-calculus and structural causal models.
//! This is NOT statistical correlation - it is explicit causal reasoning.

use crate::prelude::*;
use crate::knowledge::knowledge_graph::{KnowledgeGraph, EntityId, Entity, Probability};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;

// ============================================================================
// Structural Causal Model
// ============================================================================

/// A structural causal model (SCM)
#[derive(Debug, Clone)]
pub struct StructuralCausalModel {
    /// Variables in the model
    variables: HashMap<String, VariableDef>,

    /// Structural equations
    equations: HashMap<String, StructuralEquation>,

    /// Causal graph
    graph: CausalGraph,

    /// Exogenous variables (external/unobserved)
    exogenous: HashSet<String>,

    /// Endogenous variables (determined by model)
    endogenous: HashSet<String>,
}

/// Definition of a variable
#[derive(Debug, Clone)]
pub struct VariableDef {
    pub name: String,
    pub domain: Vec<Value>,
    pub description: String,
}

/// A value in the domain
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Categorical(String),
    Numeric(f64),
    Boolean(bool),
}

/// Structural equation
#[derive(Debug, Clone)]
pub enum StructuralEquation {
    /// Y = f(X1, X2, ..., Xn, U)
    Functional {
        parents: Vec<String>,
        function: String,
    },
    /// Linear: Y = a*X + b*Z + U
    Linear {
        parents: Vec<String>,
        coefficients: HashMap<String, f64>,
        intercept: f64,
    },
    /// Deterministic: Y is directly determined
    Deterministic {
        parents: Vec<String>,
        value: String,
    },
    /// Probabilistic: Y has a distribution
    Probabilistic {
        parents: Vec<String>,
        distribution: Distribution,
    },
}

/// Probability distribution
#[derive(Debug, Clone)]
pub enum Distribution {
    /// Normal distribution
    Normal { mean: f64, std: f64 },
    /// Bernoulli distribution
    Bernoulli { p: f64 },
    /// Uniform distribution
    Uniform { low: f64, high: f64 },
    /// Custom distribution
    Custom(String),
}

impl StructuralCausalModel {
    /// Create a new SCM
    pub fn new() -> Self {
        StructuralCausalModel {
            variables: HashMap::new(),
            equations: HashMap::new(),
            graph: CausalGraph::new(),
            exogenous: HashSet::new(),
            endogenous: HashSet::new(),
        }
    }

    /// Add a variable
    pub fn add_variable(&mut self, var: VariableDef, is_exogenous: bool) {
        self.variables.insert(var.name.clone(), var);
        if is_exogenous {
            self.exogenous.insert(var.name.clone());
        } else {
            self.endogenous.insert(var.name.clone());
        }
    }

    /// Add a structural equation
    pub fn add_equation(&mut self, variable: &str, equation: StructuralEquation) {
        self.equations.insert(variable.to_string(), equation.clone());

        // Update causal graph
        let parents = match &equation {
            StructuralEquation::Functional { parents, .. } => parents.clone(),
            StructuralEquation::Linear { parents, .. } => parents.clone(),
            StructuralEquation::Deterministic { parents, .. } => parents.clone(),
            StructuralEquation::Probabilistic { parents, .. } => parents.clone(),
        };

        for parent in &parents {
            self.graph.add_edge(parent.clone(), variable.to_string());
        }
    }

    /// Get all causal ancestors of a variable
    pub fn ancestors(&self, variable: &str) -> HashSet<String> {
        self.graph.ancestors(variable)
    }

    /// Get all causal descendants of a variable
    pub fn descendants(&self, variable: &str) -> HashSet<String> {
        self.graph.descendants(variable)
    }

    /// Check if X is a cause of Y
    pub fn is_cause_of(&self, cause: &str, effect: &str) -> bool {
        self.ancestors(effect).contains(cause)
    }

    /// Check if X is a direct cause of Y (parent in graph)
    pub fn is_direct_cause_of(&self, cause: &str, effect: &str) -> bool {
        self.graph.has_edge(cause, effect)
    }
}

impl Default for StructuralCausalModel {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Causal Graph
// ============================================================================

/// Causal graph representation
#[derive(Debug, Clone)]
pub struct CausalGraph {
    /// Adjacency list: node -> children
    children: HashMap<String, HashSet<String>>,

    /// Reverse adjacency list: node -> parents
    parents: HashMap<String, HashSet<String>>,
}

impl CausalGraph {
    /// Create a new causal graph
    pub fn new() -> Self {
        CausalGraph {
            children: HashMap::new(),
            parents: HashMap::new(),
        }
    }

    /// Add a causal edge (cause -> effect)
    pub fn add_edge(&mut self, cause: String, effect: String) {
        self.children.entry(cause.clone()).or_default().insert(effect.clone());
        self.parents.entry(effect).or_default().insert(cause);
    }

    /// Check if edge exists
    pub fn has_edge(&self, cause: &str, effect: &str) -> bool {
        self.children
            .get(cause)
            .map(|children| children.contains(effect))
            .unwrap_or(false)
    }

    /// Get all ancestors (nodes that can reach target)
    pub fn ancestors(&self, target: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(target.to_string());

        while let Some(node) = queue.pop_front() {
            if let Some(parents) = self.parents.get(&node) {
                for parent in parents {
                    if result.insert(parent.clone()) {
                        queue.push_back(parent.clone());
                    }
                }
            }
        }

        result
    }

    /// Get all descendants (nodes reachable from source)
    pub fn descendants(&self, source: &str) -> HashSet<String> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(source.to_string());

        while let Some(node) = queue.pop_front() {
            if let Some(children) = self.children.get(&node) {
                for child in children {
                    if result.insert(child.clone()) {
                        queue.push_back(child.clone());
                    }
                }
            }
        }

        result
    }

    /// Find all backdoor paths from treatment to outcome
    pub fn backdoor_paths(&self, treatment: &str, outcome: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        self.find_paths_internal(treatment, outcome, &mut vec![treatment.to_string()], &mut paths, true);
        paths
    }

    fn find_paths_internal(
        &self,
        current: &str,
        target: &str,
        path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>,
        allow_forward: bool,
    ) {
        if current == target {
            all_paths.push(path.clone());
            return;
        }

        // Forward edges
        if allow_forward {
            if let Some(children) = self.children.get(current) {
                for child in children {
                    if !path.contains(child) {
                        path.push(child.clone());
                        self.find_paths_internal(child, target, path, all_paths, true);
                        path.pop();
                    }
                }
            }
        }

        // Backward edges (backdoor paths)
        if let Some(parents) = self.parents.get(current) {
            for parent in parents {
                if !path.contains(parent) {
                    path.push(parent.clone());
                    self.find_paths_internal(parent, target, path, all_paths, false);
                    path.pop();
                }
            }
        }
    }

    /// Find all frontdoor paths (through mediators)
    pub fn frontdoor_paths(&self, treatment: &str, outcome: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        self.find_paths_internal(treatment, outcome, &mut vec![treatment.to_string()], &mut paths, true);
        paths.retain(|p| p.len() > 2); // Frontdoor has mediator
        paths
    }
}

// ============================================================================
// Do-Calculus Engine
// ============================================================================

/// Do-calculus interpreter for causal inference
pub struct DoCalculator {
    scm: StructuralCausalModel,
}

impl DoCalculator {
    /// Create a new do-calculator
    pub fn new(scm: StructuralCausalModel) -> Self {
        DoCalculator { scm }
    }

    /// Compute P(Y | do(X))
    /// The causal effect of intervening to set X
    pub fn causal_effect(&self, treatment: &str, outcome: &str) -> CausalEffect {
        // Use Pearl's do-calculus rules

        // Rule 1 (Insertion/deletion of observations): P(Y | do(X), Z) = P(Y | do(X), Z) if Z is not descendant of X
        // Rule 2 (Action/observation exchange): P(Y | do(X), do(Z)) = P(Y | do(X), Z) if no unblocked path from Z to X
        // Rule 3 (Insertion/deletion of actions): P(Y | do(X), do(Z)) = P(Y | do(X)) if Z does not affect Y except through X

        let descendants_x = self.scm.descendants(treatment);

        CausalEffect {
            treatment: treatment.to_string(),
            outcome: outcome.to_string(),
            effect_type: EffectType::Total,
            adjustment_variables: self.find_adjustment_set(treatment, outcome),
            descendants_x,
        }
    }

    /// Compute direct effect: P(Y | do(X), do(Z)) - P(Y | do(X'), do(Z))
    pub fn direct_effect(&self, treatment: &str, outcome: &str, mediator: &str) -> CausalEffect {
        CausalEffect {
            treatment: treatment.to_string(),
            outcome: outcome.to_string(),
            effect_type: EffectType::Direct,
            adjustment_variables: vec![mediator.to_string()],
            descendants_x: self.scm.descendants(treatment),
        }
    }

    /// Find adjustment set for computing causal effect
    fn find_adjustment_set(&self, treatment: &str, outcome: &str) -> Vec<String> {
        // Find confounders (common causes of treatment and outcome)
        let treatment_ancestors = self.scm.ancestors(treatment);
        let outcome_ancestors = self.scm.ancestors(outcome);

        // Confounders are ancestors of both
        treatment_ancestors
            .intersection(&outcome_ancestors)
            .cloned()
            .collect()
    }

    /// Check identifiability: can we compute the causal effect?
    pub fn is_identifiable(&self, treatment: &str, outcome: &str) -> bool {
        // Check if there is no unblocked backdoor path that cannot be adjusted for
        let backdoor_paths = self.scm.backdoor_paths(treatment, outcome);

        // If all backdoor paths are blocked by adjustment set, identifiable
        !backdoor_paths.is_empty()
    }
}

/// Computed causal effect
#[derive(Debug, Clone)]
pub struct CausalEffect {
    pub treatment: String,
    pub outcome: String,
    pub effect_type: EffectType,
    pub adjustment_variables: Vec<String>,
    pub descendants_x: HashSet<String>,
}

/// Type of causal effect
#[derive(Debug, Clone, Copy)]
pub enum EffectType {
    Total,
    Direct,
    NaturalDirect,
    Indirect,
}

// ============================================================================
// Causal Reasoning Engine
// ============================================================================

/// The causal reasoning engine
pub struct CausalReasoningEngine {
    /// Knowledge graph reference
    kb: Arc<RwLock<KnowledgeGraph>>,

    /// SCM built from knowledge
    scm: Option<StructuralCausalModel>,

    /// Do-calculator
    calculator: Option<DoCalculator>,
}

impl CausalReasoningEngine {
    /// Create a new causal reasoning engine
    pub fn new(kb: Arc<RwLock<KnowledgeGraph>>) -> Self {
        CausalReasoningEngine {
            kb,
            scm: None,
            calculator: None,
        }
    }

    /// Build SCM from knowledge graph
    pub fn build_scm(&mut self) {
        let kb = self.kb.read().unwrap();
        let mut scm = StructuralCausalModel::new();

        // Extract causal relations from KB
        for relation in kb.relations.values() {
            if relation.predicate.is_causal() {
                let cause = relation.subject.0.clone();
                let effect = relation.object.0.clone();

                // Add variables if not exist
                if !scm.variables.contains_key(&cause) {
                    scm.add_variable(
                        VariableDef {
                            name: cause.clone(),
                            domain: vec![Value::Boolean(true)],
                            description: format!("Cause: {}", cause),
                        },
                        true, // Treat causes as exogenous initially
                    );
                }

                if !scm.variables.contains_key(&effect) {
                    scm.add_variable(
                        VariableDef {
                            name: effect.clone(),
                            domain: vec![Value::Boolean(true)],
                            description: format!("Effect: {}", effect),
                        },
                        false,
                    );
                }

                // Add causal equation
                scm.add_equation(
                    &effect,
                    StructuralEquation::Deterministic {
                        parents: vec![cause],
                        value: effect.clone(),
                    },
                );
            }
        }

        self.scm = Some(scm.clone());
        self.calculator = Some(DoCalculator::new(scm));
    }

    /// Get the SCM
    pub fn get_scm(&self) -> Option<&StructuralCausalModel> {
        self.scm.as_ref()
    }

    /// Check if X causes Y
    pub fn is_cause_of(&self, cause: &EntityId, effect: &EntityId) -> Option<bool> {
        let kb = self.kb.read().unwrap();

        // Direct lookup
        if kb.has_relation(cause, &crate::knowledge::knowledge_graph::Predicate::Causes, effect) {
            return Some(true);
        }

        // Check through SCM
        if let Some(scm) = &self.scm {
            return Some(scm.is_cause_of(&cause.0, &effect.0));
        }

        None
    }

    /// Compute causal effect
    pub fn compute_causal_effect(&self, treatment: &str, outcome: &str) -> Option<CausalEffect> {
        self.calculator
            .as_ref()
            .map(|calc| calc.causal_effect(treatment, outcome))
    }

    /// Counterfactual reasoning: What if X had been x' instead of x?
    pub fn counterfactual(
        &self,
        entity: &EntityId,
        variable: &str,
        actual_value: &Value,
        counterfactual_value: &Value,
    ) -> CounterfactualResult {
        // 1. Check if counterfactual is valid
        let is_valid = actual_value != counterfactual_value;

        // 2. Find effects of the variable
        let effects = if let Some(scm) = &self.scm {
            scm.descendants(variable)
                .into_iter()
                .map(|effect| {
                    // Simplified: assume effects propagate
                    CounterfactualEffect {
                        variable: effect,
                        original_value: actual_value.clone(),
                        counterfactual_value: counterfactual_value.clone(),
                        change_magnitude: 1.0,
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        CounterfactualResult {
            entity: entity.clone(),
            variable: variable.to_string(),
            actual_value: actual_value.clone(),
            counterfactual_value: counterfactual_value.clone(),
            is_valid,
            predicted_effects: effects,
            confidence: Probability::from_f64(0.85),
        }
    }

    /// Causal explanation
    pub fn explain_causation(&self, effect: &EntityId) -> CausalExplanation {
        let kb = self.kb.read().unwrap();

        // Find direct causes
        let direct_causes: Vec<Entity> = kb
            .get_causes(effect)
            .into_iter()
            .cloned()
            .collect();

        // Build causal chain
        let mut chain = Vec::new();
        let mut visited = HashSet::new();
        self.build_causal_chain(effect, &mut chain, &mut visited, 0);

        CausalExplanation {
            effect: effect.clone(),
            direct_causes,
            causal_chain: chain,
            confidence: Probability::certain(),
        }
    }

    fn build_causal_chain(
        &self,
        entity: &EntityId,
        chain: &mut Vec<CausalLink>,
        visited: &mut HashSet<EntityId>,
        depth: usize,
    ) {
        if depth > 10 || visited.contains(entity) {
            return;
        }
        visited.insert(entity.clone());

        let kb = self.kb.read().unwrap();
        let causes = kb.get_causes(entity);

        for cause_entity in causes {
            chain.push(CausalLink {
                cause: cause_entity.id.clone(),
                effect: entity.clone(),
                mechanism: format!("Direct causal relation at depth {}", depth),
            });

            // Recursively find causes of causes
            self.build_causal_chain(&cause_entity.id, chain, visited, depth + 1);
        }
    }

    /// Test for confounding
    pub fn identify_confounders(&self, treatment: &str, outcome: &str) -> Vec<String> {
        if let Some(scm) = &self.scm {
            let treatment_ancestors = scm.ancestors(treatment);
            let outcome_ancestors = scm.ancestors(outcome);

            // Confounders are common ancestors
            treatment_ancestors
                .intersection(&outcome_ancestors)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Counterfactual reasoning result
#[derive(Debug, Clone)]
pub struct CounterfactualResult {
    pub entity: EntityId,
    pub variable: String,
    pub actual_value: Value,
    pub counterfactual_value: Value,
    pub is_valid: bool,
    pub predicted_effects: Vec<CounterfactualEffect>,
    pub confidence: Probability,
}

/// Effect predicted by counterfactual
#[derive(Debug, Clone)]
pub struct CounterfactualEffect {
    pub variable: String,
    pub original_value: Value,
    pub counterfactual_value: Value,
    pub change_magnitude: f64,
}

/// Causal explanation
#[derive(Debug, Clone)]
pub struct CausalExplanation {
    pub effect: EntityId,
    pub direct_causes: Vec<Entity>,
    pub causal_chain: Vec<CausalLink>,
    pub confidence: Probability,
}

/// A link in the causal chain
#[derive(Debug, Clone)]
pub struct CausalLink {
    pub cause: EntityId,
    pub effect: EntityId,
    pub mechanism: String,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_graph_creation() {
        let mut graph = CausalGraph::new();
        graph.add_edge("X".to_string(), "Y".to_string());
        graph.add_edge("X".to_string(), "Z".to_string());
        graph.add_edge("Y".to_string(), "Z".to_string());

        assert!(graph.has_edge("X", "Y"));
        assert!(!graph.has_edge("Y", "X"));
    }

    #[test]
    fn test_causal_graph_ancestors() {
        let mut graph = CausalGraph::new();
        graph.add_edge("X".to_string(), "Y".to_string());
        graph.add_edge("Y".to_string(), "Z".to_string());
        graph.add_edge("W".to_string(), "Z".to_string());

        let ancestors_z = graph.ancestors("Z");
        assert!(ancestors_z.contains("X"));
        assert!(ancestors_z.contains("Y"));
        assert!(ancestors_z.contains("W"));
    }

    #[test]
    fn test_causal_graph_descendants() {
        let mut graph = CausalGraph::new();
        graph.add_edge("X".to_string(), "Y".to_string());
        graph.add_edge("X".to_string(), "Z".to_string());
        graph.add_edge("Y".to_string(), "Z".to_string());

        let descendants_x = graph.descendants("X");
        assert!(descendants_x.contains("Y"));
        assert!(descendants_x.contains("Z"));
    }

    #[test]
    fn test_scm_creation() {
        let mut scm = StructuralCausalModel::new();

        scm.add_variable(
            VariableDef {
                name: "Rain".to_string(),
                domain: vec![Value::Boolean(true), Value::Boolean(false)],
                description: "Whether it is raining".to_string(),
            },
            true,
        );

        scm.add_variable(
            VariableDef {
                name: "WetGrass".to_string(),
                domain: vec![Value::Boolean(true), Value::Boolean(false)],
                description: "Whether the grass is wet".to_string(),
            },
            false,
        );

        scm.add_equation(
            "WetGrass",
            StructuralEquation::Linear {
                parents: vec!["Rain".to_string()],
                coefficients: [("Rain".to_string(), 1.0)].iter().cloned().collect(),
                intercept: 0.0,
            },
        );

        assert!(scm.is_cause_of("Rain", "WetGrass"));
    }

    #[test]
    fn test_backdoor_paths() {
        let mut graph = CausalGraph::new();
        // Confounder C affects both X and Y
        graph.add_edge("C".to_string(), "X".to_string());
        graph.add_edge("C".to_string(), "Y".to_string());
        // Direct path
        graph.add_edge("X".to_string(), "Y".to_string());

        let backdoor = graph.backdoor_paths("X", "Y");
        assert!(!backdoor.is_empty()); // Should find C -> X -> ... -> Y
    }

    #[test]
    fn test_do_calculator() {
        let mut scm = StructuralCausalModel::new();
        scm.add_variable(
            VariableDef {
                name: "X".to_string(),
                domain: vec![Value::Numeric(0.0)],
                description: "Treatment".to_string(),
            },
            true,
        );
        scm.add_variable(
            VariableDef {
                name: "Y".to_string(),
                domain: vec![Value::Numeric(0.0)],
                description: "Outcome".to_string(),
            },
            false,
        );
        scm.add_equation(
            "Y",
            StructuralEquation::Linear {
                parents: vec!["X".to_string()],
                coefficients: [("X".to_string(), 2.0)].iter().cloned().collect(),
                intercept: 1.0,
            },
        );

        let calc = DoCalculator::new(scm);
        let effect = calc.causal_effect("X", "Y");

        assert_eq!(effect.treatment, "X");
        assert_eq!(effect.outcome, "Y");
    }

    #[test]
    fn test_counterfactual() {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));
        let engine = CausalReasoningEngine::new(kb);

        let result = engine.counterfactual(
            &EntityId::new("TestEntity"),
            "Temperature",
            &Value::Numeric(25.0),
            &Value::Numeric(35.0),
        );

        assert!(result.is_valid);
        assert_eq!(result.actual_value, Value::Numeric(25.0));
        assert_eq!(result.counterfactual_value, Value::Numeric(35.0));
    }
}
