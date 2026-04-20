// Tariro_ide_idpte/src/cognitive_arch/metaphorical_mapping.rs

//! Implements Tariro's Cross-Domain Metaphorical Mapping module,
//! enabling the discovery of structural similarities between disparate fields
//! and the transfer of insights across domain boundaries.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::metaphorical_mapping_models::{MetaphoricalBridge, InsightTransferProposal, MetaphoricalInsightReport, MetaphorAuditReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator;
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator;
use crate::cognitive_arch::meta_learner::MetaLearner;
use crate::cognitive_arch::analogical_reasoning::AnalogicalReasoningSystem;
use crate::cognitive_arch::concept_formation::ConceptFormationModule;
use crate::cognitive_arch::abstract_reasoning::AbstractReasoningModule;
use crate::cognitive_arch::learning_models::VerificationStatus;

/// The Cross-Domain Metaphorical Mapping Module.
pub struct MetaphoricalMappingModule {
    metaphorical_bridges: Mutex<HashMap<String, MetaphoricalBridge>>,
    insight_proposals: Mutex<HashMap<String, InsightTransferProposal>>,
    insight_reports: Mutex<HashMap<String, MetaphoricalInsightReport>>,
    audit_reports: Mutex<HashMap<String, MetaphorAuditReport>>,
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    analogical_reasoning_system: Arc<AnalogicalReasoningSystem>,
    concept_formation_module: Arc<ConceptFormationModule>,
    abstract_reasoning_module: Arc<AbstractReasoningModule>,
}

