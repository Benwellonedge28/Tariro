//! IDE Backend Module: Handles code management, compilation requests, and artifact handling.

#[derive(Debug, Clone, PartialEq)]
pub enum IdeBackendStatus {
    Success,
    Error(String),
    InProgress,
}

#[derive(Debug, Clone)]
pub struct IdeBackendResponse {
    pub request_id: String,
    pub status: IdeBackendStatus,
    pub payload: String,
}

pub struct IdeBackend;

impl IdeBackend {
    pub fn new() -> Self {
        IdeBackend {}
    }

    /// Processes various requests from the IDE frontend, such as saving files,
    /// triggering compilation, or requesting artifact details.
    pub fn process_request(&self, request: &str) -> String {
        println!("IDE Backend: Processing request: {}.", request);

        if request.contains("save_file") {
            format!("File saved successfully: {}", request)
        } else if request.contains("compile_code") {
            format!("Compilation requested: {}", request)
        } else if request.contains("get_artifact") {
            format!("Artifact retrieved for: {}", request)
        } else {
            format!("IDE Backend: Unknown request: {}", request)
        }
    }

    /// Simulates saving a file.
    pub fn save_file(&self, path: &str, content: &str) -> IdeBackendResponse {
        println!("IDE Backend: Saving file {} with content length {}.", path, content.len());
        IdeBackendResponse {
            request_id: format!("save_{}", path),
            status: IdeBackendStatus::Success,
            payload: format!("File '{}' saved.", path),
        }
    }

    /// Simulates triggering a compilation.
    pub fn trigger_compilation(&self, project_id: &str) -> IdeBackendResponse {
        println!("IDE Backend: Triggering compilation for project {}.", project_id);
        IdeBackendResponse {
            request_id: format!("compile_{}", project_id),
            status: IdeBackendStatus::InProgress,
            payload: "Compilation started in TUMC.".to_string(),
        }
    }

    /// Simulates retrieving a generated artifact.
    pub fn get_generated_artifact(&self, artifact_id: &str) -> IdeBackendResponse {
        println!("IDE Backend: Retrieving artifact {}.", artifact_id);
        IdeBackendResponse {
            request_id: format!("get_artifact_{}", artifact_id),
            status: IdeBackendStatus::Success,
            payload: format!("Binary data for artifact {}.", artifact_id),
        }
    }
}
