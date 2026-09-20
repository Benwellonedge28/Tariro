// src/news_engine/content_curation/personalization_engine.rs

//! Provides functionality for personalizing news content for individual users
//! based on their preferences, past interactions, and inferred interests.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::news_engine::news_engine::OmegaNewsArticle;
use crate::news_engine::content_curation::ranking_algorithm::{RankingAlgorithm, RankingProfile, RankingWeights, RankedArticle};

/// Represents a user's personalization profile.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: String,
    pub user_id: String,
    pub preferences: UserPreferences,
    pub interaction_history: Vec<InteractionRecord>,
    pub inferred_interests: Vec<String>, // Topics or keywords inferred from behavior
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

/// Stores explicit user preferences.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPreferences {
    pub preferred_categories: Vec<String>,
    pub disliked_categories: Vec<String>,
    pub preferred_sources: Vec<String>,
    pub disliked_sources: Vec<String>,
    pub preferred_authors: Vec<String>,
    pub min_sentiment_score: f32, // User prefers articles above this sentiment score
    pub min_factual_accuracy: f32, // User prefers articles above this factual accuracy
    pub language: String,
    pub reading_level: ReadingLevel,
}

impl Default for UserPreferences {
    fn default() -> Self {
        UserPreferences {
            preferred_categories: Vec::new(),
            disliked_categories: Vec::new(),
            preferred_sources: Vec::new(),
            disliked_sources: Vec::new(),
            preferred_authors: Vec::new(),
            min_sentiment_score: -0.2, // Default to tolerate slightly negative news
            min_factual_accuracy: 0.5, // Default to tolerate some inaccuracy
            language: "en".to_string(),
            reading_level: ReadingLevel::Moderate,
        }
    }
}

/// Defines the user's preferred reading complexity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReadingLevel {
    Elementary,
    JuniorHigh,
    HighSchool,
    College,
    Graduate,
    Professional,
    Academic,
    Any,
}

/// Record of a user's interaction with an article.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionRecord {
    pub article_id: String,
    pub interaction_type: InteractionType,
    pub timestamp: DateTime<Utc>,
    pub sentiment_at_interaction: Option<f32>, // What was the article's sentiment when interacted with
}

/// Types of user interactions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InteractionType {
    View,
    Click,
    Like,
    Dislike,
    Share,
    Bookmark,
    Hide,
    Comment,
}

/// The `PersonalizationEngine` service tailors news feeds for individual users.
pub struct PersonalizationEngine {
    user_profiles: HashMap<String, UserProfile>,
    ranking_algorithm: RankingAlgorithm, // Personalization leverages ranking
}

impl PersonalizationEngine {
    /// Creates a new `PersonalizationEngine` instance.
    pub fn new(ranking_algorithm: RankingAlgorithm) -> Self {
        PersonalizationEngine {
            user_profiles: HashMap::new(),
            ranking_algorithm,
        }
    }

    /// Creates or updates a user's personalization profile.
    pub fn create_or_update_user_profile(&mut self, mut profile: UserProfile) -> Result<String, String> {
        profile.last_updated = chrono::Utc::now();
        if profile.created_at.to_string().is_empty() {
            profile.created_at = chrono::Utc::now();
        }
        println!("[PersonalizationEngine] Creating/Updating user profile for: {}", profile.user_id);
        let id = profile.id.clone();
        self.user_profiles.insert(id.clone(), profile);
        Ok(id)
    }

    /// Retrieves a user's personalization profile.
    pub fn get_user_profile(&self, user_id: &str) -> Option<&UserProfile> {
        self.user_profiles.values().find(|p| p.user_id == user_id)
    }

    /// Records a user interaction and updates the profile.
    pub fn record_interaction(&mut self, user_id: &str, interaction: InteractionRecord) -> Result<(), String> {
        if let Some(profile) = self.user_profiles.values_mut().find(|p| p.user_id == user_id) {
            profile.interaction_history.push(interaction);
            profile.last_updated = chrono::Utc::now();
            // In a real system, this would also trigger inferred interest updates
            println!("[PersonalizationEngine] Recorded {:?} interaction for user {}", profile.interaction_history.last().unwrap().interaction_type, user_id);
            Ok(())
        } else {
            Err(format!("User profile for ID {} not found.", user_id))
        }
    }

