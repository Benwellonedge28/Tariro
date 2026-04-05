// src/news_engine/content_curation/ranking_algorithm.rs

//! Provides functionality for ranking news articles based on various factors
//! such as relevance, recency, quality, sentiment, and user engagement.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::news_engine::news_engine::OmegaNewsArticle;

/// Represents a ranked article with its computed score.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankedArticle {
    pub article: OmegaNewsArticle,
    pub ranking_score: f32,
    pub ranking_factors: HashMap<String, f32>, // Breakdown of scores from different factors
}

/// Represents a configuration for how articles should be ranked.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankingProfile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weights: RankingWeights,
    pub is_active: bool,
}

/// Defines the weights for different ranking factors.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankingWeights {
    pub recency: f32, // Weight for how recently the article was published
    pub relevance: f32, // Weight for how relevant to a query/profile
    pub sentiment: f32, // Weight for the sentiment score
    pub factual_accuracy: f32, // Weight for factual accuracy score
    pub entity_prominence: f32, // Weight for how prominent key entities are
    pub source_reputation: f32, // Weight for the reputation of the news source
    pub personalization: f32, // Weight for user-specific personalization
}

impl Default for RankingWeights {
    fn default() -> Self {
        RankingWeights {
            recency: 0.2,
            relevance: 0.3,
            sentiment: 0.1,
            factual_accuracy: 0.2,
            entity_prominence: 0.1,
            source_reputation: 0.05,
            personalization: 0.05,
        }
    }
}

/// The `RankingAlgorithm` service ranks a list of `OmegaNewsArticle`s.
pub struct RankingAlgorithm {
    ranking_profiles: HashMap<String, RankingProfile>,
}

impl RankingAlgorithm {
    /// Creates a new `RankingAlgorithm` instance.
    pub fn new() -> Self {
        RankingAlgorithm {
            ranking_profiles: HashMap::new(),
        }
    }

    /// Adds a new ranking profile.
    pub fn add_ranking_profile(&mut self, mut profile: RankingProfile) -> Result<String, String> {
        profile.id = format!("rank_prof_{}", Uuid::new_v4());
        println!("[RankingAlgorithm] Adding ranking profile: {}", profile.name);
        let id = profile.id.clone();
        self.ranking_profiles.insert(id.clone(), profile);
        Ok(id)
    }

    /// Retrieves a ranking profile by its ID.
    pub fn get_ranking_profile(&self, id: &str) -> Option<&RankingProfile> {
        self.ranking_profiles.get(id)
    }

