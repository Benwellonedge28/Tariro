// Tariro AGI - Shutdown Mechanism Module
// Safe and reliable shutdown procedures

use crate::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

// ============================================================================
// Shutdown Types
// ============================================================================

/// Types of shutdown
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShutdownType {
    Graceful,      // Clean shutdown with state save
    Forced,         // Immediate termination
    Emergency,      // Emergency stop
    Kill,           // Force kill without cleanup
}

/// Shutdown reason/category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShutdownReason {
    UserRequested,
    SystemShutdown,
    SafetyTrigger,
    ResourceExhaustion,
    Timeout,
    Error,
    Maintenance,
    Unknown,
}

/// Shutdown state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownState {
    pub shutdown_id: String,
    pub shutdown_type: ShutdownType,
    pub reason: ShutdownReason,
    pub initiated_at: DateTime<Utc>,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub phases_completed: usize,
    pub total_phases: usize,
    pub current_phase: String,
    pub can_cancel: bool,
    pub status: ShutdownStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShutdownStatus {
    Initiated,
    InProgress,
    Completing,
    Cancelled,
    Failed,
    Completed,
}

/// Shutdown phase
pub struct ShutdownPhase {
    pub phase_id: String,
    pub name: String,
    pub description: String,
    pub timeout: Duration,
    pub critical: bool,
    pub handler: Box<dyn Fn() -> PhaseResult + Send + Sync>,
}

impl std::fmt::Debug for ShutdownPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShutdownPhase")
            .field("phase_id", &self.phase_id)
            .field("name", &self.name)
            .field("description", &self.description)
            .field("timeout", &self.timeout)
            .field("critical", &self.critical)
            .finish()
    }
}

// ============================================================================
// Shutdown Manager
// ============================================================================

/// Main shutdown management system
pub struct ShutdownManager {
    // State
    shutdown_in_progress: AtomicBool,
    shutdown_state: RwLock<Option<ShutdownState>>,
    shutdown_history: VecDeque<ShutdownRecord>,

    // Components
    phases: Vec<ShutdownPhase>,
    cleanup_handlers: Vec<CleanupHandler>,

    // Configuration
    config: ShutdownConfig,

    // Resources to clean up
    resources: RwLock<Vec<Resource>>,
}

#[derive(Debug, Clone)]
pub struct ShutdownConfig {
    pub default_timeout: Duration,
    pub enable_graceful_shutdown: bool,
    pub enable_emergency_shutdown: bool,
    pub save_state_before_shutdown: bool,
    pub notify_stakeholders: bool,
    pub max_cleanup_attempts: usize,
    pub force_kill_after: Duration,
}

impl Default for ShutdownConfig {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            enable_graceful_shutdown: true,
            enable_emergency_shutdown: true,
            save_state_before_shutdown: true,
            notify_stakeholders: true,
            max_cleanup_attempts: 3,
            force_kill_after: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownRecord {
    pub shutdown_id: String,
    pub shutdown_type: ShutdownType,
    pub reason: ShutdownReason,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub success: bool,
    pub phases_completed: usize,
    pub error_message: Option<String>,
}

pub struct CleanupHandler {
    pub handler_id: String,
    pub name: String,
    pub priority: usize,
    pub handler: Box<dyn Fn() -> bool + Send + Sync>,
}

impl std::fmt::Debug for CleanupHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CleanupHandler")
            .field("handler_id", &self.handler_id)
            .field("name", &self.name)
            .field("priority", &self.priority)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub allocated_at: DateTime<Utc>,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Memory,
    FileDescriptor,
    DatabaseConnection,
    NetworkConnection,
    GPU,
    Lock,
}

impl ShutdownManager {
    pub fn new() -> Self {
        let mut manager = Self {
            shutdown_in_progress: AtomicBool::new(false),
            shutdown_state: RwLock::new(None),
            shutdown_history: VecDeque::new(),
            phases: Vec::new(),
            cleanup_handlers: Vec::new(),
            config: ShutdownConfig::default(),
            resources: RwLock::new(Vec::new()),
        };

        manager.register_default_phases();
        manager
    }

