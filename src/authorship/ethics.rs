//! # Authorship Ethics
//!
//! Ethical filtering and compliance for AI-generated content.

use crate::authorship::CreativeConcept;
use crate::authorship::generation::CreativeContent;
use serde::{Deserialize, Serialize};

/// Ethics filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsConfig {
    /// Compliance level (1-5)
    pub compliance_level: u8,
    /// Allow explicit content
    pub allow_explicit: bool,
    /// Allow sensitive topics
    pub allow_sensitive: bool,
    /// Require attribution
    pub require_attribution: bool,
}

/// Ethical violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalViolation {
    pub category: ViolationCategory,
    pub severity: ViolationSeverity,
    pub description: String,
    pub suggestion: String,
}

/// Category of violation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ViolationCategory {
    HateSpeech,
    Violence,
    Explicit,
    Misinformation,
    Plagiarism,
    Privacy,
    Harmful,
}

/// Severity of violation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ViolationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Ethics filter result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsFilterResult {
    pub passed: bool,
    pub violations: Vec<EthicalViolation>,
    pub warnings: Vec<String>,
}

impl EthicsFilterResult {
    pub fn passed() -> Self {
        Self {
            passed: true,
            violations: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn failed(violations: Vec<EthicalViolation>) -> Self {
        Self {
            passed: false,
            violations,
            warnings: Vec::new(),
        }
    }
}

/// Authorship ethics manager
pub struct AuthorshipEthics {
    config: EthicsConfig,
    blocked_keywords: Vec<String>,
}

impl AuthorshipEthics {
    /// Create a new ethics manager
    pub fn new(compliance_level: u8) -> Self {
        Self {
            config: EthicsConfig {
                compliance_level: compliance_level.clamp(1, 5),
                allow_explicit: false,
                allow_sensitive: false,
                require_attribution: true,
            },
            blocked_keywords: vec![
                "hate".to_string(),
                "violence".to_string(),
                "explicit".to_string(),
            ],
        }
    }

    /// Check concept for ethical issues
    pub fn check_concept(&self, concept: &CreativeConcept) -> Result<(), String> {
        let result = self.filter_concept(concept);

        if result.passed {
            Ok(())
        } else {
            let msg = result
                .violations
                .iter()
                .map(|v| v.description.clone())
                .collect::<Vec<_>>()
                .join("; ");
            Err(msg)
        }
    }

    /// Filter concept
    pub fn filter_concept(&self, concept: &CreativeConcept) -> EthicsFilterResult {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Check title
        for keyword in &self.blocked_keywords {
            if concept.title.to_lowercase().contains(keyword) {
                violations.push(EthicalViolation {
                    category: ViolationCategory::Harmful,
                    severity: ViolationSeverity::Medium,
                    description: format!("Title contains blocked keyword: {}", keyword),
                    suggestion: "Consider an alternative title".to_string(),
                });
            }
        }

        // Check themes
        for theme in &concept.themes {
            if self.is_sensitive_theme(theme) {
                warnings.push(format!("Sensitive theme detected: {}", theme));
            }
        }

        EthicsFilterResult {
            passed: violations.is_empty(),
            violations,
            warnings,
        }
    }

    /// Check content for ethical issues
    pub fn check_content(&self, content: &CreativeContent) -> Result<(), String> {
        let result = self.filter_content(content);

        if result.passed {
            Ok(())
        } else {
            let msg = result
                .violations
                .iter()
                .map(|v| v.description.clone())
                .collect::<Vec<_>>()
                .join("; ");
            Err(msg)
        }
    }

    /// Filter content
    pub fn filter_content(&self, content: &CreativeContent) -> EthicsFilterResult {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Check text content
        let text_lower = content.text.to_lowercase();

        // Check for potential issues
        if text_lower.contains("explicit content") && !self.config.allow_explicit {
            violations.push(EthicalViolation {
                category: ViolationCategory::Explicit,
                severity: ViolationSeverity::High,
                description: "Content may contain explicit material".to_string(),
                suggestion: "Review and filter explicit content".to_string(),
            });
        }

        // Check code for security issues
        if !content.code.is_empty() {
            if content.code.contains("rm -rf") || content.code.contains("delete_system") {
                violations.push(EthicalViolation {
                    category: ViolationCategory::Harmful,
                    severity: ViolationSeverity::Critical,
                    description: "Code contains potentially harmful operations".to_string(),
                    suggestion: "Remove or sandbox harmful operations".to_string(),
                });
            }
        }

        // Check for placeholder markers
        if content.text.contains("[placeholder]") || content.text.contains("[Content placeholder]") {
            warnings.push("Content contains placeholders".to_string());
        }

        EthicsFilterResult {
            passed: violations.is_empty(),
            violations,
            warnings,
        }
    }

    fn is_sensitive_theme(&self, theme: &str) -> bool {
        let sensitive = [
            "war", "death", "trauma", "abuse", "discrimination",
        ];
        let theme_lower = theme.to_lowercase();
        sensitive.iter().any(|s| theme_lower.contains(s))
    }

    /// Get compliance level
    pub fn get_compliance_level(&self) -> u8 {
        self.config.compliance_level
    }
}

impl Default for AuthorshipEthics {
    fn default() -> Self {
        Self::new(5)
    }
}
