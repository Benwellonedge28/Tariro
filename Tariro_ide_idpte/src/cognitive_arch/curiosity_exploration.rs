// Tariro_ide_idpte/src/cognitive_arch/curiosity_exploration.rs

//! Implements the Curiosity-Driven Exploration Module, enabling Tariro to actively seek out
//! novel information and experiences to expand its knowledge and capabilities, especially
//! in environments where explicit rewards are sparse or delayed.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::exploration_models::{NoveltyScore, IntrinsicReward, ExplorationTarget};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};

/// The Curiosity-Driven Exploration Module.
pub struct CuriosityExplorationModule {
    known_states_summary: Mutex<HashMap<String, u64>>, // Simple count of visited state summaries
    active_exploration_targets: Mutex<HashMap<String, ExplorationTarget>>, // ID -> Target
    event_bus: Arc<EventBus>,
}

impl CuriosityExplorationModule {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        info!("Cognitive Arch: Curiosity-Driven Exploration Module initialized.");
        CuriosityExplorationModule {
            known_states_summary: Mutex::new(HashMap::new()),
            active_exploration_targets: Mutex::new(HashMap::new()),
            event_bus,
        }
    }

    /// Evaluates the novelty of a new observation or state.
    pub async fn evaluate_novelty(&self, observation_summary: &str) -> Result<NoveltyScore, String> {
        info!("CEM: Evaluating novelty for observation: '{}'.", observation_summary);
        let mut states_guard = self.known_states_summary.lock().unwrap();
        let visit_count = states_guard.entry(observation_summary.to_string()).or_insert(0);
        *visit_count += 1;

        // Simplified novelty calculation: inverse of visit count, capped.
        let raw_novelty = 1.0 / (*visit_count as f64).sqrt();
        let score = raw_novelty.min(1.0).max(0.0);

        let novelty_id = format!("novelty-{}", Uuid::new_v4());
        let novelty_score = NoveltyScore {
            id: novelty_id.clone(),
            observation_context_summary: observation_summary.to_string(),
            score,
            method: "state_visitation_count_inverse_sqrt".to_string(),
            timestamp: Utc::now(),
        };

        info!("CEM: Novelty score for '{}': {}.", observation_summary, score);
        self.event_bus.publish(EventEnvelope::new("CEMModule".to_string(), OsEvent::Custom("NoveltyEvaluated".to_string(), format!("{{ \"observation\": \"{}\", \"score\": {} }}", observation_summary, score)))).await;

        Ok(novelty_score)
    }

    /// Generates an intrinsic reward based on a novelty score.
    pub async fn generate_intrinsic_reward(&self, novelty_score: &NoveltyScore) -> Result<IntrinsicReward, String> {
        info!("CEM: Generating intrinsic reward for novelty score '{}' (Score: {}).", novelty_score.id, novelty_score.score);
        // Intrinsic reward is proportional to novelty score.
        let reward_value = novelty_score.score * 10.0; // Scale reward

        let intrinsic_reward_id = format!("reward-{}", Uuid::new_v4());
        let intrinsic_reward = IntrinsicReward {
            id: intrinsic_reward_id.clone(),
            associated_novelty_score_id: novelty_score.id.clone(),
            reward_value,
            explanation: format!("High novelty (score: {}) indicates new learning opportunity.", novelty_score.score),
            timestamp: Utc::now(),
        };

        info!("CEM: Intrinsic reward generated: {}.", reward_value);
        self.event_bus.publish(EventEnvelope::new("CEMModule".to_string(), OsEvent::Custom("IntrinsicRewardGenerated".to_string(), format!("{{ \"novelty_id\": \"{}\", \"reward\": {} }}", novelty_score.id, reward_value)))).await;

        Ok(intrinsic_reward)
    }

    /// Selects a target for further exploration based on current novelty and goals.
    pub async fn select_exploration_target(&self, current_environment_summary: &str) -> Result<ExplorationTarget, String> {
        info!("CEM: Selecting exploration target from current context: '{}'.", current_environment_summary);
        // In a real system, this would use active inference, goal-conditioned policies, 
        // or planning over a model of novelty/information gain.

        let target_id = format!("target-{}-explore", Uuid::new_v4());
        let target = ExplorationTarget {
            id: target_id.clone(),
            description: format!("Explore areas divergent from '{}'.", current_environment_summary),
            target_state_features: HashMap::new(), // More complex: desired features of new state
            predicted_novelty_gain: 0.7, // Assume some gain
            strategy: "random_walk_with_bias_to_unseen".to_string(),
            priority: 0.8, // High priority for exploration
            created_at: Utc::now(),
        };

        self.active_exploration_targets.lock().unwrap().insert(target_id.clone(), target.clone());
        info!("CEM: Selected exploration target: '{}'.", target.description);
        self.event_bus.publish(EventEnvelope::new("CEMModule".to_string(), OsEvent::Custom("ExplorationTargetSelected".to_string(), format!("{{ \"target_id\": \"{}\", \"description\": \"{}\" }}", target_id, target.description)))).await;
        Ok(target)
    }

    /// Registers the CuriosityExplorationModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "curiosity-exploration-module".to_string(),
            name: "Curiosity-Driven Exploration Module".to_string(),
            description: "Actively seeks out novel information and experiences to expand knowledge.".to_string(),
            service_type: ServiceType::AI_Core,
            capabilities: vec![
                ServiceCapability { name: "evaluate_novelty".to_string(), description: "Evaluates the novelty of an observation.".to_string(), parameters: HashMap::new(), output_schema: "NoveltyScore".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "generate_intrinsic_reward".to_string(), description: "Generates intrinsic reward based on novelty.".to_string(), parameters: HashMap::new(), output_schema: "IntrinsicReward".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "select_exploration_target".to_string(), description: "Selects a target for further exploration.".to_string(), parameters: HashMap::new(), output_schema: "ExplorationTarget".to_string(), requires_permissions: vec![], requires_ethical_review: false },
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
