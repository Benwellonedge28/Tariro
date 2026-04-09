use super::{TumcClient, TumcClientConfig, CompileRequest, SourceFile, ConnectorError, CompileResponse, JobStatusResponse};
use wiremock::{matchers::{method, path, header}, MockServer, Mock, ResponseTemplate};
use tokio::runtime::Runtime; // For running async tests

// This test suite will run against a mock TUMC server

#[tokio::test]
async fn test_compile_success() -> Result<(), ConnectorError> {
    let mock_server = MockServer::start().await;

    // Configure the client to talk to the mock server
    let config = TumcClientConfig {
        base_url: mock_server.uri(),
        api_key: "test_api_key".to_string(),
    };
    let client = TumcClient::new(config).await?;

    // Mock the TUMC compile endpoint
    Mock::given(method("POST"))
        .and(path("/api/tumc/v1/compile"))
        .and(header("Authorization", "Bearer test_api_key"))
        .respond_with(ResponseTemplate::new(202).set_body_json(CompileResponse {
            job_id: "mock_job_123".to_string(),
            status_url: format!("{}/api/tumc/v1/job/mock_job_123/status", mock_server.uri()),
        }))
        .mount(&mock_server)
        .await;

    let request = CompileRequest {
        project_id: "test-project".to_string(),
        source_files: vec![SourceFile { path: "main.rs".to_string(), content: "fn main() {}".to_string() }],
        target_platform: "wasm".to_string(),
        output_format: "binary".to_string(),
        optimizations: None,
    };

    let response = client.compile(&request).await?;
    assert_eq!(response.job_id, "mock_job_123");
    assert!(response.status_url.contains("mock_job_123"));

    Ok(())
}

#[tokio::test]
async fn test_get_job_status_success() -> Result<(), ConnectorError> {
    let mock_server = MockServer::start().await;

    let config = TumcClientConfig {
        base_url: mock_server.uri(),
        api_key: "test_api_key".to_string(),
    };
    let client = TumcClient::new(config).await?;

    Mock::given(method("GET"))
        .and(path("/api/tumc/v1/job/mock_job_123/status"))
        .and(header("Authorization", "Bearer test_api_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(JobStatusResponse {
            job_id: "mock_job_123".to_string(),
            status: "completed".to_string(),
            progress_pct: Some(100.0),
            result_url: Some(format!("{}/results/mock_job_123", mock_server.uri())),
            error_message: None,
        }))
        .mount(&mock_server)
        .await;

    let response = client.get_job_status("mock_job_123").await?;
    assert_eq!(response.job_id, "mock_job_123");
    assert_eq!(response.status, "completed");
    assert_eq!(response.progress_pct, Some(100.0));

    Ok(())
}

#[tokio::test]
async fn test_api_error_handling() {
    let mock_server = MockServer::start().await;

    let config = TumcClientConfig {
        base_url: mock_server.uri(),
        api_key: "test_api_key".to_string(),
    };
    let client = TumcClient::new(config).await.unwrap();

    Mock::given(method("POST"))
        .and(path("/api/tumc/v1/compile"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Invalid request format"))
        .mount(&mock_server)
        .await;

    let request = CompileRequest {
        project_id: "test-project".to_string(),
        source_files: vec![], // Empty source files to trigger a mock 400
        target_platform: "wasm".to_string(),
        output_format: "binary".to_string(),
        optimizations: None,
    };

    let error = client.compile(&request).await.unwrap_err();
    match error {
        ConnectorError::ApiError(status, message) => {
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert_eq!(message, "Invalid request format");
        },
        _ => panic!("Expected ApiError, got {:?}", error),
    }
}
