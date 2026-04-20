// Tariro_ide_idpte/src/security/explainable_security_decision_making_auditability.rs

//! Implements Tariro's Explainable Security Decision Making & Auditability module,
//! ensuring all security-related decisions are accompanied by clear, human-understandable
//! explanations and are fully auditable.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::security::explainable_security_models::{SecurityDecisionExplanation, AuditTrailEntry, HumanUnderstandabilityMetric, SecurityAuditReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator;
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator;
use crate::cognitive_arch::meta_learner::MetaLearner;
use crate::cognitive_arch::explainable_ai::ExplainableAIModule; // For general explainability
use crate::communication_bridge::human_agi_teaming_trust_building::HumanAGITeamingTrustBuildingModule; // For human understanding
use crate::security::secure_enclave::SecureEnclaveModule; // For immutable logging
use crate::cognitive_arch::self_awareness_introspection::SelfAwarenessIntrospectionModule; // For self-assessment of explanations
use crate::cognitive_arch::epistemic_reasoning::EpistemicReasoningModule; // For justifying knowledge behind decisions
use crate::compliance_advisor::dynamic_ethical_constraint_generation::DynamicEthicalConstraintGenerationModule; // For ethical context
use crate::cognitive_arch::learning_models::VerificationStatus;

/// The Explainable Security Decision Making & Auditability Module.
pub struct ExplainableSecurityDecisionMakingAuditabilityModule {
    security_explanations: Mutex<HashMap<String, SecurityDecisionExplanation>>,
    audit_trail_entries: Mutex<HashMap<String, AuditTrailEntry>>,
    human_understandability_metrics: Mutex<HashMap<String, HumanUnderstandabilityMetric>>,
    audit_reports: Mutex<HashMap<String, SecurityAuditReport>>,
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    explainable_ai_module: Arc<ExplainableAIModule>,
    human_agi_teaming_trust_building_module: Arc<HumanAGITeamingTrustBuildingModule>,
    secure_enclave_module: Arc<SecureEnclaveModule>,
    self_awareness_introspection_module: Arc<SelfAwarenessIntrospectionModule>,
    epistemic_reasoning_module: Arc<EpistemicReasoningModule>,
    dynamic_ethical_constraint_generation_module: Arc<DynamicEthicalConstraintGenerationModule>,
}

