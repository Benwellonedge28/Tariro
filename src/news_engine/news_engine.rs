// src/news_engine/news_engine.rs

//! The core orchestration module for OMEGA News. It manages the entire lifecycle
//! of news processing, from ingestion and analysis to content generation and dissemination.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::time::{self, Duration};
use uuid::Uuid;

use crate::config::AGIConfig;
use crate::telemetry::TelemetryLogger;
use crate::database::DatabaseService;
use crate::news_engine::news_ingestion::rss_parser::{RssParser, FeedSource, ParsedFeedArticle};
use crate::news_engine::news_ingestion::api_client::{NewsApiClient, NewsApiConfig, ApiFetchedArticle};
use crate::news_engine::news_ingestion::scraper::{WebScraper, WebScrapeProfile, ScrapedArticle};
// New use statements for content analysis modules
use crate::news_engine::content_analysis::sentiment_analyzer::{SentimentAnalyzer, SentimentAnalysisResult};
use crate::news_engine::content_analysis::entity_extractor::{EntityExtractor, ExtractedEntity};
use crate::news_engine::content_analysis::fact_checker::{FactChecker, FactCheckResult, Claim};
use crate::news_engine::content_analysis::topic_modeler::{TopicModeler, IdentifiedTopic};

/// Represents a processed and standardized news article within the OMEGA News system.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OmegaNewsArticle {
    pub id: String,
    pub title: String,
    pub content: String,
    pub url: String,
    pub author: Option<String>,
    pub published_date: DateTime<Utc>,
    pub ingested_date: DateTime<Utc>,
    pub source_name: String,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
    pub sentiment_score: f32, // -1.0 (negative) to 1.0 (positive)
    pub factual_accuracy_score: f32, // 0.0 to 1.0
    pub data_lineage: Vec<String>, // IDs of raw ingested data
    // New fields for analysis results
    pub sentiment_analysis: Option<SentimentAnalysisResult>,
    pub extracted_entities: Vec<ExtractedEntity>,
    pub fact_check_results: Vec<FactCheckResult>,
    pub identified_topics: Vec<IdentifiedTopic>,
}

/// The `NewsEngine` is the central component for OMEGA News operations.
pub struct NewsEngine {
    config: Arc<AGIConfig>,
    telemetry_logger: Arc<TelemetryLogger>,
    db_service: Arc<DatabaseService>,
    rss_parser: Mutex<RssParser>,
    api_client: Mutex<NewsApiClient>,
    web_scraper: Mutex<WebScraper>,
    // New analysis components
    sentiment_analyzer: Mutex<SentimentAnalyzer>,
    entity_extractor: Mutex<EntityExtractor>,
    fact_checker: Mutex<FactChecker>,
    topic_modeler: Mutex<TopicModeler>,
    // Other components like content analysis, generation, dissemination will be added here
}

impl NewsEngine {
    /// Creates a new `NewsEngine` instance.
    pub fn new(config: Arc<AGIConfig>, telemetry_logger: Arc<TelemetryLogger>, db_service: Arc<DatabaseService>) -> Self {
        NewsEngine {
            config,
            telemetry_logger,
            db_service,
            rss_parser: Mutex::new(RssParser::new()),
            api_client: Mutex::new(NewsApiClient::new()),
            web_scraper: Mutex::new(WebScraper::new()),
            // Initialize new analysis components
            sentiment_analyzer: Mutex::new(SentimentAnalyzer::new()),
            entity_extractor: Mutex::new(EntityExtractor::new()),
            fact_checker: Mutex::new(FactChecker::new()),
            topic_modeler: Mutex::new(TopicModeler::new()),
        }
    }

