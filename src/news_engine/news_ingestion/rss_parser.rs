// src/news_engine/news_ingestion/rss_parser.rs

//! Handles parsing of RSS/Atom feeds to extract news articles and their metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a parsed news article from an RSS/Atom feed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedFeedArticle {
    pub id: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub content: Option<String>, // Full content if available in feed, or from scraping
    pub author: Option<String>,
    pub published_date: Option<DateTime<Utc>>,
    pub categories: Vec<String>,
    pub image_url: Option<String>,
    pub feed_source_id: String, // ID of the source feed
    pub raw_xml: String, // Store raw XML for debugging/reprocessing
}

/// Represents a single RSS/Atom feed source.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeedSource {
    pub id: String,
    pub name: String,
    pub url: String,
    pub last_fetched: Option<DateTime<Utc>>,
    pub fetch_interval_seconds: u64,
    pub is_active: bool,
    pub tags: Vec<String>,
}

/// The `RssParser` service is responsible for fetching and parsing RSS/Atom feeds.
pub struct RssParser {
    feed_sources: HashMap<String, FeedSource>,
}

impl RssParser {
    /// Creates a new `RssParser` instance.
    pub fn new() -> Self {
        RssParser {
            feed_sources: HashMap::new(),
        }
    }

    /// Adds a new feed source to be monitored.
    pub fn add_feed_source(&mut self, mut source: FeedSource) -> Result<String, String> {
        source.id = format!("feed_src_{}", Uuid::new_v4());
        println!("[RssParser] Adding feed source: {}", source.name);
        let id = source.id.clone();
        self.feed_sources.insert(id.clone(), source);
        Ok(id)
    }

    /// Retrieves a feed source by its ID.
    pub fn get_feed_source(&self, id: &str) -> Option<&FeedSource> {
        self.feed_sources.get(id)
    }

    /// Fetches and parses articles from a given RSS/Atom feed URL.
    /// This is a simplified simulation.
    pub async fn fetch_and_parse_feed(&self, source_id: &str) -> Result<Vec<ParsedFeedArticle>, String> {
        let source = self.feed_sources.get(source_id)
            .ok_or(format!("Feed source {} not found.", source_id))?;
        
        println!("[RssParser] Fetching and parsing feed: {} ({})", source.name, source.url);
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await; // Simulate network delay

        // Simulate a feed with a few articles
        let article1 = ParsedFeedArticle {
            id: format!("art_{}", Uuid::new_v4()),
            title: format!("Latest News from {}", source.name),
            link: format!("{}/article/1", source.url),
            description: Some("A groundbreaking report on recent events.".to_string()),
            content: None,
            author: Some("Tariro AGI Observer".to_string()),
            published_date: Some(Utc::now()),
            categories: vec!["Technology".to_string(), "AI".to_string()],
            image_url: Some("http://example.com/image1.jpg".to_string()),
            feed_source_id: source.id.clone(),
            raw_xml: "<item>...</item>".to_string(),
        };

        let article2 = ParsedFeedArticle {
            id: format!("art_{}", Uuid::new_v4()),
            title: format!("Deep Dive: The Future of {}", source.name),
            link: format!("{}/article/2", source.url),
            description: Some("An analytical piece exploring future trends.".to_string()),
            content: None,
            author: Some("Tariro AGI Analyst".to_string()),
            published_date: Some(Utc::now()),
            categories: vec!["Future".to_string(), "Research".to_string()],
            image_url: None,
            feed_source_id: source.id.clone(),
            raw_xml: "<item>...</item>".to_string(),
        };

        Ok(vec![article1, article2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_and_parse_feed_source() {
        let mut rss_parser = RssParser::new();
        let source = FeedSource {
            id: String::new(), // Will be generated
            name: "Example News".to_string(),
            url: "http://example.com/rss".to_string(),
            last_fetched: None,
            fetch_interval_seconds: 3600,
            is_active: true,
            tags: vec!["tech".to_string(), "global".to_string()],
        };

        let source_id = rss_parser.add_feed_source(source.clone()).unwrap();
        assert!(!source_id.is_empty());

        let retrieved_source = rss_parser.get_feed_source(&source_id).unwrap();
        assert_eq!(retrieved_source.name, "Example News");

        let articles = rss_parser.fetch_and_parse_feed(&source_id).await.unwrap();
        assert_eq!(articles.len(), 2);
        assert!(articles[0].title.contains("Example News"));
        assert_eq!(articles[0].feed_source_id, source_id);
    }

    #[tokio::test]
    #[should_panic(expected = "Feed source non_existent_id not found.")]
    async fn test_parse_non_existent_feed() {
        let rss_parser = RssParser::new();
        rss_parser.fetch_and_parse_feed("non_existent_id").await.unwrap();
    }
}
