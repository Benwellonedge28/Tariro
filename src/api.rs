// src/api.rs

//! Defines the external API endpoints and internal communication protocols for the Tariro AGI.

use crate::prelude::*;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::config::AGIConfig;
use crate::telemetry::{TelemetryEvent, TelemetryLogger};
use crate::admin::{AdminCommand, AdminCommandResult, AdminService};
use crate::agi_cloning::{CloningRequest, ClonedAGIInstance, CloningStatus, ReplicationService, ReplicationStatus}; // Renamed to ReplicationService
use crate::meta_learning::{LearningInsight, MetaLearningService, LearningInsightCategory};
use chrono::Utc;
use uuid::Uuid;

/// Represents a generic API response structure.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> Reply for ApiResponse<T> {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

/// State shared across API handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AGIConfig>,
    pub telemetry_logger: Arc<TelemetryLogger>,
    pub admin_service: Arc<Mutex<AdminService>>,
    pub replication_service: Arc<Mutex<ReplicationService>>,
    pub meta_learning_service: Arc<Mutex<MetaLearningService>>,
}

/// Creates a Warp filter for the application state.
fn with_state(state: AppState) -> impl Filter<Extract = (AppState,), Error = Rejection> + Clone {
    warp::any().map(move || state.clone())
}

/// Defines all API routes.
pub fn api_routes(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health_check()
        .or(admin_commands(state.clone()))
        .or(cloning_requests(state.clone()))
        .or(meta_learning_routes(state.clone()))
        .or(system_status(state.clone())) // New system status endpoint
}

/// GET /health
fn health_check() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&ApiResponse::<()> {
                success: true,
                message: "Tariro AGI is healthy and operational.".to_string(),
                data: None,
            })
        })
}

/// POST /admin/command
/// GET /admin/config
fn admin_commands(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("admin")
        .and(with_state(state.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_admin_command)
    .or(
        warp::path("admin")
            .and(warp::path("config"))
            .and(warp::get())
            .and(with_state(state.clone()))
            .and_then(handle_get_admin_config)
    )
}

async fn handle_admin_command(mut command: AdminCommand, state: AppState) -> Result<impl Reply, Rejection> {
    command.command_id = Uuid::new_v4().to_string();
    command.timestamp = Utc::now();

    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "POST".to_string(),
        path: "/admin/command".to_string(),
        status_code: 200,
        user_agent: None,
    }).await;

    let result = state.admin_service.lock().unwrap().execute_admin_command(command).await;
    Ok(warp::reply::json(&ApiResponse {
        success: result.status == "success",
        message: result.message.clone(),
        data: Some(result),
    }))
}

async fn handle_get_admin_config(state: AppState) -> Result<impl Reply, Rejection> {
    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "GET".to_string(),
        path: "/admin/config".to_string(),
        status_code: 200,
        user_agent: None,
    }).await;

    let config = state.admin_service.lock().unwrap().get_current_config().clone();
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        message: "Current AGI configuration.".to_string(),
        data: Some(config),
    }))
}

