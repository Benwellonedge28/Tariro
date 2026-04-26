use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents a security log event from various sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLogEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub source: String, // e.g., "Firewall", "EndpointDetections", "SIEM"
    pub event_type: String, // e.g., "FailedLogin", "NetworkConnection", "FileAccess"
    pub actor_ip: Option<String>,
    pub target_asset: Option<String>, // Asset ID or name
    pub description: String,
    pub severity: String, // "Low", "Medium", "High", "Critical"
    pub raw_log: String,
}

// Represents a known APT Tactic, Technique, or Procedure (TTP)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AptTtp {
    pub ttp_id: String,
    pub name: String, // e.g., "T1059.001 - PowerShell", "T1003 - OS Credential Dumping"
    pub category: String, // e.g., "Execution", "CredentialAccess", "Persistence"
    pub description: String,
    pub indicators: Vec<String>, // Keywords, regexes, patterns associated with this TTP
}

// Represents a detected APT campaign or activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AptCampaignDetection {
    pub detection_id: String,
    pub timestamp: DateTime<Utc>,
    pub suspected_apt_group: String, // e.g., "APT28", "Unknown_Sophisticated_Actor"
    pub correlated_event_ids: Vec<String>, // IDs of SecurityLogEvents that contributed to this detection
    pub matched_ttps: Vec<String>, // IDs of AptTtps that were matched
    pub confidence_score: f64, // 0.0 - 1.0
    pub overall_severity: String, // "High", "Critical"
    pub recommended_response: String,
}

// Placeholder for a knowledge base of known APT TTPs
pub struct AptTtpKnowledgeBase {
    pub ttps: HashMap<String, AptTtp>, // key: ttp_id
}

impl AptTtpKnowledgeBase {
    pub fn new() -> Self {
        let mut ttps = HashMap::new();
        ttps.insert("T1059.001".to_string(), AptTtp {
            ttp_id: "T1059.001".to_string(),
            name: "T1059.001 - PowerShell".to_string(),
            category: "Execution".to_string(),
            description: "Adversaries may abuse PowerShell to execute commands and scripts.".to_string(),
            indicators: vec!["powershell.exe -e".to_string(), "Invoke-Expression".to_string(), "EncodedCommand".to_string()],
        });
        ttps.insert("T1003".to_string(), AptTtp {
            ttp_id: "T1003".to_string(),
            name: "T1003 - OS Credential Dumping".to_string(),
            category: "CredentialAccess".to_string(),
            description: "Adversaries may attempt to dump credentials to obtain account access.".to_string(),
            indicators: vec!["lsass.exe".to_string(), "mimikatz".to_string()],
        });
        AptTtpKnowledgeBase { ttps }
    }

    pub fn get_ttp(&self, ttp_id: &str) -> Option<&AptTtp> {
        self.ttps.get(ttp_id)
    }
}

pub struct AptRecognizer {
    apt_ttp_knowledge_base: Arc<AptTtpKnowledgeBase>,
    security_event_buffer: Mutex<Vec<SecurityLogEvent>>,
    apt_detection_log: Mutex<Vec<AptCampaignDetection>>,
}

impl AptRecognizer {
    pub fn new(apt_ttp_knowledge_base: Arc<AptTtpKnowledgeBase>) -> Self {
        AptRecognizer {
            apt_ttp_knowledge_base,
            security_event_buffer: Mutex::new(Vec::new()),
            apt_detection_log: Mutex::new(Vec::new()),
        }
    }

    // Ingests new security log events
    pub fn ingest_security_event(&self, event: SecurityLogEvent) {
        println!("APT Recognizer: Ingested security event: {}", event.event_id);
        self.security_event_buffer.lock().unwrap().push(event);
    }

