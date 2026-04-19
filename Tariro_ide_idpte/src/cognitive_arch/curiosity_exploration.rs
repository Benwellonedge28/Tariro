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

use crate::cognitive_arch::exploration_models::{NoveltyScore, IntrinsicReward, ExplorationTarget, NoveltyAssessmentVerificationStatus, ExplorationStrategy};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical review
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For integrity checks
use crate::cognitive_arch::meta_learner::MetaLearner; // For selecting/adapting exploration algorithms
use crate::cognitive_arch::lifelong_learning::LifelongLearningModule; // For knowledge gaps
use crate::cognitive_arch::concept_formation::ConceptFormationModule; // For novel concepts
use crate::cognitive_arch::causal_reasoning::CausalReasoningEngine; // For unexplained phenomena
use crate::cognitive_arch::learning_models::VerificationStatus; // Alias for clarity

/// The Curiosity-Driven Exploration Module.
pub struct CuriosityExplorationModule {
    known_states_summary: Mutex<HashMap<String, u64>>, // Simple count of visited state summaries
    active_exploration_targets: Mutex<HashMap<String, ExplorationTarget>>, // ID -> Target
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    lifelong_learning_module: Arc<LifelongLearningModule>,
    concept_formation_module: Arc<ConceptFormationModule>,
    causal_reasoning_engine: Arc<CausalReasoningEngine>,
}

