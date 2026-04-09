use reqwest::{Client, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Serialize, Deserialize};
use std::env;
use thiserror::Error; // For custom error types

const DEFAULT_TUMC_API_BASE_URL: &str = "https://api.tariro.tumc";

// --- Custom Error Type ---
#[derive(Error, Debug)]
pub enum ConnectorError {
    #[error("API request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("API returned an error status ({0}): {1}")]
    ApiError(StatusCode, String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Missing environment variable: {0}")]
    EnvironmentError(String),
}

// --- Configuration Struct ---
pub struct TumcClientConfig {
    pub base_url: String,
    pub api_key: String,
}

impl TumcClientConfig {
    pub fn from_env() -> Result<Self, ConnectorError> {
        let base_url = env::var("TUMC_API_BASE_URL")
            .unwrap_or_else(|_| DEFAULT_TUMC_API_BASE_URL.to_string());
        let api_key = env::var("TUMC_API_KEY")
            .map_err(|_| ConnectorError::EnvironmentError("TUMC_API_KEY not set".to_string()))?;
        
        Ok(TumcClientConfig { base_url, api_key })
    }
}

// --- TUMC API Data Models (derived from api_schemas/tumc_api.json) ---
#[derive(Serialize, Deserialize, Debug)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")] // Ensure camelCase for JSON keys
pub struct CompileRequest {
    pub project_id: String,
    pub source_files: Vec<SourceFile>,
    pub target_platform: String,
    pub output_format: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizations: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct CompileResponse {
    pub job_id: String,
    pub status_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // Ensure camelCase for JSON keys
pub struct JobStatusResponse {
    pub job_id: String,
    pub status: String, // "pending", "in_progress", "completed", "failed"
    pub progress_pct: Option<f32>,
    pub result_url: Option<String>,
    pub error_message: Option<String>,
}

// --- Tariro IDE TUMC Connector Client ---
pub struct TumcClient {
    client: Client,
    base_url: String,
}

impl TumcClient {
    pub async fn new(config: TumcClientConfig) -> Result<Self, ConnectorError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .map_err(|e| ConnectorError::ConfigError(format!("Invalid API Key format: {}", e)))?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(ConnectorError::ReqwestError)?;

        Ok(TumcClient { client, base_url: config.base_url })
    }

    async fn _post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        data: &T,
    ) -> Result<R, ConnectorError> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self.client.post(&url).json(data).send().await?;

        if response.status().is_success() {
            response.json().await.map_err(ConnectorError::ReqwestError)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ConnectorError::ApiError(status, error_text))
        }
    }

    async fn _get<R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
    ) -> Result<R, ConnectorError> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            response.json().await.map_err(ConnectorError::ReqwestError)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(ConnectorError::ApiError(status, error_text))
        }
    }


    pub async fn compile(
        &self,
        request: &CompileRequest,
    ) -> Result<CompileResponse, ConnectorError> {
        self._post("api/tumc/v1/compile", request).await
    }

    pub async fn get_job_status(
        &self,
        job_id: &str,
    ) -> Result<JobStatusResponse, ConnectorError> {
        self._get(&format!("api/tumc/v1/job/{}/status", job_id)).await
    }
}