    // Identifies subtle, long-term patterns indicative of sophisticated and persistent cyber-attacks
    pub fn run_pattern_recognition_cycle(&self) -> Vec<AptCampaignDetection> {
        println!("APT Recognizer: Running pattern recognition cycle...");

        let mut event_buffer_locked = self.security_event_buffer.lock().unwrap();
        // Take a snapshot of recent events and clear the buffer (or a portion of it)
        let events_to_analyze: Vec<SecurityLogEvent> = event_buffer_locked.drain(..).collect();

        if events_to_analyze.is_empty() {
            println!("APT Recognizer: No new security events to analyze.");
            return Vec::new();
        }

        let mut new_detections = Vec::new();
        let mut potential_ttps_matched: HashMap<String, Vec<String>> = HashMap::new(); // TTP_ID -> Vec<Event_ID>

        // Stage 1: Match individual events against TTP indicators
        let all_ttps = self.apt_ttp_knowledge_base.ttps.lock().unwrap().values().cloned().collect::<Vec<AptTtp>>();

        for event in &events_to_analyze {
            for ttp in &all_ttps {
                for indicator in &ttp.indicators {
                    if event.raw_log.contains(indicator) || event.description.contains(indicator) {
                        potential_ttps_matched.entry(ttp.ttp_id.clone()).or_default().push(event.event_id.clone());
                    }
                }
            }
        }

        // Stage 2: Correlate matched TTPs into potential APT campaigns
        // Simplified correlation: if multiple distinct TTPs are matched over time for related assets/IPs
        let mut apt_campaigns_identified: HashMap<String, AptCampaignDetection> = HashMap::new(); // Suspected_APT_Group -> Detection

        for (ttp_id, event_ids) in potential_ttps_matched {
            if event_ids.len() > 1 { // Simple threshold for multi-stage TTP usage
                let ttp = self.apt_ttp_knowledge_base.get_ttp(&ttp_id).unwrap(); // Should exist
                for event_id in event_ids {
                    let event = events_to_analyze.iter().find(|e| e.event_id == event_id);
                    if let Some(e) = event {
                        let actor_ip = e.actor_ip.clone().unwrap_or("Unknown".to_string());
                        let target_asset = e.target_asset.clone().unwrap_or("Unknown".to_string());

                        // Very simplified APT grouping. In reality, this would be highly complex graph analysis.
                        let suspected_group = if ttp.name.contains("PowerShell") && actor_ip == "1.2.3.4" {
                            "APT28".to_string() // Example: link specific TTP and IP to APT28
                        } else {
                            "Unknown_Sophisticated_Actor".to_string()
                        };

                        let detection = apt_campaigns_identified.entry(suspected_group.clone()).or_insert_with(|| AptCampaignDetection {
                            detection_id: Uuid::new_v4().to_string(),
                            timestamp: Utc::now(),
                            suspected_apt_group: suspected_group.clone(),
                            correlated_event_ids: Vec::new(),
                            matched_ttps: Vec::new(),
                            confidence_score: 0.0,
                            overall_severity: "High".to_string(),
                            recommended_response: "Investigate and contain affected assets, review all related logs.".to_string(),
                        });

                        detection.correlated_event_ids.push(e.event_id.clone());
                        detection.matched_ttps.push(ttp.ttp_id.clone());
                        // Update confidence based on number of matched TTPs, time correlation, etc.
                        detection.confidence_score += 0.1;
                    }
                }
            }
        }
        new_detections.extend(apt_campaigns_identified.into_values());

        self.apt_detection_log.lock().unwrap().extend(new_detections.clone());
        println!("APT Recognizer: Generated {} new APT detections.", new_detections.len());
        new_detections
    }

    pub fn get_apt_detection_log(&self) -> Vec<AptCampaignDetection> {
        self.apt_detection_log.lock().unwrap().clone()
    }

    pub fn get_security_event_buffer(&self) -> Vec<SecurityLogEvent> {
        self.security_event_buffer.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_apt_pattern_recognition() {
        let ttp_kb = Arc::new(AptTtpKnowledgeBase::new());
        let recognizer = AptRecognizer::new(ttp_kb);

        // Simulate a series of events indicative of an APT
        recognizer.ingest_security_event(SecurityLogEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::hours(5),
            source: "EndpointDetections".to_string(),
            event_type: "ProcessExecution".to_string(),
            actor_ip: Some("1.2.3.4".to_string()),
            target_asset: Some("server_internal_01".to_string()),
            description: "powershell.exe -e JABpAHYAawBvAGsAZQAtAEUAdwB4AHAAcgBlAHMAcwBpAG8AbgA= (base64 encoded command)".to_string(),
            severity: "High".to_string(),
            raw_log: "powershell.exe -e JABpAHYAawBvAGsAZQAtAEUAdwB4AHAAcgBlAHMAcwBpAG8AbgA=".to_string(),
        });
        recognizer.ingest_security_event(SecurityLogEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::hours(4),
            source: "NetworkLogs".to_string(),
            event_type: "NetworkConnection".to_string(),
            actor_ip: Some("1.2.3.4".to_string()),
            target_asset: Some("server_internal_01".to_string()),
            description: "Outbound connection from server_internal_01 to C2 server 5.6.7.8".to_string(),
            severity: "High".to_string(),
            raw_log: "Outbound TCP connection to 5.6.7.8 on port 443".to_string(),
        });
        recognizer.ingest_security_event(SecurityLogEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::hours(3),
            source: "EndpointDetections".to_string(),
            event_type: "FileAccess".to_string(),
            actor_ip: Some("1.2.3.4".to_string()),
            target_asset: Some("server_internal_01".to_string()),
            description: "Access to lsass.exe process memory by unknown process.".to_string(),
            severity: "Critical".to_string(),
            raw_log: "Process A attempted to read memory from lsass.exe".to_string(),
        });

        // Run pattern recognition
        let detections = recognizer.run_pattern_recognition_cycle();
        println!("Generated APT Detections: {:?}", detections);

        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].suspected_apt_group, "APT28");
        assert!(detections[0].matched_ttps.contains(&"T1059.001".to_string()));
        assert!(detections[0].matched_ttps.contains(&"T1003".to_string()));
        assert!(detections[0].confidence_score > 0.0);

        let event_buffer = recognizer.get_security_event_buffer();
        assert!(event_buffer.is_empty()); // Buffer should be cleared after processing

        let all_detections = recognizer.get_apt_detection_log();
        assert_eq!(all_detections.len(), 1);
    }
}
*/