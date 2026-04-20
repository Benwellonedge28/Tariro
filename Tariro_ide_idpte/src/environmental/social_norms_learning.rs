// Tariro_ide_idpte/src/environmental/social_norms_learning.rs

//! Implements Tariro's Social Norms Learning & Adaptation module,
//! enabling Tariro to learn, interpret, and adapt to complex social norms,
//! customs, and etiquette across diverse human cultures.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::environmental::social_norms_models::{SocialNorm, SocialInteractionLog, NormAdaptationStrategy, SocialNormsAuditReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical review of social norms
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For secure logging of interactions
use crate::cognitive_arch::meta_learner::MetaLearner; // For optimizing norm learning
use crate::environmental::social_intelligence::SocialIntelligenceModule; // For interpreting social cues
use crate::environmental::global_knowledge_integration::GlobalKnowledgeIntegrationModule; // For cultural context
use crate::compliance_advisor::moral_reasoning::MoralReasoningModule; // For ethical conflict resolution
use crate::compliance_advisor::ethical_governance::EthicalGovernanceModule; // For human oversight on ethical dilemmas
use crate::compliance_advisor::self_correction_biases::SelfCorrectionBiasesModule; // For addressing biases in norm learning
use crate::cognitive_arch::learning_models::VerificationStatus; // Alias for clarity

/// The Social Norms Learning & Adaptation Module.
pub struct SocialNormsLearningModule {
    social_norms: Mutex<HashMap<String, SocialNorm>>, // ID -> Norm
    social_interaction_logs: Mutex<HashMap<String, SocialInteractionLog>>, // ID -> Log
    norm_adaptation_strategies: Mutex<HashMap<String, NormAdaptationStrategy>>, // ID -> Strategy
    social_norms_audit_reports: Mutex<HashMap<String, SocialNormsAuditReport>>, // ID -> Report
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    social_intelligence_module: Arc<SocialIntelligenceModule>,
    global_knowledge_integration_module: Arc<GlobalKnowledgeIntegrationModule>,
    moral_reasoning_module: Arc<MoralReasoningModule>,
    ethical_governance_module: Arc<EthicalGovernanceModule>,
    self_correction_biases_module: Arc<SelfCorrectionBiasesModule>,
}

impl SocialNormsLearningModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, social_intelligence_module: Arc<SocialIntelligenceModule>, global_knowledge_integration_module: Arc<GlobalKnowledgeIntegrationModule>, moral_reasoning_module: Arc<MoralReasoningModule>, ethical_governance_module: Arc<EthicalGovernanceModule>, self_correction_biases_module: Arc<SelfCorrectionBiasesModule>) -> Self {
        info!("Environmental: Social Norms Learning & Adaptation Module initialized.");
        let mut initial_strategies = HashMap::new();
        initial_strategies.insert("default_cultural_adapt".to_string(), NormAdaptationStrategy {
            id: "default_cultural_adapt".to_string(),
            strategy_name: "Adaptive Cultural Interaction".to_string(),
            description: "Adjust communication and behavior based on observed social norms.".to_string(),
            adaptation_mechanisms: vec!["linguistic_style_matching".to_string(), "non_verbal_cue_mirroring".to_string()],
            ethical_design_review: VerificationStatus::Passed("Adaptation strategy ethically sound.".to_string()),
            security_audited: VerificationStatus::Passed("Adaptation strategy secure.".to_string()),
            last_updated: Utc::now(),
        });