impl MetaphoricalMappingModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, analogical_reasoning_system: Arc<AnalogicalReasoningSystem>, concept_formation_module: Arc<ConceptFormationModule>, abstract_reasoning_module: Arc<AbstractReasoningModule>) -> Self {
        info!("Cognitive Arch: Metaphorical Mapping Module initialized.");
        MetaphoricalMappingModule {
            metaphorical_bridges: Mutex::new(HashMap::new()),
            insight_proposals: Mutex::new(HashMap::new()),
            insight_reports: Mutex::new(HashMap::new()),
            audit_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            analogical_reasoning_system,
            concept_formation_module,
            abstract_reasoning_module,
        }
    }

    /// Discovers structural isomorphisms between unrelated domains to create metaphorical bridges.
    pub async fn discover_metaphorical_bridges(&self, domain_a: &str, domain_b: &str) -> Result<MetaphoricalBridge, String> {
        info!("MMP: Discovering metaphorical bridges between '{}' and '{}'.".to_string(), domain_a, domain_b);

        // Use AnalogicalReasoningSystem to find base structural matches (Sub-Task 160.1)
        let structural_match = self.analogical_reasoning_system.solve_analogical_problem(
            &format!("Find structural isomorphism between {} and {}.", domain_a, domain_b),
            "cross_domain_structural_analysis"
        ).await.map_err(|e| e.to_string())?;

        // Use AbstractReasoningModule to refine the metaphorical relationship (Sub-Task 160.1)
        let refined_relation = self.abstract_reasoning_module.derive_abstract_principles(
            &format!("Metaphorical link: {}.", structural_match.mapped_solution),
            "metaphorical_abstraction"
        ).await.map_err(|e| e.to_string())?;

        let bridge_id = format!("bridge-{}-{}", domain_a, Uuid::new_v4());
        let bridge = MetaphoricalBridge {
            id: bridge_id.clone(),
            source_domain: domain_a.to_string(),
            target_domain: domain_b.to_string(),
            source_concept: "abstract_structure_a".to_string(), // Placeholder
            target_concept: "abstract_structure_b".to_string(), // Placeholder
            metaphorical_relation: refined_relation.principles.first().cloned().unwrap_or("is_conceptually_linked".to_string()),
            mapping_strength: 0.85,
            ethical_connotation: vec![],
            created_at: Utc::now(),
        };

        self.metaphorical_bridges.lock().unwrap().insert(bridge_id.clone(), bridge.clone());
        info!("MMP: Metaphorical bridge discovered: {}.", bridge_id);
        self.event_bus.publish(EventEnvelope::new("MetaphoricalMappingModule".to_string(), OsEvent::Custom("MetaphoricalBridgeDiscovered".to_string(), bridge_id))).await;
        Ok(bridge)
    }

    /// Transfers an insight from one domain to another using an established metaphorical bridge.
    pub async fn propose_insight_transfer(&self, bridge_id: &str, source_insight: &str) -> Result<InsightTransferProposal, String> {
        info!("MMP: Proposing insight transfer for bridge '{}'.".to_string(), bridge_id);
        
        let bridges_guard = self.metaphorical_bridges.lock().unwrap();
        let bridge = bridges_guard.get(bridge_id).cloned().ok_or(format!("Metaphorical bridge {} not found.", bridge_id))?;
        drop(bridges_guard);

        // Use MetaLearner to optimize the transfer strategy (Sub-Task 160.1)
        let _transfer_strategy = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("insight_transfer_problem_{}", Uuid::new_v4()),
            name: "Metaphorical Insight Transfer Optimization".to_string(),
            domain: "creative_problem_solving".to_string(),
            problem_type: "domain_adaptation".to_string(),
            complexity_score: bridge.mapping_strength,
            data_characteristics: HashMap::new(),
            required_accuracy: 0.9,
            created_at: Utc::now(),
        }).await.ok();

        let proposal_id = format!("ins-prop-{}", Uuid::new_v4());
        let mut proposal = InsightTransferProposal {
            id: proposal_id.clone(),
            bridge_id: bridge_id.to_string(),
            source_insight: source_insight.to_string(),
            proposed_target_application: format!("Applying structure of '{}' to solve problem in '{}'.", bridge.source_concept, bridge.target_domain),
            expected_benefit: "Novel solution inspired by metaphorical isomorphism.".to_string(),
            ethical_review_status: VerificationStatus::Pending,
            security_verified: VerificationStatus::Pending,
            proposed_at: Utc::now(),
        };

        // Ethical review of the transfer (Sub-Task 160.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            proposal.id.clone(),
            "InsightTransferProposal".to_string(),
            format!("Ethical review of insight transfer via metaphor '{}'.", bridge.metaphorical_relation)
        ).await.map_err(|e| e.to_string())?;
        proposal.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Transfer ethically sound.") } else { VerificationStatus::Failed(format!("Ethical issues: {:?}", aleca_audit.violation_details)) };

        if matches!(proposal.ethical_review_status, VerificationStatus::Failed(_)) {
            error!("MMP: Insight transfer proposal '{}' failed ethical review.", proposal.id);
            return Err(format!("Insight transfer proposal {} failed ethical review.", proposal.id));
        }

        self.insight_proposals.lock().unwrap().insert(proposal_id.clone(), proposal.clone());
        info!("MMP: Insight transfer proposal generated: {}.", proposal_id);
        self.event_bus.publish(EventEnvelope::new("MetaphoricalMappingModule".to_string(), OsEvent::Custom("InsightTransferProposed".to_string(), proposal_id))).await;
        Ok(proposal)
    }

    pub fn as_os_service() -> OsService {
        OsService {
            id: "metaphorical-mapping-module".to_string(),
            name: "Metaphorical Mapping Module".to_string(),
            description: "Enables discovery of structural isomorphisms and insight transfer between unrelated domains.".to_string(),
            service_type: ServiceType::Cognitive,
            capabilities: vec![
                ServiceCapability { name: "discover_metaphorical_bridges".to_string(), description: "Discovers structural similarities between disparate domains, creating metaphorical bridges for knowledge transfer.".to_string(), parameters: HashMap::new(), output_schema: "MetaphoricalBridge".to_string(), requires_permissions: vec!["perform_analogical_reasoning".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "propose_insight_transfer".to_string(), description: "Transfers an insight from a source domain to a target domain using an established metaphorical bridge, with ethical verification.".to_string(), parameters: HashMap::new(), output_schema: "InsightTransferProposal".to_string(), requires_permissions: vec!["generate_innovative_solutions".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "analogical-reasoning-system".to_string(), "concept-formation-module".to_string(), "abstract-reasoning-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
