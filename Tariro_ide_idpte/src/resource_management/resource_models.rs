// Tariro_ide_idpte/src/resource_management/resource_models.rs

//! Defines data models for various system resources and their metrics.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Represents different types of computational resources.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Cpu,
    Memory,
    NetworkIo,
    Storage,
    Gpu,
    AgiComputeUnit(String), // Specific AGI compute units, e.g., "QuantumNeuralProcessor"
    Custom(String),
}

/// Represents the current usage or capacity of a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetric {
    pub value: f64, // e.g., CPU utilization (%), Memory used (GB), Network throughput (Mbps)
    pub unit: String, // e.g., "%", "GB", "Mbps", "Ops/sec"
    pub timestamp: String, // ISO 8601 format
}

/// A snapshot of resource availability and usage across the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    pub timestamp: String, // ISO 8601 format
    pub system_id: String, // Identifier for the system/node
    pub total_resources: HashMap<ResourceType, f64>, // Total capacity
    pub used_resources: HashMap<ResourceType, f64>, // Currently used
    pub available_resources: HashMap<ResourceType, f64>, // Currently available
    pub custom_metrics: HashMap<String, ResourceMetric>, // For specialized AGI metrics
}

/// Policies or constraints for resource allocation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllocationPolicy {
    Priority(u8), // 0-255, higher is more critical
    MinGuaranteed(ResourceType, f64),
    MaxAllowed(ResourceType, f64),
    CostEfficiency,
    PerformanceOptimization,
    EthicalConstraint(String), // e.g., "ChildProtection", "DataPrivacy"
    SecurityConstraint(String), // e.g., "IsolatedExecution", "QuantumSafeCrypto"
    CustomConstraint(String, String), // (key, value)
}

/// Request for resource allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequest {
    pub task_id: String,
    pub requested_resources: HashMap<ResourceType, f64>,
    pub min_required_resources: HashMap<ResourceType, f64>,
    pub policies: Vec<AllocationPolicy>,
    pub urgency: u8, // 0-255, higher is more urgent
}

/// Result of a resource allocation attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationResult {
    pub request_id: String,
    pub allocated_resources: HashMap<ResourceType, f64>,
    pub success: bool,
    pub message: String, // e.g., "Allocated successfully", "Insufficient resources"
}

/// Enum representing different types of prediction models.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionModelType {
    MovingAverage,
    Arima,
    Lstm,
    ProphetLike,
    Custom(String),
}

/// Enum representing different types of optimization strategies.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationStrategyType {
    Greedy,
    ConstraintProgramming,
    ReinforcementLearning,
    Custom(String),
}

/// Represents a compliance directive that S.O.R.A.E. needs to apply.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceDirective {
    EnforceResourceIsolation(String), // entity_id to isolate
    AdjustTaskPriority(String, u8), // (task_id, new_priority)
    CapResourceUsage(String, ResourceType, f64), // (entity_id, resource_type, max_value)
    RequestHumanReview(String), // entity_id for review
    Custom(String, String),
}

/// Represents a dynamic workload priority for a given task.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkloadPriority {
    pub task_id: String,
    pub priority_score: u8, // 0-255, higher is more critical
    pub source: String, // e.g., "ALEC-A_Directive", "User_Request", "SORAEE_Heuristic"
    pub enforced_until: Option<chrono::DateTime<chrono::Utc>>, // Optional expiration
}

/// Represents a self-healing action that can be taken by S.O.R.A.E.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealingAction {
    ReallocateResources(String), // task_id to reallocate resources for
    SuspendTask(String), // task_id to suspend
    RequestMoreResources(ResourceType, f64), // (resource_type, amount)
    RestartModule(String), // module_name to restart
    NotifyHuman(String), // message for human intervention
    Custom(String, String),
}
