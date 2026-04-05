// src/agi_core.rs

//! The core module of the Tariro AGI, orchestrating all major functionalities.

use crate::config::AGIConfig;
use crate::telemetry::TelemetryLogger;
use crate::autonomy::policy_engine::AutonomyPolicyEngine;
use crate::meta_learning::MetaLearningService;
use crate::agi_project_manager::ProjectManagerService;
use crate::multi_agent::AgentOrchestrationEngine;
use crate::news_engine::NewsEngine;

/// The main struct representing the Tariro AGI core.
/// It holds references to all major sub-systems and orchestrates their interactions.
pub struct TariroAGICore {
    pub config: AGIConfig,
    pub telemetry_logger: TelemetryLogger,
    pub autonomy_policy_engine: AutonomyPolicyEngine,
    pub meta_learning_service: MetaLearningService,
    pub project_manager_service: ProjectManagerService,
    pub agent_orchestration_engine: AgentOrchestrationEngine,
    pub news_engine: NewsEngine,
    // Add other core services/modules here as they are integrated
}

impl TariroAGICore {
    /// Creates a new instance of the Tariro AGI Core.
    pub fn new(
        config: AGIConfig,
        telemetry_logger: TelemetryLogger,
        autonomy_policy_engine: AutonomyPolicyEngine,
        meta_learning_service: MetaLearningService,
        project_manager_service: ProjectManagerService,
        agent_orchestration_engine: AgentOrchestrationEngine,
        news_engine: NewsEngine,
    ) -> Self {
        TariroAGICore {
            config,
            telemetry_logger,
            autonomy_policy_engine,
            meta_learning_service,
            project_manager_service,
            agent_orchestration_engine,
            news_engine,
        }
    }

    /// Initializes the AGI core, setting up all sub-systems.
    pub async fn initialize(&self) -> Result<(), String> {
        println!("[AGICore] Initializing Tariro AGI Core v{}", self.config.get_setting("core.system_version").unwrap_or("unknown".to_string()));
        self.telemetry_logger.log_event(
            crate::telemetry::TelemetryEvent::SystemInitialized { 
                version: self.config.get_setting("core.system_version").unwrap_or_default() 
            }
        ).await;
        
        // Example of internal service initialization
        self.autonomy_policy_engine.initialize().await?;
        self.meta_learning_service.initialize().await?;
        self.project_manager_service.initialize().await?;
        self.agent_orchestration_engine.initialize().await?;
        self.news_engine.initialize().await?;

        println!("[AGICore] Tariro AGI Core initialized successfully.");
        Ok(())
    }

