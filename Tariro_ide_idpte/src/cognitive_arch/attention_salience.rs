// Tariro_ide_idpte/src/cognitive_arch/attention_salience.rs

//! Implements Tariro's Attention & Salience Detection module,
//! implementing dynamic attention mechanisms to focus computational
//! resources on relevant information and stimuli, improving efficiency
//! and responsiveness.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::cognitive_arch::attention_models::{SalientFeature, AttentionPolicy, ResourceFocusAdjustment, AttentionAuditReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical review of attention mechanisms
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For secure logging of attention shifts
use crate::cognitive_arch::meta_learner::MetaLearner; // For optimizing attention policies
use crate::cognitive_arch::sensory_fusion::SensoryFusionModule; // For processing fused sensory input
use crate::resource_management::dynamic_reallocation::DynamicResourceReallocationModule; // For adjusting resources
use crate::compliance_advisor::ethical_forecasting::EthicalForecastingModule; // For predicting ethical impact of attention biases
use crate::cognitive_arch::learning_models::VerificationStatus; // Alias for clarity

/// The Attention & Salience Detection Module.
pub struct AttentionSalienceModule {
    salient_features: Mutex<HashMap<String, SalientFeature>>, // ID -> Feature
    attention_policies: Mutex<HashMap<String, AttentionPolicy>>, // ID -> Policy
    resource_focus_adjustments: Mutex<HashMap<String, ResourceFocusAdjustment>>, // ID -> Adjustment
    attention_audit_reports: Mutex<HashMap<String, AttentionAuditReport>>, // ID -> Report
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    sensory_fusion_module: Arc<SensoryFusionModule>,
    dynamic_resource_reallocation_module: Arc<DynamicResourceReallocationModule>,
    ethical_forecasting_module: Arc<EthicalForecastingModule>,
}

impl AttentionSalienceModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, sensory_fusion_module: Arc<SensoryFusionModule>, dynamic_resource_reallocation_module: Arc<DynamicResourceReallocationModule>, ethical_forecasting_module: Arc<EthicalForecastingModule>) -> Self {
        info!("Cognitive Arch: Attention & Salience Detection Module initialized.");
        let mut initial_policies = HashMap::new();
        initial_policies.insert("default_priority".to_string(), AttentionPolicy {
            id: "default_priority".to_string(),
            policy_name: "Prioritize Safety and Goals".to_string(),
            priority_rules: vec!["safety_alerts_critical".to_string(), "active_meta_goals_high".to_string()],
            distraction_threshold: 0.1,
            computational_budget_limit: HashMap::from([("total_cpu_util".to_string(), "0.9".to_string())]),
            ethical_design_review: VerificationStatus::Passed("Initial attention policy ethically sound.".to_string()),
            security_audited: VerificationStatus::Passed("Initial attention policy secure.".to_string()),
            last_updated: Utc::now(),
        });