    /// Ranks a list of articles based on a specified ranking profile.
    /// Returns a vector of `RankedArticle`s, sorted by `ranking_score` in descending order.
    pub async fn rank_articles(
        &self,
        articles: Vec<OmegaNewsArticle>,
        profile_id: &str,
        context_keywords: Option<&[String]>,
        user_id: Option<&str>,
    ) -> Result<Vec<RankedArticle>, String> {
        let profile = self.ranking_profiles.get(profile_id)
            .ok_or(format!("Ranking profile {} not found.", profile_id))?;

        if !profile.is_active { return Err(format!("Ranking profile {} is not active.", profile_id)); }

        println!("[RankingAlgorithm] Ranking {} articles using profile: {}", articles.len(), profile.name);

        let mut ranked_articles: Vec<RankedArticle> = Vec::new();

        for article in articles {
            let mut ranking_factors = HashMap::new();
            let mut total_score = 0.0;

            // Recency Score (higher for more recent)
            let recency_duration = Utc::now().signed_duration_since(article.published_date).num_hours() as f32;
            // Example: Score declines linearly over 72 hours, then flattens
            let recency_score = (1.0 - (recency_duration / 72.0).min(1.0)).max(0.0);
            ranking_factors.insert("recency".to_string(), recency_score);
            total_score += recency_score * profile.weights.recency;

            // Relevance Score (simplified: keyword overlap with context)
            let mut relevance_score = 0.0;
            if let Some(keywords) = context_keywords {
                let matched_keywords = article.keywords.iter()
                    .filter(|ak| keywords.contains(ak)).count() as f32;
                relevance_score = matched_keywords / (keywords.len() as f32 + 1e-6);
            }
            ranking_factors.insert("relevance".to_string(), relevance_score);
            total_score += relevance_score * profile.weights.relevance;

            // Sentiment Score (directly from article analysis)
            let sentiment_score_normalized = (article.sentiment_score + 1.0) / 2.0; // Normalize from -1..1 to 0..1
            ranking_factors.insert("sentiment".to_string(), sentiment_score_normalized);
            total_score += sentiment_score_normalized * profile.weights.sentiment;

            // Factual Accuracy Score (directly from article analysis)
            ranking_factors.insert("factual_accuracy".to_string(), article.factual_accuracy_score);
            total_score += article.factual_accuracy_score * profile.weights.factual_accuracy;

            // Entity Prominence (simplified: count of extracted entities)
            let entity_prominence_score = (article.extracted_entities.len() as f32 / 10.0).min(1.0);
            ranking_factors.insert("entity_prominence".to_string(), entity_prominence_score);
            total_score += entity_prominence_score * profile.weights.entity_prominence;

            // Source Reputation (simplified: always 0.5 for now)
            let source_reputation_score = 0.5;
            ranking_factors.insert("source_reputation".to_string(), source_reputation_score);
            total_score += source_reputation_score * profile.weights.source_reputation;

            // Personalization (simplified: always 0.5 for now, would depend on user_id and preferences)
            let personalization_score = if user_id.is_some() { 0.5 } else { 0.0 };
            ranking_factors.insert("personalization".to_string(), personalization_score);
            total_score += personalization_score * profile.weights.personalization;

            ranked_articles.push(RankedArticle {
                article,
                ranking_score: total_score,
                ranking_factors,
            });
        }

        // Sort articles by ranking score in descending order
        ranked_articles.sort_by(|a, b| b.ranking_score.partial_cmp(&a.ranking_score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(ranked_articles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::news_engine::news_engine::OmegaNewsArticle;
    use crate::news_engine::content_analysis::fact_checker::FactCheckResult;
    use crate::news_engine::content_analysis::fact_checker::FactCheckVerdict;
    use crate::news_engine::content_analysis::entity_extractor::ExtractedEntity;
    use crate::news_engine::content_analysis::entity_extractor::EntityType;

    fn create_sample_article(title: &str, content: &str, published_hours_ago: i64, sentiment: f32, accuracy: f32, keywords: Vec<String>, entity_count: usize) -> OmegaNewsArticle {
        let published_date = Utc::now() - chrono::Duration::hours(published_hours_ago);
        let mut entities = Vec::new();
        for _ in 0..entity_count {
            entities.push(ExtractedEntity { id: Uuid::new_v4().to_string(), text: "entity".to_string(), entity_type: EntityType::Other("test".to_string()), confidence: 1.0, start_offset: 0, end_offset: 0, disambiguation_id: None });
        }

        OmegaNewsArticle {
            id: format!("art_{}", Uuid::new_v4()),
            title: title.to_string(),
            content: content.to_string(),
            url: format!("http://example.com/{}", Uuid::new_v4()),
            author: Some("Test Author".to_string()),
            published_date,
            ingested_date: Utc::now(),
            source_name: "Test Source".to_string(),
            categories: vec!["Tech".to_string()],
            keywords,
            sentiment_score: sentiment,
            factual_accuracy_score: accuracy,
            data_lineage: vec![],
            sentiment_analysis: None,
            extracted_entities: entities,
            fact_check_results: vec![FactCheckResult { id: Uuid::new_v4().to_string(), claim_id: Uuid::new_v4().to_string(), score: accuracy, verdict: FactCheckVerdict::True, evidence_urls: vec![], supporting_statements: vec![], refuting_statements: vec![], checks_performed: vec![], checked_at: Utc::now(), checker_agi_id: "test".to_string() }],
            identified_topics: vec![],
        }
    }

    #[tokio::test]
    async fn test_rank_articles_recency() {
        let mut ranking_algo = RankingAlgorithm::new();
        let profile_id = ranking_algo.add_ranking_profile(RankingProfile {
            id: String::new(),
            name: "Recency Focused".to_string(),
            description: "Ranks articles primarily by recency.".to_string(),
            weights: RankingWeights { recency: 1.0, relevance: 0.0, sentiment: 0.0, factual_accuracy: 0.0, entity_prominence: 0.0, source_reputation: 0.0, personalization: 0.0 },
            is_active: true,
        }).unwrap();

        let article1 = create_sample_article("Old Article", "", 24, 0.5, 0.8, vec![], 1);
        let article2 = create_sample_article("New Article", "", 1, 0.5, 0.8, vec![], 1);

        let ranked = ranking_algo.rank_articles(vec![article1, article2], &profile_id, None, None).await.unwrap();
        assert_eq!(ranked[0].article.title, "New Article");
    }

    #[tokio::test]
    async fn test_rank_articles_relevance() {
        let mut ranking_algo = RankingAlgorithm::new();
        let profile_id = ranking_algo.add_ranking_profile(RankingProfile {
            id: String::new(),
            name: "Relevance Focused".to_string(),
            description: "Ranks articles primarily by relevance.".to_string(),
            weights: RankingWeights { recency: 0.0, relevance: 1.0, sentiment: 0.0, factual_accuracy: 0.0, entity_prominence: 0.0, source_reputation: 0.0, personalization: 0.0 },
            is_active: true,
        }).unwrap();

        let article1 = create_sample_article("About AI", "", 1, 0.5, 0.8, vec!["AI".to_string(), "ML".to_string()], 1);
        let article2 = create_sample_article("About Space", "", 1, 0.5, 0.8, vec!["Space".to_string()], 1);

        let ranked = ranking_algo.rank_articles(vec![article1, article2], &profile_id, Some(&vec!["AI".to_string(), "robotics".to_string()]), None).await.unwrap();
        assert_eq!(ranked[0].article.title, "About AI");
    }
}
