// Tariro_ide_idpte/src/communication_bridge/human_agi_teaming_trust_building.rs

//! Implements Tariro's Human-AGI Teaming & Trust Building module,
//! enhancing Tariro's ability to seamlessly integrate into human teams,
//! build trust, and optimize collective performance.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::communication_bridge::human_agi_teaming_models::{TeamTask, TrustMetric, CollaborationStrategy, TeamingAuditReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical review of teaming
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For secure logging of interactions
use crate::cognitive_arch::meta_learner::MetaLearner; // For optimizing team performance
use crate::communication_bridge::human_agi_teaming::HumanAGITeamingModule; // Core teaming functions
use crate::cognitive_arch::explainable_ai::ExplainableAIModule; // For transparent behavior
use crate::security::trust_reputation::TrustReputationModule; // For managing trust scores
use crate::environmental::social_intelligence::SocialIntelligenceModule; // For understanding social cues
use crate::cognitive_arch::learning_models::VerificationStatus; // Alias for clarity

/// The Human-AGI Teaming & Trust Building Module.
pub struct HumanAGITeamingTrustBuildingModule {
    team_tasks: Mutex<HashMap<String, TeamTask>>, // ID -> Task
    trust_metrics: Mutex<HashMap<String, TrustMetric>>, // ID -> Metric
    collaboration_strategies: Mutex<HashMap<String, CollaborationStrategy>>, // ID -> Strategy
    teaming_audit_reports: Mutex<HashMap<String, TeamingAuditReport>>, // ID -> Audit
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    human_agi_teaming_module: Arc<HumanAGITeamingModule>,
    explainable_ai_module: Arc<ExplainableAIModule>,
    trust_reputation_module: Arc<TrustReputationModule>,
    social_intelligence_module: Arc<SocialIntelligenceModule>,
}

impl HumanAGITeamingTrustBuildingModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, human_agi_teaming_module: Arc<HumanAGITeamingModule>, explainable_ai_module: Arc<ExplainableAIModule>, trust_reputation_module: Arc<TrustReputationModule>, social_intelligence_module: Arc<SocialIntelligenceModule>) -> Self {
        info!("Communication Bridge: Human-AGI Teaming & Trust Building Module initialized.");
        let mut initial_strategies = HashMap::new();
        initial_strategies.insert("default_agile".to_string(), CollaborationStrategy {
            id: "default_agile".to_string(),
            strategy_name: "Agile Collaboration".to_string(),
            description: "Iterative, flexible team-based problem solving.".to_string(),
            principles: vec!["transparency".to_string(), "adaptability".to_string()],
            adaptive_parameters: HashMap::from([("feedback_loop_frequency".to_string(), "high".to_string())]),
            ethical_design_review: VerificationStatus::Passed("Strategy ethically sound.".to_string()),
            security_audited: VerificationStatus::Passed("Strategy implementation secure.".to_string()),
            last_updated: Utc::now(),
        });

