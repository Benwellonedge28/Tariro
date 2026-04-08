// src/deployment/manager.rs
use crate::default_api::{DeploymentRecord, DeploymentRecordStatus, MetaProject, AutonomyPolicyAutonomyLevel};
use crate::autonomy::policy_engine::PolicyEngine;
use crate::atve::report_analyzer::AtveReport;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DeploymentManager {
    policy_engine: Arc<Mutex<PolicyEngine>>,
}

impl DeploymentManager {
    pub fn new(policy_engine: Arc<Mutex<PolicyEngine>>) -> Self { Self { policy_engine } }

    pub async fn initiate_deployment(&self, project: &MetaProject, atve_report: &AtveReport) -> Result<DeploymentRecord, String> {
        let autonomy_level = self.policy_engine.lock().await.get_autonomy_level("deployment_management");
        if autonomy_level == AutonomyPolicyAutonomyLevel::HumanRequired {
            return Err("Human approval required for deployment.".to_string());
        }
        Ok(DeploymentRecord { id: Some("mock_deploy_id".to_string()), project_name: project.name.clone(), target: "mock_target".to_string(), status: DeploymentRecordStatus::Deploying, atve_passed: atve_report.overall_status == crate::atve::report_analyzer::AtveStatus::Success, version: project.version.clone(), notes: None, performance_score: None })
    }
}
