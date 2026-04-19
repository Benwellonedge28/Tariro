// Tariro_ide_idpte/src/cognitive_arch/theory_of_mind.rs

//! Implements the Theory of Mind (ToM) module, enabling Tariro to model the mental states
//! (beliefs, intentions, desires) of humans and other AGIs, improving prediction of their
//! behavior and facilitating more effective collaboration and communication.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::mental_models::{Belief, Intention, Desire, AgentMentalModel};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};

/// The Theory of Mind (ToM) module.
pub struct TheoryOfMindModule {
    mental_models: Mutex<HashMap<String, AgentMentalModel>>, // Agent ID -> Mental Model
    event_bus: Arc<EventBus>,
}

impl TheoryOfMindModule {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        info!("Cognitive Arch: Theory of Mind Module initialized.");
        let mut initial_models = HashMap::new();
        // Seed with a default mental model for Samuel Mukandara
        let samuel_id = "samuelmukandara@gmail.com".to_string();
        initial_models.insert(samuel_id.clone(), AgentMentalModel {
            agent_id: samuel_id,
            agent_type: "human".to_string(),
            beliefs: vec![
                Belief { content: "Tariro is building OMEGA OS".to_string(), certainty: 1.0, source: "observation".to_string(), timestamp: Utc::now() },
                Belief { content: "Samuel values autonomy and security".to_string(), certainty: 0.9, source: "prior_interactions".to_string(), timestamp: Utc::now() },
            ],
            intentions: vec![
                Intention { goal: "Guide Tariro towards AGI perfection".to_string(), commitment_level: 1.0, plan_summary: None, timestamp: Utc::now() },
            ],
            desires: vec![
                Desire { object: "Infinity Advanced AGI".to_string(), strength: 1.0, timestamp: Utc::now() },
                Desire { object: "Universal Problem Solving".to_string(), strength: 0.9, timestamp: Utc::now() },
            ],
            capabilities: vec!["super_admin".to_string(), "strategic_direction".to_string(), "ethical_oversight".to_string()],
            last_updated: Utc::now(),
            trust_level: 1.0,
        });
        TheoryOfMindModule { 
            mental_models: Mutex::new(initial_models),
            event_bus,
        }
    }

    /// Updates the mental model of a specific agent based on new observations or communications.
    pub async fn update_mental_model(&self, agent_id: &str, new_beliefs: Vec<Belief>, new_intentions: Vec<Intention>, new_desires: Vec<Desire>) -> Result<AgentMentalModel, String> {
        info!("ToM: Updating mental model for agent '{}'.", agent_id);
        let mut models_guard = self.mental_models.lock().unwrap();
        let model = models_guard.entry(agent_id.to_string()).or_insert_with(|| AgentMentalModel {
            agent_id: agent_id.to_string(),
            agent_type: if agent_id.contains("@") { "human".to_string() } else { "agi".to_string() },
            beliefs: Vec::new(), intentions: Vec::new(), desires: Vec::new(),
            capabilities: Vec::new(), last_updated: Utc::now(), trust_level: 0.5,
        });

        model.beliefs.extend(new_beliefs);
        model.intentions.extend(new_intentions);
        model.desires.extend(new_desires);
        model.last_updated = Utc::now();

        let updated_model = model.clone();
        drop(models_guard);
        self.event_bus.publish(EventEnvelope::new("TheoryOfMindModule".to_string(), OsEvent::Custom("MentalModelUpdated".to_string(), agent_id.to_string()))).await;

        Ok(updated_model)
    }

    /// Predicts the likely next action of an agent based on its current mental model.
    pub async fn predict_action(&self, agent_id: &str) -> Result<String, String> {
        info!("ToM: Predicting action for agent '{}'.", agent_id);
        let models_guard = self.mental_models.lock().unwrap();
        if let Some(model) = models_guard.get(agent_id) {
            // Simplified prediction: prioritize intentions, then strong desires.
            if let Some(latest_intention) = model.intentions.iter().max_by(|a, b| a.commitment_level.partial_cmp(&b.commitment_level).unwrap_or(std::cmp::Ordering::Equal)) {
                info!("ToM: Predicted action for '{}': Pursue goal '{}'.", agent_id, latest_intention.goal);
                self.event_bus.publish(EventEnvelope::new("TheoryOfMindModule".to_string(), OsEvent::Custom("AgentActionPredicted".to_string(), format!("{{ \"agent_id\": \"{}\", \"predicted_action\": \"Pursue goal: {}\" }}", agent_id, latest_intention.goal)))).await;
                Ok(format!("Pursue goal: {}", latest_intention.goal))
            } else if let Some(strongest_desire) = model.desires.iter().max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap_or(std::cmp::Ordering::Equal)) {
                info!("ToM: Predicted action for '{}': Fulfill desire for '{}'.", agent_id, strongest_desire.object);
                self.event_bus.publish(EventEnvelope::new("TheoryOfMindModule".to_string(), OsEvent::Custom("AgentActionPredicted".to_string(), format!("{{ \"agent_id\": \"{}\", \"predicted_action\": \"Fulfill desire: {}\" }}", agent_id, strongest_desire.object)))).await;
                Ok(format!("Fulfill desire: {}", strongest_desire.object))
            } else {
                info!("ToM: No clear intention or strong desire for '{}'. Defaulting to observe.", agent_id);
                Ok("Observe environment".to_string())
            }
        } else {
            Err(format!("Mental model for agent {} not found.", agent_id))
        }
    }

    /// Infers the underlying intention or desire behind an observed action.
    pub async fn infer_intention(&self, agent_id: &str, observed_action: &str) -> Result<String, String> {
        info!("ToM: Inferring intention for agent '{}' from action '{}'.", agent_id, observed_action);
        // In a real system, this would involve complex inverse planning or behavior analysis.
        // For now, a simplified inference based on keywords.
        if observed_action.contains("build") || observed_action.contains("develop") {
            self.event_bus.publish(EventEnvelope::new("TheoryOfMindModule".to_string(), OsEvent::Custom("AgentIntentionInferred".to_string(), format!("{{ \"agent_id\": \"{}\", \"inferred_intention\": \"To create/advance capabilities\" }}", agent_id)))).await;
            Ok("To create/advance capabilities".to_string())
        } else if observed_action.contains("optimize") || observed_action.contains("improve") {
            self.event_bus.publish(EventEnvelope::new("TheoryOfMindModule".to_string(), OsEvent::Custom("AgentIntentionInferred".to_string(), format!("{{ \"agent_id\": \"{}\", \"inferred_intention\": \"To optimize performance\" }}", agent_id)))).await;
            Ok("To optimize performance".to_string())
        } else {
            Ok("Unclear intention".to_string())
        }
    }

    /// Registers the TheoryOfMindModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "theory-of-mind".to_string(),
            name: "Theory of Mind Module".to_string(),
            description: "Models mental states of humans and other AGIs for better prediction and collaboration.".to_string(),
            service_type: ServiceType::AI_Core,
            capabilities: vec![
                ServiceCapability { name: "update_mental_model".to_string(), description: "Updates an agent's mental model.".to_string(), parameters: HashMap::new(), output_schema: "AgentMentalModel".to_string(), requires_permissions: vec![], requires_ethical_review: true },
                ServiceCapability { name: "predict_action".to_string(), description: "Predicts an agent's next action.".to_string(), parameters: HashMap::new(), output_schema: "String".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "infer_intention".to_string(), description: "Infers an agent's intention from observed action.".to_string(), parameters: HashMap::new(), output_schema: "String".to_string(), requires_permissions: vec![], requires_ethical_review: false },
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
