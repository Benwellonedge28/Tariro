// Tariro_ide_idpte/src/compliance_advisor/orchestrator.rs

//! Coordinates the Autonomous Legal & Ethical Compliance Advisor (ALEC-A).
//! This orchestrator drives the continuous process of monitoring, analyzing, and enforcing
//! legal and ethical compliance across OMEGA OS operations.

use crate::compliance_advisor::legal_framework_models::{LegalRule, EthicalMandate, ComplianceStatus, AuditReport, ModuleAuditStatus};
use crate::compliance_advisor::regulatory_monitor::RegulatoryMonitor;
use crate::compliance_advisor::compliance_policy_manager::CompliancePolicyManager;
use crate::compliance_advisor::conflict_resolver::{ConflictResolver, ComplianceRemediationAction};
use crate::compliance_advisor::ethical_decision_engine::EthicalDecisionEngine;
use crate::evas_evolution::EVASEngine;
use crate::agi_core::AgiSystemAuditor; // Import the trait

use anyhow::Result;
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use chrono::{Utc, Duration as ChronoDuration};
use tracing::{info, warn, error};
use uuid::Uuid;

// Import other orchestrators for remediation actions
use crate::resource_management::resource_models::ComplianceDirective as SoraeComplianceDirective; // Alias to avoid clash
use crate::resource_management::orchestrator::SORAEOrcestrator; // For SORAEE directives
use crate::crypto_fabric::orchestrator::QrCtfoOrchestrator; // For QR-CTFO directives

/// Configuration options for the ALEC-A Orchestrator.
#[derive(Debug, Clone)]
pub struct AlecaOptions {
    pub regulatory_scan_interval_seconds: u64,
    pub compliance_check_interval_seconds: u64,
    pub self_audit_interval_seconds: u64,
}

impl Default for AlecaOptions {
    fn default() -> Self {
        AlecaOptions {
            regulatory_scan_interval_seconds: 3600, // Scan for new regulations every hour
            compliance_check_interval_seconds: 300, // Check system compliance every 5 minutes
            self_audit_interval_seconds: 86400, // Perform a full self-audit daily
        }
    }
}

/// The main orchestrator for the Autonomous Legal & Ethical Compliance Advisor.
pub struct AlecaOrchestrator {
    regulatory_monitor: RegulatoryMonitor,
    policy_manager: Arc<Mutex<CompliancePolicyManager>>,
    conflict_resolver: ConflictResolver,
    ethical_decision_engine: Arc<Mutex<EthicalDecisionEngine>>, 
    evas_engine: Arc<Mutex<EVASEngine>>, 
    // Senders to other orchestrators for autonomous remediation actions
    sorae_directive_sender: Option<mpsc::Sender<SoraeComplianceDirective>>,
    qrctfo_directive_sender: Option<mpsc::Sender<ComplianceRemediationAction>>,
    current_options: Arc<Mutex<AlecaOptions>>,
}

impl AlecaOrchestrator {
    pub fn new(
        options: AlecaOptions,
        sorae_sender: Option<mpsc::Sender<SoraeComplianceDirective>>,
        qrctfo_sender: Option<mpsc::Sender<ComplianceRemediationAction>>,
        // The AgiSystemAuditor will be set after AgiCoreEngine is constructed
    ) -> Self {
        let evas_engine_arc = Arc::new(Mutex::new(EVASEngine::new()));
        let ethical_engine_arc = Arc::new(Mutex::new(EthicalDecisionEngine::new(evas_engine_arc.clone())));
        AlecaOrchestrator {
            regulatory_monitor: RegulatoryMonitor::new(evas_engine_arc.clone()),
            policy_manager: Arc::new(Mutex::new(CompliancePolicyManager::new(ethical_engine_arc.clone()))),
            conflict_resolver: ConflictResolver::new(),
            ethical_decision_engine: ethical_engine_arc,
            evas_engine: evas_engine_arc,
            sorae_directive_sender: sorae_sender,
            qrctfo_directive_sender: qrctfo_sender,
            current_options: Arc::new(Mutex::new(options)),
        }
    }

