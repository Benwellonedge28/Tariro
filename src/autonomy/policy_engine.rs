// src/autonomy/policy_engine.rs

//! Implements the core Autonomy Policy Engine for Tariro AGI, responsible for evaluating
//! and enforcing rules that govern Tariro's operational independence and decision-making.

use crate::prelude::*;

use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;

use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};
use crate::database::{DatabaseService, DbEntity};

/// Represents an autonomy policy rule.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutonomyPolicy {
    pub id: Option<String>,
    pub label: String,
    pub domain: String, // e.g., "file_processing", "workflow_creation", "self_modification"
    pub autonomy_level: AutonomyLevel,
    pub evas_filter: bool, // Whether EVAS (Ethical, Values-aligned, Secure) filter is active
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl DbEntity for AutonomyPolicy {
    const COLLECTION_NAME: &'static str = "AutonomyPolicy";
    fn id_as_string(&self) -> Option<String> {
        self.id.clone()
    }
    fn id_as_string_mut(&mut self) -> Option<&mut Option<String>> {
        Some(&mut self.id)
    }
}

/// Defines the level of autonomy Tariro has for a given domain.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, parse_display::FromStr)]
pub enum AutonomyLevel {
    FullAuto,       // Tariro acts independently without human intervention
    SuggestOnly,    // Tariro proposes actions and waits for human approval
    HumanRequired,  // Human must perform the action, Tariro provides guidance
}

impl ToString for AutonomyLevel {
    fn to_string(&self) -> String {
        match self {
            AutonomyLevel::FullAuto => "FullAuto".to_string(),
            AutonomyLevel::SuggestOnly => "SuggestOnly".to_string(),
            AutonomyLevel::HumanRequired => "HumanRequired".to_string(),
        }
    }
}

/// Represents the decision made by the policy engine.
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyDecision {
    Approve,
    RequireHumanApproval { reason: String },
    Block { reason: String },
}

#[async_trait]
pub trait AutonomyPolicyEngineTrait: Send + Sync {
    /// Creates a new `AutonomyPolicyEngine` instance.
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, db_service: Arc<DatabaseService>) -> Self;
    /// Loads the persisted policies and prepares the engine for use.
    async fn initialize(&self) -> Result<(), String>;
    /// Releases the resources held by the engine.
    async fn shutdown(&self) -> Result<(), String>;
    /// Evaluates a proposed action against relevant autonomy policies.
    async fn evaluate_action(&self, action: &str, context: &HashMap<String, String>) -> Result<PolicyDecision, String>;
    /// Adds a new autonomy policy.
    async fn add_policy(&mut self, policy: AutonomyPolicy) -> Result<String, String>;
    /// Updates an existing autonomy policy.
    async fn update_policy(&mut self, policy: AutonomyPolicy) -> Result<(), String>;
    /// Removes an autonomy policy.
    async fn remove_policy(&mut self, policy_id: &str) -> Result<(), String>;
    /// Retrieves a policy by its ID.
    ///
    /// The policy is returned by value: the engine keeps its policies behind
    /// an interior lock, so a borrow cannot outlive the lock guard.
    async fn get_policy(&self, policy_id: &str) -> Option<AutonomyPolicy>;
    /// Retrieves a policy by its domain.
    async fn get_policy_by_domain(&self, domain: &str) -> Option<AutonomyPolicy>;
    /// Lists all currently active policies.
    async fn list_policies(&self) -> Vec<AutonomyPolicy>;
    /// Sets the autonomy level for a specific domain. Creates a new policy if none exists.
    async fn set_autonomy_level(&mut self, domain: &str, level: AutonomyLevel) -> Result<(), String>;
}


pub struct AutonomyPolicyEngine {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    db_service: Arc<DatabaseService>,
    policies: StdMutex<HashMap<String, AutonomyPolicy>>,
}

