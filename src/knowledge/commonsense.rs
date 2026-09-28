//! Tariro Common Sense Engine
//!
//! Explicit common sense knowledge and reasoning about everyday situations.

use crate::prelude::*;
use crate::knowledge::knowledge_graph::{KnowledgeGraph, EntityId, Entity, Probability};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ============================================================================
// Common Sense Types
// ============================================================================

/// A common sense rule
#[derive(Debug, Clone)]
pub struct CommonsenseRule {
    pub id: String,
    pub antecedent: Situation,
    pub consequent: Expectation,
    pub certainty: Certainty,
    pub source: CommonSenseSource,
    pub exceptions: Vec<Situation>,
    pub domain: Domain,
}

impl CommonsenseRule {
    /// Create a new rule
    pub fn new(id: impl Into<String>, antecedent: Situation, consequent: Expectation) -> Self {
        CommonsenseRule {
            id: id.into(),
            antecedent,
            consequent,
            certainty: Certainty::Usually,
            source: CommonSenseSource::Learned,
            exceptions: Vec::new(),
            domain: Domain::General,
        }
    }

    /// Set certainty level
    pub fn with_certainty(mut self, certainty: Certainty) -> Self {
        self.certainty = certainty;
        self
    }

    /// Check if rule applies (considering exceptions)
    pub fn applies_to(&self, situation: &Situation) -> bool {
        // Check exceptions first
        for exception in &self.exceptions {
            if exception.matches(situation) {
                return false;
            }
        }
        self.antecedent.matches(situation)
    }
}

/// Certainty level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Certainty {
    Always,
    Usually,
    Sometimes,
    Rarely,
}

impl Certainty {
    pub fn to_probability(&self) -> f64 {
        match self {
            Certainty::Always => 1.0,
            Certainty::Usually => 0.8,
            Certainty::Sometimes => 0.5,
            Certainty::Rarely => 0.2,
        }
    }
}

/// Domain of the rule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    General,
    Physical,
    Social,
    Mental,
    Temporal,
    Spatial,
    Biological,
}

/// Source of the rule
#[derive(Debug, Clone, Copy)]
pub enum CommonSenseSource {
    Innate,
    Learned,
    Induced,
    UserProvided,
    Cultural,
}

// ============================================================================
// Situation and Expectation
// ============================================================================

/// A situation description
#[derive(Debug, Clone)]
pub struct Situation {
    pub entities: Vec<EntityDescription>,
    pub conditions: Vec<Condition>,
    pub temporal: Option<TemporalInfo>,
}

impl Situation {
    /// Create an empty situation
    pub fn new() -> Self {
        Situation {
            entities: Vec::new(),
            conditions: Vec::new(),
            temporal: None,
        }
    }

    /// Add an entity
    pub fn with_entity(mut self, entity: EntityDescription) -> Self {
        self.entities.push(entity);
        self
    }

    /// Check if this situation matches another
    pub fn matches(&self, other: &Situation) -> bool {
        // Check conditions
        for condition in &self.conditions {
            if !other.conditions.iter().any(|c| c.equivalent(condition)) {
                return false;
            }
        }
        true
    }
}

impl Default for Situation {
    fn default() -> Self {
        Self::new()
    }
}

/// Description of an entity in a situation
#[derive(Debug, Clone)]
pub struct EntityDescription {
    pub entity_type: String,
    pub properties: Vec<String>,
    pub location: Option<String>,
    pub state: Option<String>,
}

/// A condition in a situation
#[derive(Debug, Clone)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub subject: String,
    pub predicate: String,
    pub object: Option<String>,
}

impl Condition {
    /// Check if two conditions are equivalent
    pub fn equivalent(&self, other: &Condition) -> bool {
        self.subject == other.subject
            && self.predicate == other.predicate
            && self.object == other.object
    }
}

/// Type of condition
#[derive(Debug, Clone)]
pub enum ConditionType {
    Property,
    Action,
    Relation,
    State,
}

/// Expected outcome
#[derive(Debug, Clone)]
pub struct Expectation {
    pub outcomes: Vec<Outcome>,
    pub reasoning: String,
}

