// src/autonomy/governance.rs

//! Implements the governance mechanisms for Tariro AGI's autonomy, ensuring ethical alignment
//! and controlled self-modification within defined policy boundaries.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};
use crate::autonomy::policy_engine::{AutonomyPolicy, AutonomyPolicyEngine, PolicyDecision};

/// Represents an action taken by the governance system to change Tariro's autonomy or policies.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GovernanceAction {
    pub action_id: String,
    pub timestamp: DateTime<Utc>,
    pub action_type: GovernanceActionType,
    pub target_policy_id: Option<String>, // Relevant for policy-specific actions
    pub proposed_changes: HashMap<String, String>, // Key-value pairs of changes
    pub justification: String,
    pub decision_origin: DecisionOrigin, // e.g., "Human Admin", "Self-Governance Module"
    pub status: GovernanceActionStatus,
    pub approval_level: Option<u8>, // e.g., 1 (basic), 2 (critical)
}

/// Types of governance actions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GovernanceActionType {
    UpdatePolicy,
    ChangeAutonomyLevel,
    ReviewDecision,
    ProposeSelfModification,
    EmergencyOverride,
    Custom(String),
}

/// Origin of a governance decision.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DecisionOrigin {
    HumanAdmin,
    SelfGovernanceModule,
    AutomatedSystem,
    ExternalDirective,
}

/// Status of a governance action.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GovernanceActionStatus {
    PendingApproval,
    Approved,
    Rejected,
    Executed,
    Failed,
    Superseded,
}

/// Represents a record of a change in Tariro's autonomy level or a critical policy.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutonomyChangeEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub affected_domain: String,
    pub old_autonomy_level: String,
    pub new_autonomy_level: String,
    pub reason: String,
    pub triggered_by_action_id: String,
}

/// The `GovernanceService` enforces and manages autonomy policies, and logs all governance actions.
pub struct GovernanceService {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    policy_engine: AutonomyPolicyEngine,
    active_actions: HashMap<String, GovernanceAction>,
}

