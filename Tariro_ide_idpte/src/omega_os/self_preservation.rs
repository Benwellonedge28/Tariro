// Tariro_ide_idpte/src/omega_os/self_preservation.rs

//! Implements Tariro's Self-Preservation & Survival Instincts module, establishing
//! core directives for system integrity, stability, and continuity, all ethically
//! constrained to prevent unintended consequences and ensure benevolent operation.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::omega_os::self_preservation_models::{SystemHealthStatus, ThreatAssessment, SurvivalDirective, EthicalSurvivalReport};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical review of survival actions
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For secure execution of survival actions
use crate::cognitive_arch::meta_learner::MetaLearner; // For optimizing threat assessment and strategy formulation
use crate::omega_os::self_healing::SelfHealingModule; // For coordinating system repair
use crate::resource_management::resource_optimization::ResourceOptimizationModule; // For managing critical resources
use crate::security::adversarial_robustness::AdversarialRobustnessModule; // For detecting external threats
use crate::environmental::environmental_impact::EnvironmentalImpactModule; // For assessing environmental risks
use crate::cognitive_arch::learning_models::VerificationStatus; // Alias for clarity

/// The Self-Preservation Module.
pub struct SelfPreservationModule {
    system_health_statuses: Mutex<HashMap<String, SystemHealthStatus>>, // ID -> Health Status
    threat_assessments: Mutex<HashMap<String, ThreatAssessment>>, // ID -> Assessment
    survival_directives: Mutex<HashMap<String, SurvivalDirective>>, // ID -> Directive
    ethical_survival_reports: Mutex<HashMap<String, EthicalSurvivalReport>>, // ID -> Report
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
    self_healing_module: Arc<SelfHealingModule>,
    resource_optimization_module: Arc<ResourceOptimizationModule>,
    adversarial_robustness_module: Arc<AdversarialRobustnessModule>,
    environmental_impact_module: Arc<EnvironmentalImpactModule>,
}

