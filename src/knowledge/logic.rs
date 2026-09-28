//! Tariro Logic Programming Engine
//!
//! Datalog/Prolog-style logic programming with explicit rules and facts.

use crate::prelude::*;
use crate::knowledge::knowledge_graph::{KnowledgeGraph, EntityId, Predicate, Probability};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ============================================================================
// Logic Programming Types
// ============================================================================

/// A term in logic programming
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    /// Constant term
    Constant(Constant),
    /// Variable term
    Variable(String),
    /// Compound term
    Compound {
        functor: String,
        args: Vec<Term>,
    },
}

impl Term {
    /// Create a constant term
    pub fn constant(c: impl Into<String>) -> Self {
        Term::Constant(Constant::String(c.into()))
    }

    /// Create a variable term
    pub fn var(name: impl Into<String>) -> Self {
        Term::Variable(name.into())
    }

    /// Create a compound term
    pub fn compound(functor: impl Into<String>, args: Vec<Term>) -> Self {
        Term::Compound {
            functor: functor.into(),
            args,
        }
    }

    /// Get all variables in this term
    pub fn variables(&self) -> HashSet<&str> {
        match self {
            Term::Constant(_) => HashSet::new(),
            Term::Variable(name) => {
                let mut vars = HashSet::new();
                vars.insert(name.as_str());
                vars
            }
            Term::Compound { args, .. } => {
                args.iter().flat_map(|t| t.variables()).collect()
            }
        }
    }

    /// Apply substitution to term
    pub fn apply(&self, subst: &Substitution) -> Term {
        match self {
            Term::Constant(_) => self.clone(),
            Term::Variable(name) => {
                subst.get(name).map(|v| v.to_term()).unwrap_or(self.clone())
            }
            Term::Compound { functor, args } => {
                Term::Compound {
                    functor: functor.clone(),
                    args: args.iter().map(|a| a.apply(subst)).collect(),
                }
            }
        }
    }
}

/// Constants in logic programming
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Constant {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Entity(EntityId),
}

impl Constant {
    pub fn as_entity_id(&self) -> Option<&EntityId> {
        match self {
            Constant::Entity(id) => Some(id),
            _ => None,
        }
    }
}

/// A literal (atom or negated atom)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub predicate: String,
    pub terms: Vec<Term>,
    pub negated: bool,
}

impl Literal {
    pub fn new(predicate: impl Into<String>, terms: Vec<Term>) -> Self {
        Literal {
            predicate: predicate.into(),
            terms,
            negated: false,
        }
    }

    pub fn negated(mut self) -> Self {
        self.negated = true;
        self
    }

    /// Get variables in this literal
    pub fn variables(&self) -> HashSet<&str> {
        self.terms.iter().flat_map(|t| t.variables()).collect()
    }

    /// Apply substitution
    pub fn apply(&self, subst: &Substitution) -> Literal {
        Literal {
            predicate: self.predicate.clone(),
            terms: self.terms.iter().map(|t| t.apply(subst)).collect(),
            negated: self.negated,
        }
    }
}

/// A clause (rule or fact)
#[derive(Debug, Clone)]
pub enum Clause {
    /// Fact: Head.
    Fact {
        head: Literal,
    },
    /// Rule: Head :- Body.
    Rule {
        head: Literal,
        body: Vec<Literal>,
    },
}

impl Clause {
    /// Create a fact
    pub fn fact(predicate: impl Into<String>, terms: Vec<Term>) -> Self {
        Clause::Fact {
            head: Literal::new(predicate, terms),
        }
    }

    /// Create a rule
    pub fn rule(head: Literal, body: Vec<Literal>) -> Self {
        Clause::Rule { head, body }
    }

