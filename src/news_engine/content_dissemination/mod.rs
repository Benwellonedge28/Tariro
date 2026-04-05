// src/news_engine/content_dissemination/mod.rs

//! This module handles the dissemination of curated news content to various
//! output channels, such as web platforms, APIs, social media, or direct notifications.

pub mod web_publisher;
pub mod social_media_integrator;
pub mod notification_dispatcher;
