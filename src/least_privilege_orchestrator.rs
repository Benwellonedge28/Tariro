/// Module for Dynamic Least Privilege Orchestrator in Rust.
///
/// This module conceptually outlines a Rust-based implementation for a Dynamic Least Privilege Orchestrator.
/// This module will continuously assess and adjust permissions for all entities (users, agents, services)
/// to grant only the minimum necessary access at any given time (leveraging RAG for policy enforcement rules
/// and SIMD for real-time access matrix evaluation).

use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;

/// Types of entities that require access control.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    User,
    Agent,
    Service,
    System,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents a single permission, granting an action on a resource under certain conditions.
#[derive(Debug, Clone, PartialEq)]
pub struct Permission {
    pub action: String,    // e.g., "read", "write", "execute"
    pub resource: String,  // e.g., "file:/data/logs", "db:users_table", "service:api_endpoint"
    pub condition: Option<String>, // e.g., "time_of_day:9-5", "ip_range:192.168.1.0/24"
}

/// Represents an attempt by an entity to access a resource.
#[derive(Debug, Clone)]
pub struct AccessRequest {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub requested_action: String,
    pub requested_resource: String,
    pub context: HashMap<String, String>, // e.g., "current_time": "10:30", "source_ip":"1.1.1.1"
}

/// Represents an entity's current set of privileges.
#[derive(Debug, Clone)]
pub struct PrivilegeProfile {
    pub entity_id: String,
    pub entity_type: EntityType,
    pub current_permissions: Vec<Permission>,
    pub last_evaluated: u64,
}

/// The Dynamic Least Privilege Orchestrator.
pub struct LeastPrivilegeOrchestrator {
    pub entity_profiles: HashMap<String, PrivilegeProfile>,
    pub policy_rules_db: HashMap<String, String>, // Conceptual: RAG for policy enforcement rules
    pub audit_log: Vec<String>,
}

impl LeastPrivilegeOrchestrator {
    pub fn new() -> Self {
        let mut policy_db = HashMap::new();
        policy_db.insert("admin_policy".to_string(), "Admin users can do anything.".to_string());
        policy_db.insert("service_account_policy".to_string(), "Services need minimal read/write on their own data.".to_string());

        LeastPrivilegeOrchestrator {
            entity_profiles: HashMap::new(),
            policy_rules_db: policy_db,
            audit_log: Vec::new(),
        }
    }

    /// Registers a new entity (user, agent, service) with initial default permissions.
    pub fn register_entity(&mut self, entity_id: String, entity_type: EntityType, initial_permissions: Vec<Permission>) {
        println!("  [LPO]: Registering entity '{}' as {:?}.", entity_id, entity_type);
        let profile = PrivilegeProfile {
            entity_id: entity_id.clone(),
            entity_type,
            current_permissions: initial_permissions,
            last_evaluated: Instant::now().elapsed().as_secs(),
        };
        self.entity_profiles.insert(entity_id, profile);
        thread::sleep(Duration::from_millis(30));
    }

    /// Evaluates an access request against current permissions and policies (conceptual SIMD).
    pub fn evaluate_access_request_simd(&self, request: &AccessRequest) -> bool {
        println!("  [LPO]: Evaluating access request for '{}' to {} {} (SIMD simulation).",
                 request.entity_id, request.requested_action, request.requested_resource);
        let mut rng = rand::thread_rng();

        if let Some(profile) = self.entity_profiles.get(&request.entity_id) {
            for p in &profile.current_permissions {
                // Simplified check: actual SIMD would evaluate many rules/conditions in parallel
                if p.action == request.requested_action && p.resource == request.requested_resource {
                    if p.condition.is_none() || (p.condition.is_some() && rng.gen_bool(0.7)) { // Simulate condition check
                        self.audit_log.push(format!("Access ALLOWED for {} to {} {}.", request.entity_id, request.requested_action, request.requested_resource));
                        thread::sleep(Duration::from_millis(5));
                        return true;
                    }
                }
            }
        }
        self.audit_log.push(format!("Access DENIED for {} to {} {}.", request.entity_id, request.requested_action, request.requested_resource));
        thread::sleep(Duration::from_millis(10));
        false
    }

