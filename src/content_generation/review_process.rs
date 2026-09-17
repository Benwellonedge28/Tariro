// src/content_generation/review_process.rs
use crate::prelude::*;

//! Implements mechanisms for reviewing, evaluating, and refining AGI-generated content,
//! integrating human oversight and automated quality checks.

use chrono::{DateTime, Utc};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::content_generation::generator::GeneratedContent;

/// Represents a review of a piece of AGI-generated content.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContentReview {
    pub review_id: String,
    pub content_id: String, // ID of the GeneratedContent being reviewed
    pub reviewer_id: String, // ID of the human reviewer or AGI evaluation module
    pub review_date: DateTime<Utc>,
    pub overall_rating: f32, // 0.0 to 1.0
    pub feedback_comments: String,
    pub suggested_edits: Vec<EditSuggestion>,
    pub status: ReviewStatus,
    pub metrics: HashMap<String, f32>, // e.g., "factual_accuracy": 0.95, "cohesion": 0.8
}

/// Enumeration for the status of a content review.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReviewStatus {
    Pending,
    Approved,
    ApprovedWithEdits,
    Rejected,
    RevisionRequired,
    AutomatedPass,
}

/// Represents a specific suggested edit to the content.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditSuggestion {
    pub suggestion_id: String,
    pub start_char_idx: u32,
    pub end_char_idx: u32,
    pub suggested_text: String,
    pub rationale: String,
    pub applied: bool,
}

/// The `ContentReviewService` manages the lifecycle of content reviews.
pub struct ContentReviewService {
    reviews: HashMap<String, ContentReview>,
}

impl ContentReviewService {
    /// Creates a new `ContentReviewService` instance.
    pub fn new() -> Self {
        ContentReviewService {
            reviews: HashMap::new(),
        }
    }

    /// Initiates a new review process for a piece of generated content.
    pub fn start_review(
        &mut self,
        content_id: &str,
        reviewer_id: &str,
        initial_comments: Option<String>,
    ) -> Result<String, String> {
        let review_id = format!("review_{}", Uuid::new_v4());
        let review = ContentReview {
            review_id: review_id.clone(),
            content_id: content_id.to_string(),
            reviewer_id: reviewer_id.to_string(),
            review_date: Utc::now(),
            overall_rating: 0.0,
            feedback_comments: initial_comments.unwrap_or_default(),
            suggested_edits: Vec::new(),
            status: ReviewStatus::Pending,
            metrics: HashMap::new(),
        };
        println!("[ContentReviewService] Starting review {} for content {}.", review_id, content_id);
        self.reviews.insert(review_id.clone(), review);
        Ok(review_id)
    }

    /// Submits feedback and rating for an ongoing review.
    pub fn submit_feedback(
        &mut self,
        review_id: &str,
        overall_rating: f32,
        feedback_comments: String,
        suggested_edits: Vec<EditSuggestion>,
        status: ReviewStatus,
        metrics: HashMap<String, f32>,
    ) -> Result<(), String> {
        if let Some(review) = self.reviews.get_mut(review_id) {
            review.overall_rating = overall_rating.clamp(0.0, 1.0);
            review.feedback_comments = feedback_comments;
            review.suggested_edits = suggested_edits;
            review.status = status;
            review.metrics = metrics;
            review.review_date = Utc::now();
            println!("[ContentReviewService] Feedback submitted for review {}. Status: {:?}", review_id, review.status);
            Ok(())
        } else {
            Err(format!("Review with ID {} not found.", review_id))
        }
    }

    /// Retrieves a content review by its ID.
    pub fn get_review(&self, review_id: &str) -> Option<&ContentReview> {
        self.reviews.get(review_id)
    }

