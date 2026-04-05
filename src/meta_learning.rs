// src/meta_learning.rs

//! Implements meta-learning capabilities for Tariro AGI, allowing it to learn about its own learning processes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use async_trait::async_trait;

use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};
use crate::database::DatabaseService;

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
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, db_service: Arc<DatabaseService>) -> Self;
    async fn initialize(&self) -> Result<(), String>;
    async fn shutdown(&self) -> Result<(), String>;
    async fn process_insight(&mut self, insight: LearningInsight) -> Result<(), String>;
    async fn get_insights(&self, category: Option<LearningInsightCategory>) -> Vec<&LearningInsight>;
    async fn identify_knowledge_gaps(&self) -> Result<Vec<String>, String>;
    async fn check_for_learning_opportunities(&self);
    async fn consolidate_insights(&mut self) -> Result<u32, String>; // Returns count of consolidated insights
}

/// The `MetaLearningService` is responsible for processing, storing, and acting upon learning insights.
pub struct MetaLearningService {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    db_service: Arc<DatabaseService>,
    insights: HashMap<String, LearningInsight>,
}

#[async_trait]
impl MetaLearningServiceTrait for MetaLearningService {
    /// Creates a new `MetaLearningService` instance.
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, db_service: Arc<DatabaseService>) -> Self {
        MetaLearningService {
            config,
            telemetry_logger,
            db_service,
            insights: HashMap::new(),
        }
    }

    async fn initialize(&self) -> Result<(), String> {
        println!("[MetaLearningService] Initializing...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceInitialized { service_name: "MetaLearningService".to_string() }
        ).await;
        // Load existing insights from the database
        match self.db_service.list::<LearningInsight>().await {
            Ok(loaded_insights) => {
                let mut s_insights = self.insights.lock().unwrap(); // Access inner HashMap mutably
                for insight in loaded_insights {
                    if let Some(id) = &insight.id {
                        s_insights.insert(id.clone(), insight);
                    }
                }
                println!("[MetaLearningService] Loaded {} insights from DB.", s_insights.len());
            },
            Err(e) => eprintln!("[MetaLearningService] Failed to load insights from DB: {}", e),
        }
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), String> {
        println!("[MetaLearningService] Shutting down...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceShutdown { service_name: "MetaLearningService".to_string() }
        ).await;
        // No explicit save needed here if changes are persisted immediately.
        Ok(())
    }

    /// Processes a new learning insight, stores it, and potentially triggers further actions.
    async fn process_insight(&mut self, mut insight: LearningInsight) -> Result<(), String> {
        if insight.id.is_none() {
            insight.id = Some(Uuid::new_v4().to_string());
        }
        insight.created_at = Utc::now();
        insight.last_updated = Utc::now();

        println!("[MetaLearningService] Processing new insight: {:?}", insight);
        
        // Store in DB
        let insight_id = self.db_service.create(&mut insight).await?;
        insight.id = Some(insight_id.clone()); // Ensure the ID is set back to the insight
        self.insights.insert(insight_id.clone(), insight.clone());

        self.telemetry_logger.log_event(TelemetryEvent::LearningInsightProcessed {
            insight_id: insight_id.clone(),
            category: insight.category.to_string(),
        }).await;

        // Example of meta-learning: if insight is about system optimization, propose a feature
        if insight.category == LearningInsightCategory::SystemOptimization {
            // In a real system, this would interact with a FeatureProposal service
            println!("[MetaLearningService] Insight suggests system optimization. Proposing feature...");
            self.telemetry_logger.log_event(TelemetryEvent::FeatureProposed {
                title: format!("Optimize based on insight: {}", insight.insight.chars().take(50).collect::<String>()),
                source: "MetaLearningService".to_string(),
            }).await;
        }

        Ok(())
    }

    /// Retrieves stored learning insights, optionally filtered by category.
    async fn get_insights(&self, category: Option<LearningInsightCategory>) -> Vec<&LearningInsight> {
        self.insights.values().filter(|insight| {
            category.as_ref().map_or(true, |cat| &insight.category == cat)
        }).collect()
    }

    /// Identifies potential knowledge gaps based on existing insights and system goals.
    async fn identify_knowledge_gaps(&self) -> Result<Vec<String>, String> {
        println!("[MetaLearningService] Identifying knowledge gaps...");
        self.telemetry_logger.log_event(TelemetryEvent::KnowledgeGapIdentified {
            description: "Initiated knowledge gap analysis.".to_string(),
        }).await;

        // Simple simulation: if we don't have enough 'Fact' insights on a critical topic, it's a gap.
        let fact_insights_count = self.get_insights(Some(LearningInsightCategory::Fact)).await.len();
        if fact_insights_count < 10 {
            Ok(vec!["Insufficient factual knowledge on core operational domains.".to_string()])
        } else {
            Ok(vec![]))
        }
    }

    /// Periodically checks for new learning opportunities or triggers internal learning processes.
    async fn check_for_learning_opportunities(&self) {
        println!("[MetaLearningService] Checking for learning opportunities...");
        self.telemetry_logger.log_event(TelemetryEvent::PeriodicCheck {
            check_name: "LearningOpportunitiesCheck".to_string(),
        }).await;
        // This could involve:
        // - Analyzing performance logs for areas of improvement
        // - Reviewing failed tasks for lessons learned
        // - Monitoring external data sources for new information
    }

    /// Consolidates similar or redundant insights to maintain a lean and efficient knowledge base.
    async fn consolidate_insights(&mut self) -> Result<u32, String> {
        println!("[MetaLearningService] Consolidating insights...");
        let initial_count = self.insights.len();
        let mut consolidated_count = 0;
        let mut insights_to_keep: HashMap<String, LearningInsight> = HashMap::new();

        // Very simplistic consolidation: keep only the most recent insight for a given category for simulation
        for insight in self.insights.values() {
            let key = insight.category.to_string();
            insights_to_keep.entry(key.clone())
                .and_modify(|e| {
                    if insight.created_at > e.created_at {
                        *e = insight.clone();
                        consolidated_count += 1;
                    }
                })
                .or_insert_with(|| insight.clone());
        }
        
        let removed_count = (initial_count - insights_to_keep.len()) as u32;

        self.insights = insights_to_keep;

        println!("[MetaLearningService] Consolidated {} insights. Total remaining: {}", removed_count, self.insights.len());
        Ok(removed_count)
    }
}

