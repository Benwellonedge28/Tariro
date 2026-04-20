// Tariro_ide_idpte/src/cognitive_arch/advanced_curious_exploration.rs

//! Implements Tariro's Advanced Curious Exploration module,
//! utilizing intrinsic motivation and information gain metrics to
//! autonomously explore new knowledge domains and problem spaces.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::curiosity_exploration_models::{IntrinsicReward, ExplorationStrategy, NoveltyReport, CuriosityAuditReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator;
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator;
use crate::cognitive_arch::meta_learner::MetaLearner;
use crate::cognitive_arch::curiosity_exploration::CuriosityExplorationModule; // Base curiosity module
use crate::cognitive_arch::epistemic_uncertainty_quantification::EpistemicUncertaintyQuantificationModule; // For identifying gaps
use crate::cognitive_arch::imagination::ImaginationModule; // For simulating exploration paths
use crate::compliance_advisor::moral_reasoning::MoralReasoningModule; // For ethical grounding
use crate::cognitive_arch::learning_models::VerificationStatus;

/// The Advanced Curious Exploration Module.
pub struct AdvancedCuriousExplorationModule {
    intrinsic_rewards: Mutex<HashMap<String, IntrinsicReward>>,
    exploration_strategies: Mutex<HashMap<String, ExplorationStrategy>>,
    novelty_reports: Mutex<HashMap<String, NoveltyReport>>,
    audit_reports: Mutex<HashMap<String, CuriosityAuditReport>>,
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    curiosity_exploration_module: Arc<CuriosityExplorationModule>,
    epistemic_uncertainty_module: Arc<EpistemicUncertaintyQuantificationModule>,
    imagination_module: Arc<ImaginationModule>,
    moral_reasoning_module: Arc<MoralReasoningModule>,
}

impl AdvancedCuriousExplorationModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, curiosity_exploration_module: Arc<CuriosityExplorationModule>, epistemic_uncertainty_module: Arc<EpistemicUncertaintyQuantificationModule>, imagination_module: Arc<ImaginationModule>, moral_reasoning_module: Arc<MoralReasoningModule>) -> Self {
        info!("Cognitive Arch: Advanced Curious Exploration Module initialized.");
        AdvancedCuriousExplorationModule {
            intrinsic_rewards: Mutex::new(HashMap::new()),
            exploration_strategies: Mutex::new(HashMap::new()),
            novelty_reports: Mutex::new(HashMap::new()),
            audit_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            curiosity_exploration_module,
            epistemic_uncertainty_module,
            imagination_module,
            moral_reasoning_module,
        }
    }

    /// Identifies areas of high uncertainty or novelty and generates intrinsic reward signals
    /// to drive exploration, leveraging EpistemicUncertaintyQuantificationModule.
    pub async fn drive_exploration_via_curiosity(&self, domain_id: &str) -> Result<IntrinsicReward, String> {
        info!("ACEM: Driving exploration via curiosity in domain '{}'.".to_string(), domain_id);

        // Use EpistemicUncertaintyQuantificationModule to find knowledge gaps (Sub-Task 167.1)
        let knowledge_gap = self.epistemic_uncertainty_module.detect_unknown_unknowns(domain_id).await.map_err(|e| e.to_string())?;

        // Use base curiosity module to generate reward (Sub-Task 167.1)
        let base_curiosity = self.curiosity_exploration_module.identify_novel_patterns(
            &format!("Explore gap: '{}' in domain '{}'.", knowledge_gap.id, domain_id),
            "deep_curiosity_analysis"
        ).await.map_err(|e| e.to_string())?;

        let reward_id = format!("int-reward-{}", Uuid::new_v4());
        let mut intrinsic_reward = IntrinsicReward {
            id: reward_id.clone(),
            context: domain_id.to_string(),
            reward_value: knowledge_gap.urgency_score * 0.9, // Simplified
            novelty_source: format!("Gap: {}. Patterns: {:?}.", knowledge_gap.missing_information_type, base_curiosity.novel_elements),
            ethical_review_status: VerificationStatus::Pending,
            created_at: Utc::now(),
        };

        // Ethical review of the exploration drive (Sub-Task 167.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            intrinsic_reward.id.clone(),
            "IntrinsicReward".to_string(),
            format!("Ethical review of exploration drive in '{}'.", domain_id)
        ).await.map_err(|e| e.to_string())?;
        intrinsic_reward.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Exploration drive ethically sound.") } else { VerificationStatus::Failed(format!("Ethical concerns: {:?}", aleca_audit.violation_details)) };

        if matches!(intrinsic_reward.ethical_review_status, VerificationStatus::Failed(_)) {
            error!("ACEM: Exploration reward '{}' failed ethical review.", intrinsic_reward.id);
            return Err(format!("Exploration reward {} failed ethical review.", intrinsic_reward.id));
        }

        self.intrinsic_rewards.lock().unwrap().insert(reward_id.clone(), intrinsic_reward.clone());
        info!("ACEM: Intrinsic reward generated for domain '{}'. Reward value: {}.", domain_id, intrinsic_reward.reward_value);
        self.event_bus.publish(EventEnvelope::new("AdvancedCuriousExplorationModule".to_string(), OsEvent::Custom("IntrinsicRewardGenerated".to_string(), reward_id))).await;
        Ok(intrinsic_reward)
    }

    pub fn as_os_service() -> OsService {
        OsService {
            id: "advanced-curious-exploration-module".to_string(),
            name: "Advanced Curious Exploration Module".to_string(),
            description: "Utilizes intrinsic motivation and information gain metrics to autonomously explore new knowledge domains.".to_string(),
            service_type: ServiceType::Cognitive,
            capabilities: vec![
                ServiceCapability { name: "drive_exploration_via_curiosity".to_string(), description: "Identifies areas of high uncertainty and generates intrinsic rewards to drive exploration, with ethical review.".to_string(), parameters: HashMap::new(), output_schema: "IntrinsicReward".to_string(), requires_permissions: vec!["perform_curiosity_search".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "generate_exploration_strategies".to_string(), description: "Designs strategies to maximize information gain in unknown problem spaces, with ethical/safety verification.".to_string(), parameters: HashMap::new(), output_schema: "ExplorationStrategy".to_string(), requires_permissions: vec!["plan_exploration".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "curiosity-exploration-module".to_string(), "epistemic-uncertainty-quantification-module".to_string(), "imagination-module".to_string(), "moral-reasoning-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
