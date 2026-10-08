//! # Tariro AGI Authorship Module
//!
//! A comprehensive authorship system that enables Tariro AGI to autonomously create,
//! refine, and attribute creative works across multiple modalities.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                     TARIRO AGI AUTHORKSHIP                      │
//! ├─────────────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐ ┌─────────────┐ ┌────────────────────────┐  │
//! │  │  Ideation   │ │ Generation  │ │      Critique         │  │
//! │  │  Engine     │ │ Engine      │ │      Engine           │  │
//! │  └─────────────┘ └─────────────┘ └────────────────────────┘  │
//! │  ┌─────────────┐ ┌─────────────┐ ┌────────────────────────┐  │
//! │  │ Multimodal  │ │ Attribution │ │    Collaboration      │  │
//! │  │ Generator   │ │ Manager     │ │      Engine           │  │
//! │  └─────────────┘ └─────────────┘ └────────────────────────┘  │
//! └─────────────────────────────────────────────────────────────────┘
//! ```

pub mod core;
pub mod ideation;
pub mod generation;
pub mod critique;
pub mod multimodal;
pub mod attribution;
pub mod collaboration;
pub mod style;
pub mod ethics;

pub use core::{AuthorshipEngine, AuthorshipConfig, AuthorshipResult};
pub use ideation::{IdeationEngine, CreativeConcept, IdeationResult};
pub use generation::{GenerationEngine, CreativeContent, ContentType};
pub use critique::{CritiqueEngine, CritiqueReport, RefinementSuggestion};
pub use multimodal::{MultimodalEngine, ModalOutput};
pub use attribution::{AttributionManager, ProvenanceRecord};
pub use collaboration::{CollaborationEngine, HumanFeedback};
pub use style::{StyleEngine, WritingStyle};
pub use ethics::{AuthorshipEthics, EthicsFilter};

use serde::{Deserialize, Serialize};

/// Types of creative works
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CreativeWorkType {
    /// Literary works (fiction, non-fiction, poetry)
    Literary,
    /// Code and software
    Code,
    /// Music and audio
    Music,
    /// Visual art and images
    Visual,
    /// Video and multimedia
    Video,
    /// Scientific content
    Scientific,
    /// Technical documentation
    Technical,
    /// Mixed/multimodal
    Mixed,
}

/// Creative work with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeWork {
    /// Unique identifier
    pub id: String,
    /// Title of the work
    pub title: String,
    /// Type of creative work
    pub work_type: CreativeWorkType,
    /// Main content
    pub content: CreativeContent,
    /// Provenance and attribution
    pub provenance: ProvenanceRecord,
    /// Quality metrics
    pub quality_score: f64,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Revision count
    pub revision: u32,
}

impl CreativeWork {
    /// Create a new creative work
    pub fn new(title: String, work_type: CreativeWorkType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            work_type,
            content: CreativeContent::default(),
            provenance: ProvenanceRecord::default(),
            quality_score: 0.0,
            created_at: chrono::Utc::now(),
            revision: 0,
        }
    }
}

impl Default for CreativeContent {
    fn default() -> Self {
        Self {
            text: String::new(),
            code: String::new(),
            audio: None,
            image: None,
            video: None,
            metadata: serde_json::Map::new(),
        }
    }
}

/// Multi-modal content container
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreativeContent {
    /// Text content
    pub text: String,
    /// Code content
    pub code: String,
    /// Audio data (base64 or URL)
    pub audio: Option<String>,
    /// Image data (base64 or URL)
    pub image: Option<String>,
    /// Video data (base64 or URL)
    pub video: Option<String>,
    /// Additional metadata
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Creative prompt for authorship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativePrompt {
    /// Intent or theme
    pub intent: String,
    /// Genre or domain
    pub genre: String,
    /// Target audience
    pub audience: String,
    /// Desired style
    pub style: Option<String>,
    /// Modalities to include
    pub modalities: Vec<String>,
    /// Constraints
    pub constraints: Vec<String>,
}

impl Default for CreativePrompt {
    fn default() -> Self {
        Self {
            intent: String::new(),
            genre: "general".to_string(),
            audience: "general".to_string(),
            style: None,
            modalities: vec!["text".to_string()],
            constraints: Vec::new(),
        }
    }
}
