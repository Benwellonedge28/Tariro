// src/main.rs

//! Main entry point for the Tariro AGI (Artificial General Intelligence) system.
//! This module initializes the core components, sets up the runtime environment,
//! and starts the main operational loops for the AGI, now including
//! the supremely autonomous Quantum Security Module.

#[macro_use]
extern crate log;
extern crate simple_logger;

use std::sync::{Arc, Mutex};
use tokio;

mod config;
mod telemetry;
mod admin;
mod autonomy;
mod meta_learning;
mod agi_project_manager;
mod multi_agent;
mod news_engine;
mod agi_core;
mod api;
mod database;
mod agi_replication;
mod event_bus;
mod evaluators;
mod omega_network;
mod omega_os;
mod content_generation;

// NEW: Quantum Security Module
mod quantum_security;
use quantum_security::mod::{QuantumSecurityManager, QuantumAutonomyLevel};

// Use the prelude from quantum_security for common types and the logging macro
use quantum_security::mod::prelude as qsm_prelude;

// NEW: QSM Notification Service
mod qsm_notifications;
use qsm_notifications::QsmNotificationService;

use config::AGIConfig;
use telemetry::TelemetryLogger;
use admin::AdminService;
use autonomy::policy_engine::AutonomyPolicyEngine;
use meta_learning::MetaLearningService;
use agi_project_manager::ProjectManagerService;
use multi_agent::AgentOrchestrationEngine;
use news_engine::NewsEngine;
use agi_core::TariroAGICore;
use api::AppState;
use database::DatabaseService;
use agi_replication::ReplicationService;
use event_bus::{EventBus, AGIEvent};
use omega_os::os_interface::OmegaOSInterface;
use content_generation::generator::ContentGenerator;
use content_generation::style_guide::StyleGuideManager;
use content_generation::review_process::ContentReviewService;