    /// Initializes the news engine, loading configurations and starting ingestion loops.
    pub async fn initialize(&self) -> Result<(), String> {
        info!("[NewsEngine] Initializing...");
        
        // Example: Add initial feed sources and API configs
        { // Lock scopes
            let mut rss_parser = self.rss_parser.lock().unwrap();
            rss_parser.add_feed_source(FeedSource {
                id: String::new(),
                name: "TechCrunch".to_string(),
                url: "http://feeds.feedburner.com/TechCrunch/".to_string(),
                last_fetched: None,
                fetch_interval_seconds: 3600,
                is_active: true,
                tags: vec!["tech".to_string(), "startups".to_string()],
            }).unwrap();

            let mut api_client = self.api_client.lock().unwrap();
            api_client.add_api_config(NewsApiConfig {
                id: String::new(),
                name: "NewsAPI.org".to_string(),
                base_url: "https://newsapi.org/v2".to_string(),
                api_key_env_var: "NEWSAPI_API_KEY".to_string(),
                categories: vec!["technology".to_string(), "business".to_string(), "science".to_string()],
                languages: vec!["en".to_string(), "de".to_string()],
                last_successful_fetch: None,
                fetch_interval_seconds: 900,
                is_active: true,
            }).unwrap();

            let mut web_scraper = self.web_scraper.lock().unwrap();
            web_scraper.add_scrape_profile(WebScrapeProfile {
                id: String::new(),
                name: "CustomTechBlogScraper".to_string(),
                target_url_patterns: vec!["https://customtechblog.com/articles/.*".to_string()],
                title_selector: "h1.entry-title".to_string(),
                author_selector: Some(".author-name".to_string()),
                published_date_selector: Some("time.entry-date".to_string()),
                content_selector: "div.entry-content".to_string(),
                image_selector: Some("img.wp-post-image".to_string()),
                is_active: true,
                last_used: None,
                fetch_interval_seconds: 7200,
            }).unwrap();
        }

        // Start ingestion loops in separate tokio tasks
        self.start_ingestion_loops().await;

        Ok(())
    }

