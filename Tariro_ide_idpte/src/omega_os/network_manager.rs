// Tariro_ide_idpte/src/omega_os/network_manager.rs

//! Manages network interfaces, routing tables, and overall network connectivity within OMEGA OS.
//! This module provides the foundational layer for internal and external communications.

use anyhow::Result;
use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::omega_os::network_models::{NetworkInterface, InterfaceStatus, InterfaceType, RoutingTableEntry, RoutingProtocol};
use crate::omega_os::service_models::{OsService, ServiceType, ServiceCapability, ServiceStatus};
use crate::omega_os::event_bus::EventBus;
use crate::omega_os::event_models::{OsEvent, EventEnvelope};

/// The OMEGA OS Network Manager.
pub struct NetworkManager {
    interfaces: Mutex<HashMap<String, NetworkInterface>>, // id -> NetworkInterface
    routing_table: Mutex<Vec<RoutingTableEntry>>,
    event_bus: Arc<EventBus>,
}

impl NetworkManager {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        info!("OMEGA OS Network Manager initialized.");
        NetworkManager {
            interfaces: Mutex::new(HashMap::new()),
            routing_table: Mutex::new(Vec::new()),
            event_bus,
        }
    }

    /// Configures a new network interface or updates an existing one.
    pub async fn configure_interface(
        &self,
        interface_id: &str,
        name: &str,
        ip_address: &str,
        netmask: &str,
        gateway: &str,
        type_name: InterfaceType,
    ) -> Result<(), String> {
        let mut interfaces_guard = self.interfaces.lock().unwrap();
        let mut interface = interfaces_guard.entry(interface_id.to_string()).or_insert_with(|| NetworkInterface {
            id: interface_id.to_string(),
            name: name.to_string(),
            ip_address: ip_address.to_string(),
            netmask: netmask.to_string(),
            gateway: gateway.to_string(),
            status: InterfaceStatus::Configuring,
            type_name: type_name.clone(),
            metrics: HashMap::new(),
        });

        interface.ip_address = ip_address.to_string();
        interface.netmask = netmask.to_string();
        interface.gateway = gateway.to_string();
        interface.status = InterfaceStatus::Up;
        interface.type_name = type_name;

        info!("NetworkManager: Configured interface '{}' (ID: {}).", name, interface_id);
        self.event_bus.publish(EventEnvelope::new(
            "NetworkManager".to_string(),
            OsEvent::Custom("NetworkInterfaceConfigured".to_string(), format!("{{ \"id\": \"{}\", \"status\": \"Up\" }}", interface_id)),
        )).await;
        Ok(())
    }

    /// Updates an entry in the routing table.
    pub async fn update_routing_table(
        &self,
        destination: &str,
        gateway: &str,
        interface_id: &str,
        metric: u32,
        protocol: RoutingProtocol,
    ) -> Result<(), String> {
        let mut routing_table_guard = self.routing_table.lock().unwrap();
        // Find existing entry or create new one
        if let Some(entry) = routing_table_guard.iter_mut().find(|e| e.destination == destination) {
            entry.gateway = gateway.to_string();
            entry.interface_id = interface_id.to_string();
            entry.metric = metric;
            entry.protocol = protocol.clone();
            info!("NetworkManager: Updated routing table entry for destination '{}'.", destination);
        } else {
            routing_table_guard.push(RoutingTableEntry {
                destination: destination.to_string(),
                gateway: gateway.to_string(),
                interface_id: interface_id.to_string(),
                metric,
                protocol: protocol.clone(),
            });
            info!("NetworkManager: Added routing table entry for destination '{}'.", destination);
        }
        self.event_bus.publish(EventEnvelope::new(
            "NetworkManager".to_string(),
            OsEvent::Custom("RoutingTableUpdated".to_string(), format!("{{ \"destination\": \"{}\", \"protocol\": \"{:?}\" }}", destination, protocol)),
        )).await;
        Ok(())
    }

    /// Retrieves the current status of a network interface.
    pub async fn get_interface_status(&self, interface_id: &str) -> Option<NetworkInterface> {
        self.interfaces.lock().unwrap().get(interface_id).cloned()
    }

    /// Retrieves the current routing table.
    pub async fn get_routing_table(&self) -> Vec<RoutingTableEntry> {
        self.routing_table.lock().unwrap().clone()
    }

    /// Registers the NetworkManager itself as an OMEGA OS service.
    pub fn as_os_service() -> OsService {
        OsService {
            id: "os-network-manager".to_string(),
            name: "OMEGA OS Network Manager".to_string(),
            description: "Manages network interfaces, routing, and basic network connectivity.".to_string(),
            service_type: ServiceType::Networking,
            capabilities: vec![
                ServiceCapability { name: "configure_interface".to_string(), description: "Configures a network interface.".to_string(), parameters: HashMap::new(), output_schema: "".to_string(), requires_permissions: vec!["root".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "update_routing_table".to_string(), description: "Updates network routing table entries.".to_string(), parameters: HashMap::new(), output_schema: "".to_string(), requires_permissions: vec!["root".to_string()], requires_ethical_review: true },
                ServiceCapability { name: "get_network_status".to_string(), description: "Retrieves overall network status.".to_string(), parameters: HashMap::new(), output_schema: "".to_string(), requires_permissions: vec![], requires_ethical_review: false },
            ],
            status: ServiceStatus::Running,
            registered_at: Utc::now(), 
            last_heartbeat: Utc::now(),
            endpoints: HashMap::new(),
            dependencies: vec!["os-event-bus".to_string()],
            managed_by_agi_module: None,
        }
    }
}
