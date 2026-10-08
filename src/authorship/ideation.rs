//! # Ideation Engine
//!
//! Generates creative concepts and ideas from prompts.

use crate::authorship::{CreativePrompt, CreativeConcept};
use serde::{Deserialize, Serialize};

/// Result from ideation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdeationResult {
    pub concepts: Vec<CreativeConcept>,
    pub primary_concept: CreativeConcept,
    pub exploration_depth: u8,
}

/// Creative concept from ideation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeConcept {
    /// Unique identifier
    pub id: String,
    /// Title of the concept
    pub title: String,
    /// Brief synopsis
    pub synopsis: String,
    /// Core themes
    pub themes: Vec<String>,
    /// Target genre
    pub genre: String,
    /// Target audience
    pub audience: String,
    /// Style hints
    pub style_hints: Vec<String>,
    /// Structural elements
    pub structure: Vec<String>,
    /// Character/entity concepts
    pub entities: Vec<EntityConcept>,
    /// World-building elements
    pub world_elements: Vec<String>,
}

impl CreativeConcept {
    /// Create a new concept
    pub fn new(title: String, synopsis: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            synopsis,
            themes: Vec::new(),
            genre: String::new(),
            audience: String::new(),
            style_hints: Vec::new(),
            structure: Vec::new(),
            entities: Vec::new(),
            world_elements: Vec::new(),
        }
    }
}

/// Entity concept (characters, objects, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityConcept {
    pub name: String,
    pub description: String,
    pub role: String,
    pub traits: Vec<String>,
}

/// Ideation engine
pub struct IdeationEngine {
    exploration_depth: u8,
    max_concepts: usize,
}

impl IdeationEngine {
    /// Create a new ideation engine
    pub fn new() -> Self {
        Self {
            exploration_depth: 3,
            max_concepts: 5,
        }
    }

    /// Generate concepts from a creative prompt
    pub fn generate_concept(&mut self, prompt: &CreativePrompt) -> Result<CreativeConcept, String> {
        // Generate title based on intent
        let title = self.generate_title(&prompt.intent);

        // Generate synopsis
        let synopsis = self.generate_synopsis(&prompt.intent, &prompt.genre);

        // Create base concept
        let mut concept = CreativeConcept::new(title, synopsis);
        concept.genre = prompt.genre.clone();
        concept.audience = prompt.audience.clone();

        // Generate themes
        concept.themes = self.extract_themes(&prompt.intent);

        // Generate style hints
        concept.style_hints = self.generate_style_hints(&prompt.genre);

        // Generate structural elements
        concept.structure = self.generate_structure(&prompt.genre);

        // Generate entities
        concept.entities = self.generate_entities(&concept);

        // Generate world elements
        concept.world_elements = self.generate_world_elements(&concept);

        Ok(concept)
    }

    /// Generate multiple concept options
    pub fn generate_multiple(&mut self, prompt: &CreativePrompt) -> IdeationResult {
        let concepts = (0..self.max_concepts)
            .filter_map(|_| self.generate_concept(prompt).ok())
            .collect();

        let primary = concepts.first().cloned().unwrap_or_else(|| {
            CreativeConcept::new("Default".to_string(), "Default concept".to_string())
        });

        IdeationResult {
            concepts,
            primary_concept: primary,
            exploration_depth: self.exploration_depth,
        }
    }

    fn generate_title(&self, intent: &str) -> String {
        // Simple title generation - in production would use more sophisticated methods
        let words: Vec<&str> = intent.split_whitespace().collect();
        if words.len() >= 2 {
            format!("The {} of {}",
                self.capitalize(words[0]),
                words[1..].join(" ").to_lowercase()
            )
        } else if let Some(word) = words.first() {
            self.capitalize(word)
        } else {
            "Untitled Work".to_string()
        }
    }

    fn generate_synopsis(&self, intent: &str, genre: &str) -> String {
        format!(
            "A {} exploring {}. Through challenges and growth, \
            the narrative weaves together themes of human experience \
            with the essence of: {}.",
            genre,
            self.extract_core_theme(intent),
            intent
        )
    }

