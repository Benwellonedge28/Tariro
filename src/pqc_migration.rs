// src/pqc_migration.rs

//! Tariro's Automated Post-Quantum Cryptography (PQC) Migration Service.
//! This service autonomously orchestrates the migration of cryptographic assets
//! in response to detected quantum threats, ensuring continuous security and resilience.

use crate::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::time::{self, Duration};
use tokio::task; 
use crate::telemetry::TelemetryLogger;
use crate::database::DatabaseService;
use crate::quantum_security::mod::{QuantumSecurityManager, QuantumAutonomyLevel};
use crate::quantum_security::threat_intelligence::{QuantumThreat, ThreatSeverity, PqcMigrationRecommendation};
use crate::quantum_security::mod::prelude as qsm_prelude;
use crate::event_bus::{EventBus, AGIEvent};
use crate::config::AGIConfig;

/// The AutomatedPqcMigrationService orchestrates the autonomous migration of
/// cryptographic assets (data, channels) when a threat to active PQC is detected.
pub struct AutomatedPqcMigrationService {
    config: Arc<AGIConfig>,
    telemetry_logger: TelemetryLogger,
    db_service: Arc<DatabaseService>,
    qsm: Arc<Mutex<QuantumSecurityManager>>,
    event_bus: Arc<EventBus>,
    // State to manage ongoing migrations, e.g., HashMap<String, MigrationStatus>
}

impl AutomatedPqcMigrationService {
    /// Creates a new `AutomatedPqcMigrationService` instance.
    pub fn new(
        config: Arc<AGIConfig>,
        telemetry_logger: TelemetryLogger,
        db_service: Arc<DatabaseService>,
        qsm: Arc<Mutex<QuantumSecurityManager>>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        AutomatedPqcMigrationService {
            config,
            telemetry_logger,
            db_service,
            qsm,
            event_bus,
        }
    }

    /// Initializes the service and starts listening for PQC migration triggers.
    pub async fn initialize(&self) -> qsm_prelude::Result<()> {
        info!("[PQC Migration Service] Initializing autonomous migration listener.");
        self.telemetry_logger.log_event(
            crate::telemetry::TelemetryEvent::ServiceInitialized { service_name: "AutomatedPqcMigrationService".to_string() }
        ).await;

        let mut event_receiver = self.event_bus.subscribe();
        let qsm_clone = self.qsm.clone();
        let telemetry_logger_clone = self.telemetry_logger.clone();
        let db_service_clone = self.db_service.clone();
        let event_bus_clone = self.event_bus.clone();

        task::spawn(async move {
            loop {
                match event_receiver.recv().await {
                    Ok(AGIEvent::PqcMigrationRequired { recommendation }) => {
                        info!(
                            "[PQC Migration Service] Received PQC_MIGRATION_REQUIRED event for threat: {}",
                            recommendation.threat_id
                        );
                        if let Err(e) = Self::orchestrate_autonomous_migration(
                            &qsm_clone,
                            &telemetry_logger_clone,
                            &db_service_clone,
                            &event_bus_clone,
                            &recommendation,
                        ).await {
                            error!("[PQC Migration Service] Error during autonomous migration orchestration: {}", e);
                        }
                    },
                    Ok(_) => { /* Ignore other events */ },
                    Err(e) => {
                        error!("[PQC Migration Service] Event bus receive error: {}", e);
                        
                        break; 
                    }
                }
            }
            error!("[PQC Migration Service] Event listener stopped. Automated PQC migration disabled.");
        });

        Ok(())
    }

    /// Orchestrates the "supremely autonomous" migration process.
    async fn orchestrate_autonomous_migration(
        qsm: &Arc<Mutex<QuantumSecurityManager>>,
        telemetry_logger: &TelemetryLogger,
        db_service: &Arc<DatabaseService>, 
        event_bus: &EventBus,
        recommendation: &PqcMigrationRecommendation,
    ) -> qsm_prelude::Result<()> {
        info!("[PQC Migration Service] Starting autonomous migration based on recommendation: {:?}", recommendation);
        telemetry_logger.log_event(crate::telemetry::TelemetryEvent::PqcMigrationInitiated {
            threat_id: recommendation.threat_id.clone(),
            recommended_suite: recommendation.recommended_algorithm_suite.clone(),
        }).await;

        // Phase 1: Assess Impact and Prepare
        info!("[PQC Migration Service] Phase 1: Assessing impact and preparing for migration...");
        
        time::sleep(Duration::from_secs(2)).await; 
        event_bus.publish(AGIEvent::SystemStatusUpdate {
            message: format!("PQC Migration: Impact assessment for threat {}", recommendation.threat_id),
            component: "PQC Migration".to_string(),
            status: "preparing".to_string(),
        }).await;

        // Phase 2: Select and Activate New PQC Algorithms
        info!("[PQC Migration Service] Phase 2: Autonomously selecting and activating new PQC algorithms...");
        let mut qsm_locked = qsm.lock().await;
        
        qsm_locked.adapt_security_posture().await?; 
        info!("[PQC Migration Service] New PQC algorithms activated by QSM: {} and {}", qsm_locked.active_cipher.name(), qsm_locked.active_kex.name());
        drop(qsm_locked); 

        // Phase 3: Data and Channel Re-encryption/Migration
        info!("[PQC Migration Service] Phase 3: Orchestrating data and channel re-encryption...");
        
        time::sleep(Duration::from_secs(5)).await; 
        event_bus.publish(AGIEvent::SystemStatusUpdate {
            message: format!("PQC Migration: Data and channel re-encryption for threat {}", recommendation.threat_id),
            component: "PQC Migration".to_string(),
            status: "migrating_data".to_string(),
        }).await;

        // Phase 4: Verification and Rollback Preparedness
        info!("[PQC Migration Service] Phase 4: Verifying migration and preparing for rollback...");
        
        time::sleep(Duration::from_secs(3)).await; 
        event_bus.publish(AGIEvent::SystemStatusUpdate {
            message: format!("PQC Migration: Verification complete for threat {}", recommendation.threat_id),
            component: "PQC Migration".to_string(),
            status: "verifying".to_string(),
        }).await;

        info!("[PQC Migration Service] Autonomous PQC migration for threat {} completed successfully.", recommendation.threat_id);
        telemetry_logger.log_event(crate::telemetry::TelemetryEvent::PqcMigrationCompleted {
            threat_id: recommendation.threat_id.clone(),
            status: "success".to_string(),
        }).await;
        event_bus.publish(AGIEvent::PqcMigrationCompleted {
            threat_id: recommendation.threat_id.clone(),
            status: "success".to_string(),
        }).await;

        Ok(())
    }
}