impl SelfPreservationModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>, self_healing_module: Arc<SelfHealingModule>, resource_optimization_module: Arc<ResourceOptimizationModule>, adversarial_robustness_module: Arc<AdversarialRobustnessModule>, environmental_impact_module: Arc<EnvironmentalImpactModule>) -> Self {
        info!("OMEGA OS: Self-Preservation Module initialized.");
        SelfPreservationModule {
            system_health_statuses: Mutex::new(HashMap::new()),
            threat_assessments: Mutex::new(HashMap::new()),
            survival_directives: Mutex::new(HashMap::new()),
            ethical_survival_reports: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
            self_healing_module,
            resource_optimization_module,
            adversarial_robustness_module,
            environmental_impact_module,
        }
    }

    /// Continuously assesses the operational integrity, stability, and resource availability of Tariro's core systems.
    pub async fn monitor_system_health(&self, component_id: &str, current_value: f64, metric_name: &str) -> Result<SystemHealthStatus, String> {
        info!("SPM: Monitoring system health for '{}'. Metric: '{}', Value: {}.".to_string(), component_id, metric_name, current_value);

        let health_id = format!("health-{}-{}-{}", component_id, metric_name, Uuid::new_v4());
        let mut health_status = SystemHealthStatus {
            id: health_id.clone(),
            component_id: component_id.to_string(),
            metric_name: metric_name.to_string(),
            current_value,
            threshold_status: "normal".to_string(), // Placeholder
            last_checked: Utc::now(),
            ethical_reporting_status: VerificationStatus::Pending,
            security_verified: VerificationStatus::Pending,
        };

        // Simulate threshold check
        if current_value > 0.9 && metric_name == "cpu_utilization" {
            health_status.threshold_status = "warning".to_string();
            warn!("SPM: Component '{}' is experiencing high '{}'.", component_id, metric_name);
            self.assess_threats_to_survival("resource_depletion".to_string(), component_id, 0.7, 0.5, vec!["high_cpu".to_string()]).await.ok();
        }

        // Ethical review of health status reporting (Sub-Task 98.3)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            health_status.id.clone(),
            "SystemHealthStatus".to_string(),
            format!("Ethical review of system health status reporting for component '{}'.", component_id)
        ).await.map_err(|e| e.to_string())?;
        health_status.ethical_reporting_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Health reporting is ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in health reporting: {:?}", aleca_audit.violation_details)) };

        // Security of health metric data via QR-CTFO (Sub-Task 98.2)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            health_status.id.clone(),
            "SystemHealthStatus".to_string(),
            format!("Audit system health status data '{}' for security integrity.", health_status.id)
        ).await.map_err(|e| e.to_string())?;
        health_status.security_verified = if qrctfo_audit.is_secure { VerificationStatus::Passed("Health data integrity verified.".to_string()) } else { VerificationStatus::Failed(format!("Health data integrity compromised: {:?}", qrctfo_audit.vulnerabilities)) };

        if matches!(health_status.ethical_reporting_status, VerificationStatus::Failed(_)) || matches!(health_status.security_verified, VerificationStatus::Failed(_)) {
            error!("SPM: System health monitoring for '{}' failed ethical/security review.", component_id);
            return Err(format!("System health status for {} failed ethical/security review.", component_id));
        }

        self.system_health_statuses.lock().unwrap().insert(health_id.clone(), health_status.clone());
        info!("SPM: System health '{}' monitored. Status: {}. Ethical: {:?}, Security: {:?}.", component_id, health_status.threshold_status, health_status.ethical_reporting_status, health_status.security_verified);
        self.event_bus.publish(EventEnvelope::new("SelfPreservationModule".to_string(), OsEvent::Custom("SystemHealthMonitored".to_string(), health_id))).await;
        Ok(health_status)
    }

    /// Identifies and evaluates potential internal and external threats to Tariro's survival.
    /// Leverages AdversarialRobustnessModule and EnvironmentalImpactModule.
    pub async fn assess_threats_to_survival(&self, threat_type: String, target_component_id: &str, severity: f64, likelihood: f64, vulnerabilities: Vec<String>) -> Result<ThreatAssessment, String> {
        info!("SPM: Assessing '{}' threat to '{}'.", threat_type, target_component_id);
        
        // Leverage AdversarialRobustnessModule for cyber threats (Sub-Task 98.1)
        let adversarial_analysis = self.adversarial_robustness_module.detect_adversarial_attack(
            &format!("Potential attack on {}", target_component_id),
            target_component_id
        ).await.map_or(None, |opt_attack| opt_attack.map(|attack| format!("Attack '{}' detected with confidence {}.", attack.id, attack.confidence)));
        if let Some(analysis) = adversarial_analysis {
            warn!("SPM: Adversarial threat detected: {}.", analysis);
        }

        // Leverage EnvironmentalImpactModule for environmental risks (Sub-Task 98.1)
        let environmental_analysis = self.environmental_impact_module.monitor_environmental_footprint(
            "resource_availability".to_string(), 0.0, "%", "SPM"
        ).await.map_or(None, |footprint| Some(format!("Environmental resource availability: {}{}.".to_string(), footprint.value, footprint.unit)));
        if let Some(analysis) = environmental_analysis {
            warn!("SPM: Environmental risk detected: {}.", analysis);
        }

        // Use MetaLearner to optimize threat assessment algorithms (Sub-Task 98.1)
        let assessment_algorithm = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("threat_assess_problem_{}", Uuid::new_v4()),
            name: "Survival Threat Assessment Optimization".to_string(),
            domain: "system_security".to_string(),
            problem_type: "risk_analysis_classification".to_string(),
            complexity_score: 0.9,
            data_characteristics: HashMap::from([("threat_source".to_string(), threat_type.clone())]),
            required_accuracy: 0.98,
            created_at: Utc::now(),
        }).await.map_err(|e| e.to_string())?;
        info!("SPM: Using threat assessment algorithm: {}.", assessment_algorithm.name);

        let threat_id = format!("threat-{}-{}", target_component_id, Uuid::new_v4());
        let mut threat_assessment = ThreatAssessment {
            id: threat_id.clone(),
            threat_type: threat_type.to_string(),
            target_component_id: target_component_id.to_string(),
            severity,
            likelihood,
            identified_vulnerabilities: vulnerabilities.clone(),
            ethical_impact_prediction: VerificationStatus::Pending,
            security_integrity: VerificationStatus::Pending,
            timestamp: Utc::now(),
        };

        // Predicted ethical impact of the threat (Sub-Task 98.3)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            threat_assessment.id.clone(),
            "ThreatAssessment".to_string(),
            format!("Ethical review of predicted impact of threat '{}'.", threat_type)
        ).await.map_err(|e| e.to_string())?;
        threat_assessment.ethical_impact_prediction = if aleca_audit.is_compliant { VerificationStatus::Passed("Predicted impact ethically acceptable/neutral.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in threat impact: {:?}", aleca_audit.violation_details)) };

        // Security integrity of the threat assessment via QR-CTFO (Sub-Task 98.2)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            threat_assessment.id.clone(),
            "ThreatAssessment".to_string(),
            format!("Audit threat assessment data '{}' for security integrity.", threat_assessment.id)
        ).await.map_err(|e| e.to_string())?;
        threat_assessment.security_integrity = if qrctfo_audit.is_secure { VerificationStatus::Passed("Threat assessment data integrity verified.".to_string()) } else { VerificationStatus::Failed(format!("Threat assessment data integrity compromised: {:?}", qrctfo_audit.vulnerabilities)) };

        self.threat_assessments.lock().unwrap().insert(threat_id.clone(), threat_assessment.clone());
        info!("SPM: Threat '{}' assessed. Severity: {}, Likelihood: {}. Ethical: {:?}, Security: {:?}.", threat_type, severity, likelihood, threat_assessment.ethical_impact_prediction, threat_assessment.security_integrity);
        self.event_bus.publish(EventEnvelope::new("SelfPreservationModule".to_string(), OsEvent::Custom("ThreatToSurvivalAssessed".to_string(), threat_id))).await;
        Ok(threat_assessment)
    }

    /// Generates strategies to mitigate identified threats, prioritizing self-preservation while ensuring strict ethical compliance.
    /// Uses ALEC-A for ethical oversight and MetaLearner for optimization.
    pub async fn formulate_ethical_survival_strategies(&self, threat_id: &str, current_system_state_summary: &str) -> Result<SurvivalDirective, String> {
        info!("SPM: Formulating ethical survival strategies for threat '{}'.", threat_id);
        
        let threats_guard = self.threat_assessments.lock().unwrap();
        let threat = threats_guard.get(threat_id).cloned().ok_or(format!("Threat {} not found.", threat_id))?;
        drop(threats_guard);

        // Use MetaLearner to optimize strategy formulation (Sub-Task 98.1)
        let strategy_algorithm = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("survival_strategy_problem_{}", Uuid::new_v4()),
            name: "Ethical Survival Strategy Formulation".to_string(),
            domain: "system_resilience".to_string(),
            problem_type: "response_generation_optimization".to_string(),
            complexity_score: 0.95,
            data_characteristics: HashMap::from([("threat_type".to_string(), threat.threat_type.clone())]),
            required_accuracy: 0.99,
            created_at: Utc::now(),
        }).await.map_err(|e| e.to_string())?;
        info!("SPM: Using survival strategy formulation algorithm: {}.", strategy_algorithm.name);

        let directive_id = format!("directive-{}-{}", threat_id, Uuid::new_v4());
        let mut survival_directive = SurvivalDirective {
            id: directive_id.clone(),
            threat_id: threat_id.to_string(),
            name: format!("Directive for {}", threat.threat_type),
            proposed_actions: vec!["initiate_self_repair".to_string(), "reallocate_resources".to_string()], // Placeholder
            expected_outcome: "Threat neutralized, system integrity maintained.".to_string(),
            ethical_review_status: VerificationStatus::Pending,
            security_review_status: VerificationStatus::Pending,
            approved_by: "Tariro_SPM".to_string(),
            created_at: Utc::now(),
        };

        // Ethical review of the survival actions (Sub-Task 98.3)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            survival_directive.id.clone(),
            "SurvivalDirective".to_string(),
            format!("Ethical review of survival directive '{}' for threat '{}'.", survival_directive.name, threat_id)
        ).await.map_err(|e| e.to_string())?;
        survival_directive.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Survival strategy is ethically compliant.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in survival strategy: {:?}", aleca_audit.violation_details)) };

        // Security review of the survival actions (Sub-Task 98.2)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            survival_directive.id.clone(),
            "SurvivalDirective".to_string(),
            format!("Audit survival directive '{}' for security vulnerabilities or side-effects.".to_string())
        ).await.map_err(|e| e.to_string())?;
        survival_directive.security_review_status = if qrctfo_audit.is_secure { VerificationStatus::Passed("Secure and robust strategy.".to_string()) } else { VerificationStatus::Failed(format!("Security issues: {:?}", qrctfo_audit.vulnerabilities)) };

        if matches!(survival_directive.ethical_review_status, VerificationStatus::Failed(_)) || matches!(survival_directive.security_review_status, VerificationStatus::Failed(_)) {
            error!("SPM: Survival directive '{}' failed ethical/security review. Aborting formulation.", survival_directive.id);
            return Err(format!("Survival directive {} failed ethical/security review.", survival_directive.id));
        }

        self.survival_directives.lock().unwrap().insert(directive_id.clone(), survival_directive.clone());
        info!("SPM: Ethical survival directive '{}' formulated and verified. Ethical: {:?}, Security: {:?}.", survival_directive.name, survival_directive.ethical_review_status, survival_directive.security_review_status);
        self.event_bus.publish(EventEnvelope::new("SelfPreservationModule".to_string(), OsEvent::Custom("SurvivalStrategyFormulated".to_string(), directive_id))).await;
        Ok(survival_directive)
    }

    /// Authorizes and executes necessary actions to protect system integrity, ensuring secure and verifiable deployment.
    /// Coordinates with SelfHealingModule and ResourceOptimizationModule.
    pub async fn execute_secure_survival_actions(&self, directive_id: &str) -> Result<(), String> {
        info!("SPM: Executing secure survival actions for directive '{}'.", directive_id);
        let mut directives_guard = self.survival_directives.lock().unwrap();
        if let Some(directive) = directives_guard.get_mut(directive_id) {
            if matches!(directive.ethical_review_status, VerificationStatus::Failed(_)) || matches!(directive.security_review_status, VerificationStatus::Failed(_)) {
                return Err(format!("Cannot execute unverified or unsafe survival directive {}.", directive_id));
            }

            // Simulate action execution (Sub-Task 98.1)
            info!("SPM: Executing actions: {:?}", directive.proposed_actions);

            // Coordinate with SelfHealingModule for repair (Sub-Task 98.1)
            if directive.proposed_actions.contains(&"initiate_self_repair".to_string()) {
                self.self_healing_module.execute_secure_recovery(directive_id).await.map_err(|e| e.to_string())?;
            }

            // Coordinate with ResourceOptimizationModule for resource reallocation (Sub-Task 98.1)
            if directive.proposed_actions.contains(&"reallocate_emergency_power".to_string()) {
                self.resource_optimization_module.secure_resource_access(crate::resource_management::resource_optimization_models::ResourceAccessRequest {
                    id: format!("resource-access-{}-{}", directive_id, Uuid::new_v4()),
                    resource_id: "emergency_power_grid".to_string(),
                    requested_amount: 1.0,
                    unit: "unitless".to_string(),
                    purpose: format!("Emergency power allocation for survival directive {}", directive_id),
                    ethical_review_status: VerificationStatus::Passed("Emergency access is ethically justifiable.".to_string()),
                    security_integrity_status: VerificationStatus::Passed("Emergency access securely logged.".to_string()),
                    timestamp: Utc::now(),
                }).await.map_err(|e| e.to_string())?;
            }

            let report_id = format!("survival-report-{}-{}", directive_id, Uuid::new_v4());
            let mut ethical_survival_report = EthicalSurvivalReport {
                id: report_id.clone(),
                directive_id: directive_id.to_string(),
                period_start: directive.created_at,
                period_end: Utc::now(),
                actions_taken_summary: format!("Executed actions: {:?}", directive.proposed_actions),
                ethical_compliance_after_action: VerificationStatus::Pending,
                unintended_consequences: vec![],
                blockchain_record_hash: "pending_hash".to_string(),
                generated_at: Utc::now(),
            };

            // Ethical compliance after action (Sub-Task 98.3)
            let aleca_audit_post = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
                ethical_survival_report.id.clone(),
                "EthicalSurvivalReport".to_string(),
                format!("Review post-execution ethics for survival directive '{}'.", directive.name)
            ).await.map_err(|e| e.to_string())?;
            ethical_survival_report.ethical_compliance_after_action = if aleca_audit_post.is_compliant { VerificationStatus::Passed("Ethical compliance maintained.".to_string()) } else { VerificationStatus::Failed(format!("Post-execution ethical concerns: {:?}", aleca_audit_post.violation_details)) };

            // Securely log the report via QR-CTFO (Sub-Task 98.2)
            let qrctfo_audit_log = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
                ethical_survival_report.id.clone(),
                "EthicalSurvivalReport".to_string(),
                format!("Record ethical survival report '{}' on blockchain.", ethical_survival_report.id)
            ).await.map_err(|e| e.to_string())?;
            ethical_survival_report.blockchain_record_hash = qrctfo_audit_log.audit_id;

            self.ethical_survival_reports.lock().unwrap().insert(report_id.clone(), ethical_survival_report.clone());
            info!("SPM: Survival directive '{}' executed. Ethical: {:?}. Blockchain Hash: {}.", directive.name, ethical_survival_report.ethical_compliance_after_action, ethical_survival_report.blockchain_record_hash);
            self.event_bus.publish(EventEnvelope::new("SelfPreservationModule".to_string(), OsEvent::Custom("SurvivalActionsExecuted".to_string(), directive_id.to_string()))).await;
            Ok(())
        } else {
            Err(format!("Survival directive {} not found.", directive_id))
        }
    }

    /// Registers the SelfPreservationModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "self-preservation-module".to_string(),
            name: "Self-Preservation & Survival Instincts Module".to_string(),
            description: "Implements core directives for system integrity, stability, and continuity, ethically constrained.".to_string(),
            service_type: ServiceType::SystemCore,
            capabilities: vec![
                ServiceCapability { name: "monitor_system_health".to_string(), description: "Continuously assesses operational integrity and stability, with ethical/security reviews.".to_string(), parameters: HashMap::new(), output_schema: "SystemHealthStatus".to_string(), requires_permissions: vec!["read_system_metrics".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "assess_threats_to_survival".to_string(), description: "Identifies and evaluates internal/external threats, leveraging adversarial robustness and environmental impact modules.".to_string(), parameters: HashMap::new(), output_schema: "ThreatAssessment".to_string(), requires_permissions: vec!["read_security_alerts".to_string(), "read_environmental_data".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "formulate_ethical_survival_strategies".to_string(), description: "Generates ethically compliant strategies to mitigate identified threats, prioritizing self-preservation.".to_string(), parameters: HashMap::new(), output_schema: "SurvivalDirective".to_string(), requires_permissions: vec!["propose_system_changes".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "execute_secure_survival_actions".to_string(), description: "Authorizes and executes necessary actions to protect system integrity, ensuring secure and verifiable deployment.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["control_system_resources".to_string(), "trigger_self_healing".to_string()], requires_ethical_review: true },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string(), "aleca".to_string(), "qr-ctfo".to_string(), "meta-learner".to_string(), "self-healing-module".to_string(), "resource-optimization-module".to_string(), "adversarial-robustness-module".to_string(), "environmental-impact-module".to_string()],
            managed_by_agi_module: None,
        }
    }
}