    /// Starts the main operational loop of the AGI.
    /// This would typically involve an event loop, task processing, etc.
    pub async fn run(&self) -> Result<(), String> {
        println!("[AGICore] Tariro AGI Core starting main operational loop.");
        self.telemetry_logger.log_event(crate::telemetry::TelemetryEvent::SystemStarted).await;

        // In a real AGI, this would be a complex, continuous loop
        // orchestrating sensing, planning, acting, learning.
        // For now, simulate some continuous operation.
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            //println!("[AGICore] Core operational heartbeat...");

            // Example: AGI might periodically check for new goals or learning opportunities
            // self.meta_learning_service.check_for_learning_opportunities().await;
            // self.project_manager_service.check_for_new_tasks().await;
            
            // Handle any pending telemetry events
            self.telemetry_logger.process_pending_events().await;
        }
    }

    /// Shuts down the AGI core gracefully.
    pub async fn shutdown(&self) -> Result<(), String> {
        println!("[AGICore] Shutting down Tariro AGI Core.");
        self.telemetry_logger.log_event(crate::telemetry::TelemetryEvent::SystemShutdown).await;

        // Perform cleanup for all sub-systems
        self.autonomy_policy_engine.shutdown().await?;
        self.meta_learning_service.shutdown().await?;
        self.project_manager_service.shutdown().await?;
        self.agent_orchestration_engine.shutdown().await?;
        self.news_engine.shutdown().await?;

        println!("[AGICore] Tariro AGI Core shut down successfully.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AGIConfig;
    use crate::telemetry::TelemetryEvent;
    use tokio::sync::mpsc;
    use crate::autonomy::policy_engine::AutonomyPolicy;
    use async_trait::async_trait;
    use crate::meta_learning::{LearningInsight, Category as LearningCategory};
    use crate::agi_project_manager::ProjectTask;
    use crate::multi_agent::NanoAgent;
    use crate::news_engine::{Article, Sentiment};

    // Mock implementations for dependencies
    struct MockAutonomyPolicyEngine { initialized: bool }
    #[async_trait]
    impl crate::autonomy::policy_engine::AutonomyPolicyEngineTrait for MockAutonomyPolicyEngine {
        fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self { MockAutonomyPolicyEngine { initialized: false } }
        async fn initialize(&self) -> Result<(), String> { Ok(()) }
        async fn shutdown(&self) -> Result<(), String> { Ok(()) }
        async fn evaluate_action(&self, action: &str, context: &std::collections::HashMap<String, String>) -> Result<crate::autonomy::policy_engine::PolicyDecision, String> { Ok(crate::autonomy::policy_engine::PolicyDecision::Approve) }
        async fn add_policy(&mut self, policy: AutonomyPolicy) -> Result<(), String> { Ok(()) }
        async fn update_policy(&mut self, policy: AutonomyPolicy) -> Result<(), String> { Ok(()) }
        async fn remove_policy(&mut self, policy_id: &str) -> Result<(), String> { Ok(()) }
        async fn get_policy(&self, policy_id: &str) -> Option<&AutonomyPolicy> { None }
        async fn list_policies(&self) -> Vec<&AutonomyPolicy> { vec![] }
    }

    struct MockMetaLearningService { initialized: bool }
    #[async_trait]
    impl crate::meta_learning::MetaLearningServiceTrait for MockMetaLearningService {
        fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self { MockMetaLearningService { initialized: false } }
        async fn initialize(&self) -> Result<(), String> { Ok(()) }
        async fn shutdown(&self) -> Result<(), String> { Ok(()) }
        async fn process_insight(&mut self, insight: LearningInsight) -> Result<(), String> { Ok(()) }
        async fn get_insights(&self, category: Option<LearningCategory>) -> Vec<&LearningInsight> { vec![] }
        async fn identify_knowledge_gaps(&self) -> Result<Vec<String>, String> { Ok(vec![]) }
        async fn check_for_learning_opportunities(&self) { /* do nothing */ }
    }

    struct MockProjectManagerService { initialized: bool }
    #[async_trait]
    impl crate::agi_project_manager::ProjectManagerServiceTrait for MockProjectManagerService {
        fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self { MockProjectManagerService { initialized: false } }
        async fn initialize(&self) -> Result<(), String> { Ok(()) }
        async fn shutdown(&self) -> Result<(), String> { Ok(()) }
        async fn create_project_task(&mut self, task: ProjectTask) -> Result<String, String> { Ok("mock_task_id".to_string()) }
        async fn get_task_status(&self, task_id: &str) -> Option<&crate::agi_project_manager::TaskStatus> { None }
        async fn check_for_new_tasks(&self) { /* do nothing */ }
    }

    struct MockAgentOrchestrationEngine { initialized: bool }
    #[async_trait]
    impl crate::multi_agent::AgentOrchestrationEngineTrait for MockAgentOrchestrationEngine {
        fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self { MockAgentOrchestrationEngine { initialized: false } }
        async fn initialize(&self) -> Result<(), String> { Ok(()) }
        async fn shutdown(&self) -> Result<(), String> { Ok(()) }
        async fn deploy_nano_agent(&mut self, agent: NanoAgent) -> Result<String, String> { Ok("mock_nano_agent_id".to_string()) }
    }

    struct MockNewsEngine { initialized: bool }
    #[async_trait]
    impl crate::news_engine::NewsEngineTrait for MockNewsEngine {
        fn new(config: AGIConfig, telemetry_logger: TelemetryLogger) -> Self { MockNewsEngine { initialized: false } }
        async fn initialize(&self) -> Result<(), String> { Ok(()) }
        async fn shutdown(&self) -> Result<(), String> { Ok(()) }
        async fn analyze_article_sentiment(&self, article: &Article) -> Result<Sentiment, String> { Ok(Sentiment::Neutral) }
        async fn fetch_latest_news(&self, query: &str) -> Result<Vec<Article>, String> { Ok(vec![]) }
    }

    #[tokio::test]
    async fn test_agi_core_initialization() {
        let config = AGIConfig::default();
        let (tx, mut rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx.clone());
        
        let autonomy_policy_engine = MockAutonomyPolicyEngine::new(config.clone(), telemetry_logger.clone());
        let meta_learning_service = MockMetaLearningService::new(config.clone(), telemetry_logger.clone());
        let project_manager_service = MockProjectManagerService::new(config.clone(), telemetry_logger.clone());
        let agent_orchestration_engine = MockAgentOrchestrationEngine::new(config.clone(), telemetry_logger.clone());
        let news_engine = MockNewsEngine::new(config.clone(), telemetry_logger.clone());

        let agi_core = TariroAGICore::new(
            config,
            telemetry_logger,
            autonomy_policy_engine,
            meta_learning_service,
            project_manager_service,
            agent_orchestration_engine,
            news_engine,
        );

        let result = agi_core.initialize().await;
        assert!(result.is_ok());

        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::SystemInitialized { version } => assert!(!version.is_empty()),
            _ => panic!("Expected SystemInitialized event"),
        }
    }

    #[tokio::test]
    async fn test_agi_core_shutdown() {
        let config = AGIConfig::default();
        let (tx, mut rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx.clone());
        
        let autonomy_policy_engine = MockAutonomyPolicyEngine::new(config.clone(), telemetry_logger.clone());
        let meta_learning_service = MockMetaLearningService::new(config.clone(), telemetry_logger.clone());
        let project_manager_service = MockProjectManagerService::new(config.clone(), telemetry_logger.clone());
        let agent_orchestration_engine = MockAgentOrchestrationEngine::new(config.clone(), telemetry_logger.clone());
        let news_engine = MockNewsEngine::new(config.clone(), telemetry_logger.clone());

        let agi_core = TariroAGICore::new(
            config,
            telemetry_logger,
            autonomy_policy_engine,
            meta_learning_service,
            project_manager_service,
            agent_orchestration_engine,
            news_engine,
        );

        let result = agi_core.shutdown().await;
        assert!(result.is_ok());
        
        // Expect a SystemShutdown event
        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::SystemShutdown => { /* success */ },
            _ => panic!("Expected SystemShutdown event"),
        }
    }
}
