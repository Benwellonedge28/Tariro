// Tariro_ide_idpte/src/cognitive_arch/common_sense_reasoning.rs

//! Implements the Common Sense Reasoning (CSR) Network, integrating a vast common-sense
//! knowledge base and reasoning capabilities to enable Tariro to understand and interact
//! with the world in a more human-like, intuitive manner.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::common_sense_models::{CommonSenseFact, CommonSenseRule, ContextualAffordance};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};

/// The Common Sense Reasoning (CSR) Network module.
pub struct CommonSenseReasoningNetwork {
    facts: Mutex<HashMap<String, CommonSenseFact>>, // ID -> Fact
    rules: Mutex<HashMap<String, CommonSenseRule>>, // ID -> Rule
    affordances: Mutex<HashMap<String, ContextualAffordance>>, // Object ID -> Affordance
    event_bus: Arc<EventBus>,
}

impl CommonSenseReasoningNetwork {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        info!("Cognitive Arch: Common Sense Reasoning Network initialized.");
        let mut initial_facts = HashMap::new();
        initial_facts.insert("fact_water_wet".to_string(), CommonSenseFact {
            id: "fact_water_wet".to_string(),
            statement: "Water is wet".to_string(),
            domain: "physics".to_string(),
            certainty: 1.0,
            source: "learned_from_experience".to_string(),
            timestamp: Utc::now(),
        });
        initial_facts.insert("fact_birds_fly".to_string(), CommonSenseFact {
            id: "fact_birds_fly".to_string(),
            statement: "Most birds can fly".to_string(),
            domain: "biology".to_string(),
            certainty: 0.9,
            source: "learned_from_experience".to_string(),
            timestamp: Utc::now(),
        });
        
        let mut initial_rules = HashMap::new();
        initial_rules.insert("rule_hot_object_burns".to_string(), CommonSenseRule {
            id: "rule_hot_object_burns".to_string(),
            rule_text: "If an object is very hot, touching it will cause a burn.".to_string(),
            antecedents: vec!["object_is_hot".to_string()],
            consequents: vec!["touching_causes_burn".to_string()],
            domain: "physics".to_string(),
            applicability_context: None,
            certainty: 0.95,
            timestamp: Utc::now(),
        });

        let mut initial_affordances = HashMap::new();
        initial_affordances.insert("chair_sit".to_string(), ContextualAffordance {
            object_id: "chair_generic".to_string(),
            object_name: "chair".to_string(),
            affordance: "can be sat on".to_string(),
            context: "when in a room".to_string(),
            certainty: 0.99,
            timestamp: Utc::now(),
        });

        CommonSenseReasoningNetwork {
            facts: Mutex::new(initial_facts),
            rules: Mutex::new(initial_rules),
            affordances: Mutex::new(initial_affordances),
            event_bus,
        }
    }

    /// Adds a new common-sense fact to the network.
    pub async fn add_fact(&self, fact: CommonSenseFact) -> Result<(), String> {
        info!("CSRN: Adding common-sense fact: '{}'.", fact.statement);
        self.facts.lock().unwrap().insert(fact.id.clone(), fact.clone());
        self.event_bus.publish(EventEnvelope::new("CSRNetwork".to_string(), OsEvent::Custom("CommonSenseFactAdded".to_string(), fact.id))).await;
        Ok(())
    }

    /// Infers implications from existing facts and rules.
    pub async fn infer_implications(&self, query_fact_id: &str) -> Result<Vec<CommonSenseFact>, String> {
        info!("CSRN: Inferring implications for query fact ID: '{}'.", query_fact_id);
        let facts_guard = self.facts.lock().unwrap();
        let rules_guard = self.rules.lock().unwrap();
        let mut inferred_facts = Vec::new();

        if let Some(query_fact) = facts_guard.get(query_fact_id) {
            for (_, rule) in rules_guard.iter() {
                // Simplified rule matching: if query fact's statement appears in rule antecedents
                if rule.antecedents.iter().any(|antecedent| query_fact.statement.contains(antecedent)) {
                    for consequent in &rule.consequents {
                        let new_fact = CommonSenseFact {
                            id: format!("inferred_{}", Uuid::new_v4()),
                            statement: consequent.clone(), // More complex would generate a new statement
                            domain: rule.domain.clone(),
                            certainty: query_fact.certainty * rule.certainty,
                            source: format!("inferred_from_rule_{}", rule.id),
                            timestamp: Utc::now(),
                        };
                        info!("CSRN: Inferred new fact: '{}'.", new_fact.statement);
                        inferred_facts.push(new_fact.clone());
                        self.event_bus.publish(EventEnvelope::new("CSRNetwork".to_string(), OsEvent::Custom("CommonSenseFactInferred".to_string(), new_fact.id))).await;
                    }
                }
            }
        }
        Ok(inferred_facts)
    }

    /// Checks the plausibility of a given statement based on common-sense knowledge.
    pub async fn check_plausibility(&self, statement: &str) -> Result<f64, String> {
        info!("CSRN: Checking plausibility of statement: '{}'.", statement);
        let facts_guard = self.facts.lock().unwrap();
        let rules_guard = self.rules.lock().unwrap();

        // Simplified plausibility check: if the statement directly contradicts a known high-certainty fact
        if let Some(fact) = facts_guard.values().find(|f| f.statement == statement && f.certainty > 0.9) {
            info!("CSRN: Statement '{}' is highly plausible (known fact).", statement);
            self.event_bus.publish(EventEnvelope::new("CSRNetwork".to_string(), OsEvent::Custom("PlausibilityCheck".to_string(), format!("{{ \"statement\": \"{}\", \"plausibility\": 0.99 }}", statement)))).await;
            Ok(0.99)
        } else if statement.contains("cold is hot") || statement.contains("birds can't fly") { // Direct contradiction
            info!("CSRN: Statement '{}' is implausible (contradicts common sense).", statement);
            self.event_bus.publish(EventEnvelope::new("CSRNetwork".to_string(), OsEvent::Custom("PlausibilityCheck".to_string(), format!("{{ \"statement\": \"{}\", \"plausibility\": 0.01 }}", statement)))).await;
            Ok(0.01)
        } else {
            info!("CSRN: Plausibility of statement '{}' is moderate (no direct evidence).", statement);
            self.event_bus.publish(EventEnvelope::new("CSRNetwork".to_string(), OsEvent::Custom("PlausibilityCheck".to_string(), format!("{{ \"statement\": \"{}\", \"plausibility\": 0.5 }}", statement)))).await;
            Ok(0.5) // Default moderate plausibility
        }
    }

    /// Registers the CommonSenseReasoningNetwork itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "common-sense-reasoning".to_string(),
            name: "Common Sense Reasoning Network".to_string(),
            description: "Integrates common-sense knowledge for human-like understanding and plausible inference.".to_string(),
            service_type: ServiceType::AI_Core,
            capabilities: vec![
                ServiceCapability { name: "add_fact".to_string(), description: "Adds a common-sense fact to the network.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec![], requires_ethical_review: true },
                ServiceCapability { name: "infer_implications".to_string(), description: "Infers implications from facts and rules.".to_string(), parameters: HashMap::new(), output_schema: "Vec<CommonSenseFact>".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "check_plausibility".to_string(), description: "Checks the plausibility of a given statement.".to_string(), parameters: HashMap::new(), output_schema: "f64".to_string(), requires_permissions: vec![], requires_ethical_review: false },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string()],
            managed_by_agi_module: None,
        }
    }
}