    /// Parse from string (simplified)
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        // Simplified parser
        if s.contains(":-") {
            // Rule
            let parts: Vec<&str> = s.split(":-").collect();
            if parts.len() != 2 {
                return Err(ParseError::InvalidSyntax(s.to_string()));
            }

            let head = Self::parse_literal(parts[0].trim())?;
            let body_strs: Vec<&str> = parts[1].split(',')
                .map(|s| s.trim())
                .collect();
            let mut body = Vec::new();
            for body_str in body_strs {
                body.push(Self::parse_literal(body_str)?);
            }

            Ok(Clause::rule(head, body))
        } else {
            // Fact
            Ok(Clause::fact_from_string(s))
        }
    }

    fn parse_literal(s: &str) -> Result<Literal, ParseError> {
        let s = s.trim();
        let negated = s.starts_with("not ");
        let s = if negated { &s[4..] } else { s };

        // Parse predicate(terms)
        if let Some(paren_start) = s.find('(') {
            let pred = &s[..paren_start];
            let terms_str = &s[paren_start + 1..s.len() - 1];
            let terms: Vec<Term> = terms_str.split(',')
                .map(|s| Self::parse_term(s.trim()))
                .collect();

            Ok(Literal {
                predicate: pred.to_string(),
                terms,
                negated,
            })
        } else {
            Ok(Literal::new(s, vec![]))
        }
    }

    fn parse_term(s: &str) -> Term {
        let s = s.trim();
        if s.is_empty() {
            return Term::constant("_");
        }

        // Check if uppercase (variable)
        if s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            Term::var(s)
        } else if s.chars().next().map(|c| c.is_lowercase()).unwrap_or(false) {
            // Check for compound term
            if let Some(paren) = s.find('(') {
                let functor = &s[..paren];
                let args_str = &s[paren + 1..s.len() - 1];
                let args: Vec<Term> = args_str.split(',')
                    .map(Self::parse_term)
                    .collect();
                Term::compound(functor, args)
            } else {
                Term::constant(s)
            }
        } else {
            Term::constant(s)
        }
    }

    fn fact_from_string(s: &str) -> Self {
        let s = s.trim().trim_end('.');
        Self::parse_literal(s).map(|head| Clause::Fact { head }).unwrap_or_else(|_| {
            Clause::fact("unknown", vec![])
        })
    }
}

/// Parse error
#[derive(Debug, Clone)]
pub struct ParseError(pub String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

/// Substitution for unification
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    bindings: HashMap<String, Term>,
}

impl Substitution {
    pub fn new() -> Self {
        Substitution { bindings: HashMap::new() }
    }

    pub fn insert(&mut self, var: String, term: Term) {
        self.bindings.insert(var, term);
    }

    pub fn get(&self, var: &str) -> Option<&Term> {
        self.bindings.get(var)
    }

    pub fn merge(&mut self, other: Substitution) {
        for (k, v) in other.bindings {
            self.bindings.insert(k, v);
        }
    }

    pub fn apply(&self, term: &Term) -> Term {
        term.apply(self)
    }
}

// ============================================================================
// Datalog Program
// ============================================================================

/// Extensional Database (facts)
#[derive(Debug, Clone, Default)]
pub struct EDB {
    facts: HashSet<Atom>,
}

impl EDB {
    pub fn new() -> Self {
        EDB { facts: HashSet::new() }
    }

    pub fn add(&mut self, atom: Atom) {
        self.facts.insert(atom);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Atom> {
        self.facts.iter()
    }

    pub fn match_atoms(&self, pattern: &Atom) -> Vec<Substitution> {
        self.facts.iter()
            .filter(|atom| atom.unifies_with(pattern, &mut Substitution::new()).is_some())
            .filter_map(|atom| atom.unifies_with(pattern, &mut Substitution::new()))
            .collect()
    }
}

/// Intensional Database (rules)
#[derive(Debug, Clone, Default)]
pub struct IDB {
    rules: Vec<Clause>,
}

impl IDB {
    pub fn new() -> Self {
        IDB { rules: Vec::new() }
    }

