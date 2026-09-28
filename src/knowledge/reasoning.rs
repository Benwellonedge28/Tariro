//! Tariro Symbolic Reasoning Engine
//!
//! Reasoning is symbolic manipulation, NOT token prediction.
//! Uses formal logic, inference rules, and deduction.

use crate::prelude::*;
use crate::knowledge::knowledge_graph::{
    KnowledgeGraph, EntityId, Entity, Relation, Fact, Predicate, Probability,
    Evidence, KnowledgeError, Attribute, Value,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::fmt::Debug;

// ============================================================================
// Inference Rules
// ============================================================================

/// An inference rule - explicit and interpretable
#[derive(Debug, Clone)]
pub struct InferenceRule {
    pub name: String,
    pub description: String,
    pub antecedents: Vec<Pattern>,
    pub consequent: Pattern,
    pub confidence: Probability,
    pub justification: String,
}

impl InferenceRule {
    /// Create a new inference rule
    pub fn new(name: impl Into<String>) -> Self {
        InferenceRule {
            name: name.into(),
            description: String::new(),
            antecedents: Vec::new(),
            consequent: Pattern::Wildcard,
            confidence: Probability::certain(),
            justification: String::new(),
        }
    }

    /// Add antecedent pattern
    pub fn when(mut self, pattern: Pattern) -> Self {
        self.antecedents.push(pattern);
        self
    }

    /// Set consequent pattern
    pub fn then(mut self, pattern: Pattern) -> Self {
        self.consequent = pattern;
        self
    }

    /// Set confidence
    pub fn with_confidence(mut self, prob: Probability) -> Self {
        self.confidence = prob;
        self
    }

    /// Check if rule can be applied
    pub fn matches(&self, facts: &[&Fact]) -> Option<Substitution> {
        if self.antecedents.len() != facts.len() {
            return None;
        }

        let mut sub = Substitution::new();

        for (pattern, fact) in self.antecedents.iter().zip(facts.iter()) {
            if let Some(s) = pattern.unify_fact(fact, &sub) {
                sub.merge(s);
            } else {
                return None;
            }
        }

        Some(sub)
    }
}

/// Pattern for unification
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Match any entity
    Wildcard,
    /// Match specific entity
    Entity(EntityId),
    /// Match any predicate
    AnyPredicate,
    /// Match specific predicate
    Predicate(Predicate),
    /// Match with variable
    Variable(String),
    /// Match entity with type constraint
    TypedVariable(String, EntityTypeConstraint),
}

impl Pattern {
    /// Match against a fact
    pub fn unify_fact(&self, fact: &Fact, subst: &Substitution) -> Option<Substitution> {
        match self {
            Pattern::Wildcard => Some(Substitution::new()),
            Pattern::Entity(id) => {
                if fact.subject.id == *id || fact.object.id == *id {
                    Some(Substitution::new())
                } else {
                    None
                }
            }
            Pattern::AnyPredicate => Some(Substitution::new()),
            Pattern::Predicate(p) => {
                if fact.predicate == *p {
                    Some(Substitution::new())
                } else {
                    None
                }
            }
            Pattern::Variable(name) => {
                let mut new_sub = Substitution::new();
                new_sub.insert(name.clone(), VariableValue::Entity(fact.subject.id.clone()));
                Some(new_sub)
            }
            Pattern::TypedVariable(name, type_constraint) => {
                if type_constraint.matches(&fact.subject) {
                    let mut new_sub = Substitution::new();
                    new_sub.insert(name.clone(), VariableValue::Entity(fact.subject.id.clone()));
                    Some(new_sub)
                } else {
                    None
                }
            }
        }
    }
}

/// Type constraint for pattern matching
#[derive(Debug, Clone)]
pub enum EntityTypeConstraint {
    Any,
    Specific(EntityType),
    AnyOf(Vec<EntityType>),
}

impl EntityTypeConstraint {
    fn matches(&self, entity: &Entity) -> bool {
        match self {
            EntityTypeConstraint::Any => true,
            EntityTypeConstraint::Specific(t) => &entity.entity_type == t,
            EntityTypeConstraint::AnyOf(types) => types.iter().any(|t| t == &entity.entity_type),
        }
    }
}

