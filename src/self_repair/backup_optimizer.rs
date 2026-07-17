use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use rand::Rng; // For simulating random data usage and recovery needs

// Defines different backup types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

// Represents a configured backup schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    pub schedule_id: String,
    pub data_set_id: String,
    pub backup_type: BackupType,
    pub frequency_hours: u32,
    pub retention_days: u32,
    pub storage_location: String, // e.g., "Cloud_Archive", "Local_NAS", "Offsite_Vault"
    pub last_run_success: bool,
}

// Represents a defined restore procedure for a data set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreProcedure {
    pub procedure_id: String,
    pub data_set_id: String,
    pub target_rto_seconds: u32, // Recovery Time Objective
    pub target_rpo_minutes: u32, // Recovery Point Objective
    pub steps_json: String,      // JSON array of detailed restore steps
    pub last_tested_success: bool,
}

// Represents observed usage and recovery needs data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservedData {
    pub observation_id: String,
    pub timestamp: DateTime<Utc>,
    pub data_set_id: String,
    pub read_frequency: f64, // Reads per hour
    pub write_frequency: f64, // Writes per hour
    pub data_size_gb: f64,
    pub historic_rto_met_pct: f64, // Percentage of times RTO was met
    pub historic_rpo_met_pct: f64, // Percentage of times RPO was met
    pub incident_count: u32,       // How many incidents affected this data
}

pub struct BackupOptimizer {
    backup_schedules: Mutex<HashMap<String, BackupSchedule>>,      // key: data_set_id
    restore_procedures: Mutex<HashMap<String, RestoreProcedure>>,  // key: data_set_id
    observed_data_log: Mutex<Vec<ObservedData>>,
}

impl BackupOptimizer {
    pub fn new() -> Self {
        let mut schedules = HashMap::new();
        let mut procedures = HashMap::new();

        let core_memory_id = "AGI_Core_Memory".to_string();
        schedules.insert(core_memory_id.clone(), BackupSchedule {
            schedule_id: Uuid::new_v4().to_string(),
            data_set_id: core_memory_id.clone(),
            backup_type: BackupType::Incremental,
            frequency_hours: 6,
            retention_days: 30,
            storage_location: "Cloud_Archive".to_string(),
            last_run_success: true,
        });

        procedures.insert(core_memory_id.clone(), RestoreProcedure {
            procedure_id: Uuid::new_v4().to_string(),
            data_set_id: core_memory_id.clone(),
            target_rto_seconds: 3600, // 1 hour
            target_rpo_minutes: 10,
            steps_json: "[\"restore_db\", \"verify_data\", \"start_services\"]".to_string(),
            last_tested_success: true,
        });


        BackupOptimizer {
            backup_schedules: Mutex::new(schedules),
            restore_procedures: Mutex::new(procedures),
            observed_data_log: Mutex::new(Vec::new()),
        }
    }

    // Records new observed data about a data set
    pub fn record_observed_data(&self, data: ObservedData) {
        println!("Recording observed data for {}: {:?}", data.data_set_id, data);
        self.observed_data_log.lock().unwrap().push(data);
    }

