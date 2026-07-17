// src/meta_learning.rs

//! Implements meta-learning capabilities for Tariro AGI, allowing it to learn about its own learning processes.

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};
use crate::database::DatabaseService;
use crate::quantum_security::mod::{QuantumSecurityManager, QuantumAutonomyLevel};
use crate::quantum_security::mod::prelude as qsm_prelude;

/// Represents a learned insight about Tariro's own operation, user interactions, or external world.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LearningInsight {
    pub id: Option<String>,
    pub insight: String,
    pub category: LearningInsightCategory,
    pub confidence: f32, // Confidence score from 0.0 to 1.0
    pub source_conversation_id: Option<String>, // Where the insight was derived from
    pub times_reinforced: u32, // How many times this insight has been observed/confirmed
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Categories for learning insights.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LearningInsightCategory {
    Preference,        // User preferences, AGI's own operational preferences
    Fact,              // New factual knowledge
    Behavior,          // Behavioral patterns (user or AGI itself)
    TopicInterest,     // User or AGI's interest in specific topics
    CommunicationStyle,// Preferred communication style
    SystemOptimization,// Insights leading to self-modification/optimization
    EthicalGuideline,  // Refinement of ethical frameworks
    KnowledgeGap,      // Identification of missing knowledge
    PredictionModel,   // Learned predictive patterns
    Custom(String),    // User-defined or dynamic categories
}

impl ToString for LearningInsightCategory {
    fn to_string(&self) -> String {
        match self {
            LearningInsightCategory::Preference => "Preference".to_string(),
            LearningInsightCategory::Fact => "Fact".to_string(),
            LearningInsightCategory::Behavior => "Behavior".to_string(),
            LearningInsightCategory::TopicInterest => "TopicInterest".to_string(),
            LearningInsightCategory::CommunicationStyle => "CommunicationStyle".to_string(),
            LearningInsightCategory::SystemOptimization => "SystemOptimization".to_string(),
            LearningInsightCategory::EthicalGuideline => "EthicalGuideline".to_string(),
            LearningInsightCategory::KnowledgeGap => "KnowledgeGap".to_string(),
            LearningInsightCategory::PredictionModel => "PredictionModel".to_string(),
            LearningInsightCategory::Custom(s) => s.clone(),
        }
    }
}

#[async_trait]
pub trait MetaLearningServiceTrait: Send + Sync {
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, db_service: Arc<DatabaseService>, qsm: Arc<Mutex<QuantumSecurityManager>>) -> Self;
    async fn initialize(&self) -> qsm_prelude::Result<()>;
    async fn shutdown(&self) -> qsm_prelude::Result<()>;
    async fn process_insight(&mut self, insight: LearningInsight) -> qsm_prelude::Result<()>;
    async fn get_insights(&self, category: Option<LearningInsightCategory>) -> Vec<&LearningInsight>;
    async fn identify_knowledge_gaps(&self) -> qsm_prelude::Result<Vec<String>>; 
    async fn check_for_learning_opportunities(&self);
    async fn consolidate_insights(&mut self) -> qsm_prelude::Result<u32>; 
}

/// The `MetaLearningService` is responsible for processing, storing, and acting upon learning insights.
pub struct MetaLearningService {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    db_service: Arc<DatabaseService>,
    insights: Mutex<HashMap<String, LearningInsight>>, 
    qsm: Arc<Mutex<QuantumSecurityManager>>, 
}

