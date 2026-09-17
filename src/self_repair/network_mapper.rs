use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents a network node or endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: String,
    pub name: String,
    pub node_type: String, // e.g., "Server", "Gateway", "ServiceEndpoint", "ExternalAPI"
    pub ip_address: String,
    pub location: String, // e.g., "DataCenter-A", "Cloud-Region-B", "External"
    pub current_status: String, // "Up", "Down", "Degraded"
    pub latency_ms: Option<u32>,
    pub packet_loss_pct: Option<f64>,
}

// Represents a connection between two network nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub connection_type: String, // e.g., "InternalLAN", "WAN", "Internet", "VPN"
    pub bandwidth_mbps: u32,
    pub current_latency_ms: Option<u32>,
    pub current_packet_loss_pct: Option<f64>,
    pub current_status: String, // "Stable", "Congested", "Intermittent", "Down"
    pub resilience_score: f64, // 0.0 - 1.0 (higher is better)
}

// Represents the overall network resilience map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMap {
    pub map_id: String,
    pub timestamp: DateTime<Utc>,
    pub nodes: HashMap<String, NetworkNode>, // key: node_id
    pub connections: HashMap<String, NetworkConnection>, // key: connection_id
    pub overall_resilience_score: f64, // Aggregated score
    pub potential_failure_points: Vec<String>, // List of node/connection IDs
}

pub struct NetworkResilienceMapper {
    current_map: Mutex<Option<NetworkMap>>,
}

impl NetworkResilienceMapper {
    pub fn new() -> Self {
        NetworkResilienceMapper {
            current_map: Mutex::new(None),
        }
    }

    // Updates the network map with latest data
    pub fn update_network_map(&self, new_nodes: Vec<NetworkNode>, new_connections: Vec<NetworkConnection>) {
        println!("Updating real-time network map...");

        let mut nodes_map = HashMap::new();
        for node in new_nodes {
            nodes_map.insert(node.id.clone(), node);
        }

        let mut connections_map = HashMap::new();
        let mut overall_resilience_score = 0.0;
        let mut potential_failure_points = Vec::new();
        let mut connection_count = 0;

        for mut conn in new_connections {
            // Calculate resilience score for connection (simplified)
            conn.resilience_score = 1.0;
            if conn.current_status != "Stable" {
                conn.resilience_score -= 0.3;
            }
            if let Some(loss) = conn.current_packet_loss_pct {
                conn.resilience_score -= loss;
            }
            if let Some(latency) = conn.current_latency_ms {
                if latency > 100 { conn.resilience_score -= 0.2; }
            }
            conn.resilience_score = conn.resilience_score.max(0.0); // Ensure score is not negative

            overall_resilience_score += conn.resilience_score;
            connection_count += 1;

            if conn.resilience_score < 0.7 || conn.current_status != "Stable" {
                potential_failure_points.push(format!("Connection: {}", conn.id));
            }

            connections_map.insert(conn.id.clone(), conn);
        }

        // Identify potential failure points from nodes
        for node in nodes_map.values() {
            if node.current_status == "Down" || node.current_status == "Degraded" {
                potential_failure_points.push(format!("Node: {}", node.id));
            }
        }

        let avg_resilience = if connection_count > 0 { overall_resilience_score / connection_count as f64 } else { 1.0 };

        let new_map = NetworkMap {
            map_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            nodes: nodes_map,
            connections: connections_map,
            overall_resilience_score: avg_resilience,
            potential_failure_points,
        };
        *self.current_map.lock().unwrap() = Some(new_map);
        println!("Network map updated. Overall resilience: {:.2}", avg_resilience);
    }

    // Retrieves the current network map
    pub fn get_current_network_map(&self) -> Option<NetworkMap> {
        self.current_map.lock().unwrap().clone()
    }

