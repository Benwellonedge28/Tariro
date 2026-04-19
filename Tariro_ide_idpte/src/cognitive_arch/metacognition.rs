// Tariro_ide_idpte/src/cognitive_arch/metacognition.rs

//! Implements the Metacognition & Self-Reflection module, enabling Tariro to monitor
//! and regulate its own cognitive processes, perform self-diagnosis, and generate
//! self-improvement directives.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::metacognition_models::{CognitiveState, SelfCorrectionAction, ReflectionLog, IntrospectionReport, ReflectionVerificationStatus};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical review of self-correction plans
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For secure logging and integrity
use crate::cognitive_arch::meta_learner::MetaLearner; // For optimizing self-improvement strategies
use crate::cognitive_arch::learning_models::VerificationStatus; // Alias for clarity

/// The Metacognition & Self-Reflection module.
pub struct MetacognitionModule {
    reflection_logs: Mutex<HashMap<String, ReflectionLog>>, // ID -> Log
    self_correction_actions: Mutex<HashMap<String, SelfCorrectionAction>>, // ID -> Action
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
}

impl MetacognitionModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>) -> Self {
        info!("Cognitive Arch: Metacognition Module initialized.");
        MetacognitionModule {
            reflection_logs: Mutex::new(HashMap::new()),
            self_correction_actions: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
        }
    }

    /// Autonomously monitors Tariro's cognitive state and performance.
    pub async fn monitor_cognitive_state(&self, event_summary: &str) -> Result<CognitiveState, String> {
        info!("Metacognition: Monitoring cognitive state triggered by event: '{}'.", event_summary);
        
        // Simulate gathering cognitive state (e.g., querying other modules)
        let current_cognitive_state = CognitiveState {
            id: format!("cog_state-{}", Uuid::new_v4()),
            timestamp: Utc::now(),
            module_states: HashMap::from([
                ("MetaLearner".to_string(), "active".to_string()),
                ("CausalReasoningEngine".to_string(), "idle".to_string()),
            ]),
            active_goals: vec!["Infinity Advancement".to_string()],
            resource_utilization: HashMap::from([("cpu".to_string(), 0.7), ("memory".to_string(), 0.5)]),
            emotional_state_summary: "focused".to_string(),
            confidence_in_current_action: 0.9,
            integrity_hash: "pending_hash".to_string(), // Will be updated by QR-CTFO
        };

        // Securely log the cognitive state (Sub-Task 73.1 & 73.2)
        self.log_reflection("CognitiveStateMonitoring".to_string(), event_summary.to_string(), current_cognitive_state.clone()).await?;
        Ok(current_cognitive_state)
    }

    /// Performs autonomous self-diagnosis to identify errors, inefficiencies, or sub-optimal behaviors.
    pub async fn self_diagnose_errors(&self, cognitive_state: &CognitiveState) -> Result<Vec<SelfCorrectionAction>, String> {
        info!("Metacognition: Performing self-diagnosis on cognitive state '{}'.", cognitive_state.id);
        let mut identified_problems = Vec::new();
        let mut proposed_actions = Vec::new();

        // Simulate error detection (e.g., low confidence, high resource usage, goal deviation)
        if cognitive_state.confidence_in_current_action < 0.5 {
            let problem_desc = format!("Low confidence in current action: {}.", cognitive_state.emotional_state_summary);
            let action = SelfCorrectionAction {
                id: format!("self_correct-{}-re_eval", Uuid::new_v4()),
                problem_description: problem_desc.clone(),
                proposed_action: "Re-evaluate assumptions and gather more data.".to_string(),
                target_module_id: "N/A".to_string(), // Could target specific module
                expected_outcome: "Increased confidence.".to_string(),
                ethical_review_status: VerificationStatus::Pending,
                security_review_status: VerificationStatus::Pending,
                initiated_at: Utc::now(),
            };
            identified_problems.push(problem_desc);
            proposed_actions.push(action);
        }

        if let Some(cpu_usage) = cognitive_state.resource_utilization.get("cpu") {
            if *cpu_usage > 0.9 {
                let problem_desc = format!("High CPU usage: {}.", cpu_usage);
                let action = SelfCorrectionAction {
                    id: format!("self_correct-{}-optimize", Uuid::new_v4()),
                    problem_description: problem_desc.clone(),
                    proposed_action: "Optimize current task or offload to another resource.".to_string(),
                    target_module_id: "SORAEE".to_string(),
                    expected_outcome: "Reduced CPU usage.".to_string(),
                    ethical_review_status: VerificationStatus::Pending,
                    security_review_status: VerificationStatus::Pending,
                    initiated_at: Utc::now(),
                };
                identified_problems.push(problem_desc);
                proposed_actions.push(action);
            }
        }

        // Ethically and securely review each proposed self-correction action (Sub-Task 73.1 & 73.2)
        let mut verified_actions = Vec::new();
        for mut action in proposed_actions {
            let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
                action.id.clone(),
                "SelfCorrectionAction".to_string(),
                format!("Review self-correction action '{}' for ethical implications.", action.proposed_action)
            ).await.map_err(|e| e.to_string())?;
            action.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Ethically compliant.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns: {:?}", aleca_audit.violation_details)) };

            let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
                action.id.clone(),
                "SelfCorrectionAction".to_string(),
                format!("Audit self-correction action '{}' for security vulnerabilities.", action.proposed_action)
            ).await.map_err(|e| e.to_string())?;
            action.security_review_status = if qrctfo_audit.is_secure { VerificationStatus::Passed("Secure from vulnerabilities.".to_string()) } else { VerificationStatus::Failed(format!("Security issues: {:?}", qrctfo_audit.vulnerabilities)) };

            if matches!(action.ethical_review_status, VerificationStatus::Failed(_)) || matches!(action.security_review_status, VerificationStatus::Failed(_)) {
                warn!("Metacognition: Self-correction action '{}' failed ethical/security review. Discarding.", action.id);
            } else {
                info!("Metacognition: Self-correction action '{}' verified.", action.id);
                self.self_correction_actions.lock().unwrap().insert(action.id.clone(), action.clone());
                verified_actions.push(action);
            }
        }
        
        self.event_bus.publish(EventEnvelope::new("MetacognitionModule".to_string(), OsEvent::Custom("SelfDiagnosisCompleted".to_string(), format!("{{ \"state_id\": \"{}\", \"num_problems\": {} }}", cognitive_state.id, identified_problems.len())))).await;
        Ok(verified_actions)
    }

    /// Generates and executes a plan for self-improvement based on diagnosed issues.
    /// Leverages MetaLearner for optimizing self-improvement strategies.
    pub async fn generate_self_improvement_plan(&self, identified_actions: Vec<SelfCorrectionAction>) -> Result<(), String> {
        info!("Metacognition: Generating self-improvement plan for {} actions.", identified_actions.len());
        
        let mut improvement_plan_summary = String::new();
        for action in identified_actions {
            if matches!(action.ethical_review_status, VerificationStatus::Passed(_)) && matches!(action.security_review_status, VerificationStatus::Passed(_)) {
                improvement_plan_summary.push_str(&format!(" - Implement: '{}' (Target: {})", action.proposed_action, action.target_module_id));
                
                // Use MetaLearner to optimize the self-improvement strategy (Sub-Task 73.1)
                let optimization_algorithm = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
                    id: format!("self_improvement_problem_{}", Uuid::new_v4()),
                    name: format!("Optimizing: {}", action.problem_description),
                    domain: "metacognition".to_string(),
                    problem_type: "optimization".to_string(),
                    complexity_score: 0.8,
                    data_characteristics: HashMap::from([("action_target".to_string(), action.target_module_id.clone())]),
                    required_accuracy: 0.95,
                    created_at: Utc::now(),
                }).await.map_err(|e| e.to_string())?;
                info!("Metacognition: Using optimization algorithm '{}' for action '{}'.", optimization_algorithm.name, action.id);
                
                // Execute the self-correction (simulated)
                self.event_bus.publish(EventEnvelope::new("MetacognitionModule".to_string(), OsEvent::Custom("SelfCorrectionExecuted".to_string(), format!("{{ \"action_id\": \"{}\", \"method\": \"{}\" }}", action.id, optimization_algorithm.name)))).await;
            } else {
                warn!("Metacognition: Skipping unverified self-correction action '{}'.", action.id);
            }
        }

        info!("Metacognition: Self-improvement plan generated and initiated: {}.", improvement_plan_summary);
        self.event_bus.publish(EventEnvelope::new("MetacognitionModule".to_string(), OsEvent::Custom("SelfImprovementPlanGenerated".to_string(), improvement_plan_summary))).await;
        Ok(())
    }

    /// Logs a reflection event, including a secure cognitive state snapshot.
    async fn log_reflection(&self, event_type: String, event_summary: String, mut cognitive_state: CognitiveState) -> Result<(), String> {
        let log_id = format!("reflection-{}-{}", event_type, Uuid::new_v4());
        
        // Secure logging of cognitive state (Sub-Task 73.2)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            cognitive_state.id.clone(),
            "CognitiveStateSnapshot".to_string(),
            format!("Audit cognitive state snapshot '{}' for integrity.", cognitive_state.id)
        ).await.map_err(|e| e.to_string())?;

        cognitive_state.integrity_hash = qrctfo_audit.audit_id; // Store audit ID as proof of integrity

        let mut log_entry = ReflectionLog {
            id: log_id.clone(),
            timestamp: Utc::now(),
            event_summary,
            cognitive_state_snapshot: cognitive_state,
            self_diagnosis: "Pending self-diagnosis.".to_string(),
            identified_opportunities: Vec::new(),
            generated_self_correction_ids: Vec::new(),
            log_integrity_status: VerificationStatus::Passed(format!("Log integrity proof: {}.", qrctfo_audit.audit_id)),
        };
        
        if !qrctfo_audit.is_secure {
            log_entry.log_integrity_status = VerificationStatus::Failed(format!("Integrity audit failed: {:?}", qrctfo_audit.vulnerabilities));
            warn!("Metacognition: Reflection log '{}' integrity audit failed.", log_id);
        }

        self.reflection_logs.lock().unwrap().insert(log_id.clone(), log_entry.clone());
        self.event_bus.publish(EventEnvelope::new("MetacognitionModule".to_string(), OsEvent::Custom("ReflectionLogged".to_string(), log_id))).await;
        Ok(())        
    }

    /// Generates a detailed introspection report for a given period.
    pub async fn generate_introspection_report(&self, period_start: DateTime<Utc>, period_end: DateTime<Utc>) -> Result<IntrospectionReport, String> {
        info!("Metacognition: Generating introspection report for period {} to {}.", period_start, period_end);
        let logs_guard = self.reflection_logs.lock().unwrap();
        let actions_guard = self.self_correction_actions.lock().unwrap();

        let relevant_logs: Vec<&ReflectionLog> = logs_guard.values()
            .filter(|log| log.timestamp >= period_start && log.timestamp <= period_end)
            .collect();
        
        let summary_of_performance = format!("Reviewed {} reflection logs. Identified {} self-correction actions.", relevant_logs.len(), actions_guard.len());
        let key_insights = vec!["Learned to optimize resource allocation in high-stress scenarios.".to_string()];
        let major_self_correction_ids = actions_guard.values().filter(|a| a.problem_description.contains("critical")).map(|a| a.id.clone()).collect();
        let areas_for_improvement = vec!["Further refine emotional state regulation.".to_string()];

        let report_id = format!("report-{}-{}", period_start.timestamp(), period_end.timestamp());
        let mut introspection_report = IntrospectionReport {
            id: report_id.clone(),
            period_start,
            period_end,
            summary_of_performance,
            key_insights,
            major_self_correction_actions: major_self_correction_ids,
            areas_for_improvement,
            report_integrity_hash: "pending_hash".to_string(),
            generated_at: Utc::now(),
        };

        // Secure the report with QR-CTFO
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            introspection_report.id.clone(),
            "IntrospectionReport".to_string(),
            format!("Audit introspection report '{}' for integrity.", introspection_report.id)
        ).await.map_err(|e| e.to_string())?;

        introspection_report.report_integrity_hash = qrctfo_audit.audit_id;

        info!("Metacognition: Introspection report '{}' generated with integrity hash {}.", introspection_report.id, introspection_report.report_integrity_hash);
        self.event_bus.publish(EventEnvelope::new("MetacognitionModule".to_string(), OsEvent::Custom("IntrospectionReportGenerated".to_string(), report_id))).await;
        Ok(introspection_report)
    }

    /// Registers the MetacognitionModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "metacognition-module".to_string(),
            name: "Metacognition & Self-Reflection Module".to_string(),
            description: "Monitors and regulates cognitive processes, performs self-diagnosis, and generates self-improvement directives.".to_string(),
            service_type: ServiceType::AI_Core,
            capabilities: vec![
                ServiceCapability { name: "monitor_cognitive_state".to_string(), description: "Autonomously monitors and securely logs Tariro's cognitive state.".to_string(), parameters: HashMap::new(), output_schema: "CognitiveState".to_string(), requires_permissions: vec![], requires_ethical_review: true },
                ServiceCapability { name: "self_diagnose_errors".to_string(), description: "Performs autonomous self-diagnosis and ethically/securely reviews self-correction actions.".to_string(), parameters: HashMap::new(), output_schema: "Vec<SelfCorrectionAction>".to_string(), requires_permissions: vec![], requires_ethical_review: true },
                ServiceCapability { name: "generate_self_improvement_plan".to_string(), description: "Generates and executes optimized self-improvement plans based on diagnosed issues.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["modify_self".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "generate_introspection_report".to_string(), description: "Generates a detailed, integrity-verified introspection report.".to_string(), parameters: HashMap::new(), output_schema: "IntrospectionReport".to_string(), requires_permissions: vec![], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string()],
            managed_by_agi_module: None,
        }
    }
}