/// Type for pattern matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    Concrete,
    Abstract,
    Event,
    Process,
    Property,
    Concept,
    Agent,
    Custom(String),
}

/// Variable value for substitution
#[derive(Debug, Clone)]
pub enum VariableValue {
    Entity(EntityId),
    Predicate(Predicate),
    String(String),
}

/// Substitution for pattern variables
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    bindings: HashMap<String, VariableValue>,
}

impl Substitution {
    pub fn new() -> Self {
        Substitution { bindings: HashMap::new() }
    }

    pub fn insert(&mut self, var: String, value: VariableValue) {
        self.bindings.insert(var, value);
    }

    pub fn get(&self, var: &str) -> Option<&VariableValue> {
        self.bindings.get(var)
    }

    pub fn merge(&mut self, other: Substitution) {
        for (k, v) in other.bindings {
            self.bindings.insert(k, v);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}

// ============================================================================
// Deduction Result
// ============================================================================

/// Result of a deduction
#[derive(Debug, Clone)]
pub struct Deduction {
    pub fact: Fact,
    pub proof: Proof,
    pub confidence: Probability,
}

impl Deduction {
    pub fn new(fact: Fact, proof: Proof, confidence: Probability) -> Self {
        Deduction { fact, proof, confidence }
    }
}

/// Proof chain for explainability
#[derive(Debug, Clone)]
pub struct Proof {
    pub rule_name: String,
    pub antecedents: Vec<Fact>,
    pub substitutions: Vec<Substitution>,
    pub steps: Vec<ProofStep>,
}

#[derive(Debug, Clone)]
pub struct ProofStep {
    pub step_number: usize,
    pub description: String,
    pub fact: Fact,
}

/// Query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub deductions: Vec<Deduction>,
    pub proof_tree: Option<Proof>,
    pub execution_time_ms: u64,
}

impl QueryResult {
    pub fn new(deductions: Vec<Deduction>) -> Self {
        QueryResult {
            deductions,
            proof_tree: None,
            execution_time_ms: 0,
        }
    }

    pub fn with_proof(mut self, proof: Proof) -> Self {
        self.proof_tree = Some(proof);
        self
    }
}

// ============================================================================
// Reasoning Engine
// ============================================================================

/// The symbolic reasoning engine
pub struct ReasoningEngine {
    /// Knowledge base
    kb: Arc<RwLock<KnowledgeGraph>>,

    /// Inference rules
    rules: Vec<InferenceRule>,

    /// Forward chaining agenda
    agenda: VecDeque<Fact>,

    /// Statistics
    stats: ReasoningStats,
}

/// Reasoning statistics
#[derive(Debug, Clone, Default)]
pub struct ReasoningStats {
    pub deductions_made: usize,
    pub rules_fired: usize,
    pub queries_executed: usize,
}

impl ReasoningEngine {
    /// Create a new reasoning engine
    pub fn new(kb: Arc<RwLock<KnowledgeGraph>>) -> Self {
        let mut engine = ReasoningEngine {
            kb,
            rules: Vec::new(),
            agenda: VecDeque::new(),
            stats: ReasoningStats::default(),
        };

        // Initialize built-in rules
        engine.init_builtin_rules();

        engine
    }

