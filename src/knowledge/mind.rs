//! Tariro Mind - Integrated AGI Mind Architecture
//!
//! Combines knowledge graph, reasoning, logic, causal reasoning,
//! common sense, and formal verification into a unified cognitive system.

use crate::prelude::*;
use crate::knowledge::knowledge_graph::{
    KnowledgeGraph, EntityId, Entity, Fact, Predicate, Probability,
};
use crate::knowledge::reasoning::{ReasoningEngine, Query, QueryResult, Deduction};
use crate::knowledge::logic::LogicEngine;
use crate::knowledge::causal::{CausalReasoningEngine, CausalEffect, CounterfactualResult};
use crate::knowledge::commonsense::{CommonSenseEngine, Situation, Observation, Inference};
use crate::knowledge::formal::{FormalVerifier, VerificationResult};
use std::sync::Arc;

// ============================================================================
// The Tariro Mind
// ============================================================================

/// The integrated Tariro Mind - a unified cognitive architecture
pub struct TariroMind {
    /// Knowledge Graph - explicit first-class knowledge
    pub knowledge_graph: Arc<RwLock<KnowledgeGraph>>,

    /// Symbolic Reasoning Engine
    pub reasoning: ReasoningEngine,

    /// Logic Programming Engine
    pub logic: LogicEngine,

    /// Causal Reasoning Engine
    pub causal: CausalReasoningEngine,

    /// Common Sense Engine
    pub commonsense: CommonSenseEngine,

    /// Formal Verifier
    pub verifier: FormalVerifier,

    /// Mind statistics
    stats: MindStats,

    /// Configuration
    config: MindConfig,
}

/// Mind statistics
#[derive(Debug, Clone, Default)]
pub struct MindStats {
    pub queries_processed: usize,
    pub facts_learned: usize,
    pub deductions_made: usize,
    pub causal_inferences: usize,
    pub contradictions_found: usize,
    pub verifications_passed: usize,
}

/// Mind configuration
#[derive(Debug, Clone)]
pub struct MindConfig {
    /// Enable common sense reasoning
    pub use_commonsense: bool,

    /// Enable causal reasoning
    pub use_causal: bool,

    /// Enable formal verification
    pub use_verification: bool,

    /// Enable logic programming
    pub use_logic: bool,

    /// Max deduction depth
    pub max_deduction_depth: usize,

    /// Auto-verify new facts
    pub auto_verify: bool,

    /// Learning rate (how quickly to incorporate new knowledge)
    pub learning_rate: f64,
}

impl Default for MindConfig {
    fn default() -> Self {
        MindConfig {
            use_commonsense: true,
            use_causal: true,
            use_verification: true,
            use_logic: true,
            max_deduction_depth: 10,
            auto_verify: true,
            learning_rate: 0.5,
        }
    }
}