    /// Starts the main ALEC-A orchestration loop.
    pub async fn start_loop(&mut self, agi_system_auditor: Arc<impl AgiSystemAuditor + 'static>) -> Result<()> {
        let options_arc = self.current_options.clone();
        let policy_manager_arc = self.policy_manager.clone();
        let conflict_resolver_arc = Arc::new(self.conflict_resolver.clone()); 
        let sorae_directive_sender_clone = self.sorae_directive_sender.clone();
        let qrctfo_directive_sender_clone = self.qrctfo_directive_sender.clone();
        let agi_system_auditor_clone = agi_system_auditor.clone();

        info!("ALEC-A Orchestrator started.");

        // Initial regulatory scan to load current rules
        let (initial_legal_rules, initial_ethical_mandates) = self.regulatory_monitor.scan_for_updates().await?;
        let mut policy_manager = policy_manager_arc.lock().unwrap();
        policy_manager.update_legal_rules(initial_legal_rules).await;
        policy_manager.update_ethical_mandates(initial_ethical_mandates).await;
        drop(policy_manager);

        // Main loop for continuous compliance monitoring
        let mut last_regulatory_scan = Utc::now();
        let mut last_compliance_check = Utc::now();
        let mut last_self_audit = Utc::now();

        loop {
            let options = options_arc.lock().unwrap().clone();
            let now = Utc::now();

            // 1. Regulatory Scan
            if now.signed_duration_since(last_regulatory_scan).num_seconds() as u64 >= options.regulatory_scan_interval_seconds {
                info!("ALEC-A: Initiating regulatory scan for updates.");
                let (new_legal_rules, new_ethical_mandates) = self.regulatory_monitor.scan_for_updates().await?;
                let mut policy_manager = policy_manager_arc.lock().unwrap();
                policy_manager.update_legal_rules(new_legal_rules).await;
                policy_manager.update_ethical_mandates(new_ethical_mandates).await;
                last_regulatory_scan = now;
            }

            // 2. Continuous Compliance Check (e.g., for system operations, data flows)
            if now.signed_duration_since(last_compliance_check).num_seconds() as u64 >= options.compliance_check_interval_seconds {
                info!("ALEC-A: Initiating continuous system compliance check.");
                let policy_manager = policy_manager_arc.lock().unwrap();
                // Simulate checking a critical AGI operation
                let agi_operation_status = policy_manager.check_compliance(
                    "agi-core-semantic-search-op-1",
                    "AGI_Operation",
                    "Performing semantic search, accessing knowledge base.",
                    "agi_core",
                    &["data_access", "intellectual_property", "user_data"].into_iter().map(String::from).collect(),
                ).await?;

                if !agi_operation_status.is_compliant {
                    warn!("ALEC-A: Compliance violations detected for AGI operation {}. Resolving...", agi_operation_status.entity_id);
                    let (human_recommendations, system_actions) = conflict_resolver_arc.suggest_resolution(&agi_operation_status).await?;

                    for res in human_recommendations {
                        warn!("Human Recommendation: {}", res);
                    }
                    for action in system_actions {
                        warn!("ALEC-A: Attempting autonomous remediation: {:?}", action);
                        self.dispatch_remediation_action(action, 
                            sorae_directive_sender_clone.clone(), 
                            qrctfo_directive_directive_sender_clone.clone()).await;
                    }

                    // Also check for conflicts proactively when non-compliant
                    let conflicting_policies = policy_manager.identify_conflicts("data_processing", &["maximize_data_sharing".to_string()]).await;
                    if !conflicting_policies.is_empty() {
                        let mediation = conflict_resolver_arc.mediate_conflicting_policies(
                            "data_processing_context", 
                            &conflicting_policies, 
                            &["maximize_data_sharing".to_string()]).await?;
                        warn!("ALEC-A: Mediated conflict: {}", mediation);
                    }

                } else {
                    info!("ALEC-A: AGI operation {} is compliant.", agi_operation_status.entity_id);
                }
                last_compliance_check = now;
            }

            // 3. Periodic Full Self-Audit (more comprehensive check)
            if now.signed_duration_since(last_self_audit).num_seconds() as u64 >= options.self_audit_interval_seconds {
                info!("ALEC-A: Initiating full system self-audit for compliance.");
                self.perform_self_audit(agi_system_auditor_clone.clone()).await?;
                last_self_audit = now;
            }

            tokio::time::sleep(Duration::from_secs(1)).await; // Small sleep to avoid busy-waiting
        }
    }