impl Expectation {
    pub fn new(outcomes: Vec<Outcome>) -> Self {
        Expectation {
            outcomes,
            reasoning: String::new(),
        }
    }
}

/// A predicted outcome
#[derive(Debug, Clone)]
pub struct Outcome {
    pub description: String,
    pub probability: f64,
    pub conditions: Vec<Condition>,
}

// ============================================================================
// Temporal Information
// ============================================================================

#[derive(Debug, Clone)]
pub struct TemporalInfo {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub temporal_relation: Option<TemporalRelation>,
}

/// Temporal relations
#[derive(Debug, Clone, Copy)]
pub enum TemporalRelation {
    Before,
    After,
    During,
    Overlaps,
    Starts,
    Ends,
}

// ============================================================================
// Common Sense Knowledge Base
// ============================================================================

/// The common sense knowledge base
pub struct CommonsenseKB {
    /// Physical rules
    physics: Vec<CommonsenseRule>,

    /// Social rules
    social: Vec<CommonsenseRule>,

    /// Mental rules
    mental: Vec<CommonsenseRule>,

    /// Temporal rules
    temporal: Vec<CommonsenseRule>,

    /// Spatial rules
    spatial: Vec<CommonsenseRule>,

    /// General rules
    general: Vec<CommonsenseRule>,

    /// Knowledge graph reference
    kb: Arc<RwLock<KnowledgeGraph>>,
}

impl CommonsenseKB {
    /// Create a new common sense KB
    pub fn new(kb: Arc<RwLock<KnowledgeGraph>>) -> Self {
        let mut commonsense = CommonsenseKB {
            physics: Vec::new(),
            social: Vec::new(),
            mental: Vec::new(),
            temporal: Vec::new(),
            spatial: Vec::new(),
            general: Vec::new(),
            kb,
        };

        // Initialize with basic common sense
        commonsense.init_physics();
        commonsense.init_social();
        commonsense.init_mental();
        commonsense.init_temporal();

        commonsense
    }