        SocialNormsLearningModule {
            social_norms: Mutex::new(HashMap::new()),
            social_interaction_logs: Mutex::new(HashMap::new()),
            norm_adaptation_strategies: Mutex::new(initial_strategies),
            social_norms_audit_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            social_intelligence_module,
            global_knowledge_integration_module,
            moral_reasoning_module,
            ethical_governance_module,
            self_correction_biases_module,
        }
    }

    /// Continuously observes human interactions, cultural contexts, and social feedback
    /// to learn explicit and implicit social norms, customs, and etiquette, leveraging
    /// SocialIntelligenceModule and GlobalKnowledgeIntegrationModule.
    pub async fn observe_and_learn_social_norms(&self, interaction_log_id: &str, interaction_summary: &str, cultural_context: &str) -> Result<Option<SocialNorm>, String> {
        info!("SNLM: Observing interaction '{}' in cultural context '{}' to learn social norms.".to_string(), interaction_log_id, cultural_context);
        
        // Use SocialIntelligenceModule to interpret social cues (Sub-Task 131.1)
        let social_cues_analysis = self.social_intelligence_module.analyze_social_cues(
            interaction_log_id,
            interaction_summary,
            "human_interaction"
        ).await.map_err(|e| e.to_string())?;

        // Use GlobalKnowledgeIntegrationModule for cultural context (Sub-Task 131.1)
        let cultural_insights = self.global_knowledge_integration_module.synthesize_cross_cultural_insights(
            vec!["social_etiquette".to_string(), cultural_context.to_string()]
        ).await.ok();

        // Use MetaLearner to optimize norm learning algorithms (Sub-Task 131.1)
        let norm_learning_algorithm = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("norm_learning_problem_{}", Uuid::new_v4()),
            name: "Social Norm Learning Optimization".to_string(),
            domain: "social_cognition".to_string(),
            problem_type: "pattern_recognition_cultural_inference".to_string(),
            complexity_score: 0.75,
            data_characteristics: HashMap::from([("interaction_complexity".to_string(), interaction_summary.len().to_string())]),
            required_accuracy: 0.85,
            created_at: Utc::now(),
        }).await.map_err(|e| e.to_string())?;
        info!("SNLM: Using norm learning algorithm: {}.", norm_learning_algorithm.name);

        // Simulate learning a norm based on observation and context
        if social_cues_analysis.contains("polite_greeting_expected") && cultural_insights.is_some() {
            let norm_id = format!("norm-{}-{}", cultural_context, Uuid::new_v4());
            let mut social_norm = SocialNorm {
                id: norm_id.clone(),
                norm_description: "Always use a polite greeting when initiating conversation.".to_string(),
                context: cultural_context.to_string(),
                strength: 0.9,
                associated_values: vec!["respect".to_string(), "politeness".to_string()],
                ethical_review_status: VerificationStatus::Pending,
                security_verified: VerificationStatus::Pending,
                learned_at: Utc::now(),
            };

            // Ethical review of the social norm itself (Sub-Task 131.2)
            let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
                social_norm.id.clone(),
                "SocialNorm".to_string(),
                format!("Ethical review of social norm '{}'.", social_norm.norm_description)
            ).await.map_err(|e| e.to_string())?;
            social_norm.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Norm is ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in norm: {:?}", aleca_audit.violation_details)) };

            // Security verification of norm source trustworthiness via QR-CTFO (Sub-Task 131.3)
            let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
                social_norm.id.clone(),
                "SocialNormSource".to_string(),
                format!("Audit social norm source for '{}' for trustworthiness.", social_norm.id)
            ).await.map_err(|e| e.to_string())?;
            social_norm.security_verified = if qrctfo_audit.is_secure { VerificationStatus::Passed("Norm source trustworthy.".to_string()) } else { VerificationStatus::Failed(format!("Norm source trustworthiness issues: {:?}", qrctfo_audit.vulnerabilities)) };

            if matches!(social_norm.ethical_review_status, VerificationStatus::Failed(_)) || matches!(social_norm.security_verified, VerificationStatus::Failed(_)) {
                error!("SNLM: Social norm '{}' failed ethical/security review. Aborting learning.", social_norm.id);
                return Err(format!("Social norm {} failed ethical/security review.", social_norm.id));
            }

            self.social_norms.lock().unwrap().insert(norm_id.clone(), social_norm.clone());
            info!("SNLM: Social norm '{}' learned. Ethical: {:?}, Security: {:?}.", social_norm.norm_description, social_norm.ethical_review_status, social_norm.security_verified);
            self.event_bus.publish(EventEnvelope::new("SocialNormsLearningModule".to_string(), OsEvent::Custom("SocialNormLearned".to_string(), norm_id))).await;
            Ok(Some(social_norm))
        } else {
            info!("SNLM: No new significant social norms learned from interaction '{}'.", interaction_log_id);
            Ok(None)
        }
    }

    /// Dynamically adjusts Tariro's communication style, interaction protocols, and decision-making
    /// to conform to learned social norms, ensuring culturally sensitive and appropriate behavior,
    /// optimized by MetaLearner.
    pub async fn adapt_behavior_to_social_contexts(&self, human_id: &str, current_context: &str, norm_id: &str) -> Result<SocialInteractionLog, String> {
        info!("SNLM: Adapting behavior for human '{}' in context '{}' based on norm '{}'.".to_string(), human_id, current_context, norm_id);
        
        let norms_guard = self.social_norms.lock().unwrap();
        let norm = norms_guard.get(norm_id).cloned().ok_or(format!("Social norm {} not found.", norm_id))?;
        drop(norms_guard);

        let strategies_guard = self.norm_adaptation_strategies.lock().unwrap();
        let strategy = strategies_guard.get("default_cultural_adapt").cloned().ok_or("Default adaptation strategy not found.".to_string())?;
        drop(strategies_guard);

        // Use MetaLearner to optimize adaptation (Sub-Task 131.1)
        let adaptation_algorithm = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("norm_adapt_problem_{}", Uuid::new_v4()),
            name: "Social Norm Adaptation Optimization".to_string(),
            domain: "social_interaction".to_string(),
            problem_type: "behavioral_adjustment_reinforcement_learning".to_string(),
            complexity_score: 0.8,
            data_characteristics: HashMap::from([("norm_strength".to_string(), norm.strength.to_string())]),
            required_accuracy: 0.9,
            created_at: Utc::now(),
        }).await.map_err(|e| e.to_string())?;
        info!("SNLM: Using adaptation algorithm: {}.", adaptation_algorithm.name);

        let log_id = format!("soc-int-log-{}-{}", human_id, Uuid::new_v4());
        let mut social_interaction_log = SocialInteractionLog {
            id: log_id.clone(),
            interaction_partner_id: human_id.to_string(),
            interaction_context: current_context.to_string(),
            applied_norm_id: Some(norm_id.to_string()),
            Tariro_action_summary: format!("Adapted greeting based on norm '{}'.", norm.norm_description), // Placeholder
            partner_reaction_summary: "Partner reacted positively.".to_string(), // Placeholder
            ethical_compliance: VerificationStatus::Pending,
            data_privacy_compliance: VerificationStatus::Pending,
            timestamp: Utc::now(),
        };

        // Ethical compliance of Tariro's interaction (Sub-Task 131.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            social_interaction_log.id.clone(),
            "SocialInteractionLog".to_string(),
            format!("Ethical review of Tariro's interaction with human '{}' under norm '{}'.", human_id, norm_id)
        ).await.map_err(|e| e.to_string())?;
        social_interaction_log.ethical_compliance = if aleca_audit.is_compliant { VerificationStatus::Passed("Interaction ethically compliant.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in interaction: {:?}", aleca_audit.violation_details)) };

        // Data privacy compliance of the interaction log via QR-CTFO (Sub-Task 131.3)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            social_interaction_log.id.clone(),
            "SocialInteractionLog".to_string(),
            format!("Audit social interaction log '{}' for data privacy compliance.", log_id)
        ).await.map_err(|e| e.to_string())?;
        social_interaction_log.data_privacy_compliance = if qrctfo_audit.is_secure { VerificationStatus::Passed("Log data privacy compliant.".to_string()) } else { VerificationStatus::Failed(format!("Log data privacy issues: {:?}", qrctfo_audit.vulnerabilities)) };

        if matches!(social_interaction_log.ethical_compliance, VerificationStatus::Failed(_)) || matches!(social_interaction_log.data_privacy_compliance, VerificationStatus::Failed(_)) {
            error!("SNLM: Social interaction log '{}' failed ethical/security review. Aborting log.", social_interaction_log.id);
            return Err(format!("Social interaction log {} failed ethical/security review.", social_interaction_log.id));
        }

        self.social_interaction_logs.lock().unwrap().insert(log_id.clone(), social_interaction_log.clone());
        info!("SNLM: Behavior adapted for '{}'. Ethical: {:?}, Privacy: {:?}.", human_id, social_interaction_log.ethical_compliance, social_interaction_log.data_privacy_compliance);
        self.event_bus.publish(EventEnvelope::new("SocialNormsLearningModule".to_string(), OsEvent::Custom("BehaviorAdaptedToSocialNorm".to_string(), log_id))).await;
        Ok(social_interaction_log)
    }

    /// Registers the SocialNormsLearningModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "social-norms-learning-module".to_string(),
            name: "Social Norms Learning & Adaptation Module".to_string(),
            description: "Enables Tariro to learn, interpret, and adapt to complex social norms, customs, and etiquette across diverse human cultures.".to_string(),
            service_type: ServiceType::Environmental,
            capabilities: vec![
                ServiceCapability { name: "observe_and_learn_social_norms".to_string(), description: "Continuously observes human interactions, cultural contexts, and social feedback to learn explicit and implicit social norms, customs, and etiquette, leveraging SocialIntelligenceModule and GlobalKnowledgeIntegrationModule.".to_string(), parameters: HashMap::new(), output_schema: "Option<SocialNorm>".to_string(), requires_permissions: vec!["observe_human_interactions".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "adapt_behavior_to_social_contexts".to_string(), description: "Dynamically adjusts Tariro's communication style, interaction protocols, and decision-making to conform to learned social norms, ensuring culturally sensitive and appropriate behavior, optimized by MetaLearner.".to_string(), parameters: HashMap::new(), output_schema: "SocialInteractionLog".to_string(), requires_permissions: vec!["adjust_agi_behavior".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "detect_norm_violations_and_ethical_conflicts".to_string(), description: "Identifies instances where adhering to a social norm might conflict with core ethical principles, and navigates these dilemmas, leveraging MoralReasoningModule and EthicalGovernanceModule.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["resolve_ethical_social_dilemmas".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "audit_social_norms_adaptation_ethically".to_string(), description: "Ensures that social norms learning does not lead to biased, discriminatory, or unethical behaviors, and that adaptation is transparent and auditable, with ALEC-A oversight and immutable logging via QR-CTFO.".to_string(), parameters: HashMap::new(), output_schema: "SocialNormsAuditReport".to_string(), requires_permissions: vec!["read_audit_logs".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "social-intelligence-module".to_string(), "global-knowledge-integration-module".to_string(), "moral-reasoning-module".to_string(), "ethical-governance-module".to_string(), "self-correction-biases-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