    pub fn add(&mut self, clause: Clause) {
        self.rules.push(clause);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Clause> {
        self.rules.iter()
    }

    pub fn get_rules_for(&self, predicate: &str) -> Vec<&Clause> {
        self.rules.iter()
            .filter(|clause| clause.head_predicate() == Some(predicate))
            .collect()
    }
}

/// An atom (predicate applied to terms)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Atom {
    pub predicate: String,
    pub terms: Vec<Term>,
}

impl Atom {
    pub fn new(predicate: impl Into<String>, terms: Vec<Term>) -> Self {
        Atom {
            predicate: predicate.into(),
            terms,
        }
    }

    /// Unify with another atom
    pub fn unifies_with(&self, other: &Atom, subst: &mut Substitution) -> Option<Substitution> {
        if self.predicate != other.predicate {
            return None;
        }

        if self.terms.len() != other.terms.len() {
            return None;
        }

        let mut sub = subst.clone();

        for (t1, t2) in self.terms.iter().zip(other.terms.iter()) {
            if let Some(s) = unify_terms(t1, t2, &mut sub) {
                sub = s;
            } else {
                return None;
            }
        }

        Some(sub)
    }
}

fn unify_terms(t1: &Term, t2: &Term, subst: &mut Substitution) -> Option<Substitution> {
    // Apply current substitution
    let t1 = t1.apply(subst);
    let t2 = t2.apply(subst);

    match (&t1, &t2) {
        // Variable unifies with anything
        (Term::Variable(v), t) | (t, Term::Variable(v)) => {
            if occurs_check(v, t) {
                None
            } else {
                let mut new_sub = subst.clone();
                new_sub.insert(v.clone(), t.clone());
                Some(new_sub)
            }
        }
        // Same constants unify
        (Term::Constant(c1), Term::Constant(c2)) if c1 == c2 => Some(subst.clone()),
        // Compound terms
        (Term::Compound { functor: f1, args: a1 }, Term::Compound { functor: f2, args: a2 })
            if f1 == f2 && a1.len() == a2.len() => {
            let mut sub = subst.clone();
            for (arg1, arg2) in a1.iter().zip(a2.iter()) {
                if let Some(s) = unify_terms(arg1, arg2, &mut sub) {
                    sub = s;
                } else {
                    return None;
                }
            }
            Some(sub)
        }
        _ => None,
    }
}

fn occurs_check(var: &str, term: &Term) -> bool {
    match term {
        Term::Variable(v) => v == var,
        Term::Compound { args, .. } => args.iter().any(|a| occurs_check(var, a)),
        Term::Constant(_) => false,
    }
}

impl Clause {
    pub fn head_predicate(&self) -> Option<&str> {
        match self {
            Clause::Fact { head } => Some(head.predicate.as_str()),
            Clause::Rule { head, .. } => Some(head.predicate.as_str()),
        }
    }
}

// ============================================================================
// Logic Engine
// ============================================================================

/// Datalog logic programming engine
pub struct LogicEngine {
    /// Extensional database (facts)
    edb: EDB,

    /// Intensional database (rules)
    idb: IDB,

    /// Knowledge graph reference
    kb: Arc<RwLock<KnowledgeGraph>>,

    /// Maximum recursion depth
    max_depth: usize,
}

impl LogicEngine {
    /// Create a new logic engine
    pub fn new(kb: Arc<RwLock<KnowledgeGraph>>) -> Self {
        LogicEngine {
            edb: EDB::new(),
            idb: IDB::new(),
            kb,
            max_depth: 10,
        }
    }

    /// Add a fact
    pub fn assert_fact(&mut self, predicate: impl Into<String>, terms: Vec<Term>) {
        self.edb.add(Atom::new(predicate, terms));
    }

    /// Add a clause
    pub fn assert_clause(&mut self, clause: Clause) {
        self.idb.add(clause);
    }