    /// Initialize physical common sense
    fn init_physics(&mut self) {
        // Objects fall down, not up
        self.physics.push(CommonsenseRule::new(
            "physics_fall",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Object".to_string(),
                properties: vec!["above_ground".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Object moves downward".to_string(),
                probability: 1.0,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Always));

        // Fire causes heat
        self.physics.push(CommonsenseRule::new(
            "fire_heat",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Fire".to_string(),
                properties: vec!["burning".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Nearby objects may become hot".to_string(),
                probability: 0.9,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Usually));

        // Water is wet
        self.physics.push(CommonsenseRule::new(
            "water_wet",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Water".to_string(),
                properties: vec![],
                location: None,
                state: Some("liquid".to_string()),
            }),
            Expectation::new(vec![Outcome {
                description: "Water makes things wet".to_string(),
                probability: 1.0,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Always));

        // Glass breaks when dropped
        self.physics.push(CommonsenseRule::new(
            "glass_break",
            Situation::new()
                .with_entity(EntityDescription {
                    entity_type: "Glass".to_string(),
                    properties: vec![],
                    location: None,
                    state: None,
                })
                .with_entity(EntityDescription {
                    entity_type: "Floor".to_string(),
                    properties: vec!["hard".to_string()],
                    location: None,
                    state: None,
                }),
            Expectation::new(vec![Outcome {
                description: "Glass may break".to_string(),
                probability: 0.85,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Usually));

        // Hot things cool down over time
        self.physics.push(CommonsenseRule::new(
            "hot_cool",
            Situation::new().with_entity(EntityDescription {
                entity_type: "HotObject".to_string(),
                properties: vec!["hot".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Object cools down to room temperature".to_string(),
                probability: 1.0,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Always));
    }

    /// Initialize social common sense
    fn init_social(&mut self) {
        // People greet each other
        self.social.push(CommonsenseRule::new(
            "social_greet",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Person".to_string(),
                properties: vec!["meeting".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "People may greet each other".to_string(),
                probability: 0.7,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Sometimes));

        // People work for money
        self.social.push(CommonsenseRule::new(
            "work_money",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Person".to_string(),
                properties: vec!["working".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Person expects compensation".to_string(),
                probability: 0.9,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Usually));

        // Children need supervision
        self.social.push(CommonsenseRule::new(
            "children_supervision",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Child".to_string(),
                properties: vec![],
                location: Some("dangerous_place".to_string()),
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Adult supervision is needed".to_string(),
                probability: 0.95,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Usually));
    }

    /// Initialize mental common sense
    fn init_mental(&mut self) {
        // Hungry people seek food
        self.mental.push(CommonsenseRule::new(
            "hungry_seek_food",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Person".to_string(),
                properties: vec!["hungry".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Person will look for food".to_string(),
                probability: 0.85,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Usually));

        // Tired people sleep
        self.mental.push(CommonsenseRule::new(
            "tired_sleep",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Person".to_string(),
                properties: vec!["tired".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Person may sleep or rest".to_string(),
                probability: 0.8,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Usually));

        // Happy people smile
        self.mental.push(CommonsenseRule::new(
            "happy_smile",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Person".to_string(),
                properties: vec!["happy".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Person may smile".to_string(),
                probability: 0.7,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Sometimes));
    }

    /// Initialize temporal common sense
    fn init_temporal(&mut self) {
        // Night follows day
        self.temporal.push(CommonsenseRule::new(
            "night_after_day",
            Situation::new().with_entity(EntityDescription {
                entity_type: "TimePeriod".to_string(),
                properties: vec!["daytime".to_string()],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Night will come".to_string(),
                probability: 1.0,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Always));

        // Seasons change
        self.temporal.push(CommonsenseRule::new(
            "seasons_change",
            Situation::new().with_entity(EntityDescription {
                entity_type: "Season".to_string(),
                properties: vec![],
                location: None,
                state: None,
            }),
            Expectation::new(vec![Outcome {
                description: "Season will eventually change".to_string(),
                probability: 1.0,
                conditions: vec![],
            }]),
        ).with_certainty(Certainty::Always));
    }

    /// Get rules for a domain
    pub fn get_rules(&self, domain: Domain) -> &[CommonsenseRule] {
        match domain {
            Domain::Physical => &self.physics,
            Domain::Social => &self.social,
            Domain::Mental => &self.mental,
            Domain::Temporal => &self.temporal,
            Domain::Spatial => &self.spatial,
            Domain::General => &self.general,
            Domain::Biological => &self.general,
        }
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: CommonsenseRule) {
        match rule.domain {
            Domain::Physical => self.physics.push(rule),
            Domain::Social => self.social.push(rule),
            Domain::Mental => self.mental.push(rule),
            Domain::Temporal => self.temporal.push(rule),
            Domain::Spatial => self.spatial.push(rule),
            Domain::General | Domain::Biological => self.general.push(rule),
        }
    }

    /// Find applicable rules for a situation
    pub fn find_applicable(&self, situation: &Situation) -> Vec<&CommonsenseRule> {
        let mut applicable = Vec::new();

        // Check all domains
        for domain in [
            Domain::Physical,
            Domain::Social,
            Domain::Mental,
            Domain::Temporal,
            Domain::Spatial,
            Domain::General,
        ] {
            for rule in self.get_rules(domain) {
                if rule.applies_to(situation) {
                    applicable.push(rule);
                }
            }
        }

        applicable
    }
}

// ============================================================================
// Common Sense Engine
// ============================================================================

/// The common sense reasoning engine
pub struct CommonSenseEngine {
    /// Knowledge base
    kb: Arc<RwLock<KnowledgeGraph>>,

    /// Common sense knowledge
    commonsense: CommonsenseKB,

    /// Spatial reasoner
    spatial: SpatialReasoner,

    /// Temporal reasoner
    temporal: TemporalReasoner,
}

impl CommonSenseEngine {
    /// Create a new common sense engine
    pub fn new(kb: Arc<RwLock<KnowledgeGraph>>) -> Self {
        CommonSenseEngine {
            kb: kb.clone(),
            commonsense: CommonsenseKB::new(kb),
            spatial: SpatialReasoner::new(),
            temporal: TemporalReasoner::new(),
        }
    }

    /// Infer consequences of a situation
    pub fn infer_consequences(&self, situation: &Situation) -> Vec<Inference> {
        let mut inferences = Vec::new();

        // Find applicable common sense rules
        let applicable_rules = self.commonsense.find_applicable(situation);

        for rule in applicable_rules {
            inferences.push(Inference {
                rule_id: rule.id.clone(),
                expectation: rule.consequent.clone(),
                confidence: Probability::from_f64(rule.certainty.to_probability()),
                reasoning: format!(
                    "Based on {} rule: {}",
                    match rule.domain {
                        Domain::Physical => "physical",
                        Domain::Social => "social",
                        Domain::Mental => "mental",
                        Domain::Temporal => "temporal",
                        _ => "general",
                    },
                    rule.reasoning
                ),
            });
        }

        // Add spatial inferences
        let spatial_inferences = self.spatial.reason(&situation);
        inferences.extend(spatial_inferences);

        // Add temporal inferences
        let temporal_inferences = self.temporal.reason(&situation);
        inferences.extend(temporal_inferences);

        inferences
    }

    /// Answer "what if" questions
    pub fn what_if(&self, hypothetical: &Hypothetical) -> Vec<Inference> {
        // Create modified situation
        let mut situation = hypothetical.original_situation.clone();

        // Apply changes
        for change in &hypothetical.changes {
            situation.conditions.push(Condition {
                condition_type: ConditionType::Property,
                subject: change.subject.clone(),
                predicate: "changed_to".to_string(),
                object: Some(change.new_value.clone()),
            });
        }

        self.infer_consequences(&situation)
    }

    /// Explain why something happened
    pub fn explain(&self, observation: &Observation) -> Vec<Explanation> {
        let mut explanations = Vec::new();

        // Find rules that predict this observation
        let situation = &observation.situation;
        let applicable_rules = self.commonsense.find_applicable(situation);

        for rule in applicable_rules {
            explanations.push(Explanation {
                cause: rule.antecedent.clone(),
                effect: observation.clone(),
                rule: rule.clone(),
                confidence: Probability::from_f64(rule.certainty.to_probability()),
            });
        }

        explanations
    }
}

/// An inference from common sense
#[derive(Debug, Clone)]
pub struct Inference {
    pub rule_id: String,
    pub expectation: Expectation,
    pub confidence: Probability,
    pub reasoning: String,
}

/// An observation to explain
#[derive(Debug, Clone)]
pub struct Observation {
    pub situation: Situation,
    pub description: String,
}

/// An explanation for an observation
#[derive(Debug, Clone)]
pub struct Explanation {
    pub cause: Situation,
    pub effect: Observation,
    pub rule: CommonsenseRule,
    pub confidence: Probability,
}

/// A hypothetical scenario
#[derive(Debug, Clone)]
pub struct Hypothetical {
    pub original_situation: Situation,
    pub changes: Vec<HypotheticalChange>,
}

/// A change in a hypothetical
#[derive(Debug, Clone)]
pub struct HypotheticalChange {
    pub subject: String,
    pub property: String,
    pub old_value: String,
    pub new_value: String,
}

// ============================================================================
// Spatial Reasoning
// ============================================================================

/// Spatial reasoning engine
pub struct SpatialReasoner {
    /// Spatial relations
    spatial_kb: Vec<SpatialRule>,
}

impl SpatialReasoner {
    pub fn new() -> Self {
        let mut reasoner = SpatialReasoner {
            spatial_kb: Vec::new(),
        };
        reasoner.init_spatial_rules();
        reasoner
    }

    fn init_spatial_rules(&mut self) {
        // Objects cannot be in two places at once
        self.spatial_kb.push(SpatialRule {
            name: "unique_location".to_string(),
            rule: "An object cannot be in two different locations simultaneously".to_string(),
            certainty: Certainty::Always,
        });

        // Container relations
        self.spatial_kb.push(SpatialRule {
            name: "container_inside".to_string(),
            rule: "If A is inside B, and B is inside C, then A is inside C".to_string(),
            certainty: Certainty::Always,
        });

        // Spatial proximity
        self.spatial_kb.push(SpatialRule {
            name: "proximity_properties".to_string(),
            rule: "Objects close together may influence each other".to_string(),
            certainty: Certainty::Usually,
        });
    }

    pub fn reason(&self, situation: &Situation) -> Vec<Inference> {
        let mut inferences = Vec::new();

        // Check for spatial violations or implications
        for rule in &self.spatial_kb {
            // Simplified: check if rule applies
            if rule.certainty == Certainty::Always || rule.certainty == Certainty::Usually {
                inferences.push(Inference {
                    rule_id: rule.name.clone(),
                    expectation: Expectation::new(vec![Outcome {
                        description: rule.rule.clone(),
                        probability: rule.certainty.to_probability(),
                        conditions: vec![],
                    }]),
                    confidence: Probability::from_f64(rule.certainty.to_probability()),
                    reasoning: rule.rule.clone(),
                });
            }
        }

        inferences
    }
}

/// A spatial rule
#[derive(Debug, Clone)]
pub struct SpatialRule {
    pub name: String,
    pub rule: String,
    pub certainty: Certainty,
}

// ============================================================================
// Temporal Reasoning
// ============================================================================

/// Temporal reasoning engine
pub struct TemporalReasoner {
    /// Temporal rules
    temporal_kb: Vec<TemporalRule>,
}

impl TemporalReasoner {
    pub fn new() -> Self {
        let mut reasoner = TemporalReasoner {
            temporal_kb: Vec::new(),
        };
        reasoner.init_temporal_rules();
        reasoner
    }

    fn init_temporal_rules(&mut self) {
        // Causality requires temporal precedence
        self.temporal_kb.push(TemporalRule {
            name: "causality_precedence".to_string(),
            rule: "A cause must occur before or simultaneously with its effect".to_string(),
            certainty: Certainty::Always,
        });

        // Duration
        self.temporal_kb.push(TemporalRule {
            name: "event_duration".to_string(),
            rule: "Events have some duration in time".to_string(),
            certainty: Certainty::Always,
        });

        // Sequential actions
        self.temporal_kb.push(TemporalRule {
            name: "sequential_actions".to_string(),
            rule: "If A must happen before B, then B cannot happen until A completes".to_string(),
            certainty: Certainty::Always,
        });
    }

    pub fn reason(&self, situation: &Situation) -> Vec<Inference> {
        let mut inferences = Vec::new();

        for rule in &self.temporal_kb {
            inferences.push(Inference {
                rule_id: rule.name.clone(),
                expectation: Expectation::new(vec![Outcome {
                    description: rule.rule.clone(),
                    probability: rule.certainty.to_probability(),
                    conditions: vec![],
                }]),
                confidence: Probability::from_f64(rule.certainty.to_probability()),
                reasoning: rule.rule.clone(),
            });
        }

        inferences
    }
}

/// A temporal rule
#[derive(Debug, Clone)]
pub struct TemporalRule {
    pub name: String,
    pub rule: String,
    pub certainty: Certainty,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_situation_creation() {
        let situation = Situation::new()
            .with_entity(EntityDescription {
                entity_type: "Glass".to_string(),
                properties: vec![],
                location: None,
                state: None,
            });

        assert_eq!(situation.entities.len(), 1);
    }

    #[test]
    fn test_commonsense_kb_creation() {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));
        let commonsense = CommonsenseKB::new(kb);

        // Should have physics rules
        assert!(!commonsense.physics.is_empty());
        assert!(!commonsense.social.is_empty());
        assert!(!commonsense.mental.is_empty());
    }

    #[test]
    fn test_find_applicable_rules() {
        let kb = Arc::new(RwLock::new(KnowledgeGraph::new()));
        let commonsense = CommonsenseKB::new(kb);

        let situation = Situation::new().with_entity(EntityDescription {
            entity_type: "Fire".to_string(),
            properties: vec!["burning".to_string()],
            location: None,
            state: None,
        });

        let applicable = commonsense.find_applicable(&situation);
        assert!(!applicable.is_empty());
    }

    #[test]
    fn test_spatial_reasoner() {
        let reasoner = SpatialReasoner::new();
        let situation = Situation::new();

        let inferences = reasoner.reason(&situation);
        assert!(!inferences.is_empty());
    }

    #[test]
    fn test_temporal_reasoner() {
        let reasoner = TemporalReasoner::new();
        let situation = Situation::new();

        let inferences = reasoner.reason(&situation);
        assert!(!inferences.is_empty());
    }
}
