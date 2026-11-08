// src/database.rs

//! Manages database connections and CRUD operations for Tariro AGI entities.

use crate::prelude::*;
use mongodb::{options::ClientOptions, Client, Collection, Database};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use async_trait::async_trait;

// Re-export entity types from the SDK
pub use tariro_sdk::entities::{AGICategory, AGIControlSetting, AutonomyPolicy, CapabilityStatus, CompilerProject, Conversation, DeploymentRecord, ExplainabilityLog, FeatureProposal, GitHubProfile, GitHubProjectContext, Goal, IDEProfile, KnowledgeItem, LearningInsight, ManagedFile, MetaProject, NanoAgent, NotificationRecord, PerformanceLog, ProjectTask, ServicePrice, SystemHealthLog, WellbeingLog, WorkflowRule};

// A trait for any entity that can be stored in the database.
pub trait DbEntity: Serialize + for<'de> Deserialize<'de> + Debug + Unpin + Send + Sync {
    const COLLECTION_NAME: &'static str;
    fn id_as_string(&self) -> Option<String>;

    /// Mutable access to the entity's identifier.
    ///
    /// Used to assign a generated id the first time an entity is stored.
    fn id_as_string_mut(&mut self) -> Option<&mut Option<String>>;
}

/// Access to an entity identifier that is stored in more than one shape.
///
/// The SDK entities use three different identifier layouts: most store a
/// mandatory `String`, `AGIControlSetting` stores an `Option<String>`, and a
/// couple of entities have no identifier of their own. These helpers let the
/// `DbEntity` implementation work for all of them.
trait EntityId {
    /// The identifier, if the entity has one.
    fn id_option(&self) -> Option<String>;

    /// Mutable access to the identifier, if the entity has an assignable one.
    fn id_option_mut(&mut self) -> Option<&mut Option<String>>;
}

impl EntityId for String {
    fn id_option(&self) -> Option<String> {
        Some(self.clone())
    }

    fn id_option_mut(&mut self) -> Option<&mut Option<String>> {
        // A mandatory identifier cannot be filled in by the storage layer.
        None
    }
}

impl EntityId for Option<String> {
    fn id_option(&self) -> Option<String> {
        self.clone()
    }

    fn id_option_mut(&mut self) -> Option<&mut Option<String>> {
        Some(self)
    }
}

/// Implements [`DbEntity`] for SDK types that carry an identifier.
macro_rules! impl_db_entity {
    ($($struct_name:ident),* $(,)?) => {
        $(impl DbEntity for $struct_name {
            const COLLECTION_NAME: &'static str = stringify!($struct_name);
            fn id_as_string(&self) -> Option<String> {
                EntityId::id_option(&self.id)
            }
            fn id_as_string_mut(&mut self) -> Option<&mut Option<String>> {
                EntityId::id_option_mut(&mut self.id)
            }
        })*
    };
}

/// Implements [`DbEntity`] for SDK types that have no identifier field.
macro_rules! impl_db_entity_without_id {
    ($($struct_name:ident),* $(,)?) => {
        $(impl DbEntity for $struct_name {
            const COLLECTION_NAME: &'static str = stringify!($struct_name);
            fn id_as_string(&self) -> Option<String> {
                None
            }
            fn id_as_string_mut(&mut self) -> Option<&mut Option<String>> {
                None
            }
        })*
    };
}

impl_db_entity!(
    AGIControlSetting, AutonomyPolicy, CompilerProject, Conversation, DeploymentRecord,
    ExplainabilityLog, FeatureProposal, GitHubProfile, Goal, IDEProfile, KnowledgeItem,
    LearningInsight, ManagedFile, MetaProject, NanoAgent, NotificationRecord, PerformanceLog,
    ProjectTask, SystemHealthLog, WellbeingLog, WorkflowRule
);

impl_db_entity_without_id!(GitHubProjectContext, ServicePrice);

/// `DatabaseService` manages the connection to MongoDB and provides CRUD operations.
pub struct DatabaseService {
    db: Database,
}

impl DatabaseService {
    /// Creates a new `DatabaseService` instance and connects to MongoDB.
    pub async fn new(db_uri: &str, db_name: &str) -> Result<Self, String> {
        let client_options = ClientOptions::parse(db_uri).await
            .map_err(|e| format!("Failed to parse MongoDB URI: {}", e))?;
        let client = Client::with_options(client_options)
            .map_err(|e| format!("Failed to create MongoDB client: {}", e))?;

        let db = client.database(db_name);
        println!("[DatabaseService] Connected to MongoDB database: {}", db_name);
        Ok(Self { db })
    }