    /// Initialize built-in inference rules
    fn init_builtin_rules(&mut self) {
        // Modus ponens: If A and (A → B) then B
        self.rules.push(InferenceRule::new("modus_ponens")
            .when(Pattern::Wildcard)
            .when(Pattern::Wildcard)
            .then(Pattern::Wildcard)
            .with_confidence(Probability::certain())
        );

        // Transitivity of IsA: If A is-a B and B is-a C, then A is-a C
        self.rules.push(InferenceRule::new("transitivity_is_a")
            .when(Pattern::TypedVariable("X".into(), EntityTypeConstraint::Any))
            .when(Pattern::Predicate(Predicate::IsA))
            .when(Pattern::TypedVariable("Y".into(), EntityTypeConstraint::Any))
            .when(Pattern::TypedVariable("Z".into(), EntityTypeConstraint::Any))
            .then(Pattern::Wildcard)
            .with_confidence(Probability::from_f64(0.95))
        );

        // Symmetry of SimilarTo: If A similar-to B, then B similar-to A
        self.rules.push(InferenceRule::new("symmetry_similar_to")
            .when(Pattern::Variable("A".into()))
            .when(Pattern::Predicate(Predicate::SimilarTo))
            .when(Pattern::Variable("B".into()))
            .then(Pattern::Wildcard)
            .with_confidence(Probability::certain())
        );

        // Causation chains: If A causes B and B causes C, then A causes C (with uncertainty)
        self.rules.push(InferenceRule::new("causation_chain")
            .when(Pattern::Variable("A".into()))
            .when(Pattern::Predicate(Predicate::Causes))
            .when(Pattern::Variable("B".into()))
            .when(Pattern::Variable("B".into()))
            .when(Pattern::Predicate(Predicate::Causes))
            .when(Pattern::Variable("C".into()))
            .then(Pattern::Wildcard)
            .with_confidence(Probability::from_f64(0.85))
        );
    }

    /// Add a custom inference rule
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.rules.push(rule);
    }

    /// Deduce new facts through backward chaining
    pub fn deduce(&self, query: &Query) -> QueryResult {
        let start = std::time::Instant::now();

        let mut deductions = Vec::new();

        // Try to prove query using backward chaining
        if let Some(proof) = self.backward_chain(query) {
            for fact in &proof.antecedents {
                deductions.push(Deduction::new(
                    fact.clone(),
                    proof.clone(),
                    Probability::certain(),
                ));
            }
        }

        let result = QueryResult::new(deductions);
        let mut result = result.with_proof(Proof {
            rule_name: "query".to_string(),
            antecedents: Vec::new(),
            substitutions: Vec::new(),
            steps: Vec::new(),
        });
        result.execution_time_ms = start.elapsed().as_millis() as u64;

        self.stats.queries_executed += 1;

        result
    }

    /// Forward chaining - apply rules to derive new facts
    pub fn forward_chain(&mut self) -> Vec<Deduction> {
        let kb = self.kb.read().unwrap();
        let mut new_deductions = Vec::new();

        // For each rule, check if antecedents are satisfied
        for rule in &self.rules {
            // Get all facts that might match
            let matching_facts = self.find_matching_facts(&kb, &rule.antecedents);

            // Try all combinations
            for facts in matching_facts {
                if let Some(subst) = rule.matches(&facts) {
                    // Apply substitution to consequent
                    let deduced_fact = self.apply_rule(rule, &facts, &subst);

                    if let Some(fact) = deduced_fact {
                        // Check if fact already exists
                        if !kb.has_relation(&fact.subject.id, &fact.predicate, &fact.object.id) {
                            new_deductions.push(Deduction::new(
                                fact,
                                self.build_proof(rule, &facts, &subst),
                                rule.confidence,
                            ));
                            self.stats.deductions_made += 1;
                        }
                    }
                }
            }

            self.stats.rules_fired += 1;
        }

        new_deductions
    }

    /// Apply a rule to produce a new fact
    fn apply_rule(&self, rule: &InferenceRule, facts: &[&Fact], subst: &Substitution) -> Option<Fact> {
        match rule.name.as_str() {
            "transitivity_is_a" => {
                // A is-a B, B is-a C => A is-a C
                if facts.len() >= 2 {
                    let subject = facts[0].subject.clone();
                    let object = facts[1].object.clone();
                    return Some(Fact::new(subject, Predicate::IsA, object)
                        .with_confidence(rule.confidence));
                }
            }
            "symmetry_similar_to" => {
                // A similar-to B => B similar-to A
                if facts.len() >= 2 {
                    let subject = facts[0].object.clone();
                    let object = facts[0].subject.clone();
                    return Some(Fact::new(subject, Predicate::SimilarTo, object)
                        .with_confidence(rule.confidence));
                }
            }
            _ => {}
        }

        None
    }

