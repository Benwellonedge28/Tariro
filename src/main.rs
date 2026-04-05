// src/main.rs

//! Main entry point for the Tariro AGI (Artificial General Intelligence) system.
//! This module initializes the core components, sets up the runtime environment,
//! and starts the main operational loops for the AGI.

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
mod agi_cloning;
mod agi_replication;
mod agi_swarm;
mod compiler; // Compiler module for Meta-Compiler functionality
mod deployment; // Deployment module for managing deployments
mod event_bus; // Event bus for inter-module communication
mod evaluators; // Evaluation module for ATVE
mod omega_network;

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
use agi_replication::ReplicationService; // Renamed from agi_cloning::CloningService to agi_replication::ReplicationService
use event_bus::{EventBus, AGIEvent};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let telemetry_logger = Arc::new(TelemetryLogger::new(event_bus.sender.subscribe()));

    // 5. Initialize Database Service
    info!("Connecting to database...");
    let db_service = Arc::new(DatabaseService::new(&shared_config.database_url, "tariro_agi").await
        .map_err(|e| format!("Failed to connect to database: {}", e))?);
    info!("Database service initialized.");

    // 6. Initialize core AGI services
    let admin_service = Arc::new(Mutex::new(AdminService::new(shared_config.clone(), telemetry_logger.clone())));
    let autonomy_policy_engine = Arc::new(AutonomyPolicyEngine::new(shared_config.clone(), telemetry_logger.clone(), db_service.clone()));
    let meta_learning_service = Arc::new(Mutex::new(MetaLearningService::new(shared_config.clone(), telemetry_logger.clone(), db_service.clone())));
    let project_manager_service = Arc::new(Mutex::new(ProjectManagerService::new(shared_config.clone(), telemetry_logger.clone())));
    let agent_orchestration_engine = Arc::new(Mutex::new(AgentOrchestrationEngine::new(shared_config.clone(), telemetry_logger.clone())));
    let news_engine = Arc::new(Mutex::new(NewsEngine::new(shared_config.clone(), telemetry_logger.clone())));
    let replication_service = Arc::new(Mutex::new(ReplicationService::new(shared_config.clone(), telemetry_logger.clone())));
    // Initialize other services here

    // 7. Initialize Tariro AGI Core
    let agi_core = Arc::new(TariroAGICore::new(
        shared_config.clone(),
        telemetry_logger.clone(),
        autonomy_policy_engine.clone(),
        meta_learning_service.clone(),
        project_manager_service.clone(),
        agent_orchestration_engine.clone(),
        news_engine.clone(),
    ));

    // 8. Initialize API server state
    let api_state = api::AppState {
        config: shared_config.clone(),
        telemetry_logger: telemetry_logger.clone(),
        admin_service: admin_service.clone(),
        replication_service: replication_service.clone(),
        meta_learning_service: meta_learning_service.clone(),
    };

    // 9. Start API server
    let api_port = shared_config.api_port;
    let api_server = { 
        let routes = api::api_routes(api_state.clone());
        info!("API server starting on port {}", api_port);
        warp::serve(routes).run(([127, 0, 0, 1], api_port))
    };
    
    // 10. Start AGI Core operational loop
    let agi_core_clone = agi_core.clone();
    let agi_core_task = tokio::spawn(async move { 
        agi_core_clone.initialize().await.expect("Failed to initialize AGI Core");
        agi_core_clone.run().await.expect("AGI Core failed to run");
    });

    // 11. Start Telemetry event processing loop
    let telemetry_logger_clone = telemetry_logger.clone();
    let telemetry_task = tokio::spawn(async move {
        telemetry_logger_clone.process_events_loop().await;
    });

    // 12. Simulate some initial AGI events
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
        _ = telemetry_task => {
            info!("Telemetry processing stopped.");
        }
    }

    info!("Tariro AGI System Shutting Down...");
    agi_core.shutdown().await?;

    Ok(())
}
