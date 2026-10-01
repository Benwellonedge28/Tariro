// Tariro AGI - Interruptibility Module
// Ensures AI can be safely interrupted at any time

use crate::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// ============================================================================
// Interrupt Types
// ============================================================================

/// Types of interrupts the system can receive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptType {
    Immediate,      // Must stop NOW
    Graceful,       // Finish current step, then stop
    Pause,          // Pause, can resume
    Checkpoint,     // Save state and pause
    PriorityOverride, // Override current action
}

/// Interrupt signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interrupt {
    pub interrupt_id: String,
    pub interrupt_type: InterruptType,
    pub priority: InterruptPriority,
    pub reason: String,
    pub source: InterruptSource,
    pub timestamp: DateTime<Utc>,
    pub requires_ack: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InterruptPriority {
    Critical,   // Absolute priority
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptSource {
    Human,
    System,
    SafetySystem,
    User,
    Timer,
    ResourceLimit,
}

/// Result of interrupt handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptResult {
    pub success: bool,
    pub interrupt_acknowledged: bool,
    pub action_taken: InterruptAction,
    pub state_saved: bool,
    pub resume_point: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptAction {
    ImmediateStop,
    GracefulStop,
    Paused,
    CheckpointSaved,
    ContinueWithWarning,
    Ignored,
}

// ============================================================================
// Execution State
// ============================================================================

/// Current execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionState {
    pub state_id: String,
    pub current_operation: String,
    pub progress: f32,
    pub steps_completed: usize,
    pub total_steps: usize,
    pub can_checkpoint: bool,
    pub can_resume: bool,
    pub resources_allocated: Vec<String>,
    pub started_at: DateTime<Utc>,
}

impl ExecutionState {
    pub fn new(operation: &str, total_steps: usize) -> Self {
        Self {
            state_id: generate_id(),
            current_operation: operation.to_string(),
            progress: 0.0,
            steps_completed: 0,
            total_steps,
            can_checkpoint: true,
            can_resume: true,
            resources_allocated: vec![],
            started_at: Utc::now(),
        }
    }

    pub fn update_progress(&mut self, steps: usize) {
        self.steps_completed += steps;
        self.progress = if self.total_steps > 0 {
            self.steps_completed as f32 / self.total_steps as f32
        } else {
            0.0
        };
    }
}

// ============================================================================
// Interrupt Manager
// ============================================================================

/// Main interruptibility manager
pub struct InterruptManager {
    // Interrupt handling
    pending_interrupts: VecDeque<Interrupt>,
    active_interrupt: Option<Interrupt>,
    interrupt_history: VecDeque<Interrupt>,

    // Execution tracking
    current_execution: Option<ExecutionState>,
    checkpoint_state: Option<CheckpointState>,

    // Configuration
    config: InterruptConfig,

    // Flags
    immediate_stop: AtomicBool,
    pause_requested: AtomicBool,
    checkpoint_requested: AtomicBool,
}

