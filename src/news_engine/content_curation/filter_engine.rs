// src/news_engine/content_curation/filter_engine.rs

//! Provides functionality for filtering news articles based on predefined rules,
//! such as keywords, sentiment, source, or factual accuracy.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::news_engine::news_engine::OmegaNewsArticle;
use crate::news_engine::content_analysis::fact_checker::FactCheckVerdict;

/// Represents a single filtering rule.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterRule {
    pub id: String,
    pub name: String,
    pub rule_type: FilterRuleType,
    pub value: String, // The value to match against (e.g., keyword, source name)
    pub threshold: Option<f32>, // For numerical filters like sentiment, accuracy
    pub is_active: bool,
}

/// Enumeration of different types of filter rules.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FilterRuleType {
    KeywordBlacklist,
    KeywordWhitelist,
    SourceBlacklist,
    SourceWhitelist,
    CategoryBlacklist,
    CategoryWhitelist,
    MinSentimentScore,
    MaxSentimentScore,
    MinFactualAccuracy,
    MaxFactualAccuracy,
    FactCheckVerdictExclude,
    FactCheckVerdictInclude,
    LanguageExclude,
    LanguageInclude,
}

/// The `FilterEngine` service applies filtering rules to a list of news articles.
pub struct FilterEngine {
    filter_rules: HashMap<String, FilterRule>,
}

impl FilterEngine {
    /// Creates a new `FilterEngine` instance.
    pub fn new() -> Self {
        FilterEngine {
            filter_rules: HashMap::new(),
        }
    }

    /// Adds a new filtering rule.
    pub fn add_filter_rule(&mut self, mut rule: FilterRule) -> Result<String, String> {
        rule.id = format!("filter_rule_{}", Uuid::new_v4());
        println!("[FilterEngine] Adding filter rule: {}", rule.name);
        let id = rule.id.clone();
        self.filter_rules.insert(id.clone(), rule);
        Ok(id)
    }

    /// Retrieves a filter rule by its ID.
    pub fn get_filter_rule(&self, id: &str) -> Option<&FilterRule> {
        self.filter_rules.get(id)
    }

