// src/modules/self_modification_module.rs

//! This module defines the Self-Modification Module, which is central to Tariro's
//! autonomous evolution. It identifies opportunities for self-improvement, orchestrates
//! the sandboxed experimentation of proposed changes, and decides whether to apply,
//! refine, or escalate these changes for human review.

use crate::{prelude::*,
    Module,
    ModuleContext,
    SelfModificationModuleConfig,
    FeatureProposalId,
    modules::sandboxed_environment::{SandboxedEnvironmentModule, SandboxReport, SandboxStatus},
    modules::idpte_module::IDPTEModule,
};
use async_trait::async_trait;
use tokio::time::{sleep, Duration};
use rand::Rng;
use std::collections::HashMap;

/// Represents the Self-Modification Module.
pub struct SelfModificationModule {
    config: SelfModificationModuleConfig,
    sandboxed_environment_module: SandboxedEnvironmentModule,
    context: ModuleContext, // Store ModuleContext to access API tools
    meta_learning_outcomes: tokio::sync::Mutex<Vec<SelfModificationOutcome>>, // Store outcomes for meta-learning
    goal_monitor: GoalMonitor, // New: Goal Monitor
    idpte_module: IDPTEModule, // New: IDPTE Module
}

#[async_trait]
pub trait Module: Send + Sync {
    fn new(context: &ModuleContext) -> Result<Self> where Self: Sized;
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    fn name(&self) -> &'static str;
}

impl SelfModificationModule {
    fn new(context: &ModuleContext) -> Result<Self> {
        let config = context.config.self_modification_module.clone().unwrap_or_default();
        let sandboxed_environment_module = SandboxedEnvironmentModule::new(context)?;
        let goal_monitor = GoalMonitor::new(context); // Initialize GoalMonitor
        let idpte_module = IDPTEModule::new(context)?; // Initialize IDPTEModule

        Ok(Self { 
            config,
            sandboxed_environment_module,
            context: context.clone(), // Clone context for tool access
            meta_learning_outcomes: tokio::sync::Mutex::new(Vec::new()),
            goal_monitor, 
            idpte_module,
        })
    }

    async fn start(&self) -> Result<()> {
        info!("SelfModificationModule starting...");
        self.sandboxed_environment_module.start().await?;
        self.goal_monitor.start().await?;
        self.idpte_module.start().await?;
        info!("SelfModificationModule started. Self-modification enabled: {}. Frequency: {}s.",
              self.config.enable_self_modification, self.config.modification_frequency_seconds);

        if self.config.meta_learning.as_ref().map_or(false, |m| m.enable_meta_learning) {
            info!("Meta-learning enabled. Strategy evaluation frequency: {}s.", self.config.meta_learning.as_ref().unwrap().strategy_evaluation_frequency_seconds);
            let freq = self.config.meta_learning.as_ref().unwrap().strategy_evaluation_frequency_seconds;
            tokio::spawn(Self::run_periodic_meta_learning_analysis(self.meta_learning_outcomes.clone(), freq));
        }
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        info!("SelfModificationModule stopping...");
        self.sandboxed_environment_module.stop().await?;
        self.goal_monitor.stop().await?;
        self.idpte_module.stop().await?;
        info!("SelfModificationModule stopped.");
        Ok(())
    }

    fn name(&self) -> &'static str {
        "SelfModificationModule"
    }
}

impl SelfModificationModule {
    /// Periodically analyzes past self-modification outcomes to refine meta-strategies.
    async fn run_periodic_meta_learning_analysis(outcomes: tokio::sync::Mutex<Vec<SelfModificationOutcome>>, frequency_seconds: u64) {
        loop {
            sleep(Duration::from_secs(frequency_seconds)).await;
            info!("Periodic meta-learning analysis triggered.");
            let mut locked_outcomes = outcomes.lock().await;
            if !locked_outcomes.is_empty() {
                let analysis = Self::analyze_self_modification_outcomes_static(&locked_outcomes);
                info!("Meta-learning analysis results: {:?}", analysis);
                locked_outcomes.clear();
            }
        }
    }

