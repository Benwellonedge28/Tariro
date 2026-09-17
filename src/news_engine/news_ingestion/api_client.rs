// src/news_engine/news_ingestion/api_client.rs
use crate::prelude::*;

//! Provides functionality to interact with various external news APIs (e.g., NewsAPI, Google News API)
//! to fetch structured news data.

use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a news article fetched from a structured news API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiFetchedArticle {
    pub id: String,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub content: Option<String>, // Full content if provided by API
    pub author: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub source_name: String,
    pub source_id: Option<String>,
    pub language: String,
    pub image_url: Option<String>,
    pub api_config_id: String, // ID of the API configuration used
    pub raw_json: String, // Store raw JSON response for reprocessing
}

/// Represents the configuration for an external news API.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsApiConfig {
    pub id: String,
    pub name: String,
    pub base_url: String,
    pub api_key_env_var: String, // Environment variable name for the API key
    pub categories: Vec<String>, // Supported categories by this API
    pub languages: Vec<String>,  // Supported languages by this API
    pub last_successful_fetch: Option<DateTime<Utc>>,
    pub fetch_interval_seconds: u64,
    pub is_active: bool,
}

/// The `NewsApiClient` service manages configured news APIs and fetches data.
pub struct NewsApiClient {
    api_configs: HashMap<String, NewsApiConfig>,
}

impl NewsApiClient {
    /// Creates a new `NewsApiClient` instance.
    pub fn new() -> Self {
        NewsApiClient {
            api_configs: HashMap::new(),
        }
    }

    /// Adds a new news API configuration.
    pub fn add_api_config(&mut self, mut config: NewsApiConfig) -> Result<String, String> {
        config.id = format!("news_api_cfg_{}", Uuid::new_v4());
        println!("[NewsApiClient] Adding API config: {}", config.name);
        let id = config.id.clone();
        self.api_configs.insert(id.clone(), config);
        Ok(id)
    }

    /// Retrieves an API configuration by its ID.
    pub fn get_api_config(&self, id: &str) -> Option<&NewsApiConfig> {
        self.api_configs.get(id)
    }

    /// Fetches news articles using a configured API. This is a simplified simulation.
    pub async fn fetch_articles(
        &self,
        config_id: &str,
        query: &str,
        language: Option<&str>,
        category: Option<&str>,
    ) -> Result<Vec<ApiFetchedArticle>, String> {
        let config = self.api_configs.get(config_id)
            .ok_or(format!("API config {} not found.", config_id))?;

        // In a real scenario, retrieve API key from environment variable using config.api_key_env_var
        // Then construct HTTP request to config.base_url with query parameters
        // And parse the JSON response into ApiFetchedArticle objects.

        println!("[NewsApiClient] Fetching articles from {} for query: {}", config.name, query);
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await; // Simulate API call delay

        // Simulate API response
        let article1 = ApiFetchedArticle {
            id: format!("api_art_{}", Uuid::new_v4()),
            title: format!("AI Breakthrough in {}", query),
            url: format!("{}/articles/ai-breakthrough", config.base_url),
            description: Some("Scientists announce a major leap in AI capabilities.".to_string()),
            content: Some("Full detailed content about the AI breakthrough...".to_string()),
            author: Some("Tech Insights Daily".to_string()),
            published_at: Some(Utc::now()),
            source_name: config.name.clone(),
            source_id: config.source_id.clone(),
            language: language.unwrap_or("en").to_string(),
            image_url: Some("http://example.com/ai_image.jpg".to_string()),
            api_config_id: config.id.clone(),
            raw_json: "{}".to_string(),
        };

        let article2 = ApiFetchedArticle {
            id: format!("api_art_{}", Uuid::new_v4()),
            title: format!("{} Market Trends", query),
            url: format!("{}/articles/market-trends", config.base_url),
            description: Some("Analysis of the latest market movements related to {}.".to_string()),
            content: None, // Sometimes API doesn't provide full content
            author: Some("Global Finance Daily".to_string()),
            published_at: Some(Utc::now()),
            source_name: config.name.clone(),
            source_id: config.source_id.clone(),
            language: language.unwrap_or("en").to_string(),
            image_url: None,
            api_config_id: config.id.clone(),
            raw_json: "{}".to_string(),
        };

        Ok(vec![article1, article2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    fn test_add_and_get_api_config() {
        let mut client = NewsApiClient::new();
        let config = NewsApiConfig {
            id: String::new(),
            name: "Mock News API".to_string(),
            base_url: "https://api.mocknews.com".to_string(),
            api_key_env_var: "MOCK_NEWS_API_KEY".to_string(),
            categories: vec!["tech".to_string(), "business".to_string()],
            languages: vec!["en".to_string(), "es".to_string()],
            last_successful_fetch: None,
            fetch_interval_seconds: 600,
            is_active: true,
        };

        let config_id = client.add_api_config(config.clone()).unwrap();
        assert!(!config_id.is_empty());

        let retrieved_config = client.get_api_config(&config_id).unwrap();
        assert_eq!(retrieved_config.name, "Mock News API");
    }

    #[tokio::test]
    async fn test_fetch_articles() {
        let mut client = NewsApiClient::new();
        let config = NewsApiConfig {
            id: String::new(),
            name: "Test News API".to_string(),
            base_url: "https://api.testnews.com".to_string(),
            api_key_env_var: "TEST_API_KEY".to_string(),
            categories: vec!["general".to_string()],
            languages: vec!["en".to_string()],
            last_successful_fetch: None,
            fetch_interval_seconds: 300,
            is_active: true,
        };

        let config_id = client.add_api_config(config.clone()).unwrap();

        let articles = client.fetch_articles(&config_id, "Space Exploration", Some("en"), None).await.unwrap();
        assert_eq!(articles.len(), 2);
        assert!(articles[0].title.contains("Space Exploration"));
        assert_eq!(articles[0].api_config_id, config_id);
    }
}