    /// Parse and add a clause from string
    pub fn assert_string(&mut self, s: &str) -> Result<(), ParseError> {
        let clause = Clause::parse(s)?;
        match clause {
            Clause::Fact { .. } => {
                // Convert to EDB atom
                if let Clause::Fact { head } = clause {
                    self.edb.add(Atom::new(head.predicate, head.terms));
                }
            }
            Clause::Rule { .. } => {
                self.idb.add(clause);
            }
        }
        Ok(())
    }

    /// Query the program
    pub fn query(&self, predicate: &str, terms: &[Term]) -> Vec<Substitution> {
        let query_atom = Atom::new(predicate, terms.to_vec());
        self.sld_resolve(&query_atom, 0)
    }

    /// SLD resolution (Selective Linear Definite clause resolution)
    fn sld_resolve(&self, goal: &Atom, depth: usize) -> Vec<Substitution> {
        if depth > self.max_depth {
            return Vec::new();
        }

        let mut results = Vec::new();

        // Try facts first
        for atom in self.edb.iter() {
            if atom.predicate == goal.predicate {
                let mut subst = Substitution::new();
                if let Some(s) = atom.unifies_with(goal, &mut subst) {
                    results.push(s);
                }
            }
        }

        // Try rules
        for clause in self.idb.get_rules_for(&goal.predicate) {
            let clause_substs = self.resolve_clause(clause, goal, depth);
            results.extend(clause_substs);
        }

        results
    }

    /// Resolve a clause against a goal
    fn resolve_clause(&self, clause: &Clause, goal: &Atom, depth: usize) -> Vec<Substitution> {
        match clause {
            Clause::Fact { head } => {
                let mut subst = Substitution::new();
                head.unifies_with(goal, &mut subst).map(|s| vec![s]).unwrap_or_default()
            }
            Clause::Rule { head, body } => {
                // Unify head with goal
                let mut subst = Substitution::new();
                if let Some(s) = head.unifies_with(goal, &mut subst) {
                    // Prove body goals
                    self.prove_goals(body, s, depth + 1)
                } else {
                    Vec::new()
                }
            }
        }
    }

    /// Prove a list of goals (conjunction)
    fn prove_goals(&self, goals: &[Literal], mut subst: Substitution, depth: usize) -> Vec<Substitution> {
        if goals.is_empty() {
            return vec![subst];
        }

        let first = &goals[0];
        let rest = &goals[1..];

        // Create atom from literal
        let atom = Atom::new(&first.predicate, first.terms.clone());

        // Resolve first goal
        let first_results = self.sld_resolve(&atom, depth);

        let mut all_results = Vec::new();

        for sub in first_results {
            let mut combined = subst.clone();
            combined.merge(sub);

            // Prove rest with combined substitution
            let rest_results = self.prove_goals(rest, combined, depth);
            all_results.extend(rest_results);
        }

        all_results
    }

    /// Evaluate a boolean query
    pub fn boolean_query(&self, predicate: &str, terms: &[Term]) -> bool {
        !self.query(predicate, terms).is_empty()
    }

    /// Get all solutions for a query
    pub fn all_solutions(&self, predicate: &str, terms: &[Term]) -> Vec<HashMap<String, Term>> {
        self.query(predicate, terms)
            .into_iter()
            .map(|subst| {
                subst.bindings
                    .into_iter()
                    .collect()
            })
            .collect()
    }

    /// Set maximum recursion depth
    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }

    /// Load facts from knowledge graph
    pub fn load_from_kb(&mut self) {
        let kb = self.kb.read().unwrap();

        for rel in kb.relations.values() {
            // Convert relation to atom
            let subject = Term::constant(&rel.subject.0);
            let object = Term::constant(&rel.object.0);

            let atom = Atom::new(rel.predicate.name(), vec![subject, object]);
            self.edb.add(atom);
        }
    }
}

// ============================================================================
// Example: Common Rules
// ============================================================================

