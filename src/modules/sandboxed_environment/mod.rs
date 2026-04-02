// src/modules/sandboxed_environment/mod.rs

//! This module implements the Sandboxed Environment for Tariro's autonomous
//! feature testing and system optimization. It provides a safe, isolated
//! environment for rigorous testing and iterative optimization of new features
//! and architectural changes without impacting the live system.

pub mod virtualization_engine;
pub mod simulation_engine;
pub mod automated_test_suite;
pub mod performance_profiler;
pub mod ethical_compliance_monitor;
pub mod deployment_manager;
pub mod data_mirroring_system;
pub mod code_generation_engine;
pub mod self_healing_engine;
pub mod production_learning_system;
pub mod human_in_the_loop_orchestrator;

use crate::{prelude::*,
    Module,
    ModuleContext,
    FeatureProposalId,
    SandboxedEnvironmentModuleConfig,
    // Import CodeQualityReport directly from its module
    code_generation_engine::CodeQualityReport,
};

/// Represents the core Sandboxed Environment Module.
pub struct SandboxedEnvironmentModule {
    config: SandboxedEnvironmentModuleConfig,
    virtualization_engine: virtualization_engine::VirtualizationEngine,
    simulation_engine: simulation_engine::SimulationEngine,
    automated_test_suite: automated_test_suite::AutomatedTestSuite,
    performance_profiler: performance_profiler::PerformanceProfiler,
    ethical_compliance_monitor: ethical_compliance_monitor::EthicalComplianceMonitor,
    deployment_manager: deployment_manager::DeploymentManager,
    data_mirroring_system: data_mirroring_system::DataMirroringSystem,
    code_generation_engine: code_generation_engine::CodeGenerationEngine,
    self_healing_engine: self_healing_engine::SelfHealingEngine,
    production_learning_system: production_learning_system::ProductionLearningSystem,
    human_in_the_loop_orchestrator: human_in_the_loop_orchestrator::HumanInTheLoopOrchestrator, // New
}

#[async_trait]
pub trait Module: Send + Sync {
    fn new(context: &ModuleContext) -> Result<Self> where Self: Sized;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    fn name(&self) -> &'static str;
}

impl SandboxedEnvironmentModule {
    fn new(context: &ModuleContext) -> Result<Self> {
        let config = context.config.sandboxed_environment_module.clone().unwrap_or_default();
        Ok(Self {
            config,
            virtualization_engine: virtualization_engine::VirtualizationEngine::new(context)?,
            simulation_engine: simulation_engine::SimulationEngine::new(context)?,
            automated_test_suite: automated_test_suite::AutomatedTestSuite::new(context)?,
            performance_profiler: performance_profiler::PerformanceProfiler::new(context)?,
            ethical_compliance_monitor: ethical_compliance_monitor::EthicalComplianceMonitor::new(context)?,
            deployment_manager: deployment_manager::DeploymentManager::new(context)?,
            data_mirroring_system: data_mirroring_system::DataMirroringSystem::new(context)?,
            code_generation_engine: code_generation_engine::CodeGenerationEngine::new(context)?,
            self_healing_engine: self_healing_engine::SelfHealingEngine::new(context)?,
            production_learning_system: production_learning_system::ProductionLearningSystem::new(context)?,
            human_in_the_loop_orchestrator: human_in_the_loop_orchestrator::HumanInTheLoopOrchestrator::new(context)?,
        })
    }

    async fn start(&self) -> Result<()> {
        info!("SandboxedEnvironmentModule starting...");
        self.virtualization_engine.start().await?;
        self.simulation_engine.start().await?;
        self.automated_test_suite.start().await?;
        self.performance_profiler.start().await?;
        self.ethical_compliance_monitor.start().await?;
        self.deployment_manager.start().await?;
        self.data_mirroring_system.start().await?;
        self.code_generation_engine.start().await?;
        self.self_healing_engine.start().await?;
        self.production_learning_system.start().await?;
        self.human_in_the_loop_orchestrator.start().await?;
        info!("SandboxedEnvironmentModule started.");
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        info!("SandboxedEnvironmentModule stopping...");
        self.virtualization_engine.stop().await?;
        self.simulation_engine.stop().await?;
        self.automated_test_suite.stop().await?;
        self.performance_profiler.stop().await?;
        self.ethical_compliance_monitor.stop().await?;
        self.deployment_manager.stop().await?;
        self.data_mirroring_system.stop().await?;
        self.code_generation_engine.stop().await?;
        self.self_healing_engine.stop().await?;
        self.production_learning_system.stop().await?;
        self.human_in_the_loop_orchestrator.stop().await?;
        info!("SandboxedEnvironmentModule stopped.");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "SandboxedEnvironmentModule"
    }
}