    /// Starts asynchronous loops for fetching data from RSS feeds, APIs, and web scraping.
    async fn start_ingestion_loops(&self) {
        let rss_parser_clone = Arc::new(self.rss_parser.clone());
        let api_client_clone = Arc::new(self.api_client.clone());
        let web_scraper_clone = Arc::new(self.web_scraper.clone());
        let telemetry_logger_clone = self.telemetry_logger.clone();
        let db_service_clone = self.db_service.clone();

        // RSS Ingestion Loop
        tokio::spawn(async move {
            let rss_parser = rss_parser_clone.lock().unwrap();
            for (id, source) in rss_parser.feed_sources.clone() {
                if source.is_active {
                    let interval = Duration::from_secs(source.fetch_interval_seconds);
                    let db_service_rss = db_service_clone.clone();
                    let telemetry_logger_rss = telemetry_logger_clone.clone();
                    let rss_parser_inner = Arc::new(rss_parser_clone.clone());
                    tokio::spawn(async move {
                        loop {
                            info!("[NewsEngine] Running RSS ingestion for: {}", source.name);
                            match rss_parser_inner.lock().unwrap().fetch_and_parse_feed(&id).await {
                                Ok(articles) => {
                                    info!("Fetched {} articles from RSS feed: {}", articles.len(), source.name);
                                    for article in articles {
                                        // Process and store articles (e.g., in db_service_rss)
                                        info!("Parsed RSS Article: {}", article.title);
                                        // Self-call to process_ingested_article_with_analysis
                                        // This would ideally be an event on a channel or queue
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to fetch RSS feed {}: {}", source.name, e);
                                }
                            }
                            time::sleep(interval).await;
                        }
                    });
                }
            }
        });

        // API Ingestion Loop
        tokio::spawn(async move {
            let api_client = api_client_clone.lock().unwrap();
            for (id, config) in api_client.api_configs.clone() {
                if config.is_active {
                    let interval = Duration::from_secs(config.fetch_interval_seconds);
                    let db_service_api = db_service_clone.clone();
                    let telemetry_logger_api = telemetry_logger_clone.clone();
                    let api_client_inner = Arc::new(api_client_clone.clone());
                    tokio::spawn(async move {
                        loop {
                            info!("[NewsEngine] Running API ingestion for: {}", config.name);
                            match api_client_inner.lock().unwrap().fetch_articles(&id, "AI", Some("en"), Some("technology")).await {
                                Ok(articles) => {
                                    info!("Fetched {} articles from API: {}", articles.len(), config.name);
                                    for article in articles {
                                        info!("Fetched API Article: {}", article.title);
                                        // Self-call to process_ingested_article_with_analysis
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to fetch API articles from {}: {}", config.name, e);
                                }
                            }
                            time::sleep(interval).await;
                        }
                    });
                }
            }
        });

        // Web Scraper Loop
        tokio::spawn(async move {
            let web_scraper = web_scraper_clone.lock().unwrap();
            for (id, profile) in web_scraper.scrape_profiles.clone() {
                if profile.is_active {
                    let interval = Duration::from_secs(profile.fetch_interval_seconds);
                    let db_service_scraper = db_service_clone.clone();
                    let telemetry_logger_scraper = telemetry_logger_clone.clone();
                    let web_scraper_inner = Arc::new(web_scraper_clone.clone());
                    tokio::spawn(async move {
                        loop {
                            info!("[NewsEngine] Running Web Scraper for: {}", profile.name);
                            let example_url = "https://customtechblog.com/articles/ai-revolution-unfolds";
                            match web_scraper_inner.lock().unwrap().scrape_url(example_url, &id).await {
                                Ok(article) => {
                                    info!("Scraped Web Article: {}", article.title.unwrap_or_default());
                                    // Self-call to process_ingested_article_with_analysis
                                },
                                Err(e) => {
                                    error!("Failed to scrape URL {} with profile {}: {}", example_url, profile.name, e);
                                }
                            }
                            time::sleep(interval).await;
                        }
                    });
                }
            }
        });
    }

    /// Processes a raw ingested article into a standardized `OmegaNewsArticle`,
    /// applying all content analysis modules.
    pub async fn process_ingested_article_with_analysis(&self, raw_content_id: String, title: String, content: String, url: String, source_name: String) -> Result<OmegaNewsArticle, String> {
        info!("[NewsEngine] Processing raw content ID: {} with analysis...", raw_content_id);
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Perform Sentiment Analysis
        let sentiment_result = self.sentiment_analyzer.lock().unwrap().analyze_sentiment(&content).await?;
        info!("Sentiment for '{}': Score={}, Magnitude={}", title, sentiment_result.score, sentiment_result.magnitude);

        // Perform Entity Extraction
        let extracted_entities = self.entity_extractor.lock().unwrap().extract_entities(&content).await?;
        info!("Extracted {} entities from '{}'.", extracted_entities.len(), title);

        // Perform Fact-Checking (identify claims and then check them)
        let mut fact_check_results = Vec::new();
        let claims = self.fact_checker.lock().unwrap().identify_claims(&content).await?;
        for claim in claims {
            let check_result = self.fact_checker.lock().unwrap().check_claim(&claim).await?;
            fact_check_results.push(check_result);
        }
        info!("Fact-checked {} claims for '{}'.", fact_check_results.len(), title);

        // Perform Topic Modeling
        let identified_topics = self.topic_modeler.lock().unwrap().identify_topics(&content).await?;
        let categories = self.topic_modeler.lock().unwrap().categorize_document(&identified_topics, &vec!["Technology".to_string(), "Business".to_string(), "Politics".to_string(), "Science".to_string(), "Environment".to_string(), "Health".to_string(), "General".to_string()]);
        info!("Identified {} topics and categories for '{}': {:?}", identified_topics.len(), title, categories);

        Ok(OmegaNewsArticle {
            id: format!("omega_art_{}", Uuid::new_v4()),
            title,
            content,
            url,
            author: Some("OMEGA News AGI".to_string()), // Placeholder
            published_date: Utc::now(), // Placeholder
            ingested_date: Utc::now(),
            source_name,
            categories,
            keywords: sentiment_result.keywords.clone(), // Using sentiment keywords as general keywords
            sentiment_score: sentiment_result.score,
            factual_accuracy_score: fact_check_results.first().map_or(0.0, |r| r.score), // Simplified: just first claim's score
            data_lineage: vec![raw_content_id],
            sentiment_analysis: Some(sentiment_result),
            extracted_entities,
            fact_check_results,
            identified_topics,
        })
    }

    // Other NewsEngine functionalities like content analysis, generation, dissemination
}