    /// Generic function to get a collection.
    fn collection<T: DbEntity>(&self) -> Collection<T> {
        self.db.collection(T::COLLECTION_NAME)
    }

    /// Inserts a new entity into the database.
    pub async fn create<T: DbEntity + Debug>(&self, entity: &mut T) -> Result<String, String> {
        let collection = self.collection::<T>();
        // Assign a new UUID if the entity does not have an ID
        if entity.id_as_string().is_none() {
            let id_field = Self::get_id_field_mut(entity);
            if let Some(id_field) = id_field {
                *id_field = Some(uuid::Uuid::new_v4().to_string());
            }
        }

        let result = collection.insert_one(entity, None).await
            .map_err(|e| format!("Failed to create {}: {:?}", T::COLLECTION_NAME, e))?;
        
        let inserted_id = result.inserted_id.as_str().map(|s| s.to_string())
            .unwrap_or_else(|| entity.id_as_string().unwrap_or_else(|| "unknown_id".to_string()));

        println!("[DatabaseService] Created {}: ID = {}", T::COLLECTION_NAME, inserted_id);
        Ok(inserted_id)
    }

    /// Reads an entity by its ID.
    pub async fn read<T: DbEntity + Debug>(&self, id: &str) -> Result<Option<T>, String> {
        use mongodb::bson::doc;
        let collection = self.collection::<T>();
        let filter = doc! { "id": id };
        let entity = collection.find_one(filter, None).await
            .map_err(|e| format!("Failed to read {}: {:?}", T::COLLECTION_NAME, e))?;
        Ok(entity)
    }

    /// Updates an entity by its ID.
    pub async fn update<T: DbEntity + Debug>(&self, id: &str, new_data: &T) -> Result<(), String> {
        use mongodb::bson::doc;
        let collection = self.collection::<T>();
        let filter = doc! { "id": id };
        let update = mongodb::bson::to_document(new_data)
            .map_err(|e| format!("Failed to convert new_data to BSON document: {}", e))?;

        let result = collection.update_one(filter, doc! { "$set": update }, None).await
            .map_err(|e| format!("Failed to update {}: {:?}", T::COLLECTION_NAME, e))?;
        
        if result.matched_count == 0 {
            return Err(format!("No {} found with ID {}.", T::COLLECTION_NAME, id));
        }

        println!("[DatabaseService] Updated {}: ID = {}", T::COLLECTION_NAME, id);
        Ok(())
    }

    /// Deletes an entity by its ID.
    pub async fn delete<T: DbEntity + Debug>(&self, id: &str) -> Result<(), String> {
        use mongodb::bson::doc;
        let collection = self.collection::<T>();
        let filter = doc! { "id": id };
        let result = collection.delete_one(filter, None).await
            .map_err(|e| format!("Failed to delete {}: {:?}", T::COLLECTION_NAME, e))?;
        
        if result.deleted_count == 0 {
            return Err(format!("No {} found with ID {}.", T::COLLECTION_NAME, id));
        }

        println!("[DatabaseService] Deleted {}: ID = {}", T::COLLECTION_NAME, id);
        Ok(())
    }

    /// Lists all entities of a given type.
    pub async fn list<T: DbEntity + Debug>(&self) -> Result<Vec<T>, String> {
        use mongodb::bson::doc;
        let collection = self.collection::<T>();
        let mut cursor = collection.find(doc! {}, None).await
            .map_err(|e| format!("Failed to list {}: {:?}", T::COLLECTION_NAME, e))?;
        
        let mut entities = Vec::new();
        while let Some(result) = cursor.try_next().await {
            entities.push(result.map_err(|e| format!("Failed to parse {} document: {:?}", T::COLLECTION_NAME, e))?);
        }
        Ok(entities)
    }

