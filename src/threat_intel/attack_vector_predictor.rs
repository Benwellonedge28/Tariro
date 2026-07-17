use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Re-using simplified `ThreatAlert` from fusion_center.rs for incoming intel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAlert {
    pub alert_id: String,
    pub timestamp: DateTime<Utc>,
    pub overall_severity: String,
    pub alert_type: String, // e.g., "ZeroDayExploit", "TargetedAttack", "Campaign"
    pub summary: String,
    pub technical_details: String, // e.g., "CVE-202X-YYYY affecting Apache Struts"
    pub tags: Vec<String>,
}

// Re-using simplified `DiscoveredAsset` and `AttackVector` from attack_surface_mapper.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredAsset {
    pub asset_id: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub asset_type: String,
    pub open_ports: Vec<u16>,
    pub services: HashMap<u16, String>, // port -> service name (e.g., 80 -> "HTTP")
    pub vulnerabilities_detected: Vec<String>, // List of CVEs or detected weaknesses
    pub last_scanned: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackVector {
    pub vector_id: String,
    pub asset_id: String,
    pub vector_type: String,
    pub description: String,
    pub severity: String,
    pub recommended_remediation: String,
}

// Represents a predicted future attack vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedAttackVector {
    pub prediction_id: String,
    pub prediction_timestamp: DateTime<Utc>,
    pub predicted_method: String, // e.g., "SupplyChainCompromise", "DDoS", "Ransomware"
    pub predicted_target_asset_ids: Vec<String>,
    pub likelihood_score: f64, // 0.0 - 1.0
    pub confidence_score: f64, // 0.0 - 1.0
    pub influencing_threat_intel_ids: Vec<String>, // IDs of ThreatAlerts that influenced this
    pub influencing_vulnerability_ids: Vec<String>, // IDs of AttackVectors or Vulnerabilities
    pub recommended_proactive_defense: String,
}

// Placeholder for accessing current threat intelligence data
pub struct ThreatIntelFeed {
    pub current_alerts: Mutex<Vec<ThreatAlert>>,
}

impl ThreatIntelFeed {
    pub fn new() -> Self {
        ThreatIntelFeed { current_alerts: Mutex::new(Vec::new()) }
    }

    pub fn add_alert(&self, alert: ThreatAlert) {
        self.current_alerts.lock().unwrap().push(alert);
    }
}

// Placeholder for accessing Tariro's internal vulnerability data
pub struct VulnerabilityScanResult {
    pub scanned_assets: Mutex<Vec<DiscoveredAsset>>,
    pub identified_attack_vectors: Mutex<Vec<AttackVector>>,
}

impl VulnerabilityScanResult {
    pub fn new() -> Self {
        VulnerabilityScanResult {
            scanned_assets: Mutex::new(Vec::new()),
            identified_attack_vectors: Mutex::new(Vec::new()),
        }
    }

    pub fn add_asset(&self, asset: DiscoveredAsset) {
        self.scanned_assets.lock().unwrap().push(asset);
    }

    pub fn add_attack_vector(&self, vector: AttackVector) {
        self.identified_attack_vectors.lock().unwrap().push(vector);
    }
}

pub struct AttackVectorPredictor {
    threat_intel_feed: Arc<ThreatIntelFeed>,
    vulnerability_scan_result: Arc<VulnerabilityScanResult>,
    prediction_log: Mutex<Vec<PredictedAttackVector>>,
}

impl AttackVectorPredictor {
    pub fn new(threat_intel_feed: Arc<ThreatIntelFeed>, vulnerability_scan_result: Arc<VulnerabilityScanResult>) -> Self {
        AttackVectorPredictor {
            threat_intel_feed,
            vulnerability_scan_result,
            prediction_log: Mutex::new(Vec::new()),
        }
    }

