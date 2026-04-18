//! Gathers real-time system resource usage data.

use crate::resource_management::resource_models::{ResourceMetric, ResourceSnapshot, ResourceType};
use std::collections::HashMap;
use chrono::Utc; // For timestamps
use num_cpus; // For getting number of CPU cores
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt};
use std::sync::{Arc, Mutex};

/// ResourceMonitor is responsible for observing and collecting system resource data.
pub struct ResourceMonitor {
    system_id: String,
    sys: Arc<Mutex<System>>, // sysinfo System struct for OS-level info
}

impl ResourceMonitor {
    pub fn new(system_id: String) -> Self {
        println!("ResourceMonitor initialized for system: {}", system_id);
        ResourceMonitor {
            system_id,
            sys: Arc::new(Mutex::new(System::new_with_specifics(sysinfo::RefreshKind::new()))),
        }
    }

    /// Gathers low-level, OS-specific resource metrics using sysinfo.
    fn get_os_level_metrics(&self) -> (HashMap<ResourceType, f64>, HashMap<ResourceType, f64>, HashMap<ResourceType, f64>) {
        let mut sys = self.sys.lock().unwrap();
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_disks();
        sys.refresh_networks();

        let mut total_resources = HashMap::new();
        let mut used_resources = HashMap::new();
        let mut available_resources = HashMap::new();

        // CPU
        let total_cpus = num_cpus::get() as f64;
        let cpu_usage_percentage = sys.global_cpu_info().cpu_usage() as f64;
        // sysinfo returns percentage, convert to equivalent cores used
        let cpu_used_cores = total_cpus * (cpu_usage_percentage / 100.0);

        total_resources.insert(ResourceType::Cpu, total_cpus);
        used_resources.insert(ResourceType::Cpu, cpu_used_cores);
        available_resources.insert(ResourceType::Cpu, total_cpus - cpu_used_cores);

        // Memory (bytes to GB)
        let total_memory_gb = sys.total_memory() as f64 / 1_073_741_824.0; // Bytes to GB
        let used_memory_gb = sys.used_memory() as f64 / 1_073_741_824.0; // Bytes to GB
        let available_memory_gb = sys.available_memory() as f64 / 1_073_741_824.0; // Bytes to GB

        total_resources.insert(ResourceType::Memory, total_memory_gb);
        used_resources.insert(ResourceType::Memory, used_memory_gb);
        available_resources.insert(ResourceType::Memory, available_memory_gb);

        // Network I/O (simple total from all interfaces, bytes/sec to Mbps)
        let mut total_incoming_bytes = 0;
        let mut total_outgoing_bytes = 0;
        for (_interface_name, network) in sys.networks() {
            total_incoming_bytes += network.received();
            total_outgoing_bytes += network.transmitted();
        }
        // This is cumulative. To get a rate, we would need to store previous values and calculate delta.
        // For simplicity, let's use a mock dynamic value for bandwidth usage in Mbps for now.
        let network_usage_mbps = (total_incoming_bytes + total_outgoing_bytes) as f64 / 1_000_000.0 * 8.0; // rough conversion to Mbps

        let total_net_capacity = 1000.0; // Assuming 1 Gbps link for now (1000 Mbps)
        total_resources.insert(ResourceType::NetworkIo, total_net_capacity);
        used_resources.insert(ResourceType::NetworkIo, network_usage_mbps.min(total_net_capacity)); // Cap at total capacity
        available_resources.insert(ResourceType::NetworkIo, (total_net_capacity - network_usage_mbps).max(0.0));

        // Disk I/O (simplified: total space, used space)
        let mut total_disk_space = 0.0;
        let mut used_disk_space = 0.0;
        for disk in sys.disks() {
            total_disk_space += disk.total_space() as f64 / 1_073_741_824.0; // Bytes to GB
            used_disk_space += (disk.total_space() - disk.available_space()) as f64 / 1_073_741_824.0; // Bytes to GB
        }
        total_resources.insert(ResourceType::Storage, total_disk_space);
        used_resources.insert(ResourceType::Storage, used_disk_space);
        available_resources.insert(ResourceType::Storage, total_disk_space - used_disk_space);

        // GPU - sysinfo doesn't directly provide GPU usage. Keep mock for now or integrate external crate.
        let gpu_used_pct = 0.65; // Mock: utilization percentage
        total_resources.insert(ResourceType::Gpu, 1.0); // Assume 1 GPU for now
        used_resources.insert(ResourceType::Gpu, gpu_used_pct);
        available_resources.insert(ResourceType::Gpu, 1.0 - gpu_used_pct);

        // TODO: Implement monitoring for specialized AGI Compute Units (e.g., custom accelerators)

        (total_resources, used_resources, available_resources)
    }

    /// Gathers a snapshot of current resource usage.
    pub async fn get_current_snapshot(&self) -> ResourceSnapshot {
        let (total_res, used_res, available_res) = self.get_os_level_metrics();
        let mut custom_metrics = HashMap::new();

        // Example: Add a custom AGI metric. This would ideally come from an AGI runtime API.
        custom_metrics.insert(
            "agi_inference_rate".to_string(),
            ResourceMetric {
                value: 1200.5,
                unit: "ops/sec".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            },
        );

        ResourceSnapshot {
            timestamp: Utc::now().to_rfc3339(),
            system_id: self.system_id.clone(),
            total_resources: total_res,
            used_resources: used_res,
            available_resources: available_res,
            custom_metrics,
        }
    }

    /// Continuously monitors resources and sends updates (e.g., to a channel).
    pub async fn start_monitoring<F>(&self, interval_seconds: u64, mut callback: F)
    where
        F: FnMut(ResourceSnapshot),
    {
        println!("Starting continuous resource monitoring ({}s interval)...
", interval_seconds);
        loop {
            let snapshot = self.get_current_snapshot().await;
            callback(snapshot);
            tokio::time::sleep(tokio::time::Duration::from_secs(interval_seconds)).await;
        }
    }
}
