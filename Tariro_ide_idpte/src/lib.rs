mod syntax_highlighting;
mod ide_core;
mod ide_frontend;
pub mod db; // Add the db module

pub async fn start_ide_bridge() {
    ide_frontend::start().await;
}
