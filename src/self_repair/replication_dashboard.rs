use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents the status of a single data replica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicaStatus {
    pub replica_id: String,
    pub location: String, // e.g., "Region-A/Node-1", "Cloud-AZ-East"
    pub last_sync_time: DateTime<Utc>,
    pub lag_seconds: i64, // How far behind the primary this replica is
    pub health_status: String, // e.g., "Healthy", "Degraded", "Offline"
    pub is_primary: bool,
}

// Represents the overall replication status for a critical data set or system state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatus {
    pub data_set_id: String,
    pub data_set_name: String, // e.g., "AGI_Core_Memory", "User_Profiles", "Configuration_Store"
    pub total_replicas: usize,
    pub active_replicas: usize,
    pub min_required_replicas: usize,
    pub consistency_model: String, // e.g., "Strong", "Eventual", "Causal"
    pub overall_health: String, // "Optimal", "Warning", "Critical"
    pub replicas: Vec<ReplicaStatus>,
    pub last_updated: DateTime<Utc>,
}

pub struct ReplicationDashboard {
    replication_data: Mutex<HashMap<String, ReplicationStatus>>, // key: data_set_id
}

impl ReplicationDashboard {
    pub fn new() -> Self {
        ReplicationDashboard {
            replication_data: Mutex::new(HashMap::new()),
        }
    }

    // Updates the replication status for a given data set
    pub fn update_replication_status(&self, status: ReplicationStatus) {
        println!("Updating replication status for data set: {}", status.data_set_name);
        self.replication_data.lock().unwrap().insert(status.data_set_id.clone(), status);
    }

    // Retrieves the current replication status for a data set
    pub fn get_replication_status(&self, data_set_id: &str) -> Option<ReplicationStatus> {
        self.replication_data.lock().unwrap().get(data_set_id).cloned()
    }

    // Displays an aggregated view of all critical replication statuses
    pub fn display_dashboard_summary(&self) -> Vec<HashMap<String, String>> {
        println!("Displaying replication dashboard summary...");
        let data = self.replication_data.lock().unwrap();
        data.values()
            .map(|s| {
                HashMap::from([
                    ("data_set_name".to_string(), s.data_set_name.clone()),
                    ("overall_health".to_string(), s.overall_health.clone()),
                    ("active_replicas".to_string(), s.active_replicas.to_string()),
                    ("total_replicas".to_string(), s.total_replicas.to_string()),
                    ("min_required_replicas".to_string(), s.min_required_replicas.to_string()),
                    ("consistency_model".to_string(), s.consistency_model.clone()),
                    ("last_updated".to_string(), s.last_updated.to_rfc3339()),
                ])
            })
            .collect()
    }

    // Provides detailed view for a specific data set
    pub fn display_detailed_status(&self, data_set_id: &str) -> Option<HashMap<String, String>> {
        if let Some(status) = self.get_replication_status(data_set_id) {
            println!("Displaying detailed status for: {}", status.data_set_name);
            let mut details = HashMap::new();
            details.insert("data_set_name".to_string(), status.data_set_name.clone());
            details.insert("overall_health".to_string(), status.overall_health.clone());
            details.insert("consistency_model".to_string(), status.consistency_model.clone());
            details.insert("min_required".to_string(), status.min_required_replicas.to_string());
            details.insert("replica_breakdown".to_string(), format!("{:?}", status.replicas));
            Some(details)
        } else {
            None
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_replication_dashboard() {
        let dashboard = ReplicationDashboard::new();

        // Simulate status for AGI Core Memory
        let agi_memory_id = Uuid::new_v4().to_string();
        let mut agi_replicas = Vec::new();
        agi_replicas.push(ReplicaStatus {
            replica_id: Uuid::new_v4().to_string(),
            location: "Cloud-AZ-West".to_string(),
            last_sync_time: Utc::now() - Duration::seconds(1),
            lag_seconds: 1,
            health_status: "Healthy".to_string(),
            is_primary: true,
        });
        agi_replicas.push(ReplicaStatus {
            replica_id: Uuid::new_v4().to_string(),
            location: "Cloud-AZ-East".to_string(),
            last_sync_time: Utc::now() - Duration::seconds(2),
            lag_seconds: 2,
            health_status: "Healthy".to_string(),
            is_primary: false,
        });
        agi_replicas.push(ReplicaStatus {
            replica_id: Uuid::new_v4().to_string(),
            location: "Local-Node-1".to_string(),
            last_sync_time: Utc::now() - Duration::minutes(1), // Lagging
            lag_seconds: 60,
            health_status: "Degraded".to_string(),
            is_primary: false,
        });

        let agi_status = ReplicationStatus {
            data_set_id: agi_memory_id.clone(),
            data_set_name: "AGI_Core_Memory".to_string(),
            total_replicas: 3,
            active_replicas: 2, // One degraded
            min_required_replicas: 2,
            consistency_model: "Strong".to_string(),
            overall_health: "Warning".to_string(),
            replicas: agi_replicas,
            last_updated: Utc::now(),
        };
        dashboard.update_replication_status(agi_status);

        // Simulate status for User Profiles (all healthy)
        let user_profiles_id = Uuid::new_v4().to_string();
        let mut user_replicas = Vec::new();
        user_replicas.push(ReplicaStatus {
            replica_id: Uuid::new_v4().to_string(),
            location: "Cloud-AZ-Central".to_string(),
            last_sync_time: Utc::now() - Duration::milliseconds(500),
            lag_seconds: 0,
            health_status: "Healthy".to_string(),
            is_primary: true,
        });
        user_replicas.push(ReplicaStatus {
            replica_id: Uuid::new_v4().to_string(),
            location: "On-Prem-Backup".to_string(),
            last_sync_time: Utc::now() - Duration::seconds(5),
            lag_seconds: 5,
            health_status: "Healthy".to_string(),
            is_primary: false,
        });
        let user_status = ReplicationStatus {
            data_set_id: user_profiles_id.clone(),
            data_set_name: "User_Profiles".to_string(),
            total_replicas: 2,
            active_replicas: 2,
            min_required_replicas: 2,
            consistency_model: "Eventual".to_string(),
            overall_health: "Optimal".to_string(),
            replicas: user_replicas,
            last_updated: Utc::now(),
        };
        dashboard.update_replication_status(user_status);

        // Test dashboard summary
        let summary = dashboard.display_dashboard_summary();
        println!("Summary: {:?}", summary);
        assert_eq!(summary.len(), 2);
        assert!(summary.iter().any(|s| s["overall_health"] == "Warning"));
        assert!(summary.iter().any(|s| s["overall_health"] == "Optimal"));

        // Test detailed status
        let agi_details = dashboard.display_detailed_status(&agi_memory_id);
        println!("AGI Details: {:?}", agi_details);
        assert!(agi_details.is_some());
        assert!(agi_details.unwrap()["replica_breakdown"].contains("Degraded"));
    }
}
*/