    // Helper to get a mutable reference to the 'id' field, if it exists and is Option<String>
    fn get_id_field_mut<T: DbEntity>(entity: &mut T) -> Option<&mut Option<String>> {
        entity.id_as_string_mut()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use tariro_sdk::entities::Goal;
    use mongodb::bson::doc;

    const TEST_DB_URI: &str = "mongodb://localhost:27017";
    const TEST_DB_NAME: &str = "tariro_test_db";

    // Helper to get a clean database for testing
    async fn setup_test_db() -> DatabaseService {
        let db_service = DatabaseService::new(TEST_DB_URI, TEST_DB_NAME).await.unwrap();
        // Clean up previous test data
        db_service.db.drop(None).await.unwrap();
        DatabaseService::new(TEST_DB_URI, TEST_DB_NAME).await.unwrap()
    }

    #[tokio::test]
    async fn test_database_connection() {
        let db_service = setup_test_db().await;
        assert!(!db_service.db.name().is_empty());
    }

    #[tokio::test]
    async fn test_create_and_read_goal() {
        let db_service = setup_test_db().await;
        let mut new_goal = Goal {
            id: None,
            title: "Test Goal".to_string(),
            description: Some("A goal for testing database CRUD.".to_string()),
            domain: Some("personal".to_string()),
            priority: tariro_sdk::entities::GoalPriority::Medium,
            status: tariro_sdk::entities::GoalStatus::Active,
            progress_pct: Some(0.0),
            target_date: None,
            resources_linked: None,
            sub_tasks: None,
            tariro_notes: None,
        };

        let goal_id = db_service.create(&mut new_goal).await.unwrap();
        assert!(!goal_id.is_empty());
        assert_eq!(new_goal.id, Some(goal_id.clone())); // Ensure ID is set in the entity

        let fetched_goal = db_service.read::<Goal>(&goal_id).await.unwrap().unwrap();
        assert_eq!(fetched_goal.title, "Test Goal");
        assert_eq!(fetched_goal.id, Some(goal_id.clone()));
    }

    #[tokio::test]
    async fn test_update_goal() {
        let db_service = setup_test_db().await;
        let mut new_goal = Goal {
            id: None,
            title: "Goal to Update".to_string(),
            description: None,
            domain: None,
            priority: tariro_sdk::entities::GoalPriority::Low,
            status: tariro_sdk::entities::GoalStatus::Active,
            progress_pct: Some(0.0),
            target_date: None,
            resources_linked: None,
            sub_tasks: None,
            tariro_notes: None,
        };
        let goal_id = db_service.create(&mut new_goal).await.unwrap();

        let updated_goal = Goal {
            id: Some(goal_id.clone()), // ID must be set for update
            title: "Updated Goal Title".to_string(),
            description: Some("New description.".to_string()),
            domain: Some("professional".to_string()),
            priority: tariro_sdk::entities::GoalPriority::High,
            status: tariro_sdk::entities::GoalStatus::InProgress,
            progress_pct: Some(50.0),
            target_date: None,
            resources_linked: None,
            sub_tasks: None,
            tariro_notes: None,
        };

        db_service.update(&goal_id, &updated_goal).await.unwrap();

        let fetched_goal = db_service.read::<Goal>(&goal_id).await.unwrap().unwrap();
        assert_eq!(fetched_goal.title, "Updated Goal Title");
        assert_eq!(fetched_goal.description, Some("New description.".to_string()));
        assert_eq!(fetched_goal.progress_pct, Some(50.0));
    }

    #[tokio::test]
    async fn test_delete_goal() {
        let db_service = setup_test_db().await;
        let mut new_goal = Goal {
            id: None,
            title: "Goal to Delete".to_string(),
            description: None,
            domain: None,
            priority: tariro_sdk::entities::GoalPriority::Low,
            status: tariro_sdk::entities::GoalStatus::Active,
            progress_pct: Some(0.0),
            target_date: None,
            resources_linked: None,
            sub_tasks: None,
            tariro_notes: None,
        };
        let goal_id = db_service.create(&mut new_goal).await.unwrap();

        db_service.delete::<Goal>(&goal_id).await.unwrap();
        let fetched_goal = db_service.read::<Goal>(&goal_id).await.unwrap();
        assert!(fetched_goal.is_none());
    }

    #[tokio::test]
    async fn test_list_goals() {
        let db_service = setup_test_db().await;
        let mut goal1 = Goal {
            id: None, title: "List Goal 1".to_string(), description: None, domain: None,
            priority: tariro_sdk::entities::GoalPriority::Medium, status: tariro_sdk::entities::GoalStatus::Active,
            progress_pct: Some(0.0), target_date: None, resources_linked: None, sub_tasks: None, tariro_notes: None,
        };
        let mut goal2 = Goal {
            id: None, title: "List Goal 2".to_string(), description: None, domain: None,
            priority: tariro_sdk::entities::GoalPriority::Medium, status: tariro_sdk::entities::GoalStatus::Active,
            progress_pct: Some(0.0), target_date: None, resources_linked: None, sub_tasks: None, tariro_notes: None,
        };
        db_service.create(&mut goal1).await.unwrap();
        db_service.create(&mut goal2).await.unwrap();

        let goals = db_service.list::<Goal>().await.unwrap();
        assert_eq!(goals.len(), 2);
        assert!(goals.iter().any(|g| g.title == "List Goal 1"));
        assert!(goals.iter().any(|g| g.title == "List Goal 2"));
    }
}