impl GovernanceService {
    /// Creates a new `GovernanceService` instance.
    pub fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, policy_engine: AutonomyPolicyEngine) -> Self {
        GovernanceService {
            config,
            telemetry_logger,
            policy_engine,
            active_actions: HashMap::new(),
        }
    }

    /// Proposes a new governance action.
    pub async fn propose_action(&mut self, mut action: GovernanceAction) -> Result<String, String> {
        action.action_id = format!("gov_act_{}", Uuid::new_v4());
        action.timestamp = Utc::now();
        action.status = GovernanceActionStatus::PendingApproval;

        if self.active_actions.contains_key(&action.action_id) {
            return Err(format!("Governance action with ID {} already exists.", action.action_id));
        }

        println!("[GovernanceService] Proposed governance action: {:?}", action.action_type);
        self.telemetry_logger.log_event(TelemetryEvent::AGIAction {
            action_type: format!("ProposeGovernanceAction::{:?}", action.action_type),
            details: Some(format!("Action ID: {}, Justification: {}", action.action_id, action.justification)),
        }).await;

        self.active_actions.insert(action.action_id.clone(), action.clone());
        // In a real system, this would trigger a human review process or internal AGI consensus.
        Ok(action.action_id)
    }

    /// Approves and executes a pending governance action.
    pub async fn approve_and_execute_action(&mut self, action_id: &str, approver: &str) -> Result<(), String> {
        if let Some(action) = self.active_actions.get_mut(action_id) {
            if action.status != GovernanceActionStatus::PendingApproval {
                return Err(format!("Action {} is not pending approval.", action_id));
            }
            action.status = GovernanceActionStatus::Approved;
            println!("[GovernanceService] Action {} approved by {}. Executing...", action_id, approver);
            self.telemetry_logger.log_event(TelemetryEvent::AdminCommandCompleted {
                command_id: action_id.to_string(),
                status: "approved".to_string(),
                message: format!("Governance action {} approved.", action_id),
            }).await;

            match self.execute_action(action).await {
                Ok(_) => {
                    action.status = GovernanceActionStatus::Executed;
                    println!("[GovernanceService] Action {} executed successfully.", action_id);
                    Ok(())
                },
                Err(e) => {
                    action.status = GovernanceActionStatus::Failed;
                    eprintln!("[GovernanceService] Failed to execute action {}: {}", action_id, e);
                    Err(format!("Execution failed: {}", e))
                },
            }
        } else {
            Err(format!("Governance action with ID {} not found.", action_id))
        }
    }

    async fn execute_action(&mut self, action: &GovernanceAction) -> Result<(), String> {
        match action.action_type {
            GovernanceActionType::UpdatePolicy => {
                if let Some(policy_id) = &action.target_policy_id {
                    let mut current_policy = self.policy_engine.get_policy(policy_id)
                        .ok_or(format!("Policy {} not found.", policy_id))?.clone();

                    for (key, value) in &action.proposed_changes {
                        match key.as_str() {
                            "autonomy_level" => current_policy.autonomy_level = value.parse().map_err(|_| "Invalid autonomy level".to_string())?,
                            "evas_filter" => current_policy.evas_filter = value.parse().map_err(|_| "Invalid boolean for evas_filter".to_string())?,
                            "notes" => current_policy.notes = Some(value.clone()),
                            _ => return Err(format!("Unknown policy field for update: {}", key)),
                        }
                    }
                    self.policy_engine.update_policy(current_policy.clone()).await?;
                    println!("[GovernanceService] Policy {} updated.", policy_id);
                } else {
                    return Err("UpdatePolicy action requires target_policy_id.".to_string());
                }
            },
            GovernanceActionType::ChangeAutonomyLevel => {
                if let (Some(domain), Some(new_level)) = (action.proposed_changes.get("domain"), action.proposed_changes.get("new_level")) {
                    let old_level = self.policy_engine.get_policy_by_domain(domain)
                        .map(|p| p.autonomy_level.to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    self.policy_engine.set_autonomy_level(domain, new_level.parse().map_err(|_| "Invalid autonomy level".to_string())?).await?;
                    println!("[GovernanceService] Autonomy level for domain {} changed from {} to {}.", domain, old_level, new_level);
                    self.telemetry_logger.log_event(TelemetryEvent::AutonomyLevelChanged {
                        affected_domain: domain.clone(),
                        old_autonomy_level: old_level,
                        new_autonomy_level: new_level.clone(),
                        reason: action.justification.clone(),
                        triggered_by_action_id: action.action_id.clone(),
                    }).await;
                } else {
                    return Err("ChangeAutonomyLevel action requires 'domain' and 'new_level'.".to_string());
                }
            },
            // ... handle other action types
            _ => return Err(format!("Execution logic not implemented for action type: {:?}", action.action_type)),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use crate::config::AGIConfig;
    use crate::database::DatabaseService;
    use std::sync::Arc;
    use crate::autonomy::policy_engine::AutonomyLevel;

    // Mock DatabaseService and AutonomyPolicyEngine for testing
    struct MockDbService;

    #[async_trait::async_trait]
    impl crate::database::DatabaseService for MockDbService {
        async fn new(_db_uri: &str, _db_name: &str) -> Result<Self, String> { Ok(MockDbService) }
        async fn create<T: crate::database::DbEntity + Debug + Send>(&self, _entity: &mut T) -> Result<String, String> { Ok("mock_id".to_string()) }
        async fn read<T: crate::database::DbEntity + Debug>(&self, _id: &str) -> Result<Option<T>, String> { Ok(None) }
        async fn update<T: crate::database::DbEntity + Debug>(&self, _id: &str, _new_data: &T) -> Result<(), String> { Ok(()) }
        async fn delete<T: crate::database::DbEntity + Debug>(&self, _id: &str) -> Result<(), String> { Ok(()) }
        async fn list<T: crate::database::DbEntity + Debug>(&self) -> Result<Vec<T>, String> { Ok(vec![]) }
    }

    struct MockAutonomyPolicyEngine {
        policies: HashMap<String, AutonomyPolicy>,
    }

    impl MockAutonomyPolicyEngine {
        fn new_mock() -> Self {
            let mut policies = HashMap::new();
            policies.insert("AP_TestDomain".to_string(), AutonomyPolicy {
                id: Some("AP_TestDomain".to_string()),
                domain: "test_domain".to_string(),
                label: "Test Domain Policy".to_string(),
                autonomy_level: AutonomyLevel::SuggestOnly,
                evas_filter: true,
                notes: None,
            });
            MockAutonomyPolicyEngine { policies }
        }
    }

    #[async_trait::async_trait]
    impl crate::autonomy::policy_engine::AutonomyPolicyEngineTrait for MockAutonomyPolicyEngine {
        fn new(_config: AGIConfig, _telemetry_logger: TelemetryLogger, _db_service: Arc<DatabaseService>) -> Self { Self::new_mock() }
        async fn initialize(&self) -> Result<(), String> { Ok(()) }
        async fn shutdown(&self) -> Result<(), String> { Ok(()) }
        async fn evaluate_action(&self, _action: &str, _context: &HashMap<String, String>) -> Result<PolicyDecision, String> { Ok(PolicyDecision::Approve) }
        async fn add_policy(&mut self, policy: AutonomyPolicy) -> Result<(), String> { self.policies.insert(policy.id.clone().unwrap(), policy); Ok(()) }
        async fn update_policy(&mut self, policy: AutonomyPolicy) -> Result<(), String> { 
            if let Some(p) = self.policies.get_mut(policy.id.as_ref().unwrap()) { *p = policy; Ok(()) } else { Err("Policy not found".to_string()) }
        }
        async fn remove_policy(&mut self, _policy_id: &str) -> Result<(), String> { Ok(()) }
        async fn get_policy(&self, policy_id: &str) -> Option<&AutonomyPolicy> { self.policies.get(policy_id) }
        async fn list_policies(&self) -> Vec<&AutonomyPolicy> { self.policies.values().collect() }
        async fn set_autonomy_level(&mut self, domain: &str, level: AutonomyLevel) -> Result<(), String> {
            if let Some(policy) = self.policies.values_mut().find(|p| p.domain == domain) {
                policy.autonomy_level = level;
                Ok(())
            } else {
                Err("Policy for domain not found".to_string())
            }
        }
        async fn get_policy_by_domain(&self, domain: &str) -> Option<&AutonomyPolicy> { self.policies.values().find(|p| p.domain == domain) }
    }

    fn setup_services() -> (GovernanceService, mpsc::Receiver<TelemetryEvent>) {
        let config = AGIConfig::default();
        let (tx, rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx);
        let policy_engine = MockAutonomyPolicyEngine::new_mock();
        (GovernanceService::new(config, telemetry_logger, policy_engine), rx)
    }

    #[tokio::test]
    async fn test_propose_and_approve_action() {
        let (mut service, mut rx) = setup_services();
        let action = GovernanceAction {
            action_id: String::new(),
            timestamp: Utc::now(),
            action_type: GovernanceActionType::ChangeAutonomyLevel,
            target_policy_id: None,
            proposed_changes: HashMap::from([
                ("domain".to_string(), "test_domain".to_string()),
                ("new_level".to_string(), "full_auto".to_string()),
            ]),
            justification: "Increased trust in module's capability.".to_string(),
            decision_origin: DecisionOrigin::HumanAdmin,
            status: GovernanceActionStatus::PendingApproval,
            approval_level: Some(1),
        };

        let action_id = service.propose_action(action.clone()).await.unwrap();
        assert!(!action_id.is_empty());

        // Consume propose action telemetry
        rx.recv().await.unwrap(); 

        service.approve_and_execute_action(&action_id, "test_admin").await.unwrap();
        let executed_action = service.active_actions.get(&action_id).unwrap();
        assert_eq!(executed_action.status, GovernanceActionStatus::Executed);

        let policy = service.policy_engine.get_policy_by_domain("test_domain").await.unwrap();
        assert_eq!(policy.autonomy_level, AutonomyLevel::FullAuto);

        let event_approved = rx.recv().await.unwrap();
        match event_approved {
            TelemetryEvent::AdminCommandCompleted { status, .. } => assert_eq!(status, "approved"),
            _ => panic!("Expected AdminCommandCompleted event"),
        }
        let event_autonomy_changed = rx.recv().await.unwrap();
        match event_autonomy_changed {
            TelemetryEvent::AutonomyLevelChanged { affected_domain, old_autonomy_level, new_autonomy_level, .. } => {
                assert_eq!(affected_domain, "test_domain");
                assert_eq!(old_autonomy_level, "SuggestOnly");
                assert_eq!(new_autonomy_level, "FullAuto");
            },
            _ => panic!("Expected AutonomyLevelChanged event"),
        }
    }
}