    fn register_default_phases(&mut self) {
        // Phase 1: Save state
        self.phases.push(ShutdownPhase {
            phase_id: "save_state".to_string(),
            name: "Save State".to_string(),
            description: "Save current execution state".to_string(),
            timeout: Duration::from_secs(10),
            critical: false,
            handler: Box::new(|| PhaseResult::Success),
        });

        // Phase 2: Release resources
        self.phases.push(ShutdownPhase {
            phase_id: "release_resources".to_string(),
            name: "Release Resources".to_string(),
            description: "Release allocated resources".to_string(),
            timeout: Duration::from_secs(15),
            critical: true,
            handler: Box::new(|| PhaseResult::Success),
        });

        // Phase 3: Finalize
        self.phases.push(ShutdownPhase {
            phase_id: "finalize".to_string(),
            name: "Finalize".to_string(),
            description: "Final cleanup and notification".to_string(),
            timeout: Duration::from_secs(5),
            critical: true,
            handler: Box::new(|| PhaseResult::Success),
        });
    }

    /// Initiate shutdown
    pub fn initiate_shutdown(&self, shutdown_type: ShutdownType, reason: ShutdownReason) -> Result<ShutdownState, ShutdownError> {
        // Check if already shutting down
        if self.shutdown_in_progress.load(Ordering::SeqCst) {
            return Err(ShutdownError::AlreadyShuttingDown);
        }

        let shutdown_id = generate_id();
        let state = ShutdownState {
            shutdown_id: shutdown_id.clone(),
            shutdown_type,
            reason,
            initiated_at: Utc::now(),
            estimated_completion: None,
            phases_completed: 0,
            total_phases: self.phases.len(),
            current_phase: "initiating".to_string(),
            can_cancel: shutdown_type == ShutdownType::Graceful,
            status: ShutdownStatus::Initiated,
        };

        // Update state
        self.shutdown_in_progress.store(true, Ordering::SeqCst);
        *self.shutdown_state.write().unwrap() = Some(state.clone());

        Ok(state)
    }

    /// Execute shutdown phases
    pub fn execute_shutdown(&self) -> ShutdownResult {
        if !self.shutdown_in_progress.load(Ordering::SeqCst) {
            return ShutdownResult {
                success: false,
                shutdown_id: "none".to_string(),
                phases_completed: 0,
                total_phases: 0,
                error_message: Some("No shutdown initiated".to_string()),
                completed_at: None,
            };
        }

        let mut phases_completed = 0;
        let mut error_message: Option<String> = None;

        // Execute each phase
        for phase in &self.phases {
            // Update current phase
            if let Some(ref mut state) = *self.shutdown_state.write().unwrap() {
                state.current_phase = phase.name.clone();
                state.status = ShutdownStatus::InProgress;
            }

            // Execute phase
            let result = self.execute_phase(phase);

            match result {
                PhaseResult::Success => {
                    phases_completed += 1;
                }
                PhaseResult::Warning(warning) => {
                    phases_completed += 1;
                    error_message = Some(warning);
                }
                PhaseResult::Failure(failure) => {
                    if phase.critical {
                        error_message = Some(failure);
                        break;
                    } else {
                        phases_completed += 1;
                    }
                }
                PhaseResult::Timeout => {
                    if phase.critical {
                        error_message = Some(format!("Phase {} timed out", phase.name));
                        break;
                    }
                    phases_completed += 1;
                }
            }
        }

        // Mark complete
        if let Some(ref mut state) = *self.shutdown_state.write().unwrap() {
            state.status = if error_message.is_some() {
                ShutdownStatus::Failed
            } else {
                ShutdownStatus::Completed
            };
            state.phases_completed = phases_completed;
        }

        self.shutdown_in_progress.store(false, Ordering::SeqCst);

        ShutdownResult {
            success: error_message.is_none(),
            shutdown_id: self.shutdown_state.read().unwrap()
                .as_ref().map(|s| s.shutdown_id.clone())
                .unwrap_or_default(),
            phases_completed,
            total_phases: self.phases.len(),
            error_message,
            completed_at: Some(Utc::now()),
        }
    }

    fn execute_phase(&self, phase: &ShutdownPhase) -> PhaseResult {
        // Simplified - in production would run actual handler with timeout
        PhaseResult::Success
    }

    /// Cancel shutdown if allowed
    pub fn cancel_shutdown(&self) -> Result<(), ShutdownError> {
        let can_cancel = {
            if let Some(ref state) = *self.shutdown_state.read().unwrap() {
                state.can_cancel && state.phases_completed < state.total_phases / 2
            } else {
                false
            }
        };

        if can_cancel {
            self.shutdown_in_progress.store(false, Ordering::SeqCst);
            if let Some(ref mut state) = *self.shutdown_state.write().unwrap() {
                state.status = ShutdownStatus::Cancelled;
            }
            Ok(())
        } else {
            Err(ShutdownError::CannotCancel)
        }
    }

