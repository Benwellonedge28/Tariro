/// Module for Fault-Tolerant Data Storage and Recovery in Rust.
///
/// This module conceptually outlines a system for storing data with redundancy
/// and recovering it in the event of node failures, enhancing overall system resilience.

use crate::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rand::Rng;

/// Represents a piece of data to be stored.
#[derive(Debug, Clone, PartialEq)]
pub struct DataRecord {
    pub id: String,
    pub content: String,
    pub version: u644,
}

/// Represents a storage node with its current status.
#[derive(Debug, Clone, PartialEq)]
pub struct StorageNode {
    pub id: String,
    pub is_healthy: bool,
    data: HashMap<String, DataRecord>, // Data stored on this specific node
}

impl StorageNode {
    pub fn new(id: String) -> Self {
        StorageNode { id, is_healthy: true, data: HashMap::new() }
    }

    pub fn store(&mut self, record: DataRecord) {
        self.data.insert(record.id.clone(), record);
    }

    pub fn retrieve(&self, id: &str) -> Option<DataRecord> {
        self.data.get(id).cloned()
    }

    pub fn fail(&mut self) {
        self.is_healthy = false;
        println!("    -> Node {} FAILED.", self.id);
    }

    pub fn restore(&mut self) {
        self.is_healthy = true;
        println!("    -> Node {} RESTORED.", self.id);
    }
}

/// The Fault-Tolerant Storage System.
pub struct RedundantStorage {
    nodes: Arc<RwLock<HashMap<String, StorageNode>>>,
    replication_factor: usize,
}

impl RedundantStorage {
    pub fn new(node_count: usize, replication_factor: usize) -> Self {
        let mut nodes_map = HashMap::new();
        for i in 0..node_count {
            let node_id = format!("node_{}", i);
            nodes_map.insert(node_id.clone(), StorageNode::new(node_id));
        }
        RedundantStorage {
            nodes: Arc::new(RwLock::new(nodes_map)),
            replication_factor,
        }
    }

    /// Simulates writing data to multiple nodes for redundancy.
    pub fn write_data(&self, record: DataRecord) -> Result<(), String> {
        let mut nodes_write_guard = self.nodes.write().unwrap();
        let healthy_nodes: Vec<&mut StorageNode> = nodes_write_guard.values_mut().filter(|n| n.is_healthy).collect();

        if healthy_nodes.len() < self.replication_factor {
            return Err(format!("Not enough healthy nodes for replication. Required: {}, Available: {}",
                               self.replication_factor, healthy_nodes.len()));
        }

        // Distribute copies to 'replication_factor' healthy nodes
        let mut rng = rand::thread_rng();
        let mut chosen_nodes_indices: Vec<usize> = (0..healthy_nodes.len()).collect();
        rng.shuffle(&mut chosen_nodes_indices);

        let mut successful_writes = 0;
        for i in 0..self.replication_factor {
            let node_index = chosen_nodes_indices[i];
            let node = healthy_nodes[node_index];
            node.store(record.clone());
            println!("  [Storage]: Data '{}' written to node {}.", record.id, node.id);
            successful_writes += 1;
        }

        if successful_writes < self.replication_factor {
            Err(format!("Could only write to {} out of {} required nodes.", successful_writes, self.replication_factor))
        } else {
            Ok(())
        }
    }

    /// Simulates reading data, trying multiple nodes if necessary.
    pub fn read_data(&self, id: &str) -> Result<DataRecord, String> {
        let nodes_read_guard = self.nodes.read().unwrap();
        for node in nodes_read_guard.values() {
            if node.is_healthy {
                if let Some(record) = node.retrieve(id) {
                    println!("  [Storage]: Data '{}' read from node {}.", id, node.id);
                    return Ok(record);
                }
            }
        }
        Err(format!("Data '{}' not found or all replicas unhealthy.", id))
    }

    /// Simulates recovering data after a node failure by reconstructing from healthy replicas.
    pub fn recover_data(&self, id: &str) -> Result<DataRecord, String> {
        let nodes_read_guard = self.nodes.read().unwrap();
        let mut healthy_replicas = Vec::new();

        for node in nodes_read_guard.values() {
            if node.is_healthy {
                if let Some(record) = node.retrieve(id) {
                    healthy_replicas.push(record);
                }
            }
        }

        if healthy_replicas.is_empty() {
            Err(format!("Cannot recover data '{}': no healthy replicas found.", id))
        } else {
            // In a real system, this would involve consistency checks (e.g., quorum read)
            // For simplicity, we just return the first found healthy replica.
            println!("  [Storage]: Data '{}' successfully recovered from healthy replicas.", id);
            Ok(healthy_replicas[0].clone())
        }
    }

    /// Simulates a node failing.
    pub fn fail_node(&self, node_id: &str) {
        let mut nodes_write_guard = self.nodes.write().unwrap();
        if let Some(node) = nodes_write_guard.get_mut(node_id) {
            node.fail();
        }
    }