#[async_trait]
impl MetaLearningServiceTrait for MetaLearningService {
    /// Creates a new `MetaLearningService` instance.
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, db_service: Arc<DatabaseService>, qsm: Arc<Mutex<QuantumSecurityManager>>) -> Self { 
        MetaLearningService {
            config,
            telemetry_logger,
            db_service,
            insights: Mutex::new(HashMap::new()), 
            qsm, 
        }
    }

    async fn initialize(&self) -> qsm_prelude::Result<()> { 
        info!("[MetaLearningService] Initializing...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceInitialized { service_name: "MetaLearningService".to_string() }
        ).await;
        // Load existing insights from the database
        match self.db_service.list::<LearningInsight>().await {
            Ok(loaded_insights) => {
                let mut s_insights = self.insights.lock().unwrap(); 
                for insight in loaded_insights {
                    if let Some(id) = &insight.id {
                        s_insights.insert(id.clone(), insight);
                    }
                }
                info!("[MetaLearningService] Loaded {} insights from DB.", s_insights.len());
            },
            Err(e) => error!("[MetaLearningService] Failed to load insights from DB: {}", e), 
        }
        Ok(())
    }

    async fn shutdown(&self) -> qsm_prelude::Result<()> { 
        info!("[MetaLearningService] Shutting down...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceShutdown { service_name: "MetaLearningService".to_string() }
        ).await;
        // No explicit save needed here if changes are persisted immediately.
        Ok(())
    }

    /// Processes a new learning insight, stores it, and potentially triggers further actions.
    async fn process_insight(&mut self, mut insight: LearningInsight) -> qsm_prelude::Result<()> { 
        if insight.id.is_none() {
            insight.id = Some(Uuid::new_v4().to_string());
        }
        insight.created_at = Utc::now();
        insight.last_updated = Utc::now();

        info!("[MetaLearningService] Processing new insight: {:?}", insight);
        
        // NEW: Protect the insight content using QSM before storing (if sensitive)
        let qsm_locked = self.qsm.lock().await;
        let protected_insight_content = qsm_locked.protect_data(
            insight.insight.as_bytes(),
            &format!("meta_learning_insight_{}", insight.id.as_ref().unwrap_or(&"unknown".to_string()))
        ).await?;
        
        // Store in DB - A real implementation might store the protected_insight_content in DB.
        // For this example, we'll keep the original insight for simplicity in DB, but log the protection.
        let insight_id = self.db_service.create(&mut insight).await?;
        insight.id = Some(insight_id.clone()); 
        self.insights.lock().unwrap().insert(insight_id.clone(), insight.clone()); 

        info!("[MetaLearningService] Insight data also quantum-protected. Original: {} bytes, Protected: {} bytes.", 
            insight.insight.len(), protected_insight_content.len()); 

        self.telemetry_logger.log_event(TelemetryEvent::LearningInsightProcessed {
            insight_id: insight_id.clone(),
            category: insight.category.to_string(),
        }).await;

        // Example of meta-learning: if insight is about system optimization, propose a feature
        if insight.category == LearningInsightCategory::SystemOptimization {
            // In a real system, this would interact with a FeatureProposal service
            info!("[MetaLearningService] Insight suggests system optimization. Proposing feature...");
            self.telemetry_logger.log_event(TelemetryEvent::FeatureProposed {
                title: format!("Optimize based on insight: {}", insight.insight.chars().take(50).collect::<String>()),
                source: "MetaLearningService".to_string(),
            }).await;
        }

        Ok(())
    }

    /// Retrieves stored learning insights, optionally filtered by category.
    async fn get_insights(&self, category: Option<LearningInsightCategory>) -> Vec<&LearningInsight> {
        self.insights.lock().unwrap().values().filter(|insight| {
            category.as_ref().map_or(true, |cat| &insight.category == cat)
        }).collect()
    }

    /// Identifies potential knowledge gaps based on existing insights and system goals.
    async fn identify_knowledge_gaps(&self) -> qsm_prelude::Result<Vec<String>> { 
        info!("[MetaLearningService] Identifying knowledge gaps...");
        self.telemetry_logger.log_event(TelemetryEvent::KnowledgeGapIdentified {
            description: "Initiated knowledge gap analysis.".to_string(),
        }).await;

        // Simple simulation: if we don't have enough 'Fact' insights on a critical topic, it's a gap.
        let fact_insights_count = self.get_insights(Some(LearningInsightCategory::Fact)).await.len();
        if fact_insights_count < 10 {
            Ok(vec!["Insufficient factual knowledge on core operational domains.".to_string()])
        } else {
            Ok(vec![])
        }
    }

    /// Periodically checks for new learning opportunities or triggers internal learning processes.
    async fn check_for_learning_opportunities(&self) {
        info!("[MetaLearningService] Checking for learning opportunities...");
        self.telemetry_logger.log_event(TelemetryEvent::PeriodicCheck {
            check_name: "LearningOpportunitiesCheck".to_string(),
        }).await;
        // This could involve:
        // - Analyzing performance logs for areas of improvement
        // - Reviewing failed tasks for lessons learned
        // - Monitoring external data sources for new information
    }

    /// Consolidates similar or redundant insights to maintain a lean and efficient knowledge base.
    async fn consolidate_insights(&mut self) -> qsm_prelude::Result<u32> { 
        info!("[MetaLearningService] Consolidating insights...");
        let mut insights_locked = self.insights.lock().unwrap();
        let initial_count = insights_locked.len();
        let mut consolidated_count = 0;
        let mut insights_to_keep: HashMap<String, LearningInsight> = HashMap::new();

        // Very simplistic consolidation: keep only the most recent insight for a given category for simulation
        for insight in insights_locked.values() {
            let key = insight.category.to_string();
            insights_to_keep.entry(key.clone())
                .and_modify(|e| {
                    if insight.created_at > e.created_at {
                        *e = insight.clone();
                        consolidated_count += 1;
                    }
                })
                .or_insert(insight.clone());
        }
        
        *insights_locked = insights_to_keep; 
        info!("[MetaLearningService] Consolidated {} insights. New count: {}.", consolidated_count, insights_locked.len());
        Ok(consolidated_count)
    }
}
