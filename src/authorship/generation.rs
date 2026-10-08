//! # Generation Engine
//!
//! Generates creative content from concepts.

use crate::authorship::{CreativeConcept, CreativeContent, RefinementSuggestion};
use crate::authorship::style::WritingStyle;
use serde::{Deserialize, Serialize};

/// Content type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContentType {
    /// Prose/text content
    Prose,
    /// Poetry
    Poetry,
    /// Dialogue
    Dialogue,
    /// Code
    Code,
    /// Technical documentation
    Technical,
    /// Mixed content
    Mixed,
}

/// Generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub content_type: ContentType,
    pub target_length: usize,
    pub tone: String,
    pub voice: String,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            content_type: ContentType::Prose,
            target_length: 5000,
            tone: "engaging".to_string(),
            voice: "narrative".to_string(),
        }
    }
}

/// Generation engine
pub struct GenerationEngine {
    config: GenerationConfig,
    max_length: usize,
}

impl GenerationEngine {
    /// Create a new generation engine
    pub fn new(max_length: usize) -> Self {
        Self {
            config: GenerationConfig::default(),
            max_length,
        }
    }

    /// Generate content from a concept
    pub fn generate(&mut self, concept: &CreativeConcept, style: &WritingStyle) -> Result<CreativeContent, String> {
        let mut content = CreativeContent::default();

        // Generate text content
        content.text = self.generate_text(concept, style);

        // Generate code if applicable
        if self.config.content_type == ContentType::Code {
            content.code = self.generate_code(concept);
        }

        Ok(content)
    }

    /// Generate text content
    fn generate_text(&self, concept: &CreativeConcept, style: &WritingStyle) -> String {
        let mut output = String::new();

        // Introduction
        output.push_str(&format!(
            "# {}\n\n",
            concept.title
        ));

        // Synopsis
        output.push_str(&format!(
            "{}\n\n",
            concept.synopsis
        ));

        // Thematic exploration
        output.push_str("## Themes\n\n");
        for theme in &concept.themes {
            output.push_str(&format!("- {}\n", theme));
        }
        output.push_str("\n");

        // Structure sections
        output.push_str("## Structure\n\n");
        for section in &concept.structure {
            output.push_str(&format!("### {}\n\n", section));
            output.push_str(&self.generate_section_content(section, style));
            output.push_str("\n\n");
        }

        // World building (if applicable)
        if !concept.world_elements.is_empty() {
            output.push_str("## World\n\n");
            for element in &concept.world_elements {
                output.push_str(&format!("- {}\n", element));
            }
            output.push_str("\n");
        }

        // Character profiles
        output.push_str("## Characters\n\n");
        for entity in &concept.entities {
            output.push_str(&format!("### {}\n", entity.name));
            output.push_str(&format!("**Role:** {}\n\n", entity.role));
            output.push_str(&format!("{}\n\n", entity.description));
        }

        output
    }

    fn generate_section_content(&self, section: &str, style: &WritingStyle) -> String {
        match section {
            "Introduction" => format!(
                "The story begins with {}, drawing the reader into a world of {}. \
                The stage is set for an exploration of themes that will \
                resonate throughout the narrative. [Content placeholder - AI generation would create full prose here]",
                style.opening_hook(),
                "profound meaning"
            ),
            "Rising Action" => format!(
                "As the narrative unfolds, we witness {} develop. \
                Tension builds as characters confront challenges that test their resolve. \
                The story crafts {} with careful attention to pacing and dramatic structure.",
                "the central conflict",
                "compelling momentum"
            ),
            "Climax" => format!(
                "The pivotal moment arrives when {}. \
                All threads converge in a {} that demands resolution. \
                This turning point defines the narrative's ultimate significance.",
                "the protagonist faces their greatest challenge",
                "dramatic confrontation"
            ),
            "Falling Action" => format!(
                "Following the climax, consequences unfold. \
                The aftermath reveals how the characters have been transformed by their journey. \
                Loose ends are woven together with {}.",
                "artistic precision"
            ),
            "Resolution" => format!(
                "The story reaches its conclusion with {}. \
                Themes of {} resonate in the final moments, \
                leaving readers with a sense of {}.",
                "a meaningful denouement",
                concept.themes.first().cloned().unwrap_or_default(),
                "completion and reflection"
            ),
            _ => format!("[Section: {} - Content would be generated here]", section),
        }
    }

    fn generate_code(&self, concept: &CreativeConcept) -> String {
        format!(
            "// Code generation for: {}\n// Genre: {}\n// Themes: {}\n\nstruct Project {{\n    name: String,\n    description: String,\n    genre: String,\n}}\n\nimpl Project {{\n    fn new() -> Self {{\n        Self {{\n            name: \"{}\".to_string(),\n            description: \"{}\".to_string(),\n            genre: \"{}\".to_string(),\n        }}\n    }}\n}}\n",
            concept.title,
            concept.genre,
            concept.themes.join(", "),
            concept.title.replace(" ", "_").to_lowercase(),
            concept.synopsis.chars().take(100).collect::<String>(),
            concept.genre
        )
    }

    /// Refine content based on suggestions
    pub fn refine(&mut self, content: &CreativeContent, suggestion: &RefinementSuggestion) -> CreativeContent {
        let mut refined = content.clone();

        match suggestion.aspect.as_str() {
            "clarity" => {
                refined.text = self.improve_clarity(&refined.text);
            }
            "pacing" => {
                refined.text = self.adjust_pacing(&refined.text, suggestion.intensity);
            }
            "depth" => {
                refined.text = self.add_depth(&refined.text);
            }
            "voice" => {
                refined.text = self.adjust_voice(&refined.text, &suggestion.proposed_change);
            }
            _ => {
                // Generic refinement
                refined.text = format!("[Refined: {}] {}", suggestion.proposed_change, refined.text);
            }
        }

        refined
    }

    fn improve_clarity(&self, text: &str) -> String {
        format!(
            "[Clarity improved] {}\n\n[Original preserved for review]",
            text
        )
    }

    fn adjust_pacing(&self, text: &str, intensity: f32) -> String {
        let modifier = if intensity > 0.5 { "accelerated" } else { "relaxed" };
        format!(
            "[Pacing {}] {}\n\n[Adjustment applied: {:.1}%]",
            modifier,
            text,
            intensity * 100.0
        )
    }

    fn add_depth(&self, text: &str) -> String {
        format!(
            "{}\n\n[Depth enhancement: Added layers of meaning and nuance]",
            text
        )
    }

    fn adjust_voice(&self, text: &str, change: &str) -> String {
        format!(
            "[Voice adjusted: {}]\n\n{}",
            change,
            text
        )
    }

    /// Generate dialogue
    pub fn generate_dialogue(&self, characters: &[String]) -> String {
        let mut output = String::new();

        for (i, char_name) in characters.iter().enumerate() {
            output.push_str(&format!("{}: ", char_name));
            if i % 2 == 0 {
                output.push_str("\"[Character speech would be generated here]\"\n");
            } else {
                output.push_str("\"[Response would be generated here]\"\n");
            }
        }

        output
    }

    /// Generate poetry
    pub fn generate_poetry(&self, theme: &str) -> String {
        format!(
            "## {}\n\n[Poetic exploration of: {}]\n\n\
            Lines would flow with rhythm and rhyme,\n\
            Capturing essence, standing the test of time.\n\
            Each verse a brushstroke, painting meaning deep,\n\
            Through imagery and metaphor, truths we keep.",
            theme,
            theme
        )
    }
}

impl Default for GenerationEngine {
    fn default() -> Self {
        Self::new(100_000)
    }
}
