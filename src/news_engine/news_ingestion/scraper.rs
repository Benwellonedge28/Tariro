// src/news_engine/news_ingestion/scraper.rs
use crate::prelude::*;

//! Implements web scraping functionalities to extract news content from web pages
//! when RSS feeds or dedicated APIs are not available or sufficient.

use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a scraped news article with its extracted content.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScrapedArticle {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub published_date: Option<DateTime<Utc>>,
    pub main_content: String, // The extracted body of the article
    pub scraped_at: DateTime<Utc>,
    pub source_domain: String,
    pub selector_profile_id: String, // ID of the selector profile used
    pub raw_html_hash: String, // Hash of the raw HTML for change detection
}

/// Defines how to extract information from a specific website structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebScrapeProfile {
    pub id: String,
    pub name: String,
    pub target_url_patterns: Vec<String>, // Regex patterns for URLs to scrape
    pub title_selector: String, // CSS selector for the article title
    pub author_selector: Option<String>, // CSS selector for the author
    pub published_date_selector: Option<String>, // CSS selector for the published date
    pub content_selector: String, // CSS selector for the main article content
    pub image_selector: Option<String>, // CSS selector for the main image
    pub is_active: bool,
    pub last_used: Option<DateTime<Utc>>,
    pub fetch_interval_seconds: u64,
}

/// The `WebScraper` service handles fetching and parsing web pages based on profiles.
pub struct WebScraper {
    scrape_profiles: HashMap<String, WebScrapeProfile>,
}

impl WebScraper {
    /// Creates a new `WebScraper` instance.
    pub fn new() -> Self {
        WebScraper {
            scrape_profiles: HashMap::new(),
        }
    }

    /// Adds a new web scraping profile.
    pub fn add_scrape_profile(&mut self, mut profile: WebScrapeProfile) -> Result<String, String> {
        profile.id = format!("scrape_prof_{}", Uuid::new_v4());
        println!("[WebScraper] Adding scrape profile: {}", profile.name);
        let id = profile.id.clone();
        self.scrape_profiles.insert(id.clone(), profile);
        Ok(id)
    }

    /// Retrieves a web scrape profile by its ID.
    pub fn get_scrape_profile(&self, id: &str) -> Option<&WebScrapeProfile> {
        self.scrape_profiles.get(id)
    }

    /// Scrapes a given URL using a specified profile. This is a simplified simulation.
    pub async fn scrape_url(
        &self,
        url: &str,
        profile_id: &str,
    ) -> Result<ScrapedArticle, String> {
        let profile = self.scrape_profiles.get(profile_id)
            .ok_or(format!("Scrape profile {} not found.", profile_id))?;

        println!("[WebScraper] Scraping URL: {} using profile: {}", url, profile.name);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Simulate network fetch and parsing delay

        // Simulate content extraction based on selectors
        let title = Some(format!("Scraped Title for {}", url));
        let author = Some("ScraperBot".to_string());
        let published_date = Some(Utc::now());
        let main_content = format!("This is the extracted content from the webpage: {}. It was processed using selectors like '{}'.", url, profile.content_selector);
        let source_domain = url.split('/').nth(2).unwrap_or("unknown").to_string();
        let raw_html_hash = format!("{:x}", md5::compute(format!("<html><body>{}</body></html>", main_content)));

        Ok(ScrapedArticle {
            id: format!("scr_art_{}", Uuid::new_v4()),
            url: url.to_string(),
            title,
            author,
            published_date,
            main_content,
            scraped_at: Utc::now(),
            source_domain,
            selector_profile_id: profile.id.clone(),
            raw_html_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use md5;

    #[test]
    fn test_add_and_get_scrape_profile() {
        let mut scraper = WebScraper::new();
        let profile = WebScrapeProfile {
            id: String::new(),
            name: "Example News Site Scraper".to_string(),
            target_url_patterns: vec!["https://example.com/news/.*".to_string()],
            title_selector: ".article-title".to_string(),
            author_selector: Some(".article-author".to_string()),
            published_date_selector: Some(".article-date".to_string()),
            content_selector: ".article-body".to_string(),
            image_selector: Some(".article-image img".to_string()),
            is_active: true,
            last_used: None,
            fetch_interval_seconds: 1800,
        };

        let profile_id = scraper.add_scrape_profile(profile.clone()).unwrap();
        assert!(!profile_id.is_empty());

        let retrieved_profile = scraper.get_scrape_profile(&profile_id).unwrap();
        assert_eq!(retrieved_profile.name, "Example News Site Scraper");
    }

    #[tokio::test]
    async fn test_scrape_url() {
        let mut scraper = WebScraper::new();
        let profile = WebScrapeProfile {
            id: String::new(),
            name: "Test Site Scraper".to_string(),
            target_url_patterns: vec!["https://test.com/article/.*".to_string()],
            title_selector: "h1".to_string(),
            content_selector: "div.content".to_string(),
            author_selector: None,
            published_date_selector: None,
            image_selector: None,
            is_active: true,
            last_used: None,
            fetch_interval_seconds: 3600,
        };

        let profile_id = scraper.add_scrape_profile(profile.clone()).unwrap();

        let test_url = "https://test.com/article/123-example-story";
        let article = scraper.scrape_url(test_url, &profile_id).await.unwrap();

        assert!(!article.id.is_empty());
        assert_eq!(article.url, test_url);
        assert!(article.title.unwrap().contains("123-example-story"));
        assert!(article.main_content.contains("div.content"));
        assert_eq!(article.selector_profile_id, profile_id);
    }
}
