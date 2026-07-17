use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use std::thread;

// Represents a critical infrastructure component or data store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrComponent {
    pub id: String,
    pub name: String,
    pub component_type: String, // e.g., "Database", "ComputeCluster", "StorageBucket"
    pub current_status: String, // "Operational", "Impacted", "Recovered"
    pub recovery_priority: u32, // Lower number = higher priority
    pub recovery_step: String,  // Description of how to recover this component
}

// Represents a step within a disaster recovery plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrStep {
    pub step_id: String,
    pub description: String,
    pub expected_duration_seconds: u32,
    pub depends_on: Vec<String>, // IDs of other steps that must complete first
    pub status: String, // "Pending", "InProgress", "Completed", "Failed"
    pub logs: String,   // Execution logs for this step
}

// Represents an overall disaster recovery plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryPlan {
    pub plan_id: String,
    pub name: String,
    pub description: String,
    pub activation_criteria: String, // e.g., "MajorOutage", "DataLossEvent"
    pub components: Vec<DrComponent>, // List of components in scope
    pub steps: Vec<DrStep>,           // Ordered steps for recovery
    pub last_tested: Option<DateTime<Utc>>,
}

// Represents an active disaster recovery incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrIncident {
    pub incident_id: String,
    pub plan_id: String,
    pub activation_time: DateTime<Utc>,
    pub resolution_time: Option<DateTime<Utc>>,
    pub status: String, // "Active", "Recovering", "Resolved", "Failed"
    pub current_step_index: usize,
    pub logs: Vec<String>, // Overall incident logs
}

pub struct DisasterRecoveryOrchestrator {
    dr_plans: Mutex<HashMap<String, DisasterRecoveryPlan>>, // key: plan_id
    active_incidents: Mutex<HashMap<String, DrIncident>>,  // key: incident_id
}

impl DisasterRecoveryOrchestrator {
    pub fn new() -> Self {
        let mut initial_plans = HashMap::new();
        // Example DR Plan
        let plan_id = Uuid::new_v4().to_string();
        let db_component = DrComponent {
            id: Uuid::new_v4().to_string(),
            name: "MainDatabase".to_string(),
            component_type: "Database".to_string(),
            current_status: "Operational".to_string(),
            recovery_priority: 1,
            recovery_step: "Restore from latest backup.".to_string(),
        };
        let app_component = DrComponent {
            id: Uuid::new_v4().to_string(),
            name: "WebAppServers".to_string(),
            component_type: "ComputeCluster".to_string(),
            current_status: "Operational".to_string(),
            recovery_priority: 2,
            recovery_step: "Deploy application to new instances.".to_string(),
        };

        let step1 = DrStep {
            step_id: Uuid::new_v4().to_string(),
            description: format!("Restore {} backup", db_component.name),
            expected_duration_seconds: 600,
            depends_on: vec![],
            status: "Pending".to_string(),
            logs: "".to_string(),
        };
        let step2 = DrStep {
            step_id: Uuid::new_v4().to_string(),
            description: format!("Deploy {}", app_component.name),
            expected_duration_seconds: 300,
            depends_on: vec![step1.step_id.clone()],
            status: "Pending".to_string(),
            logs: "".to_string(),
        };

        let example_plan = DisasterRecoveryPlan {
            plan_id: plan_id.clone(),
            name: "CriticalServiceDR".to_string(),
            description: "Plan to recover core AGI services after major outage.".to_string(),
            activation_criteria: "MajorOutage".to_string(),
            components: vec![db_component, app_component],
            steps: vec![step1, step2],
            last_tested: None,
        };
        initial_plans.insert(plan_id, example_plan);

        DisasterRecoveryOrchestrator {
            dr_plans: Mutex::new(initial_plans),
            active_incidents: Mutex::new(HashMap::new()),
        }
    }

    // Activates a disaster recovery plan
    pub fn activate_dr_plan(&self, plan_id: &str, activation_reason: &str) -> Option<DrIncident> {
        let plans = self.dr_plans.lock().unwrap();
        if let Some(plan) = plans.get(plan_id) {
            println!("DR Orchestrator: Activating DR Plan '{}' due to: {}", plan.name, activation_reason);

            let new_incident = DrIncident {
                incident_id: Uuid::new_v4().to_string(),
                plan_id: plan.plan_id.clone(),
                activation_time: Utc::now(),
                resolution_time: None,
                status: "Active".to_string(),
                current_step_index: 0,
                logs: vec![format!("Plan activated: {}", activation_reason)],
            };
            self.active_incidents.lock().unwrap().insert(new_incident.incident_id.clone(), new_incident.clone());

            // Start executing the first steps (simplified to just log)
            self.execute_next_steps(&new_incident.incident_id);
            Some(new_incident)
        } else {
            println!("DR Orchestrator: DR Plan '{}' not found.", plan_id);
            None
        }
    }

