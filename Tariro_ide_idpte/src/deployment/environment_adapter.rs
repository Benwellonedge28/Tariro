// src/deployment/environment_adapter.rs
use async_trait::async_trait;

#[async_trait]
pub trait EnvironmentInterface: Send + Sync {
    async fn deploy_application(&self, _params: &str) -> Result<String, String>;
}
pub struct SimulatedCloudAdapter;
#[async_trait]
impl EnvironmentInterface for SimulatedCloudAdapter {
    async fn deploy_application(&self, _params: &str) -> Result<String, String> { Ok("deployed".into()) }
}
