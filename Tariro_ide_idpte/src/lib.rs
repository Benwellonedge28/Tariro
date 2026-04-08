// Tariro_ide_idpte/src/lib.rs
use std::sync::Arc;
use tariro_core_services::config::AGIConfig;
use tariro_core_services::database::DatabaseService;
use tariro_core_services::event_bus::EventBus;

mod syntax_highlighting;
mod ide_core;
mod ide_frontend;
pub mod db; // This 'db' module might need refactoring or removal if sharing the AGI's DatabaseService

pub async fn start_ide_bridge(
    config: Arc<AGIConfig>,
    db_service: Arc<DatabaseService>,
    event_bus: Arc<EventBus>,
) {
    // Pass these shared resources down to the frontend, which will then pass them to core components
    ide_frontend::start(config, db_service, event_bus).await;
}
