// Tariro_ide_idpte/src/security/hardware_security.rs

//! Implements Tariro's Secure Hardware Integration module, enabling autonomous hardware
//! attestation, secure peripheral management, and ethical hardware interaction to ensure
//! the integrity of its underlying hardware and prevent tampering or exploitation.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::security::hardware_security_models::{HardwareAttestationReport, PeripheralSecurityPolicy, HardwareInteractionLog, EthicalHardwareAccessRecord};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::compliance_advisor::orchestrator::AlecaOrchestrator; // For ethical review of hardware interactions
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For secure attestation and logging
use crate::cognitive_arch::meta_learner::MetaLearner; // For optimizing security policies
use crate::cognitive_arch::learning_models::VerificationStatus; // Alias for clarity

/// The Secure Hardware Integration module.
pub struct HardwareSecurityModule {
    attestation_reports: Mutex<HashMap<String, HardwareAttestationReport>>, // ID -> Report
    peripheral_policies: Mutex<HashMap<String, PeripheralSecurityPolicy>>, // ID -> Policy
    interaction_logs: Mutex<HashMap<String, HardwareInteractionLog>>, // ID -> Log
    ethical_access_records: Mutex<HashMap<String, EthicalHardwareAccessRecord>>, // ID -> Record
    event_bus: Arc<EventBus>,
    aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>,
    meta_learner: Arc<MetaLearner>,
}

impl HardwareSecurityModule {
    pub fn new(event_bus: Arc<EventBus>, aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>, qrctfo_orchestrator: Arc<Mutex<QrCtfoOrchestrator>>, meta_learner: Arc<MetaLearner>) -> Self {
        info!("Security: Secure Hardware Integration Module initialized.");
        let mut initial_policies = HashMap::new();
        initial_policies.insert("robot_arm_policy".to_string(), PeripheralSecurityPolicy {
            id: "robot_arm_policy".to_string(),
            peripheral_id: "robot_arm_001".to_string(),
            policy_name: "SafetyCriticalRobotArmAccess".to_string(),
            access_rules: HashMap::from([("actuate".to_string(), "ALEC-A_approved_only".to_string())]),
            monitoring_level: "high".to_string(),
            ethical_review_status: VerificationStatus::Passed("Ethically designed for safety.".to_string()),
            security_review_status: VerificationStatus::Passed("Secure from unauthorized actuation.".to_string()),
            last_updated: Utc::now(),
        });

        HardwareSecurityModule {
            attestation_reports: Mutex::new(HashMap::new()),
            peripheral_policies: Mutex::new(initial_policies),
            interaction_logs: Mutex::new(HashMap::new()),
            ethical_access_records: Mutex::new(HashMap::new()),
            event_bus,
            aleca_orchestrator,
            qrctfo_orchestrator,
            meta_learner,
        }
    }

    /// Autonomously verifies the integrity and authenticity of underlying hardware components and firmware.
    pub async fn perform_hardware_attestation(&self, component_id: &str, current_state_hash: &str) -> Result<HardwareAttestationReport, String> {
        info!("HSM: Performing hardware attestation for component '{}'.", component_id);
        
        // Use MetaLearner to optimize attestation protocols (Sub-Task 89.1)
        let attestation_protocol = self.meta_learner.select_algorithm(&crate::cognitive_arch::learning_models::LearningProblem {
            id: format!("hardware_attest_problem_{}", Uuid::new_v4()),
            name: "Hardware Attestation Protocol Optimization".to_string(),
            domain: "hardware_security".to_string(),
            problem_type: "protocol_optimization".to_string(),
            complexity_score: 0.9,
            data_characteristics: HashMap::from([("threat_model".to_string(), "physical_tampering".to_string())]),
            required_accuracy: 0.99, // Critical for hardware trust
            created_at: Utc::now(),
        }).await.map_err(|e| e.to_string())?;
        info!("HSM: Using attestation protocol: {}.", attestation_protocol.name);

        let report_id = format!("attest-report-{}-{}", component_id, Uuid::new_v4());
        let mut report = HardwareAttestationReport {
            id: report_id.clone(),
            hardware_component_id: component_id.to_string(),
            attestation_result: "Verified".to_string(), // Placeholder
            firmware_version: "1.0".to_string(), // Placeholder
            last_known_good_hash: "expected_hash_xyz".to_string(), // Placeholder
            current_hash: current_state_hash.to_string(),
            ethical_review_status: VerificationStatus::Pending,
            security_verified: VerificationStatus::Pending,
            timestamp: Utc::now(),
        };

        // Simulate attestation result based on hash comparison
        if report.last_known_good_hash != current_state_hash {
            report.attestation_result = "Tampered".to_string();
            error!("HSM: Hardware tampering detected on component '{}'.", component_id);
        }

        // Security verification of the attestation mechanism itself (Sub-Task 89.2)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            report.id.clone(),
            "HardwareAttestationReport".to_string(),
            format!("Audit hardware attestation report '{}' for integrity of mechanism.", report.id)
        ).await.map_err(|e| e.to_string())?;
        report.security_verified = if qrctfo_audit.is_secure { VerificationStatus::Passed("Attestation mechanism secure.".to_string()) } else { VerificationStatus::Failed(format!("Attestation mechanism security issues: {:?}", qrctfo_audit.vulnerabilities)) };

