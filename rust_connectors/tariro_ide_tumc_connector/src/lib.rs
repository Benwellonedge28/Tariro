use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Serialize, Deserialize};
use std::env;

const TUMC_API_BASE_URL: &str = "https://api.tariro.tumc";

// --- TUMC API Data Models (derived from api_schemas/tumc_api.json) ---
#[derive(Serialize, Deserialize, Debug)]
pub struct SourceFile {
    pub path: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
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
    api_key: String,
    base_url: String,
}

impl TumcClient {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build Reqwest client");

        TumcClient {
            client,
            api_key,
            base_url: base_url.unwrap_or_else(|| TUMC_API_BASE_URL.to_string()),
        }
    }

    pub fn compile(
        &self,
        request: &CompileRequest,
    ) -> Result<CompileResponse, reqwest::Error> {
        let url = format!("{}/api/tumc/v1/compile", self.base_url);
        self.client.post(&url).json(request).send()?.json()
    }

    pub fn get_job_status(
        &self,
        job_id: &str,
    ) -> Result<JobStatusResponse, reqwest::Error> {
        let url = format!("{}/api/tumc/v1/job/{}/status", self.base_url, job_id);
        self.client.get(&url).send()?.json()
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv::dotenv().ok(); // Load .env file
//     let tumc_api_key = env::var("TUMC_API_KEY")
//         .expect("TUMC_API_KEY not set in .env or environment");

//     let tumc_client = TumcClient::new(tumc_api_key, None);

//     let compile_request = CompileRequest {
//         project_id: "my_ide_project_123".to_string(),
//         source_files: vec![
//             SourceFile {
//                 path: "main.c".to_string(),
//                 content: "int main() { return 0; }".to_string(),
//             },
//         ],
//         target_platform: "x86_64".to_string(),
//         output_format: "executable".to_string(),
//         optimizations: Some(vec!["O3".to_string()]),
//     };

//     println!("Initiating TUMC compilation...");
//     let compile_response = tumc_client.compile(&compile_request)?;
//     println!("Compile Job ID: {}", compile_response.job_id);

//     // In a real scenario, this would be asynchronous and polled later or
//     // triggered by a webhook from TUMC.
//     // For demonstration:
//     std::thread::sleep(std::time::Duration::from_secs(5));
//     let job_status = tumc_client.get_job_status(&compile_response.job_id)?;
//     println!("Job Status: {:?}", job_status);

//     Ok(())
// }