    /// Analyzes logged outcomes of self-modification attempts to identify patterns and refine strategies.
    fn analyze_self_modification_outcomes_static(outcomes: &[SelfModificationOutcome]) -> HashMap<String, String> {
        let mut analysis = HashMap::new();
        let total_outcomes = outcomes.len();
        if total_outcomes == 0 { return analysis; }

        let successful_deployments = outcomes.iter().filter(|o| o.final_status == SandboxStatus::CompletedAndDeployed).count();
        let failed_experiments = outcomes.iter().filter(|o| o.final_status == SandboxStatus::Failed).count();
        let human_reviews = outcomes.iter().filter(|o| o.final_status == SandboxStatus::HumanReviewPending).count();

        analysis.insert("total_experiments".to_string(), total_outcomes.to_string());
        analysis.insert("successful_deployments".to_string(), successful_deployments.to_string());
        analysis.insert("failed_experiments".to_string(), failed_experiments.to_string());
        analysis.insert("human_reviews_needed".to_string(), human_reviews.to_string());

        if (successful_deployments as f32 / total_outcomes as f32) < 0.5 {
            analysis.insert("overall_strategy_recommendation".to_string(), "Consider more conservative code generation, or more aggressive self-healing.".to_string());
        } else if (human_reviews as f32 / total_outcomes as f32) > 0.3 {
            analysis.insert("overall_strategy_recommendation".to_string(), "Refine code review thresholds for AI-generated code, or improve ethical monitoring.".to_string());
        }

        analysis
    }

    /// Identifies opportunities for Tariro to self-modify or improve.
    /// This could be based on: performance logs, user feedback, new knowledge items,
    /// ethical flags, or strategic goals.
    /// This method now simulates creating actual FeatureProposal entities, potentially linked to goals.
    pub async fn identify_modification_opportunities(&self) -> Result<Vec<FeatureProposalId>> {
        info!("Identifying self-modification opportunities...");
        sleep(Duration::from_secs(2)).await;

        let mut opportunities = Vec::new();
        let mut rng = rand::thread_rng();

        // Get active goals to identify areas for improvement
        let active_goals = self.goal_monitor.get_active_goals().await?;

        if rng.gen_bool(0.6) { // Increased chance of finding an opportunity if goals are present
            let target_goal = if !active_goals.is_empty() {
                // Pick a random active goal to link the proposal to
                let goal = &active_goals[rng.gen_range(0..active_goals.len())];
                Some(goal.clone())
            } else { None };

            let proposal_title = format!("Optimize {} core component ({})", rng.gen_range(1..100), target_goal.as_ref().map_or("General Improvement".to_string(), |g| g.title.clone()));
            let problem_statement = format!("Identified high resource utilization during peak load in a core service. This impacts goal: {}.", 
                                            target_goal.as_ref().map_or("N/A".to_string(), |g| g.title.clone()));
            let proposed_solution = "Implement a more efficient algorithm for data processing in the identified service, potentially using dynamic code generation.".to_string();
            let estimated_impact = format!("Reduce CPU usage by 10-15% and latency by 5% during high traffic periods, thereby accelerating progress on goal: {}.",
                                           target_goal.as_ref().map_or("N/A".to_string(), |g| g.title.clone()));

            let new_proposal_result = default_api::create_FeatureProposal(
                proposal_title.clone(),
                problem_statement,
                proposed_solution,
                Some(estimated_impact),
                Some(FeatureProposalPriority::High),
                Some(FeatureProposalSource::SelfMonitoring),
                Some(FeatureProposalStatus::Proposed),
                None, // draft_config_or_code
            ).await?;
            
            let proposal_id = FeatureProposalId(new_proposal_result.get("id").and_then(|id| id.as_str()).map(|s| s.to_string()).ok_or_else(|| anyhow!("Failed to get FeatureProposal ID"))?);
            opportunities.push(proposal_id.clone());
            info!("Created new FeatureProposal: {} (ID: {}).", proposal_title, proposal_id.0);
        }
        info!("Found {} self-modification opportunities.", opportunities.len());
        Ok(opportunities)
    }

