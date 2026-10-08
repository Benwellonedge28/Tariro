//! # Authorship Core Engine
//!
//! The central orchestrator for the Tariro AGI authorship system.

use crate::authorship::{
    AuthorshipEngine, AuthorshipConfig, AuthorshipResult, AuthorshipError,
    CreativeWork, CreativePrompt, CreativeWorkType, CreativeContent,
    ideation::IdeationEngine,
    generation::GenerationEngine,
    critique::CritiqueEngine,
    multimodal::MultimodalEngine,
    attribution::AttributionManager,
    collaboration::CollaborationEngine,
    style::StyleEngine,
    ethics::AuthorshipEthics,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Instant;

/// Configuration for the authorship engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorshipConfig {
    /// Maximum iterations for self-critique
    pub max_critique_iterations: u8,
    /// Minimum quality threshold (0.0 - 1.0)
    pub min_quality_threshold: f64,
    /// Enable multimodal generation
    pub enable_multimodal: bool,
    /// Enable collaboration features
    pub enable_collaboration: bool,
    /// Maximum content length
    pub max_content_length: usize,
    /// Enable style adaptation
    pub enable_style_adaptation: bool,
    /// Ethical compliance level (1-5)
    pub ethical_compliance_level: u8,
}

impl Default for AuthorshipConfig {
    fn default() -> Self {
        Self {
            max_critique_iterations: 5,
            min_quality_threshold: 0.7,
            enable_multimodal: true,
            enable_collaboration: true,
            max_content_length: 100_000,
            enable_style_adaptation: true,
            ethical_compliance_level: 5,
        }
    }
}

/// Result from authorship operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorshipResult {
    /// Success status
    pub success: bool,
    /// Generated work
    pub work: Option<CreativeWork>,
    /// Quality score
    pub quality_score: f64,
    /// Iterations taken
    pub iterations: u32,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Errors encountered
    pub errors: Vec<String>,
}

impl AuthorshipResult {
    /// Create a successful result
    pub fn success(work: CreativeWork, iterations: u32, time_ms: u64) -> Self {
        Self {
            success: true,
            work: Some(work),
            quality_score: work.quality_score,
            iterations,
            execution_time_ms: time_ms,
            errors: Vec::new(),
        }
    }

    /// Create a failure result
    pub fn failure(errors: Vec<String>, time_ms: u64) -> Self {
        Self {
            success: false,
            work: None,
            quality_score: 0.0,
            iterations: 0,
            execution_time_ms: time_ms,
            errors,
        }
    }
}

/// Authorship errors
#[derive(Debug, Clone)]
pub enum AuthorshipError {
    /// Ideation failed
    IdeationFailed(String),
    /// Generation failed
    GenerationFailed(String),
    /// Critique failed
    CritiqueFailed(String),
    /// Ethics violation
    EthicsViolation(String),
    /// Collaboration error
    CollaborationError(String),
    /// Configuration error
    ConfigurationError(String),
}

impl std::fmt::Display for AuthorshipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdeationFailed(msg) => write!(f, "Ideation failed: {}", msg),
            Self::GenerationFailed(msg) => write!(f, "Generation failed: {}", msg),
            Self::CritiqueFailed(msg) => write!(f, "Critique failed: {}", msg),
            Self::EthicsViolation(msg) => write!(f, "Ethics violation: {}", msg),
            Self::CollaborationError(msg) => write!(f, "Collaboration error: {}", msg),
            Self::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for AuthorshipError {}

/// Main authorship engine - orchestrates the creative process
pub struct AuthorshipEngine {
    config: AuthorshipConfig,
    ideation: IdeationEngine,
    generation: GenerationEngine,
    critique: CritiqueEngine,
    multimodal: MultimodalEngine,
    attribution: AttributionManager,
    collaboration: CollaborationEngine,
    style: StyleEngine,
    ethics: AuthorshipEthics,
}

impl AuthorshipEngine {
    /// Create a new authorship engine
    pub fn new(config: AuthorshipConfig) -> Self {
        Self {
            config: config.clone(),
            ideation: IdeationEngine::new(),
            generation: GenerationEngine::new(config.max_content_length),
            critique: CritiqueEngine::new(config.min_quality_threshold),
            multimodal: MultimodalEngine::new(),
            attribution: AttributionManager::new(),
            collaboration: CollaborationEngine::new(),
            style: StyleEngine::new(),
            ethics: AuthorshipEthics::new(config.ethical_compliance_level),
        }
    }