    /// Simulates a node restoring.
    pub fn restore_node(&self, node_id: &str) {
        let mut nodes_write_guard = self.nodes.write().unwrap();
        if let Some(node) = nodes_write_guard.get_mut(node_id) {
            node.restore();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read_data() {
        let storage = RedundantStorage::new(3, 2);
        let record = DataRecord { id: "test_data_1".to_string(), content: "Hello World".to_string(), version: 1 };
        assert!(storage.write_data(record.clone()).is_ok());
        let read_record = storage.read_data("test_data_1").unwrap();
        assert_eq!(read_record, record);
    }

    #[test]
    fn test_read_after_node_failure() {
        let storage = RedundantStorage::new(3, 2);
        let record = DataRecord { id: "test_data_2".to_string(), content: "Important Info".to_string(), version: 1 };
        storage.write_data(record.clone()).unwrap();

        storage.fail_node("node_0");
        let read_record = storage.read_data("test_data_2").unwrap();
        assert_eq!(read_record, record);
    }

    #[test]
    fn test_recovery_from_failure() {
        let storage = RedundantStorage::new(3, 2);
        let record = DataRecord { id: "test_data_3".to_string(), content: "Recoverable Data".to_string(), version: 1 };
        storage.write_data(record.clone()).unwrap();

        storage.fail_node("node_0");
        storage.fail_node("node_1"); // Simulate two nodes failing

        // Now, we can't read directly, but we can recover if enough replicas exist
        assert!(storage.read_data("test_data_3").is_ok()); // Should still be able to read from node_2
        let recovered_record = storage.recover_data("test_data_3").unwrap();
        assert_eq!(recovered_record, record);

        storage.fail_node("node_2"); // All nodes failed
        assert!(storage.read_data("test_data_3").is_err());
        assert!(storage.recover_data("test_data_3").is_err());
    }

    #[test]
    fn test_not_enough_healthy_nodes_for_write() {
        let storage = RedundantStorage::new(2, 2); // 2 nodes, replication factor 2
        storage.fail_node("node_0");
        let record = DataRecord { id: "test_data_4".to_string(), content: "Insufficient nodes".to_string(), version: 1 };
        assert!(storage.write_data(record).is_err());
    }
}

fn main() {
    println!("\n--- Fault-Tolerant Data Storage and Recovery Demonstration ---");

    // Setup a storage system with 3 nodes and a replication factor of 2
    let storage_system = RedundantStorage::new(3, 2);
    println!("Storage system initialized with 3 nodes, replication factor 2.");

    // 1. Write some data
    let record1 = DataRecord { id: "user_config_123".to_string(), content: "{{\"theme\":\"dark\"}}".to_string(), version: 1 };
    let record2 = DataRecord { id: "system_log_456".to_string(), content: "[INFO] System startup complete.".to_string(), version: 1 };
    let record3 = DataRecord { id: "critical_asset_789".to_string(), content: "ENCRYPTED_KEY:xyz123abc".to_string(), version: 1 };

    println!("\nWriting data...");
    storage_system.write_data(record1.clone()).unwrap();
    storage_system.write_data(record2.clone()).unwrap();
    storage_system.write_data(record3.clone()).unwrap();

    // 2. Read data (should work fine)
    println!("\nReading data...");
    let read_data1 = storage_system.read_data("user_config_123").unwrap();
    println!("  Retrieved: {:?}", read_data1);

    // 3. Simulate a node failure
    println!("\nSimulating failure of node_0...");
    storage_system.fail_node("node_0");
    thread::sleep(Duration::from_millis(500));

    // 4. Try reading data again (should still work due to replication)
    println!("\nAttempting to read data after node_0 failure...");
    let read_data2 = storage_system.read_data("system_log_456").unwrap();
    println!("  Retrieved: {:?}", read_data2);

    // 5. Simulate another node failure (e.g., node_1)
    println!("\nSimulating failure of node_1...");
    storage_system.fail_node("node_1");
    thread::sleep(Duration::from_millis(500));

    // 6. Try reading data (should still work if replication factor is 2, and node_2 is healthy)
    println!("\nAttempting to read data after node_1 failure...");
    let read_data3 = storage_system.read_data("critical_asset_789").unwrap();
    println!("  Retrieved: {:?}", read_data3);

    // 7. Try recovering data (which internally reads from healthy replicas)
    println!("\nAttempting to recover data (from any healthy replica)...");
    let recovered_data = storage_system.recover_data("user_config_123").unwrap();
    println!("  Recovered: {:?}", recovered_data);

    // 8. Restore a node
    println!("\nRestoring node_0...");
    storage_system.restore_node("node_0");
    thread::sleep(Duration::from_millis(500));

    println!("\nFault-Tolerant Data Storage and Recovery Demonstration Complete.");
}
