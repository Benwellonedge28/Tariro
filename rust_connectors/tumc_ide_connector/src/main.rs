use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, http::StatusCode};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;
use log::{info, error};
use thiserror::Error;
use uuid::Uuid;

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
    #[error("Invalid request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl actix_web::ResponseError for ConnectorError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ConnectorError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ConnectorError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ConnectorError::ApiError(status, _) => status,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({ "error": self.to_string() }))
    }
}

// --- Configuration Struct ---
pub struct IdeClientConfig {
    pub base_url: String,
    pub api_key: String,
}

impl IdeClientConfig {
    pub fn from_env() -> Result<Self, ConnectorError> {
        let base_url = env::var("IDE_API_BASE_URL")
            .unwrap_or_else(|_| "https://api.tariro.ide".to_string());
        let api_key = env::var("IDE_API_KEY")
            .map_err(|_| ConnectorError::EnvironmentError("IDE_API_KEY not set".to_string()))?;
        
        Ok(IdeClientConfig { base_url, api_key })
    }
}


// --- IDE API Data Models (derived from api_schemas/ide_api.json) ---
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdeNotification {
    #[serde(rename = "type")]
    pub notification_type: String, // "info", "warning", "error"
    pub title: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdeAnalysisReport {
    pub report_type: String, // "ATVE_Performance", "ATVE_Security"
    pub title: String,
    pub content: String,
    pub severity: String, // "info", "warning", "error"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

// --- TUMC's Client to IDE ---
#[derive(Clone)] // Allow cloning for web::Data
pub struct IdeClient {
    client: Client,
    base_url: String,
}

impl IdeClient {
    pub async fn new(config: IdeClientConfig) -> Result<Self, ConnectorError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"));
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .map_err(|e| ConnectorError::ConfigError(format!("Invalid API Key format: {}", e)))?
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(ConnectorError::ReqwestError)?;

        Ok(IdeClient {
            client,
            base_url: config.base_url,
        })
    }

    pub async fn display_notification(&self, notification: &IdeNotification) -> Result<(), ConnectorError> {
        let url = format!("{}/api/ide/display_notification", self.base_url);
        self.client.post(&url).json(notification).send().await?
            .error_for_status()? // Automatically handles 4xx/5xx responses
            .bytes() // Consume the body to prevent connection issues
            .await?; 
        Ok(())
    }

    pub async fn display_analysis_report(&self, report: &IdeAnalysisReport) -> Result<(), ConnectorError> {
        let url = format!("{}/api/ide/display_analysis_report", self.base_url);
        self.client.post(&url).json(report).send().await?
            .error_for_status()?
            .bytes()
            .await?; 
        Ok(())
    }
}

// --- TUMC API Endpoints (this is the TUMC server side) ---
// Dummy authentication check
fn authenticate_request(req: &HttpRequest) -> Result<(), ConnectorError> {
    let auth_header = req.headers().get(reqwest::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    let expected_api_key = env::var("TUMC_INBOUND_API_KEY")
        .map_err(|_| ConnectorError::Unauthorized("Server's expected API key not configured".to_string()))?;

    match auth_header {
        Some(header) if header == format!("Bearer {}", expected_api_key) => Ok(()),
        _ => Err(ConnectorError::Unauthorized("Invalid or missing Authorization header".to_string())),
    }
}


async fn compile_handler(
    req: HttpRequest,
    body: web::Json<serde_json::Value>,
    ide_client: web::Data<IdeClient>
) -> Result<HttpResponse, ConnectorError> {
    authenticate_request(&req)?; // Authenticate incoming request

    info!("Received compile request: {:?}", body);
    
    // In a real scenario:
    // 1. Validate request body against OpenAPI schema
    // 2. Initiate compilation job (async)
    let job_id = Uuid::new_v4().to_string();

    // Example of calling back to IDE with notification
    let notification = IdeNotification {
        notification_type: "info".to_string(),
        title: "Compilation Initiated".to_string(),
        message: format!("TUMC started job {} for your code.", job_id),
        duration: Some(5000),
    };
    if let Err(e) = ide_client.display_notification(&notification).await {
        error!("Failed to send compilation initiation notification to IDE: {}", e);
        // This error shouldn't stop the compile request itself from succeeding, but should be logged.
    }

    Ok(HttpResponse::Accepted().json(serde_json::json!({
        "job_id": job_id,
        "status_url": format!("/api/tumc/v1/job/{}/status", job_id)
    })))
}

async fn get_job_status_handler(
    req: HttpRequest,
    path: web::Path<String>
) -> Result<HttpResponse, ConnectorError> {
    authenticate_request(&req)?; // Authenticate incoming request

    let job_id = path.into_inner();
    info!("Received job status request for job_id: {}", job_id);
    
    // In a real scenario:
    // 1. Query actual job status from TUMC's internal job manager
    // 2. Return status
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "job_id": job_id,
        "status": "completed", // Placeholder
        "progress_pct": 100,
        "result_url": "https://tumc.tariro.com/reports/job-123.log",
        "error_message": null
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv::dotenv().ok(); // Load .env file

    let ide_client_config = IdeClientConfig::from_env()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("IDE Client Config Error: {}", e)))?;
    let ide_client = web::Data::new(IdeClient::new(ide_client_config).await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create IDE Client: {}", e)))?);

    let host = env::var("TUMC_CONNECTOR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("TUMC_CONNECTOR_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    info!("TUMC IDE Connector starting on {}", bind_address);
    HttpServer::new(move || {
        App::new()
            .app_data(ide_client.clone()) // Share IdeClient with all routes
            .service(web::resource("/api/tumc/v1/compile").route(web::post().to(compile_handler)))
            .service(web::resource("/api/tumc/v1/job/{job_id}/status").route(web::get().to(get_job_status_handler)))
            // Add other TUMC API endpoints as defined in tumc_api.json
            // Example: /api/tumc/v1/analyze_code
    })
    .bind(&bind_address)?
    .run()
    .await
}


#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{matchers::{method, path, header}, MockServer, Mock, ResponseTemplate};

    #[tokio::test]
    async fn test_ide_client_display_notification_success() -> Result<(), ConnectorError> {
        let mock_ide_server = MockServer::start().await;

        // Mock the IDE's notification endpoint
        Mock::given(method("POST"))
            .and(path("/api/ide/display_notification"))
            .and(header("Authorization", "Bearer test_ide_api_key"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_ide_server)
            .await;

        let config = IdeClientConfig {
            base_url: mock_ide_server.uri(),
            api_key: "test_ide_api_key".to_string(),
        };
        let client = IdeClient::new(config).await?;

        let notification = IdeNotification {
            notification_type: "info".to_string(),
            title: "Test Title".to_string(),
            message: "Test Message".to_string(),
            duration: Some(1000),
        };

        client.display_notification(&notification).await?;

        // Verify that the mock server received the request
        mock_ide_server.verify()
            .await
            .expect("Notification request was not received by mock IDE server");

        Ok(())
    }

    #[tokio::test]
    async fn test_ide_client_display_notification_api_error() {
        let mock_ide_server = MockServer::start().await;

        // Mock the IDE's notification endpoint to return an error
        Mock::given(method("POST"))
            .and(path("/api/ide/display_notification"))
            .respond_with(ResponseTemplate::new(500).set_body_string("IDE internal error"))
            .mount(&mock_ide_server)
            .await;

        let config = IdeClientConfig {
            base_url: mock_ide_server.uri(),
            api_key: "test_ide_api_key".to_string(),
        };
        let client = IdeClient::new(config).await.unwrap();

        let notification = IdeNotification {
            notification_type: "error".to_string(),
            title: "Error".to_string(),
            message: "Failed to compile".to_string(),
            duration: None,
        };

        let error = client.display_notification(&notification).await.unwrap_err();
        match error {
            ConnectorError::ReqwestError(e) => {
                assert!(e.is_status());
                assert_eq!(e.status().unwrap(), StatusCode::INTERNAL_SERVER_ERROR);
            },
            _ => panic!("Expected ReqwestError from API error, got {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_ide_client_display_analysis_report_success() -> Result<(), ConnectorError> {
        let mock_ide_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/ide/display_analysis_report"))
            .and(header("Authorization", "Bearer test_ide_api_key"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_ide_server)
            .await;

        let config = IdeClientConfig {
            base_url: mock_ide_server.uri(),
            api_key: "test_ide_api_key".to_string(),
        };
        let client = IdeClient::new(config).await?;

        let report = IdeAnalysisReport {
            report_type: "ATVE_Performance".to_string(),
            title: "Performance Report".to_string(),
            content: "CPU usage high".to_string(),
            severity: "warning".to_string(),
            file_path: Some("/src/main.rs".to_string()),
        };

        client.display_analysis_report(&report).await?;

        mock_ide_server.verify()
            .await
            .expect("Analysis report request was not received by mock IDE server");

        Ok(())
    }
}
