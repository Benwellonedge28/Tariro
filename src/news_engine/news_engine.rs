// src/news_engine/news_engine.rs
use crate::prelude::*;

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
// New use statements for content curation modules
use crate::news_engine::content_curation::filter_engine::{FilterEngine, FilterRule};
use crate::news_engine::content_curation::ranking_algorithm::{RankingAlgorithm, RankingProfile, RankedArticle};
use crate::news_engine::content_curation::personalization_engine::{PersonalizationEngine, UserProfile, InteractionRecord};

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
    // New curation components
    filter_engine: Mutex<FilterEngine>,
    ranking_algorithm: Mutex<RankingAlgorithm>,
    personalization_engine: Mutex<PersonalizationEngine>,
}

impl NewsEngine {
    /// Creates a new `NewsEngine` instance.
    pub fn new(config: Arc<AGIConfig>, telemetry_logger: Arc<TelemetryLogger>, db_service: Arc<DatabaseService>) -> Self {
        let ranking_algo_instance = RankingAlgorithm::new(); // Create once here
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
            // Initialize new curation components
            filter_engine: Mutex::new(FilterEngine::new()),
            ranking_algorithm: Mutex::new(ranking_algo_instance.clone()), // Use clone for shared ownership
            personalization_engine: Mutex::new(PersonalizationEngine::new(ranking_algo_instance)), // Pass the original instance
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

            // Example: Add initial ranking profile
            self.ranking_algorithm.lock().unwrap().add_ranking_profile(RankingProfile {
                id: String::new(),
                name: "Default Ranking".to_string(),
                description: "Default ranking profile with balanced weights.".to_string(),
                weights: Default::default(),
                is_active: true,
            }).unwrap();

            // Example: Add initial user profile for personalization
            self.personalization_engine.lock().unwrap().create_or_update_user_profile(UserProfile {
                id: String::new(),
                user_id: "test_user_1".to_string(),
                preferences: Default::default(),
                interaction_history: Vec::new(),
                inferred_interests: vec!["AI".to_string(), "quantum computing".to_string()],
                created_at: Utc::now(),
                last_updated: Utc::now(),
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
        // Clones of self for processing ingested articles
        let news_engine_self_clone = Arc::new(self.clone());

        // RSS Ingestion Loop
        tokio::spawn(async move {
            let rss_parser = rss_parser_clone.lock().unwrap();
            for (id, source) in rss_parser.feed_sources.clone() {
                if source.is_active {
                    let interval = Duration::from_secs(source.fetch_interval_seconds);
                    let db_service_rss = db_service_clone.clone();
                    let telemetry_logger_rss = telemetry_logger_clone.clone();
                    let rss_parser_inner = Arc::new(rss_parser_clone.clone());
                    let news_engine_for_processing = news_engine_self_clone.clone();
                    tokio::spawn(async move {
                        loop {
                            info!("[NewsEngine] Running RSS ingestion for: {}", source.name);
                            match rss_parser_inner.lock().unwrap().fetch_and_parse_feed(&id).await {
                                Ok(articles) => {
                                    info!("Fetched {} articles from RSS feed: {}", articles.len(), source.name);
                                    for article in articles {
                                        match news_engine_for_processing.process_ingested_article_with_analysis(
                                            article.id.clone(),
                                            article.title.clone(),
                                            article.description.unwrap_or_else(|| article.title.clone()),
                                            article.link.clone(),
                                            article.feed_source_id.clone(),
                                        ).await {
                                            Ok(omega_article) => {
                                                info!("Processed Omega Article: {}", omega_article.title);
                                                // TODO: Store omega_article in DB
                                            },
                                            Err(e) => error!("Failed to process RSS article: {}", e),
                                        }
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
                    let news_engine_for_processing = news_engine_self_clone.clone();
                    tokio::spawn(async move {
                        loop {
                            info!("[NewsEngine] Running API ingestion for: {}", config.name);
                            match api_client_inner.lock().unwrap().fetch_articles(&id, "AI", Some("en"), Some("technology")).await {
                                Ok(articles) => {
                                    info!("Fetched {} articles from API: {}", articles.len(), config.name);
                                    for article in articles {
                                        match news_engine_for_processing.process_ingested_article_with_analysis(
                                            article.id.clone(),
                                            article.title.clone(),
                                            article.content.unwrap_or_else(|| article.title.clone()),
                                            article.url.clone(),
                                            article.source_name.clone(),
                                        ).await {
                                            Ok(omega_article) => {
                                                info!("Processed Omega Article: {}", omega_article.title);
                                                // TODO: Store omega_article in DB
                                            },
                                            Err(e) => error!("Failed to process API article: {}", e),
                                        }
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
                    let news_engine_for_processing = news_engine_self_clone.clone();
                    tokio::spawn(async move {
                        loop {
                            info!("[NewsEngine] Running Web Scraper for: {}", profile.name);
                            let example_url = "https://customtechblog.com/articles/ai-revolution-unfolds";
                            match web_scraper_inner.lock().unwrap().scrape_url(example_url, &id).await {
                                Ok(article) => {
                                    match news_engine_for_processing.process_ingested_article_with_analysis(
                                        article.id.clone(),
                                        article.title.unwrap_or_else(|| "Untitled Scraped Article".to_string()),
                                        article.main_content.clone(),
                                        article.url.clone(),
                                        article.source_domain.clone(),
                                    ).await {
                                        Ok(omega_article) => {
                                            info!("Processed Omega Article: {}", omega_article.title);
                                            // TODO: Store omega_article in DB
                                        },
                                        Err(e) => error!("Failed to process Scraped article: {}", e),
                                    }
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

    // Methods to expose curation functionalities

    /// Filters a list of OmegaNewsArticles using the configured FilterEngine.
    pub fn filter_articles(&self, articles: Vec<OmegaNewsArticle>, rules: &[FilterRule]) -> Vec<OmegaNewsArticle> {
        let mut filter_engine = self.filter_engine.lock().unwrap();
        for rule in rules {
            filter_engine.add_filter_rule(rule.clone()).unwrap(); // Temporarily add rules
        }
        let filtered = filter_engine.apply_filters(articles);
        // In a real scenario, would need to remove temporary rules or use a dedicated method for ad-hoc filtering
        filtered
    }

    /// Ranks a list of OmegaNewsArticles using the configured RankingAlgorithm.
    pub async fn rank_articles(
        &self,
        articles: Vec<OmegaNewsArticle>,
        profile_id: &str,
        context_keywords: Option<&[String]>,
        user_id: Option<&str>,
    ) -> Result<Vec<RankedArticle>, String> {
        self.ranking_algorithm.lock().unwrap().rank_articles(articles, profile_id, context_keywords, user_id).await
    }

    /// Personalizes a list of OmegaNewsArticles for a specific user.
    pub async fn personalize_articles(
        &self,
        articles: Vec<OmegaNewsArticle>,
        user_id: &str,
    ) -> Result<Vec<RankedArticle>, String> {
        self.personalization_engine.lock().unwrap().personalize_articles(articles, user_id).await
    }
}

impl Clone for NewsEngine {
    fn clone(&self) -> Self {
        NewsEngine {
            config: self.config.clone(),
            telemetry_logger: self.telemetry_logger.clone(),
            db_service: self.db_service.clone(),
            rss_parser: Mutex::new(self.rss_parser.lock().unwrap().clone()),
            api_client: Mutex::new(self.api_client.lock().unwrap().clone()),
            web_scraper: Mutex::new(self.web_scraper.lock().unwrap().clone()),
            sentiment_analyzer: Mutex::new(self.sentiment_analyzer.lock().unwrap().clone()),
            entity_extractor: Mutex::new(self.entity_extractor.lock().unwrap().clone()),
            fact_checker: Mutex::new(self.fact_checker.lock().unwrap().clone()),
            topic_modeler: Mutex::new(self.topic_modeler.lock().unwrap().clone()),
            filter_engine: Mutex::new(self.filter_engine.lock().unwrap().clone()),
            ranking_algorithm: Mutex::new(self.ranking_algorithm.lock().unwrap().clone()),
            personalization_engine: Mutex::new(self.personalization_engine.lock().unwrap().clone()),
        }
    }
}
