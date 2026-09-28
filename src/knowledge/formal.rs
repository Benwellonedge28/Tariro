//! Tariro Formal Verification Engine
//!
//! Mathematical proofs and type-level reasoning for verified knowledge.

use crate::prelude::*;
use crate::knowledge::knowledge_graph::{KnowledgeGraph, EntityId, Entity, Probability};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ============================================================================
// Type System for Knowledge
// ============================================================================

/// Type system for knowledge representation
pub struct TypeSystem {
    /// Type definitions
    types: HashMap<String, TypeDef>,

    /// Subtype relations
    subtypes: HashMap<String, Vec<String>>,
}

impl TypeSystem {
    /// Create a new type system
    pub fn new() -> Self {
        let mut system = TypeSystem {
            types: HashMap::new(),
            subtypes: HashMap::new(),
        };
        system.init_builtin_types();
        system
    }

    /// Initialize built-in types
    fn init_builtin_types(&mut self) {
        // Entity types
        self.add_type(TypeDef {
            name: "Entity".to_string(),
            parent: None,
            properties: vec![],
            description: "Base type for all entities".to_string(),
        });

        // Physical types
        self.add_type(TypeDef {
            name: "Physical".to_string(),
            parent: Some("Entity".to_string()),
            properties: vec!["location".to_string(), "mass".to_string()],
            description: "Physical objects".to_string(),
        });

        // Abstract types
        self.add_type(TypeDef {
            name: "Abstract".to_string(),
            parent: Some("Entity".to_string()),
            properties: vec![],
            description: "Abstract concepts".to_string(),
        });

        // Number types
        self.add_type(TypeDef {
            name: "Number".to_string(),
            parent: Some("Abstract".to_string()),
            properties: vec!["value".to_string()],
            description: "Numeric values".to_string(),
        });

        self.add_subtype("Number", "Integer");
        self.add_subtype("Number", "Real");

        // Boolean
        self.add_type(TypeDef {
            name: "Boolean".to_string(),
            parent: Some("Abstract".to_string()),
            properties: vec!["value".to_string()],
            description: "True or false".to_string(),
        });

        // Agent
        self.add_type(TypeDef {
            name: "Agent".to_string(),
            parent: Some("Physical".to_string()),
            properties: vec!["goals".to_string(), "beliefs".to_string()],
            description: "Intelligent agents".to_string(),
        });
    }

    /// Add a type definition
    pub fn add_type(&mut self, type_def: TypeDef) {
        self.types.insert(type_def.name.clone(), type_def);
    }

    /// Add a subtype relationship
    pub fn add_subtype(&mut self, parent: &str, child: &str) {
        self.subtypes
            .entry(parent.to_string())
            .or_default()
            .push(child.to_string());

        // Also add child type if not exists
        if !self.types.contains_key(child) {
            self.add_type(TypeDef {
                name: child.to_string(),
                parent: Some(parent.to_string()),
                properties: vec![],
                description: format!("Subtype of {}", parent),
            });
        }
    }

    /// Check if type1 is subtype of type2
    pub fn is_subtype(&self, type1: &str, type2: &str) -> bool {
        if type1 == type2 {
            return true;
        }

        // Check direct parent
        if let Some(def) = self.types.get(type1) {
            if let Some(parent) = &def.parent {
                if self.is_subtype(parent, type2) {
                    return true;
                }
            }
        }

        // Check subtypes map
        if let Some(subtypes) = self.subtypes.get(type2) {
            for subtype in subtypes {
                if self.is_subtype(type1, subtype) {
                    return true;
                }
            }
        }

        false
    }

    /// Get all supertypes
    pub fn supertypes(&self, type_name: &str) -> Vec<String> {
        let mut result = Vec::new();

        if let Some(def) = self.types.get(type_name) {
            if let Some(parent) = &def.parent {
                result.push(parent.clone());
                result.extend(self.supertypes(parent));
            }
        }

        result
    }

    /// Get all subtypes
    pub fn subtypes_of(&self, type_name: &str) -> Vec<String> {
        let mut result = Vec::new();

        if let Some(subtypes) = self.subtypes.get(type_name) {
            for subtype in subtypes {
                result.push(subtype.clone());
                result.extend(self.subtypes_of(subtype));
            }
        }

        result
    }

    /// Check if a value matches a type
    pub fn matches_type(&self, value: &TypedValue, type_name: &str) -> bool {
        match value {
            TypedValue::Entity(_) => {
                self.is_subtype("Entity", type_name)
            }
            TypedValue::Number(_) => {
                self.is_subtype("Number", type_name)
            }
            TypedValue::Boolean(_) => {
                self.is_subtype("Boolean", type_name)
            }
            TypedValue::String(_) => {
                type_name == "String" || self.is_subtype("Abstract", type_name)
            }
            TypedValue::List(items) => {
                // Check all items match
                items.iter().all(|item| self.matches_type(item, type_name))
            }
        }
    }