#[async_trait]
impl AutonomyPolicyEngineTrait for AutonomyPolicyEngine {
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, db_service: Arc<DatabaseService>) -> Self {
        AutonomyPolicyEngine {
            config,
            telemetry_logger,
            db_service,
            policies: StdMutex::new(HashMap::new()),
        }
    }

    async fn initialize(&self) -> Result<(), String> {
        println!("[AutonomyPolicyEngine] Initializing...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceInitialized { service_name: "AutonomyPolicyEngine".to_string() }
        ).await;
        match self.db_service.list::<AutonomyPolicy>().await {
            Ok(loaded_policies) => {
                let mut s_policies = self.policies.lock().unwrap();
                for policy in loaded_policies {
                    if let Some(id) = &policy.id {
                        s_policies.insert(id.clone(), policy);
                    }
                }
                println!("[AutonomyPolicyEngine] Loaded {} policies from DB.", s_policies.len());
            },
            Err(e) => eprintln!("[AutonomyPolicyEngine] Failed to load policies from DB: {}", e),
        }
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), String> {
        println!("[AutonomyPolicyEngine] Shutting down...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceShutdown { service_name: "AutonomyPolicyEngine".to_string() }
        ).await;
        Ok(())
    }

    async fn evaluate_action(&self, action: &str, context: &HashMap<String, String>) -> Result<PolicyDecision, String> {
        let action_domain = context.get("domain").cloned().unwrap_or_else(|| "default".to_string());
        
        let policy = {
            let policies_guard = self.policies.lock().unwrap();
            policies_guard
                .values()
                .find(|p| p.domain == action_domain)
                .cloned()
                .unwrap_or_else(|| AutonomyPolicy {
                    id: None,
                    label: format!("Default Policy for {}", action_domain),
                    domain: action_domain.clone(),
                    autonomy_level: AutonomyLevel::SuggestOnly,
                    evas_filter: true,
                    notes: Some("Automatically generated default policy.".to_string()),
                    created_at: Utc::now(),
                    last_updated: Utc::now(),
                })
        };

        self.telemetry_logger.log_event(TelemetryEvent::AGIAction {
            action_type: "EvaluateAction".to_string(),
            details: Some(format!("Action: {}, Domain: {}, Policy: {:?}", action, action_domain, policy.autonomy_level)),
        }).await;

        if policy.evas_filter {
            if let Some(ethical_concern) = context.get("ethical_concern") {
                if ethical_concern == "true" {
                    return Ok(PolicyDecision::Block { reason: format!("EVAS filter blocked action due to ethical concern: {}", action) });
                }
            }
        }

        match policy.autonomy_level {
            AutonomyLevel::FullAuto => Ok(PolicyDecision::Approve),
            AutonomyLevel::SuggestOnly => Ok(PolicyDecision::RequireHumanApproval {
                reason: format!("Policy for domain '{}' requires human approval for action: {}", action_domain, action),
            }),
            AutonomyLevel::HumanRequired => Ok(PolicyDecision::RequireHumanApproval {
                reason: format!("Policy for domain '{}' requires human to perform action: {}", action_domain, action),
            }),
        }
    }

    async fn add_policy(&mut self, mut policy: AutonomyPolicy) -> Result<String, String> {
        if policy.id.is_none() {
            policy.id = Some(Uuid::new_v4().to_string());
        }
        policy.created_at = Utc::now();
        policy.last_updated = Utc::now();

        let policy_id_clone = policy.id.clone().unwrap();
        if self.policies.lock().unwrap().contains_key(&policy_id_clone) {
            return Err(format!("Policy with ID {} already exists.", policy_id_clone));
        }
        println!("[AutonomyPolicyEngine] Adding new policy: {}", policy.label);
        let inserted_id = self.db_service.create(&mut policy).await?;
        // Ensure the ID from DB is used
        if policy.id.is_none() { policy.id = Some(inserted_id.clone()); }
        self.policies.lock().unwrap().insert(policy.id.clone().unwrap(), policy);
        Ok(inserted_id)
    }

    async fn update_policy(&mut self, mut policy: AutonomyPolicy) -> Result<(), String> {
        let id = match &policy.id {
            Some(id) => id.clone(),
            None => return Err("Policy to update must have an ID.".to_string()),
        };

        // The policy is written while the lock is held, and the label is read
        // before the lock is released for the database call.
        let existing_label = {
            let mut policies_guard = self.policies.lock().unwrap();
            match policies_guard.get_mut(&id) {
                Some(existing_policy) => {
                    policy.last_updated = Utc::now();
                    *existing_policy = policy.clone();
                    Some(existing_policy.label.clone())
                }
                None => None,
            }
        };

        let existing_label = match existing_label {
            Some(label) => label,
            None => return Err(format!("Policy with ID {} not found.", id)),
        };

        self.db_service.update(&id, &policy).await?;
        println!("[AutonomyPolicyEngine] Updated policy: {}", existing_label);
        self.telemetry_logger
            .log_event(TelemetryEvent::PolicyUpdated {
                policy_id: id,
                action: "update".to_string(),
            })
            .await;
        Ok(())
    }

    async fn remove_policy(&mut self, policy_id: &str) -> Result<(), String> {
        if self.policies.lock().unwrap().remove(policy_id).is_some() {
            self.db_service.delete::<AutonomyPolicy>(policy_id).await?;
            println!("[AutonomyPolicyEngine] Removed policy: {}", policy_id);
            self.telemetry_logger.log_event(TelemetryEvent::PolicyUpdated {
                policy_id: policy_id.to_string(),
                action: "remove".to_string(),
            }).await;
            Ok(())
        } else {
            Err(format!("Policy with ID {} not found.", policy_id))
        }
    }

    async fn get_policy(&self, policy_id: &str) -> Option<AutonomyPolicy> {
        let policies = self.policies.lock().unwrap();
        policies.get(policy_id).cloned()
    }

    async fn get_policy_by_domain(&self, domain: &str) -> Option<AutonomyPolicy> {
        let policies = self.policies.lock().unwrap();
        policies.values().find(|p| p.domain == domain).cloned()
    }

    async fn list_policies(&self) -> Vec<AutonomyPolicy> {
        self.policies.lock().unwrap().values().cloned().collect()
    }

    async fn set_autonomy_level(&mut self, domain: &str, level: AutonomyLevel) -> Result<(), String> {
        // The policy is read and written while the lock is held, and copied
        // before the lock is released for the database call.
        let updated = {
            let mut policies_guard = self.policies.lock().unwrap();
            let mut found_policy_id = None;
            let mut updated = None;
            if let Some(policy) = policies_guard.values_mut().find(|p| p.domain == domain) {
                let old_level = policy.autonomy_level.to_string();
                policy.autonomy_level = level.clone();
                policy.last_updated = Utc::now();
                found_policy_id = policy.id.clone();
                updated = Some((old_level, policy.clone()));
            }
            let _ = found_policy_id;
            updated
        };

        if let Some((old_level, updated_policy)) = updated {
            if let Some(id) = updated_policy.id.as_ref() {
                self.db_service.update(id, &updated_policy).await?;
            }
            println!("[AutonomyPolicyEngine] Autonomy level for domain '{}' updated to {:?}", domain, level);
            self.telemetry_logger.log_event(TelemetryEvent::AutonomyLevelChanged {
                affected_domain: domain.to_string(),
                old_autonomy_level: old_level,
                new_autonomy_level: level.to_string(),
                reason: "Admin/Self-governance command.".to_string(),
                triggered_by_action_id: Uuid::new_v4().to_string(), // Placeholder
            }).await;
            Ok(())
        } else {
            // Create a new policy for this domain
            let mut new_policy = AutonomyPolicy {
                id: None,
                label: format!("Autonomy Policy for {}", domain),
                domain: domain.to_string(),
                autonomy_level: level.clone(),
                evas_filter: true, // Default to active
                notes: Some("Automatically created by set_autonomy_level.".to_string()),
                created_at: Utc::now(),
                last_updated: Utc::now(),
            };
            let new_policy_id = self.add_policy(new_policy.clone()).await?;
            new_policy.id = Some(new_policy_id);
            println!("[AutonomyPolicyEngine] New autonomy policy created for domain '{}' with level {:?}", domain, level);
            self.telemetry_logger.log_event(TelemetryEvent::AutonomyLevelChanged {
                affected_domain: domain.to_string(),
                old_autonomy_level: "None".to_string(),
                new_autonomy_level: level.to_string(),
                reason: "New policy created.".to_string(),
                triggered_by_action_id: Uuid::new_v4().to_string(), // Placeholder
            }).await;
            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use crate::config::AGIConfig;
    use crate::database::DatabaseService;
    use std::sync::Arc;

    // Mock DatabaseService
    struct MockDbService {
        policies: StdMutex<HashMap<String, AutonomyPolicy>>,
    }

    #[async_trait]
    impl DatabaseService for MockDbService {
        async fn new(_db_uri: &str, _db_name: &str) -> Result<Self, String> { Ok(MockDbService { policies: StdMutex::new(HashMap::new()) }) }
        async fn create<T: crate::database::DbEntity + Debug + Send>(&self, entity: &mut T) -> Result<String, String> {
            let policy = (entity as &dyn std::any::Any).downcast_ref::<AutonomyPolicy>().unwrap().clone();
            let id = policy.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
            self.policies.lock().unwrap().insert(id.clone(), policy);
            Ok(id)
        }
        async fn read<T: crate::database::DbEntity + Debug>(&self, id: &str) -> Result<Option<T>, String> {
            if let Some(policy) = self.policies.lock().unwrap().get(id) {
                let boxed: Box<dyn std::any::Any> = Box::new(policy.clone());
                Ok(Some(*boxed.downcast::<T>().unwrap()))
            } else {
                Ok(None)
            }
        }
        async fn update<T: crate::database::DbEntity + Debug>(&self, id: &str, new_data: &T) -> Result<(), String> {
            let mut policies = self.policies.lock().unwrap();
            if let Some(policy) = policies.get_mut(id) {
                *policy = (new_data as &dyn std::any::Any).downcast_ref::<AutonomyPolicy>().unwrap().clone();
                Ok(())
            } else {
                Err(format!("Policy {} not found in mock DB", id))
            }
        }
        async fn delete<T: crate::database::DbEntity + Debug>(&self, id: &str) -> Result<(), String> {
            self.policies.lock().unwrap().remove(id);
            Ok(())
        }
        async fn list<T: crate::database::DbEntity + Debug>(&self) -> Result<Vec<T>, String> {
            let policies_vec: Vec<T> = self.policies.lock().unwrap().values().cloned().map(|p| {
                let boxed: Box<dyn std::any::Any> = Box::new(p);
                *boxed.downcast::<T>().unwrap()
            }).collect();
            Ok(policies_vec)
        }
    }

    fn setup_services() -> (AutonomyPolicyEngine, mpsc::Receiver<TelemetryEvent>, Arc<MockDbService>) {
        let config = AGIConfig::default();
        let (tx, rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx);
        let db_service = Arc::new(MockDbService::new("", "").unwrap());
        (AutonomyPolicyEngine::new(config, telemetry_logger, db_service.clone()), rx, db_service)
    }

    #[tokio::test]
    async fn test_initialize_and_load_policies() {
        let (service, _rx, db_service_arc) = setup_services();
        let mut policy1 = AutonomyPolicy {
            id: Some("policy1".to_string()), label: "Policy One".to_string(), domain: "test.domain1".to_string(),
            autonomy_level: AutonomyLevel::FullAuto, evas_filter: true, notes: None,
            created_at: Utc::now(), last_updated: Utc::now(),
        };
        let mut policy2 = AutonomyPolicy {
            id: Some("policy2".to_string()), label: "Policy Two".to_string(), domain: "test.domain2".to_string(),
            autonomy_level: AutonomyLevel::SuggestOnly, evas_filter: true, notes: None,
            created_at: Utc::now(), last_updated: Utc::now(),
        };
        db_service_arc.create(&mut policy1).await.unwrap();
        db_service_arc.create(&mut policy2).await.unwrap();

        service.initialize().await.unwrap();
        assert_eq!(service.policies.lock().unwrap().len(), 2);
        assert!(service.policies.lock().unwrap().contains_key("policy1"));
    }

    #[tokio::test]
    async fn test_evaluate_action() {
        let (mut service, _rx, db_service_arc) = setup_services();
        let mut policy = AutonomyPolicy {
            id: Some("eval_policy_id".to_string()), label: "Eval Policy".to_string(), domain: "test.evaluation".to_string(),
            autonomy_level: AutonomyLevel::SuggestOnly, evas_filter: true, notes: None,
            created_at: Utc::now(), last_updated: Utc::now(),
        };
        service.add_policy(policy.clone()).await.unwrap();

        let mut context = HashMap::new();
        context.insert("domain".to_string(), "test.evaluation".to_string());
        
        let decision = service.evaluate_action("perform_critical_task", &context).await.unwrap();
        assert!(matches!(decision, PolicyDecision::RequireHumanApproval { .. }));

        // Test full auto
        policy.autonomy_level = AutonomyLevel::FullAuto;
        service.update_policy(policy.clone()).await.unwrap();
        let decision_full_auto = service.evaluate_action("perform_critical_task", &context).await.unwrap();
        assert_eq!(decision_full_auto, PolicyDecision::Approve);

        // Test EVAS block
        context.insert("ethical_concern".to_string(), "true".to_string());
        let decision_evas_block = service.evaluate_action("perform_critical_task", &context).await.unwrap();
        assert!(matches!(decision_evas_block, PolicyDecision::Block { .. }));
    }

    #[tokio::test]
    async fn test_set_autonomy_level_existing_policy() {
        let (mut service, mut rx, _db) = setup_services();
        let policy_id = service.add_policy(AutonomyPolicy {
            id: None, label: "Existing Policy".to_string(), domain: "existing.domain".to_string(),
            autonomy_level: AutonomyLevel::SuggestOnly, evas_filter: true, notes: None,
            created_at: Utc::now(), last_updated: Utc::now(),
        }).await.unwrap();

        service.set_autonomy_level("existing.domain", AutonomyLevel::FullAuto).await.unwrap();
        let updated_policy = service.get_policy(&policy_id).await.unwrap();
        assert_eq!(updated_policy.autonomy_level, AutonomyLevel::FullAuto);

        let _ = rx.recv().await.unwrap(); // consume AGIAction event
        let event = rx.recv().await.unwrap(); // consume AutonomyLevelChanged event
        match event {
            TelemetryEvent::AutonomyLevelChanged { affected_domain, old_autonomy_level, new_autonomy_level, .. } => {
                assert_eq!(affected_domain, "existing.domain");
                assert_eq!(old_autonomy_level, "SuggestOnly");
                assert_eq!(new_autonomy_level, "FullAuto");
            },
            _ => panic!("Expected AutonomyLevelChanged event"),
        }
    }

    #[tokio::test]
    async fn test_set_autonomy_level_new_policy() {
        let (mut service, mut rx, _db) = setup_services();
        // Assuming "new.domain" does not exist
        service.set_autonomy_level("new.domain", AutonomyLevel::HumanRequired).await.unwrap();
        let new_policy = service.get_policy_by_domain("new.domain").await.unwrap();
        assert_eq!(new_policy.autonomy_level, AutonomyLevel::HumanRequired);
        
        let _ = rx.recv().await.unwrap(); // consume AGIAction event
        let event = rx.recv().await.unwrap(); // consume AutonomyLevelChanged event
        match event {
            TelemetryEvent::AutonomyLevelChanged { affected_domain, old_autonomy_level, new_autonomy_level, .. } => {
                assert_eq!(affected_domain, "new.domain");
                assert_eq!(old_autonomy_level, "None");
                assert_eq!(new_autonomy_level, "HumanRequired");
            },
            _ => panic!("Expected AutonomyLevelChanged event"),
        }
    }
}
