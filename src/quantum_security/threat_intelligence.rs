// src/quantum_security/threat_intelligence.rs

//! Manages supremely autonomous quantum threat intelligence and adaptive responses for Tariro.

use crate::quantum_security::prelude::*;
use chrono::{DateTime, Utc, Duration};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::event_bus::{EventBus, AGIEvent};

/// Represents a detected quantum threat.
#[derive(Debug, Clone)]
pub struct QuantumThreat {
    pub id: String,
    pub description: String,
    pub severity: ThreatSeverity,
    pub affected_algorithms: Vec<String>,
    pub recommended_action: String,
    pub timestamp: DateTime<Utc>,
    pub predicted_impact_date: Option<DateTime<Utc>>,
}

/// Severity of a quantum threat.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)] 
pub enum ThreatSeverity {
    Low,
    Medium,
    High,
    Critical,
    Existential, 
}

/// NEW: Represents a recommendation for PQC migration.
#[derive(Debug, Clone)]
pub struct PqcMigrationRecommendation {
    pub threat_id: String,
    pub affected_algorithms: Vec<String>,
    pub recommended_algorithm_suite: String, 
    pub urgency: ThreatSeverity,
    pub details: String,
}


/// The core component for Tariro's Quantum Threat Intelligence.
/// Infinitely advanced means predictive and autonomously acting.
#[derive(Debug)]
pub struct QuantumThreatIntelligence {
    current_threat_model: Arc<Mutex<Vec<QuantumThreat>>>,
    last_model_update: Arc<Mutex<DateTime<Utc>>>,
    predictive_analytics_engine_active: bool,
    external_quantum_sensors_active: bool,
    active_pqc_algorithms: Arc<Mutex<Vec<String>>>,
    event_bus: Arc<EventBus>,
}

impl QuantumThreatIntelligence {
    /// Initializes the Quantum Threat Intelligence system.
    pub async fn new(active_pqc_algorithms: Vec<String>, event_bus: Arc<EventBus>) -> Result<Self> { 
        tariro_log!(info, "Initializing QuantumThreatIntelligence system.");
        let mut instance = Self {
            current_threat_model: Arc::new(Mutex::new(Vec::new())),
            last_model_update: Arc::new(Mutex::new(Utc::now())),
            predictive_analytics_engine_active: true,
            external_quantum_sensors_active: false,
            active_pqc_algorithms: Arc::new(Mutex::new(active_pqc_algorithms)), 
            event_bus, 
        };
        // Perform initial autonomous threat model update
        instance.update_threat_model().await?;
        Ok(instance)
    }

    /// NEW: Update the list of PQC algorithms currently active in Tariro.
    pub async fn update_active_pqc_algorithms(&self, new_algorithms: Vec<String>) {
        let mut active = self.active_pqc_algorithms.lock().await;
        *active = new_algorithms;
        tariro_log!(info, "QuantumThreatIntelligence: Updated active PQC algorithms to: {:?}", active);
    }

    /// Autonomously updates the quantum threat model by fetching new data,
    /// running predictive analytics, and evaluating cryptographic standards.
    pub async fn update_threat_model(&self) -> Result<()> {
        tariro_log!(info, "Autonomously updating quantum threat model.");

        // Simulate fetching from a global quantum threat network (QGIS)
        let new_threats = self.simulate_threat_data_fetch().await?;

        let mut current_model = self.current_threat_model.lock().await;
        current_model.clear();
        current_model.extend(new_threats);
        *self.last_model_update.lock().await = Utc::now();

        // Autonomic analysis and adaptive response triggering
        self.analyze_and_adapt(&current_model).await?; 

        tariro_log!(info, "Quantum threat model updated. Current active threats: {}", current_model.len());
        Ok(())
    }