    /// Dynamically selects the most appropriate self-modification strategy based on meta-learning insights.
    pub fn select_modification_strategy(&self, issue_context: &str) -> ModificationStrategy {
        if self.config.meta_learning.as_ref().map_or(false, |m| m.enable_meta_learning) {
            if issue_context.contains("performance") {
                ModificationStrategy { 
                    code_gen_aggressiveness: 0.8,
                    self_healing_attempts: 2,
                    priority: 0.9,
                    notes: "Aggressive code generation for performance issues due to past success.".to_string(),
                }
            } else if issue_context.contains("ethical") {
                ModificationStrategy { 
                    code_gen_aggressiveness: 0.5,
                    self_healing_attempts: 1,
                    priority: 1.0,
                    notes: "Conservative strategy for ethical issues, prioritizing human review.".to_string(),
                }
            } else {
                ModificationStrategy::default()
            }
        } else {
            ModificationStrategy::default()
        }
    }

    /// Initiates a self-modification process for a given feature proposal.
    /// This is the orchestrator that uses the SandboxedEnvironmentModule and IDPTEModule.
    pub async fn initiate_self_modification(&self, proposal_id: FeatureProposalId, issue_context: &str, file_to_modify: &str) -> Result<()> {
        info!("Initiating self-modification process for proposal: {}", proposal_id.0);

        if !self.config.enable_self_modification {
            warn!("Self-modification is disabled. Cannot proceed with proposal {}.", proposal_id.0);
            return Err(anyhow!("Self-modification disabled."));
        }

        let strategy = self.select_modification_strategy(issue_context);
        info!("Applying modification strategy: {:?}", strategy);

        // 1. Run the sandboxed experiment
        let sandbox_report = self.sandboxed_environment_module.run_sandbox_experiment(proposal_id.clone()).await?;
        info!("Sandboxed experiment for {} completed with status: {:?}", proposal_id.0, sandbox_report.status);

        // 2. Log outcome for meta-learning
        let outcome = SelfModificationOutcome {
            proposal_id: proposal_id.clone(),
            issue_context: issue_context.to_string(),
            strategy_applied: strategy.clone(),
            final_status: sandbox_report.status.clone(),
            recommendation: sandbox_report.recommendation.clone(),
        };
        self.meta_learning_outcomes.lock().await.push(outcome);

        // 3. Interpret the sandbox report and decide next steps
        match sandbox_report.status {
            SandboxStatus::CompletedAndDeployed => {
                info!("Self-modification for {} successfully deployed to production!", proposal_id.0);
                // Assuming sandbox_report provides the final modified code
                let modified_code_content = format!("// {} - Optimized code based on sandbox experiment {}\n// Original proposal: {}\nfn optimized_function() {{ /* ... */ }}\n", file_to_modify, proposal_id.0, issue_context);

                // Deploy to IDE
                if self.idpte_module.config.ide_interaction_mode == "auto" || self.idpte_module.config.enable_autonomous_file_management {
                    info!("Autonomously writing optimized code to IDE: {}.", file_to_modify);
                    self.idpte_module.write_file_to_ide(file_to_modify, &modified_code_content).await?;
                    let commit_message = self.idpte_module.generate_autonomous_commit_message(&format!("Optimized {} based on sandbox results.", file_to_modify)).await?;
                    self.idpte_module.execute_ide_command(&format!("git add {} && git commit -m \"{}\"", file_to_modify, commit_message), None).await?;
                    info!("Optimized code for {} deployed to IDE and committed.", file_to_modify);
                } else {
                    info!("Optimized code for {} requires user confirmation for IDE deployment.", file_to_modify);
                    self.idpte_module.propose_file_changes_to_user(file_to_modify, &format!("Optimized code for {} based on sandbox results.", file_to_modify)).await?;
                    // If confirmed, then Tariro would execute the write and commit
                }

            },
            SandboxStatus::Completed => {
                info!("Self-modification for {} completed in sandbox, but manual deployment is required or autonomous deployment is disabled. Recommendation: {}", proposal_id.0, sandbox_report.recommendation);
            },
            SandboxStatus::HumanReviewPending => {
                warn!("Self-modification for {} requires human review. Reason: {}.", proposal_id.0, sandbox_report.recommendation);
            },
            SandboxStatus::Failed => {
                error!("Self-modification for {} failed in sandbox. Reason: {}.", proposal_id.0, sandbox_report.recommendation);
            },
            _ => {
                warn!("Self-modification for {} ended in an unexpected sandbox status: {:?}", proposal_id.0, sandbox_report.status);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModificationOutcome {
    pub proposal_id: FeatureProposalId,
    pub issue_context: String,
    pub strategy_applied: ModificationStrategy,
    pub final_status: SandboxStatus,
    pub recommendation: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModificationStrategy {
    pub code_gen_aggressiveness: f32,
    pub self_healing_attempts: u32,
    pub priority: f32,
    pub notes: String,
}

// Temporary enums for FeatureProposal fields, should be imported from actual data model
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureProposalPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureProposalSource {
    SelfMonitoring,
    UserInteraction,
    PerformanceGap,
    SecurityConcern,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureProposalStatus {
    Proposed,
    UnderReview,
    Approved,
    Rejected,
    Implemented,
}

// --- Goal Monitoring Sub-Module --- (Internal to SelfModificationModule for now)
pub struct GoalMonitor {
    _context: ModuleContext, // Context might be useful for future enhancements
}

impl GoalMonitor {
    pub fn new(context: &ModuleContext) -> Self {
        Self { _context: context.clone() }
    }

    pub async fn start(&self) -> Result<()> {
        info!("GoalMonitor starting...");
        info!("GoalMonitor started.");
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        info!("GoalMonitor stopping...");
        info!("GoalMonitor stopped.");
        Ok(())
    }

    /// Retrieves a list of active goals that could be targets for self-modification.
    pub async fn get_active_goals(&self) -> Result<Vec<Goal>> {
        info!("Retrieving active goals...");
        // Use the default_api to read Goal entities
        let goals_response = default_api::read_Goal(
            Some(vec!["id", "title", "description", "progress_pct", "priority"]),
            None,
            Some(serde_json::json!({ "status": "active" }).into()),
            None,
            None
        ).await?;
        
        let goals_data = goals_response.get("records").ok_or_else(|| anyhow!("No records field in read_Goal response"))?;
        let goals_json: Vec<serde_json::Value> = serde_json::from_value(goals_data.clone())?;

        let mut active_goals = Vec::new();
        for goal_json in goals_json {
            let goal_id = goal_json.get("id").and_then(|id| id.as_str()).map(|s| s.to_string()).ok_or_else(|| anyhow!("Goal missing ID"))?;
            let title = goal_json.get("title").and_then(|t| t.as_str()).map(|s| s.to_string()).ok_or_else(|| anyhow!("Goal missing title"))?;
            let description = goal_json.get("description").and_then(|d| d.as_str()).map(|s| s.to_string()).unwrap_or_default();
            let progress_pct = goal_json.get("progress_pct").and_then(|p| p.as_f64()).unwrap_or(0.0);
            let priority_str = goal_json.get("priority").and_then(|p| p.as_str()).map(|s| s.to_string()).unwrap_or_default();

            active_goals.push(Goal {
                id: goal_id,
                title,
                description,
                progress_pct: progress_pct as f32,
                priority: priority_str, // Storing as String for simplicity
            });
        }

        info!("Retrieved {} active goals.", active_goals.len());
        Ok(active_goals)
    }
}

// Simplified Goal structure for internal use in SelfModificationModule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub progress_pct: f32,
    pub priority: String,
}
