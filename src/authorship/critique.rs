//! # Critique Engine
//!
//! Self-critique and refinement engine for creative content.

use crate::authorship::{CreativeConcept, CreativeContent, RefinementSuggestion};
use serde::{Deserialize, Serialize};

/// Critique report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritiqueReport {
    pub overall_score: f64,
    pub is_satisfied: bool,
    pub dimension_scores: DimensionScores,
    pub issues: Vec<CritiqueIssue>,
    pub suggestions: Vec<RefinementSuggestion>,
}

/// Scores across different creative dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionScores {
    pub originality: f64,
    pub coherence: f64,
    pub engagement: f64,
    pub depth: f64,
    pub clarity: f64,
    pub style: f64,
    pub emotional_impact: f64,
}

impl DimensionScores {
    pub fn average(&self) -> f64 {
        (self.originality + self.coherence + self.engagement +
         self.depth + self.clarity + self.style + self.emotional_impact) / 7.0
    }
}

/// Individual critique issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CritiqueIssue {
    pub dimension: String,
    pub severity: IssueSeverity,
    pub description: String,
    pub location: Option<String>,
}

/// Severity level for issues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueSeverity {
    Minor,
    Moderate,
    Significant,
    Critical,
}

/// Critique engine
pub struct CritiqueEngine {
    min_quality_threshold: f64,
}

impl CritiqueEngine {
    /// Create a new critique engine
    pub fn new(min_quality_threshold: f64) -> Self {
        Self {
            min_quality_threshold,
        }
    }

    /// Critique content against a concept
    pub fn critique(&self, content: &CreativeContent, concept: &CreativeConcept) -> CritiqueReport {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();

        // Evaluate dimensions
        let dimension_scores = self.evaluate_dimensions(content, concept, &mut issues, &mut suggestions);

        let overall_score = dimension_scores.average();
        let is_satisfied = overall_score >= self.min_quality_threshold &&
                           issues.iter().all(|i| i.severity < IssueSeverity::Significant);

        CritiqueReport {
            overall_score,
            is_satisfied,
            dimension_scores,
            issues,
            suggestions,
        }
    }