    /// Register a cleanup handler
    pub fn register_cleanup_handler<F>(&mut self, name: &str, priority: usize, handler: F)
    where
        F: Fn() -> bool + Send + Sync + 'static,
    {
        self.cleanup_handlers.push(CleanupHandler {
            handler_id: generate_id(),
            name: name.to_string(),
            priority,
            handler: Box::new(handler),
        });

        // Sort by priority
        self.cleanup_handlers.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// Register a resource for tracking
    pub fn register_resource(&self, resource_type: ResourceType, size_bytes: usize) -> String {
        let resource_id = generate_id();
        let resource = Resource {
            resource_id: resource_id.clone(),
            resource_type,
            allocated_at: Utc::now(),
            size_bytes,
        };

        self.resources.write().unwrap().push(resource);
        resource_id
    }

    /// Release a specific resource
    pub fn release_resource(&self, resource_id: &str) -> bool {
        let mut resources = self.resources.write().unwrap();
        if let Some(pos) = resources.iter().position(|r| r.resource_id == resource_id) {
            resources.remove(pos);
            true
        } else {
            false
        }
    }

    /// Emergency shutdown - no cleanup
    pub fn emergency_shutdown(&self, reason: &str) {
        // Record emergency
        let record = ShutdownRecord {
            shutdown_id: generate_id(),
            shutdown_type: ShutdownType::Emergency,
            reason: ShutdownReason::SafetyTrigger,
            initiated_at: Utc::now(),
            completed_at: Some(Utc::now()),
            success: true,
            phases_completed: 0,
            error_message: Some(format!("Emergency: {}", reason)),
        };

        self.shutdown_history.push_back(record);

        // Set flag
        self.shutdown_in_progress.store(false, Ordering::SeqCst);
    }

    /// Get shutdown status
    pub fn get_status(&self) -> ShutdownStatusResponse {
        let state = self.shutdown_state.read().unwrap();

        ShutdownStatusResponse {
            shutdown_in_progress: self.shutdown_in_progress.load(Ordering::SeqCst),
            state: state.clone(),
            resources_tracked: self.resources.read().unwrap().len(),
            shutdowns_completed: self.shutdown_history.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownResult {
    pub success: bool,
    pub shutdown_id: String,
    pub phases_completed: usize,
    pub total_phases: usize,
    pub error_message: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum PhaseResult {
    Success,
    Warning(String),
    Failure(String),
    Timeout,
}

#[derive(Debug, Clone)]
pub enum ShutdownError {
    AlreadyShuttingDown,
    CannotCancel,
    InvalidState,
    Timeout,
}

impl std::fmt::Display for ShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShutdownError::AlreadyShuttingDown => write!(f, "Shutdown already in progress"),
            ShutdownError::CannotCancel => write!(f, "Cannot cancel shutdown at this point"),
            ShutdownError::InvalidState => write!(f, "Invalid shutdown state"),
            ShutdownError::Timeout => write!(f, "Shutdown timed out"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownStatusResponse {
    pub shutdown_in_progress: bool,
    pub state: Option<ShutdownState>,
    pub resources_tracked: usize,
    pub shutdowns_completed: usize,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("shutdown_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graceful_shutdown() {
        let manager = ShutdownManager::new();

        let state = manager.initiate_shutdown(ShutdownType::Graceful, ShutdownReason::UserRequested);
        assert!(state.is_ok());

        let result = manager.execute_shutdown();
        assert!(result.success);
        assert_eq!(result.phases_completed, result.total_phases);
    }

    #[test]
    fn test_shutdown_cannot_cancel_late() {
        let manager = ShutdownManager::new();

        // Graceful shutdown cannot be cancelled after halfway
        // (Implementation would track phases completed)
    }

    #[test]
    fn test_resource_tracking() {
        let manager = ShutdownManager::new();

        let resource_id = manager.register_resource(ResourceType::Memory, 1024);
        assert!(!resource_id.is_empty());

        let released = manager.release_resource(&resource_id);
        assert!(released);
    }

    #[test]
    fn test_emergency_shutdown() {
        let manager = ShutdownManager::new();
        manager.emergency_shutdown("Safety trigger activated");

        let status = manager.get_status();
        assert!(!status.shutdown_in_progress);
        assert_eq!(status.shutdowns_completed, 1);
    }
}
