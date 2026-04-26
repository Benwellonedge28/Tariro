use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents a discovered asset within Tariro's digital footprint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredAsset {
    pub asset_id: String,
    pub ip_address: String,
    pub hostname: Option<String>,
    pub asset_type: String, // e.g., "Server", "Workstation", "IoTDevice", "CloudInstance"
    pub open_ports: Vec<u16>,
    pub services: HashMap<u16, String>, // port -> service name (e.g., 80 -> "HTTP")
    pub vulnerabilities_detected: Vec<String>, // List of CVEs or detected weaknesses
    pub last_scanned: DateTime<Utc>,
}

// Represents a potential attack vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackVector {
    pub vector_id: String,
    pub asset_id: String, // Which asset this vector applies to
    pub vector_type: String, // e.g., "UnpatchedSoftware", "OpenPort", "WeakCredentials", "Misconfiguration"
    pub description: String,
    pub severity: String, // "Low", "Medium", "High", "Critical"
    pub recommended_remediation: String,
}

// Represents a visualization of the attack surface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackSurfaceMap {
    pub map_id: String,
    pub timestamp: DateTime<Utc>,
    pub total_assets: usize,
    pub total_attack_vectors: usize,
    pub map_data_json: String, // JSON for a network graph visualization
    pub summary_report: String,
}

// Placeholder for SIMD-accelerated network scanning and asset discovery
pub struct SimdNetworkScanner;

impl SimdNetworkScanner {
    pub fn new() -> Self {
        SimdNetworkScanner {}
    }

    // Simulate SIMD-accelerated scanning of a network range.
    // In a real SIMD context, this would involve vectorized operations for:
    // - Parallel port scanning across many IPs
    // - Rapid banner grabbing and service identification
    // - Concurrent vulnerability checks
    pub fn scan_network_segment(&self, ip_range: &str) -> Vec<DiscoveredAsset> {
        println!("SIMD Network Scanner: Scanning IP range: {}", ip_range);
        let mut assets = Vec::new();

        // Simulate discovering a few assets
        let mut rng = rand::thread_rng();

        // Example 1: Web server with some common open ports
        assets.push(DiscoveredAsset {
            asset_id: Uuid::new_v4().to_string(),
            ip_address: "192.168.1.10".to_string(),
            hostname: Some("webserver-prod-01".to_string()),
            asset_type: "Server".to_string(),
            open_ports: vec![80, 443, 22, 8080],
            services: HashMap::from([(80, "HTTP".to_string()), (443, "HTTPS".to_string()), (22, "SSH".to_string()), (8080, "Apache Tomcat".to_string())]),
            vulnerabilities_detected: if rng.gen_bool(0.3) { vec!["CVE-2023-1234 (Outdated Tomcat)".to_string()] } else { vec![] },
            last_scanned: Utc::now(),
        });

        // Example 2: Database server with a high-severity vulnerability
        assets.push(DiscoveredAsset {
            asset_id: Uuid::new_v4().to_string(),
            ip_address: "192.168.1.20".to_string(),
            hostname: Some("db-cluster-05".to_string()),
            asset_type: "Server".to_string(),
            open_ports: vec![3306],
            services: HashMap::from([(3306, "MySQL".to_string())]),
            vulnerabilities_detected: vec!["CVE-2022-5678 (MySQL Remote Code Execution)".to_string()],
            last_scanned: Utc::now(),
        });

        assets
    }
}

pub struct AttackSurfaceMapper {
    simd_network_scanner: Arc<SimdNetworkScanner>,
    discovered_assets: Mutex<HashMap<String, DiscoveredAsset>>, // key: asset_id
    attack_vectors_log: Mutex<Vec<AttackVector>>,
    attack_surface_maps: Mutex<Vec<AttackSurfaceMap>>,
}

impl AttackSurfaceMapper {
    pub fn new(simd_network_scanner: Arc<SimdNetworkScanner>) -> Self {
        AttackSurfaceMapper {
            simd_network_scanner,
            discovered_assets: Mutex::new(HashMap::new()),
            attack_vectors_log: Mutex::new(Vec::new()),
            attack_surface_maps: Mutex::new(Vec::new()),
        }
    }