    /// Find facts matching patterns
    fn find_matching_facts(&self, kb: &KnowledgeGraph, patterns: &[Pattern]) -> Vec<Vec<&Fact>> {
        // Simplified: get all relations and filter
        let all_relations: Vec<&Relation> = kb.relations.values().collect();

        // For each pattern, find matching facts
        let mut matching: Vec<Vec<&Fact>> = patterns.iter().map(|_| Vec::new()).collect();

        for rel in all_relations {
            // Create a dummy fact for matching
            let subject = kb.get_entity(&rel.subject).cloned().unwrap_or_else(|| {
                Entity::new(&rel.subject.0, EntityType::Concept)
            });
            let object = kb.get_entity(&rel.object).cloned().unwrap_or_else(|| {
                Entity::new(&rel.object.0, EntityType::Concept)
            });
            let fact = Fact::new(subject, rel.predicate.clone(), object);

            for (i, pattern) in patterns.iter().enumerate() {
                if let Some(_) = pattern.unify_fact(&fact, &Substitution::new()) {
                    matching[i].push(&fact);
                }
            }
        }

        // Return all combinations
        let mut result = Vec::new();
        self.combine_matches(&matching, &mut result, 0, Vec::new());
        result
    }

    fn combine_matches(&self, matching: &[Vec<&Fact>], results: &mut Vec<Vec<&Fact>>, idx: usize, current: Vec<&Fact>) {
        if idx == matching.len() {
            results.push(current);
            return;
        }

        for fact in &matching[idx] {
            let mut next = current.clone();
            next.push(*fact);
            self.combine_matches(matching, results, idx + 1, next);
        }
    }

    /// Build proof for a deduction
    fn build_proof(&self, rule: &InferenceRule, facts: &[&Fact], subst: &Substitution) -> Proof {
        let mut steps = Vec::new();

        for (i, fact) in facts.iter().enumerate() {
            steps.push(ProofStep {
                step_number: i,
                description: format!("Apply antecedent {} of rule '{}'", i + 1, rule.name),
                fact: (*fact).clone(),
            });
        }

        Proof {
            rule_name: rule.name.clone(),
            antecedents: facts.iter().map(|f| (*f).clone()).collect(),
            substitutions: vec![subst.clone()],
            steps,
        }
    }

    /// Backward chaining to prove a query
    fn backward_chain(&self, query: &Query) -> Option<Proof> {
        // Try each rule that could derive the query
        for rule in &self.rules {
            // Try to prove antecedents
            let mut antecedent_proofs = Vec::new();
            let mut all_proven = true;

            for antecedent in &rule.antecedents {
                if let Some(proof) = self.prove_pattern(antecedent) {
                    antecedent_proofs.push(proof);
                } else {
                    all_proven = false;
                    break;
                }
            }

            if all_proven {
                return Some(Proof {
                    rule_name: rule.name.clone(),
                    antecedents: antecedent_proofs.iter().flat_map(|p| p.antecedents.clone()).collect(),
                    substitutions: Vec::new(),
                    steps: antecedent_proofs.into_iter().flat_map(|p| p.steps).collect(),
                });
            }
        }

        None
    }

    /// Try to prove a pattern
    fn prove_pattern(&self, _pattern: &Pattern) -> Option<Proof> {
        // Simplified: look for matching facts in KB
        let kb = self.kb.read().unwrap();

        for rel in kb.relations.values() {
            let subject = kb.get_entity(&rel.subject).cloned().unwrap_or_else(|| {
                Entity::new(&rel.subject.0, EntityType::Concept)
            });
            let object = kb.get_entity(&rel.object).cloned().unwrap_or_else(|| {
                Entity::new(&rel.object.0, EntityType::Concept)
            });
            let fact = Fact::new(subject, rel.predicate.clone(), object);

            return Some(Proof {
                rule_name: "lookup".to_string(),
                antecedents: vec![fact],
                substitutions: Vec::new(),
                steps: vec![],
            });
        }

        None
    }

