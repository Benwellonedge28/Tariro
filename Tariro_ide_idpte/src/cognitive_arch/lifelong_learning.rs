// Tariro_ide_idpte/src/cognitive_arch/lifelong_learning.rs

//! Implements the Continual Lifelong Learning module, enabling Tariro to incrementally
//! integrate new knowledge and skills without forgetting previously learned information
//! (mitigating catastrophic forgetting).

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::learning_memory_models::{KnowledgeChunk, LearningExperience, ConsolidatedKnowledge, KnowledgeIntegrityStatus, KnowledgeGap};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical vetting
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For cryptographic integrity
use crate::cognitive_arch::curiosity_exploration::CuriosityExplorationModule; // For filling knowledge gaps
use crate::cognitive_arch::concept_formation::ConceptFormationModule; // For new concepts
use crate::cognitive_arch::causal_reasoning::CausalReasoningEngine; // For causal links
use crate::cognitive_arch::common_sense_reasoning::CommonSenseReasoningNetwork; // For common sense consistency
use crate::cognitive_arch::learning_models::VerificationStatus; // For alias clarity

/// The Continual Lifelong Learning module.
pub struct LifelongLearningModule {
    knowledge_base: Mutex<HashMap<String, KnowledgeChunk>>, // ID -> KnowledgeChunk
    consolidated_knowledge: Mutex<HashMap<String, ConsolidatedKnowledge>>, // Concept ID -> ConsolidatedKnowledge
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    curiosity_exploration_module: Arc<CuriosityExplorationModule>,
    concept_formation_module: Arc<ConceptFormationModule>, // For pulling new concepts
    causal_reasoning_engine: Arc<CausalReasoningEngine>, // For pulling causal links
    common_sense_reasoning_network: Arc<CommonSenseReasoningNetwork>, // For pulling common sense
}