    // Visualizes the network map (simplified by returning a descriptive string)
    pub fn visualize_map(&self) -> String {
        if let Some(map) = self.current_map.lock().unwrap().as_ref() {
            let mut viz = String::new();
            viz.push_str(&format!("--- Network Resilience Map (Last Updated: {}) ---\n", map.timestamp.to_rfc3339()));
            viz.push_str(&format!("Overall Resilience Score: {:.2}\n", map.overall_resilience_score));
            viz.push_str("Nodes:\n");
            for node in map.nodes.values() {
                viz.push_str(&format!("  - {}: {} ({}, {})\n", node.name, node.current_status, node.ip_address, node.location));
            }
            viz.push_str("Connections:\n");
            for conn in map.connections.values() {
                let source_name = map.nodes.get(&conn.source_node_id).map_or("Unknown", |n| &n.name);
                let target_name = map.nodes.get(&conn.target_node_id).map_or("Unknown", |n| &n.name);
                viz.push_str(&format!("  - {} <-> {}: {} ({}, {:.2} resilience)\n", source_name, target_name, conn.current_status, conn.connection_type, conn.resilience_score));
            }
            if !map.potential_failure_points.is_empty() {
                viz.push_str("Potential Failure Points:\n");
                for pfp in &map.potential_failure_points {
                    viz.push_str(&format!("  - {}\n", pfp));
                }
            } else {
                viz.push_str("No critical potential failure points detected.\n");
            }
            viz.push_str("--------------------------------------------------\n");
            viz
        } else {
            "Network map not yet initialized.".to_string()
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_network_resilience_mapper() {
        let mapper = NetworkResilienceMapper::new();

        let node1 = NetworkNode {
            id: "node-1".to_string(),
            name: "CoreServer".to_string(),
            node_type: "Server".to_string(),
            ip_address: "10.0.0.1".to_string(),
            location: "DataCenter-A".to_string(),
            current_status: "Up".to_string(),
            latency_ms: Some(5),
            packet_loss_pct: Some(0.0),
        };
        let node2 = NetworkNode {
            id: "node-2".to_string(),
            name: "ExternalAPI".to_string(),
            node_type: "ExternalAPI".to_string(),
            ip_address: "203.0.113.1".to_string(),
            location: "External".to_string(),
            current_status: "Up".to_string(),
            latency_ms: Some(50),
            packet_loss_pct: Some(0.01),
        };
        let node3 = NetworkNode {
            id: "node-3".to_string(),
            name: "EdgeGateway".to_string(),
            node_type: "Gateway".to_string(),
            ip_address: "192.168.1.1".to_string(),
            location: "DataCenter-A".to_string(),
            current_status: "Degraded".to_string(), // Degraded node
            latency_ms: Some(150),
            packet_loss_pct: Some(0.05),
        };

        let conn1 = NetworkConnection {
            id: "conn-1".to_string(),
            source_node_id: "node-1".to_string(),
            target_node_id: "node-3".to_string(),
            connection_type: "InternalLAN".to_string(),
            bandwidth_mbps: 1000,
            current_latency_ms: Some(10),
            current_packet_loss_pct: Some(0.0),
            current_status: "Stable".to_string(),
            resilience_score: 0.0, // Will be calculated
        };
        let conn2 = NetworkConnection {
            id: "conn-2".to_string(),
            source_node_id: "node-3".to_string(),
            target_node_id: "node-2".to_string(),
            connection_type: "WAN".to_string(),
            bandwidth_mbps: 100,
            current_latency_ms: Some(120), // High latency
            current_packet_loss_pct: Some(0.02),
            current_status: "Congested".to_string(), // Congested connection
            resilience_score: 0.0, // Will be calculated
        };

        mapper.update_network_map(vec![node1, node2, node3], vec![conn1, conn2]);

        let current_map = mapper.get_current_network_map().unwrap();
        println!("{}", mapper.visualize_map());

        assert!(current_map.overall_resilience_score < 1.0); // Should be less than optimal due to issues
        assert!(!current_map.potential_failure_points.is_empty());
        assert!(current_map.potential_failure_points.contains(&"Node: node-3".to_string()));
        assert!(current_map.potential_failure_points.iter().any(|p| p.contains("Connection: conn-2")));
    }
}
*/