    /// Get statistics
    pub fn stats(&self) -> &ReasoningStats {
        &self.stats
    }
}

/// Query for reasoning
#[derive(Debug, Clone)]
pub struct Query {
    pub subject: Option<EntityId>,
    pub predicate: Option<Predicate>,
    pub object: Option<EntityId>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            subject: None,
            predicate: None,
            object: None,
        }
    }

    pub fn with_subject(mut self, subject: EntityId) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_predicate(mut self, predicate: Predicate) -> Self {
        self.predicate = Some(predicate);
        self
    }

    pub fn with_object(mut self, object: EntityId) -> Self {
        self.object = Some(object);
        self
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Abduction
// ============================================================================

/// Abductive reasoning - find best explanations
pub struct AbductionEngine {
    kb: Arc<RwLock<KnowledgeGraph>>,
}

impl AbductionEngine {
    pub fn new(kb: Arc<RwLock<KnowledgeGraph>>) -> Self {
        AbductionEngine { kb }
    }

    /// Find explanations for an observation
    pub fn abduce(&self, observation: &Observation) -> Vec<Explanation> {
        let kb = self.kb.read().unwrap();
        let mut explanations = Vec::new();

        // Find facts that could explain the observation
        for relation in kb.relations.values() {
            // Check if this relation is relevant to the observation
            let relevance = self.calculate_relevance(relation, observation);

            if relevance > 0.5 {
                let subject = kb.get_entity(&relation.subject).cloned();
                let object = kb.get_entity(&relation.object).cloned();

                if let (Some(s), Some(o)) = (subject, object) {
                    explanations.push(Explanation {
                        hypothesis: Fact::new(s, relation.predicate.clone(), o),
                        probability: relation.confidence,
                        simplicity: 1.0, // Simplified
                        coverage: relevance,
                    });
                }
            }
        }

        // Sort by quality score
        explanations.sort_by(|a, b| {
            let score_a = a.probability.value * a.coverage / a.simplicity;
            let score_b = b.probability.value * b.coverage / b.simplicity;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        explanations
    }

    fn calculate_relevance(&self, relation: &Relation, observation: &Observation) -> f64 {
        // Simplified relevance calculation
        if relation.predicate.is_causal() {
            0.8
        } else {
            0.5
        }
    }
}

/// Observation to explain
#[derive(Debug, Clone)]
pub struct Observation {
    pub description: String,
    pub entity: Option<EntityId>,
    pub predicate: Option<Predicate>,
}

/// An explanation for an observation
#[derive(Debug, Clone)]
pub struct Explanation {
    pub hypothesis: Fact,
    pub probability: Probability,
    pub simplicity: f64,
    pub coverage: f64,
}

impl Explanation {
    /// Quality score: P * coverage / simplicity
    pub fn quality_score(&self) -> f64 {
        self.probability.value * self.coverage / self.simplicity
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_engine_creation() {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));
        let engine = ReasoningEngine::new(kb);
        assert_eq!(engine.rules.len(), 4); // Built-in rules
    }

    #[test]
    fn test_query_creation() {
        let query = Query::new()
            .with_subject(EntityId::new("Socrates"))
            .with_predicate(Predicate::IsA)
            .with_object(EntityId::new("Human"));

        assert!(query.subject.is_some());
        assert!(query.predicate.is_some());
        assert!(query.object.is_some());
    }

    #[test]
    fn test_inference_rule() {
        let rule = InferenceRule::new("test_rule")
            .when(Pattern::Variable("X".into()))
            .when(Pattern::Predicate(Predicate::IsA))
            .when(Pattern::Variable("Y".into()))
            .then(Pattern::Wildcard);

        assert_eq!(rule.name, "test_rule");
        assert_eq!(rule.antecedents.len(), 3);
    }

    #[test]
    fn test_substitution() {
        let mut subst = Substitution::new();
        subst.insert("X".into(), VariableValue::Entity(EntityId::new("Socrates")));

        assert!(subst.get("X").is_some());
        assert!(subst.get("Y").is_none());
    }
}