    // Executes the next pending steps in an active incident's plan
    fn execute_next_steps(&self, incident_id: &str) {
        let mut active_incidents = self.active_incidents.lock().unwrap();
        let incident = active_incidents.get_mut(incident_id)?;
        let plans = self.dr_plans.lock().unwrap();
        let plan = plans.get(&incident.plan_id)?;

        // Filter for steps that are pending and have no uncompleted dependencies
        let mut executable_steps: Vec<DrStep> = plan.steps.iter()
            .filter(|step| {
                step.status == "Pending" &&
                step.depends_on.iter().all(|dep_id| {
                    plan.steps.iter().find(|s| &s.step_id == dep_id).map_or(false, |s| s.status == "Completed")
                })
            })
            .cloned()
            .collect();

        // Sort by recovery priority (implicitly from plan.steps order for now)
        for step_to_execute in executable_steps.drain(..) {
            println!("DR Orchestrator: Executing step: '{}'", step_to_execute.description);
            let original_step_index = plan.steps.iter().position(|s| s.step_id == step_to_execute.step_id).unwrap();

            // Simulate execution
            thread::sleep(Duration::from_secs(step_to_execute.expected_duration_seconds as u64 / 10)); // Faster simulation
            let execution_log = format!("Step '{}' completed successfully at {}."; step_to_execute.description, Utc::now());

            // Update step status in the plan (need to clone and re-insert for Mutex<Vec>)
            let mut plan_copy = plan.clone();
            if let Some(s) = plan_copy.steps.get_mut(original_step_index) {
                s.status = "Completed".to_string();
                s.logs = execution_log.clone();
            }
            plans.insert(plan_copy.plan_id.clone(), plan_copy);

            incident.logs.push(execution_log);
            incident.current_step_index = original_step_index + 1; // Advance indicator
            incident.status = "Recovering".to_string(); // Indicate recovery is in progress

            // After completing a step, check if more steps are executable
            self.execute_next_steps(incident_id); // Recursive call
        }

        // Check if all steps are completed
        if plan.steps.iter().all(|s| s.status == "Completed") {
            incident.status = "Resolved".to_string();
            incident.resolution_time = Some(Utc::now());
            incident.logs.push(format!("DR Incident resolved at {}.", Utc::now()));
            println!("DR Orchestrator: Incident {} RESOLVED.", incident_id);
            // Notify Samuel here about resolution
        }
        Some(())
    }

    // Get status of an active incident
    pub fn get_incident_status(&self, incident_id: &str) -> Option<DrIncident> {
        self.active_incidents.lock().unwrap().get(incident_id).cloned()
    }

    pub fn get_all_active_incidents(&self) -> Vec<DrIncident> {
        self.active_incidents.lock().unwrap().values().cloned().collect()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dr_orchestrator_activation_and_recovery() {
        let orchestrator = DisasterRecoveryOrchestrator::new();
        let plan_id = orchestrator.dr_plans.lock().unwrap().keys().next().unwrap().clone();

        // Activate the DR plan
        let incident = orchestrator.activate_dr_plan(&plan_id, "Simulated major datacenter outage");
        assert!(incident.is_some());
        let incident_id = incident.unwrap().incident_id;

        // Simulate some time passing for recovery steps
        thread::sleep(Duration::from_secs(2)); // Allow simulated steps to run

        let final_incident_status = orchestrator.get_incident_status(&incident_id).unwrap();
        println!("Final Incident Status: {:?}", final_incident_status);

        assert_eq!(final_incident_status.status, "Resolved");
        assert!(final_incident_status.resolution_time.is_some());
        assert!(final_incident_status.logs.iter().any(|log| log.contains("Plan activated")));
        assert!(final_incident_status.logs.iter().any(|log| log.contains("Step 'Restore MainDatabase backup' completed")));
        assert!(final_incident_status.logs.iter().any(|log| log.contains("Step 'Deploy WebAppServers' completed")));
        assert!(final_incident_status.logs.iter().any(|log| log.contains("Incident RESOLVED")));
    }
}
*/