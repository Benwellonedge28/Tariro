// Tariro_ide_idpte/src/omega_os/event_bus.rs

//! Implements a central, asynchronous event bus for OMEGA OS.
//! This allows decoupled communication between services and modules, facilitating
//! dynamic composition and robust system behavior.

use tokio::sync::broadcast;
use tracing::{info, warn, error};

use crate::omega_os::event_models::{EventEnvelope, OsEvent};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};

/// The central OMEGA OS Event Bus.
pub struct EventBus {
    sender: broadcast::Sender<EventEnvelope>,
}

impl EventBus {
    /// Creates a new EventBus with a specified channel capacity.
    pub fn new(capacity: usize) -> Self {
        let (sender, _receiver) = broadcast::channel(capacity);
        info!("OMEGA OS Event Bus initialized with capacity {}.", capacity);
        EventBus { sender }
    }

    /// Publishes an event to the bus. Subscribers will receive it asynchronously.
    pub async fn publish(&self, event_envelope: EventEnvelope) {
        if let Err(e) = self.sender.send(event_envelope.clone()) {
            error!("EventBus: Failed to publish event {:?} from {}: {}", event_envelope.event, event_envelope.source_module, e);
        } else {
            info!("EventBus: Published event {:?} from {}.
", event_envelope.event, event_envelope.source_module);
        }
    }

    /// Subscribes to the event bus. Returns a receiver that will receive all future events.
    pub fn subscribe(&self) -> broadcast::Receiver<EventEnvelope> {
        self.sender.subscribe()
    }

    /// Registers the EventBus itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "os-event-bus".to_string(),
            name: "OMEGA OS Event Bus".to_string(),
            description: "Central asynchronous communication backbone for OMEGA OS services.".to_string(),
            service_type: ServiceType::SystemCore,
            capabilities: vec![
                ServiceCapability { name: "publish_event".to_string(), description: "Publishes an event to the bus.".to_string(), parameters: HashMap::new(), output_schema: "".to_string(), requires_permissions: vec![], requires_ethical_review: false },
                ServiceCapability { name: "subscribe_events".to_string(), description: "Subscribes to events on the bus.".to_string(), parameters: HashMap::new(), output_schema: "EventEnvelope".to_string(), requires_permissions: vec![], requires_ethical_review: false },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), // This will be updated upon actual registration
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec![],
            managed_by_agi_module: None,
        }
    }
}