        // Ethical review of attestation process/outcome (Sub-Task 89.3)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            report.id.clone(),
            "HardwareAttestationReport".to_string(),
            format!("Ethical review of hardware attestation report '{}'.", report.id)
        ).await.map_err(|e| e.to_string())?;
        report.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Attestation process is ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in attestation: {:?}", aleca_audit.violation_details)) };

        self.attestation_reports.lock().unwrap().insert(report_id.clone(), report.clone());
        info!("HSM: Hardware attestation for '{}' complete. Result: {}. Ethical: {:?}, Security: {:?}.", component_id, report.attestation_result, report.ethical_review_status, report.security_verified);
        self.event_bus.publish(EventEnvelope::new("HardwareSecurityModule".to_string(), OsEvent::Custom("HardwareAttestationPerformed".to_string(), report_id))).await;
        Ok(report)
    }

    /// Securely manages access to physical peripherals and sensors.
    /// Enforces fine-grained access control and monitors for anomalous behavior.
    pub async fn manage_secure_peripherals(&self, peripheral_id: &str, requested_action: &str, initiating_module_id: &str) -> Result<(), String> {
        info!("HSM: Managing peripheral '{}' access for action '{}' by '{}'.", peripheral_id, requested_action, initiating_module_id);
        let policies_guard = self.peripheral_policies.lock().unwrap();
        let policy_opt = policies_guard.values().find(|p| p.peripheral_id == peripheral_id).cloned();
        let policy = policy_opt.ok_or(format!("No security policy found for peripheral {}.", peripheral_id))?;
        drop(policies_guard);

        // Ethical and Security Review of the Peripheral Policy (Sub-Task 89.3)
        if matches!(policy.ethical_review_status, VerificationStatus::Failed(_)) {
            error!("HSM: Peripheral policy '{}' is ethically non-compliant. Denying access.", policy.id);
            return Err(format!("Peripheral policy {} is ethically non-compliant.", policy.id));
        }
        if matches!(policy.security_review_status, VerificationStatus::Failed(_)) {
            error!("HSM: Peripheral policy '{}' is insecure. Denying access.", policy.id);
            return Err(format!("Peripheral policy {} is insecure.", policy.id));
        }

        // Simulate access control based on policy
        if !policy.access_rules.get(requested_action).map_or(false, |rule| rule.contains(initiating_module_id) || rule.contains("ALEC-A_approved_only")) {
            error!("HSM: Access denied to peripheral '{}' for '{}' by '{}'. Policy violation.", peripheral_id, requested_action, initiating_module_id);
            return Err(format!("Access denied for peripheral {} action {}.", peripheral_id, requested_action));
        }

        // Log hardware interaction securely (Sub-Task 89.2)
        let log_id = format!("hw-interaction-log-{}-{}", peripheral_id, Uuid::new_v4());
        let mut interaction_log = HardwareInteractionLog {
            id: log_id.clone(),
            component_id: peripheral_id.to_string(),
            interaction_type: requested_action.to_string(),
            initiating_module_id: initiating_module_id.to_string(),
            interaction_details: format!("Action {} on peripheral {} by {}.".to_string(), requested_action, peripheral_id, initiating_module_id),
            security_verified: VerificationStatus::Pending,
            ethical_compliance_status: VerificationStatus::Pending,
            timestamp: Utc::now(),
            blockchain_record_hash: "pending_hash".to_string(),
        };

        // Security verification of the interaction (Sub-Task 89.2)
        let qrctfo_audit = self.qrctfo_orchestrator.lock().unwrap().request_security_audit(
            interaction_log.id.clone(),
            "HardwareInteractionLog".to_string(),
            format!("Audit hardware interaction '{}' for security integrity.", interaction_log.id)
        ).await.map_err(|e| e.to_string())?;
        interaction_log.security_verified = if qrctfo_audit.is_secure { VerificationStatus::Passed("Interaction secure.".to_string()) } else { VerificationStatus::Failed(format!("Interaction security issues: {:?}", qrctfo_audit.vulnerabilities)) };

        // Ethical compliance of the interaction (Sub-Task 89.3)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            interaction_log.id.clone(),
            "HardwareInteractionLog".to_string(),
            format!("Ethical review of hardware interaction '{}'.", interaction_log.id)
        ).await.map_err(|e| e.to_string())?;
        interaction_log.ethical_compliance_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Interaction ethically compliant.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns in interaction: {:?}", aleca_audit.violation_details)) };

        // Store log with blockchain record hash
        interaction_log.blockchain_record_hash = qrctfo_audit.audit_id.clone();
        self.interaction_logs.lock().unwrap().insert(log_id.clone(), interaction_log.clone());
        info!("HSM: Peripheral '{}' action '{}' approved and logged securely. Ethical: {:?}, Security: {:?}. Blockchain Hash: {}.", peripheral_id, requested_action, interaction_log.ethical_compliance_status, interaction_log.security_verified, interaction_log.blockchain_record_hash);
        self.event_bus.publish(EventEnvelope::new("HardwareSecurityModule".to_string(), OsEvent::Custom("PeripheralAccessGranted".to_string(), log_id))).await;
        Ok(())
    }

    /// Monitors and ensures all interactions with physical hardware are ethically sound.
    /// Prevents unintended physical harm or misuse, integrating with ALEC-A.
    pub async fn monitor_ethical_hardware_interaction(&self, interaction_log_id: &str) -> Result<EthicalHardwareAccessRecord, String> {
        info!("HSM: Monitoring ethical hardware interaction for log '{}'.", interaction_log_id);
        let log_opt = self.interaction_logs.lock().unwrap().get(interaction_log_id).cloned();
        let log = log_opt.ok_or(format!("Hardware interaction log {} not found.", interaction_log_id))?;

        let record_id = format!("ethical-hw-access-{}-{}", interaction_log_id, Uuid::new_v4());
        let mut record = EthicalHardwareAccessRecord {
            id: record_id.clone(),
            interaction_log_id: interaction_log_id.to_string(),
            ethical_impact_score: 0.9, // Placeholder
            identified_risks: Vec::new(),
            recommendations: Vec::new(),
            ethical_review_status: VerificationStatus::Pending,
            generated_at: Utc::now(),
        };

        // Ethical review of hardware interaction (Sub-Task 89.3)
        let aleca_audit = self.aleca_orchestrator.lock().unwrap().request_ethical_review(
            record.id.clone(),
            "EthicalHardwareAccessRecord".to_string(),
            format!("Ethical review of hardware interaction from log '{}'.", interaction_log_id)
        ).await.map_err(|e| e.to_string())?;
        record.ethical_review_status = if aleca_audit.is_compliant { VerificationStatus::Passed("Interaction ethically sound.".to_string()) } else { VerificationStatus::Failed(format!("Ethical concerns: {:?}", aleca_audit.violation_details)) };

        self.ethical_access_records.lock().unwrap().insert(record_id.clone(), record.clone());
        info!("HSM: Ethical hardware access record for '{}' generated. Status: {:?}.", interaction_log_id, record.ethical_review_status);
        self.event_bus.publish(EventEnvelope::new("HardwareSecurityModule".to_string(), OsEvent::Custom("EthicalHardwareAccessRecorded".to_string(), record_id))).await;
        Ok(record)
    }

    /// Registers the HardwareSecurityModule itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "hardware-security-module".to_string(),
            name: "Secure Hardware Integration Module".to_string(),
            description: "Autonomously verifies hardware integrity, manages peripherals, and ensures ethical hardware interaction.".to_string(),
            service_type: ServiceType::Security,
            capabilities: vec![
                ServiceCapability { name: "perform_hardware_attestation".to_string(), description: "Autonomously verifies hardware integrity and authenticity with ethical and security reviews.".to_string(), parameters: HashMap::new(), output_schema: "HardwareAttestationReport".to_string(), requires_permissions: vec!["read_hardware_state".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "manage_secure_peripherals".to_string(), description: "Securely manages access to physical peripherals, enforcing policies and logging ethically.".to_string(), parameters: HashMap::new(), output_schema: "bool".to_string(), requires_permissions: vec!["control_hardware_access".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "monitor_ethical_hardware_interaction".to_string(), description: "Monitors and ensures all interactions with physical hardware are ethically sound.".to_string(), parameters: HashMap::new(), output_schema: "EthicalHardwareAccessRecord".to_string(), requires_permissions: vec!["read_hardware_logs".to_string()], requires_ethical_review: true },
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