    /// Applies suggested edits to the generated content (simulated).
    /// In a real system, this would involve applying text diffs or feeding instructions back to the generator.
    pub async fn apply_edits(
        &self,
        content: &mut GeneratedContent,
        review_id: &str,
    ) -> Result<(), String> {
        if let Some(review) = self.reviews.get(review_id) {
            if review.status == ReviewStatus::ApprovedWithEdits || review.status == ReviewStatus::RevisionRequired {
                println!("[ContentReviewService] Applying edits from review {} to content {}.", review_id, content.id);
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                
                let mut new_content_text = content.content.clone();
                // For simplicity, just append comments and a note about applied edits
                new_content_text.push_str(&format!("\n\n-- Edits applied from review {}:\n{}", review_id, review.feedback_comments));
                
                // In a real scenario, apply each EditSuggestion using start_char_idx and end_char_idx
                // For now, let's just indicate edits were applied.
                if !review.suggested_edits.is_empty() {
                    new_content_text.push_str("\n(Specific suggested edits were processed and integrated.)");
                }

                content.content = new_content_text;
                content.last_modified = Utc::now();
                content.quality_score = Some((content.overall_rating + content.quality_score.unwrap_or(0.0)) / 2.0);
                Ok(())
            } else {
                Err(format!("Review {} is not in a state where edits can be applied.", review_id))
            }
        } else {
            Err(format!("Review with ID {} not found.", review_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_generation::generator::{ContentGenerator, ContentType, ContentTone};

    #[tokio::test]
    async fn test_start_and_submit_review() {
        let mut review_service = ContentReviewService::new();
        let generator = ContentGenerator::new();
        let content = generator.generate_content(
            "A draft for OMEGA News",
            ContentType::Article,
            None,
            ContentTone::Neutral,
            vec![],
        ).await.unwrap();

        let review_id = review_service.start_review(&content.id, "human_editor_1", Some("Initial scan looks good.".to_string())).unwrap();
        assert!(!review_id.is_empty());
        assert_eq!(review_service.get_review(&review_id).unwrap().status, ReviewStatus::Pending);

        let suggested_edits = vec![
            EditSuggestion {
                suggestion_id: Uuid::new_v4().to_string(),
                start_char_idx: 0,
                end_char_idx: 7,
                suggested_text: "OMEGA News Report".to_string(),
                rationale: "More formal opening.".to_string(),
                applied: false,
            },
        ];
        let mut metrics = HashMap::new();
        metrics.insert("factual_accuracy".to_string(), 0.95);

        review_service.submit_feedback(
            &review_id,
            0.85,
            "Good draft, needs minor adjustments.".to_string(),
            suggested_edits.clone(),
            ReviewStatus::ApprovedWithEdits,
            metrics.clone(),
        ).unwrap();

        let submitted_review = review_service.get_review(&review_id).unwrap();
        assert_eq!(submitted_review.overall_rating, 0.85);
        assert_eq!(submitted_review.status, ReviewStatus::ApprovedWithEdits);
        assert_eq!(submitted_review.suggested_edits.len(), 1);
        assert_eq!(submitted_review.metrics.get("factual_accuracy"), Some(&0.95));
    }

    #[tokio::test]
    async fn test_apply_edits() {
        let mut review_service = ContentReviewService::new();
        let generator = ContentGenerator::new();
        let mut content = generator.generate_content(
            "Another piece of content",
            ContentType::Report,
            None,
            ContentTone::Neutral,
            vec![],
        ).await.unwrap();

        let review_id = review_service.start_review(&content.id, "auto_evaluator", None).unwrap();
        let original_content_text = content.content.clone();
        
        let suggested_edits = vec![
            EditSuggestion {
                suggestion_id: Uuid::new_v4().to_string(),
                start_char_idx: 0,
                end_char_idx: 0,
                suggested_text: "Revised: ".to_string(),
                rationale: "Prefix with revised tag.".to_string(),
                applied: false,
            },
        ];

        review_service.submit_feedback(
            &review_id,
            0.9,
            "Content is factually correct but can be more engaging.".to_string(),
            suggested_edits,
            ReviewStatus::ApprovedWithEdits,
            HashMap::new(),
        ).unwrap();

        review_service.apply_edits(&mut content, &review_id).await.unwrap();

        assert!(content.content.contains(&original_content_text));
        assert!(content.content.contains("-- Edits applied from review"));
        assert!(content.last_modified > content.creation_date);
        assert!(content.quality_score.unwrap() > 0.0);
    }
}