/// POST /replication/initiate
/// GET /replication/status/{id}
/// GET /replication/instances
fn cloning_requests(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("replication")
        .and(with_state(state.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_initiate_cloning)
    .or(
        warp::path("replication")
            .and(warp::path("status"))
            .and(warp::path::param())
            .and(warp::get())
            .and(with_state(state.clone()))
            .and_then(handle_get_cloning_status)
    )
    .or(
        warp::path("replication")
            .and(warp::path("instances"))
            .and(warp::get())
            .and(with_state(state.clone()))
            .and_then(handle_list_cloned_instances)
    )
}

async fn handle_initiate_cloning(mut request: CloningRequest, state: AppState) -> Result<impl Reply, Rejection> {
    request.request_id = Uuid::new_v4().to_string();
    request.creation_timestamp = Utc::now();

    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "POST".to_string(),
        path: "/replication/initiate".to_string(),
        status_code: 200,
        user_agent: None,
    }).await;

    let result = state.replication_service.lock().unwrap().initiate_replication(request).await;
    match result {
        Ok(req_id) => Ok(warp::reply::json(&ApiResponse {
            success: true,
            message: format!("Replication initiated with request ID: {}", req_id),
            data: Some(req_id),
        })),
        Err(e) => Ok(warp::reply::json(&ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

async fn handle_get_cloning_status(request_id: String, state: AppState) -> Result<impl Reply, Rejection> {
    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "GET".to_string(),
        path: format!("/replication/status/{}", request_id),
        status_code: 200,
        user_agent: None,
    }).await;

    let service = state.replication_service.lock().unwrap();
    let status = service.get_replication_status(&request_id);
    match status {
        Some(req) => Ok(warp::reply::json(&ApiResponse {
            success: true,
            message: format!("Status for request {}.", request_id),
            data: Some(req.clone()),
        })),
        None => Ok(warp::reply::json(&ApiResponse::<()> {
            success: false,
            message: format!("Replication request {} not found.", request_id),
            data: None,
        })),
    }
}

async fn handle_list_cloned_instances(state: AppState) -> Result<impl Reply, Rejection> {
    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "GET".to_string(),
        path: "/replication/instances".to_string(),
        status_code: 200,
        user_agent: None,
    }).await;

    let service = state.replication_service.lock().unwrap();
    let instances: Vec<ClonedAGIInstance> = service.list_cloned_instances().into_iter().cloned().collect();
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        message: "List of cloned AGI instances.".to_string(),
        data: Some(instances),
    }))
}

/// POST /meta_learning/insight
/// GET /meta_learning/insights
fn meta_learning_routes(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("meta_learning")
        .and(with_state(state.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_process_insight)
    .or(
        warp::path("meta_learning")
            .and(warp::path("insights"))
            .and(warp::get())
            .and(with_state(state.clone()))
            .and_then(handle_get_learning_insights)
    )
}

async fn handle_process_insight(insight: LearningInsight, state: AppState) -> Result<impl Reply, Rejection> {
    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "POST".to_string(),
        path: "/meta_learning/insight".to_string(),
        status_code: 200,
        user_agent: None,
    }).await;

    let result = state.meta_learning_service.lock().unwrap().process_insight(insight).await;
    match result {
        Ok(_) => Ok(warp::reply::json(&ApiResponse::<()> {
            success: true,
            message: "Learning insight processed.".to_string(),
            data: None,
        })),
        Err(e) => Ok(warp::reply::json(&ApiResponse::<()> {
            success: false,
            message: e,
            data: None,
        })),
    }
}

async fn handle_get_learning_insights(state: AppState) -> Result<impl Reply, Rejection> {
    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "GET".to_string(),
        path: "/meta_learning/insights".to_string(),
        status_code: 200,
        user_agent: None,
    }).await;

    let service = state.meta_learning_service.lock().unwrap();
    let insights: Vec<LearningInsight> = service.get_insights(None).into_iter().cloned().collect();
    Ok(warp::reply::json(&ApiResponse {
        success: true,
        message: "List of learning insights.".to_string(),
        data: Some(insights),
    }))
}

/// GET /status/system
fn system_status(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("status")
        .and(warp::path("system"))
        .and(warp::get())
        .and(with_state(state.clone()))
        .and_then(handle_get_system_status)
}

async fn handle_get_system_status(state: AppState) -> Result<impl Reply, Rejection> {
    state.telemetry_logger.log_event(TelemetryEvent::HttpRequest {
        method: "GET".to_string(),
        path: "/status/system".to_string(),
        status_code: 200,
        user_agent: None,
    }).await;

    // In a real scenario, this would query various services for their status
    let system_health_report = serde_json::json!({
        "agi_core": "running",
        "admin_service": "running",
        "replication_service": "running",
        "meta_learning_service": "running",
        "uptime_seconds": Utc::now().timestamp() - 1678886400, // Placeholder
        "config_version": state.config.get_setting("core.system_version"),
    });

    Ok(warp::reply::json(&ApiResponse {
        success: true,
        message: "Overall Tariro AGI system status.".to_string(),
        data: Some(system_health_report),
    }))
}
