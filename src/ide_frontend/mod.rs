use axum::{
    routing::{get, post},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json, Router,
};
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::path::PathBuf;

use crate::{syntax_highlighting, ide_core};
use crate::omega_news_core_service::OmegaNewsCoreService;

#[derive(Debug, Deserialize, Serialize)]
struct HighlightRequest {
    code: String,
    language: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct FileOperationRequest {
    path: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ListFilesRequest {
    path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetProjectSummaryRequest {
    base_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RunCommandRequest {
    command: String,
    cwd: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CommandOutput {
    stdout: String,
    stderr: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CompletionRequest {
    language: String,
    code_prefix: String,
    full_code_context: String,
}

// LSP Related Structs
#[derive(Debug, Deserialize, Serialize)]
struct LspStartServerRequest {
    language: String,
    root_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspStopServerRequest {
    language: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspRequest {
    language: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspNotification {
    language: String,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspDefinitionRequest {
    language: String,
    uri: String,
    position: ide_core::LspPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspTypeDefinitionRequest {
    language: String,
    uri: String,
    position: ide_core::LspPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspImplementationRequest {
    language: String,
    uri: String,
    position: ide_core::LspPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspHoverRequest {
    language: String,
    uri: String,
    position: ide_core::LspPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspReferencesRequest {
    language: String,
    uri: String,
    position: ide_core::LspPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspRenameRequest {
    language: String,
    uri: String,
    position: ide_core::LspPosition,
    new_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspRange {
    start: ide_core::LspPosition,
    end: ide_core::LspPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspCodeActionRequest {
    language: String,
    uri: String,
    range: LspRange,
    context: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspExecuteCommandRequest {
    language: String,
    command: String,
    arguments: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspSignatureHelpRequest {
    language: String,
    uri: String,
    position: ide_core::LspPosition,
}

#[derive(Debug, Deserialize, Serialize)]
struct LspFormatDocumentRequest {
    language: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchWorkspaceRequest {
    query: String,
    path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMatch {
    file_path: String,
    line_number: usize,
    line_content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LspDocumentSymbolsRequest {
    language: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LspSemanticTokensRequest {
    language: String,
    uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiGenerateFunctionRequest {
    prompt: String,
    context_code: String,
    language: String,
    file_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiReviewCodeRequest {
    language: String,
    code_to_review: String,
    user_query: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiGenerateTestsRequest {
    language: String,
    code_to_test: String,
    test_framework: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiDebugAssistRequest {
    language: String,
    code_snippet: String,
    error_message: String,
    user_question: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiOptimizeCodeRequest {
    language: String,
    code_to_optimize: String,
    optimization_goal: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiGenerateDocumentationRequest {
    language: String,
    code_to_document: String,
    doc_style: String,
    user_instructions: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiTranslateCodeRequest {
    source_language: String,
    target_language: String,
    code_to_translate: String,
    user_instructions: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiGenerateUiComponentRequest {
    language: String,
    component_description: String,
    user_instructions: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiGenerateDbSchemaRequest {
    database_type: String,
    data_requirements: String,
    user_instructions: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiGenerateCommitMessageRequest {
    language: String,
    staged_changes_summary: String,
    user_instructions: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiProjectWideRenameRequest {
    language: String,
    file_path: String,
    old_name: String,
    new_name: String,
    full_project_context: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgiRefactorExtractFunctionRequest {
    language: String,
    file_path: String,
    selected_code: String,
    new_function_name: String,
    current_file_content: String,
}

// Debugger-related structs
#[derive(Debug, Deserialize, Serialize)]
pub struct DebugStartRequest {
    session_id: String,
    language: String,
    executable_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugStopRequest {
    session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugBreakpointRequest {
    session_id: String,
    file: String,
    line: usize,
    condition: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugRemoveBreakpointRequest {
    session_id: String,
    id: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugContinueRequest {
    session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugGetStackFramesRequest {
    session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugGetVariablesRequest {
    session_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugAddWatchRequest {
    session_id: String,
    expression: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugRemoveWatchRequest {
    session_id: String,
    id: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugUpdateWatchesRequest {
    session_id: String,
}

// Plugin-related structs
#[derive(Debug, Deserialize, Serialize)]
pub struct PluginLoadRequest {
    path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginExecuteRequest {
    plugin_id: String,
    args: Vec<String>,
    input_data: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginEventRequest {
    pub plugin_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

// Authorship Module Request Structs
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateCreativeIdeaRequest {
    topic: String,
    genre: String,
    audience: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RestyleContentRequest {
    content: String,
    desired_style: String,
}

// State to be shared across handlers
struct AppState {
    lsp_client: ide_core::LspClient,
    omega_news_service: OmegaNewsCoreService,
    diagnostic_receiver: Mutex<tokio::sync::mpsc::Receiver<serde_json::Value>>, // Receiver for LSP diagnostics
    frontend_command_receiver: Mutex<tokio::sync::mpsc::Receiver<ide_core::FrontendCommand>>, // Receiver for commands from backend to frontend
}

// Handler for the root path, serving the HTML file
async fn index() -> impl IntoResponse {
    let html_content = tokio::fs::read_to_string("static/index.html").await
        .unwrap_or_else(|_| "<h1>Error loading HTML</h1>".to_string());
    Html(html_content)
}

// Handler for the /highlight endpoint
async fn highlight_handler(Json(payload): Json<HighlightRequest>) -> impl IntoResponse {
    let highlighted_html = syntax_highlighting::highlight(&payload.code, &payload.language);
    Html(highlighted_html)
}

// Handler to load file content
async fn load_file_handler(Json(payload): Json<FileOperationRequest>) -> impl IntoResponse {
    match ide_core::read_file(&payload.path).await {
        Ok(content) => (StatusCode::OK, content).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to save file content
async fn save_file_handler(Json(payload): Json<FileOperationRequest>) -> impl IntoResponse {
    if let Some(content) = payload.content {
        match ide_core::write_file(&payload.path, &content).await {
            Ok(_) => (StatusCode::OK, "File saved successfully").into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        }
    } else {
        (StatusCode::BAD_REQUEST, "No content provided to save").into_response()
    }
}

// Handler to list files in the workspace
async fn list_files_handler(Json(payload): Json<ListFilesRequest>) -> impl IntoResponse {
    match ide_core::list_workspace_files(payload.path).await {
        Ok(files) => (StatusCode::OK, Json(files)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get project summary
async fn get_project_summary_handler(Json(payload): Json<GetProjectSummaryRequest>) -> impl IntoResponse {
    match ide_core::get_project_summary(payload.base_path.as_deref().unwrap_or(".")).await {
        Ok(summary) => (StatusCode::OK, summary).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to search workspace files
async fn search_workspace_handler(Json(payload): Json<SearchWorkspaceRequest>) -> impl IntoResponse {
    match ide_core::search_workspace_files(&payload.query, payload.path).await {
        Ok(results) => {
            let matches: Vec<SearchMatch> = results.into_iter().map(|(fp, ln, lc)| SearchMatch { file_path: fp, line_number: ln, line_content: lc }).collect();
            (StatusCode::OK, Json(matches)).into_response()
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to run a command
async fn run_command_handler(Json(payload): Json<RunCommandRequest>) -> impl IntoResponse {
    match ide_core::run_command(&payload.command, &payload.cwd).await {
        Ok((stdout, stderr)) => (StatusCode::OK, Json(CommandOutput { stdout, stderr })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(CommandOutput { stdout: "".to_string(), stderr: e })).into_response(),
    }
}

// Handler for AGI-powered code completion
async fn agi_code_completion_handler(
    Json(payload): Json<CompletionRequest>,
) -> impl IntoResponse {
    match ide_core::agi_code_completion(
        &payload.language,
        &payload.code_prefix,
        &payload.full_code_context,
    ).await {
        Ok(completions) => (StatusCode::OK, Json(completions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to start an LSP language server
async fn lsp_start_server_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspStartServerRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    let root_path_buf = PathBuf::from(&payload.root_path);
    match app_state.lsp_client.inner.start_language_server(&payload.language, &root_path_buf).await {
        Ok(_) => (StatusCode::OK, "LSP server started (simulated)").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to stop an LSP language server
async fn lsp_stop_server_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspStopServerRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.shutdown_language_server(&payload.language).await {
        Ok(_) => (StatusCode::OK, "LSP server stopped").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to send a generic LSP request
async fn lsp_request_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.send_request(&payload.language, &payload.method, payload.params).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to send a generic LSP notification
async fn lsp_notification_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspNotification>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.send_notification(&payload.language, &payload.method, payload.params).await {
        Ok(_) => (StatusCode::OK, "LSP notification sent").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get definition using LSP
async fn lsp_definition_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspDefinitionRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_definition(&payload.language, &payload.uri, payload.position).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get type definition using LSP
async fn lsp_type_definition_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspTypeDefinitionRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_type_definition(&payload.language, &payload.uri, payload.position).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get implementation using LSP
async fn lsp_implementation_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspImplementationRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_implementation(&payload.language, &payload.uri, payload.position).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get hover info using LSP
async fn lsp_hover_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspHoverRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_hover_info(&payload.language, &payload.uri, payload.position).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get references using LSP
async fn lsp_references_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspReferencesRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.find_all_references(&payload.language, &payload.uri, payload.position).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to rename symbol using LSP
async fn lsp_rename_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspRenameRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.rename_symbol(&payload.language, &payload.uri, payload.position, &payload.new_name).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get code actions using LSP
async fn lsp_code_action_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspCodeActionRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    let range_start = ide_core::LspPosition { line: payload.range.start.line, character: payload.range.start.character };
    let range_end = ide_core::LspPosition { line: payload.range.end.line, character: payload.range.end.character };

    match app_state.lsp_client.inner.get_code_actions(&payload.language, &payload.uri, range_start, range_end, payload.context).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get document symbols using LSP
async fn lsp_document_symbols_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspDocumentSymbolsRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_document_symbols(&payload.language, &payload.uri).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get semantic tokens using LSP
async fn lsp_semantic_tokens_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspSemanticTokensRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_semantic_tokens_full(&payload.language, &payload.uri).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to execute a command using LSP
async fn lsp_execute_command_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspExecuteCommandRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.execute_command(&payload.language, &payload.command, payload.arguments).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to get signature help using LSP
async fn lsp_signature_help_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspSignatureHelpRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_signature_help(&payload.language, &payload.uri, payload.position).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to format document using LSP
async fn lsp_format_document_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LspFormatDocumentRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.format_document(&payload.language, &payload.uri).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Debugger API Handlers
async fn debug_start_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugStartRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    let executable_path_buf = PathBuf::from(&payload.executable_path);
    match app_state.lsp_client.inner.start_debug_session(
        payload.session_id,
        payload.language,
        executable_path_buf,
    ).await {
        Ok(_) => (StatusCode::OK, "Debugger session started").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_stop_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugStopRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.stop_debug_session(&payload.session_id).await {
        Ok(_) => (StatusCode::OK, "Debugger session stopped").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_add_breakpoint_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugBreakpointRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.add_breakpoint(
        &payload.session_id,
        &payload.file,
        payload.line,
        payload.condition,
    ).await {
        Ok(id) => (StatusCode::OK, format!("Breakpoint set with ID: {}", id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_remove_breakpoint_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugRemoveBreakpointRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.remove_breakpoint(
        &payload.session_id,
        payload.id,
    ).await {
        Ok(_) => (StatusCode::OK, "Breakpoint removed").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_continue_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugContinueRequest>, // Reusing for next, step, finish
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.debug_continue(&payload.session_id).await {
        Ok(_) => (StatusCode::OK, "Debug continue").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_next_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugContinueRequest>, // Reusing for next, step, finish
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.debug_next(&payload.session_id).await {
        Ok(_) => (StatusCode::OK, "Debug next").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_step_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugContinueRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.debug_step(&payload.session_id).await {
        Ok(_) => (StatusCode::OK, "Debug step").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_finish_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugContinueRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.debug_finish(&payload.session_id).await {
        Ok(_) => (StatusCode::OK, "Debug finish").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_get_stack_frames_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugGetStackFramesRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_debug_stack_frames(&payload.session_id).await {
        Ok(frames) => (StatusCode::OK, Json(frames)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_get_variables_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugGetVariablesRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.get_debug_variables(&payload.session_id).await {
        Ok(variables) => (StatusCode::OK, Json(variables)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_add_watch_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugAddWatchRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.add_watch_expression(&payload.session_id, &payload.expression).await {
        Ok(id) => (StatusCode::OK, format!("Watch expression added with ID: {}", id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_remove_watch_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugRemoveWatchRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.remove_watch_expression(&payload.session_id, payload.id).await {
        Ok(_) => (StatusCode::OK, "Watch expression removed").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_update_watches_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DebugUpdateWatchesRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.update_watch_expressions(&payload.session_id).await {
        Ok(variables) => (StatusCode::OK, Json(variables)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn debug_events_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    let mut events = Vec::new();
    while let Ok(event) = app_state.lsp_client.inner.debugger_event_rx.lock().await.try_recv() {
        events.push(event);
    }
    (StatusCode::OK, Json(events)).into_response()
}

// Plugin API Handlers
async fn plugin_load_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<PluginLoadRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.load_plugin(std::path::Path::new(&payload.path)).await {
        Ok(_) => (StatusCode::OK, "Plugin loaded").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn plugin_list_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.list_plugins().await {
        Ok(plugins) => (StatusCode::OK, Json(plugins)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn plugin_execute_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<PluginExecuteRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    // Pass a clone of the entire LspClient (which is Arc<LspClientInner>) as the IdeApiHost
    let ide_api_host_clone = app_state.lsp_client.clone(); 
    match app_state.lsp_client.inner.execute_plugin(&payload.plugin_id, payload.args, payload.input_data, ide_api_host_clone).await {
        Ok((stdout, stderr)) => (StatusCode::OK, Json(CommandOutput { stdout, stderr })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(CommandOutput { stdout: "".to_string(), stderr: e })).into_response(),
    }
}

// Handler for events coming from the frontend (e.g., plugin UI button clicks)
async fn plugin_event_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<PluginEventRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.lsp_client.inner.plugin_manager.send_event_to_plugin(&payload.plugin_id, &payload.event_type, payload.data).await {
        Ok(_) => (StatusCode::OK, "Plugin event sent").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Frontend Command Handlers
async fn frontend_commands_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let mut app_state = state.lock().await;
    let mut commands = Vec::new();
    // Try to receive all pending commands without blocking
    while let Ok(cmd) = app_state.frontend_command_receiver.get_mut().try_recv() {
        commands.push(cmd);
    }
    (StatusCode::OK, Json(commands)).into_response()
}

async fn frontend_response_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ide_core::FrontendResponse>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    if let Some(sender) = app_state.lsp_client.inner.frontend_response_channels.lock().await.remove(&payload.request_id) {
        let _ = sender.send(payload);
        (StatusCode::OK, "Response received").into_response()
    } else {
        (StatusCode::BAD_REQUEST, format!("No pending request for ID: {}", payload.request_id)).into_response()
    }
}

// Handler for AGI-powered function generation
async fn agi_generate_function_handler(
    Json(payload): Json<AgiGenerateFunctionRequest>,
) -> impl IntoResponse {
    match ide_core::agi_generate_function(
        &payload.prompt,
        &payload.context_code,
        &payload.language,
    ).await {
        Ok(generated_code) => (StatusCode::OK, generated_code).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered code review and explanation
async fn agi_review_code_handler(
    Json(payload): Json<AgiReviewCodeRequest>,
) -> impl IntoResponse {
    match ide_core::agi_review_code(
        &payload.language,
        &payload.code_to_review,
        &payload.user_query,
    ).await {
        Ok(review_text) => (StatusCode::OK, review_text).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered unit test generation
async fn agi_generate_tests_handler(
    Json(payload): Json<AgiGenerateTestsRequest>,
) -> impl IntoResponse {
    match ide_core::agi_generate_tests(
        &payload.language,
        &payload.code_to_test,
        &payload.test_framework,
    ).await {
        Ok(test_code) => (StatusCode::OK, test_code).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered debugging assistance
async fn agi_debug_assist_handler(
    Json(payload): Json<AgiDebugAssistRequest>,
) -> impl IntoResponse {
    match ide_core::agi_debug_assist(
        &payload.language,
        &payload.code_snippet,
        &payload.error_message,
        &payload.user_question,
    ).await {
        Ok(debug_output) => (StatusCode::OK, debug_output).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered code optimization
async fn agi_optimize_code_handler(
    Json(payload): Json<AgiOptimizeCodeRequest>,
) -> impl IntoResponse {
    match ide_core::agi_optimize_code(
        &payload.language,
        &payload.code_to_optimize,
        &payload.optimization_goal,
    ).await {
        Ok(optimized_code) => (StatusCode::OK, optimized_code).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered code documentation generation
async fn agi_generate_documentation_handler(
    Json(payload): Json<AgiGenerateDocumentationRequest>,
) -> impl IntoResponse {
    match ide_core::agi_generate_documentation(
        &payload.language,
        &payload.code_to_document,
        &payload.doc_style,
        &payload.user_instructions,
    ).await {
        Ok(documented_code) => (StatusCode::OK, documented_code).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered code translation
async fn agi_translate_code_handler(
    Json(payload): Json<AgiTranslateCodeRequest>,
) -> impl IntoResponse {
    match ide_core::agi_translate_code(
        &payload.source_language,
        &payload.target_language,
        &payload.code_to_translate,
        &payload.user_instructions,
    ).await {
        Ok(translated_code) => (StatusCode::OK, translated_code).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered UI component generation
async fn agi_generate_ui_component_handler(
    Json(payload): Json<AgiGenerateUiComponentRequest>,
) -> impl IntoResponse {
    match ide_core::agi_generate_ui_component(
        &payload.language,
        &payload.component_description,
        &payload.user_instructions,
    ).await {
        Ok(ui_code) => (StatusCode::OK, ui_code).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered database schema generation
async fn agi_generate_db_schema_handler(
    Json(payload): Json<AgiGenerateDbSchemaRequest>,
) -> impl IntoResponse {
    match ide_core::agi_generate_db_schema(
        &payload.database_type,
        &payload.data_requirements,
        &payload.user_instructions,
    ).await {
        Ok(schema_code) => (StatusCode::OK, schema_code).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered Git commit message generation
async fn agi_generate_commit_message_handler(
    Json(payload): Json<AgiGenerateCommitMessageRequest>,
) -> impl IntoResponse {
    match ide_core::agi_generate_commit_message(
        &payload.language,
        &payload.staged_changes_summary,
        &payload.user_instructions,
    ).await {
        Ok(commit_message) => (StatusCode::OK, commit_message).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered project-wide rename
async fn agi_project_wide_rename_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<AgiProjectWideRenameRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;

    match ide_core::agi_project_wide_rename(
        &payload.language,
        &payload.file_path,
        &payload.old_name,
        &payload.new_name,
        &payload.full_project_context,
    ).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler for AGI-powered 'Extract Function' refactoring
async fn agi_refactor_extract_function_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<AgiRefactorExtractFunctionRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;

    match ide_core::agi_refactor_extract_function(
        &payload.language,
        &payload.file_path,
        &payload.selected_code,
        &payload.new_function_name,
        &payload.current_file_content,
    ).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Authorship Module Handlers
async fn generate_creative_idea_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<GenerateCreativeIdeaRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.omega_news_service.create_creative_idea(
        &payload.topic,
        &payload.genre,
        &payload.audience,
    ).await {
        Ok(idea) => (StatusCode::OK, Json(idea)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn restyle_content_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
    Json(payload): Json<RestyleContentRequest>,
) -> impl IntoResponse {
    let app_state = state.lock().await;
    match app_state.omega_news_service.restyle_content(
        &payload.content,
        &payload.desired_style,
    ).await {
        Ok(restyle_content) => (StatusCode::OK, restyle_content).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

// Handler to poll for LSP diagnostic notifications
async fn lsp_diagnostics_handler(
    axum::extract::State(state): axum::extract::State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let mut app_state = state.lock().await;
    let mut diagnostics = Vec::new();
    while let Ok(diagnostic) = app_state.diagnostic_receiver.get_mut().try_recv() {
        diagnostics.push(diagnostic);
    }
    (StatusCode::OK, Json(diagnostics)).into_response()
}

pub async fn start() {
    let (lsp_diag_tx, lsp_diag_rx) = tokio::sync::mpsc::channel::<serde_json::Value>(100); 
    let (debugger_event_tx, debugger_event_rx) = tokio::sync::mpsc::channel::<ide_core::DebugEvent>(100); 
    let (frontend_command_tx, frontend_command_rx) = tokio::sync::mpsc::channel::<ide_core::FrontendCommand>(100); // For backend-to-frontend commands

    let lsp_client = ide_core::LspClient::new(
        lsp_diag_tx,
        debugger_event_tx,
        debugger_event_rx,
        frontend_command_tx.clone(), // Clone the sender for LspClient
        Arc::new(Mutex::new(HashMap::new())), // Initialize frontend_response_channels
    ); 

    let omega_news_service = OmegaNewsCoreService::new().expect("Failed to initialize OmegaNewsCoreService");

    let app_state = Arc::new(Mutex::new(AppState {
        lsp_client,
        omega_news_service,
        diagnostic_receiver: Mutex::new(lsp_diag_rx),
        frontend_command_receiver: Mutex::new(frontend_command_rx),
    }));

    let app = Router::new()
        .route("/", get(index))
        .route("/highlight", post(highlight_handler))
        .route("/api/load_file", post(load_file_handler))
        .route("/api/save_file", post(save_file_handler))
        .route("/api/list_files", post(list_files_handler))
        .route("/api/get_project_summary", post(get_project_summary_handler)) 
        .route("/api/search_workspace", post(search_workspace_handler)) 
        .route("/api/run_command", post(run_command_handler))
        .route("/api/agi/code_completion", post(agi_code_completion_handler)) 
        .route("/api/lsp/start_server", post(lsp_start_server_handler)) 
        .route("/api/lsp/stop_server", post(lsp_stop_server_handler))   
        .route("/api/lsp/request", post(lsp_request_handler))       
        .route("/api/lsp/notification", post(lsp_notification_handler)) 
        .route("/api/lsp/definition", post(lsp_definition_handler))   
        .route("/api/lsp/type_definition", post(lsp_type_definition_handler)) 
        .route("/api/lsp/implementation", post(lsp_implementation_handler))   
        .route("/api/lsp/hover", post(lsp_hover_handler))            
        .route("/api/lsp/references", post(lsp_references_handler))  
        .route("/api/lsp/rename", post(lsp_rename_handler))          
        .route("/api/lsp/code_action", post(lsp_code_action_handler)) 
        .route("/api/lsp/execute_command", post(lsp_execute_command_handler)) 
        .route("/api/lsp/signature_help", post(lsp_signature_help_handler)) 
        .route("/api/lsp/format_document", post(lsp_format_document_handler)) 
        .route("/api/lsp/document_symbols", post(lsp_document_symbols_handler)) 
        .route("/api/lsp/semantic_tokens", post(lsp_semantic_tokens_handler)) 
        .route("/api/agi/generate_function", post(agi_generate_function_handler)) 
        .route("/api/agi/generate_tests", post(agi_generate_tests_handler)) 
        .route("/api/agi/refactor/extract_function", post(agi_refactor_extract_function_handler)) 
        .route("/api/agi/project_wide_rename", post(agi_project_wide_rename_handler)) 
        .route("/api/agi/review_code", post(agi_review_code_handler)) 
        .route("/api/agi/debug_assist", post(agi_debug_assist_handler)) 
        .route("/api/agi/optimize_code", post(agi_optimize_code_handler)) 
        .route("/api/agi/generate_documentation", post(agi_generate_documentation_handler)) 
        .route("/api/agi/translate_code", post(agi_translate_code_handler)) 
        .route("/api/agi/generate_ui_component", post(agi_generate_ui_component_handler)) 
        .route("/api/agi/generate_db_schema", post(agi_generate_db_schema_handler)) 
        .route("/api/agi/generate_commit_message", post(agi_generate_commit_message_handler)) 
        // Debugger routes
        .route("/api/debug/start", post(debug_start_handler))
        .route("/api/debug/stop", post(debug_stop_handler))
        .route("/api/debug/add_breakpoint", post(debug_add_breakpoint_handler))
        .route("/api/debug/remove_breakpoint", post(debug_remove_breakpoint_handler))
        .route("/api/debug/continue", post(debug_continue_handler))
        .route("/api/debug/next", post(debug_next_handler))
        .route("/api/debug/step", post(debug_step_handler))
        .route("/api/debug/finish", post(debug_finish_handler))
        .route("/api/debug/stack_frames", post(debug_get_stack_frames_handler)) 
        .route("/api/debug/variables", post(debug_get_variables_handler)) 
        .route("/api/debug/add_watch", post(debug_add_watch_handler)) 
        .route("/api/debug/remove_watch", post(debug_remove_watch_handler)) 
        .route("/api/debug/update_watches", post(debug_update_watches_handler)) 
        .route("/api/debug/events", get(debug_events_handler))
        // Plugin routes
        .route("/api/plugins/load", post(plugin_load_handler))
        .route("/api/plugins/list", get(plugin_list_handler))
        .route("/api/plugins/execute", post(plugin_execute_handler))
        .route("/api/plugins/event", post(plugin_event_handler)) // New route for plugin events
        // Authorship Module routes
        .route("/api/authorship/generate_idea", post(generate_creative_idea_handler))
        .route("/api/authorship/restyle_content", post(restyle_content_handler))
        // Frontend communication routes
        .route("/api/frontend/commands", get(frontend_commands_handler))
        .route("/api/frontend/response", post(frontend_response_handler))
        .route("/api/lsp/diagnostics", get(lsp_diagnostics_handler)) 
        .with_state(app_state) 
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("IDE Frontend running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
