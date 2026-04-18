// Tariro_ide_idpte/src/co_creation/creative_models.rs

//! Defines data structures for creative artifacts, human creative styles, and preferences.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{Utc, DateTime};

/// Represents a creative artifact generated or co-created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeArtifact {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub content: String, // e.g., code, text, image description, music score
    pub artifact_type: CreativeArtifactType,
    pub created_at: DateTime<Utc>,
    pub contributors: Vec<String>, // List of human and AGI contributors
    pub version: u32,
    pub tags: Vec<String>,
    pub parent_artifact_id: Option<String>, // For iterative creation
    pub associated_styles: Vec<String>, // Styles used/learned
    pub generative_model_used: GenerativeModelType,
}

/// Types of creative artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreativeArtifactType {
    Code,
    Text,
    Image,
    Audio,
    Design,
    Documentation,
    Other(String),
}

/// Represents a learned human creative style or preference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanCreativeStyle {
    pub id: String,
    pub user_id: String,
    pub style_name: String, // e.g., "Concise Rust", "Impressionistic Art", "Jazz Fusion"
    pub characteristics: HashMap<String, String>, // e.g., "code_indentation": "4_spaces", "color_palette": "warm"
    pub learned_from_artifacts: Vec<String>, // IDs of artifacts used for learning
    pub last_updated_at: DateTime<Utc>,
    pub confidence: f64, // How confident Tariro is about this style (0.0-1.0)
}

/// Represents specific feedback on a creative artifact.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeFeedback {
    pub artifact_id: String,
    pub user_id: String,
    pub rating: f64, // e.g., 1.0-5.0
    pub comments: String,
    pub suggested_changes: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub feedback_type: FeedbackType,
}

/// Types of feedback.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedbackType {
    DirectUser,
    AutomatedAnalysis,
    PeerReview,
    ImplicitUsage,
}

/// A creative brief or prompt given to the HACSE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativePrompt {
    pub id: String,
    pub user_id: String,
    pub objective: String, // What to create
    pub desired_artifact_type: CreativeArtifactType,
    pub constraints: Vec<String>, // e.g., "Rust language", "Max 100 lines", "Modern style"
    pub reference_artifacts: Vec<String>, // IDs of examples
    pub desired_style_id: Option<String>, // ID of a learned human style to follow
    pub created_at: DateTime<Utc>,
}

/// Enum representing different types of generative AI models.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GenerativeModelType {
    CodeGenerator,
    TextGenerator,
    ImageGenerator,
    AudioGenerator,
    MultiModalGenerator,
    Custom(String),
}
