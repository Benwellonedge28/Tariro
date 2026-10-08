//! # Style Engine
//!
//! Manages writing styles and stylistic adaptation.

use serde::{Deserialize, Serialize};

/// Writing style configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingStyle {
    /// Style name
    pub name: String,
    /// Tone (formal, casual, etc.)
    pub tone: StyleTone,
    /// Voice (first-person, third-person, etc.)
    pub voice: NarrativeVoice,
    /// Pacing
    pub pacing: PacingStyle,
    /// Complexity
    pub complexity: ComplexityLevel,
    /// Descriptive density
    pub descriptive_density: f32,
}

/// Style tone
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StyleTone {
    Formal,
    Casual,
    Academic,
    Literary,
    Technical,
    Conversational,
    Humorous,
    Dramatic,
}

/// Narrative voice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NarrativeVoice {
    FirstPerson,
    SecondPerson,
    ThirdPersonLimited,
    ThirdPersonOmniscient,
    Mixed,
}

/// Pacing style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PacingStyle {
    Fast,
    Medium,
    Slow,
    Varied,
}

/// Complexity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplexityLevel {
    Simple,
    Intermediate,
    Advanced,
    Expert,
}

impl WritingStyle {
    /// Create a style by name
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "formal" => WritingStyle {
                name: "Formal".to_string(),
                tone: StyleTone::Formal,
                voice: NarrativeVoice::ThirdPersonLimited,
                pacing: PacingStyle::Medium,
                complexity: ComplexityLevel::Advanced,
                descriptive_density: 0.7,
            },
            "casual" => WritingStyle {
                name: "Casual".to_string(),
                tone: StyleTone::Casual,
                voice: NarrativeVoice::FirstPerson,
                pacing: PacingStyle::Fast,
                complexity: ComplexityLevel::Intermediate,
                descriptive_density: 0.4,
            },
            "literary" => WritingStyle {
                name: "Literary".to_string(),
                tone: StyleTone::Literary,
                voice: NarrativeVoice::ThirdPersonOmniscient,
                pacing: PacingStyle::Slow,
                complexity: ComplexityLevel::Expert,
                descriptive_density: 0.9,
            },
            "technical" => WritingStyle {
                name: "Technical".to_string(),
                tone: StyleTone::Technical,
                voice: NarrativeVoice::ThirdPersonLimited,
                pacing: PacingStyle::Medium,
                complexity: ComplexityLevel::Expert,
                descriptive_density: 0.3,
            },
            _ => WritingStyle::default(),
        }
    }

    /// Opening hook for this style
    pub fn opening_hook(&self) -> String {
        match self.tone {
            StyleTone::Formal => "In the realm of".to_string(),
            StyleTone::Literary => "A tale unfolds".to_string(),
            StyleTone::Casual => "So here's the thing".to_string(),
            StyleTone::Technical => "This document presents".to_string(),
            _ => "The story begins".to_string(),
        }
    }
}

impl Default for WritingStyle {
    fn default() -> Self {
        Self {
            name: "General".to_string(),
            tone: StyleTone::Casual,
            voice: NarrativeVoice::ThirdPersonLimited,
            pacing: PacingStyle::Medium,
            complexity: ComplexityLevel::Intermediate,
            descriptive_density: 0.5,
        }
    }
}

/// Style engine
pub struct StyleEngine {
    available_styles: Vec<String>,
}

impl StyleEngine {
    /// Create a new style engine
    pub fn new() -> Self {
        Self {
            available_styles: vec![
                "formal".to_string(),
                "casual".to_string(),
                "literary".to_string(),
                "technical".to_string(),
                "academic".to_string(),
                "humorous".to_string(),
                "dramatic".to_string(),
            ],
        }
    }

    /// Get style by name
    pub fn get_style(&self, name: Option<&str>) -> WritingStyle {
        match name {
            Some(n) => WritingStyle::from_name(n),
            None => WritingStyle::default(),
        }
    }

    /// Get all available styles
    pub fn available_styles(&self) -> &[String] {
        &self.available_styles
    }

    /// Adapt style to genre
    pub fn adapt_to_genre(&self, style: &mut WritingStyle, genre: &str) {
        let genre_lower = genre.to_lowercase();

        if genre_lower.contains("scifi") || genre_lower.contains("science fiction") {
            style.tone = StyleTone::Technical;
            style.complexity = ComplexityLevel::Advanced;
        } else if genre_lower.contains("fantasy") {
            style.tone = StyleTone::Literary;
            style.descriptive_density = 0.8;
        } else if genre_lower.contains("romance") {
            style.tone = StyleTone::Literary;
            style.descriptive_density = 0.7;
        } else if genre_lower.contains("mystery") {
            style.pacing = PacingStyle::Varied;
        }
    }
}

impl Default for StyleEngine {
    fn default() -> Self {
        Self::new()
    }
}
