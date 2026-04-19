// Tariro_ide_idpte/src/omega_os/event_models.rs

//! Defines data models for OMEGA OS system events.

use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use uuid::Uuid;
use std::collections::HashMap;

use crate::resource_management::resource_models::{ResourceRequest, AllocationResult};
use crate::compliance_advisor::legal_framework_models::{ComplianceStatus, AuditReport};
use crate::co_creation::creative_models::CreativeArtifact;

/// Unique identifier for an event.
pub type EventId = String;

/// The main OMEGA OS Event enum, encompassing all types of system-wide events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OsEvent {
    // Core OS Events
    ServiceRegistered(String), // service_id
    ServiceStatusChanged(String, String), // service_id, new_status
    ProcessSpawned(String, String), // process_id, process_name
    ProcessTerminated(String, String), // process_id, process_name

    // S.O.R.A.E. Events
    ResourceRequestReceived(ResourceRequest),
    ResourceAllocated(AllocationResult),
    ResourceShortageDetected(HashMap<String, f64>), // resource_type -> severity

    // QR-CTFO Events
    QrKeyRotated(String), // key_id
    SecurityAlert(String), // description
    PqcPolicyEnforced(String), // context

    // HACSE Events
    CreativePromptReceived(String), // prompt_id
    CreativeArtifactGenerated(CreativeArtifact),
    HumanFeedbackReceived(String), // artifact_id
    NewCreativeSuggestion(String), // suggestion_id

    // ALEC-A Events
    ComplianceViolationDetected(ComplianceStatus),
    SystemAudited(AuditReport),
    EthicalAxiomProposed(String), // axiom_id
    LegalRuleUpdated(String), // rule_id

    // General System Events
    SystemInitialized,
    SystemShutdown,
    Custom(String, String), // event_type, payload
}

/// Represents a generic OMEGA OS event structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: EventId,
    pub timestamp: DateTime<Utc>,
    pub source_module: String, // e.g., "SORAEE", "ALEC-A", "ProcessManager"
    pub event: OsEvent,
    pub correlation_id: Option<String>, // For tracing related events
}

impl EventEnvelope {
    pub fn new(source_module: String, event: OsEvent) -> Self {
        EventEnvelope {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            source_module,
            event,
            correlation_id: None,
        }
    }
}