// To make `insights` mutable within `initialize`, `get_insights`, etc. without `&mut self`,
// we need to wrap it in a `Mutex` or `RwLock`.
// Let's modify `MetaLearningService` to use a `Mutex` for `insights`.

use std::sync::Mutex as StdMutex;

pub struct MetaLearningService {
    config: AGIConfig,
    telemetry_logger: TelemetryLogger,
    db_service: Arc<DatabaseService>,
    insights: Arc<StdMutex<HashMap<String, LearningInsight>>>,
}

#[async_trait]
impl MetaLearningServiceTrait for MetaLearningService {
    fn new(config: AGIConfig, telemetry_logger: TelemetryLogger, db_service: Arc<DatabaseService>) -> Self {
        MetaLearningService {
            config,
            telemetry_logger,
            db_service,
            insights: Arc::new(StdMutex::new(HashMap::new())),
        }
    }

    async fn initialize(&self) -> Result<(), String> {
        println!("[MetaLearningService] Initializing...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceInitialized { service_name: "MetaLearningService".to_string() }
        ).await;
        // Load existing insights from the database
        match self.db_service.list::<LearningInsight>().await {
            Ok(loaded_insights) => {
                let mut s_insights = self.insights.lock().unwrap(); // Access inner HashMap mutably
                for insight in loaded_insights {
                    if let Some(id) = &insight.id {
                        s_insights.insert(id.clone(), insight);
                    }
                }
                println!("[MetaLearningService] Loaded {} insights from DB.", s_insights.len());
            },
            Err(e) => eprintln!("[MetaLearningService] Failed to load insights from DB: {}", e),
        }
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), String> {
        println!("[MetaLearningService] Shutting down...");
        self.telemetry_logger.log_event(
            TelemetryEvent::ServiceShutdown { service_name: "MetaLearningService".to_string() }
        ).await;
        Ok(()) // No explicit save needed here if changes are persisted immediately.
    }

    async fn process_insight(&mut self, mut insight: LearningInsight) -> Result<(), String> {
        if insight.id.is_none() {
            insight.id = Some(Uuid::new_v4().to_string());
        }
        insight.created_at = Utc::now();
        insight.last_updated = Utc::now();

        println!("[MetaLearningService] Processing new insight: {:?}", insight);
        
        // Store in DB
        let insight_id = self.db_service.create(&mut insight).await?;
        insight.id = Some(insight_id.clone());
        self.insights.lock().unwrap().insert(insight_id.clone(), insight.clone());

        self.telemetry_logger.log_event(TelemetryEvent::LearningInsightProcessed {
            insight_id: insight_id.clone(),
            category: insight.category.to_string(),
        }).await;

        if insight.category == LearningInsightCategory::SystemOptimization {
            println!("[MetaLearningService] Insight suggests system optimization. Proposing feature...");
            self.telemetry_logger.log_event(TelemetryEvent::FeatureProposed {
                title: format!("Optimize based on insight: {}", insight.insight.chars().take(50).collect::<String>()),
                source: "MetaLearningService".to_string(),
            }).await;
        }

        Ok(())
    }

