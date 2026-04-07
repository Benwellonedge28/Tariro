use axum::{
    routing::{post},
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::omega_news_core_service::{OmegaNewsCoreService, ArticleDraft};
use crate::authorship_module_core::CreativeIdea;

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

// --- API Request/Response Models for News Generation ---

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

// --- Handlers for News Generation ---

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

// --- Router for OMEGA News API ---

pub fn omega_news_api_router() -> Router<Arc<ApiState>> {
    Router::new()
        .route("/authorship/idea", post(generate_creative_idea_handler))
        .route("/authorship/restyle", post(restyle_content_handler))
        .route("/news/draft", post(generate_article_draft_handler))
        .route("/news/summarize", post(summarize_news_content_handler))
        .route("/news/verify_fact", post(verify_news_fact_handler))
        .route("/news/process_feed", post(process_and_filter_feed_handler))
}