    // Predicts likely future attack methods and targets
    pub fn predict_attack_vectors(&self) -> Vec<PredictedAttackVector> {
        println!("Attack Vector Predictor: Generating new predictions...");

        let mut predictions = Vec::new();

        let current_threat_alerts = self.threat_intel_feed.current_alerts.lock().unwrap().clone();
        let current_vulnerabilities = self.vulnerability_scan_result.identified_attack_vectors.lock().unwrap().clone();
        let current_assets = self.vulnerability_scan_result.scanned_assets.lock().unwrap().clone();

        // --- Prediction Logic (Simplified) ---
        // Scenario 1: Critical vulnerability + zero-day alert = high likelihood of exploit
        if current_threat_alerts.iter().any(|a| a.alert_type == "ZeroDayExploit" && a.overall_severity == "Critical") {
            for vuln in &current_vulnerabilities {
                if vuln.severity == "High" || vuln.severity == "Critical" {
                    let affected_asset = current_assets.iter().find(|a| a.asset_id == vuln.asset_id);
                    if let Some(asset) = affected_asset {
                        let prediction = PredictedAttackVector {
                            prediction_id: Uuid::new_v4().to_string(),
                            prediction_timestamp: Utc::now(),
                            predicted_method: "Exploitation of Known Vulnerability".to_string(),
                            predicted_target_asset_ids: vec![asset.asset_id.clone()],
                            likelihood_score: 0.9,
                            confidence_score: 0.85,
                            influencing_threat_intel_ids: current_threat_alerts.iter().filter(|a| a.alert_type == "ZeroDayExploit").map(|a| a.alert_id.clone()).collect(),
                            influencing_vulnerability_ids: vec![vuln.vector_id.clone()],
                            recommended_proactive_defense: format!("Immediately patch asset {} for vulnerability {}.", asset.hostname.clone().unwrap_or(asset.ip_address.clone()), vuln.description),
                        };
                        predictions.push(prediction);
                    }
                }
            }
        }

        // Scenario 2: Broad phishing campaign + assets with exposed user services = high likelihood of credential compromise
        if current_threat_alerts.iter().any(|a| a.alert_type.contains("Phishing") && a.overall_severity == "High") {
            for asset in &current_assets {
                if asset.open_ports.contains(&443) || asset.open_ports.contains(&80) { // Web services
                    let prediction = PredictedAttackVector {
                        prediction_id: Uuid::new_v4().to_string(),
                        prediction_timestamp: Utc::now(),
                        predicted_method: "Credential Phishing / Account Takeover".to_string(),
                        predicted_target_asset_ids: vec![asset.asset_id.clone()],
                        likelihood_score: 0.7,
                        confidence_score: 0.75,
                        influencing_threat_intel_ids: current_threat_alerts.iter().filter(|a| a.alert_type.contains("Phishing")).map(|a| a.alert_id.clone()).collect(),
                        influencing_vulnerability_ids: vec![], // No direct vuln, but exposed service
                        recommended_proactive_defense: format!("Implement advanced email filtering and user security awareness training for assets with exposed web services like {}.", asset.hostname.clone().unwrap_or(asset.ip_address.clone())),
                    };
                    predictions.push(prediction);
                }
            }
        }

        self.prediction_log.lock().unwrap().extend(predictions.clone());
        println!("Attack Vector Predictor: Generated {} new attack vector predictions.", predictions.len());
        predictions
    }

    pub fn get_prediction_log(&self) -> Vec<PredictedAttackVector> {
        self.prediction_log.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_attack_vector_predictor() {
        let threat_intel = Arc::new(ThreatIntelFeed::new());
        let vuln_results = Arc::new(VulnerabilityScanResult::new());
        let predictor = AttackVectorPredictor::new(threat_intel.clone(), vuln_results.clone());

        // Setup: Add some mock data to the feeds
        threat_intel.add_alert(ThreatAlert {
            alert_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::hours(1),
            overall_severity: "Critical".to_string(),
            alert_type: "ZeroDayExploit".to_string(),
            summary: "New zero-day exploit for common Linux kernel versions.".to_string(),
            technical_details: "CVE-202X-YYYY: Linux Kernel RCE".to_string(),
            tags: vec!["Linux".to_string(), "Kernel".to_string()],
        });

        let asset1 = DiscoveredAsset {
            asset_id: Uuid::new_v4().to_string(),
            ip_address: "192.168.1.100".to_string(),
            hostname: Some("linux-server-prod".to_string()),
            asset_type: "Server".to_string(),
            open_ports: vec![22, 80],
            services: HashMap::from([(22, "SSH".to_string()), (80, "HTTP".to_string())]),
            vulnerabilities_detected: vec!["CVE-202X-YYYY (Linux Kernel Flaw)".to_string()], // Matches the alert
            last_scanned: Utc::now() - Duration::days(1),
        };
        let vuln1 = AttackVector {
            vector_id: Uuid::new_v4().to_string(),
            asset_id: asset1.asset_id.clone(),
            vector_type: "Vulnerability".to_string(),
            description: "CVE-202X-YYYY (Linux Kernel Flaw)".to_string(),
            severity: "Critical".to_string(),
            recommended_remediation: "Patch kernel".to_string(),
        };
        vuln_results.add_asset(asset1.clone());
        vuln_results.add_attack_vector(vuln1.clone());

        // Add a phishing alert
        threat_intel.add_alert(ThreatAlert {
            alert_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now() - Duration::minutes(30),
            overall_severity: "High".to_string(),
            alert_type: "PhishingCampaign".to_string(),
            summary: "Widespread phishing campaign targeting SaaS login credentials.".to_string(),
            technical_details: "Phishing emails impersonating common SaaS providers.".to_string(),
            tags: vec!["Phishing".to_string(), "Credentials".to_string()],
        });
        let asset2 = DiscoveredAsset {
            asset_id: Uuid::new_v4().to_string(),
            ip_address: "192.168.1.101".to_string(),
            hostname: Some("web-app-proxy".to_string()),
            asset_type: "Server".to_string(),
            open_ports: vec![443],
            services: HashMap::from([(443, "HTTPS".to_string())]),
            vulnerabilities_detected: vec![],
            last_scanned: Utc::now() - Duration::days(1),
        };
        vuln_results.add_asset(asset2.clone());


        let predictions = predictor.predict_attack_vectors();
        println!("Generated Predictions: {:?}", predictions);

        assert!(!predictions.is_empty());

        let kernel_exploit_prediction = predictions.iter().find(|p| p.predicted_method.contains("Exploitation of Known Vulnerability"));
        assert!(kernel_exploit_prediction.is_some());
        assert_eq!(kernel_exploit_prediction.unwrap().predicted_target_asset_ids[0], asset1.asset_id);
        assert!(kernel_exploit_prediction.unwrap().likelihood_score > 0.8);

        let phishing_prediction = predictions.iter().find(|p| p.predicted_method.contains("Credential Phishing"));
        assert!(phishing_prediction.is_some());
        assert_eq!(phishing_prediction.unwrap().predicted_target_asset_ids[0], asset2.asset_id);
    }
}
*/