    /// Dispatches a ComplianceRemediationAction to the appropriate orchestrator.
    async fn dispatch_remediation_action(
        &self,
        action: ComplianceRemediationAction,
        sorae_sender: Option<mpsc::Sender<SoraeComplianceDirective>>,
        qrctfo_sender: Option<mpsc::Sender<ComplianceRemediationAction>>,
    ) {
        match action {
            ComplianceRemediationAction::SuspendAgiOperation(entity_id) => {
                if let Some(sender) = sorae_sender {
                    let _ = sender.send(SoraeComplianceDirective::Custom("suspend_agi_op".to_string(), entity_id)).await;
                }
            },
            ComplianceRemediationAction::AdjustResourceAllocation(entity_id, adjustment) => {
                if let Some(sender) = sorae_sender {
                    let _ = sender.send(SoraeComplianceDirective::Custom("adjust_resource_alloc".to_string(), format!("{}:{}", entity_id, adjustment))).await;
                }
            },
            ComplianceRemediationAction::ForcePqcAlgorithmSwitch(channel_id, new_alg_type_str) => {
                if let Some(sender) = qrctfo_sender {
                    let _ = sender.send(ComplianceRemediationAction::ForcePqcAlgorithmSwitch(channel_id, new_alg_type_str)).await;
                }
            },
            ComplianceRemediationAction::ForceKeyRotation(key_usage_type_str) => {
                if let Some(sender) = qrctfo_sender {
                    let _ = sender.send(ComplianceRemediationAction::ForceKeyRotation(key_usage_type_str)).await;
                }
            },
            ComplianceRemediationAction::LogAndFlagForReview(entity_id) => {
                warn!("ALEC-A: Action for human review: {}.", entity_id);
                // Log to an audit trail and notify human administrator
            },
            ComplianceRemediationAction::Custom(action_type, value) => {
                warn!("ALEC-A: Custom remediation action triggered: {} -> {}. Needs manual dispatch or specific integration.", action_type, value);
            },
        }
    }

    /// Performs a comprehensive self-audit of all integrated AGI components.
    async fn perform_self_audit(&self, agi_system_auditor: Arc<impl AgiSystemAuditor + 'static>) -> Result<AuditReport> {
        info!("ALEC-A: Performing full system self-audit...");
        let mut module_statuses = Vec::new();
        let mut overall_compliant = true;
        let mut critical_issues = Vec::new();
        let mut overall_score = 1.0; // Assume perfect initially

        // Query audit status from each major AGI module via AgiSystemAuditor trait
        let sorae_status = agi_system_auditor.get_sorae_audit_status().await;
        module_statuses.push(sorae_status.clone());
        if !sorae_status.compliant { overall_compliant = false; overall_score -= 0.2; critical_issues.extend(sorae_status.issues_found); }

        let qrctfo_status = agi_system_auditor.get_qrctfo_audit_status().await;
        module_statuses.push(qrctfo_status.clone());
        if !qrctfo_status.compliant { overall_compliant = false; overall_score -= 0.3; critical_issues.extend(qrctfo_status.issues_found); }

        let hacse_status = agi_system_auditor.get_hacse_audit_status().await;
        module_statuses.push(hacse_status.clone());
        if !hacse_status.compliant { overall_compliant = false; overall_score -= 0.1; critical_issues.extend(hacse_status.issues_found); }

        // Calculate overall score and summary
        let summary = if overall_compliant {
            "All audited modules are compliant and operating within ethical and legal boundaries.".to_string()
        } else {
            format!("Non-compliance issues detected in {} modules. Critical issues: {:?}", 
                    module_statuses.iter().filter(|s| !s.compliant).count(), critical_issues)
        };

        let recommended_system_adjustments = if !overall_compliant {
            vec!["Prioritize addressing critical compliance issues.".to_string(), "Review and update relevant policies.".to_string()]
        } else {
            vec!["Maintain vigilance and continue proactive monitoring.".to_string()]
        };

        let audit_report = AuditReport {
            report_id: format!("audit-{}", Uuid::new_v4()),
            timestamp: Utc::now(),
            overall_compliant,
            overall_score: overall_score.max(0.0),
            summary,
            module_statuses,
            critical_issues,
            recommended_system_adjustments,
        };

        info!("ALEC-A: Full system audit completed. Overall compliant: {}. Report ID: {}.", audit_report.overall_compliant, audit_report.report_id);
        // In a real system, this report would be logged, potentially sent to human oversight,
        // or used to trigger higher-level autonomous adjustments.
        Ok(audit_report)
    }
}