    /// Get engine configuration
    pub fn get_config(&self) -> &AuthorshipConfig {
        &self.config
    }

    /// Create a creative work from a prompt
    pub fn create_work(&mut self, prompt: &CreativePrompt) -> AuthorshipResult {
        let start = Instant::now();
        let mut errors = Vec::new();

        // Step 1: Ideation - Generate creative concept
        let concept = match self.ideation.generate_concept(prompt) {
            Ok(c) => c,
            Err(e) => {
                return AuthorshipResult::failure(
                    vec![format!("Ideation failed: {}", e)],
                    start.elapsed().as_millis() as u64,
                );
            }
        };

        // Step 2: Apply ethics filter to concept
        if let Err(e) = self.ethics.check_concept(&concept) {
            return AuthorshipResult::failure(
                vec![format!("Ethics violation: {}", e)],
                start.elapsed().as_millis() as u64,
            );
        }

        // Step 3: Generate initial content
        let style = self.style.get_style(prompt.style.as_deref());
        let mut content = match self.generation.generate(&concept, &style) {
            Ok(c) => c,
            Err(e) => {
                return AuthorshipResult::failure(
                    vec![format!("Generation failed: {}", e)],
                    start.elapsed().as_millis() as u64,
                );
            }
        };

        // Step 4: Apply ethics filter to content
        if let Err(e) = self.ethics.check_content(&content) {
            return AuthorshipResult::failure(
                vec![format!("Ethics violation: {}", e)],
                start.elapsed().as_millis() as u64,
            );
        }

        // Step 5: Iterative self-critique and refinement
        let mut iterations = 0u32;
        let mut quality_score = 0.0;

        while iterations < self.config.max_critique_iterations {
            iterations += 1;

            let critique = self.critique.critique(&content, &concept);

            if critique.is_satisfied() {
                quality_score = critique.overall_score;
                break;
            }

            // Apply refinement suggestions
            for suggestion in &critique.suggestions {
                content = self.generation.refine(&content, suggestion);
            }

            // Check ethics after refinement
            if let Err(e) = self.ethics.check_content(&content) {
                errors.push(format!("Ethics issue during refinement: {}", e));
            }

            quality_score = critique.overall_score;
        }

        // Step 6: Generate multimodal content if enabled
        if self.config.enable_multimodal && prompt.modalities.len() > 1 {
            let multimodal_content = self.multimodal.generate(&content, &prompt.modalities);
            content.merge(multimodal_content);
        }

        // Step 7: Generate attribution
        let provenance = self.attribution.create_provenance(&concept, iterations);

        // Step 8: Create final work
        let mut work = CreativeWork::new(concept.title.clone(), self.determine_work_type(prompt));
        work.content = content;
        work.provenance = provenance;
        work.quality_score = quality_score;

        AuthorshipResult::success(work, iterations, start.elapsed().as_millis() as u64)
    }

    /// Determine work type from prompt
    fn determine_work_type(&self, prompt: &CreativePrompt) -> CreativeWorkType {
        match prompt.modalities.first().map(|s| s.as_str()) {
            Some("code") | Some("rust") | Some("python") => CreativeWorkType::Code,
            Some("audio") | Some("music") => CreativeWorkType::Music,
            Some("image") | Some("visual") => CreativeWorkType::Visual,
            Some("video") => CreativeWorkType::Video,
            _ => CreativeWorkType::Literary,
        }
    }

    /// Get current capabilities
    pub fn get_capabilities(&self) -> AuthorshipCapabilities {
        AuthorshipCapabilities {
            can_generate_text: true,
            can_generate_code: true,
            can_generate_multimodal: self.config.enable_multimodal,
            can_collaborate: self.config.enable_collaboration,
            max_iterations: self.config.max_critique_iterations,
            ethical_compliance: self.config.ethical_compliance_level,
        }
    }
}

/// Engine capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorshipCapabilities {
    pub can_generate_text: bool,
    pub can_generate_code: bool,
    pub can_generate_multimodal: bool,
    pub can_collaborate: bool,
    pub max_iterations: u8,
    pub ethical_compliance: u8,
}

impl Default for AuthorshipEngine {
    fn default() -> Self {
        Self::new(AuthorshipConfig::default())
    }
}
