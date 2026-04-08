// src/agi_core.rs

//! The core module of the Tariro AGI, orchestrating all major functionalities.

use std::sync::{Arc, Mutex};
use crate::config::AGIConfig;
use crate::telemetry::TelemetryLogger;
use crate::autonomy::policy_engine::AutonomyPolicyEngine;
use crate::meta_learning::MetaLearningService;
use crate::agi_project_manager::ProjectManagerService;
use crate::multi_agent::AgentOrchestrationEngine;
use crate::news_engine::NewsEngine;
use crate::quantum_security::mod::{QuantumSecurityManager, QuantumAutonomyLevel};
use crate::quantum_security::mod::prelude as qsm_prelude;

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
    // NEW: Quantum Security Manager
    pub qsm: Arc<Mutex<QuantumSecurityManager>>,
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
        qsm: Arc<Mutex<QuantumSecurityManager>>, // NEW
    ) -> Self {
        TariroAGICore {
            config,
            telemetry_logger,
            autonomy_policy_engine,
            meta_learning_service,
            project_manager_service,
            agent_orchestration_engine,
            news_engine,
            qsm, // NEW
        }
    }

    /// Initializes the AGI core, setting up all sub-systems.
    pub async fn initialize(&self) -> qsm_prelude::Result<()> { // Using QSM prelude for Result type
        info!("Initializing Tariro AGI Core v{}", self.config.get_setting("core.system_version").unwrap_or("unknown".to_string()));
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

        info!("Tariro AGI Core initialized successfully.");
        Ok(())
    }

    /// Starts the main operational loop of the AGI.
    /// This would typically involve an event loop, task processing, etc.
    pub async fn run(&self) -> qsm_prelude::Result<()> { // Using QSM prelude for Result type
        info!("Tariro AGI Core starting main operational loop.");
        self.telemetry_logger.log_event(crate::telemetry::TelemetryEvent::SystemStarted).await;

        // In a real AGI, this would be a complex, continuous loop
        // orchestrating sensing, planning, acting, learning.
        // For now, simulate some continuous operation.
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            info!("Core operational heartbeat...");

            // Simulate inter-agent communication using QSM
            let qsm_locked = self.qsm.lock().await;
            match qsm_locked.establish_quantum_safe_channel("agi_sub_agent_alpha").await {
                Ok(channel) => info!("AGI Core: Established quantum-safe channel with {}.", channel.peer_id),
                Err(e) => error!("AGI Core: Failed to establish quantum-safe channel: {}", e),
            }

            // Simulate protecting sensitive internal state with QSM
            let sensitive_state = b"Core AGI internal decision parameters.";
            match qsm_locked.protect_data(sensitive_state, "agi_core_internal_state").await {
                Ok(protected) => info!("AGI Core: Protected {} bytes of internal state.", protected.len()),
                Err(e) => error!("AGI Core: Failed to protect internal state: {}", e),
            }
            drop(qsm_locked); // Release mutex

            // Example: AGI might periodically check for new goals or learning opportunities
            // self.meta_learning_service.check_for_learning_opportunities().await;
            // self.project_manager_service.check_for_new_tasks().await;
            
            // Handle any pending telemetry events
            self.telemetry_logger.process_pending_events().await;
        }
    }

    /// Shuts down the AGI core gracefully.
    pub async fn shutdown(&self) -> qsm_prelude::Result<()> { // Using QSM prelude for Result type
        info!("Shutting down Tariro AGI Core.");
        self.telemetry_logger.log_event(crate::telemetry::TelemetryEvent::SystemShutdown).await;

        // Perform cleanup for all sub-systems
        self.autonomy_policy_engine.shutdown().await?;
        self.meta_learning_service.shutdown().await?;
        self.project_manager_service.shutdown().await?;
        self.agent_orchestration_engine.shutdown().await?;
        self.news_engine.shutdown().await?;

        info!("Tariro AGI Core shut down successfully.");
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
    use crate::database::DatabaseService; // For mock DB

    // Mock implementations for dependencies
    // (Ensure these mocks are updated to accept QSM if they eventually need it, or pass a dummy)
    // For now, QSM isn't directly used in these mocks, so no change is needed yet.
    
    // NOTE: Need a mock for DatabaseService now that it's in MetaLearningService new()
    struct MockDatabaseService;
    impl MockDatabaseService {
        async fn new(_db_url: &str, _db_name: &str) -> qsm_prelude::Result<Self> { Ok(MockDatabaseService) }
        async fn list<T: for<'de> serde::Deserialize<'de> + Send + 'static>(&self) -> qsm_prelude::Result<Vec<T>> { Ok(vec![]) }
        async fn create<T: serde::Serialize + Send + 'static>(&self, _data: &mut T) -> qsm_prelude::Result<String> { Ok("mock_id".to_string()) }
    }


    struct MockAutonomyPolicyEngine { initialized: bool }
    #[async_trait]
    impl crate::autonomy::policy_engine::AutonomyPolicyEngineTrait for MockAutonomyPolicyEngine {
        fn new(_config: AGIConfig, _telemetry_logger: TelemetryLogger, _db_service: Arc<DatabaseService>) -> Self { MockAutonomyPolicyEngine { initialized: false } }
        async fn initialize(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn shutdown(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn evaluate_action(&self, _action: &str, _context: &std::collections::HashMap<String, String>) -> qsm_prelude::Result<crate::autonomy::policy_engine::PolicyDecision> { Ok(crate::autonomy::policy_engine::PolicyDecision::Approve) }
        async fn add_policy(&mut self, _policy: AutonomyPolicy) -> qsm_prelude::Result<()> { Ok(()) }
        async fn update_policy(&mut self, _policy: AutonomyPolicy) -> qsm_prelude::Result<()> { Ok(()) }
        async fn remove_policy(&mut self, _policy_id: &str) -> qsm_prelude::Result<()> { Ok(()) }
        async fn get_policy(&self, _policy_id: &str) -> Option<&AutonomyPolicy> { None }
        async fn list_policies(&self) -> Vec<&AutonomyPolicy> { vec![] }
    }

    struct MockMetaLearningService { initialized: bool }
    #[async_trait]
    impl crate::meta_learning::MetaLearningServiceTrait for MockMetaLearningService {
        fn new(_config: AGIConfig, _telemetry_logger: TelemetryLogger, _db_service: Arc<DatabaseService>, _qsm: Arc<Mutex<QuantumSecurityManager>>) -> Self { MockMetaLearningService { initialized: false } } // NEW QSM arg
        async fn initialize(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn shutdown(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn process_insight(&mut self, _insight: LearningInsight) -> qsm_prelude::Result<()> { Ok(()) }
        async fn get_insights(&self, _category: Option<LearningCategory>) -> Vec<&LearningInsight> { vec![] }
        async fn identify_knowledge_gaps(&self) -> qsm_prelude::Result<Vec<String>> { Ok(vec![]) }
        async fn check_for_learning_opportunities(&self) { /* do nothing */ }
        async fn consolidate_insights(&mut self) -> qsm_prelude::Result<u32> { Ok(0) }
    }

    struct MockProjectManagerService { initialized: bool }
    #[async_trait]
    impl crate::agi_project_manager::ProjectManagerServiceTrait for MockProjectManagerService {
        fn new(_config: AGIConfig, _telemetry_logger: TelemetryLogger) -> Self { MockProjectManagerService { initialized: false } }
        async fn initialize(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn shutdown(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn create_project_task(&mut self, _task: ProjectTask) -> qsm_prelude::Result<String> { Ok("mock_task_id".to_string()) }
        async fn get_task_status(&self, _task_id: &str) -> Option<&crate::agi_project_manager::TaskStatus> { None }
        async fn check_for_new_tasks(&self) { /* do nothing */ }
    }

    struct MockAgentOrchestrationEngine { initialized: bool }
    #[async_trait]
    impl crate::multi_agent::AgentOrchestrationEngineTrait for MockAgentOrchestrationEngine {
        fn new(_config: AGIConfig, _telemetry_logger: TelemetryLogger) -> Self { MockAgentOrchestrationEngine { initialized: false } }
        async fn initialize(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn shutdown(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn deploy_nano_agent(&mut self, _agent: NanoAgent) -> qsm_prelude::Result<String> { Ok("mock_nano_agent_id".to_string()) }
    }

    struct MockNewsEngine { initialized: bool }
    #[async_trait]
    impl crate::news_engine::NewsEngineTrait for MockNewsEngine {
        fn new(_config: AGIConfig, _telemetry_logger: TelemetryLogger, _db_service: Arc<DatabaseService>) -> Self { MockNewsEngine { initialized: false } }
        async fn initialize(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn shutdown(&self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn analyze_article_sentiment(&self, _article: &Article) -> qsm_prelude::Result<Sentiment> { Ok(Sentiment::Neutral) }
        async fn fetch_latest_news(&self, _query: &str) -> qsm_prelude::Result<Vec<Article>> { Ok(vec![]) }
    }

    // Mock QSM for tests
    use crate::quantum_security::pqc_algorithms::{QuantumResistantCipher, QuantumResistantKeyExchange};
    use async_trait::async_trait;

    #[derive(Debug, Clone)]
    struct MockQuantumResistantCipher;
    impl QuantumResistantCipher for MockQuantumResistantCipher {
        fn name(&self) -> &'static str { "MockCipher" }
        fn encrypt(&self, _key: &[u8], plaintext: &[u8], _aad: &[u8]) -> qsm_prelude::Result<Vec<u8>> { Ok(plaintext.to_vec()) }
        fn decrypt(&self, _key: &[u8], ciphertext: &[u8], _aad: &[u8]) -> qsm_prelude::Result<Vec<u8>> { Ok(ciphertext.to_vec()) }
        fn generate_key(&self) -> Vec<u8> { vec![0; 32] }
        fn key_size(&self) -> usize { 32 }
        fn box_clone(&self) -> Box<dyn QuantumResistantCipher + Send + Sync> { Box::new(self.clone()) }
    }

    #[derive(Debug, Clone)]
    struct MockQuantumResistantKeyExchange;
    impl QuantumResistantKeyExchange for MockQuantumResistantKeyExchange {
        fn name(&self) -> &'static str { "MockKEX" }
        fn generate_keypair(&self) -> qsm_prelude::Result<(Vec<u8>, Vec<u8>)> { Ok((vec![0], vec![0])) }
        fn encapsulate_key(&self, _recipient_public_key: &[u8]) -> qsm_prelude::Result<(Vec<u8>, Vec<u8>)> { Ok((vec![0], vec![0])) }
        fn decapsulate_key(&self, _recipient_private_key: &[u8], _encapsulated_key: &[u8]) -> qsm_prelude::Result<Vec<u8>> { Ok(vec![0]) }
        fn box_clone(&self) -> Box<dyn QuantumResistantKeyExchange + Send + Sync> { Box::new(self.clone()) }
    }
    
    // Mock QuantumSecurityManager
    struct MockQuantumSecurityManager;
    impl MockQuantumSecurityManager {
        async fn new(_autonomy: QuantumAutonomyLevel) -> qsm_prelude::Result<Self> { Ok(MockQuantumSecurityManager) }
        async fn adapt_security_posture(&mut self) -> qsm_prelude::Result<()> { Ok(()) }
        async fn establish_quantum_safe_channel(&self, _peer_id: &str) -> qsm_prelude::Result<crate::quantum_security::quantum_safe_channel::QuantumSafeChannel> {
            Ok(crate::quantum_security::quantum_safe_channel::QuantumSafeChannel {
                peer_id: _peer_id.to_string(),
                session_key: Arc::new(Mutex::new(vec![0])),
                kex_algo: Box::new(MockQuantumResistantKeyExchange),
                cipher_algo: Box::new(MockQuantumResistantCipher),
                evas_policy: crate::quantum_security::mod::EvasCompliancePolicy {
                    enabled: false,
                    data_minimization_active: false,
                    consent_audited: false,
                },
                last_key_rotation: Arc::new(Mutex::new(chrono::Utc::now())),
                rekey_interval_seconds: 3600,
                quantum_random_seed_used: false,
            })
        }
        async fn protect_data(&self, plaintext: &[u8], _context: &str) -> qsm_prelude::Result<Vec<u8>> { Ok(plaintext.to_vec()) }
        async fn unprotect_data(&self, encrypted_data: &[u8], _context: &str) -> qsm_prelude::Result<Vec<u8>> { Ok(encrypted_data.to_vec()) }
    }


    #[tokio::test]
    async fn test_agi_core_initialization() {
        let config = AGIConfig::default();
        let (tx, _rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx.clone());
        
        let db_service = Arc::new(MockDatabaseService::new("mock_url", "mock_db").await.unwrap()); // Mock DB

        // Mock QSM
        let mock_qsm = Arc::new(Mutex::new(MockQuantumSecurityManager::new(QuantumAutonomyLevel::FullAuto).await.unwrap()));

        let autonomy_policy_engine = MockAutonomyPolicyEngine::new(config.clone(), telemetry_logger.clone(), db_service.clone());
        let meta_learning_service = MockMetaLearningService::new(config.clone(), telemetry_logger.clone(), db_service.clone(), mock_qsm.clone()); // Pass mock_qsm
        let project_manager_service = MockProjectManagerService::new(config.clone(), telemetry_logger.clone());
        let agent_orchestration_engine = MockAgentOrchestrationEngine::new(config.clone(), telemetry_logger.clone());
        let news_engine = MockNewsEngine::new(config.clone(), telemetry_logger.clone(), db_service.clone());

        let agi_core = TariroAGICore::new(
            config,
            telemetry_logger,
            autonomy_policy_engine,
            meta_learning_service,
            project_manager_service,
            agent_orchestration_engine,
            news_engine,
            mock_qsm.clone(), // Pass mock_qsm
        );

        let result = agi_core.initialize().await;
        assert!(result.is_ok(), "AGI Core initialization failed: {:?}", result.err());

        // You could also test parts of the run loop here if desired,
        // but it's often more complex due to the infinite loop.
    }
}