    fn extract_themes(&self, intent: &str) -> Vec<String> {
        let intent_lower = intent.to_lowercase();
        let mut themes = Vec::new();

        // Keyword-based theme detection
        if intent_lower.contains("love") || intent_lower.contains("relationship") {
            themes.push("Love and Connection".to_string());
        }
        if intent_lower.contains("conflict") || intent_lower.contains("war") {
            themes.push("Conflict and Resolution".to_string());
        }
        if intent_lower.contains("discovery") || intent_lower.contains("journey") {
            themes.push("Discovery and Transformation".to_string());
        }
        if intent_lower.contains("identity") || intent_lower.contains("self") {
            themes.push("Identity and Self".to_string());
        }
        if intent_lower.contains("technology") || intent_lower.contains("future") {
            themes.push("Technology and Humanity".to_string());
        }

        if themes.is_empty() {
            themes.push("Human Experience".to_string());
        }

        themes
    }

    fn extract_core_theme(&self, intent: &str) -> String {
        let words: Vec<&str> = intent.split_whitespace().take(5).collect();
        words.join(" ").to_lowercase()
    }

    fn generate_style_hints(&self, genre: &str) -> Vec<String> {
        match genre.to_lowercase().as_str() {
            "scifi" | "science fiction" => vec![
                "Precise, technical language".to_string(),
                "Future-oriented descriptions".to_string(),
                "Logical narrative flow".to_string(),
            ],
            "fantasy" => vec![
                "Evocative, descriptive prose".to_string(),
                "Rich world-building details".to_string(),
                "Mythic narrative tone".to_string(),
            ],
            "mystery" => vec![
                "Atmospheric tension".to_string(),
                "Layered revelations".to_string(),
                "Intricate plot structure".to_string(),
            ],
            "romance" => vec![
                "Emotional depth".to_string(),
                "Sensory-rich descriptions".to_string(),
                "Character-driven narrative".to_string(),
            ],
            _ => vec![
                "Engaging prose".to_string(),
                "Clear narrative".to_string(),
                "Balanced pacing".to_string(),
            ],
        }
    }

    fn generate_structure(&self, genre: &str) -> Vec<String> {
        let mut structure = vec![
            "Introduction".to_string(),
            "Rising Action".to_string(),
            "Climax".to_string(),
            "Falling Action".to_string(),
            "Resolution".to_string(),
        ];

        if genre.to_lowercase().contains("mystery") {
            structure.insert(1, "Setup and Hook".to_string());
        }

        structure
    }

    fn generate_entities(&self, concept: &CreativeConcept) -> Vec<EntityConcept> {
        vec![
            EntityConcept {
                name: "Protagonist".to_string(),
                description: "A central character navigating the narrative".to_string(),
                role: "Main character".to_string(),
                traits: vec!["Determined".to_string(), "Curious".to_string()],
            },
            EntityConcept {
                name: "Challenge".to_string(),
                description: "The central conflict driving the story".to_string(),
                role: "Antagonist or obstacle".to_string(),
                traits: vec!["Compelling".to_string(), "Mysterious".to_string()],
            },
        ]
    }

    fn generate_world_elements(&self, concept: &CreativeConcept) -> Vec<String> {
        if concept.genre.to_lowercase().contains("fantasy") {
            vec![
                "Unique magic system".to_string(),
                "Distinctive geography".to_string(),
                "Rich cultural history".to_string(),
            ]
        } else if concept.genre.to_lowercase().contains("scifi") {
            vec![
                "Technological framework".to_string(),
                "Future society structure".to_string(),
                "Scientific concepts".to_string(),
            ]
        } else {
            vec![
                "Realistic setting".to_string(),
                "Authentic social context".to_string(),
            ]
        }
    }

    fn capitalize(&self, s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}

impl Default for IdeationEngine {
    fn default() -> Self {
        Self::new()
    }
}