    async fn get_insights(&self, category: Option<LearningInsightCategory>) -> Vec<&LearningInsight> {
        let s_insights = self.insights.lock().unwrap();
        s_insights.values().filter(|insight| {
            category.as_ref().map_or(true, |cat| &insight.category == cat)
        }).collect()
    }

    async fn identify_knowledge_gaps(&self) -> Result<Vec<String>, String> {
        println!("[MetaLearningService] Identifying knowledge gaps...");
        self.telemetry_logger.log_event(TelemetryEvent::KnowledgeGapIdentified {
            description: "Initiated knowledge gap analysis.".to_string(),
        }).await;

        let fact_insights_count = self.get_insights(Some(LearningInsightCategory::Fact)).await.len();
        if fact_insights_count < 10 {
            Ok(vec!["Insufficient factual knowledge on core operational domains.".to_string()])
        } else {
            Ok(vec![])
        }
    }

    async fn check_for_learning_opportunities(&self) {
        println!("[MetaLearningService] Checking for learning opportunities...");
        self.telemetry_logger.log_event(TelemetryEvent::PeriodicCheck {
            check_name: "LearningOpportunitiesCheck".to_string(),
        }).await;
    }

    async fn consolidate_insights(&mut self) -> Result<u32, String> {
        println!("[MetaLearningService] Consolidating insights...");
        let mut s_insights = self.insights.lock().unwrap();
        let initial_count = s_insights.len();
        let mut insights_to_keep: HashMap<String, LearningInsight> = HashMap::new();

        for insight in s_insights.values() {
            let key = insight.category.to_string();
            insights_to_keep.entry(key.clone())
                .and_modify(|e| {
                    if insight.created_at > e.created_at {
                        *e = insight.clone();
                    }
                })
                .or_insert_with(|| insight.clone());
        }
        
        let removed_count = (initial_count - insights_to_keep.len()) as u32;
        *s_insights = insights_to_keep; // Replace the old map with the consolidated one

        println!("[MetaLearningService] Consolidated {} insights. Total remaining: {}", removed_count, s_insights.len());
        Ok(removed_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use crate::database::DatabaseService;
    use std::sync::Arc;
    use chrono::Duration;

    // Mock DB Service for tests
    struct MockDbService {
        insights: StdMutex<HashMap<String, LearningInsight>>,
    }

    #[async_trait]
    impl DatabaseService for MockDbService {
        async fn new(_db_uri: &str, _db_name: &str) -> Result<Self, String> {
            Ok(MockDbService { insights: StdMutex::new(HashMap::new()) })
        }

        async fn create<T: tariro_sdk::entities::DbEntity + Debug + Send>(&self, entity: &mut T) -> Result<String, String> {
            if let Some(insight) = (entity as &dyn std::any::Any).downcast_ref::<LearningInsight>() {
                let mut insights = self.insights.lock().unwrap();
                let id = Uuid::new_v4().to_string();
                insights.insert(id.clone(), insight.clone());
                // Manually set ID if possible. This is a weakness of the current SDK trait design for testing.
                Ok(id)
            } else {
                Err("Only LearningInsight supported in mock DB for create".to_string())
            }
        }

        async fn read<T: tariro_sdk::entities::DbEntity + Debug>(&self, id: &str) -> Result<Option<T>, String> {
            if let Some(insight) = self.insights.lock().unwrap().get(id) {
                // Downcast to T, this is unsafe in general, but for tests where we control types.
                let boxed: Box<dyn std::any::Any> = Box::new(insight.clone());
                Ok(Some(*boxed.downcast::<T>().unwrap()))
            } else {
                Ok(None)
            }
        }

        async fn list<T: tariro_sdk::entities::DbEntity + Debug>(&self) -> Result<Vec<T>, String> {
            let insights_vec: Vec<T> = self.insights.lock().unwrap().values().cloned().map(|i| {
                let boxed: Box<dyn std::any::Any> = Box::new(i);
                *boxed.downcast::<T>().unwrap()
            }).collect();
            Ok(insights_vec)
        }

        // Implement other methods if needed for other tests
        async fn update<T: tariro_sdk::entities::DbEntity + Debug>(&self, id: &str, new_data: &T) -> Result<(), String> { Ok(()) }
        async fn delete<T: tariro_sdk::entities::DbEntity + Debug>(&self, id: &str) -> Result<(), String> { Ok(()) }
    }

    fn setup_services() -> (MetaLearningService, mpsc::Receiver<TelemetryEvent>, Arc<MockDbService>) {
        let config = AGIConfig::default();
        let (tx, rx) = mpsc::channel(10);
        let telemetry_logger = TelemetryLogger::new(tx);
        let db_service = Arc::new(MockDbService { insights: StdMutex::new(HashMap::new()) });
        (MetaLearningService::new(config, telemetry_logger, db_service.clone()), rx, db_service)
    }

    #[tokio::test]
    async fn test_process_insight() {
        let (mut service, mut rx, _db) = setup_services();
        let insight = LearningInsight {
            id: None,
            insight: "User prefers concise summaries.".to_string(),
            category: LearningInsightCategory::Preference,
            confidence: 0.9,
            source_conversation_id: Some("conv_123".to_string()),
            times_reinforced: 1,
            created_at: Utc::now(),
            last_updated: Utc::now(),
        };

        let result = service.process_insight(insight.clone()).await;
        assert!(result.is_ok());
        
        let stored_insight = service.insights.lock().unwrap().values().next().unwrap().clone();
        assert_eq!(stored_insight.insight, "User prefers concise summaries.");
        assert!(stored_insight.id.is_some());

        let event = rx.recv().await.unwrap();
        match event {
            TelemetryEvent::LearningInsightProcessed { insight_id, category } => {
                assert_eq!(insight_id, stored_insight.id.unwrap());
                assert_eq!(category, "Preference");
            },
            _ => panic!("Unexpected event"),
        }
    }

    #[tokio::test]
    async fn test_get_insights_filtered() {
        let (mut service, _rx, _db) = setup_services();
        let insight1 = LearningInsight {
            id: Some("id1".to_string()), insight: "fact1".to_string(), category: LearningInsightCategory::Fact,
            confidence: 0.8, source_conversation_id: None, times_reinforced: 1, created_at: Utc::now(), last_updated: Utc::now(),
        };
        let insight2 = LearningInsight {
            id: Some("id2".to_string()), insight: "pref1".to_string(), category: LearningInsightCategory::Preference,
            confidence: 0.9, source_conversation_id: None, times_reinforced: 1, created_at: Utc::now(), last_updated: Utc::now(),
        };
        let insight3 = LearningInsight {
            id: Some("id3".to_string()), insight: "fact2".to_string(), category: LearningInsightCategory::Fact,
            confidence: 0.7, source_conversation_id: None, times_reinforced: 1, created_at: Utc::now() + Duration::seconds(1), last_updated: Utc::now(),
        };

        let mut insights_map = service.insights.lock().unwrap();
        insights_map.insert(insight1.id.clone().unwrap(), insight1);
        insights_map.insert(insight2.id.clone().unwrap(), insight2);
        insights_map.insert(insight3.id.clone().unwrap(), insight3);
        drop(insights_map);

        let facts = service.get_insights(Some(LearningInsightCategory::Fact)).await;
        assert_eq!(facts.len(), 2);
        assert!(facts.iter().all(|i| i.category == LearningInsightCategory::Fact));

        let preferences = service.get_insights(Some(LearningInsightCategory::Preference)).await;
        assert_eq!(preferences.len(), 1);
    }

    #[tokio::test]
    async fn test_consolidate_insights() {
        let (mut service, _rx, _db) = setup_services();
        let now = Utc::now();

        let insight_old = LearningInsight {
            id: Some("id_old".to_string()), insight: "Old Fact".to_string(), category: LearningInsightCategory::Fact,
            confidence: 0.5, source_conversation_id: None, times_reinforced: 1, created_at: now - Duration::days(1), last_updated: now - Duration::days(1),
        };
        let insight_new = LearningInsight {
            id: Some("id_new".to_string()), insight: "Newer Fact".to_string(), category: LearningInsightCategory::Fact,
            confidence: 0.9, source_conversation_id: None, times_reinforced: 2, created_at: now, last_updated: now,
        };
        let insight_other_cat = LearningInsight {
            id: Some("id_other".to_string()), insight: "Other Category".to_string(), category: LearningInsightCategory::Behavior,
            confidence: 0.8, source_conversation_id: None, times_reinforced: 1, created_at: now, last_updated: now,
        };

        let mut insights_map = service.insights.lock().unwrap();
        insights_map.insert(insight_old.id.clone().unwrap(), insight_old);
        insights_map.insert(insight_new.id.clone().unwrap(), insight_new);
        insights_map.insert(insight_other_cat.id.clone().unwrap(), insight_other_cat);
        drop(insights_map);

        let consolidated_count = service.consolidate_insights().await.unwrap();
        assert_eq!(consolidated_count, 1); // One old insight of category 'Fact' should be removed
        assert_eq!(service.insights.lock().unwrap().len(), 2);

        let facts = service.get_insights(Some(LearningInsightCategory::Fact)).await;
        assert_eq!(facts.len(), 1);
        assert_eq!(facts[0].insight, "Newer Fact");
    }
}
