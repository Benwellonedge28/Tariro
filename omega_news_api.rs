use axum::{
    routing::{post},
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::omega_news_core_service::{OmegaNewsCoreService, ArticleDraft, EvasReport}; // Import EvasReport
use crate::authorship_module_core::CreativeIdea;
use crate::self_modification_module::{ConfigChangeProposal, CodeRefactorProposal};

// --- API Request/Response Models for Authorship Module ---

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateCreativeIdeaRequest {
    pub topic: String,
    pub genre: String,
    pub audience: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RestyleContentRequest {
    pub content: String,
    pub desired_style: String,
}

// --- API Request/Response Models for Self-Modification Module ---

#[derive(Debug, Deserialize, Serialize)]
pub struct ProposeConfigChangeRequest {
    pub current_context: String,
    pub desired_outcome: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProposeCodeRefactorRequest {
    pub current_code: String,
    pub refactoring_goal: String,
    pub language: String,
}

// --- API Request/Response Models for News Generation and EVAS ---

#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateArticleDraftRequest {
    pub topic: String,
    pub length_hint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SummarizeNewsContentRequest {
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyNewsFactRequest {
    pub fact: String,
    pub context: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessAndFilterFeedRequest {
    pub source: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunEvasAnalysisRequest {
    pub content: String,
}

// --- AppState for Axum Router ---

pub struct ApiState {
    pub omega_news_service: OmegaNewsCoreService,
}

// --- Handlers for Authorship Module ---

pub async fn generate_creative_idea_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<GenerateCreativeIdeaRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.create_creative_idea(&payload.topic, &payload.genre, &payload.audience).await {
        Ok(idea) => (StatusCode::OK, Json(idea)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn restyle_content_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<RestyleContentRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.restyle_content(&payload.content, &payload.desired_style).await {
        Ok(re_styled_content) => (StatusCode::OK, re_styled_content).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// --- Handlers for Self-Modification Module ---

pub async fn propose_config_change_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<ProposeConfigChangeRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.propose_config_change(&payload.current_context, &payload.desired_outcome).await {
        Ok(proposal) => (StatusCode::OK, Json(proposal)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn propose_code_refactor_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<ProposeCodeRefactorRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.propose_code_refactor(&payload.current_code, &payload.refactoring_goal, &payload.language).await {
        Ok(proposal) => (StatusCode::OK, Json(proposal)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// --- Handlers for News Generation and EVAS ---

pub async fn generate_article_draft_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<GenerateArticleDraftRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.generate_article_draft(&payload.topic, &payload.length_hint).await {
        Ok(draft) => (StatusCode::OK, Json(draft)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn summarize_news_content_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<SummarizeNewsContentRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.summarize_news_content(&payload.content).await {
        Ok(summary) => (StatusCode::OK, summary).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn verify_news_fact_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<VerifyNewsFactRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.verify_news_fact(&payload.fact, &payload.context).await {
        Ok(verification_result) => (StatusCode::OK, verification_result).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn process_and_filter_feed_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<ProcessAndFilterFeedRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.process_and_filter_feed(&payload.source).await {
        Ok(articles) => (StatusCode::OK, Json(articles)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn run_evas_analysis_handler(
    axum::extract::State(state): axum::extract::State<Arc<ApiState>>,
    Json(payload): Json<RunEvasAnalysisRequest>,
) -> impl IntoResponse {
    match state.omega_news_service.run_evas_analysis(&payload.content).await {
        Ok(report) => (StatusCode::OK, Json(report)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// --- Router for OMEGA News API ---

pub fn omega_news_api_router() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/authorship/idea", post(generate_creative_idea_handler))
        .route("/authorship/restyle", post(restyle_content_handler))
        .route("/self-modification/config-change", post(propose_config_change_handler))
        .route("/self-modification/code-refactor", post(propose_code_refactor_handler))
        .route("/news/draft", post(generate_article_draft_handler))
        .route("/news/summarize", post(summarize_news_content_handler))
        .route("/news/verify_fact", post(verify_news_fact_handler))
        .route("/news/process_feed", post(process_and_filter_feed_handler))
        .route("/news/evas-analysis", post(run_evas_analysis_handler)) // New EVAS analysis route
}
