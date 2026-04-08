// src/news_engine/mod.rs

//! This module orchestrates the entire OMEGA News process, from ingestion to generation and dissemination.

pub mod news_engine;
pub mod news_ingestion;
pub mod content_analysis;
pub mod content_curation;
pub mod content_dissemination;
pub mod omega_news_api; // NEW
pub mod omega_news_content_generator_llm; // NEW
pub mod omega_news_core_service; // NEW