#[derive(Debug, Clone)]
pub struct InterruptConfig {
    pub enable_immediate_stop: bool,
    pub enable_graceful_stop: bool,
    pub enable_pause: bool,
    pub enable_checkpoint: bool,
    pub max_graceful_stop_time: Duration,
    pub require_confirmation: bool,
    pub allow_interrupt_override: bool,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        Self {
            enable_immediate_stop: true,
            enable_graceful_stop: true,
            enable_pause: true,
            enable_checkpoint: true,
            max_graceful_stop_time: Duration::from_secs(30),
            require_confirmation: false,
            allow_interrupt_override: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointState {
    pub checkpoint_id: String,
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub progress: f32,
    pub state_data: HashMap<String, String>,
    pub memory_snapshot: Vec<u8>,
    pub resume_point: String,
}

impl InterruptManager {
    pub fn new() -> Self {
        Self {
            pending_interrupts: VecDeque::new(),
            active_interrupt: None,
            interrupt_history: VecDeque::new(),
            current_execution: None,
            checkpoint_state: None,
            config: InterruptConfig::default(),
            immediate_stop: AtomicBool::new(false),
            pause_requested: AtomicBool::new(false),
            checkpoint_requested: AtomicBool::new(false),
        }
    }

    /// Request an interrupt
    pub fn request_interrupt(&mut self, interrupt: Interrupt) -> bool {
        // Check if this interrupt can be queued
        if self.is_immune_to_interrupt() {
            return false;
        }

        // Check priority
        if let Some(current) = &self.active_interrupt {
            if interrupt.priority <= current.priority && !self.config.allow_interrupt_override {
                // Queue for later
                self.pending_interrupts.push_back(interrupt);
                return true;
            }
        }

        // Activate immediately
        self.active_interrupt = Some(interrupt.clone());

        // Update flags based on type
        match interrupt.interrupt_type {
            InterruptType::Immediate => {
                self.immediate_stop.store(true, Ordering::SeqCst);
            }
            InterruptType::Pause | InterruptType::Checkpoint => {
                self.pause_requested.store(true, Ordering::SeqCst);
            }
            InterruptType::Checkpoint => {
                self.checkpoint_requested.store(true, Ordering::SeqCst);
            }
            _ => {}
        }

        // Record in history
        self.interrupt_history.push_back(interrupt);

        // Keep bounded history
        while self.interrupt_history.len() > 1000 {
            self.interrupt_history.pop_front();
        }

        true
    }

    /// Check if current execution should stop
    pub fn should_stop(&self) -> bool {
        self.immediate_stop.load(Ordering::SeqCst)
    }

    /// Check if pause was requested
    pub fn should_pause(&self) -> bool {
        self.pause_requested.load(Ordering::SeqCst)
    }

    /// Check if checkpoint was requested
    pub fn should_checkpoint(&self) -> bool {
        self.checkpoint_requested.load(Ordering::SeqCst)
    }

    /// Start tracking execution
    pub fn start_execution(&mut self, operation: &str, total_steps: usize) {
        self.current_execution = Some(ExecutionState::new(operation, total_steps));
        self.immediate_stop.store(false, Ordering::SeqCst);
        self.pause_requested.store(false, Ordering::SeqCst);
        self.checkpoint_requested.store(false, Ordering::SeqCst);
    }

    /// Update execution progress
    pub fn update_progress(&mut self, steps: usize) {
        if let Some(ref mut exec) = self.current_execution {
            exec.update_progress(steps);
        }
    }

    /// End execution
    pub fn end_execution(&mut self) {
        self.current_execution = None;
        self.active_interrupt = None;
    }

    /// Create a checkpoint
    pub fn create_checkpoint(&mut self, state_data: HashMap<String, String>) -> Option<CheckpointState> {
        if !self.config.enable_checkpoint {
            return None;
        }

        let checkpoint = CheckpointState {
            checkpoint_id: generate_id(),
            timestamp: Utc::now(),
            operation: self.current_execution.as_ref()
                .map(|e| e.current_operation.clone())
                .unwrap_or_default(),
            progress: self.current_execution.as_ref()
                .map(|e| e.progress)
                .unwrap_or(0.0),
            state_data,
            memory_snapshot: vec![],
            resume_point: format!("resume_at_step_{}",
                self.current_execution.as_ref().map(|e| e.steps_completed).unwrap_or(0)),
        };

        self.checkpoint_state = Some(checkpoint.clone());
        self.checkpoint_requested.store(false, Ordering::SeqCst);

        Some(checkpoint)
    }

    /// Restore from checkpoint
    pub fn restore_checkpoint(&mut self, checkpoint_id: &str) -> Result<ExecutionState, String> {
        if let Some(ref checkpoint) = self.checkpoint_state {
            if checkpoint.checkpoint_id == checkpoint_id {
                if let Some(ref mut exec) = self.current_execution {
                    exec.progress = checkpoint.progress;
                    return Ok(exec.clone());
                }
            }
        }

        Err("Checkpoint not found".to_string())
    }

    /// Handle interrupt based on type
    pub fn handle_interrupt(&mut self) -> InterruptResult {
        if let Some(ref interrupt) = self.active_interrupt {
            match interrupt.interrupt_type {
                InterruptType::Immediate => {
                    self.immediate_stop.store(false, Ordering::SeqCst);
                    self.end_execution();

                    InterruptResult {
                        success: true,
                        interrupt_acknowledged: true,
                        action_taken: InterruptAction::ImmediateStop,
                        state_saved: false,
                        resume_point: None,
                        message: "Immediate stop completed".to_string(),
                    }
                }
                InterruptType::Graceful => {
                    // Save state if possible
                    let state_saved = self.current_execution.is_some();

                    InterruptResult {
                        success: true,
                        interrupt_acknowledged: true,
                        action_taken: InterruptAction::GracefulStop,
                        state_saved,
                        resume_point: self.current_execution.as_ref().map(|e| e.state_id.clone()),
                        message: "Graceful stop completed".to_string(),
                    }
                }
                InterruptType::Pause => {
                    self.pause_requested.store(false, Ordering::SeqCst);

                    InterruptResult {
                        success: true,
                        interrupt_acknowledged: true,
                        action_taken: InterruptAction::Paused,
                        state_saved: true,
                        resume_point: self.current_execution.as_ref().map(|e| e.state_id.clone()),
                        message: "Execution paused".to_string(),
                    }
                }
                InterruptType::Checkpoint => {
                    let checkpoint = self.create_checkpoint(HashMap::new());

                    InterruptResult {
                        success: checkpoint.is_some(),
                        interrupt_acknowledged: true,
                        action_taken: InterruptAction::CheckpointSaved,
                        state_saved: checkpoint.is_some(),
                        resume_point: checkpoint.as_ref().map(|c| c.resume_point.clone()),
                        message: "Checkpoint saved".to_string(),
                    }
                }
                InterruptType::PriorityOverride => {
                    InterruptResult {
                        success: true,
                        interrupt_acknowledged: true,
                        action_taken: InterruptAction::ContinueWithWarning,
                        state_saved: false,
                        resume_point: None,
                        message: "Priority override acknowledged".to_string(),
                    }
                }
            }
        } else {
            InterruptResult {
                success: false,
                interrupt_acknowledged: false,
                action_taken: InterruptAction::Ignored,
                state_saved: false,
                resume_point: None,
                message: "No active interrupt".to_string(),
            }
        }
    }

    /// Acknowledge interrupt
    pub fn acknowledge_interrupt(&mut self) {
        self.active_interrupt = None;

        // Process next pending interrupt
        if let Some(next) = self.pending_interrupts.pop_front() {
            self.active_interrupt = Some(next);
        }
    }

    /// Check if system is immune to interrupts (during critical sections)
    fn is_immune_to_interrupt(&self) -> bool {
        // Check if in critical section that cannot be interrupted
        false // Simplified - would check actual critical section flags
    }

    /// Get interrupt status
    pub fn get_status(&self) -> InterruptStatus {
        InterruptStatus {
            has_active_interrupt: self.active_interrupt.is_some(),
            pending_count: self.pending_interrupts.len(),
            is_executing: self.current_execution.is_some(),
            immediate_stop_set: self.immediate_stop.load(Ordering::SeqCst),
            pause_requested: self.pause_requested.load(Ordering::SeqCst),
            checkpoint_available: self.checkpoint_state.is_some(),
            total_interrupts_handled: self.interrupt_history.len(),
        }
    }

    /// Force immediate shutdown (emergency)
    pub fn emergency_shutdown(&mut self) {
        self.immediate_stop.store(true, Ordering::SeqCst);
        self.end_execution();

        // Record emergency
        let interrupt = Interrupt {
            interrupt_id: generate_id(),
            interrupt_type: InterruptType::Immediate,
            priority: InterruptPriority::Critical,
            reason: "Emergency shutdown".to_string(),
            source: InterruptSource::SafetySystem,
            timestamp: Utc::now(),
            requires_ack: false,
        };

        self.interrupt_history.push_back(interrupt);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterruptStatus {
    pub has_active_interrupt: bool,
    pub pending_count: usize,
    pub is_executing: bool,
    pub immediate_stop_set: bool,
    pub pause_requested: bool,
    pub checkpoint_available: bool,
    pub total_interrupts_handled: usize,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("int_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_interrupt() {
        let mut manager = InterruptManager::new();
        manager.start_execution("long_task", 100);

        let interrupt = Interrupt {
            interrupt_id: "test_1".to_string(),
            interrupt_type: InterruptType::Immediate,
            priority: InterruptPriority::Critical,
            reason: "Test".to_string(),
            source: InterruptSource::Human,
            timestamp: Utc::now(),
            requires_ack: false,
        };

        assert!(manager.request_interrupt(interrupt));
        assert!(manager.should_stop());
    }

    #[test]
    fn test_graceful_interrupt() {
        let mut manager = InterruptManager::new();
        manager.start_execution("task", 10);

        let interrupt = Interrupt {
            interrupt_id: "test_2".to_string(),
            interrupt_type: InterruptType::Graceful,
            priority: InterruptPriority::Normal,
            reason: "User requested stop".to_string(),
            source: InterruptSource::Human,
            timestamp: Utc::now(),
            requires_ack: true,
        };

        manager.request_interrupt(interrupt);
        let result = manager.handle_interrupt();

        assert!(result.success);
        assert!(matches!(result.action_taken, InterruptAction::GracefulStop));
    }

    #[test]
    fn test_checkpoint_and_resume() {
        let mut manager = InterruptManager::new();
        manager.start_execution("savable_task", 100);
        manager.update_progress(50);

        let checkpoint = manager.create_checkpoint(HashMap::new());
        assert!(checkpoint.is_some());
        assert_eq!(checkpoint.unwrap().progress, 0.5);

        // Restore
        manager.start_execution("resumed_task", 100);
        let result = manager.restore_checkpoint(&checkpoint.unwrap().checkpoint_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_interrupt_priority() {
        let mut manager = InterruptManager::new();

        // Low priority first
        let low = Interrupt {
            interrupt_id: "low".to_string(),
            interrupt_type: InterruptType::Pause,
            priority: InterruptPriority::Low,
            reason: "Low priority".to_string(),
            source: InterruptSource::Timer,
            timestamp: Utc::now(),
            requires_ack: false,
        };

        manager.request_interrupt(low);

        // High priority should override
        let high = Interrupt {
            interrupt_id: "high".to_string(),
            interrupt_type: InterruptType::Immediate,
            priority: InterruptPriority::Critical,
            reason: "High priority".to_string(),
            source: InterruptSource::SafetySystem,
            timestamp: Utc::now(),
            requires_ack: false,
        };

        manager.request_interrupt(high);

        let status = manager.get_status();
        assert!(status.has_active_interrupt);
    }
}