    /// Enforces policy rules based on external RAG knowledge base.
    pub fn enforce_policy_rag(&self, entity_id: &str) {
        println!("  [LPO]: Enforcing policy for '{}' (RAG simulation).", entity_id);
        // Simulate fetching relevant policies from RAG based on entity type or role
        if let Some(profile) = self.entity_profiles.get(entity_id) {
            if let Some(policy_content) = self.policy_rules_db.get(&format!("{}_policy", profile.entity_type.to_string().to_lowercase())) {
                println!("    -> RAG identified policy for {:?}: '{}'
", profile.entity_type, policy_content);
            }
        }
        thread::sleep(Duration::from_millis(70));
    }

    /// Proposes permission adjustments to ensure least privilege.
    pub fn propose_permission_adjustments(&self) -> HashMap<String, Vec<Permission>> {
        println!("  [LPO]: Proposing permission adjustments.");
        let mut proposed_changes = HashMap::new();
        let mut rng = rand::thread_rng();

        // Simulate identifying over-privileged entities or roles
        for (entity_id, profile) in &self.entity_profiles {
            if profile.entity_type == EntityType::User && profile.current_permissions.len() > 3 && rng.gen_bool(0.4) {
                // Example: User has too many permissions, suggest removal
                let mut adjusted_perms = profile.current_permissions.clone();
                if let Some(index) = adjusted_perms.iter().position(|p| p.resource.contains("admin")) {
                    adjusted_perms.remove(index);
                    println!("    -> Proposing to revoke 'admin' access for user '{}'.", entity_id);
                    proposed_changes.insert(entity_id.clone(), adjusted_perms);
                }
            }
        }
        thread::sleep(Duration::from_millis(100));
        proposed_changes
    }

    /// Applies proposed permission changes.
    pub fn apply_permission_changes(&mut self, adjustments: HashMap<String, Vec<Permission>>) {
        println!("  [LPO]: Applying {} permission adjustments.", adjustments.len());
        for (entity_id, new_permissions) in adjustments {
            if let Some(profile) = self.entity_profiles.get_mut(&entity_id) {
                profile.current_permissions = new_permissions;
                profile.last_evaluated = Instant::now().elapsed().as_secs();
                self.audit_log.push(format!("Permissions updated for {}. New count: {}.", entity_id, profile.current_permissions.len()));
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_privilege_flow() {
        let mut orchestrator = LeastPrivilegeOrchestrator::new();
        orchestrator.register_entity("AdminUser".to_string(), EntityType::User, vec![
            Permission { action: "read".to_string(), resource: "file:/data/logs".to_string(), condition: None },
            Permission { action: "write".to_string(), resource: "db:users_table".to_string(), condition: None },
            Permission { action: "execute".to_string(), resource: "service:admin_api".to_string(), condition: None },
        ]);

        let request = AccessRequest {
            entity_id: "AdminUser".to_string(), entity_type: EntityType::User,
            requested_action: "read".to_string(), requested_resource: "file:/data/logs".to_string(), context: HashMap::new(),
        };
        assert!(orchestrator.evaluate_access_request_simd(&request));

        orchestrator.enforce_policy_rag("AdminUser");

        let adjustments = orchestrator.propose_permission_adjustments();
        orchestrator.apply_permission_changes(adjustments);

        // Test access after potential adjustment
        let post_adjustment_request = AccessRequest {
            entity_id: "AdminUser".to_string(), entity_type: EntityType::User,
            requested_action: "execute".to_string(), requested_resource: "service:admin_api".to_string(), context: HashMap::new(),
        };
        // May be true or false depending on random adjustment in test
        orchestrator.evaluate_access_request_simd(&post_adjustment_request);
    }
}

fn main() {
    println!("\n--- Dynamic Least Privilege Orchestrator Demonstration ---");

    let mut orchestrator = LeastPrivilegeOrchestrator::new();

    // 1. Register various entities with their initial (potentially over-privileged) roles
    orchestrator.register_entity("AnalyticsService".to_string(), EntityType::Service, vec![
        Permission { action: "read".to_string(), resource: "db:analytics_data".to_string(), condition: None },
        Permission { action: "write".to_string(), resource: "db:raw_logs".to_string(), condition: None },
        Permission { action: "read".to_string(), resource: "file:/secrets/api_keys".to_string(), condition: None }, // Over-privileged
    ]);

    orchestrator.register_entity("AuditorUser".to_string(), EntityType::User, vec![
        Permission { action: "read".to_string(), resource: "file:/audit_reports".to_string(), condition: None },
        Permission { action: "read".to_string(), resource: "db:all_user_data".to_string(), condition: None }, // Over-privileged
    ]);

    orchestrator.register_entity("DASA_Agent_001".to_string(), EntityType::Agent, vec![
        Permission { action: "read".to_string(), resource: "network:traffic_logs".to_string(), condition: None },
        Permission { action: "block".to_string(), resource: "network:ip_address".to_string(), condition: Some("threat_level:>0.8".to_string()) },
    ]);

    // Simulate several cycles of access requests and privilege adjustments
    for i in 0..3 {
        println!("\n========================== LPO Cycle {} ==========================", i + 1);

        let current_time = Instant::now().elapsed().as_secs();
        let mut rng = rand::thread_rng();

        // 2. Simulate access requests
        let requests = vec![
            AccessRequest {
                entity_id: "AnalyticsService".to_string(), entity_type: EntityType::Service,
                requested_action: "read".to_string(), requested_resource: "db:analytics_data".to_string(), context: HashMap::new(),
            },
            AccessRequest {
                entity_id: "AnalyticsService".to_string(), entity_type: EntityType::Service,
                requested_action: "read".to_string(), requested_resource: "file:/secrets/api_keys".to_string(), context: HashMap::new(), // Should be denied eventually
            },
            AccessRequest {
                entity_id: "AuditorUser".to_string(), entity_type: EntityType::User,
                requested_action: "read".to_string(), requested_resource: "db:all_user_data".to_string(), context: HashMap::new(),
            },
            AccessRequest {
                entity_id: "DASA_Agent_001".to_string(), entity_type: EntityType::Agent,
                requested_action: "block".to_string(), requested_resource: "network:ip_address".to_string(), context: HashMap::from([("threat_level".to_string(), "0.9".to_string())]),
            },
        ];

        for req in requests {
            orchestrator.evaluate_access_request_simd(&req);
        }

        // 3. Enforce RAG policies periodically
        if i % 2 == 0 {
            orchestrator.enforce_policy_rag("AnalyticsService");
            orchestrator.enforce_policy_rag("AuditorUser");
        }

        // 4. Propose and apply permission adjustments
        let proposed_adjustments = orchestrator.propose_permission_adjustments();
        orchestrator.apply_permission_changes(proposed_adjustments);

        println!("  Current Entity Permissions:");
        for profile in orchestrator.entity_profiles.values() {
            println!("    - {}: {} permissions.", profile.entity_id, profile.current_permissions.len());
        }

        thread::sleep(Duration::from_secs(1));
    }

    println!("\n--- Audit Log ---");
    for entry in &orchestrator.audit_log {
        println!("  {}", entry);
    }

    println!("\nDynamic Least Privilege Orchestrator Demonstration Complete.");
}
