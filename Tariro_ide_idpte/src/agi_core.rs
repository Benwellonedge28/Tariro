// src/agi_core.rs

use tracing::{info, warn};
use anyhow::Result;
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use serde_json::Value;

// Import S.O.R.A.E. modules
use crate::resource_management::orchestrator::{SORAEOrcestrator, SORAEOptions};
use crate::resource_management::resource_models::{
    ResourceRequest, ComplianceDirective as SoraeComplianceDirective, // Alias to avoid clash
};

// Import QR-CTFO modules
use crate::crypto_fabric::orchestrator::{QrCtfoOrchestrator, QrCtfoOptions};
use crate::crypto_fabric::communication_adapter::CommunicationChannel;

// Import Communication Bridge
use crate::communication_bridge::CommunicationBridge;

// Import HACSE modules
use crate::co_creation::orchestrator::{HacseOrchestrator, HacseOptions};
use crate::co_creation::creative_models::CreativePrompt;
use crate::bridge_protocol::MessageType;

// Import ALEC-A modules
use crate::compliance_advisor::orchestrator::{AlecaOrchestrator, AlecaOptions};
use crate::compliance_advisor::legal_framework_models::{ComplianceStatus, ModuleAuditStatus};
use crate::compliance_advisor::conflict_resolver::ComplianceRemediationAction; // Import for dispatching

/// Trait for auditing different AGI system modules.
/// This provides a standardized way for ALEC-A to query the compliance status of other modules.
pub trait AgiSystemAuditor: Send + Sync {
    async fn get_sorae_audit_status(&self) -> ModuleAuditStatus;
    async fn get_qrctfo_audit_status(&self) -> ModuleAuditStatus;
    async fn get_hacse_audit_status(&self) -> ModuleAuditStatus;
    // Add methods for other critical AGI modules as they are developed.
}

pub struct AgiCoreEngine {
    // S.O.R.A.E. related field
    pub sorae_request_sender: mpsc::Sender<ResourceRequest>,
    sorae_orchestrator_internal: Arc<Mutex<SORAEOrcestrator>>, // Hold internal reference for audit
    // QR-CTFO related field
    pub qr_ctfo_orchestrator: Arc<QrCtfoOrchestrator>,
    // Communication Bridge for IDE
    pub ide_communication_bridge: Arc<CommunicationBridge>,
    // HACSE related field
    pub hacse_prompt_sender: mpsc::Sender<CreativePrompt>,
    hacse_orchestrator_internal: Arc<HacseOrchestrator>, // Hold internal reference for audit
    // ALEC-A related field
    pub aleca_orchestrator: Arc<Mutex<AlecaOrchestrator>>,
    // Senders for compliance directives (to SORAEE and QR-CTFO)
    sorae_compliance_directive_sender: mpsc::Sender<SoraeComplianceDirective>,
    qrctfo_compliance_directive_sender: mpsc::Sender<ComplianceRemediationAction>,
}