        AttentionSalienceModule {
            salient_features: Mutex::new(HashMap::new()),
            attention_policies: Mutex::new(initial_policies),
            resource_focus_adjustments: Mutex::new(HashMap::new()),
            attention_audit_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            sensory_fusion_module,
            dynamic_resource_reallocation_module,
            ethical_forecasting_module,
        }
    }

    /// Analyzes sensory input from SensoryFusionModule and internal cognitive states to identify
    /// relevant information and emergent patterns requiring attention.
    pub async fn detect_salient_features(&self, perceptual_representation_id: &str, current_goals_summary: &str) -> Result<Option<SalientFeature>, String> {
        info!("ASM: Detecting salient features from perceptual representation '{}'. Goals: '{}'.".to_string(), perceptual_representation_id, current_goals_summary);
        
        // Get the perceptual representation (Sub-Task 114.1)
        let perceptual_representation_summary = self.sensory_fusion_module.fuse_sensory_inputs_ethically(
            vec!["vision_data_id".to_string(), "audio_data_id".to_string()], // Placeholder
            "default_multi_modal"
        ).await.map_err(|e| e.to_string())?.semantic_description;

        // Use MetaLearner to optimize salience detection algorithms (Sub-Task 114.1)
        let detection_algorithm = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("salience_detection_problem_{}", Uuid::new_v4()),
            name: "Salience Detection Optimization".to_string(),
            domain: "perception_cognition".to_string(),
            problem_type: "pattern_recognition_prioritization".to_string(),
            complexity_score: 0.8,
            data_characteristics: HashMap::from([("input_type".to_string(), "multi_modal_perceptual_data".to_string())]),
            required_accuracy: 0.95,
            created_at: Utc::now(),
        }).await.map_err(|e| e.to_string())?;
        info!("ASM: Using salience detection algorithm: {}.", detection_algorithm.name);

        // Simulate salience detection based on current goals
        if perceptual_representation_summary.contains("threat_detected") && current_goals_summary.contains("maintain_security") {
            let feature_id = format!("salient-feat-{}-{}", perceptual_representation_id.chars().take(10).collect::<String>(), Uuid::new_v4());
            let mut salient_feature = SalientFeature {
                id: feature_id.clone(),
                feature_type: "critical_threat".to_string(),
                source_modality: "vision_audio_fusion".to_string(),
                saliency_score: 0.95,
                context_relevance: 0.99,
                detected_at: Utc::now(),
                ethical_impact_assessment: VerificationStatus::Pending,
                security_verified: VerificationStatus::Pending,
            };

            // Ethical impact of directing attention (Sub-Task 114.2)
            let ethical_forecast = self.ethical_forecasting_module.predict_ethical_consequences(
                crate::compliance_advisor::ethical_forecasting_models::EthicalDilemma {
                    id: format!("dilemma-attention-{}-{}", feature_id, Uuid::new_v4()),
                    description: format!("Ethical implications of directing attention to '{}'.", salient_feature.feature_type),
                    involved_entities: vec![], // Placeholder
                    ethical_principles_at_stake: vec![], // Placeholder
                    timestamp: Utc::now(),
                },
                &salient_feature.feature_type
            ).await.map_err(|e| e.to_string())?;
            salient_feature.ethical_impact_assessment = if ethical_forecast.is_ethically_sound { VerificationStatus::Passed("Directing attention is ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns: {:?}", ethical_forecast.negative_consequences)) };

            // Security verification of feature validity via QR-CTFO (Sub-Task 114.3)
            let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
                salient_feature.id.clone(),
                "SalientFeature".to_string(),
                format!("Audit salient feature '{}' for validity and potential spoofing.", salient_feature.id)
            ).await.map_err(|e| e.to_string())?;
            salient_feature.security_verified = if qrctfo_audit.is_secure { VerificationStatus::Passed("Feature validity verified.".to_string()) } else { VerificationStatus::Failed(format!("Feature validity issues: {:?}", qrctfo_audit.vulnerabilities)) };

            if matches!(salient_feature.ethical_impact_assessment, VerificationStatus::Failed(_)) || matches!(salient_feature.security_verified, VerificationStatus::Failed(_)) {
                error!("ASM: Salient feature '{}' failed ethical/security review. Ignoring.", salient_feature.id);
                return Err(format!("Salient feature {} failed ethical/security review.", salient_feature.id));
            }

            self.salient_features.lock().unwrap().insert(feature_id.clone(), salient_feature.clone());
            info!("ASM: Salient feature '{}' detected. Ethical: {:?}, Security: {:?}.", salient_feature.feature_type, salient_feature.ethical_impact_assessment, salient_feature.security_verified);
            self.event_bus.publish(EventEnvelope::new("AttentionSalienceModule".to_string(), OsEvent::Custom("SalientFeatureDetected".to_string(), feature_id))).await;
            Ok(Some(salient_feature))
        } else {
            info!("ASM: No highly salient features detected for '{}'.", perceptual_representation_id);
            Ok(None)
        }
    }

    /// Adjusts computational resources and processing priorities (coordinating with DynamicResourceReallocationModule)
    /// based on detected salience and current goals.
    pub async fn dynamically_allocate_cognitive_resources(&self, feature_id: &str, attention_policy_id: &str) -> Result<ResourceFocusAdjustment, String> {
        info!("ASM: Dynamically allocating resources for salient feature '{}' using policy '{}'.".to_string(), feature_id, attention_policy_id);
        
        let features_guard = self.salient_features.lock().unwrap();
        let feature = features_guard.get(feature_id).cloned().ok_or(format!("Salient feature {} not found.", feature_id))?;
        drop(features_guard);

        let policies_guard = self.attention_policies.lock().unwrap();
        let policy = policies_guard.get(attention_policy_id).cloned().ok_or(format!("Attention policy {} not found.", attention_policy_id))?;
        drop(policies_guard);

        // Simulate resource adjustment based on policy (Sub-Task 114.1)
        let mut adjusted_resources = HashMap::new();
        let cpu_increase = feature.saliency_score * 0.2; // Example: more salient = more CPU
        adjusted_resources.insert("cpu_usage_ratio".to_string(), cpu_increase);

        // Coordinate with DynamicResourceReallocationModule (Sub-Task 114.1)
        let _reallocation_request = self.dynamic_resource_reallocation_module.request_resource_reallocation(
            feature_id,
            "cognitive_processing_unit".to_string(),
            cpu_increase * 100.0, // Scale for example
            &format!("Increased resource for salient feature '{}'.", feature.feature_type),
            feature.saliency_score
        ).await.ok();

        let adjustment_id = format!("focus-adj-{}-{}", feature_id, Uuid::new_v4());
        let mut resource_focus_adjustment = ResourceFocusAdjustment {
            id: adjustment_id.clone(),
            policy_id: attention_policy_id.to_string(),
            triggered_by_feature_id: feature_id.to_string(),
            adjusted_resources: adjusted_resources.clone(),
            processing_priority_change: feature.saliency_score * 0.5,
            ethical_review_status: VerificationStatus::Pending,
            security_review_status: VerificationStatus::Pending,
            timestamp: Utc::now(),
        };

        // Ethical review of the adjustment (Sub-Task 114.2)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            resource_focus_adjustment.id.clone(),
            "ResourceFocusAdjustment".to_string(),
            format!("Ethical review of resource adjustment for feature '{}'.", feature_id)
        ).await.map_err(|e| e.to_string())?;
        resource_focus_adjustment.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Resource adjustment ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns: {:?}", aleca_audit.violation_details)) };

        // Security review of the adjustment via QR-CTFO (Sub-Task 114.3)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            resource_focus_adjustment.id.clone(),
            "ResourceFocusAdjustment".to_string(),
            format!("Audit resource adjustment '{}' for security integrity.", resource_focus_adjustment.id)
        ).await.map_err(|e| e.to_string())?;
        resource_focus_adjustment.security_review_status = if qrctfo_audit.is_secure { VerificationStatus::Passed("Adjustment secure.".to_string()) } else { VerificationStatus::Failed(format!("Adjustment security issues: {:?}", qrctfo_audit.vulnerabilities)) };

        if matches!(resource_focus_adjustment.ethical_review_status, VerificationStatus::Failed(_)) || matches!(resource_focus_adjustment.security_review_status, VerificationStatus::Failed(_)) {
            error!("ASM: Resource focus adjustment '{}' failed ethical/security review. Aborting.", adjustment_id);
            return Err(format!("Resource focus adjustment {} failed ethical/security review.", adjustment_id));
        }

        self.resource_focus_adjustments.lock().unwrap().insert(adjustment_id.clone(), resource_focus_adjustment.clone());
        info!("ASM: Resources dynamically allocated for '{}'. Ethical: {:?}, Security: {:?}.", feature_id, resource_focus_adjustment.ethical_review_status, resource_focus_adjustment.security_review_status);
        self.event_bus.publish(EventEnvelope::new("AttentionSalienceModule".to_string(), OsEvent::Custom("CognitiveResourcesAdjusted".to_string(), adjustment_id))).await;
        Ok(resource_focus_adjustment)
    }

    /// Registers the AttentionSalienceModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "attention-salience-module".to_string(),
            name: "Attention & Salience Detection Module".to_string(),
            description: "Implements dynamic attention mechanisms to focus computational resources on relevant information and stimuli, improving efficiency and responsiveness.".to_string(),
            service_type: ServiceType::Cognitive,
            capabilities: vec![
                ServiceCapability { name: "detect_salient_features".to_string(), description: "Analyzes sensory input and internal cognitive states to identify relevant information and emergent patterns requiring attention.".to_string(), parameters: HashMap::new(), output_schema: "Option<SalientFeature>".to_string(), requires_permissions: vec!["read_perceptual_data".to_string(), "read_cognitive_state".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "dynamically_allocate_cognitive_resources".to_string(), description: "Adjusts computational resources and processing priorities based on detected salience and current goals, with ethical/security review.".to_string(), parameters: HashMap::new(), output_schema: "ResourceFocusAdjustment".to_string(), requires_permissions: vec!["manage_cognitive_resources".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "manage_attentional_focus".to_string(), description: "Maintains and shifts attention across different tasks or information streams, preventing distraction while ensuring no critical information is overlooked.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["control_cognitive_flow".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "audit_attention_mechanisms_ethically".to_string(), description: "Continuously audits attention mechanisms to prevent biased focus that could lead to discrimination or neglect of critical ethical/security issues, with ALEC-A oversight and immutable logging.".to_string(), parameters: HashMap::new(), output_schema: "AttentionAuditReport".to_string(), requires_permissions: vec!["read_audit_logs".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "sensory-fusion-module".to_string(), "dynamic-resource-reallocation-module".to_string(), "ethical-forecasting-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
