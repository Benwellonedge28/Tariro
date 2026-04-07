use crate::omega_news_content_generator_llm::ContentGeneratorLlm;
use crate::authorship_module_core::{AuthorshipModule, CreativeIdea};
use crate::self_modification_module::{SelfModificationModule, ConfigChangeProposal, CodeRefactorProposal};
use crate::git_conflict_resolution_module::{GitConflictResolutionModule, GitConflictResolution}; // Import new module
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// --- EVAS Filter (Ethical, Verifiable, Accountable, Safe) ---
pub struct EvasFilter {
    llm_analyzer: ContentGeneratorLlm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvasReport {
    pub score: f32,
    pub bias_detected: bool,
    pub factual_inaccuracies: Vec<String>,
    pub ethical_violations: Vec<String>,
    pub recommendations: String,
}

impl EvasFilter {
    pub fn new(llm_analyzer: ContentGeneratorLlm) -> Self {
        EvasFilter { llm_analyzer }
    }

    pub async fn apply(&self, content: &str) -> Result<EvasReport, String> {
        println!("Applying advanced EVAS filter to content...");
        let system_instruction = "You are an AI specializing in ethical, verifiable, accountable, and safe content analysis for journalism. Provide a detailed EVAS report in JSON format.";
        let prompt = format!(
            "Analyze the following news content for ethical violations, potential biases, and factual inaccuracies. 
            Provide a confidence score (0.0-1.0), list any detected biases, factual inaccuracies, and ethical violations. 
            Finally, give recommendations for improvement. Output your analysis as a JSON object with fields: 
            `score` (f32), `bias_detected` (bool), `factual_inaccuracies` (Vec<String>), `ethical_violations` (Vec<String>), `recommendations` (String).\n\nContent:\n{}",
            content
        );

        let llm_response_json = self.llm_analyzer.generate_content(&prompt, Some(system_instruction)).await?;

        let report: EvasReport = serde_json::from_str(&llm_response_json)
            .map_err(|e| format!("Failed to parse LLM's EVAS report JSON: {}. Response: {}", e, llm_response_json))?;
        
        Ok(report)
    }
}

// --- Feed Engine (Placeholder) ---
pub struct FeedEngine;

impl FeedEngine {
    pub fn new() -> Self {
        FeedEngine {}
    }

    pub async fn ingest_feed(&self, source: &str) -> Result<String, String> {
        println!("Ingesting feed from: {}", source);
        Ok(format!("Content from {} feed.", source))
    }
}

// --- OMEGA News Core Service ---
pub struct OmegaNewsCoreService {
    llm_generator: ContentGeneratorLlm,
    evas_filter: EvasFilter,
    feed_engine: FeedEngine,
    authorship_module: AuthorshipModule,
    self_modification_module: SelfModificationModule,
    git_conflict_resolution_module: GitConflictResolutionModule, // Add GitConflictResolutionModule
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleDraft {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub evas_report: EvasReport,
}

impl OmegaNewsCoreService {
    pub fn new() -> Result<Self, String> {
        let llm_generator = ContentGeneratorLlm::new()?;
        let evas_filter = EvasFilter::new(llm_generator.clone());
        let feed_engine = FeedEngine::new();
        let authorship_module = AuthorshipModule::new()?;
        let self_modification_module = SelfModificationModule::new()?;
        let git_conflict_resolution_module = GitConflictResolutionModule::new()?; // Initialize
        Ok(OmegaNewsCoreService { 
            llm_generator, 
            evas_filter, 
            feed_engine, 
            authorship_module, 
            self_modification_module, 
            git_conflict_resolution_module 
        })
    }

    pub async fn generate_article_draft(&self, topic: &str, length_hint: &str) -> Result<ArticleDraft, String> {
        println!("Generating article draft on topic: {}", topic);
        let system_instruction = "You are an AI assistant for a cutting-edge journalism platform. Generate a concise, objective, and well-structured news article draft.";
        let prompt = format!("Generate a news article draft on the topic: '{}'. Aim for a {} length. Include a title, a brief summary, and the main body.", topic, length_hint);
        
        let full_article_text = self.llm_generator.generate_content(&prompt, Some(system_instruction)).await?;

        let mut lines = full_article_text.lines();
        let title = lines.next().unwrap_or("Untitled Article").trim().to_string();
        let summary = lines.next().unwrap_or("No summary.").trim().to_string();
        let body = lines.collect::<Vec<&str>>().join("\n").trim().to_string();

        let evas_report = self.evas_filter.apply(&full_article_text).await?;

        Ok(ArticleDraft {
            title,
            summary,
            body,
            evas_report,
        })
    }

    pub async fn summarize_news_content(&self, content: &str) -> Result<String, String> {
        println!("Summarizing news content...");
        self.llm_generator.summarize_text(content).await
    }

    pub async fn verify_news_fact(&self, fact: &str, context: &str) -> Result<String, String> {
        println!("Verifying news fact: {}", fact);
        self.llm_generator.verify_fact(fact, context).await
    }

    pub async fn process_and_filter_feed(&self, source: &str) -> Result<Vec<ArticleDraft>, String> {
        let raw_feed_content = self.feed_engine.ingest_feed(source).await?;
        println!("Raw feed content ingested. Summarizing and filtering...");
        
        let summary = self.llm_generator.summarize_text(&raw_feed_content).await?;
        
        let full_content_for_evas = format!("Summary: {}\nOriginal Content Sample: {}", summary, &raw_feed_content[..raw_feed_content.len().min(200)]);
        let evas_report = self.evas_filter.apply(&full_content_for_evas).await?;

        if evas_report.score > 0.7 && !evas_report.bias_detected && evas_report.factual_inaccuracies.is_empty() {
            let dummy_draft = ArticleDraft {
                title: format!("Summarized Article from {}", source),
                summary: summary.clone(),
                body: format!("Full content based on summary: {}\n... (truncated original content) ...", summary),
                evas_report,
            };
            Ok(vec![dummy_draft])
        } else {
            Err(format!("Feed from {} failed strict EVAS verification. Report: {:?}", source, evas_report))
        }
    }

    // --- Authorship Module Delegations ---
    pub async fn create_creative_idea(&self, topic: &str, genre: &str, audience: &str) -> Result<CreativeIdea, String> {
        self.authorship_module.generate_creative_idea(topic, genre, audience).await
    }

    pub async fn restyle_content(&self, content: &str, desired_style: &str) -> Result<String, String> {
        self.authorship_module.refine_content_style(content, desired_style).await
    }

    // --- Self-Modification Module Delegations ---
    pub async fn propose_config_change(&self, current_context: &str, desired_outcome: &str) -> Result<ConfigChangeProposal, String> {
        self.self_modification_module.propose_config_change(current_context, desired_outcome).await
    }

    pub async fn propose_code_refactor(&self, current_code: &str, refactoring_goal: &str, language: &str) -> Result<CodeRefactorProposal, String> {
        self.self_modification_module.propose_code_refactor(current_code, refactoring_goal, language).await
    }

    // --- Direct EVAS Application ---
    pub async fn run_evas_analysis(&self, content: &str) -> Result<EvasReport, String> {
        self.evas_filter.apply(content).await
    }

    // --- Git Conflict Resolution Module Delegations ---
    pub async fn resolve_git_conflict(&self, conflicting_file_content: &str, file_path: &str) -> Result<GitConflictResolution, String> {
        self.git_conflict_resolution_module.resolve_conflict(conflicting_file_content, file_path).await
    }
}