    /// Evaluate all creative dimensions
    fn evaluate_dimensions(
        &self,
        content: &CreativeContent,
        concept: &CreativeConcept,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> DimensionScores {
        // Evaluate originality
        let originality = self.evaluate_originality(content, concept, issues, suggestions);

        // Evaluate coherence
        let coherence = self.evaluate_coherence(content, issues, suggestions);

        // Evaluate engagement
        let engagement = self.evaluate_engagement(content, issues, suggestions);

        // Evaluate depth
        let depth = self.evaluate_depth(content, concept, issues, suggestions);

        // Evaluate clarity
        let clarity = self.evaluate_clarity(content, issues, suggestions);

        // Evaluate style
        let style = self.evaluate_style(content, concept, issues, suggestions);

        // Evaluate emotional impact
        let emotional_impact = self.evaluate_emotional_impact(content, issues, suggestions);

        DimensionScores {
            originality,
            coherence,
            engagement,
            depth,
            clarity,
            style,
            emotional_impact,
        }
    }

    fn evaluate_originality(
        &self,
        content: &CreativeContent,
        concept: &CreativeConcept,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> f64 {
        let mut score = 0.7; // Base score

        // Check for template-like patterns
        if content.text.contains("[placeholder]") || content.text.contains("[Content placeholder]") {
            issues.push(CritiqueIssue {
                dimension: "originality".to_string(),
                severity: IssueSeverity::Significant,
                description: "Content contains placeholders".to_string(),
                location: Some("body".to_string()),
            });
            suggestions.push(RefinementSuggestion {
                aspect: "originality".to_string(),
                proposed_change: "Replace placeholders with original content".to_string(),
                intensity: 0.8,
            });
            score -= 0.2;
        }

        // Check concept engagement
        if concept.themes.is_empty() {
            score -= 0.1;
        }

        score.max(0.0).min(1.0)
    }

    fn evaluate_coherence(
        &self,
        content: &CreativeContent,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> f64 {
        let mut score = 0.8;

        // Check for abrupt transitions
        if content.text.contains("## Structure") && content.text.contains("### Resolution") {
            // Good structure flow
        } else {
            issues.push(CritiqueIssue {
                dimension: "coherence".to_string(),
                severity: IssueSeverity::Minor,
                description: "Review narrative flow and transitions".to_string(),
                location: None,
            });
            score -= 0.1;
        }

        score.max(0.0).min(1.0)
    }

    fn evaluate_engagement(
        &self,
        content: &CreativeContent,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> f64 {
        let mut score = 0.75;

        // Check text length (engagement often correlates with sufficient length)
        if content.text.len() < 500 {
            issues.push(CritiqueIssue {
                dimension: "engagement".to_string(),
                severity: IssueSeverity::Moderate,
                description: "Content may be too brief for sustained engagement".to_string(),
                location: Some("overall".to_string()),
            });
            suggestions.push(RefinementSuggestion {
                aspect: "depth".to_string(),
                proposed_change: "Expand narrative with more detail and description".to_string(),
                intensity: 0.7,
            });
            score -= 0.15;
        }

        score.max(0.0).min(1.0)
    }

    fn evaluate_depth(
        &self,
        content: &CreativeContent,
        concept: &CreativeConcept,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> f64 {
        let mut score = 0.7;

        // Check theme exploration
        if concept.themes.len() > 2 {
            // Multiple themes explored - good
            score += 0.1;
        }

        // Check entity development
        if concept.entities.is_empty() {
            issues.push(CritiqueIssue {
                dimension: "depth".to_string(),
                severity: IssueSeverity::Moderate,
                description: "Limited character/entity development".to_string(),
                location: Some("Characters section".to_string()),
            });
            score -= 0.1;
        }

        score.max(0.0).min(1.0)
    }

    fn evaluate_clarity(
        &self,
        content: &CreativeContent,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> f64 {
        let mut score = 0.85;

        // Simple heuristic: check for very long sentences
        let long_sentences = content.text.lines()
            .filter(|l| l.len() > 200)
            .count();

        if long_sentences > 5 {
            issues.push(CritiqueIssue {
                dimension: "clarity".to_string(),
                severity: IssueSeverity::Minor,
                description: "Some sentences may be too long".to_string(),
                location: None,
            });
            suggestions.push(RefinementSuggestion {
                aspect: "clarity".to_string(),
                proposed_change: "Break long sentences for better readability".to_string(),
                intensity: 0.3,
            });
            score -= 0.05;
        }

        score.max(0.0).min(1.0)
    }

    fn evaluate_style(
        &self,
        content: &CreativeContent,
        concept: &CreativeConcept,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> f64 {
        let mut score = 0.8;

        // Check genre consistency
        if !concept.style_hints.is_empty() {
            // Style hints provided - would check consistency in full implementation
        }

        score.max(0.0).min(1.0)
    }

    fn evaluate_emotional_impact(
        &self,
        content: &CreativeContent,
        issues: &mut Vec<CritiqueIssue>,
        suggestions: &mut Vec<RefinementSuggestion>,
    ) -> f64 {
        let mut score = 0.7;

        // This is subjective - simplified heuristic
        let emotional_words = ["love", "fear", "hope", "joy", "anger", "sorrow", "wonder", "passion"];
        let text_lower = content.text.to_lowercase();
        let emotional_count: usize = emotional_words
            .iter()
            .map(|w| if text_lower.contains(w) { 1 } else { 0 })
            .sum();

        if emotional_count < 2 {
            issues.push(CritiqueIssue {
                dimension: "emotional_impact".to_string(),
                severity: IssueSeverity::Minor,
                description: "Limited emotional resonance detected".to_string(),
                location: None,
            });
            suggestions.push(RefinementSuggestion {
                aspect: "voice".to_string(),
                proposed_change: "Add more emotional depth and resonance".to_string(),
                intensity: 0.4,
            });
            score -= 0.1;
        }

        score.max(0.0).min(1.0)
    }

    /// Evaluate quality score
    pub fn evaluate_quality(&self, content: &CreativeContent, concept: &CreativeConcept) -> f64 {
        self.critique(content, concept).overall_score
    }
}

impl Default for CritiqueEngine {
    fn default() -> Self {
        Self::new(0.7)
    }
}
