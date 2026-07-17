use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents data from a physical security sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalSensorData {
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub sensor_type: String, // e.g., "CCTV", "Microphone", "MotionSensor", "AccessControl"
    pub location: String,
    pub event_description: String, // e.g., "Motion detected", "Loud noise", "Unauthorized access attempt"
    pub associated_media_url: Option<String>, // URL to video clip or audio recording
    pub severity: String, // "Low", "Medium", "High", "Critical"
}

// Represents relevant digital threat intelligence (e.g., from Fusion Center)
// Re-using simplified `ThreatAlert` from fusion_center.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalThreatData {
    pub alert_id: String,
    pub timestamp: DateTime<Utc>,
    pub overall_severity: String,
    pub alert_type: String, // e.g., "ZeroDayExploit", "TargetedAttack", "Campaign"
    pub summary: String,
    pub affected_systems: Vec<String>, // Systems potentially affected within Tariro
    pub source_ip: Option<String>, // IP address of attacker/origin
}

// Represents a unified threat event, correlating physical and digital data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedThreatEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub correlated_physical_data_ids: Vec<String>, // IDs of PhysicalSensorData
    pub correlated_digital_data_ids: Vec<String>, // IDs of DigitalThreatData
    pub overall_threat_level: String, // "Low", "Medium", "High", "Critical"
    pub event_summary: String,
    pub identified_threat_actors: Vec<String>, // Potential threat actors (from Counter-Intel)
    pub recommended_response: String,
}

pub struct OmniSensorIntegrator {
    physical_sensor_log: Mutex<Vec<PhysicalSensorData>>,
    digital_threat_log: Mutex<Vec<DigitalThreatData>>,
    unified_threat_events_log: Mutex<Vec<UnifiedThreatEvent>>,
}

impl OmniSensorIntegrator {
    pub fn new() -> Self {
        OmniSensorIntegrator {
            physical_sensor_log: Mutex::new(Vec::new()),
            digital_threat_log: Mutex::new(Vec::new()),
            unified_threat_events_log: Mutex::new(Vec::new()),
        }
    }

    // Ingests new physical sensor data
    pub fn ingest_physical_data(&self, data: PhysicalSensorData) {
        println!("Omni-Sensor Integrator: Ingested physical sensor data: {}", data.sensor_type);
        self.physical_sensor_log.lock().unwrap().push(data);
    }

    // Ingests new digital threat data
    pub fn ingest_digital_data(&self, data: DigitalThreatData) {
        println!("Omni-Sensor Integrator: Ingested digital threat data: {}", data.alert_type);
        self.digital_threat_log.lock().unwrap().push(data);
    }

    // Integrates and correlates physical and digital data for comprehensive security awareness
    pub fn correlate_and_generate_events(&self) -> Vec<UnifiedThreatEvent> {
        println!("Omni-Sensor Integrator: Correlating physical and digital data...");

        let physical_data_locked = self.physical_sensor_log.lock().unwrap();
        let digital_data_locked = self.digital_threat_log.lock().unwrap();
        let mut new_events = Vec::new();

        // --- Correlation Logic (Simplified) ---
        // Scenario: A motion sensor detected movement, and simultaneously, a digital alert
        // indicated a potential breach attempt from an IP associated with a physical location.
        for phys_data in physical_data_locked.iter() {
            if phys_data.event_description.contains("Motion detected") || phys_data.event_description.contains("Unauthorized access attempt") {
                for dig_data in digital_data_locked.iter() {
                    // Simulate correlation based on proximity in time and potential spatial link (simplified)
                    if (phys_data.timestamp - dig_data.timestamp).num_minutes().abs() < 5 &&
                       dig_data.affected_systems.iter().any(|s| phys_data.location.contains(s)) {
                        let event = UnifiedThreatEvent {
                            event_id: Uuid::new_v4().to_string(),
                            timestamp: Utc::now(),
                            correlated_physical_data_ids: vec![phys_data.sensor_id.clone()],
                            correlated_digital_data_ids: vec![dig_data.alert_id.clone()],
                            overall_threat_level: "Critical".to_string(),
                            event_summary: format!("Physical intrusion detected at {} correlated with digital breach attempt targeting affected system: {}. Threat type: {}.",
                                                  phys_data.location, dig_data.affected_systems.join(", "), dig_data.alert_type),
                            identified_threat_actors: vec![], // This would come from Counter-Intel
                            recommended_response: format!("Lockdown physical location: {}. Isolate affected digital systems: {:?}. Alert security personnel.",
                                                          phys_data.location, dig_data.affected_systems),
                        };
                        new_events.push(event);
                    }
                }
            }
        }

        self.unified_threat_events_log.lock().unwrap().extend(new_events.clone());
        println!("Omni-Sensor Integrator: Generated {} new unified threat events.", new_events.len());
        new_events
    }

    pub fn get_physical_sensor_log(&self) -> Vec<PhysicalSensorData> {
        self.physical_sensor_log.lock().unwrap().clone()
    }

    pub fn get_digital_threat_log(&self) -> Vec<DigitalThreatData> {
        self.digital_threat_log.lock().unwrap().clone()
    }

    pub fn get_unified_threat_events_log(&self) -> Vec<UnifiedThreatEvent> {
        self.unified_threat_events_log.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_omni_modal_sensor_integration() {
        let integrator = OmniSensorIntegrator::new();

        // Ingest some physical sensor data
        let phys_data1 = PhysicalSensorData {
            sensor_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::minutes(2),
            sensor_type: "MotionSensor".to_string(),
            location: "ServerRoom_A".to_string(),
            event_description: "Motion detected near primary server rack.".to_string(),
            associated_media_url: None,
            severity: "Medium".to_string(),
        };
        integrator.ingest_physical_data(phys_data1.clone());

        // Ingest some digital threat data that correlates
        let dig_data1 = DigitalThreatData {
            alert_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::minutes(1),
            overall_severity: "High".to_string(),
            alert_type: "IntrusionAttempt".to_string(),
            summary: "Suspicious login attempts from external IP targeting 'ServerRoom_A-Control'".to_string(),
            affected_systems: vec!["ServerRoom_A-Control".to_string()],
            source_ip: Some("103.45.67.89".to_string()),
        };
        integrator.ingest_digital_data(dig_data1.clone());

        // Run correlation
        let events = integrator.correlate_and_generate_events();
        println!("Generated Unified Threat Events: {:?}", events);

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].overall_threat_level, "Critical");
        assert!(events[0].event_summary.contains("Physical intrusion detected at ServerRoom_A"));
        assert!(events[0].event_summary.contains("digital breach attempt"));

        // Ingest non-correlating data
        let phys_data2 = PhysicalSensorData {
            sensor_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::hours(5),
            sensor_type: "CCTV".to_string(),
            location: "Lobby".to_string(),
            event_description: "Person entered lobby".to_string(),
            associated_media_url: None,
            severity: "Low".to_string(),
        };
        integrator.ingest_physical_data(phys_data2.clone());

        let dig_data2 = DigitalThreatData {
            alert_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::days(1),
            overall_severity: "Low".to_string(),
            alert_type: "PhishingEmail".to_string(),
            summary: "Generic phishing email detected.".to_string(),
            affected_systems: vec![],
            source_ip: None,
        };
        integrator.ingest_digital_data(dig_data2.clone());

        let new_events = integrator.correlate_and_generate_events();
        assert_eq!(new_events.len(), 0);

        let all_events = integrator.get_unified_threat_events_log();
        assert_eq!(all_events.len(), 1);
    }
}
*/