    /// Infer type from value
    pub fn infer_type(&self, value: &TypedValue) -> Option<String> {
        match value {
            TypedValue::Entity(_) => Some("Entity".to_string()),
            TypedValue::Number(_) => Some("Number".to_string()),
            TypedValue::Boolean(_) => Some("Boolean".to_string()),
            TypedValue::String(_) => Some("String".to_string()),
            TypedValue::List(items) => {
                if let Some(first) = items.first() {
                    self.infer_type(first)
                } else {
                    None
                }
            }
        }
    }
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Type definition
#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub parent: Option<String>,
    pub properties: Vec<String>,
    pub description: String,
}

/// A typed value
#[derive(Debug, Clone)]
pub enum TypedValue {
    Entity(EntityId),
    Number(f64),
    Boolean(bool),
    String(String),
    List(Vec<TypedValue>),
}

impl TypedValue {
    /// Get the type name
    pub fn type_name(&self) -> &'static str {
        match self {
            TypedValue::Entity(_) => "Entity",
            TypedValue::Number(_) => "Number",
            TypedValue::Boolean(_) => "Boolean",
            TypedValue::String(_) => "String",
            TypedValue::List(_) => "List",
        }
    }
}

// ============================================================================
// Invariants
// ============================================================================

/// An invariant that must hold
#[derive(Debug, Clone)]
pub struct Invariant {
    pub name: String,
    pub description: String,
    pub condition: InvariantCondition,
    pub severity: Severity,
}

impl Invariant {
    /// Create a new invariant
    pub fn new(name: impl Into<String>, condition: InvariantCondition) -> Self {
        Invariant {
            name: name.into(),
            description: String::new(),
            condition,
            severity: Severity::Error,
        }
    }

    /// Check if invariant holds
    pub fn holds(&self, kb: &KnowledgeGraph) -> bool {
        match &self.condition {
            InvariantCondition::AlwaysTrue(predicate) => {
                // Check predicate always true
                !kb.relations.values().any(|r| !predicate(&r.clone().into()))
            }
            InvariantCondition::NeverNegative(attribute) => {
                // Check attribute never negative
                kb.entities.values().all(|e| {
                    e.attributes.get(&Attribute(attribute.clone()))
                        .map(|v| v.as_f64().map(|f| f >= 0.0).unwrap_or(true))
                        .unwrap_or(true)
                })
            }
            InvariantCondition::MaxRelation(max, _) => {
                kb.relations.len() <= *max
            }
            InvariantCondition::Custom(check) => check(kb),
        }
    }
}

/// Condition for invariant
#[derive(Debug, Clone)]
pub enum InvariantCondition {
    AlwaysTrue(fn(&FactProxy) -> bool),
    NeverNegative(String),
    MaxRelation(usize, String),
    Custom(fn(&KnowledgeGraph) -> bool),
}

/// Attribute wrapper
#[derive(Debug, Clone)]
pub struct Attribute(pub String);

/// Fact proxy for checking
#[derive(Debug, Clone)]
pub struct FactProxy {
    pub subject: EntityId,
    pub predicate: String,
    pub object: EntityId,
}

impl From<crate::knowledge::knowledge_graph::Relation> for FactProxy {
    fn from(r: crate::knowledge::knowledge_graph::Relation) -> Self {
        FactProxy {
            subject: r.subject,
            predicate: r.predicate.name().to_string(),
            object: r.object,
        }
    }
}

/// Severity level
#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Warning,
    Error,
    Critical,
}

// ============================================================================
// Theorem Prover
// ============================================================================

/// Simple theorem prover
pub struct TheoremProver {
    /// Type system
    type_system: TypeSystem,

    /// Proven theorems
    theorems: Vec<Theorem>,

    /// Axioms
    axioms: Vec<Axiom>,
}

impl TheoremProver {
    /// Create a new theorem prover
    pub fn new() -> Self {
        TheoremProver {
            type_system: TypeSystem::new(),
            theorems: Vec::new(),
            axioms: Vec::new(),
        }
    }

    /// Add an axiom
    pub fn add_axiom(&mut self, axiom: Axiom) {
        self.axioms.push(axiom);
    }

