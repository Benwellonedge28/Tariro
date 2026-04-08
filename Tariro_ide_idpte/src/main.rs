mod syntax_highlighting;
mod ide_core;
mod ide_frontend;
pub mod db; // Add the db module

#[tokio::main]
async fn main() {
    ide_frontend::start().await;
}