impl ExplainableSecurityDecisionMakingAuditabilityModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, explainable_ai_module: Arc<ExplainableAIModule>, human_agi_teaming_trust_building_module: Arc<HumanAGITeamingTrustBuildingModule>, secure_enclave_module: Arc<SecureEnclaveModule>, self_awareness_introspection_module: Arc<SelfAwarenessIntrospectionModule>, epistemic_reasoning_module: Arc<EpistemicReasoningModule>, dynamic_ethical_constraint_generation_module: Arc<DynamicEthicalConstraintGenerationModule>) -> Self {
        info!("Security: Explainable Security Decision Making & Auditability Module initialized.");
        ExplainableSecurityDecisionMakingAuditabilityModule {
            security_explanations: Mutex::new(HashMap::new()),
            audit_trail_entries: Mutex::new(HashMap::new()),
            human_understandability_metrics: Mutex::new(HashMap::new()),
            audit_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            explainable_ai_module,
            human_agi_teaming_trust_building_module,
            secure_enclave_module,
            self_awareness_introspection_module,
            epistemic_reasoning_module,
            dynamic_ethical_constraint_generation_module,
        }
    }

    /// For every security-related decision (e.g., access grants, threat responses, policy changes),
    /// Tariro generates a clear, concise, and context-rich explanation intelligible to human operators.
    pub async fn generate_human_understandable_security_explanations(&self, decision_id: &str, decision_summary: &str) -> Result<SecurityDecisionExplanation, String> {
        info!("ESDMA: Generating security explanation for decision '{}'.".to_string(), decision_id);

        // Use ExplainableAIModule for core explanation generation (Sub-Task 181.1)
        let explanation_details = self.explainable_ai_module.generate_explanation_for_decision(
            decision_id,
            decision_summary,
            "security_decision_rationale"
        ).await.map_err(|e| e.to_string())?;

        // Use HumanAGITeamingTrustBuildingModule for human-centric communication (Sub-Task 181.1)
        let human_feedback = self.human_agi_teaming_trust_building_module.request_explanation_feedback(
            &format!("Explanation for '{}'.", decision_id),
            &explanation_details.explanation_summary
        ).await.map_err(|e| e.to_string())?;

        let explanation_id = format!("sec-exp-{}-{}", decision_id.chars().take(10).collect::<String>(), Uuid::new_v4());
        let mut security_decision_explanation = SecurityDecisionExplanation {
            id: explanation_id.clone(),
            decision_id: decision_id.to_string(),
            decision_summary: decision_summary.to_string(),
            rationale_text: explanation_details.explanation_summary,
            supporting_evidence: explanation_details.key_factors,
            ethical_transparency_review: VerificationStatus::Pending,
            generated_at: Utc::now(),
        };

        // Ethical transparency review of the explanation's transparency (Sub-Task 181.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            security_decision_explanation.id.clone(),
            "SecurityDecisionExplanation".to_string(),
            format!("Ethical transparency review of security explanation for '{}'.", decision_id)
        ).await.map_err(|e| e.to_string())?;
        security_decision_explanation.ethical_transparency_review = if aleca_audit.is_compliant { VerificationStatus::Passed("Explanation transparent.") } else { VerificationStatus::Failed(format!("Transparency concerns: {:?}", aleca_audit.violation_details)) };

        if matches!(security_decision_explanation.ethical_transparency_review, VerificationStatus::Failed(_)) {
            error!("ESDMA: Security explanation '{}' failed ethical review. Human review required.", security_decision_explanation.id);
            return Err(format!("Security explanation {} failed ethical review.", security_decision_explanation.id));
        }

        self.security_explanations.lock().unwrap().insert(explanation_id.clone(), security_decision_explanation.clone());
        info!("ESDMA: Security decision explanation generated for '{}'. Human feedback: {}.", decision_id, human_feedback.clarity_score);
        self.event_bus.publish(EventEnvelope::new("ExplainableSecurityDecisionMakingAuditabilityModule".to_string(), OsEvent::Custom("SecurityExplanationGenerated".to_string(), explanation_id))).await;
        Ok(security_decision_explanation)
    }

    /// Ensures that all security decisions, actions, and their corresponding explanations
    /// are logged immutably in a tamper-proof audit trail.
    pub async fn ensure_comprehensive_immutable_security_audit_trails(&self, decision_id: &str, explanation_id: &str, event_type: &str) -> Result<AuditTrailEntry, String> {
        info!("ESDMA: Ensuring immutable audit trail for decision '{}' with explanation '{}'.".to_string(), decision_id, explanation_id);
        
        let explanations_guard = self.security_explanations.lock().unwrap();
        let explanation = explanations_guard.get(explanation_id).cloned().ok_or(format!("Explanation {} not found.", explanation_id))?;
        drop(explanations_guard);

        // Use SecureEnclaveModule for immutable logging (Sub-Task 181.1)
        let immutable_log_hash = self.secure_enclave_module.log_critical_event_to_immutable_ledger(
            &format!("Security event '{}' for decision '{}'. Explanation: {}.", event_type, decision_id, explanation.rationale_text)
        ).await.map_err(|e| e.to_string())?;

        // Use QrCtfoOrchestrator for cryptographic hashing (Sub-Task 181.1)
        let cryptographic_proof = self.qrctfo_orchestrator.lock().unwrap().generate_cryptographic_proof(
            &format!("Hash of audit entry for '{}'.", decision_id),
            immutable_log_hash.event_hash.clone()
        ).await.map_err(|e| e.to_string())?;

        let entry_id = format!("audit-entry-{}-{}", decision_id.chars().take(10).collect::<String>(), Uuid::new_v4());
        let mut audit_trail_entry = AuditTrailEntry {
            id: entry_id.clone(),
            event_type: event_type.to_string(),
            timestamp: Utc::now(),
            component_id: "AGI_Security_System".to_string(), // Placeholder
            associated_explanation_id: Some(explanation_id.to_string()),
            ethical_integrity_status: VerificationStatus::Pending,
            cryptographic_hash: cryptographic_proof.proof,
            recorded_at: Utc::now(),
        };

        // Ethical integrity status of the audit process (Sub-Task 181.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            audit_trail_entry.id.clone(),
            "AuditTrailEntry".to_string(),
            format!("Ethical integrity review of audit trail entry for '{}'.", decision_id)
        ).await.map_err(|e| e.to_string())?;
        audit_trail_entry.ethical_integrity_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Audit trail ethically sound.") } else { VerificationStatus::Failed(format!("Ethical concerns in audit trail: {:?}", aleca_audit.violation_details)) };

        if matches!(audit_trail_entry.ethical_integrity_status, VerificationStatus::Failed(_)) {
            error!("ESDMA: Audit trail entry '{}' failed ethical review. Potential tampering or flaw.", audit_trail_entry.id);
            return Err(format!("Audit trail entry {} failed ethical review.", audit_trail_entry.id));
        }

        self.audit_trail_entries.lock().unwrap().insert(entry_id.clone(), audit_trail_entry.clone());
        info!("ESDMA: Immutable audit trail entry recorded for decision '{}'. Hash: {}.", decision_id, audit_trail_entry.cryptographic_hash);
        self.event_bus.publish(EventEnvelope::new("ExplainableSecurityDecisionMakingAuditabilityModule".to_string(), OsEvent::Custom("AuditTrailEntryRecorded".to_string(), entry_id))).await;
        Ok(audit_trail_entry)
    }

    pub fn as_os_service() -> OsService {
        OsService {
            id: "explainable-security-decision-making-auditability-module".to_string(),
            name: "Explainable Security Decision Making & Auditability Module".to_string(),
            description: "Ensures all security-related decisions are accompanied by clear, human-understandable explanations and are fully auditable.".to_string(),
            service_type: ServiceType::Security,
            capabilities: vec![
                ServiceCapability { name: "generate_human_understandable_security_explanations".to_string(), description: "Generates clear, concise, and context-rich explanations for every security-related decision, intelligible to human operators, with ethical review.".to_string(), parameters: HashMap::new(), output_schema: "SecurityDecisionExplanation".to_string(), requires_permissions: vec!["explain_security_decisions".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "ensure_comprehensive_immutable_security_audit_trails".to_string(), description: "All security decisions, actions, and explanations are logged immutably in a tamper-proof audit trail for transparency and accountability, with ethical/security verification.".to_string(), parameters: HashMap::new(), output_schema: "AuditTrailEntry".to_string(), requires_permissions: vec!["log_security_events".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "proactively_assess_explanation_clarity_and_completeness".to_string(), description: "Continuously evaluates the clarity, accuracy, and completeness of generated explanations, adjusting communication strategy based on human feedback and understanding metrics.".to_string(), parameters: HashMap::new(), output_schema: "HumanUnderstandabilityMetric".to_string(), requires_permissions: vec!["evaluate_explainability".to_string()], requires_ethical_review: false },
                ServiceCapability { name: "audit_security_explainability_for_compliance_and_trust".to_string(), description: "Logs all security explainability and auditability processes, their effectiveness, and any detected discrepancies for transparent and auditable compliance, with ALEC-A oversight and immutable logging via QR-CTFO.".to_string(), parameters: HashMap::new(), output_schema: "SecurityAuditReport".to_string(), requires_permissions: vec!["read_audit_logs".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "explainable-ai-module".to_string(), "human-agi-teaming-trust-building-module".to_string(), "secure-enclave-module".to_string(), "self-awareness-introspection-module".to_string(), "epistemic-reasoning-module".to_string(), "dynamic-ethical-constraint-generation-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
