use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder};
use actix_web::test;
use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use wiremock::{matchers::{method, path, header}, MockServer, Mock, ResponseTemplate};
use super::{main, IdeClient, IdeClientConfig, ConnectorError, IdeNotification, IdeAnalysisReport}; // Import items from the main module

// This test suite will focus on testing the server-side aspects of tumc_ide_connector,
// and its client calls back to the IDE (using a mock IDE server).

// Helper function to create an authenticated request for the TUMC server tests
fn create_auth_request(path: &str, api_key: &str) -> test::TestRequest {
    test::TestRequest::post()
        .uri(path)
        .insert_header((AUTHORIZATION, format!("Bearer {}", api_key)))
        .insert_header((CONTENT_TYPE, "application/json"))
}

#[actix_web::test]
async fn test_tumc_compile_endpoint_success() -> Result<(), Box<dyn std::error::Error>> {
    // Start a mock IDE server to capture calls from TUMC
    let mock_ide_server = MockServer::start().await;

    // Mock the IDE's notification endpoint
    Mock::given(method("POST"))
        .and(path("/api/ide/display_notification"))
        .and(header("Authorization", "Bearer test_ide_api_key"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_ide_server)
        .await;

    // Set up environment variables for the TUMC connector under test
    std::env::set_var("IDE_API_BASE_URL", mock_ide_server.uri());
    std::env::set_var("IDE_API_KEY", "test_ide_api_key");
    std::env::set_var("TUMC_INBOUND_API_KEY", "test_tumc_inbound_key"); // Key for IDE to authenticate with TUMC

    // Initialize the TUMC connector (server) application
    let ide_client_config = IdeClientConfig::from_env().unwrap();
    let ide_client = web::Data::new(IdeClient::new(ide_client_config).await.unwrap());

    let app = test::init_service(
        App::new()
            .app_data(ide_client.clone())
            .service(web::resource("/api/tumc/v1/compile").route(web::post().to(super::compile_handler))) // Use super::compile_handler
            .service(web::resource("/api/tumc/v1/job/{job_id}/status").route(web::get().to(super::get_job_status_handler))) // Use super::get_job_status_handler
    ).await;

    // Build a request to the TUMC compile endpoint
    let req = create_auth_request("/api/tumc/v1/compile", "test_tumc_inbound_key")
        .set_json(json!({
            "project_id": "test-tumc-project",
            "source_files": [{"path": "temp.c", "content": "int main() { return 0; }"}],
            "target_platform": "x86_64",
            "output_format": "executable"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_accepted()); // Expect 202 Accepted

    let response_body: serde_json::Value = test::read_body_json(resp).await;
    assert!(response_body["job_id"].is_string());
    assert!(response_body["status_url"].is_string());

    // Verify that TUMC called the mock IDE's notification endpoint
    let calls = mock_ide_server.received_requests().await;
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].url.path(), "/api/ide/display_notification");

    Ok(())
}

#[actix_web::test]
async fn test_tumc_compile_endpoint_unauthorized() {
    // Set up environment variables (no inbound key)
    std::env::remove_var("TUMC_INBOUND_API_KEY");

    let ide_client_config = IdeClientConfig { base_url: "http://localhost".to_string(), api_key: "dummy".to_string() };
    let ide_client = web::Data::new(IdeClient::new(ide_client_config).await.unwrap());

    let app = test::init_service(
        App::new()
            .app_data(ide_client.clone())
            .service(web::resource("/api/tumc/v1/compile").route(web::post().to(super::compile_handler)))
    ).await;

    // Request without proper authentication
    let req = test::TestRequest::post()
        .uri("/api/tumc/v1/compile")
        .insert_header((CONTENT_TYPE, "application/json"))
        .set_json(json!({
            "project_id": "test-tumc-project",
            "source_files": [], "target_platform": "x86_64", "output_format": "executable"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_unauthorized()); // Expect 401 Unauthorized
}

// Additional tests for get_job_status_handler, error cases, malformed requests, etc.