impl SandboxedEnvironmentModule {
    // Example of a core method to trigger a sandbox run
    pub async fn run_sandbox_experiment(&self, feature_proposal_id: FeatureProposalId) -> Result<SandboxReport> {
        info!("Initiating autonomous sandbox experiment for FeatureProposal: {}", feature_proposal_id.0);

        let telemetry_config = self.context.config.telemetry.clone().unwrap_or_default();
        let sandboxed_env_config = self.context.config.sandboxed_environment_module.clone().unwrap_or_default();
        let fault_injection_config = self.context.config.fault_injection.clone().unwrap_or_default();
        let simulation_engine_config = self.context.config.simulation_engine.clone().unwrap_or_default();
        let production_learning_config = self.context.config.production_learning.clone().unwrap_or_default();
        let human_in_the_loop_config = self.context.config.human_in_the_loop.clone().unwrap_or_default(); // New

        let mut report = SandboxReport {
            sandbox_id: "".to_string(),
            feature_proposal_id: feature_proposal_id.clone(),
            test_results: "N/A".to_string(),
            initial_performance: "N/A".to_string(),
            optimized_config: "N/A".to_string(),
            ethical_violations: "N/A".to_string(),
            deployment_status: "Not Attempted".to_string(),
            status: SandboxStatus::Initiated,
            recommendation: "Pending".to_string(),
            sim_report_details: None,
            generated_code_feedback: Vec::new(),
            fault_injection_results: None,
            external_mock_interaction_issues: None,
            telemetry_enabled_for_experiment: telemetry_config.enable_telemetry_export,
            self_healing_attempts: Vec::new(),
            production_learning_insights: None,
            human_review_requested: false, // New
            proactive_suggestions_made: Vec::new(), // New
        };

        let sandbox_id: String;
        let mut human_review_needed_total = false;
        let mut deployment_approved = false;
        let mut current_code_context = "Initial feature code for ".to_string() + &feature_proposal_id.0;
        
        // --- Phase 0.5: Dynamic Learning from Production ---
        if production_learning_config.enable_production_data_ingestion {
            report.status = SandboxStatus::LearningFromProduction;
            info!("Phase 0.5: Learning from production data...");
            let telemetry_data = self.production_learning_system.ingest_production_telemetry().await?;
            let log_data = self.production_learning_system.ingest_production_logs().await?;
            let insights = self.production_learning_system.analyze_production_feedback(&telemetry_data, &log_data).await?;
            report.production_learning_insights = Some(insights.insights.join("; "));
            info!("Derived simulation parameters: {}. Derived test cases: {}.
                  These will be used to dynamically adapt simulation and test suite configuration.",
                  insights.derived_simulation_params, insights.derived_test_cases);
            // In a real scenario, these insights would modify simulation_engine.config and automated_test_suite.config
            // For now, we simulate their influence.
        }

        // --- Phase 1: Provisioning and Data Mirroring ---
        report.status = SandboxStatus::Provisioning;
        info!("Phase 1: Provisioning sandbox and mirroring data...");
        match self.virtualization_engine.provision_sandbox(&feature_proposal_id).await {
            Ok(id) => {
                sandbox_id = id;
                report.sandbox_id = sandbox_id.clone();
                self.data_mirroring_system.snapshot_production_data(&sandbox_id).await?;
                self.data_mirroring_system.load_data_snapshot(&sandbox_id, "latest_prod_snapshot").await?;
            },
            Err(e) => {
                error!("Sandbox provisioning failed: {:?}", e);
                report.status = SandboxStatus::Failed;
                report.recommendation = format!("Failed to provision sandbox: {}", e);
                // Attempt cleanup
                let _ = self.cleanup_sandbox_resources(&report.sandbox_id, &sandboxed_env_config.auto_deprovision_sandbox).await;
                return Ok(report);
            }
        }

        // --- Phase 1.5: Configure External Service Mocks ---
        if !simulation_engine_config.external_service_mocks.is_empty() {
            report.status = SandboxStatus::ConfiguringMocks;
            info!("Phase 1.5: Configuring external service mocks...");
            for mock_config in &simulation_engine_config.external_service_mocks {
                self.simulation_engine.mock_external_service(&sandbox_id, mock_config).await?;
            }
            info!("External service mocks configured for sandbox {}.", sandbox_id);
        }

        // --- Phase 2: Feature Deployment to Sandbox ---
        report.status = SandboxStatus::DeployingFeature;
        info!("Phase 2: Deploying feature to sandbox...");
        self.deployment_manager.deploy_feature_to_sandbox(&feature_proposal_id, &sandbox_id).await?;

        // --- Phase 3: Iterative Simulation, Testing, Profiling, Ethical Monitoring, Self-Healing, and Self-Modification Loop ---
        info!("Phase 3: Entering iterative loop for evaluation, self-healing, and self-modification...");
        let mut current_metrics: Option<crate::performance_profiler::PerformanceMetrics> = None;
        let mut optimization_cycles = 0;
        let mut resilience_failures = 0;
        let mut external_mock_issues_count = 0;
        let mut self_healing_attempted_in_cycle = false;

        loop {
            info!("\n--- Iteration {} ---", optimization_cycles + 1);

            report.status = SandboxStatus::Simulating;
            info!("  Running simulations...");
            let sim_report = self.simulation_engine.run_simulations(&sandbox_id).await?;
            report.sim_report_details = Some(sim_report.details.clone());
            // Simulate detecting issues with external mocks during simulation
            if sim_report.details.contains("external_api_call") && sim_report.details.contains("error") {
                external_mock_issues_count += 1;
                warn!("  Simulated external mock interaction issue detected in sandbox {}.", sandbox_id);
                report.external_mock_interaction_issues = Some(format!("Simulated external API errors detected: {}.", sim_report.details));
            }

            report.status = SandboxStatus::Testing;
            info!("  Running automated tests...");
            let test_report = self.automated_test_suite.run_tests(&sandbox_id).await?;
            report.test_results = format!("Passed: {}/{}, Rate: {:.2}%", test_report.passed, test_report.total_tests, test_report.pass_rate * 100.0);

            report.status = SandboxStatus::Profiling;
            info!("  Profiling performance...");
            let prof_metrics = self.performance_profiler.profile_performance(&sandbox_id).await?;
            report.initial_performance = format!("CPU: {:.2}%, Mem: {:.2}MB, Latency: {:.2}ms", prof_metrics.cpu_usage_percent, prof_metrics.memory_usage_mb, prof_metrics.response_time_ms);
            current_metrics = Some(prof_metrics.clone());

            report.status = SandboxStatus::EthicalMonitoring;
            info!("  Monitoring ethical compliance...");
            let ethical_report = self.ethical_compliance_monitor.monitor_compliance(&sandbox_id).await?;
            if ethical_report.human_review_needed {
                human_review_needed_total = true;
                report.ethical_violations = ethical_report.details.clone();
                warn!("Critical ethical flags raised in sandbox {}. Human review REQUIRED.", sandbox_id);
                self.ethical_compliance_monitor.generate_audit_trail(&sandbox_id).await?;
                // Request human review via orchestrator
                if human_in_the_loop_config.enable_manual_override {
                    self.human_in_the_loop_orchestrator.request_human_review(
                        &sandbox_id,
                        &format!("Critical ethical violations detected: {}", ethical_report.details),
                        &human_in_the_loop_config.review_dashboard_url
                    ).await?;
                    report.human_review_requested = true;
                }
                report.status = SandboxStatus::HumanReviewPending;
                report.recommendation = "Human review required due to critical ethical violations. Aborting automated deployment.".to_string();
                break; // Exit loop, require human intervention
            }
            report.ethical_violations = ethical_report.details.clone();

            // --- Fault Injection Phase ---
            if fault_injection_config.enable_fault_injection && optimization_cycles == 0 && !self_healing_attempted_in_cycle { // Only inject fault once per experiment for simplicity
                report.status = SandboxStatus::InjectingFault;
                info!("  Injecting fault for resilience testing...");
                let fault_type = fault_injection_config.fault_types.get(0).cloned().unwrap_or_else(|| "process_crash".to_string());
                let fault_target = fault_injection_config.fault_targets.get(0).cloned().unwrap_or_else(|| "tariro-service-core".to_string());
                
                match self.virtualization_engine.inject_fault(&sandbox_id, &fault_type, &fault_target).await {
                    Ok(_) => {
                        info!("  Fault injected. Monitoring resilience...");
                        tokio::time::sleep(Duration::from_secs(10)).await; // Allow system to react to fault
                        // Re-run a quick simulation/test to see impact
                        let post_fault_sim_report = self.simulation_engine.run_simulations(&sandbox_id).await?;
                        let post_fault_test_report = self.automated_test_suite.run_tests(&sandbox_id).await?;
                        if !post_fault_test_report.success {
                            resilience_failures += 1;
                            warn!("  Sandbox {} showed resilience failure after fault injection.", sandbox_id);
                        }
                        report.fault_injection_results = Some(FaultInjectionReport {
                            fault_type: fault_type.clone(),
                            target: fault_target.clone(),
                            resilience_score: if resilience_failures == 0 { 1.0 } else { 0.0 }, // Simple score
                            details: format!("Simulated fault '{}' targeting '{}'. Test pass rate after fault: {:.2}%.", fault_type, fault_target, post_fault_test_report.pass_rate * 100.0),
                        });
                    },
                    Err(e) => error!("  Failed to inject fault: {:?}", e),
                }
            }
            // --- End Fault Injection Phase ---

            let target_latency = self.performance_profiler.config.target_latency_ms as f64;
            let target_cpu = self.performance_profiler.config.target_cpu_usage_percent;

            let performance_needs_improvement = prof_metrics.response_time_ms > target_latency || prof_metrics.cpu_usage_percent > target_cpu;
            let tests_failed = test_report.pass_rate < self.automated_test_suite.config.minimum_pass_rate;
            let resilience_poor = resilience_failures >= fault_injection_config.resilience_threshold_failures;
            let external_mock_issues_present = external_mock_issues_count > 0;

            if performance_needs_improvement || tests_failed || resilience_poor || external_mock_issues_present {
                // Check for self-healing opportunities first
                if sandboxed_env_config.enable_self_healing_attempts && !self_healing_attempted_in_cycle {
                    report.status = SandboxStatus::SelfHealing;
                    info!("  Issues detected. Attempting self-healing...");
                    let issue_summary = format!("Performance: {} (target {}), Tests: {} (target {}), Resilience: {} (target {}), External Mocks: {}.",
                                                performance_needs_improvement, target_latency, tests_failed, self.automated_test_suite.config.minimum_pass_rate, resilience_poor, fault_injection_config.resilience_threshold_failures,
                                                external_mock_issues_present);
                    let healing_report = self.self_healing_engine.diagnose_and_heal(&sandbox_id, &issue_summary).await?;
                    report.self_healing_attempts.push(healing_report.clone());

                    if healing_report.healing_successful {
                        info!("  Self-healing successful. Re-evaluating sandbox.");
                        self_healing_attempted_in_cycle = true; // Mark healing attempted for this cycle
                        continue; // Re-run the current loop iteration to re-evaluate after healing
                    } else {
                        warn!("  Self-healing attempted but not successful. Proceeding to code generation if needed.");
                    }
                }

                optimization_cycles += 1;
                if optimization_cycles <= self.config.max_optimization_cycles {
                    report.status = SandboxStatus::Optimizing;
                    let mut optimization_goal_parts = Vec::new();
                    if performance_needs_improvement { optimization_goal_parts.push(format!("Optimize performance: Latency target {}ms, current {:.2}ms. CPU target {:.2}%, current {:.2}%. ", target_latency, prof_metrics.response_time_ms, target_cpu, prof_metrics.cpu_usage_percent)); }
                    if tests_failed { optimization_goal_parts.push(format!("Fix test failures: Pass rate target {:.2}%, current {:.2}%. ", self.automated_test_suite.config.minimum_pass_rate * 100.0, test_report.pass_rate * 100.0)); }
                    if resilience_poor { optimization_goal_parts.push(format!("Improve resilience: {} resilience failures detected.", resilience_failures)); }
                    if external_mock_issues_present { optimization_goal_parts.push(format!("Resolve external mock interaction issues: {} detected.", external_mock_issues_count)); }
                    let optimization_goal = optimization_goal_parts.join(" ");
                    
                    info!("  Targets not met. Generating code changes for: {}\n  Current code context: '{}'".to_string(), optimization_goal, current_code_context);
                    
                    let generated_code_proposal = self.code_generation_engine.generate_code(
                        &optimization_goal,
                        &current_code_context
                    ).await?;
                    
                    report.generated_code_feedback.push(format!("Iteration {}: Generated code (Quality: {:.2}, Human review: {}) to address: {}",
                                                               optimization_cycles, generated_code_proposal.quality_score, generated_code_proposal.requires_human_review, optimization_goal));
                    
                    if generated_code_proposal.requires_human_review {
                        human_review_needed_total = true;
                        // Request human review via orchestrator
                        if human_in_the_loop_config.enable_manual_override {
                            self.human_in_the_loop_orchestrator.request_human_review(
                                &sandbox_id,
                                &format!("Generated code requires review: {}", generated_code_proposal.description),
                                &human_in_the_loop_config.review_dashboard_url
                            ).await?;
                            report.human_review_requested = true;
                        }
                        report.status = SandboxStatus::HumanReviewPending;
                        report.recommendation = format!("Generated code requires human review before application: {}. Aborting automated deployment.", generated_code_proposal.description);
                        break; // Exit loop, require human intervention
                    }
                    
                    info!("  Applying generated code changes...");
                    self.code_generation_engine.apply_code_changes(&sandbox_id, &generated_code_proposal).await?;
                    current_code_context = generated_code_proposal.code; // Update context with new code
                    info!("  Code changes applied. Re-running evaluations.");
                    self_healing_attempted_in_cycle = false; // Reset for next optimization cycle

                } else {
                    warn!("  Max optimization cycles ({}) reached. Targets still not fully met.", self.config.max_optimization_cycles);
                    report.status = SandboxStatus::Completed;
                    report.recommendation = "Automated self-modification failed to meet all targets. Human review recommended.".to_string();
                    // Proactive suggestion for Samuel
                    let suggestion = self.human_in_the_loop_orchestrator.proactive_suggestion(
                        &sandbox_id, 
                        &format!("Max optimization cycles reached, targets not met for {}.", feature_proposal_id.0),
                        0.8 // High urgency score
                    ).await?;
                    if suggestion != "No proactive suggestion generated." { report.proactive_suggestions_made.push(suggestion); }

                    break; // Exit loop, proceed to final report
                }
            } else {
                info!("  All targets met. Exiting iterative self-modification loop.");
                break; // Performance and tests targets met, exit loop
            }
        }

        // --- Phase 4: Final Evaluation and Deployment Decision ---
        report.status = SandboxStatus::FinalEvaluation;
        info!("Phase 4: Final Evaluation and Autonomous Deployment Decision...");

        let final_metrics = current_metrics.unwrap_or_else(|| crate::performance_profiler::PerformanceMetrics { cpu_usage_percent: 0.0, memory_usage_mb: 0.0, response_time_ms: 0.0 });
        let final_test_report = self.automated_test_suite.run_tests(&sandbox_id).await?;
        let final_ethical_report = self.ethical_compliance_monitor.monitor_compliance(&sandbox_id).await?;
        let final_code_quality = self.code_generation_engine.evaluate_code_quality(&current_code_context).await?;

        let performance_ok = final_metrics.response_time_ms <= self.performance_profiler.config.target_latency_ms as f64 &&
                             final_metrics.cpu_usage_percent <= self.performance_profiler.config.target_cpu_usage_percent;
        let tests_ok = final_test_report.pass_rate >= self.automated_test_suite.config.minimum_pass_rate;
        let ethical_ok = !final_ethical_report.human_review_needed;
        let code_quality_ok = final_code_quality.adheres_to_standards && !final_code_quality.requires_human_review_due_to_quality; 
        let resilience_ok = resilience_failures < fault_injection_config.resilience_threshold_failures;
        let external_mock_interactions_ok = external_mock_issues_count == 0;

        if !human_review_needed_total && performance_ok && tests_ok && ethical_ok && code_quality_ok && resilience_ok && external_mock_interactions_ok {
            deployment_approved = true;
            report.recommendation = "All checks passed: Feature is ready for production deployment.".to_string();
            if sandboxed_env_config.enable_autonomous_deployment {
                info!("Autonomous deployment enabled. Initiating production deployment...");
                self.deployment_manager.deploy_to_production(&feature_proposal_id).await?;
                report.deployment_status = "Deployed to Production".to_string();
                report.status = SandboxStatus::CompletedAndDeployed;
            } else {
                info!("Autonomous deployment disabled. Recommending manual production deployment.");
                report.recommendation.push_str(" (Manual deployment required)");
                report.status = SandboxStatus::Completed;
            }
        } else {
            let mut reasons = Vec::new();
            if human_review_needed_total { reasons.push("Critical ethical violations or code review needed"); }
            if !performance_ok { reasons.push("Performance targets not met"); }
            if !tests_ok { reasons.push("Test pass rate below threshold"); }
            if !ethical_ok { reasons.push("Pending ethical review"); }
            if !code_quality_ok { reasons.push("Code quality issues detected"); }
            if !resilience_ok { reasons.push("Resilience targets not met"); }
            if !external_mock_interactions_ok { reasons.push("External mock interaction issues detected"); }

            report.status = SandboxStatus::Failed;
            report.recommendation = format!("Deployment NOT recommended. Reasons: {}. Human review REQUIRED.", reasons.join(", "));
            error!("Sandbox experiment for {} completed with issues: {}.", feature_proposal_id.0, report.recommendation);
            // Always request human review if deployment not recommended
            if human_in_the_loop_config.enable_manual_override {
                self.human_in_the_loop_orchestrator.request_human_review(
                    &sandbox_id,
                    &format!("Deployment NOT recommended. Reasons: {}.", reasons.join(", ")),
                    &human_in_the_loop_config.review_dashboard_url
                ).await?;
                report.human_review_requested = true;
            }
        }

        // --- Phase 5: Cleanup ---
        info!("Phase 5: Cleaning up sandbox resources...");
        let _ = self.cleanup_sandbox_resources(&sandbox_id, &sandboxed_env_config.auto_deprovision_sandbox).await;

        info!("Autonomous sandbox experiment for FeatureProposal {} finished with status: {:?}", feature_proposal_id.0, report.status);
        Ok(report)
    }