impl TariroMind {
    /// Create a new Tariro Mind
    pub fn new() -> Self {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));

        TariroMind {
            knowledge_graph: kb.clone(),
            reasoning: ReasoningEngine::new(kb.clone()),
            logic: LogicEngine::new(kb.clone()),
            causal: CausalReasoningEngine::new(kb.clone()),
            commonsense: CommonSenseEngine::new(kb.clone()),
            verifier: FormalVerifier::new(),
            stats: MindStats::default(),
            config: MindConfig::default(),
        }
    }

    /// Create with configuration
    pub fn with_config(config: MindConfig) -> Self {
        let mut mind = Self::new();
        mind.config = config;

        // Load logic rules
        if mind.config.use_logic {
            mind.logic.add_common_rules();
        }

        mind
    }

    // ========================================================================
    // Knowledge Operations
    // ========================================================================

    /// Learn a new fact (with optional verification)
    pub fn learn_fact(&mut self, fact: Fact) -> Result<LearningResult, LearningError> {
        self.stats.facts_learned += 1;

        // Verify if enabled
        if self.config.auto_verify {
            let verification = self.verifier.verify_invariants(&self.knowledge_graph.read().unwrap());
            if !verification.is_valid() {
                return Err(LearningError::VerificationFailed(verification));
            }
        }

        // Type check
        if self.config.use_verification {
            if let Err(e) = self.verifier.type_check(&fact.subject) {
                return Err(LearningError::TypeError(e));
            }
            if let Err(e) = self.verifier.type_check(&fact.object) {
                return Err(LearningError::TypeError(e));
            }
        }

        // Add to knowledge graph
        let kb = &mut *self.knowledge_graph.write().unwrap();
        kb.assert_fact(fact.clone())
            .map_err(LearningError::KnowledgeError)?;

        // Add to logic engine
        if self.config.use_logic {
            let subject = fact.subject.name.clone();
            let predicate = fact.predicate.name().to_string();
            let object = fact.object.name.clone();
            self.logic.assert_fact(&predicate, vec![
                crate::knowledge::logic::Term::constant(subject),
                crate::knowledge::logic::Term::constant(object),
            ]);
        }

        // Run forward chaining
        let deductions = self.reasoning.forward_chain();

        // Apply deductions
        for deduction in &deductions {
            let _ = kb.assert_fact(deduction.fact.clone());
            self.stats.deductions_made += 1;
        }

        Ok(LearningResult::new(fact, deductions, deductions.len()))
    }

    /// Add an entity
    pub fn add_entity(&mut self, entity: Entity) -> Result<(), LearningError> {
        if self.config.use_verification {
            if let Err(e) = self.verifier.type_check(&entity) {
                return Err(LearningError::TypeError(e));
            }
        }

        let mut kb = self.knowledge_graph.write().unwrap();
        kb.assert_entity(entity)
            .map_err(LearningError::KnowledgeError)
    }

    // ========================================================================
    // Reasoning Operations
    // ========================================================================

    /// Query the mind
    pub fn query(&self, query: Query) -> QueryResult {
        self.stats.queries_processed += 1;
        self.reasoning.deduce(&query)
    }

    /// Query with commonsense
    pub fn query_with_context(&self, query: Query, situation: Situation) -> ContextualResult {
        let direct_result = self.query(query);

        // Add common sense inferences
        let mut inferences = Vec::new();
        if self.config.use_commonsense {
            inferences = self.commonsense.infer_consequences(&situation);
        }

        ContextualResult {
            direct_results: direct_result.deductions,
            common_sense_inferences: inferences,
            causal_effects: Vec::new(),
        }
    }

    /// Explain a conclusion
    pub fn explain(&self, conclusion: &Fact) -> Explanation {
        // Build proof chain
        let mut proof_steps = Vec::new();

        // Find supporting facts
        let kb = self.knowledge_graph.read().unwrap();

        // Check direct support
        let supporting_facts: Vec<_> = kb.get_subject_relations(&conclusion.subject.id)
            .iter()
            .filter(|r| r.predicate == conclusion.predicate)
            .map(|r| {
                kb.get_entity(&r.object)
                    .cloned()
                    .unwrap_or_else(|| Entity::new(&r.object.0, crate::knowledge::knowledge_graph::EntityType::Concept))
            })
            .collect();

        for (i, fact) in supporting_facts.iter().enumerate() {
            proof_steps.push(ProofStep {
                step_number: i,
                description: format!("Supported by fact: {} -> {:?}", fact.name, conclusion.predicate),
                support: SupportLevel::Direct,
            });
        }

        Explanation {
            conclusion: conclusion.clone(),
            proof_steps,
            confidence: conclusion.confidence,
            reasoning_type: ReasoningType::Deductive,
        }
    }

    // ========================================================================
    // Causal Reasoning
    // ========================================================================

    /// Analyze causation
    pub fn analyze_causation(&mut self, effect: &EntityId) -> CausalAnalysisResult {
        self.stats.causal_inferences += 1;

        // Build SCM from knowledge graph
        self.causal.build_scm();

        // Get explanation
        let explanation = self.causal.explain_causation(effect);

        CausalAnalysisResult {
            effect: effect.clone(),
            direct_causes: explanation.direct_causes,
            causal_chain: explanation.causal_chain,
            confidence: explanation.confidence,
        }
    }

    /// Compute causal effect
    pub fn compute_causal_effect(
        &mut self,
        treatment: &str,
        outcome: &str,
    ) -> Option<CausalEffect> {
        self.causal.build_scm();
        self.causal.compute_causal_effect(treatment, outcome)
    }

    /// Counterfactual reasoning
    pub fn what_if(
        &self,
        entity: &EntityId,
        variable: &str,
        counterfactual: &str,
    ) -> CounterfactualResult {
        self.causal.counterfactual(
            entity,
            variable,
            &crate::knowledge::causal::Value::String("actual".to_string()),
            &crate::knowledge::causal::Value::String(counterfactual.to_string()),
        )
    }

    // ========================================================================
    // Common Sense
    // ========================================================================

    /// Infer consequences of a situation
    pub fn infer_consequences(&self, situation: &Situation) -> Vec<Inference> {
        self.commonsense.infer_consequences(situation)
    }

    /// Answer "what if" questions
    pub fn common_sense_what_if(&self, hypothetical: &Hypothetical) -> Vec<Inference> {
        self.commonsense.what_if(hypothetical)
    }

    // ========================================================================
    // Formal Verification
    // ========================================================================

    /// Verify knowledge graph invariants
    pub fn verify(&self) -> VerificationResult {
        let kb = self.knowledge_graph.read().unwrap();
        let result = self.verifier.verify_invariants(&kb);

        if result.is_valid() {
            self.stats.verifications_passed += 1;
        }

        result
    }

    /// Check for contradictions
    pub fn check_contradictions(&self) -> Vec<Contradiction> {
        let kb = self.knowledge_graph.read().unwrap();
        let mut contradictions = Vec::new();

        // Check for A and not A
        for relation in kb.relations.values() {
            // Check for inverse relations
            let inverse_predicate = match &relation.predicate {
                Predicate::Causes => Predicate::Causes,
                Predicate::Before => Predicate::After,
                _ => continue,
            };

            // Look for contradictory relations
            let inverse_exists = kb.relations.values().any(|r| {
                r.subject == relation.object
                    && r.predicate == inverse_predicate
                    && r.object == relation.subject
            });

            if inverse_exists && relation.predicate == relation.predicate {
                contradictions.push(Contradiction {
                    fact1: relation.clone().into(),
                    fact2: None,
                    contradiction_type: ContradictionType::MutualExclusion,
                });
                self.stats.contradictions_found += 1;
            }
        }

        contradictions
    }

    // ========================================================================
    // Logic Programming
    // ========================================================================

    /// Query with logic engine
    pub fn logic_query(&self, predicate: &str, terms: &[crate::knowledge::logic::Term]) -> Vec<std::collections::HashMap<String, crate::knowledge::logic::Term>> {
        self.logic.all_solutions(predicate, terms)
    }

    /// Add logic rule
    pub fn add_logic_rule(&mut self, rule: &str) -> Result<(), crate::knowledge::logic::ParseError> {
        self.logic.assert_string(rule)
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    /// Get mind statistics
    pub fn stats(&self) -> &MindStats {
        &self.stats
    }

    /// Reset statistics
    pub fn reset_stats(&mut self) {
        self.stats = MindStats::default();
    }

    /// Get configuration
    pub fn config(&self) -> &MindConfig {
        &self.config
    }

    // ========================================================================
    // Export/Import
    // ========================================================================

    /// Export knowledge graph to JSON
    pub fn export_knowledge(&self) -> String {
        let kb = self.knowledge_graph.read().unwrap();
        kb.to_json()
    }

    /// Import knowledge graph from JSON
    pub fn import_knowledge(&mut self, json: &str) -> Result<(), LearningError> {
        let imported = KnowledgeGraph::from_json(json)
            .map_err(|e| LearningError::ParseError(e.to_string()))?;

        let mut kb = self.knowledge_graph.write().unwrap();
        *kb = imported;
        Ok(())
    }
}