    /// Simulates fetching new quantum threat data from various advanced sources.
    async fn simulate_threat_data_fetch(&self) -> Result<Vec<QuantumThreat>> {
        // In an infinitely advanced system, this would:
        // 1. Query global intelligence networks.
        // 2. Parse academic research papers in real-time.
        // 3. Monitor quantum computing advancements and breakthroughs.
        // 4. Incorporate data from Tariro's own quantum sensor network (if `external_quantum_sensors_active`).

        let mut threats = Vec::new();
        // Simulate periodic new threat detection
        if Utc::now().minute() % 5 == 0 { 
            threats.push(QuantumThreat {
                id: Uuid::new_v4().to_string(),
                description: "New theoretical attack vector for current elliptic curve cryptography based on improved quantum factoring algorithms.".to_string(),
                severity: ThreatSeverity::High,
                affected_algorithms: vec!["ECDH".to_string(), "ECDSA".to_string()],
                recommended_action: "Increase re-keying frequency, prioritize transition to lattice-based KEX.".to_string(),
                timestamp: Utc::now(),
                predicted_impact_date: Some(Utc::now() + Duration::days(365 * 3)), 
            });
        }
        if Utc::now().second() % 30 == 0 { 
             threats.push(QuantumThreat {
                id: Uuid::new_v4().to_string(),
                description: "BREAKING: Practical post-quantum attack on Kyber-512 variant demonstrated in controlled lab environment.".to_string(),
                severity: ThreatSeverity::Critical,
                affected_algorithms: vec!["Kyber-512".to_string()],
                recommended_action: "Immediate migration to Kyber-768 or Kyber-1024, or alternative PQC suite.".to_string(),
                timestamp: Utc::now(),
                predicted_impact_date: Some(Utc::now() + Duration::days(30)), 
            });
        }
        Ok(threats)
    }

    /// Autonomously analyzes the current threat model and triggers adaptive responses.
    /// This is where the "supremely autonomous" and "infinitely advanced" decision-making happens.
    async fn analyze_and_adapt(&self, threat_model: &[QuantumThreat]) -> Result<()> {
        tariro_log!(debug, "Analyzing threat model for autonomous adaptation.");

        let active_algs = self.active_pqc_algorithms.lock().await;

        for threat in threat_model {
            // Check if this threat affects any of our currently active algorithms
            let affects_active_alg = threat.affected_algorithms.iter()
                .any(|threat_alg| active_algs.contains(threat_alg));

            if affects_active_alg {
                tariro_log!(warn, "Detected threat affecting ACTIVE PQC algorithms: {:?} due to {:?}", threat.affected_algorithms, threat.severity);

                // For critical/existential threats affecting active algorithms, trigger migration event
                if threat.severity >= ThreatSeverity::Critical { // Using >= because Existential > Critical
                    tariro_log!(error, "CRITICAL QUANTUM THREAT DETECTED affecting active algorithms: {}", threat.description);
                    
                    let recommendation = self.get_migration_recommendation(threat).await;
                    self.event_bus.publish(AGIEvent::PqcMigrationRequired {
                        recommendation: recommendation.clone()
                    }).await;
                    tariro_log!(critical, "Published PqcMigrationRequired event for threat: {}. Urgency: {:?}", threat.id, recommendation.urgency);
                }
            } else {
                tariro_log!(info, "Detected quantum threat (severity {:?}) not currently affecting active algorithms.", threat.severity);
            }
        }
        Ok(())
    }

    /// NEW: Provides a PQC migration recommendation based on a detected threat.
    pub async fn get_migration_recommendation(&self, threat: &QuantumThreat) -> PqcMigrationRecommendation {
        // This is where advanced AI would dynamically determine the best next steps.
        // For infinitely advanced: this would involve real-time PQC algorithm benchmarking,
        // compatibility checks, and a complex decision-making process.
        
        let recommended_suite = match threat.severity {
            ThreatSeverity::Existential | ThreatSeverity::Critical => "Kyber-768, Dilithium-III, Falcon-512".to_string(),
            ThreatSeverity::High => "Kyber-768, Dilithium-II".to_string(),
            _ => "Review existing PQC algorithms.".to_string(),
        };

        PqcMigrationRecommendation {
            threat_id: threat.id.clone(),
            affected_algorithms: threat.affected_algorithms.clone(),
            recommended_algorithm_suite: recommended_suite,
            urgency: threat.severity.clone(),
            details: threat.recommended_action.clone(),
        }
    }

    /// Retrieves the current quantum threat landscape.
    pub async fn get_current_threats(&self) -> Vec<QuantumThreat> {
        self.current_threat_model.lock().await.clone()
    }
}