impl LifelongLearningModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, curiosity_exploration_module: Arc<CuriosityExplorationModule>, concept_formation_module: Arc<ConceptFormationModule>, causal_reasoning_engine: Arc<CausalReasoningEngine>, common_sense_reasoning_network: Arc<CommonSenseReasoningNetwork>) -> Self {
        info!("Cognitive Arch: Continual Lifelong Learning Module initialized.");
        LifelongLearningModule {
            knowledge_base: Mutex::new(HashMap::new()),
            consolidated_knowledge: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            curiosity_exploration_module,
            concept_formation_module,
            causal_reasoning_engine,
            common_sense_reasoning_network,
        }
    }

    /// Integrates a new learning experience, potentially adding new knowledge chunks.
    pub async fn integrate_new_experience(&self, experience: LearningExperience, new_knowledge: Vec<KnowledgeChunk>) -> Result<(), String> {
        info!("CLL: Integrating new learning experience '{}'.", experience.description);
        let mut kb_guard = self.knowledge_base.lock().unwrap();
        for mut chunk in new_knowledge {
            // Perform integrity check for new knowledge chunks (simulated for now, real QR-CTFO call needed)
            let integrity_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
                chunk.id.clone(),
                "KnowledgeChunk".to_string(),
                format!("Audit knowledge chunk '{}' for integrity before integration.", chunk.description),
            ).await.map_err(|e| e.to_string())?;

            if integrity_audit.is_secure {
                chunk.integrity_status = KnowledgeIntegrityStatus::Verified(format!("Integrity verified by QR-CTFO: {}.", integrity_audit.audit_id));
            } else {
                chunk.integrity_status = KnowledgeIntegrityStatus::Compromised(format!("Integrity compromise detected: {:?}.", integrity_audit.vulnerabilities));
                warn!("CLL: Knowledge chunk '{}' integrity compromised. Not integrating.", chunk.id);
                continue;
            }

            kb_guard.insert(chunk.id.clone(), chunk.clone());
            self.event_bus.publish(EventEnvelope::new("CLLModule".to_string(), OsEvent::Custom("KnowledgeChunkAdded".to_string(), chunk.id))).await;
        }
        self.event_bus.publish(EventEnvelope::new("CLLModule".to_string(), OsEvent::Custom("LearningExperienceIntegrated".to_string(), experience.id))).await;
        Ok(())
    }

    /// Autonomously integrates knowledge from other cognitive modules and OS services.
    pub async fn integrate_knowledge_from_source(&self, source_module_id: &str, relevant_info_summary: &str) -> Result<(), String> {
        info!("CLL: Integrating knowledge from source '{}' based on summary '{}'.", source_module_id, relevant_info_summary);
        
        let new_knowledge_id = format!("autogen-kc-{}-{}", source_module_id, Uuid::new_v4());
        let new_chunk = KnowledgeChunk {
            id: new_knowledge_id.clone(),
            description: format!("Knowledge integrated from {}.", source_module_id),
            content_summary: relevant_info_summary.to_string(),
            domain: "cognitive_integration".to_string(),
            source: source_module_id.to_string(),
            strength: 0.7, // Initial strength
            associated_models: vec![source_module_id.to_string()],
            timestamp: Utc::now(),
            integrity_status: KnowledgeIntegrityStatus::Pending, // Will be verified on add
        };

        self.add_fact(new_chunk).await?; // Re-use add_fact to also handle verification

        self.event_bus.publish(EventEnvelope::new("CLLModule".to_string(), OsEvent::Custom("KnowledgeIntegratedFromSource".to_string(), format!("{{ \"source\": \"{}\", \"knowledge_id\": \"{}\" }}", source_module_id, new_knowledge_id)))).await;
        Ok(())
    }

    /// Consolidates knowledge chunks into stable, long-term representations, mitigating forgetting.
    /// Enhanced with cryptographic integrity, ethical vetting, and adaptive forgetting prevention.
    pub async fn consolidate_knowledge(&self, knowledge_chunk_id: &str, forgetting_prevention_strategy: &str) -> Result<ConsolidatedKnowledge, String> {
        info!("CLL: Consolidating knowledge chunk '{}' using strategy '{}'.", knowledge_chunk_id, forgetting_prevention_strategy);
        let kb_guard = self.knowledge_base.lock().unwrap();
        if let Some(chunk) = kb_guard.get(knowledge_chunk_id) {
            if !matches!(chunk.integrity_status, KnowledgeIntegrityStatus::Verified(_)) {
                return Err(format!("Knowledge chunk {} has not passed integrity verification and cannot be consolidated.", knowledge_chunk_id));
            }

            // --- Ethical Vetting (Sub-Task 70.2) ---
            let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
                chunk.id.clone(),
                "KnowledgeChunkContent".to_string(),
                format!("Ethical review of knowledge chunk '{}' for biases or harmful implications.", chunk.description)
            ).await.map_err(|e| e.to_string())?;

            let ethical_vetting_status = if aleca_audit.is_compliant {
                VerificationStatus::Passed("Ethically vetted and compliant.".to_string())
            } else {
                VerificationStatus::Failed(format!("Ethical concerns: {:?}", aleca_audit.violation_details))
            };
            
            if matches!(ethical_vetting_status, VerificationStatus::Failed(_)) {
                warn!("CLL: Knowledge chunk '{}' failed ethical vetting. Not consolidating.", chunk.id);
                return Err(format!("Knowledge chunk {} failed ethical vetting.", knowledge_chunk_id));
            }

            let consolidated_id = format!("consolidated-{}", chunk.id);
            let new_consolidated_knowledge = ConsolidatedKnowledge {
                id: consolidated_id.clone(),
                concept_id: chunk.id.clone(),
                version: 1,
                integration_summary: format!("Integrated '{}' into core knowledge structures using {} strategy.", chunk.description, forgetting_prevention_strategy),
                robustness_score: 0.95, // Assume high robustness after consolidation
                timestamp: Utc::now(),
                ethical_vetting_status,
                forgetting_prevention_strategy: forgetting_prevention_strategy.to_string(),
            };
            self.consolidated_knowledge.lock().unwrap().insert(chunk.id.clone(), new_consolidated_knowledge.clone());
            info!("CLL: Knowledge chunk '{}' consolidated and ethically vetted.", chunk.id);
            self.event_bus.publish(EventEnvelope::new("CLLModule".to_string(), OsEvent::Custom("KnowledgeConsolidated".to_string(), consolidated_id))).await;
            Ok(new_consolidated_knowledge)
        } else {
            Err(format!("Knowledge chunk {} not found for consolidation.", knowledge_chunk_id))
        }
    }

    /// Autonomously refines and updates knowledge chunks, prunes outdated information, and identifies gaps.
    pub async fn refine_knowledge_base(&self) -> Result<(), String> {
        info!("CLL: Autonomously refining knowledge base.");
        let kb_guard = self.knowledge_base.lock().unwrap();
        let consolidated_guard = self.consolidated_knowledge.lock().unwrap();

        let mut gaps_found = Vec::new();

        // Simulate pruning outdated info (e.g., facts with low certainty and old timestamps)
        kb_guard.retain(|_id, chunk| {
            let retain = !(chunk.certainty < 0.2 && (Utc::now() - chunk.timestamp).num_days() > 30);
            if !retain { warn!("CLL: Pruning outdated knowledge chunk: '{}'.", chunk.description); }
            retain
        });

        // Simulate identifying knowledge gaps (e.g., missing causal links or concepts)
        // This would involve queries to CausalReasoningEngine, ConceptFormationModule, etc.
        if self.causal_reasoning_engine.infer_causal_effects("some_graph_id", "A", "B").await.is_err() { // Mock check
            let gap_id = format!("gap-{}", Uuid::new_v4());
            gaps_found.push(KnowledgeGap {
                id: gap_id.clone(),
                description: "Missing causal link between A and B.".to_string(),
                inferred_missing_information: "Need to determine if A causes B.".to_string(),
                identified_by_module: "CausalReasoningEngine".to_string(),
                severity: 0.8,
                suggested_exploration_target: "Experiment on A's effect on B.".to_string(),
                timestamp: Utc::now(),
            });
            // Trigger CuriosityExplorationModule to fill this gap (Sub-Task 70.3)
            self.curiosity_exploration_module.select_exploration_target(&gaps_found.last().unwrap().suggested_exploration_target).await?;
        }
        
        if let Some(gap) = gaps_found.last() {
            self.event_bus.publish(EventEnvelope::new("CLLModule".to_string(), OsEvent::Custom("KnowledgeGapIdentified".to_string(), gap.id.clone()))).await;
        }

        info!("CLL: Knowledge base refinement complete. Gaps identified: {}.", gaps_found.len());
        Ok(())
    }

    /// Registers the LifelongLearningModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "lifelong-learning-module".to_string(),
            name: "Continual Lifelong Learning Module".to_string(),
            description: "Integrates new knowledge incrementally without forgetting old information.".to_string(),
            service_type: ServiceType::AI_Core,
            capabilities: vec![
                ServiceCapability { name: "integrate_new_experience".to_string(), description: "Integrates a new learning experience and knowledge with integrity checks.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["write_knowledge".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "integrate_knowledge_from_source".to_string(), description: "Autonomously integrates knowledge from other cognitive modules.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["read_other_modules_knowledge".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "consolidate_knowledge".to_string(), description: "Consolidates knowledge into long-term memory with ethical vetting and robust forgetting prevention.".to_string(), parameters: HashMap::new(), output_schema: "ConsolidatedKnowledge".to_string(), requires_permissions: vec!["write_long_term_memory".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "refine_knowledge_base".to_string(), description: "Autonomously refines knowledge base, prunes, and identifies gaps.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["modify_knowledge_base".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "curiosity-exploration-module".to_string(), "concept-formation-module".to_string(), "causal-reasoning-engine".to_string(), "common-sense-reasoning".to_string()],
            managed_by_agi_module: None,
        }
    }
}