#[tokio::main]
async fn main() -> qsm_prelude::Result<()> { // Use QSM's Result type
    // 1. Initialize logging
    simple_logger::init_with_level(log::Level::Info)?;
    info!("Tariro AGI System Starting...");

    // 2. Load Configuration
    let config = match AGIConfig::load("AGIConfig.toml") {
        Ok(cfg) => {
            info!("Configuration loaded from AGIConfig.toml.");
            cfg
        },
        Err(e) => {
            warn!("Failed to load AGIConfig.toml ({}). Using default configuration.", e);
            AGIConfig::default()
        }
    };
    let shared_config = Arc::new(config.clone());
    
    // 3. Initialize Event Bus
    let event_bus = Arc::new(EventBus::new(100)); // Capacity for 100 events

    // 4. Initialize Telemetry Logger
    let telemetry_logger = Arc::new(TelemetryLogger::new(100)); // Buffer for 100 events
    let telemetry_event_bus_subscriber = event_bus.subscribe();
    let telemetry_logger_clone = telemetry_logger.clone();
    tokio::spawn(async move {
        telemetry_logger_clone.process_events_loop(telemetry_event_bus_subscriber).await;
    });

    // 5. Initialize Database Service
    info!("Connecting to database...");
    let db_service = Arc::new(DatabaseService::new(&shared_config.database_url, "tariro_agi").await
        .map_err(|e| format!("Failed to connect to database: {}", e))?);
    info!("Database service initialized.");

    // 6. Initialize Quantum Security Manager - Supremely Autonomous
    info!("Initializing Quantum Security Manager with Full Autonomy...");
    let qsm = Arc::new(Mutex::new(
        QuantumSecurityManager::new(QuantumAutonomyLevel::FullAuto).await?
    ));
    info!("Quantum Security Manager initialized with Full Autonomy.");

    // 7. Initialize core AGI services
    let admin_service = Arc::new(Mutex::new(AdminService::new(shared_config.clone(), telemetry_logger.clone())));
    let autonomy_policy_engine = Arc::new(AutonomyPolicyEngine::new(shared_config.clone(), telemetry_logger.clone(), db_service.clone()));
    let meta_learning_service = Arc::new(Mutex::new(MetaLearningService::new(shared_config.clone(), telemetry_logger.clone(), db_service.clone(), qsm.clone()))); // Pass QSM
    let project_manager_service = Arc::new(Mutex::new(ProjectManagerService::new(shared_config.clone(), telemetry_logger.clone())));
    let agent_orchestration_engine = Arc::new(Mutex::new(AgentOrchestrationEngine::new(shared_config.clone(), telemetry_logger.clone())));
    let news_engine = Arc::new(Mutex::new(NewsEngine::new(shared_config.clone(), telemetry_logger.clone(), db_service.clone())));
    let replication_service = Arc::new(Mutex::new(ReplicationService::new(shared_config.clone(), telemetry_logger.clone())));
    let omega_os_interface = Arc::new(Mutex::new(OmegaOSInterface::new()));
    let content_generator = Arc::new(Mutex::new(ContentGenerator::new()));
    let style_guide_manager = Arc::new(Mutex::new(StyleGuideManager::new()));
    let content_review_service = Arc::new(Mutex::new(ContentReviewService::new()));

    // 8. Initialize Tariro AGI Core
    let agi_core = Arc::new(TariroAGICore::new(
        shared_config.clone(),
        telemetry_logger.clone(),
        autonomy_policy_engine.clone(),
        meta_learning_service.clone(),
        project_manager_service.clone(),
        agent_orchestration_engine.clone(),
        news_engine.clone(),
        qsm.clone(), // Pass QSM
    ));

    // 9. Initialize API server state
    let api_state = api::AppState {
        config: shared_config.clone(),
        telemetry_logger: telemetry_logger.clone(),
        admin_service: admin_service.clone(),
        replication_service: replication_service.clone(),
        meta_learning_service: meta_learning_service.clone(),
    };

    // 10. Initialize QSM Notification Service and start its autonomous monitoring
    info!("Initializing QSM Notification Service...");
    let qsm_notification_service = Arc::new(QsmNotificationService::new(
        shared_config.clone(),
        telemetry_logger.clone(),
        db_service.clone(),
        qsm.clone(),
        event_bus.clone(),
    ));
    qsm_notification_service.initialize().await?;
    info!("QSM Notification Service initialized and monitoring autonomously.");

    // 11. Start autonomous QSM background tasks (already implemented for adapt_security_posture)
    let qsm_clone_for_tasks = qsm.clone();
    let qsm_monitoring_interval_seconds = shared_config.qsm_monitoring_interval_seconds;
    tokio::spawn(async move {
        loop {
            // Autonomous adaptation to threats
            if let Err(e) = qsm_clone_for_tasks.lock().await.adapt_security_posture().await {
                error!("QSM background task error during security posture adaptation: {}", e);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(qsm_monitoring_interval_seconds)).await;
        }
    });

    // 12. Start API server
    let api_port = shared_config.api_port;
    let api_server = { 
        let routes = api::api_routes(api_state.clone());
        info!("API server starting on port {}.", api_port);
        warp::serve(routes).run(([127, 0, 0, 1], api_port))
    };
    
    // 13. Start AGI Core operational loop
    let agi_core_clone = agi_core.clone();
    let agi_core_task = tokio::spawn(async move { 
        agi_core_clone.initialize().await.expect("Failed to initialize AGI Core");
        agi_core_clone.run().await.expect("AGI Core failed to run");
    });

    // Initialize NewsEngine (now after db_service is available)
    news_engine.lock().unwrap().initialize().await?;

    // 14. Simulate some initial AGI events
    let event_bus_clone = event_bus.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        event_bus_clone.publish(AGIEvent::SystemInitialized { version: shared_config.system_version.clone() }).await;
        event_bus_clone.publish(AGIEvent::SystemStarted).await;
        event_bus_clone.publish(AGIEvent::PerformanceMetricRecorded { metric_name: "startup_time".to_string(), value: 3.5 }).await;
    });

    // Run all tasks concurrently
    tokio::select! {
        _ = api_server => {
            info!("API server stopped.");
        }
        _ = agi_core_task => {
            info!("AGI Core stopped.");
        }
    }

    info!("Tariro AGI System Shutting Down...");
    agi_core.shutdown().await?; // Ensure AGI Core gracefully shuts down

    Ok(())
}