impl Default for TariroMind {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Result of learning
#[derive(Debug, Clone)]
pub struct LearningResult {
    pub learned_fact: Fact,
    pub derived_facts: Vec<Deduction>,
    pub deduction_count: usize,
}

impl LearningResult {
    pub fn new(fact: Fact, deductions: Vec<Deduction>, deduction_count: usize) -> Self {
        LearningResult {
            learned_fact: fact,
            derived_facts: deductions,
            deduction_count,
        }
    }
}

/// Learning error
#[derive(Debug, Clone)]
pub enum LearningError {
    KnowledgeError(crate::knowledge::knowledge_graph::KnowledgeError),
    TypeError(crate::knowledge::formal::TypeError),
    VerificationFailed(VerificationResult),
    ParseError(String),
}

impl std::fmt::Display for LearningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LearningError::KnowledgeError(e) => write!(f, "Knowledge error: {}", e),
            LearningError::TypeError(e) => write!(f, "Type error: {}", e),
            LearningError::VerificationFailed(v) => {
                write!(f, "Verification failed: {} violations", v.violation_count())
            }
            LearningError::ParseError(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl std::error::Error for LearningError {}

/// Contextual query result
#[derive(Debug, Clone)]
pub struct ContextualResult {
    pub direct_results: Vec<Deduction>,
    pub common_sense_inferences: Vec<Inference>,
    pub causal_effects: Vec<CausalEffect>,
}

/// Explanation for a conclusion
#[derive(Debug, Clone)]
pub struct Explanation {
    pub conclusion: Fact,
    pub proof_steps: Vec<ProofStep>,
    pub confidence: Probability,
    pub reasoning_type: ReasoningType,
}

/// A step in an explanation
#[derive(Debug, Clone)]
pub struct ProofStep {
    pub step_number: usize,
    pub description: String,
    pub support: SupportLevel,
}

/// Level of support
#[derive(Debug, Clone, Copy)]
pub enum SupportLevel {
    Direct,
    Inferred,
    Assumed,
}

/// Type of reasoning used
#[derive(Debug, Clone, Copy)]
pub enum ReasoningType {
    Deductive,
    Inductive,
    Abductive,
    Causal,
    CommonSense,
}

/// Causal analysis result
#[derive(Debug, Clone)]
pub struct CausalAnalysisResult {
    pub effect: EntityId,
    pub direct_causes: Vec<Entity>,
    pub causal_chain: Vec<crate::knowledge::causal::CausalLink>,
    pub confidence: Probability,
}

/// A contradiction in knowledge
#[derive(Debug, Clone)]
pub struct Contradiction {
    pub fact1: crate::knowledge::knowledge_graph::Fact,
    pub fact2: Option<crate::knowledge::knowledge_graph::Fact>,
    pub contradiction_type: ContradictionType,
}

/// Type of contradiction
#[derive(Debug, Clone, Copy)]
pub enum ContradictionType {
    MutualExclusion,
    DoubleNegation,
    InconsistentProperty,
}

/// Hypothetical for commonsense reasoning
#[derive(Debug, Clone)]
pub struct Hypothetical {
    pub original_situation: Situation,
    pub changes: Vec<crate::knowledge::commonsense::HypotheticalChange>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mind_creation() {
        let mind = TariroMind::new();
        assert_eq!(mind.stats.queries_processed, 0);
    }

    #[test]
    fn test_mind_with_config() {
        let config = MindConfig {
            use_commonsense: true,
            use_causal: true,
            use_verification: true,
            use_logic: true,
            max_deduction_depth: 5,
            auto_verify: false,
            learning_rate: 0.3,
        };

        let mind = TariroMind::with_config(config);
        assert_eq!(mind.config.max_deduction_depth, 5);
        assert!(!mind.config.auto_verify);
    }

    #[test]
    fn test_learn_fact() {
        let mut mind = TariroMind::new();

        let subject = Entity::new("Socrates", crate::knowledge::knowledge_graph::EntityType::Agent);
        let object = Entity::new("Human", crate::knowledge::knowledge_graph::EntityType::Concept);
        let fact = Fact::new(subject, Predicate::IsA, object);

        let result = mind.learn_fact(fact);
        assert!(result.is_ok());
        assert_eq!(mind.stats.facts_learned, 1);
    }

    #[test]
    fn test_verify() {
        let mut mind = TariroMind::new();

        let subject = Entity::new("Test", crate::knowledge::knowledge_graph::EntityType::Concept);
        let object = Entity::new("Other", crate::knowledge::knowledge_graph::EntityType::Concept);
        let fact = Fact::new(subject, Predicate::RelatedTo, object);

        let _ = mind.learn_fact(fact);

        let verification = mind.verify();
        assert!(verification.is_valid());
    }

    #[test]
    fn test_query() {
        let mut mind = TariroMind::new();

        let subject = Entity::new("A", crate::knowledge::knowledge_graph::EntityType::Concept);
        let object = Entity::new("B", crate::knowledge::knowledge_graph::EntityType::Concept);
        let fact = Fact::new(subject, Predicate::RelatedTo, object);

        let _ = mind.learn_fact(fact);

        let query = Query::new()
            .with_subject(EntityId::new("A"));

        let result = mind.query(query);
        assert_eq!(mind.stats.queries_processed, 1);
    }

    #[test]
    fn test_causal_analysis() {
        let mut mind = TariroMind::new();

        // Add causal fact
        let fire = Entity::new("Fire", crate::knowledge::knowledge_graph::EntityType::Concrete);
        let smoke = Entity::new("Smoke", crate::knowledge::knowledge_graph::EntityType::Event);
        let fact = Fact::new(fire, Predicate::Causes, smoke);

        let _ = mind.learn_fact(fact);

        let result = mind.analyze_causation(&EntityId::new("Smoke"));
        assert_eq!(result.direct_causes.len(), 1);
    }

    #[test]
    fn test_export_import() {
        let mut mind = TariroMind::new();

        let subject = Entity::new("Test", crate::knowledge::knowledge_graph::EntityType::Concept);
        let object = Entity::new("Value", crate::knowledge::knowledge_graph::EntityType::Concept);
        let fact = Fact::new(subject, Predicate::RelatedTo, object);

        let _ = mind.learn_fact(fact);

        // Export
        let json = mind.export_knowledge();
        assert!(!json.is_empty());

        // Import to new mind
        let mut new_mind = TariroMind::new();
        let result = new_mind.import_knowledge(&json);
        assert!(result.is_ok());
    }
}
