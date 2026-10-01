//! Database service with MongoDB integration

use crate::prelude::*;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection, Database,
};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

use tariro_sdk::Result;

/// Database service trait for abstraction
#[async_trait]
pub trait DatabaseServiceTrait: Send + Sync {
    async fn new(db_url: &str, db_name: &str) -> Result<Self>
    where
        Self: Sized;

    async fn list<T: for<'de> DeserializeOwned + Send + 'static>(&self) -> Result<Vec<T>>;

    async fn create<T: Serialize + Send + 'static>(&self, data: &mut T) -> Result<String>;

    async fn find_by_id<T: for<'de> DeserializeOwned + Send + 'static>(
        &self,
        id: &str
    ) -> Result<Option<T>>;

    async fn filter<T: for<'de> DeserializeOwned + Send + 'static>(
        &self,
        filter: Option<Document>
    ) -> Result<Vec<T>>;

    async fn update<T: Serialize + Send + 'static>(
        &self,
        id: &str,
        data: &T
    ) -> Result<bool>;

    async fn delete(&self, id: &str) -> Result<bool>;
}

/// MongoDB Database Service implementation
pub struct DatabaseService {
    client: Arc<Mutex<Client>>,
    db: Database,
    collections: Mutex<std::collections::HashMap<String, String>>,
}

impl DatabaseService {
    /// Create a new database service
    pub async fn new(db_url: &str, db_name: &str) -> Result<Self> {
        info!("Connecting to MongoDB at {}...", db_url);

        let client_options = ClientOptions::parse(db_url).await
            .map_err(|e| anyhow::anyhow!("Failed to parse MongoDB URL: {}", e))?;

        let client = Client::with_options(client_options)
            .map_err(|e| anyhow::anyhow!("Failed to create MongoDB client: {}", e))?;

        let db = client.database(db_name);

        info!("Successfully connected to MongoDB database: {}", db_name);

        Ok(Self {
            client: Arc::new(Mutex::new(client)),
            db,
            collections: Mutex::new(std::collections::HashMap::new()),
        })
    }

    /// Get a collection by type name
    fn get_collection_name<T: 'static>() -> String {
        let type_name = std::any::type_name::<T>();
        type_name.split("::")
            .last()
            .unwrap_or("unknown")
            .to_lowercase()
    }

    /// Get a typed collection
    pub async fn get_collection<T: Send + Sync + 'static>(&self) -> Collection<T> {
        let name = Self::get_collection_name::<T>();
        self.db.collection(&name)
    }
}

#[async_trait]
impl DatabaseServiceTrait for DatabaseService {
    async fn new(db_url: &str, db_name: &str) -> Result<Self> {
        Self::new(db_url, db_name).await
    }

    async fn list<T: for<'de> DeserializeOwned + Send + 'static>(&self) -> Result<Vec<T>> {
        let collection: Collection<T> = self.get_collection().await;
        let mut cursor = collection.find(doc! {}).await
            .map_err(|e| anyhow::anyhow!("Failed to list documents: {}", e))?;

        let mut results = Vec::new();
        while cursor.advance().await
            .map_err(|e| anyhow::anyhow!("Failed to advance cursor: {}", e))? {
            results.push(cursor.deserialize_current()
                .map_err(|e| anyhow::anyhow!("Failed to deserialize: {}", e))?);
        }

        Ok(results)
    }

    async fn create<T: Serialize + Send + 'static>(&self, data: &mut T) -> Result<String> {
        let collection: Collection<T> = self.get_collection().await;

        let result = collection.insert_one(data).await
            .map_err(|e| anyhow::anyhow!("Failed to insert document: {}", e))?;

        let id = result.inserted_id.as_object_id()
            .map(|oid| oid.to_hex())
            .unwrap_or_else(|| "unknown".to_string());

        info!("Created document with ID: {}", id);
        Ok(id)
    }

    async fn find_by_id<T: for<'de> DeserializeOwned + Send + 'static>(
        &self,
        id: &str
    ) -> Result<Option<T>> {
        let collection: Collection<T> = self.get_collection().await;

        use mongodb::bson::oid::ObjectId;
        let oid = ObjectId::parse_str(id)
            .map_err(|e| anyhow::anyhow!("Invalid ObjectId: {}", e))?;

        let result = collection.find_one(doc! { "_id": oid }).await
            .map_err(|e| anyhow::anyhow!("Failed to find document: {}", e))?;

        Ok(result)
    }

    async fn filter<T: for<'de> DeserializeOwned + Send + 'static>(
        &self,
        filter: Option<Document>
    ) -> Result<Vec<T>> {
        let collection: Collection<T> = self.get_collection().await;

        let filter_doc = filter.unwrap_or(doc! {});
        let mut cursor = collection.find(filter_doc).await
            .map_err(|e| anyhow::anyhow!("Failed to filter documents: {}", e))?;

        let mut results = Vec::new();
        while cursor.advance().await
            .map_err(|e| anyhow::anyhow!("Failed to advance cursor: {}", e))? {
            results.push(cursor.deserialize_current()
                .map_err(|e| anyhow::anyhow!("Failed to deserialize: {}", e))?);
        }

        Ok(results)
    }

    async fn update<T: Serialize + Send + 'static>(
        &self,
        id: &str,
        data: &T
    ) -> Result<bool> {
        let collection: Collection<T> = self.get_collection().await;

        use mongodb::bson::oid::ObjectId;
        let oid = ObjectId::parse_str(id)
            .map_err(|e| anyhow::anyhow!("Invalid ObjectId: {}", e))?;

        let result = collection.replace_one(doc! { "_id": oid }, data).await
            .map_err(|e| anyhow::anyhow!("Failed to update document: {}", e))?;

        Ok(result.modified_count > 0)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let collection: Collection<Document> = self.db.collection("generic");

        use mongodb::bson::oid::ObjectId;
        let oid = ObjectId::parse_str(id)
            .map_err(|e| anyhow::anyhow!("Invalid ObjectId: {}", e))?;

        let result = collection.delete_one(doc! { "_id": oid }).await
            .map_err(|e| anyhow::anyhow!("Failed to delete document: {}", e))?;

        Ok(result.deleted_count > 0)
    }
}

impl Clone for DatabaseService {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            db: self.db.clone(),
            collections: Mutex::new(std::collections::HashMap::new()),
        }
    }
}