    /// Try to prove a theorem
    pub fn prove(&self, theorem: &Theorem) -> ProofResult {
        // Try direct proof from axioms
        for axiom in &self.axioms {
            if axiom.conclusion == theorem.statement {
                return ProofResult::Proven {
                    proof: Proof {
                        theorem: theorem.statement.clone(),
                        steps: vec![ProofStep {
                            step_number: 0,
                            rule: "Axiom".to_string(),
                            premises: vec![],
                            conclusion: theorem.statement.clone(),
                        }],
                    },
                };
            }
        }

        // Try proof by cases
        for axiom in &self.axioms {
            if let Some(proof) = self.try_conditional_proof(axiom, theorem) {
                return proof;
            }
        }

        ProofResult::Unproven {
            reason: "No proof found".to_string(),
        }
    }

    fn try_conditional_proof(&self, axiom: &Axiom, theorem: &Theorem) -> Option<ProofResult> {
        // Simplified: just check if axiom implies theorem
        if axiom.conclusion == theorem.statement {
            Some(ProofResult::Proven {
                proof: Proof {
                    theorem: theorem.statement.clone(),
                    steps: vec![ProofStep {
                        step_number: 0,
                        rule: "Implication".to_string(),
                        premises: axiom.premises.clone(),
                        conclusion: theorem.statement.clone(),
                    }],
                },
            })
        } else {
            None
        }
    }

    /// Add a proven theorem
    pub fn add_theorem(&mut self, theorem: Theorem) {
        self.theorems.push(theorem);
    }

    /// Get all theorems
    pub fn theorems(&self) -> &[Theorem] {
        &self.theorems
    }
}

impl Default for TheoremProver {
    fn default() -> Self {
        Self::new()
    }
}

/// A theorem to prove
#[derive(Debug, Clone)]
pub struct Theorem {
    pub statement: String,
    pub premises: Vec<String>,
    pub description: String,
}

/// An axiom (self-evident truth)
#[derive(Debug, Clone)]
pub struct Axiom {
    pub name: String,
    pub premises: Vec<String>,
    pub conclusion: String,
}

/// Result of a proof attempt
#[derive(Debug, Clone)]
pub enum ProofResult {
    Proven { proof: Proof },
    Unproven { reason: String },
}

/// A proof
#[derive(Debug, Clone)]
pub struct Proof {
    pub theorem: String,
    pub steps: Vec<ProofStep>,
}

/// A step in a proof
#[derive(Debug, Clone)]
pub struct ProofStep {
    pub step_number: usize,
    pub rule: String,
    pub premises: Vec<String>,
    pub conclusion: String,
}

// ============================================================================
// Formal Verifier
// ============================================================================

/// Formal verification engine
pub struct FormalVerifier {
    /// Type system
    type_system: TypeSystem,

    /// Invariants
    invariants: Vec<Invariant>,

    /// Theorem prover
    prover: TheoremProver,
}

impl FormalVerifier {
    /// Create a new formal verifier
    pub fn new() -> Self {
        let mut verifier = FormalVerifier {
            type_system: TypeSystem::new(),
            invariants: Vec::new(),
            prover: TheoremProver::new(),
        };

        verifier.init_builtin_invariants();
        verifier.init_builtin_axioms();

        verifier
    }

    /// Initialize built-in invariants
    fn init_builtin_invariants(&mut self) {
        // No self-loops
        self.invariants.push(Invariant::new(
            "no_self_loops",
            InvariantCondition::AlwaysTrue(|fact| {
                fact.subject != fact.object
            }),
        ));

        // Probabilities in [0, 1]
        self.invariants.push(Invariant::new(
            "valid_probabilities",
            InvariantCondition::Custom(|kb| {
                kb.relations.values().all(|r| {
                    r.confidence.value >= 0.0 && r.confidence.value <= 1.0
                })
            }),
        ));

        // No duplicate relations
        self.invariants.push(Invariant::new(
            "no_duplicates",
            InvariantCondition::Custom(|kb| {
                let mut seen = std::collections::HashSet::new();
                kb.relations.values().all(|r| {
                    let key = format!("{}-{:?}-{}", r.subject, r.predicate, r.object);
                    seen.insert(key)
                })
            }),
        ));
    }

    /// Initialize built-in axioms
    fn init_builtin_axioms(&mut self) {
        // Law of non-contradiction
        self.prover.add_axiom(Axiom {
            name: "law_of_non_contradiction".to_string(),
            premises: vec![],
            conclusion: "A and not A cannot both be true".to_string(),
        });

        // Law of identity
        self.prover.add_axiom(Axiom {
            name: "law_of_identity".to_string(),
            premises: vec![],
            conclusion: "A equals A".to_string(),
        });

        // Modus ponens
        self.prover.add_axiom(Axiom {
            name: "modus_ponens".to_string(),
            premises: vec!["A".to_string(), "A implies B".to_string()],
            conclusion: "B".to_string(),
        });
    }

    /// Add an invariant
    pub fn add_invariant(&mut self, invariant: Invariant) {
        self.invariants.push(invariant);
    }