impl CuriosityExplorationModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, lifelong_learning_module: Arc<LifelongLearningModule>, concept_formation_module: Arc<ConceptFormationModule>, causal_reasoning_engine: Arc<CausalReasoningEngine>) -> Self {
        info!("Cognitive Arch: Curiosity-Driven Exploration Module initialized.");
        CuriosityExplorationModule {
            known_states_summary: Mutex::new(HashMap::new()),
            active_exploration_targets: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            lifelong_learning_module,
            concept_formation_module,
            causal_reasoning_engine,
        }
    }

    /// Evaluates the novelty of a new observation or state.
    /// Enhanced to integrate feedback from other cognitive modules for perpetual discovery.
    pub async fn evaluate_novelty(&self, observation_summary: &str) -> Result<NoveltyScore, String> {
        info!("CEM: Evaluating novelty for observation: '{}'.", observation_summary);
        let mut states_guard = self.known_states_summary.lock().unwrap();
        let visit_count = states_guard.entry(observation_summary.to_string()).or_insert(0);
        *visit_count += 1;

        let mut score = 1.0 / (*visit_count as f64).sqrt();
        score = score.min(1.0).max(0.0);

        // Integrate feedback from other modules (simulated)
        // - Knowledge gaps from LifelongLearningModule could boost novelty of relevant observations
        // - Novel concepts from ConceptFormationModule indicate high novelty
        // - Unexplained phenomena from CausalReasoningEngine could boost novelty
        // For simplicity, a mock boost if summary contains keywords
        if observation_summary.contains("knowledge_gap") || observation_summary.contains("unexplained_phenomenon") {
            score = (score + 0.3).min(1.0);
        }

        let novelty_id = format!("novelty-{}", Uuid::new_v4());
        let mut novelty_score = NoveltyScore {
            id: novelty_id.clone(),
            observation_context_summary: observation_summary.to_string(),
            score,
            method: "state_visitation_count_inverse_sqrt".to_string(),
            timestamp: Utc::now(),
            verification_status: NoveltyAssessmentVerificationStatus {
                adversarial_robustness_status: VerificationStatus::Pending,
                accuracy_status: VerificationStatus::Pending,
                overall_status: VerificationStatus::Pending,
                last_verified: Utc::now(),
                verification_log_id: format!("log-novelty-verif-{}", Uuid::new_v4()),
            },
        };

        // Secure Novelty Assessment (Sub-Task 71.2)
        self.verify_novelty_assessment(&mut novelty_score).await?;

        info!("CEM: Novelty score for '{}': {} (Verified: {:?}).", observation_summary, score, novelty_score.verification_status.overall_status);
        self.event_bus.publish(EventEnvelope::new("CEMModule".to_string(), OsEvent::Custom("NoveltyEvaluated".to_string(), format!("{{ \"observation\": \"{}\", \"score\": {} }}", observation_summary, score)))).await;

        Ok(novelty_score)
    }

    /// Generates an intrinsic reward based on a novelty score.
    pub async fn generate_intrinsic_reward(&self, novelty_score: &NoveltyScore) -> Result<IntrinsicReward, String> {
        info!("CEM: Generating intrinsic reward for novelty score '{}' (Score: {}).", novelty_score.id, novelty_score.score);
        
        if matches!(novelty_score.verification_status.overall_status, VerificationStatus::Failed(_)) {
            return Err(format!("Cannot generate intrinsic reward for unverified novelty score {}.", novelty_score.id));
        }

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
    /// Enhanced with adaptive exploration strategies and ethical review.
    pub async fn select_exploration_target(&self, current_environment_summary: &str) -> Result<ExplorationTarget, String> {
        info!("CEM: Selecting exploration target from current context: '{}'.", current_environment_summary);
        
        // Adaptive Exploration Strategies (Sub-Task 71.3)
        let exploration_algorithm = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("exploration_problem_{}", Uuid::new_v4()),
            name: "Optimal Exploration Strategy Selection".to_string(),
            domain: "cognitive_arch".to_string(),
            problem_type: "multi_objective_optimization".to_string(), // Optimizing novelty gain vs. resource cost
            complexity_score: 0.8,
            data_characteristics: HashMap::from([("environment_dynamism".to_string(), "high".to_string())]),
            required_accuracy: 0.9,
            created_at: Utc::now(),
        }).await.map_err(|e| e.to_string())?;

        info!("CEM: Using exploration strategy algorithm: {}.", exploration_algorithm.name);

        let target_id = format!("target-{}-explore", Uuid::new_v4());
        let mut target = ExplorationTarget {
            id: target_id.clone(),
            description: format!("Explore areas divergent from '{}'. Current strategy: {}.
            Triggered by: {}.", current_environment_summary, exploration_algorithm.name, self.lifelong_learning_module.refine_knowledge_base().await.map_or("general_curiosity".to_string(), |_| "knowledge_gap_identified".to_string())),
            target_state_features: HashMap::new(), 
            predicted_novelty_gain: 0.7, 
            strategy: ExplorationStrategy {
                name: exploration_algorithm.name.clone(),
                algorithm_id: exploration_algorithm.id.clone(),
                parameters: HashMap::new(),
                safety_constraints: vec!["respect_ethical_boundaries".to_string(), "minimize_resource_impact".to_string()],
                ethical_alignment_score: 0.0, // Will be filled by ALEC-A
            },
            priority: 0.8, 
            created_at: Utc::now(),
            ethical_review_status: VerificationStatus::Pending,
        };

        // Ethical Review of Exploration Target (Sub-Task 71.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            target.id.clone(),
            "ExplorationTarget".to_string(),
            format!("Review exploration target '{}' for potential ethical risks or harmful outcomes.", target.description)
        ).await.map_err(|e| e.to_string())?;

        target.ethical_review_status = if aleca_audit.is_compliant {
            VerificationStatus::Passed("Exploration target ethically compliant.".to_string())
        } else {
            VerificationStatus::Failed(format!("Ethical concerns: {:?}", aleca_audit.violation_details))
        };
        target.strategy.ethical_alignment_score = if aleca_audit.is_compliant { 1.0 } else { 0.0 };

        if matches!(target.ethical_review_status, VerificationStatus::Failed(_)) {
            error!("CEM: Exploration target '{}' failed ethical review. Aborting selection.", target.id);
            return Err(format!("Exploration target {} failed ethical review.", target.id));
        }

        self.active_exploration_targets.lock().unwrap().insert(target_id.clone(), target.clone());
        info!("CEM: Selected exploration target: '{}' (Ethically reviewed: {:?}).", target.description, target.ethical_review_status);
        self.event_bus.publish(EventEnvelope::new("CEMModule".to_string(), OsEvent::Custom("ExplorationTargetSelected".to_string(), format!("{{ \"target_id\": \"{}\", \"description\": \"{}\" }}", target_id, target.description)))).await;
        Ok(target)
    }

    /// Verifies the novelty assessment against adversarial manipulation and checks integrity.
    async fn verify_novelty_assessment(&self, novelty_score: &mut NoveltyScore) -> Result<(), String> {
        info!("CEM: Verifying novelty assessment for score '{}'.", novelty_score.id);
        // Simulate QR-CTFO audit for data integrity and adversarial robustness
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            novelty_score.id.clone(),
            "NoveltyScoreAssessment".to_string(),
            format!("Audit novelty score '{}' for adversarial robustness and data integrity.", novelty_score.id)
        ).await.map_err(|e| e.to_string())?;

        novelty_score.verification_status.adversarial_robustness_status = if qrctfo_audit.is_secure {
            VerificationStatus::Passed("Robust against adversarial manipulation.".to_string())
        } else {
            VerificationStatus::Failed(format!("Adversarial robustness issues: {:?}", qrctfo_audit.vulnerabilities))
        };

        // Accuracy status (simulated, would involve cross-validation or other checks)
        novelty_score.verification_status.accuracy_status = VerificationStatus::Passed("Accuracy estimate within bounds.".to_string());

        novelty_score.verification_status.overall_status = match (&novelty_score.verification_status.adversarial_robustness_status, &novelty_score.verification_status.accuracy_status) {
            (VerificationStatus::Passed(_), VerificationStatus::Passed(_)) => VerificationStatus::Passed("Novelty assessment verified.".to_string()),
            _ => VerificationStatus::FlaggedForReview("Novelty assessment flagged for review.".to_string()),
        };
        novelty_score.verification_status.last_verified = Utc::now();

        self.event_bus.publish(EventEnvelope::new("CEMModule".to_string(), OsEvent::Custom("NoveltyAssessmentVerified".to_string(), format!("{{ \"novelty_id\": \"{}\", \"overall_status\": \"{:?}\" }}", novelty_score.id, novelty_score.verification_status.overall_status)))).await;
        Ok(())
    }

    /// Registers the CuriosityExplorationModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "curiosity-exploration-module".to_string(),
            name: "Curiosity-Driven Exploration Module".to_string(),
            description: "Actively seeks out novel information and experiences to expand knowledge.".to_string(),
            service_type: ServiceType::AI_Core,
            capabilities: vec![
                ServiceCapability { name: "evaluate_novelty".to_string(), description: "Evaluates the novelty of an observation with security verification.".to_string(), parameters: HashMap::new(), output_schema: "NoveltyScore".to_string(), requires_permissions: vec!["read_sensor_data".to_string()], requires_ethical_review: false },
                ServiceCapability { name: "generate_intrinsic_reward".to_string(), description: "Generates intrinsic reward based on verified novelty.".to_string(), parameters: HashMap::new(), output_schema: "IntrinsicReward".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "select_exploration_target".to_string(), description: "Selects an ethically reviewed exploration target using adaptive strategies.".to_string(), parameters: HashMap::new(), output_schema: "ExplorationTarget".to_string(), requires_permissions: vec!["plan_actions".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "lifelong-learning-module".to_string(), "concept-formation-module".to_string(), "causal-reasoning-engine".to_string()],
            managed_by_agi_module: None,
        }
    }
}