    /// Applies all active filtering rules to a list of `OmegaNewsArticle`s.
    pub fn apply_filters(&self, articles: Vec<OmegaNewsArticle>) -> Vec<OmegaNewsArticle> {
        let active_rules: Vec<&FilterRule> = self.filter_rules.values().filter(|r| r.is_active).collect();
        if active_rules.is_empty() {
            return articles; // No active filters, return all articles
        }

        println!("[FilterEngine] Applying {} active filters to {} articles.", active_rules.len(), articles.len());

        articles.into_iter().filter(|article| {
            for rule in &active_rules {
                let passes_filter = match &rule.rule_type {
                    FilterRuleType::KeywordBlacklist => !article.keywords.iter().any(|k| k.contains(&rule.value)),
                    FilterRuleType::KeywordWhitelist => article.keywords.iter().any(|k| k.contains(&rule.value)),
                    FilterRuleType::SourceBlacklist => !article.source_name.contains(&rule.value),
                    FilterRuleType::SourceWhitelist => article.source_name.contains(&rule.value),
                    FilterRuleType::CategoryBlacklist => !article.categories.iter().any(|c| c.contains(&rule.value)),
                    FilterRuleType::CategoryWhitelist => article.categories.iter().any(|c| c.contains(&rule.value)),
                    FilterRuleType::MinSentimentScore => article.sentiment_score >= rule.threshold.unwrap_or(-1.0),
                    FilterRuleType::MaxSentimentScore => article.sentiment_score <= rule.threshold.unwrap_or(1.0),
                    FilterRuleType::MinFactualAccuracy => article.factual_accuracy_score >= rule.threshold.unwrap_or(0.0),
                    FilterRuleType::MaxFactualAccuracy => article.factual_accuracy_score <= rule.threshold.unwrap_or(1.0),
                    FilterRuleType::FactCheckVerdictExclude => {
                        let excluded_verdict: Option<FactCheckVerdict> = serde_json::from_str(&rule.value).ok();
                        if let Some(ev) = excluded_verdict {
                            !article.fact_check_results.iter().any(|fcr| fcr.verdict == ev)
                        } else {
                            true // Malformed rule, ignore
                        }
                    }
                    FilterRuleType::FactCheckVerdictInclude => {
                        let included_verdict: Option<FactCheckVerdict> = serde_json::from_str(&rule.value).ok();
                        if let Some(iv) = included_verdict {
                            article.fact_check_results.iter().any(|fcr| fcr.verdict == iv)
                        } else {
                            true // Malformed rule, ignore
                        }
                    }
                    FilterRuleType::LanguageExclude => !article.content.to_lowercase().contains(&rule.value.to_lowercase()), // Simplified language check
                    FilterRuleType::LanguageInclude => article.content.to_lowercase().contains(&rule.value.to_lowercase()), // Simplified language check
                };
                if !passes_filter { return false; }
            }
            true
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use crate::news_engine::content_analysis::fact_checker::FactCheckResult;

    fn create_sample_article(title: &str, content: &str, sentiment: f32, accuracy: f32, categories: Vec<String>, keywords: Vec<String>, verdict: FactCheckVerdict) -> OmegaNewsArticle {
        OmegaNewsArticle {
            id: format!("art_{}", Uuid::new_v4()),
            title: title.to_string(),
            content: content.to_string(),
            url: format!("http://example.com/{}", Uuid::new_v4()),
            author: Some("Test Author".to_string()),
            published_date: Utc::now(),
            ingested_date: Utc::now(),
            source_name: "Test Source".to_string(),
            categories,
            keywords,
            sentiment_score: sentiment,
            factual_accuracy_score: accuracy,
            data_lineage: vec![],
            sentiment_analysis: None,
            extracted_entities: vec![],
            fact_check_results: vec![FactCheckResult { id: Uuid::new_v4().to_string(), claim_id: Uuid::new_v4().to_string(), score: accuracy, verdict, evidence_urls: vec![], supporting_statements: vec![], refuting_statements: vec![], checks_performed: vec![], checked_at: Utc::now(), checker_agi_id: "test".to_string() }],
            identified_topics: vec![],
        }
    }

    #[test]
    fn test_keyword_blacklist() {
        let mut engine = FilterEngine::new();
        engine.add_filter_rule(FilterRule {
            id: String::new(),
            name: "No AI Articles".to_string(),
            rule_type: FilterRuleType::KeywordBlacklist,
            value: "AI".to_string(),
            threshold: None,
            is_active: true,
        }).unwrap();

        let article1 = create_sample_article("AI in the Future", "Content about AI.", 0.7, 0.9, vec!["Tech".to_string()], vec!["AI".to_string()], FactCheckVerdict::True);
        let article2 = create_sample_article("Mars Exploration", "Content about space.", 0.8, 0.95, vec!["Space".to_string()], vec!["Mars".to_string()], FactCheckVerdict::True);

        let filtered = engine.apply_filters(vec![article1.clone(), article2.clone()]);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "Mars Exploration");
    }

    #[test]
    fn test_min_sentiment_score() {
        let mut engine = FilterEngine::new();
        engine.add_filter_rule(FilterRule {
            id: String::new(),
            name: "Positive News Only".to_string(),
            rule_type: FilterRuleType::MinSentimentScore,
            value: "".to_string(),
            threshold: Some(0.5),
            is_active: true,
        }).unwrap();

        let article1 = create_sample_article("Good News", "Positive content.", 0.6, 0.9, vec![], vec![], FactCheckVerdict::True);
        let article2 = create_sample_article("Bad News", "Negative content.", 0.3, 0.8, vec![], vec![], FactCheckVerdict::True);

        let filtered = engine.apply_filters(vec![article1.clone(), article2.clone()]);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "Good News");
    }

    #[test]
    fn test_fact_check_verdict_exclude() {
        let mut engine = FilterEngine::new();
        engine.add_filter_rule(FilterRule {
            id: String::new(),
            name: "Exclude False News".to_string(),
            rule_type: FilterRuleType::FactCheckVerdictExclude,
            value: serde_json::to_string(&FactCheckVerdict::False).unwrap(),
            threshold: None,
            is_active: true,
        }).unwrap();

        let article1 = create_sample_article("True Story", "Content.", 0.5, 0.9, vec![], vec![], FactCheckVerdict::True);
        let article2 = create_sample_article("False Claim", "Content.", 0.1, 0.2, vec![], vec![], FactCheckVerdict::False);

        let filtered = engine.apply_filters(vec![article1.clone(), article2.clone()]);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].title, "True Story");
    }
}