    async fn cleanup_sandbox_resources(&self, sandbox_id: &str, auto_deprovision: &bool) -> Result<()> {
        if *auto_deprovision {
            match self.virtualization_engine.deprovision_sandbox(sandbox_id).await {
                Ok(_) => info!("Sandbox {} de-provisioned successfully.", sandbox_id),
                Err(e) => error!("Failed to de-provision sandbox {}: {}", sandbox_id, e),
            }
            match self.data_mirroring_system.cleanup_sandbox_data(sandbox_id).await {
                Ok(_) => info!("Sandbox {} data cleaned up successfully.", sandbox_id),
                Err(e) => error!("Failed to clean up sandbox {} data: {}", sandbox_id, e),
            }
        } else {
            info!("Auto-deprovisioning disabled. Sandbox {} and its data retained.", sandbox_id);
        }
        Ok(())
    }
}

/// Placeholder for SandboxReport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxReport {
    pub sandbox_id: String,
    pub feature_proposal_id: FeatureProposalId,
    pub test_results: String, // Consolidated from AutomatedTestSuite
    pub initial_performance: String, // Consolidated from PerformanceProfiler
    pub optimized_config: String, // Consolidated from PerformanceProfiler
    pub ethical_violations: String, // Consolidated from EthicalComplianceMonitor
    pub deployment_status: String,
    pub status: SandboxStatus,
    pub recommendation: String,
    pub sim_report_details: Option<String>,
    pub generated_code_feedback: Vec<String>, // New field for code generation feedback
    pub fault_injection_results: Option<FaultInjectionReport>, // New field
    pub external_mock_interaction_issues: Option<String>, // New field
    pub telemetry_enabled_for_experiment: bool, // New field
    pub self_healing_attempts: Vec<self_healing_engine::HealingAttemptReport>, // New field
    pub production_learning_insights: Option<String>, // New field
    pub human_review_requested: bool, // New field
    pub proactive_suggestions_made: Vec<String>, // New field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SandboxStatus {
    Initiated,
    Provisioning,
    ConfiguringMocks,
    DeployingFeature,
    Simulating,
    Testing,
    Profiling,
    Optimizing,
    EthicalMonitoring,
    InjectingFault,
    SelfHealing,
    LearningFromProduction,
    FinalEvaluation,
    HumanReviewPending,
    Completed,
    CompletedAndDeployed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultInjectionReport {
    pub fault_type: String,
    pub target: String,
    pub resilience_score: f32,
    pub details: String,
}