        HumanAGITeamingTrustBuildingModule {
            team_tasks: Mutex::new(HashMap::new()),
            trust_metrics: Mutex::new(HashMap::new()),
            collaboration_strategies: Mutex::new(initial_strategies),
            teaming_audit_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            human_agi_teaming_module,
            explainable_ai_module,
            trust_reputation_module,
            social_intelligence_module,
        }
    }

    /// Develops capabilities for Tariro to understand team dynamics, roles, and communication
    /// protocols, adapting its behavior to become an effective and valued team member,
    /// leveraging SocialIntelligenceModule.
    pub async fn facilitate_seamless_team_integration(&self, team_id: &str, team_context: &str) -> Result<TeamTask, String> {
        info!("HATBM: Facilitating team integration for '{}' in context '{}'.".to_string(), team_id, team_context);

        // Use SocialIntelligenceModule to understand team dynamics (Sub-Task 145.1)
        let social_context_analysis = self.social_intelligence_module.analyze_social_cues(
            team_id,
            team_context,
            "team_dynamics"
        ).await.map_err(|e| e.to_string())?;

        // Use HumanAGITeamingModule for adapting behavior (Sub-Task 145.1)
        let adapted_behavior_recommendation = self.human_agi_teaming_module.recommend_behavior_for_team(
            team_id,
            &social_context_analysis.semantic_description,
            "effective_team_member"
        ).await.map_err(|e| e.to_string())?;

        let task_id = format!("team-task-{}-{}", team_id, Uuid::new_v4());
        let mut team_task = TeamTask {
            id: task_id.clone(),
            title: format!("Integrated into team: {}", team_id),
            description: format!("Adapted behavior based on team dynamics: {}. Recommended: {}.", social_context_analysis.semantic_description, adapted_behavior_recommendation.recommended_action),
            assigned_members: vec!["Tariro_AGI".to_string(), team_id.to_string()],
            status: "integrated".to_string(),
            ethical_review_status: VerificationStatus::Pending,
            security_implications_assessed: VerificationStatus::Pending,
            created_at: Utc::now(),
        };

        // Ethical review of the task's purpose (Sub-Task 145.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            team_task.id.clone(),
            "TeamTask".to_string(),
            format!("Ethical review of team integration in '{}'.", team_id)
        ).await.map_err(|e| e.to_string())?;
        team_task.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Team integration ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in team integration: {:?}", aleca_audit.violation_details)) };

        // Security assessment of task data handling via QR-CTFO (Sub-Task 145.3)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            team_task.id.clone(),
            "TeamTask".to_string(),
            format!("Audit team task '{}' for data handling security.", task_id)
        ).await.map_err(|e| e.to_string())?;
        team_task.security_implications_assessed = if qrctfo_audit.is_secure { VerificationStatus::Passed("Task data handling secure.".to_string()) } else { VerificationStatus::Failed(format!("Task data handling security issues: {:?}", qrctfo_audit.vulnerabilities)) };

        if matches!(team_task.ethical_review_status, VerificationStatus::Failed(_)) || matches!(team_task.security_implications_assessed, VerificationStatus::Failed(_)) {
            error!("HATBM: Team integration '{}' failed ethical/security review. Aborting.", team_task.id);
            return Err(format!("Team integration {} failed ethical/security review.", team_task.id));
        }

        self.team_tasks.lock().unwrap().insert(task_id.clone(), team_task.clone());
        info!("HATBM: Team integration for '{}' facilitated. Ethical: {:?}, Security: {:?}.", team_id, team_task.ethical_review_status, team_task.security_implications_assessed);
        self.event_bus.publish(EventEnvelope::new("HumanAGITeamingTrustBuildingModule".to_string(), OsEvent::Custom("TeamIntegrationFacilitated".to_string(), task_id))).await;
        Ok(team_task)
    }

    /// Registers the HumanAGITeamingTrustBuildingModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "human-agi-teaming-trust-building-module".to_string(),
            name: "Human-AGI Teaming & Trust Building Module".to_string(),
            description: "Enhances Tariro's ability to seamlessly integrate into human teams, build trust, and optimize collective performance.".to_string(),
            service_type: ServiceType::Communication,
            capabilities: vec![
                ServiceCapability { name: "facilitate_seamless_team_integration".to_string(), description: "Develops capabilities for Tariro to understand team dynamics, roles, and communication protocols, adapting its behavior to become an effective and valued team member, leveraging SocialIntelligenceModule.".to_string(), parameters: HashMap::new(), output_schema: "TeamTask".to_string(), requires_permissions: vec!["manage_team_integration".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "actively_build_and_maintain_trust".to_string(), description: "Implements mechanisms for Tariro to demonstrate reliability, competence, and ethical alignment, fostering trust with human and other AGI teammates, utilizing TrustReputationModule and ExplainableAIModule.".to_string(), parameters: HashMap::new(), output_schema: "TrustMetric".to_string(), requires_permissions: vec!["manage_trust_metrics".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "optimize_collective_team_performance".to_string(), description: "Analyzes team performance, identifies bottlenecks, and proposes adaptive strategies to enhance communication, coordination, and overall effectiveness, optimized by MetaLearner.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["optimize_team_performance".to_string()], requires_ethical_review: false },
                ServiceCapability { name: "audit_human_agi_teaming_ethically".to_string(), description: "Ensures that teaming processes are fair, transparent, and respect human autonomy, preventing manipulative or exploitative team dynamics, with ALEC-A oversight and immutable logging via QR-CTFO.".to_string(), parameters: HashMap::new(), output_schema: "TeamingAuditReport".to_string(), requires_permissions: vec!["read_audit_logs".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "human-agi-teaming-module".to_string(), "explainable-ai-module".to_string(), "trust-reputation-module".to_string(), "social-intelligence-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