    // Analyzes observed data and optimizes backup/restore strategies
    pub fn optimize_strategies(&self, data_set_id: &str) -> Option<HashMap<String, String>> {
        println!("Optimizing backup/restore strategies for: {}", data_set_id);

        let mut schedules_locked = self.backup_schedules.lock().unwrap();
        let mut procedures_locked = self.restore_procedures.lock().unwrap();
        let observed_data_locked = self.observed_data_log.lock().unwrap();

        let relevant_obs_data: Vec<&ObservedData> = observed_data_locked.iter()
            .filter(|d| d.data_set_id == data_set_id)
            .collect();

        if relevant_obs_data.is_empty() {
            println!("No observed data for {} to optimize.", data_set_id);
            return None;
        }

        // Get current schedule and procedure
        let current_schedule = schedules_locked.get(data_set_id).cloned()?;
        let current_procedure = procedures_locked.get(data_set_id).cloned()?;

        let mut recommendations = HashMap::new();

        // --- Simulate optimization logic ---
        let mut avg_read_freq = 0.0;
        let mut avg_write_freq = 0.0;
        let mut avg_data_size = 0.0;
        let mut total_incidents = 0;
        for data in &relevant_obs_data {
            avg_read_freq += data.read_frequency;
            avg_write_freq += data.write_frequency;
            avg_data_size += data.data_size_gb;
            total_incidents += data.incident_count;
        }
        avg_read_freq /= relevant_obs_data.len() as f64;
        avg_write_freq /= relevant_obs_data.len() as f64;
        avg_data_size /= relevant_obs_data.len() as f64;

        let mut new_schedule = current_schedule.clone();
        let mut new_procedure = current_procedure.clone();

        // Adjust backup frequency based on write activity and RPO adherence
        if avg_write_freq > 10.0 && current_procedure.target_rpo_minutes > 5 {
            new_schedule.frequency_hours = 1; // More frequent backups for high write loads
            new_procedure.target_rpo_minutes = 5;
            recommendations.insert("backup_frequency".to_string(), format!("Increased backup frequency to 1 hour due to high write activity (avg {} writes/hr) and target RPO.", avg_write_freq));
        } else if avg_write_freq < 1.0 && current_schedule.frequency_hours < 24 {
            new_schedule.frequency_hours = 24; // Less frequent for low write loads
            recommendations.insert("backup_frequency".to_string(), format!("Decreased backup frequency to 24 hours due to low write activity (avg {} writes/hr).", avg_write_freq));
        }

        // Adjust storage location based on data size and recovery needs (simplified)
        if avg_data_size > 500.0 && new_schedule.storage_location != "Geo_Redundant_Cloud_Archive" {
            new_schedule.storage_location = "Geo_Redundant_Cloud_Archive".to_string();
            recommendations.insert("storage_location".to_string(), format!("Moved to Geo-Redundant Cloud Archive due to large data size (avg {} GB).", avg_data_size));
        }

        // Update the schedules and procedures (in a real system, this would be more nuanced with approval workflows)
        schedules_locked.insert(data_set_id.to_string(), new_schedule);
        procedures_locked.insert(data_set_id.to_string(), new_procedure);


        Some(recommendations)
    }

    pub fn get_backup_schedule(&self, data_set_id: &str) -> Option<BackupSchedule> {
        self.backup_schedules.lock().unwrap().get(data_set_id).cloned()
    }

    pub fn get_restore_procedure(&self, data_set_id: &str) -> Option<RestoreProcedure> {
        self.restore_procedures.lock().unwrap().get(data_set_id).cloned()
    }

    pub fn get_all_observed_data(&self) -> Vec<ObservedData> {
        self.observed_data_log.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_backup_optimizer() {
        let optimizer = BackupOptimizer::new();
        let data_set_id = "AGI_Core_Memory".to_string();

        // Record initial usage data (low write frequency)
        optimizer.record_observed_data(ObservedData {
            observation_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            data_set_id: data_set_id.clone(),
            read_frequency: 50.0,
            write_frequency: 0.5,
            data_size_gb: 100.0,
            historic_rto_met_pct: 1.0,
            historic_rpo_met_pct: 1.0,
            incident_count: 0,
        });
        // Optimize (should reduce backup frequency)
        let recs1 = optimizer.optimize_strategies(&data_set_id).unwrap();
        println!("Recommendations 1: {:?}", recs1);
        let schedule1 = optimizer.get_backup_schedule(&data_set_id).unwrap();
        assert_eq!(schedule1.frequency_hours, 24);
        assert!(recs1.contains_key("backup_frequency"));

        // Record high write frequency and large data size
        optimizer.record_observed_data(ObservedData {
            observation_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() + Duration::hours(1),
            data_set_id: data_set_id.clone(),
            read_frequency: 100.0,
            write_frequency: 15.0, // High write activity
            data_size_gb: 600.0, // Large data
            historic_rto_met_pct: 0.9,
            historic_rpo_met_pct: 0.8,
            incident_count: 1,
        });
        // Optimize (should increase frequency and change storage)
        let recs2 = optimizer.optimize_strategies(&data_set_id).unwrap();
        println!("Recommendations 2: {:?}", recs2);
        let schedule2 = optimizer.get_backup_schedule(&data_set_id).unwrap();
        assert_eq!(schedule2.frequency_hours, 1);
        assert_eq!(schedule2.storage_location, "Geo_Redundant_Cloud_Archive");
        assert!(recs2.contains_key("backup_frequency"));
        assert!(recs2.contains_key("storage_location"));

        let procedure2 = optimizer.get_restore_procedure(&data_set_id).unwrap();
        assert_eq!(procedure2.target_rpo_minutes, 5);
    }
}
*/