    /// Personalizes a list of articles for a given user.
    pub async fn personalize_articles(
        &self,
        articles: Vec<OmegaNewsArticle>,
        user_id: &str,
    ) -> Result<Vec<RankedArticle>, String> {
        let user_profile = self.get_user_profile(user_id)
            .ok_or(format!("User profile for ID {} not found.", user_id))?;

        println!("[PersonalizationEngine] Personalizing {} articles for user {}", articles.len(), user_id);

        // 1. Filter articles based on explicit preferences
        let filtered_articles: Vec<OmegaNewsArticle> = articles.into_iter().filter(|article| {
            // Disliked categories
            if user_profile.preferences.disliked_categories.iter().any(|cat| article.categories.contains(cat)) {
                return false;
            }
            // Disliked sources
            if user_profile.preferences.disliked_sources.contains(&article.source_name) {
                return false;
            }
            // Min sentiment and factual accuracy
            if article.sentiment_score < user_profile.preferences.min_sentiment_score ||
               article.factual_accuracy_score < user_profile.preferences.min_factual_accuracy {
                return false;
            }
            true
        }).collect();

        // 2. Create a dynamic ranking profile based on user preferences and inferred interests
        let mut personalized_weights = RankingWeights::default();
        // Boost preferred categories/topics
        for pref_cat in &user_profile.preferences.preferred_categories {
            // Simulate boosting relevance if article is in preferred category/topic
            // In a real system, this would be more sophisticated
            personalized_weights.relevance += 0.1; 
        }
        // Adjust weights based on inferred interests (simplified)
        if !user_profile.inferred_interests.is_empty() {
            personalized_weights.relevance += 0.15;
        }
        personalized_weights.personalization = 0.5; // High personalization weight

        let dynamic_ranking_profile = RankingProfile {
            id: format!("dynamic_rank_prof_{}", Uuid::new_v4()),
            name: format!("Personalized for {}", user_id),
            description: "Dynamically generated personalization profile.".to_string(),
            weights: personalized_weights,
            is_active: true,
        };
        
        // Add this dynamic profile to the ranking algorithm (temporarily or permanently based on design)
        let profile_id = self.ranking_algorithm.add_ranking_profile(dynamic_ranking_profile).unwrap();

        // 3. Rank filtered articles using the personalized ranking profile
        let ranked_articles = self.ranking_algorithm.rank_articles(
            filtered_articles,
            &profile_id,
            Some(&user_profile.inferred_interests),
            Some(user_id),
        ).await?;

        Ok(ranked_articles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::news_engine::content_analysis::fact_checker::{FactCheckResult, FactCheckVerdict};
    use crate::news_engine::content_analysis::entity_extractor::{ExtractedEntity, EntityType};
    use crate::news_engine::news_engine::OmegaNewsArticle;

    fn create_sample_article(title: &str, content: &str, sentiment: f32, accuracy: f32, categories: Vec<String>, keywords: Vec<String>) -> OmegaNewsArticle {
        OmegaNewsArticle {
            id: format!("art_{}", Uuid::new_v4()),
            title: title.to_string(),
            content: content.to_string(),
            url: format!("http://example.com/{}", Uuid::new_v4()),
            author: Some("Test Author".to_string()),
            published_date: chrono::Utc::now(),
            ingested_date: chrono::Utc::now(),
            source_name: "Test Source".to_string(),
            categories,
            keywords,
            sentiment_score: sentiment,
            factual_accuracy_score: accuracy,
            data_lineage: vec![],
            sentiment_analysis: None,
            extracted_entities: vec![ExtractedEntity { id: Uuid::new_v4().to_string(), text: "entity".to_string(), entity_type: EntityType::Other("test".to_string()), confidence: 1.0, start_offset: 0, end_offset: 0, disambiguation_id: None }],
            fact_check_results: vec![FactCheckResult { id: Uuid::new_v4().to_string(), claim_id: Uuid::new_v4().to_string(), score: accuracy, verdict: FactCheckVerdict::True, evidence_urls: vec![], supporting_statements: vec![], refuting_statements: vec![], checks_performed: vec![], checked_at: chrono::Utc::now(), checker_agi_id: "test".to_string() }],
            identified_topics: vec![],
        }
    }

    #[tokio::test]
    async fn test_personalize_articles() {
        let ranking_algo = RankingAlgorithm::new();
        let mut personalization_engine = PersonalizationEngine::new(ranking_algo);

        let user_id = "user123";
        let user_profile = UserProfile {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            preferences: UserPreferences {
                preferred_categories: vec!["Tech".to_string()],
                disliked_categories: vec!["Politics".to_string()],
                min_sentiment_score: 0.3,
                min_factual_accuracy: 0.7,
                ..Default::default()
            },
            inferred_interests: vec!["AI".to_string(), "Rust".to_string()],
            interaction_history: vec![],
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
        };
        personalization_engine.create_or_update_user_profile(user_profile).unwrap();

        let article1 = create_sample_article("AI in Rust", "Content about AI and Rust.", 0.8, 0.9, vec!["Tech".to_string(), "Programming".to_string()], vec!["AI".to_string(), "Rust".to_string()]);
        let article2 = create_sample_article("Political Debate", "Content about politics.", 0.6, 0.8, vec!["Politics".to_string()], vec!["election".to_string()]);
        let article3 = create_sample_article("Space News", "Content about space.", 0.4, 0.6, vec!["Science".to_string()], vec!["space".to_string()]);
        let article4 = create_sample_article("Positive Tech News", "Very positive tech content.", 0.9, 0.95, vec!["Tech".to_string()], vec!["innovation".to_string()]);

        let articles = vec![article1, article2, article3, article4];
        let personalized = personalization_engine.personalize_articles(articles, user_id).await.unwrap();

        assert_eq!(personalized.len(), 2); // article2 (Politics) and article3 (low accuracy) should be filtered
        assert_eq!(personalized[0].article.title, "Positive Tech News"); // Should be highest ranked due to sentiment and category
        assert_eq!(personalized[1].article.title, "AI in Rust");
    }

    #[tokio::test]
    async fn test_record_interaction() {
        let ranking_algo = RankingAlgorithm::new();
        let mut personalization_engine = PersonalizationEngine::new(ranking_algo);

        let user_id = "user456";
        let user_profile = UserProfile {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            ..Default::default()
        };
        personalization_engine.create_or_update_user_profile(user_profile).unwrap();

        let interaction = InteractionRecord {
            article_id: "art_abc".to_string(),
            interaction_type: InteractionType::Click,
            timestamp: chrono::Utc::now(),
            sentiment_at_interaction: Some(0.7),
        };

        personalization_engine.record_interaction(user_id, interaction.clone()).unwrap();

        let profile = personalization_engine.get_user_profile(user_id).unwrap();
        assert_eq!(profile.interaction_history.len(), 1);
        assert_eq!(profile.interaction_history[0].article_id, "art_abc");
    }
}