impl AgiCoreEngine {
    pub async fn new() -> Result<AgiCoreEngine> {
        info!("Initializing AGI Core Engine...");

        // Initialize S.O.R.A.E. Orchestrator
        let sorae_options = SORAEOptions::default();
        let mut sorae_orchestrator = SORAEOrcestrator::new(sorae_options.clone());
        let sorae_request_sender = sorae_orchestrator.get_request_sender();
        let sorae_compliance_directive_sender = sorae_orchestrator.get_compliance_directive_sender();
        let sorae_orchestrator_arc = Arc::new(Mutex::new(sorae_orchestrator)); // Keep an Arc for audit

        let sorae_orchestrator_clone = sorae_orchestrator_arc.clone();
        tokio::spawn(async move {
            sorae_orchestrator_clone.lock().unwrap().start_loop().await; // Lock before calling start_loop
        });
        info!("S.O.R.A.E. Orchestrator initialized and started.");

        // Initialize QR-CTFO Orchestrator
        let qr_ctfo_options = QrCtfoOptions::default();
        let qr_ctfo_policy_engine_arc = Arc::new(Mutex::new(crate::crypto_fabric::trust_policy_engine::TrustPolicyEngine::new()));
        let qr_ctfo_orchestrator = Arc::new(QrCtfoOrchestrator::new(qr_ctfo_options, qr_ctfo_policy_engine_arc.clone()));

        let qrctfo_compliance_directive_sender = mpsc::channel(50).0; // Temp sender for QR-CTFO until it has its own

        let qr_ctfo_orchestrator_clone = qr_ctfo_orchestrator.clone();
        tokio::spawn(async move {
            if let Err(e) = qr_ctfo_orchestrator_clone.start_loop().await {
                eprintln!("QR-CTFO Orchestrator failed: {:?}", e);
            }
        });
        info!("QR-CTFO Orchestrator initialized and started.");

        // Initialize HACSE Orchestrator
        let hacse_options = HacseOptions::default();
        let hacse_orchestrator = HacseOrchestrator::new(hacse_options);
        let hacse_prompt_sender = hacse_orchestrator.get_prompt_sender();
        let hacse_orchestrator_arc = Arc::new(hacse_orchestrator); // Keep an Arc for audit

        let hacse_orchestrator_clone = hacse_orchestrator_arc.clone();
        tokio::spawn(async move {
            if let Err(e) = hacse_orchestrator_clone.start_loop().await {
                eprintln!("HACSE Orchestrator failed: {:?}", e);
            }
        });
        info!("HACSE Orchestrator initialized and started.");

        // Initialize ALEC-A Orchestrator (requires `self` reference for auditing)
        let aleca_options = AlecaOptions::default();
        let aleca_orchestrator_instance = AlecaOrchestrator::new(aleca_options, 
                                                             Some(sorae_compliance_directive_sender.clone()), 
                                                             Some(qrctfo_compliance_directive_sender.clone()),
                                                             // Temporarily pass a mock auditor or self after construction
                                                             None, // Placeholder for AgiSystemAuditor
                                                        );
        let aleca_orchestrator_arc = Arc::new(Mutex::new(aleca_orchestrator_instance));

        // Initialize Communication Bridge for IDE
        let ide_channel = CommunicationChannel::InterProcess("tariro-ide-bridge".to_string());
        let ide_communication_bridge = Arc::new(CommunicationBridge::new(
            Arc::new(crate::crypto_fabric::communication_adapter::CommunicationAdapter::new(qr_ctfo_policy_engine_arc.clone())), // Pass policy engine
            ide_channel
        ).await?);
        info!("IDE Communication Bridge initialized and secured.");

        let agi_core_engine_instance = AgiCoreEngine {
            sorae_request_sender,
            sorae_orchestrator_internal: sorae_orchestrator_arc,
            qr_ctfo_orchestrator,
            ide_communication_bridge,
            hacse_prompt_sender,
            hacse_orchestrator_internal: hacse_orchestrator_arc,
            aleca_orchestrator: aleca_orchestrator_arc.clone(), // Clone for return
            sorae_compliance_directive_sender: sorae_compliance_directive_sender.clone(),
            qrctfo_compliance_directive_sender: qrctfo_compliance_directive_sender.clone(),
        };
        
        // Now pass self (as an auditor) to ALEC-A. This requires a bit of refactoring for circular dep.
        // For now, let's update the ALEC-A orchestrator to hold the auditor AFTER construction.
        // ALEC-A's new method now takes a concrete implementation of AgiSystemAuditor.
        // This needs to be done AFTER AgiCoreEngine is fully constructed.
        // This pattern needs careful handling for self-references or will be done by passing `Arc<self>`
        // For simplicity, let's keep the auditor field in ALEC-A Orchestrator as None initially and update later
        // Or, better, just pass relevant *status functions* from each module to ALEC-A.

        let agi_auditor_arc = Arc::new(agi_core_engine_instance);

        let aleca_orchestrator_clone = aleca_orchestrator_arc.clone();
        let agi_auditor_for_aleca = agi_auditor_arc.clone();
        tokio::spawn(async move {
            if let Err(e) = aleca_orchestrator_clone.lock().unwrap().start_loop(agi_auditor_for_aleca).await {
                eprintln!("ALEC-A Orchestrator failed: {:?}", e);
            }
        });

        Ok(agi_auditor_arc)
    }
}

#[async_trait::async_trait]
impl AgiSystemAuditor for AgiCoreEngine {
    async fn get_sorae_audit_status(&self) -> ModuleAuditStatus {
        info!("AgiSystemAuditor: Auditing S.O.R.A.E.");
        let sorae = self.sorae_orchestrator_internal.lock().unwrap();
        // Here, SORAEE would provide its internal health, compliance status, etc.
        ModuleAuditStatus {
            module_name: "SORAEE".to_string(),
            compliant: true,
            issues_found: vec![],
            recommendations: vec!["Optimize energy usage by 10%.".to_string()],
            last_audited: chrono::Utc::now(),
        }
    }

    async fn get_qrctfo_audit_status(&self) -> ModuleAuditStatus {
        info!("AgiSystemAuditor: Auditing QR-CTFO.");
        // Here, QR-CTFO would report key expiration, policy compliance, algorithm robustness.
        ModuleAuditStatus {
            module_name: "QR-CTFO".to_string(),
            compliant: true,
            issues_found: vec![],
            recommendations: vec!["Rotate root keys bi-annually.".to_string()],
            last_audited: chrono::Utc::now(),
        }
    }

    async fn get_hacse_audit_status(&self) -> ModuleAuditStatus {
        info!("AgiSystemAuditor: Auditing HACSE.");
        // HACSE would report ethical alignment of generated content, fairness metrics, user feedback integration.
        ModuleAuditStatus {
            module_name: "HACSE".to_string(),
            compliant: false,
            issues_found: vec!["Stylistic drift detected from user preferences.".to_string()],
            recommendations: vec!["Review human style learner parameters.".to_string()],
            last_audited: chrono::Utc::now(),
        }
    }
}
