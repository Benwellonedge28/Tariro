// src/news_engine/content_dissemination/web_publisher.rs

//! Provides functionality for publishing curated news content to web platforms,
//! such as a dedicated OMEGA News website, API endpoints, or content management systems.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::news_engine::news_engine::OmegaNewsArticle;

/// Configuration for a web publishing target.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebPublisherConfig {
    pub id: String,
    pub name: String,
    pub endpoint_url: String, // API endpoint or base URL for publishing
    pub auth_token_env_var: Option<String>, // Env var for authentication token
    pub publish_format: PublishFormat,
    pub is_active: bool,
    pub last_published: Option<DateTime<Utc>>,
    pub categories_map: HashMap<String, String>, // Map internal categories to external ones
}

/// Defines the format in which content should be published.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PublishFormat {
    JsonApi,
    HtmlPage,
    Markdown,
    Xml,
}

/// Represents a record of a published article.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishRecord {
    pub id: String,
    pub article_id: String,
    pub publisher_config_id: String,
    pub published_at: DateTime<Utc>,
    pub target_url: String, // The final URL of the published article
    pub status: PublishStatus,
    pub error_message: Option<String>,
}

/// Status of a publishing attempt.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PublishStatus {
    Pending,
    Success,
    Failed,
    Retrying,
}

/// The `WebPublisher` service handles sending articles to various web endpoints.
pub struct WebPublisher {
    publisher_configs: HashMap<String, WebPublisherConfig>,
}

impl WebPublisher {
    /// Creates a new `WebPublisher` instance.
    pub fn new() -> Self {
        WebPublisher {
            publisher_configs: HashMap::new(),
        }
    }

    /// Adds a new web publisher configuration.
    pub fn add_publisher_config(&mut self, mut config: WebPublisherConfig) -> Result<String, String> {
        config.id = format!("web_pub_cfg_{}", Uuid::new_v4());
        println!("[WebPublisher] Adding publisher config: {}", config.name);
        let id = config.id.clone();
        self.publisher_configs.insert(id.clone(), config);
        Ok(id)
    }

    /// Retrieves a publisher config by its ID.
    pub fn get_publisher_config(&self, id: &str) -> Option<&WebPublisherConfig> {
        self.publisher_configs.get(id)
    }

    /// Publishes an `OmegaNewsArticle` to a configured web platform.
    /// This is a simplified simulation of the publishing process.
    pub async fn publish_article(
        &self,
        article: &OmegaNewsArticle,
        config_id: &str,
    ) -> Result<PublishRecord, String> {
        let config = self.publisher_configs.get(config_id)
            .ok_or(format!("Web publisher config {} not found.", config_id))?;

        if !config.is_active { return Err(format!("Web publisher config {} is not active.", config_id)); }

        println!("[WebPublisher] Publishing article \"{}\" to {}...", article.title, config.name);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await; // Simulate network call and processing

        let success = rand::random::<f32>() > 0.1; // 90% chance of success

        let (status, error_message, target_url) = if success {
            (PublishStatus::Success, None, format!("{}/articles/{}", config.endpoint_url, article.id))
        } else {
            (PublishStatus::Failed, Some("Simulated network error or API rejection.".to_string()), "".to_string())
        };

        Ok(PublishRecord {
            id: format!("pub_rec_{}", Uuid::new_v4()),
            article_id: article.id.clone(),
            publisher_config_id: config.id.clone(),
            published_at: Utc::now(),
            target_url,
            status,
            error_message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::news_engine::news_engine::OmegaNewsArticle;
    use crate::news_engine::content_analysis::fact_checker::FactCheckResult;
    use crate::news_engine::content_analysis::fact_checker::FactCheckVerdict;

    fn create_sample_article(title: &str) -> OmegaNewsArticle {
        OmegaNewsArticle {
            id: format!("art_{}", Uuid::new_v4()),
            title: title.to_string(),
            content: "Sample content.".to_string(),
            url: format!("http://example.com/{}", Uuid::new_v4()),
            author: Some("Test Author".to_string()),
            published_date: Utc::now(),
            ingested_date: Utc::now(),
            source_name: "Test Source".to_string(),
            categories: vec!["Tech".to_string()],
            keywords: vec!["test".to_string()],
            sentiment_score: 0.5,
            factual_accuracy_score: 0.8,
            data_lineage: vec![],
            sentiment_analysis: None,
            extracted_entities: vec![],
            fact_check_results: vec![FactCheckResult { id: Uuid::new_v4().to_string(), claim_id: Uuid::new_v4().to_string(), score: 0.8, verdict: FactCheckVerdict::True, evidence_urls: vec![], supporting_statements: vec![], refuting_statements: vec![], checks_performed: vec![], checked_at: Utc::now(), checker_agi_id: "test".to_string() }],
            identified_topics: vec![],
        }
    }

    #[tokio::test]
    async fn test_publish_article_success() {
        let mut publisher = WebPublisher::new();
        let config_id = publisher.add_publisher_config(WebPublisherConfig {
            id: String::new(),
            name: "OMEGA News Website".to_string(),
            endpoint_url: "https://news.omega.com/api".to_string(),
            auth_token_env_var: Some("OMEGA_WEB_PUBLISHER_TOKEN".to_string()),
            publish_format: PublishFormat::JsonApi,
            is_active: true,
            last_published: None,
            categories_map: HashMap::new(),
        }).unwrap();

        let article = create_sample_article("Latest AI Innovations");
        let record = publisher.publish_article(&article, &config_id).await.unwrap();

        assert_eq!(record.status, PublishStatus::Success);
        assert!(!record.target_url.is_empty());
        assert_eq!(record.article_id, article.id);
    }

    #[tokio::test]
    async fn test_publish_article_inactive_config() {
        let mut publisher = WebPublisher::new();
        let config_id = publisher.add_publisher_config(WebPublisherConfig {
            id: String::new(),
            name: "Inactive Website".to_string(),
            endpoint_url: "https://inactive.omega.com/api".to_string(),
            auth_token_env_var: None,
            publish_format: PublishFormat::JsonApi,
            is_active: false,
            last_published: None,
            categories_map: HashMap::new(),
        }).unwrap();

        let article = create_sample_article("Inactive Test");
        let result = publisher.publish_article(&article, &config_id).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not active"));
    }
}