impl LogicEngine {
    /// Add common knowledge base rules
    pub fn add_common_rules(&mut self) {
        // Socrates is mortal (basic fact)
        self.assert_string("mortal(socrates)").ok();

        // All humans are mortal (rule)
        self.assert_string("mortal(X) :- human(X)").ok();

        // All birds can fly (with exceptions via negation)
        self.assert_string("can_fly(X) :- bird(X), not penguin(X)").ok();

        // Ancestor relationship
        self.assert_string("ancestor(X, Y) :- parent(X, Y)").ok();
        self.assert_string("ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y)").ok();

        // Transitive closure
        self.assert_string("connected(X, Y) :- edge(X, Y)").ok();
        self.assert_string("connected(X, Y) :- edge(X, Z), connected(Z, Y)").ok();
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_parsing() {
        let t = Term::var("X");
        assert!(matches!(t, Term::Variable(_)));

        let t = Term::constant("john");
        assert!(matches!(t, Term::Constant(_)));
    }

    #[test]
    fn test_literal_creation() {
        let lit = Literal::new("father", vec![
            Term::constant("socrates"),
            Term::var("X"),
        ]);

        assert_eq!(lit.predicate, "father");
        assert_eq!(lit.terms.len(), 2);
        assert!(!lit.negated);
    }

    #[test]
    fn test_clause_parsing() {
        let clause = Clause::parse("mortal(X) :- human(X)").unwrap();

        match clause {
            Clause::Rule { head, body } => {
                assert_eq!(head.predicate, "mortal");
                assert_eq!(body.len(), 1);
                assert_eq!(body[0].predicate, "human");
            }
            _ => panic!("Expected Rule"),
        }
    }

    #[test]
    fn test_fact_parsing() {
        let clause = Clause::parse("father(socrates, sophroniscus).").unwrap();

        match clause {
            Clause::Fact { head } => {
                assert_eq!(head.predicate, "father");
                assert_eq!(head.terms.len(), 2);
            }
            _ => panic!("Expected Fact"),
        }
    }

    #[test]
    fn test_atom_unification() {
        let a1 = Atom::new("parent", vec![
            Term::constant("john"),
            Term::var("Y"),
        ]);

        let a2 = Atom::new("parent", vec![
            Term::var("X"),
            Term::constant("jane"),
        ]);

        let mut subst = Substitution::new();
        let result = a1.unifies_with(&a2, &mut subst);

        assert!(result.is_some());
        let s = result.unwrap();
        assert_eq!(s.get("X"), Some(&Term::constant("john")));
        assert_eq!(s.get("Y"), Some(&Term::constant("jane")));
    }

    #[test]
    fn test_logic_engine_query() {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));
        let mut engine = LogicEngine::new(kb);

        // Add facts
        engine.assert_fact("parent", vec![
            Term::constant("alice"),
            Term::constant("bob"),
        ]);
        engine.assert_fact("parent", vec![
            Term::constant("bob"),
            Term::constant("charlie"),
        ]);

        // Query
        let results = engine.query("parent", &[
            Term::var("X"),
            Term::constant("charlie"),
        ]);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].get("X"), Some(&Term::constant("bob")));
    }

    #[test]
    fn test_logic_engine_string_parse() {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));
        let mut engine = LogicEngine::new(kb);

        engine.assert_string("mortal(socrates)").unwrap();

        let result = engine.boolean_query("mortal", &[Term::constant("socrates")]);
        assert!(result);
    }

    #[test]
    fn test_recursive_rule() {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));
        let mut engine = LogicEngine::new(kb);

        // Facts
        engine.assert_string("parent(alice, bob)").unwrap();
        engine.assert_string("parent(bob, charlie)").unwrap();

        // Recursive rule
        engine.assert_string("ancestor(X, Y) :- parent(X, Y)").unwrap();
        engine.assert_string("ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y)").unwrap();

        // Query
        let results = engine.query("ancestor", &[
            Term::var("X"),
            Term::constant("charlie"),
        ]);

        // Should find bob and alice as ancestors of charlie
        assert!(results.len() >= 2);
    }
}
