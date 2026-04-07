use crate::omega_news_content_generator_llm::ContentGeneratorLlm;
use crate::authorship_module_core::{AuthorshipModule, CreativeIdea};
use crate::self_modification_module::{SelfModificationModule, ConfigChangeProposal, CodeRefactorProposal}; // Import new module
use serde::{Deserialize, Serialize};

// --- EVAS Filter (Ethical, Verifiable, Accountable, Safe) ---
pub struct EvasFilter;

impl EvasFilter {
    pub fn new() -> Self {
        EvasFilter {}
    }

    pub fn apply(&self, content: &str) -> bool {
        println!("Applying EVAS filter to content...");
        !content.contains("harmful_keyword")
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
    self_modification_module: SelfModificationModule, // Add SelfModificationModule
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleDraft {
    pub title: String,
    pub summary: String,
    pub body: String,
    pub verified_by_evas: bool,
}

impl OmegaNewsCoreService {
    pub fn new() -> Result<Self, String> {
        let llm_generator = ContentGeneratorLlm::new()?;
        let evas_filter = EvasFilter::new();
        let feed_engine = FeedEngine::new();
        let authorship_module = AuthorshipModule::new()?;
        let self_modification_module = SelfModificationModule::new()?; // Initialize
        Ok(OmegaNewsCoreService { 
            llm_generator, 
            evas_filter, 
            feed_engine, 
            authorship_module, 
            self_modification_module 
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

        let verified = self.evas_filter.apply(&full_article_text);

        Ok(ArticleDraft {
            title,
            summary,
            body,
            verified_by_evas: verified,
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
        let is_verified = self.evas_filter.apply(&full_content_for_evas);

        if is_verified {
            let dummy_draft = ArticleDraft {
                title: format!("Summarized Article from {}", source),
                summary: summary.clone(),
                body: format!("Full content based on summary: {}", summary),
                verified_by_evas: true,
            };
            Ok(vec![dummy_draft])
        } else {
            Err(format!("Feed from {} failed EVAS verification.", source))
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
}