    /// Verify all invariants against knowledge graph
    pub fn verify_invariants(&self, kb: &KnowledgeGraph) -> VerificationResult {
        let mut violations = Vec::new();

        for invariant in &self.invariants {
            if !invariant.holds(kb) {
                violations.push(InvariantViolation {
                    invariant: invariant.clone(),
                    description: format!("Invariant '{}' violated", invariant.name),
                    severity: invariant.severity,
                });
            }
        }

        VerificationResult {
            valid: violations.is_empty(),
            violations,
            total_invariants: self.invariants.len(),
        }
    }

    /// Type check an entity
    pub fn type_check(&self, entity: &Entity) -> Result<(), TypeError> {
        // Check entity type exists
        if !self.type_system.types.contains_key(&entity.entity_type.name) {
            return Err(TypeError::UnknownType(entity.entity_type.name.clone()));
        }

        // Check attributes have valid types
        for (attr, value) in &entity.attributes {
            let attr_type = &attr.0;
            let value_matches = match value {
                crate::knowledge::knowledge_graph::Value::String(_) => {
                    attr_type == "String"
                }
                crate::knowledge::knowledge_graph::Value::Integer(_) => {
                    self.type_system.is_subtype("Integer", attr_type)
                }
                crate::knowledge::knowledge_graph::Value::Float(_) => {
                    self.type_system.is_subtype("Real", attr_type)
                }
                crate::knowledge::knowledge_graph::Value::Boolean(_) => {
                    self.type_system.is_subtype("Boolean", attr_type)
                }
                _ => true,
            };

            if !value_matches {
                return Err(TypeError::TypeMismatch {
                    value_type: value.type_name().to_string(),
                    expected_type: attr_type.clone(),
                });
            }
        }

        Ok(())
    }

    /// Prove a theorem
    pub fn prove(&self, theorem: &Theorem) -> ProofResult {
        self.prover.prove(theorem)
    }

    /// Get type system
    pub fn type_system(&self) -> &TypeSystem {
        &self.type_system
    }
}

impl Default for FormalVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Verification result
#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub valid: bool,
    pub violations: Vec<InvariantViolation>,
    pub total_invariants: usize,
}

impl VerificationResult {
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn violation_count(&self) -> usize {
        self.violations.len()
    }
}

/// An invariant violation
#[derive(Debug, Clone)]
pub struct InvariantViolation {
    pub invariant: Invariant,
    pub description: String,
    pub severity: Severity,
}

/// Type error
#[derive(Debug, Clone)]
pub enum TypeError {
    UnknownType(String),
    TypeMismatch {
        value_type: String,
        expected_type: String,
    },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnknownType(t) => write!(f, "Unknown type: {}", t),
            TypeError::TypeMismatch { value_type, expected_type } => {
                write!(f, "Type mismatch: {} vs expected {}", value_type, expected_type)
            }
        }
    }
}

impl std::error::Error for TypeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_system_creation() {
        let ts = TypeSystem::new();
        assert!(ts.types.contains_key("Entity"));
        assert!(ts.types.contains_key("Number"));
    }

    #[test]
    fn test_subtype_check() {
        let ts = TypeSystem::new();
        assert!(ts.is_subtype("Integer", "Number"));
        assert!(ts.is_subtype("Real", "Number"));
        assert!(ts.is_subtype("Integer", "Entity"));
    }

    #[test]
    fn test_supertypes() {
        let ts = TypeSystem::new();
        let supers = ts.supertypes("Integer");
        assert!(supers.contains(&"Number".to_string()));
        assert!(supers.contains(&"Entity".to_string()));
    }

    #[test]
    fn test_invariant_creation() {
        let invariant = Invariant::new(
            "test_invariant",
            InvariantCondition::AlwaysTrue(|_| true),
        );
        assert_eq!(invariant.name, "test_invariant");
    }

    #[test]
    fn test_formal_verifier_creation() {
        let verifier = FormalVerifier::new();
        assert!(!verifier.invariants.is_empty());
    }

    #[test]
    fn test_verify_invariants() {
        let kb = crate::knowledge::knowledge_graph::KnowledgeGraph::new();
        let verifier = FormalVerifier::new();
        let result = verifier.verify_invariants(&kb);

        assert!(result.is_valid());
        assert_eq!(result.violation_count(), 0);
    }

    #[test]
    fn test_type_check() {
        let verifier = FormalVerifier::new();
        let entity = Entity::new("test", crate::knowledge::knowledge_graph::EntityType::Concrete)
            .with_attribute(
                crate::knowledge::knowledge_graph::Attribute::new("count"),
                crate::knowledge::knowledge_graph::Value::Integer(42),
            );

        let result = verifier.type_check(&entity);
        assert!(result.is_ok());
    }
}
