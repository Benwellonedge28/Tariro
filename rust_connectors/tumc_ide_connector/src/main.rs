use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;
use log::{info, error};

// --- IDE API Data Models (derived from api_schemas/ide_api.json) ---
#[derive(Serialize, Deserialize, Debug)]
pub struct IdeNotification {
    #[serde(rename = "type")]
    pub notification_type: String, // "info", "warning", "error"
    pub title: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdeAnalysisReport {
    pub report_type: String, // "ATVE_Performance", "ATVE_Security"
    pub title: String,
    pub content: String,
    pub severity: String, // "info", "warning", "error"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

// --- TUMC's Client to IDE ---
pub struct IdeClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl IdeClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let client = Client::builder()
            .default_headers(
                reqwest::header::HeaderMap::new()
                    .insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/json"))
                    .insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap())
            )
            .build()
            .expect("Failed to build reqwest client");

        IdeClient {
            client,
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.tariro.ide".to_string()), // IDE's base URL
        }
    }

    pub async fn display_notification(&self, notification: &IdeNotification) -> Result<(), reqwest::Error> {
        let url = format!("{}/api/ide/display_notification", self.base_url);
        self.client.post(&url).json(notification).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn display_analysis_report(&self, report: &IdeAnalysisReport) -> Result<(), reqwest::Error> {
        let url = format!("{}/api/ide/display_analysis_report", self.base_url);
        self.client.post(&url).json(report).send().await?.error_for_status()?;
        Ok(())
    }
}

// --- TUMC API Endpoints (this is the TUMC server side) ---
async fn compile_handler(body: web::Json<serde_json::Value>) -> impl Responder {
    info!("Received compile request: {:?}", body);
    // In a real scenario:
    // 1. Validate request body against OpenAPI schema
    // 2. Authenticate the request (API key)
    // 3. Initiate compilation job (async)
    // 4. Return job ID
    let job_id = uuid::Uuid::new_v4().to_string();
    HttpResponse::Accepted().json(serde_json::json!({"job_id": job_id, "status_url": format!("/api/tumc/v1/job/{}/status", job_id)}))
}

async fn get_job_status_handler(path: web::Path<String>) -> impl Responder {
    let job_id = path.into_inner();
    info!("Received job status request for job_id: {}", job_id);
    // In a real scenario:
    // 1. Authenticate the request
    // 2. Query actual job status from TUMC's internal job manager
    // 3. Return status
    HttpResponse::Ok().json(serde_json::json!({
        "job_id": job_id,
        "status": "completed", // Placeholder
        "progress_pct": 100,
        "result_url": "https://tumc.tariro.com/reports/job-123.log",
        "error_message": null
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // For logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load .env for API key
    dotenv::dotenv().ok();
    let ide_api_key = env::var("IDE_API_KEY").expect("IDE_API_KEY not set in .env or environment");
    let ide_client = web::Data::new(IdeClient::new(ide_api_key, None));

    info!("TUMC IDE Connector starting on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(ide_client.clone()) // Share IdeClient with all routes
            .service(web::resource("/api/tumc/v1/compile").route(web::post().to(compile_handler)))
            .service(web::resource("/api/tumc/v1/job/{job_id}/status").route(web::get().to(get_job_status_handler)))
            // Add other TUMC API endpoints as defined in tumc_api.json
            // Example: /api/tumc/v1/analyze_code
    })
    .bind(("127.0.0.1", 8080))? // Consider making host/port configurable
    .run()
    .await
}