    // Dynamically maps Tariro's digital footprint and identifies potential attack vectors
    pub fn run_mapping_cycle(&self, ip_ranges_to_scan: Vec<String>) -> AttackSurfaceMap {
        println!("Attack Surface Mapper: Running new mapping cycle for IP ranges: {:?}", ip_ranges_to_scan);

        let mut discovered_this_cycle = Vec::new();
        for ip_range in ip_ranges_to_scan {
            discovered_this_cycle.extend(self.simd_network_scanner.scan_network_segment(&ip_range));
        }

        let mut current_assets_locked = self.discovered_assets.lock().unwrap();
        let mut current_vectors_locked = self.attack_vectors_log.lock().unwrap();
        let mut total_attack_vectors = 0;

        for asset in discovered_this_cycle {
            current_assets_locked.insert(asset.asset_id.clone(), asset.clone());

            // Identify attack vectors for this asset
            if !asset.open_ports.is_empty() {
                for port in &asset.open_ports {
                    let service = asset.services.get(port).map_or("Unknown Service", |s| s.as_str());
                    let vector = AttackVector {
                        vector_id: Uuid::new_v4().to_string(),
                        asset_id: asset.asset_id.clone(),
                        vector_type: "OpenPort".to_string(),
                        description: format!("Open port {} detected with service: {}", port, service),
                        severity: "Low".to_string(), // Severity would be determined by port/service combo
                        recommended_remediation: format!("Review necessity of port {} for service {}.", port, service),
                    };
                    current_vectors_locked.push(vector);
                    total_attack_vectors += 1;
                }
            }
            for vul in &asset.vulnerabilities_detected {
                let vector = AttackVector {
                    vector_id: Uuid::new_v4().to_string(),
                    asset_id: asset.asset_id.clone(),
                    vector_type: "Vulnerability".to_string(),
                    description: format!("Detected vulnerability: {}", vul),
                    severity: "High".to_string(), // Assume high for detected CVEs
                    recommended_remediation: format!("Apply patch or implement compensating controls for: {}", vul),
                };
                current_vectors_locked.push(vector);
                total_attack_vectors += 1;
            }
        }

        // Generate a simplified network graph JSON
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for asset in current_assets_locked.values() {
            nodes.push(format!("{{ id: '{}', label: '{}' ({}), group: '{}', color: '{}' }}",
                               asset.asset_id,
                               asset.hostname.as_ref().unwrap_or(&asset.ip_address),
                               asset.asset_type,
                               asset.asset_type,
                               if asset.vulnerabilities_detected.is_empty() { "green" } else { "red" }));
        }
        // Simplified: just connect nodes if they were scanned in the same segment, or to a 'network' node
        nodes.push("{ id: 'network', label: 'Tariro Network', group: 'Network', color: 'blue' }".to_string());
        for asset in current_assets_locked.values() {
            edges.push(format!("{{ from: '{}', to: 'network', label: 'part_of' }}", asset.asset_id));
        }

        let map_data_json = format!("{{ nodes: [{}], edges: [{}] }}", nodes.join(", "), edges.join(", "));


        let map = AttackSurfaceMap {
            map_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            total_assets: current_assets_locked.len(),
            total_attack_vectors,
            map_data_json,
            summary_report: format!("Scanned {} IP ranges. Discovered {} assets with {} potential attack vectors.",
                                    ip_ranges_to_scan.len(), current_assets_locked.len(), total_attack_vectors),
        };

        self.attack_surface_maps.lock().unwrap().push(map.clone());
        println!("Attack Surface Mapper: Generated new attack surface map: {}", map.map_id);
        map
    }

    pub fn get_discovered_assets(&self) -> Vec<DiscoveredAsset> {
        self.discovered_assets.lock().unwrap().values().cloned().collect()
    }

    pub fn get_attack_vectors_log(&self) -> Vec<AttackVector> {
        self.attack_vectors_log.lock().unwrap().clone()
    }

    pub fn get_attack_surface_maps(&self) -> Vec<AttackSurfaceMap> {
        self.attack_surface_maps.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_surface_mapper() {
        let simd_scanner = Arc::new(SimdNetworkScanner::new());
        let mapper = AttackSurfaceMapper::new(simd_scanner);

        let ip_ranges = vec!["192.168.1.0/24".to_string(), "10.0.0.0/16".to_string()];
        let attack_map = mapper.run_mapping_cycle(ip_ranges.clone());

        println!("Generated Attack Surface Map Summary: {}", attack_map.summary_report);
        println!("Map Data JSON (truncated): {}", &attack_map.map_data_json[..500]); // Print first 500 chars

        assert!(attack_map.total_assets > 0);
        assert!(attack_map.total_attack_vectors > 0);
        assert!(!attack_map.map_data_json.is_empty());

        let assets = mapper.get_discovered_assets();
        assert!(!assets.is_empty());

        let vectors = mapper.get_attack_vectors_log();
        assert!(!vectors.is_empty());

        let maps_log = mapper.get_attack_surface_maps();
        assert_eq!(maps_log.len(), 1);
    }
}
*/