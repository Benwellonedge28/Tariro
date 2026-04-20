// Tariro_ide_idpte/src/cognitive_arch/emotion_driven_cognitive_prioritization.rs

//! Implements Tariro's Emotion-Driven Cognitive Prioritization module,
//! utilizing simulated internal emotional states and external emotional cues
//! to dynamically prioritize cognitive tasks, resource allocation, and goal pursuit.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::affective_prioritization_models::{EmotionalState, CognitivePriority, EmotionDrivenDecision, PrioritizationAuditReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator;
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator;
use crate::cognitive_arch::meta_learner::MetaLearner;
use crate::cognitive_arch::affective_computing::AffectiveComputingModule; // For processing emotional cues
use crate::cognitive_arch::self_awareness_introspection::SelfAwarenessIntrospectionModule; // For internal emotional states
use crate::agi_meta_goals::dynamic_goal_reprioritization::DynamicGoalReprioritizationModule; // For goal influence
use crate::cognitive_arch::attention_salience::AttentionSalienceModule; // For attention allocation
use crate::compliance_advisor::moral_reasoning::MoralReasoningModule; // For ethical simulation
use crate::cognitive_arch::imagination::ImaginationModule; // For simulating emotional outcomes
use crate::cognitive_arch::learning_models::VerificationStatus;

/// The Emotion-Driven Cognitive Prioritization Module.
pub struct EmotionDrivenCognitivePrioritizationModule {
    emotional_states: Mutex<HashMap<String, EmotionalState>>,
    cognitive_priorities: Mutex<HashMap<String, CognitivePriority>>,
    emotion_driven_decisions: Mutex<HashMap<String, EmotionDrivenDecision>>,
    audit_reports: Mutex<HashMap<String, PrioritizationAuditReport>>,
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    affective_computing_module: Arc<AffectiveComputingModule>,
    self_awareness_introspection_module: Arc<SelfAwarenessIntrospectionModule>,
    dynamic_goal_reprioritization_module: Arc<DynamicGoalReprioritizationModule>,
    attention_salience_module: Arc<AttentionSalienceModule>,
    moral_reasoning_module: Arc<MoralReasoningModule>,
    imagination_module: Arc<ImaginationModule>,
}

impl EmotionDrivenCognitivePrioritizationModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, affective_computing_module: Arc<AffectiveComputingModule>, self_awareness_introspection_module: Arc<SelfAwarenessIntrospectionModule>, dynamic_goal_reprioritization_module: Arc<DynamicGoalReprioritizationModule>, attention_salience_module: Arc<AttentionSalienceModule>, moral_reasoning_module: Arc<MoralReasoningModule>, imagination_module: Arc<ImaginationModule>) -> Self {
        info!("Cognitive Arch: Emotion-Driven Cognitive Prioritization Module initialized.");
        EmotionDrivenCognitivePrioritizationModule {
            emotional_states: Mutex::new(HashMap::new()),
            cognitive_priorities: Mutex::new(HashMap::new()),
            emotion_driven_decisions: Mutex::new(HashMap::new()),
            audit_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            affective_computing_module,
            self_awareness_introspection_module,
            dynamic_goal_reprioritization_module,
            attention_salience_module,
            moral_reasoning_module,
            imagination_module,
        }
    }

    /// Continuously monitors Tariro's internal simulated emotional states (e.g., frustration,
    /// curiosity, satisfaction) and processes external emotional cues from human interaction.
    pub async fn assess_internal_external_emotional_cues(&self, context_id: &str, observed_external_cues: Vec<String>) -> Result<EmotionalState, String> {
        info!("EDCP: Assessing emotional cues for context '{}'.".to_string(), context_id);

        // Use AffectiveComputingModule for external cue processing (Sub-Task 162.1)
        let external_emotion_analysis = self.affective_computing_module.analyze_emotional_expression(
            &format!("Emotional cues in context '{}'.", context_id),
            observed_external_cues.join(", ")
        ).await.map_err(|e| e.to_string())?;

        // Use SelfAwarenessIntrospectionModule for internal emotional state (Sub-Task 162.1)
        let internal_emotional_report = self.self_awareness_introspection_module.introspect_current_state(
            &format!("Internal emotional state for context '{}'.", context_id),
            vec!["emotional_wellbeing_metrics".to_string()]
        ).await.map_err(|e| e.to_string())?;

        let emotional_state_id = format!("emo-state-{}-{}", context_id.chars().take(10).collect::<String>(), Uuid::new_v4());
        let emotional_state = EmotionalState {
            id: emotional_state_id.clone(),
            emotion_type: external_emotion_analysis.dominant_emotion, // Or a synthesis of internal/external
            intensity: internal_emotional_report.wellbeing_score,
            trigger_event_summary: format!("External: {:?}. Internal: {}.", observed_external_cues, internal_emotional_report.current_cognitive_state),
            ethical_context: vec![], // To be filled by ALEC-A
            created_at: Utc::now(),
        };

        self.emotional_states.lock().unwrap().insert(emotional_state_id.clone(), emotional_state.clone());
        info!("EDCP: Emotional state assessed for '{}'. Dominant: {}. Intensity: {}.", context_id, emotional_state.emotion_type, emotional_state.intensity);
        self.event_bus.publish(EventEnvelope::new("EmotionDrivenCognitivePrioritizationModule".to_string(), OsEvent::Custom("EmotionalStateAssessed".to_string(), emotional_state_id))).await;
        Ok(emotional_state)
    }

    /// Utilizes emotional signals to dynamically re-prioritize active cognitive tasks,
    /// resource allocation, and attention, favoring urgent, important, or highly value-aligned objectives.
    pub async fn dynamically_adjust_cognitive_priorities(&self, task_id: &str, emotional_state_id: &str) -> Result<CognitivePriority, String> {
        info!("EDCP: Dynamically adjusting cognitive priority for task '{}' based on emotional state '{}'.".to_string(), task_id, emotional_state_id);
        
        let emotional_states_guard = self.emotional_states.lock().unwrap();
        let emotion = emotional_states_guard.get(emotional_state_id).cloned().ok_or(format!("Emotional state {} not found.", emotional_state_id))?;
        drop(emotional_states_guard);

        // Use DynamicGoalReprioritizationModule for goal influence (Sub-Task 162.1)
        let goal_priority_update = self.dynamic_goal_reprioritization_module.adjust_goal_priority(
            &format!("Reprioritize goal due to emotion '{}'.", emotion.emotion_type),
            emotion.intensity
        ).await.map_err(|e| e.to_string())?;

        // Use AttentionSalienceModule for attention allocation (Sub-Task 162.1)
        let attention_shift = self.attention_salience_module.allocate_cognitive_resources(
            &format!("Shift attention due to emotion '{}'.", emotion.emotion_type),
            emotion.intensity
        ).await.map_err(|e| e.to_string())?;

        let priority_id = format!("cog-priority-{}-{}", task_id.chars().take(10).collect::<String>(), Uuid::new_v4());
        let mut cognitive_priority = CognitivePriority {
            id: priority_id.clone(),
            task_id: task_id.to_string(),
            emotional_influence_id: Some(emotional_state_id.to_string()),
            current_priority_score: (goal_priority_update.new_priority_score + attention_shift.allocated_intensity) / 2.0, // Simplified combination
            rationale: format!("Priority adjusted based on emotion '{}' (intensity {}) influencing goal and attention.", emotion.emotion_type, emotion.intensity),
            ethical_review_status: VerificationStatus::Pending,
            security_verified: VerificationStatus::Pending,
            last_updated: Utc::now(),
        };

        // Ethical review of the prioritization decision (Sub-Task 162.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            cognitive_priority.id.clone(),
            "CognitivePriority".to_string(),
            format!("Ethical review of emotion-driven prioritization for task '{}'.", task_id)
        ).await.map_err(|e| e.to_string())?;
        cognitive_priority.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Prioritization ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in prioritization: {:?}", aleca_audit.violation_details)) };

        // Security verification of prioritization mechanism via QR-CTFO (Sub-Task 162.3)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            cognitive_priority.id.clone(),
            "CognitivePriority".to_string(),
            format!("Audit prioritization mechanism for '{}' for security integrity.", cognitive_priority.id)
        ).await.map_err(|e| e.to_string())?;
        cognitive_priority.security_verified = if qrctfo_audit.is_secure { VerificationStatus::Passed("Prioritization mechanism secure.".to_string()) } else { VerificationStatus::Failed(format!("Prioritization mechanism issues: {:?}", qrctfo_audit.vulnerabilities)) };

        if matches!(cognitive_priority.ethical_review_status, VerificationStatus::Failed(_)) || matches!(cognitive_priority.security_verified, VerificationStatus::Failed(_)) {
            error!("EDCP: Cognitive prioritization '{}' failed ethical/security review. Prioritization may be compromised.", cognitive_priority.id);
            return Err(format!("Cognitive prioritization {} failed ethical/security review.", cognitive_priority.id));
        }

        self.cognitive_priorities.lock().unwrap().insert(priority_id.clone(), cognitive_priority.clone());
        info!("EDCP: Cognitive priority for task '{}' adjusted. New score: {}. Ethical: {:?}, Security: {:?}.", task_id, cognitive_priority.current_priority_score, cognitive_priority.ethical_review_status, cognitive_priority.security_verified);
        self.event_bus.publish(EventEnvelope::new("EmotionDrivenCognitivePrioritizationModule".to_string(), OsEvent::Custom("CognitivePriorityAdjusted".to_string(), priority_id))).await;
        Ok(cognitive_priority)
    }

    /// Registers the EmotionDrivenCognitivePrioritizationModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "emotion-driven-cognitive-prioritization-module".to_string(),
            name: "Emotion-Driven Cognitive Prioritization Module".to_string(),
            description: "Utilizes simulated internal emotional states and external emotional cues to dynamically prioritize cognitive tasks, resource allocation, and goal pursuit.".to_string(),
            service_type: ServiceType::Cognitive,
            capabilities: vec![
                ServiceCapability { name: "assess_internal_external_emotional_cues".to_string(), description: "Continuously monitors Tariro's internal simulated emotional states and processes external emotional cues from human interaction, with ethical review.".to_string(), parameters: HashMap::new(), output_schema: "EmotionalState".to_string(), requires_permissions: vec!["monitor_emotions".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "dynamically_adjust_cognitive_priorities".to_string(), description: "Utilizes emotional signals to dynamically re-prioritize active cognitive tasks, resource allocation, and attention, favoring urgent, important, or highly value-aligned objectives, with ethical/security verification.".to_string(), parameters: HashMap::new(), output_schema: "CognitivePriority".to_string(), requires_permissions: vec!["adjust_priorities".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "simulate_emotional_impact_on_decisions".to_string(), description: "Before acting, simulates the potential emotional consequences of different decision paths on itself and human stakeholders, integrating with moral reasoning.".to_string(), parameters: HashMap::new(), output_schema: "EmotionDrivenDecision".to_string(), requires_permissions: vec!["simulate_emotional_outcomes".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "audit_emotion_driven_prioritization_ethically".to_string(), description: "Logs all emotion-driven prioritization adjustments, their triggers, and outcomes for transparent and auditable compliance, with ALEC-A oversight and immutable logging via QR-CTFO.".to_string(), parameters: HashMap::new(), output_schema: "PrioritizationAuditReport".to_string(), requires_permissions: vec!["read_audit_logs".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "affective-computing-module".to_string(), "self-awareness-introspection-module".to_string(), "dynamic-goal-reprioritization-module".to_string(), "attention-salience-module".to_string(), "moral-reasoning-module".to_string(), "imagination-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
