//! Device Capability Detection
//!
//! This module provides automatic detection of device capabilities
//! and classification into device tiers for the POCO-REAF paradigm.
//!
//! # Device Tiers
//!
//! Tariro AGI supports five device tiers, each with different
//! capabilities and constraints:
//!
//! | Tier | Memory | Typical Use Case | Key Constraints |
//! |------|--------|------------------|-----------------|
//! | Microcontroller | < 64KB | IoT sensors, embedded control | No OS, limited RAM |
//! | Embedded | 64KB-16MB | Smart devices, robotics | RTOS, single-core typical |
//! | Edge | 16MB-4GB | Gateways, local AI | Full OS, network required |
//! | Cloud | > 4GB | Servers, data centers | Unlimited resources |
//! | Planet | Distributed | Global AI networks | Geo-distributed, high latency |

// ============================================================================
// Core Types
// ============================================================================

use serde::{Deserialize, Serialize};

/// Device capability tiers for POCO-REAF deployment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum DeviceTier {
    /// Microcontrollers: < 64KB RAM, no OS
    /// Example: ARM Cortex-M0, ESP32, Arduino
    Microcontroller = 0,

    /// Embedded: 64KB - 16MB RAM, RTOS or simple Linux
    /// Example: Raspberry Pi Pico, STM32, Jetson Nano
    Embedded = 1,

    /// Edge: 16MB - 4GB RAM, full OS
    /// Example: Raspberry Pi 4, desktop, workstation
    Edge = 2,

    /// Cloud: > 4GB RAM, distributed cluster
    /// Example: Server, data center, HPC
    Cloud = 3,

    /// Planet: Geo-distributed, fault-tolerant
    /// Example: Multi-region cloud, satellite networks
    Planet = 4,
}

impl DeviceTier {
    /// Get human-readable name for the tier
    pub fn name(&self) -> &'static str {
        match self {
            DeviceTier::Microcontroller => "Microcontroller",
            DeviceTier::Embedded => "Embedded",
            DeviceTier::Edge => "Edge",
            DeviceTier::Cloud => "Cloud",
            DeviceTier::Planet => "Planet",
        }
    }

    /// Get description of the tier
    pub fn description(&self) -> &'static str {
        match self {
            DeviceTier::Microcontroller => {
                "Resource-constrained microcontroller (< 64KB RAM)"
            }
            DeviceTier::Embedded => {
                "Embedded system with limited resources (64KB - 16MB RAM)"
            }
            DeviceTier::Edge => {
                "Edge device with full OS (16MB - 4GB RAM)"
            }
            DeviceTier::Cloud => {
                "Cloud server or data center (> 4GB RAM)"
            }
            DeviceTier::Planet => {
                "Geo-distributed planet-scale deployment"
            }
        }
    }

    /// Get minimum memory requirement for this tier (bytes)
    pub fn minimum_memory(&self) -> u64 {
        match self {
            DeviceTier::Microcontroller => 8 * 1024,              // 8KB
            DeviceTier::Embedded => 64 * 1024,                   // 64KB
            DeviceTier::Edge => 16 * 1024 * 1024,                // 16MB
            DeviceTier::Cloud => 4 * 1024 * 1024 * 1024,         // 4GB
            DeviceTier::Planet => 512 * 1024 * 1024 * 1024,      // 512GB
        }
    }

    /// Check if this tier supports networking
    pub fn supports_networking(&self) -> bool {
        !matches!(self, DeviceTier::Microcontroller)
    }

    /// Check if this tier supports multi-threading
    pub fn supports_multithreading(&self) -> bool {
        matches!(self, DeviceTier::Edge | DeviceTier::Cloud | DeviceTier::Planet)
    }

    /// Check if this tier supports floating-point operations
    pub fn supports_floating_point(&self) -> bool {
        !matches!(self, DeviceTier::Microcontroller)
    }

    /// Check if this tier supports dynamic memory allocation
    pub fn supports_dynamic_memory(&self) -> bool {
        !matches!(self, DeviceTier::Microcontroller)
    }

    /// Get recommended module loading strategy for this tier
    pub fn loading_strategy(&self) -> LoadingStrategy {
        match self {
            DeviceTier::Microcontroller => LoadingStrategy::Minimal,
            DeviceTier::Embedded => LoadingStrategy::Selective,
            DeviceTier::Edge => LoadingStrategy::Standard,
            DeviceTier::Cloud => LoadingStrategy::Full,
            DeviceTier::Planet => LoadingStrategy::Distributed,
        }
    }
}

impl std::fmt::Display for DeviceTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Default for DeviceTier {
    fn default() -> Self {
        DeviceTier::Edge
    }
}

/// Loading strategy based on device capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadingStrategy {
    /// Load only essential modules (kernel)
    Minimal,

    /// Load essential + selected modules
    Selective,

    /// Load all standard modules
    Standard,

    /// Load all modules including experimental
    Full,

    /// Load distributed modules across nodes
    Distributed,
}

/// Storage type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    /// No persistent storage
    None,

    /// ROM/Flash storage (read-mostly)
    Rom,

    /// EEPROM or similar (limited writes)
    Eeprom,

    /// SD card or similar removable storage
    SdCard,

    /// Solid state drive
    Ssd,

    /// Hard disk drive
    Hdd,

    /// Network attached storage
    Nas,

    /// Distributed storage
    Distributed,
}

/// Power mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerMode {
    /// Always on, no power constraints
    AlwaysOn,

    /// Active computation mode
    Active,

    /// Low power standby
    Standby,

    /// Deep sleep mode
    Sleep,

    /// Battery powered
    Battery,

    /// Solar powered
    Solar,

    /// Energy harvesting
    Harvesting,
}

/// Feature flags for capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FeatureFlags(u32);

impl FeatureFlags {
    /// No special features
    pub const EMPTY: Self = FeatureFlags(0);

    /// Symbolic reasoning support
    pub const SYMBOLIC: Self = FeatureFlags(1 << 0);

    /// Language processing support
    pub const LANGUAGE: Self = FeatureFlags(1 << 1);

    /// Consciousness engine support
    pub const CONSCIOUSNESS: Self = FeatureFlags(1 << 2);

    /// Distributed computing support
    pub const DISTRIBUTED: Self = FeatureFlags(1 << 3);

    /// Quantum-safe cryptography
    pub const QUANTUM_SAFE: Self = FeatureFlags(1 << 4);

    /// Hardware floating point
    pub const FPU: Self = FeatureFlags(1 << 5);

    /// SIMD instructions
    pub const SIMD: Self = FeatureFlags(1 << 6);

    /// GPU acceleration
    pub const GPU: Self = FeatureFlags(1 << 7);

    /// Neural network acceleration
    pub const NPU: Self = FeatureFlags(1 << 8);

    /// Trusted execution environment
    pub const TEE: Self = FeatureFlags(1 << 9);

    /// Create flags with given bits
    pub const fn new(bits: u32) -> Self {
        FeatureFlags(bits)
    }

    /// Check if a flag is set
    pub fn has(self, flag: FeatureFlags) -> bool {
        (self.0 & flag.0) != 0
    }

    /// Set a flag
    pub fn set(&mut self, flag: FeatureFlags) {
        self.0 |= flag.0;
    }

    /// Clear a flag
    pub fn clear(&mut self, flag: FeatureFlags) {
        self.0 &= !flag.0;
    }

    /// Get all flags as bits
    pub fn bits(&self) -> u32 {
        self.0
    }
}

impl std::ops::BitOr for FeatureFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        FeatureFlags(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for FeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        FeatureFlags(self.0 & rhs.0)
    }
}

// ============================================================================
// Capability Profile
// ============================================================================

/// System capability profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityProfile {
    /// Device tier classification
    pub tier: DeviceTier,

    // Memory constraints (bytes)
    /// Minimum guaranteed memory
    pub memory_min: u64,

    /// Maximum available memory
    pub memory_max: u64,

    /// Recommended stack size (bytes)
    pub stack_size: usize,

    /// Recommended heap size (bytes)
    pub heap_size: usize,

    // Compute constraints
    /// Number of CPU cores
    pub cpu_cores: usize,

    /// CPU frequency (MHz)
    pub cpu_frequency_mhz: f32,

    /// Has hardware FPU
    pub has_fpu: bool,

    /// Has SIMD instructions
    pub has_simd: bool,

    /// Has GPU
    pub has_gpu: bool,

    /// Has NPU
    pub has_npu: bool,

    // Network constraints
    /// Has network connectivity
    pub has_network: bool,

    /// Typical bandwidth (Mbps)
    pub bandwidth_mbps: u32,

    /// Typical latency (ms)
    pub latency_ms: u32,

    /// Network type
    pub network_type: NetworkType,

    // Persistence
    /// Has persistent storage
    pub has_storage: bool,

    /// Storage type
    pub storage_type: StorageType,

    /// Storage size (bytes)
    pub storage_size: u64,

    // Power constraints
    /// Power mode
    pub power_mode: PowerMode,

    /// Thermal limit (Celsius)
    pub thermal_limit_celsius: f32,

    /// Typical power consumption (Watts)
    pub power_consumption_watts: f32,

    // OS constraints
    /// Has standard library
    pub has_std: bool,

    /// Operating system
    pub os: OperatingSystem,

    /// Architecture
    pub architecture: Architecture,

    /// Feature flags
    pub features: FeatureFlags,
}

impl Default for CapabilityProfile {
    fn default() -> Self {
        Self {
            tier: DeviceTier::default(),
            memory_min: 0,
            memory_max: 0,
            stack_size: 8 * 1024 * 1024,
            heap_size: 128 * 1024 * 1024,
            cpu_cores: 1,
            cpu_frequency_mhz: 100.0,
            has_fpu: true,
            has_simd: false,
            has_gpu: false,
            has_npu: false,
            has_network: false,
            bandwidth_mbps: 0,
            latency_ms: 1000,
            network_type: NetworkType::None,
            has_storage: false,
            storage_type: StorageType::None,
            storage_size: 0,
            power_mode: PowerMode::Active,
            thermal_limit_celsius: 85.0,
            power_consumption_watts: 1.0,
            has_std: true,
            os: OperatingSystem::Unknown,
            architecture: Architecture::Unknown,
            features: FeatureFlags::EMPTY,
        }
    }
}

impl CapabilityProfile {
    /// Detect device tier automatically
    pub fn detect() -> Self {
        #[cfg(target_arch = "wasm32")]
        return Self::detect_wasm();

        #[cfg(all(feature = "std", not(target_arch = "wasm32")))]
        return Self::detect_native();

        #[cfg(not(feature = "std"))]
        return Self::detect_embedded();
    }

    /// Detect for embedded (no-std) environments
    pub fn detect_embedded() -> Self {
        // Conservative defaults for unknown embedded
        CapabilityProfile {
            tier: DeviceTier::Microcontroller,
            memory_min: 0,
            memory_max: 64 * 1024,
            stack_size: 1024,
            heap_size: 32 * 1024,
            cpu_cores: 1,
            cpu_frequency_mhz: 48.0,
            has_fpu: false,
            has_simd: false,
            has_gpu: false,
            has_npu: false,
            has_network: false,
            bandwidth_mbps: 0,
            latency_ms: 1000,
            network_type: NetworkType::None,
            has_storage: false,
            storage_type: StorageType::Rom,
            storage_size: 512 * 1024,
            power_mode: PowerMode::Battery,
            thermal_limit_celsius: 85.0,
            power_consumption_watts: 0.1,
            has_std: false,
            os: OperatingSystem::None,
            architecture: Architecture::Arm,
            features: FeatureFlags::SYMBOLIC,
        }
    }

    /// Detect for native (std) environments
    #[cfg(feature = "std")]
    pub fn detect_native() -> Self {
        use std::mem;

        // Try to detect actual system resources
        let memory_total = Self::detect_memory();
        let cpu_count = Self::detect_cpu_count();
        let cpu_freq = Self::detect_cpu_frequency();
        let os = Self::detect_os();
        let arch = Self::detect_architecture();
        let network = Self::detect_network();

        // Determine tier based on total memory
        let tier = if memory_total < 64 * 1024 {
            DeviceTier::Microcontroller
        } else if memory_total < 16 * 1024 * 1024 {
            DeviceTier::Embedded
        } else if memory_total < 4 * 1024 * 1024 * 1024 {
            DeviceTier::Edge
        } else {
            DeviceTier::Cloud
        };

        let mut features = FeatureFlags::SYMBOLIC | FeatureFlags::LANGUAGE;
        if Self::detect_has_fpu() {
            features = features | FeatureFlags::FPU;
        }
        if Self::detect_has_simd() {
            features = features | FeatureFlags::SIMD;
        }

        CapabilityProfile {
            tier,
            memory_min: 0,
            memory_max: memory_total,
            stack_size: 8 * 1024 * 1024,
            heap_size: memory_total / 4,
            cpu_cores: cpu_count,
            cpu_frequency_mhz: cpu_freq,
            has_fpu: Self::detect_has_fpu(),
            has_simd: Self::detect_has_simd(),
            has_gpu: Self::detect_gpu(),
            has_npu: false,
            has_network: network.0,
            bandwidth_mbps: network.1,
            latency_ms: network.2,
            network_type: network.3,
            has_storage: true,
            storage_type: StorageType::Ssd,
            storage_size: Self::detect_storage_size(),
            power_mode: PowerMode::AlwaysOn,
            thermal_limit_celsius: 90.0,
            power_consumption_watts: 50.0,
            has_std: true,
            os,
            architecture: arch,
            features,
        }
    }

    /// Detect for WebAssembly
    pub fn detect_wasm() -> Self {
        CapabilityProfile {
            tier: DeviceTier::Embedded,
            memory_min: 0,
            memory_max: 2 * 1024 * 1024 * 1024, // 2GB limit in wasm
            stack_size: 512 * 1024,
            heap_size: 16 * 1024 * 1024,
            cpu_cores: 4, // Navigator.hardwareConcurrency if available
            cpu_frequency_mhz: 1000.0,
            has_fpu: true,
            has_simd: false,
            has_gpu: false,
            has_npu: false,
            has_network: true,
            bandwidth_mbps: 100,
            latency_ms: 50,
            network_type: NetworkType::Wifi,
            has_storage: false,
            storage_type: StorageType::None,
            storage_size: 0,
            power_mode: PowerMode::Active,
            thermal_limit_celsius: 60.0,
            power_consumption_watts: 5.0,
            has_std: true,
            os: OperatingSystem::Browser,
            architecture: Architecture::Wasm32,
            features: FeatureFlags::SYMBOLIC | FeatureFlags::LANGUAGE,
        }
    }

    #[cfg(feature = "std")]
    fn detect_memory() -> u64 {
        // Try sys_info crate or default
        #[cfg(feature = "sys-info")]
        {
            if let Ok(info) = sys_info::mem_info() {
                return info.total * 1024; // KB to bytes
            }
        }
        // Fallback: assume 4GB
        4 * 1024 * 1024 * 1024
    }

    #[cfg(not(feature = "sys-info"))]
    const fn detect_memory() -> u64 {
        4 * 1024 * 1024 * 1024 // Default 4GB
    }

    #[cfg(feature = "std")]
    fn detect_cpu_count() -> usize {
        // Use num_cpus crate or standard library
        #[cfg(feature = "num-cpus")]
        {
            return num_cpus::get();
        }
        #[cfg(not(feature = "num-cpus"))]
        {
            std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1)
        }
    }

    #[cfg(not(feature = "std"))]
    const fn detect_cpu_count() -> usize {
        1
    }

    #[cfg(feature = "std")]
    fn detect_cpu_frequency() -> f32 {
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            if let Ok(contents) = fs::read_to_string("/proc/cpuinfo") {
                for line in contents.lines() {
                    if line.starts_with("cpu MHz") {
                        if let Some(freq) = line.split(':').nth(1) {
                            if let Ok(mhz) = freq.trim().parse::<f32>() {
                                return mhz;
                            }
                        }
                    }
                }
            }
        }
        1000.0 // Default 1GHz
    }

    #[cfg(not(feature = "std"))]
    const fn detect_cpu_frequency() -> f32 {
        100.0 // Default 100MHz
    }

    #[cfg(feature = "std")]
    fn detect_os() -> OperatingSystem {
        #[cfg(target_os = "linux")]
        return OperatingSystem::Linux;

        #[cfg(target_os = "macos")]
        return OperatingSystem::Macos;

        #[cfg(target_os = "windows")]
        return OperatingSystem::Windows;

        #[cfg(target_os = "freebsd")]
        return OperatingSystem::FreeBSD;

        #[cfg(target_os = "fuchsia")]
        return OperatingSystem::Fuchsia;

        OperatingSystem::Unknown
    }

    #[cfg(not(feature = "std"))]
    const fn detect_os() -> OperatingSystem {
        OperatingSystem::None
    }

    #[cfg(feature = "std")]
    fn detect_architecture() -> Architecture {
        #[cfg(target_arch = "x86_64")]
        return Architecture::X86_64;

        #[cfg(target_arch = "aarch64")]
        return Architecture::Arm64;

        #[cfg(target_arch = "arm")]
        return Architecture::Arm;

        #[cfg(target_arch = "riscv32")]
        return Architecture::Riscv32;

        #[cfg(target_arch = "riscv64")]
        return Architecture::Riscv64;

        #[cfg(target_arch = "wasm32")]
        return Architecture::Wasm32;

        Architecture::Unknown
    }

    #[cfg(not(feature = "std"))]
    const fn detect_architecture() -> Architecture {
        Architecture::Unknown
    }

    #[cfg(feature = "std")]
    fn detect_has_fpu() -> bool {
        #[cfg(target_arch = "x86_64")]
        return true;

        #[cfg(target_arch = "aarch64")]
        return true;

        #[cfg(target_arch = "arm")]
        // Most ARM chips have FPU except M0/M1
        return true;

        false
    }

    #[cfg(not(feature = "std"))]
    const fn detect_has_fpu() -> bool {
        false
    }

    #[cfg(feature = "std")]
    fn detect_has_simd() -> bool {
        #[cfg(target_arch = "x86_64")]
        return true;

        #[cfg(target_arch = "aarch64")]
        return true;

        false
    }

    #[cfg(not(feature = "std"))]
    const fn detect_has_simd() -> bool {
        false
    }

    #[cfg(feature = "std")]
    fn detect_gpu() -> bool {
        // This is a simplified check - in practice would need platform-specific code
        false
    }

    #[cfg(not(feature = "std"))]
    const fn detect_gpu() -> bool {
        false
    }

    #[cfg(feature = "std")]
    fn detect_network() -> (bool, u32, u32, NetworkType) {
        // Simplified - assume network available on desktop/server
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        {
            (true, 1000, 1, NetworkType::Ethernet)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            (false, 0, 1000, NetworkType::None)
        }
    }

    #[cfg(not(feature = "std"))]
    fn detect_network() -> (bool, u32, u32, NetworkType) {
        (false, 0, 1000, NetworkType::None)
    }

    #[cfg(feature = "std")]
    fn detect_storage_size() -> u64 {
        // Simplified - assume 256GB
        256 * 1024 * 1024 * 1024
    }

    #[cfg(not(feature = "std"))]
    const fn detect_storage_size() -> u64 {
        0
    }

    /// Create a profile for a specific tier
    pub fn for_tier(tier: DeviceTier) -> Self {
        match tier {
            DeviceTier::Microcontroller => Self::for_microcontroller(),
            DeviceTier::Embedded => Self::for_embedded(),
            DeviceTier::Edge => Self::for_edge(),
            DeviceTier::Cloud => Self::for_cloud(),
            DeviceTier::Planet => Self::for_planet(),
        }
    }

    /// Profile for microcontroller tier
    pub fn for_microcontroller() -> Self {
        CapabilityProfile {
            tier: DeviceTier::Microcontroller,
            memory_min: 0,
            memory_max: 64 * 1024,
            stack_size: 512,
            heap_size: 32 * 1024,
            cpu_cores: 1,
            cpu_frequency_mhz: 48.0,
            has_fpu: false,
            has_simd: false,
            has_gpu: false,
            has_npu: false,
            has_network: false,
            bandwidth_mbps: 0,
            latency_ms: 1000,
            network_type: NetworkType::None,
            has_storage: true,
            storage_type: StorageType::Rom,
            storage_size: 512 * 1024,
            power_mode: PowerMode::Battery,
            thermal_limit_celsius: 85.0,
            power_consumption_watts: 0.05,
            has_std: false,
            os: OperatingSystem::None,
            architecture: Architecture::Arm,
            features: FeatureFlags::SYMBOLIC,
        }
    }

    /// Profile for embedded tier
    pub fn for_embedded() -> Self {
        CapabilityProfile {
            tier: DeviceTier::Embedded,
            memory_min: 64 * 1024,
            memory_max: 16 * 1024 * 1024,
            stack_size: 8 * 1024,
            heap_size: 8 * 1024 * 1024,
            cpu_cores: 1,
            cpu_frequency_mhz: 240.0,
            has_fpu: true,
            has_simd: false,
            has_gpu: false,
            has_npu: false,
            has_network: true,
            bandwidth_mbps: 100,
            latency_ms: 50,
            network_type: NetworkType::Wifi,
            has_storage: true,
            storage_type: StorageType::SdCard,
            storage_size: 32 * 1024 * 1024 * 1024,
            power_mode: PowerMode::Battery,
            thermal_limit_celsius: 85.0,
            power_consumption_watts: 2.0,
            has_std: true,
            os: OperatingSystem::Linux,
            architecture: Architecture::Arm,
            features: FeatureFlags::SYMBOLIC | FeatureFlags::LANGUAGE,
        }
    }

    /// Profile for edge tier
    pub fn for_edge() -> Self {
        CapabilityProfile {
            tier: DeviceTier::Edge,
            memory_min: 16 * 1024 * 1024,
            memory_max: 4 * 1024 * 1024 * 1024,
            stack_size: 8 * 1024 * 1024,
            heap_size: 512 * 1024 * 1024,
            cpu_cores: 4,
            cpu_frequency_mhz: 2000.0,
            has_fpu: true,
            has_simd: true,
            has_gpu: false,
            has_npu: false,
            has_network: true,
            bandwidth_mbps: 1000,
            latency_ms: 5,
            network_type: NetworkType::Ethernet,
            has_storage: true,
            storage_type: StorageType::Ssd,
            storage_size: 512 * 1024 * 1024 * 1024,
            power_mode: PowerMode::AlwaysOn,
            thermal_limit_celsius: 80.0,
            power_consumption_watts: 15.0,
            has_std: true,
            os: OperatingSystem::Linux,
            architecture: Architecture::Arm64,
            features: FeatureFlags::SYMBOLIC | FeatureFlags::LANGUAGE | FeatureFlags::QUANTUM_SAFE,
        }
    }

    /// Profile for cloud tier
    pub fn for_cloud() -> Self {
        CapabilityProfile {
            tier: DeviceTier::Cloud,
            memory_min: 4 * 1024 * 1024 * 1024,
            memory_max: 256 * 1024 * 1024 * 1024,
            stack_size: 8 * 1024 * 1024,
            heap_size: 64 * 1024 * 1024 * 1024,
            cpu_cores: 64,
            cpu_frequency_mhz: 3000.0,
            has_fpu: true,
            has_simd: true,
            has_gpu: true,
            has_npu: true,
            has_network: true,
            bandwidth_mbps: 10000,
            latency_ms: 1,
            network_type: NetworkType::Ethernet,
            has_storage: true,
            storage_type: StorageType::Distributed,
            storage_size: 10 * 1024 * 1024 * 1024 * 1024,
            power_mode: PowerMode::AlwaysOn,
            thermal_limit_celsius: 70.0,
            power_consumption_watts: 500.0,
            has_std: true,
            os: OperatingSystem::Linux,
            architecture: Architecture::X86_64,
            features: FeatureFlags::SYMBOLIC
                | FeatureFlags::LANGUAGE
                | FeatureFlags::CONSCIOUSNESS
                | FeatureFlags::DISTRIBUTED
                | FeatureFlags::QUANTUM_SAFE
                | FeatureFlags::FPU
                | FeatureFlags::SIMD
                | FeatureFlags::GPU
                | FeatureFlags::TEE,
        }
    }

    /// Profile for planet tier
    pub fn for_planet() -> Self {
        let mut profile = Self::for_cloud();
        profile.tier = DeviceTier::Planet;
        profile.features = profile.features | FeatureFlags::DISTRIBUTED;
        profile.network_type = NetworkType::Satellite;
        profile.latency_ms = 100;
        profile
    }

    /// Check if this profile meets requirements for a module
    pub fn can_load(&self, required_memory: u64, required_tier: DeviceTier) -> bool {
        self.tier >= required_tier && self.memory_max >= required_memory
    }

    /// Get available memory (accounting for overhead)
    pub fn available_memory(&self) -> u64 {
        // Reserve 20% for system overhead
        self.memory_max * 80 / 100
    }

    /// Estimate processing capacity as a normalized score (0.0 - 1.0)
    pub fn processing_capacity(&self) -> f32 {
        let memory_score = (self.memory_max as f32 / (4 * 1024 * 1024 * 1024) as f32).min(1.0);
        let cpu_score = (self.cpu_cores as f32 / 64.0).min(1.0);
        let freq_score = (self.cpu_frequency_mhz / 3000.0).min(1.0);

        (memory_score + cpu_score + freq_score) / 3.0
    }
}

/// Network type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    /// No network
    None,

    /// Serial communication
    Serial,

    /// CAN bus
    Can,

    /// I2C/SPI (device-to-device)
    Bus,

    /// Bluetooth/BLE
    Bluetooth,

    /// WiFi
    Wifi,

    /// Ethernet
    Ethernet,

    /// Cellular (4G/5G)
    Cellular,

    /// LoRa WAN
    Lora,

    /// Satellite
    Satellite,

    /// Mesh network
    Mesh,
}

/// Operating system enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatingSystem {
    /// No OS (bare metal)
    None,

    /// Real-time OS
    Rtos,

    /// Linux
    Linux,

    /// macOS
    Macos,

    /// Windows
    Windows,

    /// FreeBSD
    FreeBSD,

    /// Fuchsia
    Fuchsia,

    /// Android
    Android,

    /// iOS
    Ios,

    /// Browser (WebAssembly)
    Browser,

    /// Unknown
    Unknown,
}

/// Architecture enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Architecture {
    /// x86-64
    X86_64,

    /// x86 (32-bit)
    X86,

    /// ARM 32-bit
    Arm,

    /// ARM 64-bit
    Arm64,

    /// RISC-V 32-bit
    Riscv32,

    /// RISC-V 64-bit
    Riscv64,

    /// WebAssembly 32-bit
    Wasm32,

    /// MIPS
    MIPS,

    /// PowerPC
    PowerPC,

    /// SPARC
    SPARC,

    /// Unknown
    Unknown,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_ordering() {
        assert!(DeviceTier::Microcontroller < DeviceTier::Embedded);
        assert!(DeviceTier::Embedded < DeviceTier::Edge);
        assert!(DeviceTier::Edge < DeviceTier::Cloud);
        assert!(DeviceTier::Cloud < DeviceTier::Planet);
    }

    #[test]
    fn test_tier_capabilities() {
        assert!(!DeviceTier::Microcontroller.supports_networking());
        assert!(DeviceTier::Embedded.supports_networking());
        assert!(DeviceTier::Edge.supports_multithreading());
        assert!(DeviceTier::Cloud.supports_multithreading());
    }

    #[test]
    fn test_profile_tier_classification() {
        let micro = CapabilityProfile::for_microcontroller();
        assert_eq!(micro.tier, DeviceTier::Microcontroller);
        assert!(micro.memory_max < 64 * 1024);

        let embedded = CapabilityProfile::for_embedded();
        assert_eq!(embedded.tier, DeviceTier::Embedded);

        let edge = CapabilityProfile::for_edge();
        assert_eq!(edge.tier, DeviceTier::Edge);

        let cloud = CapabilityProfile::for_cloud();
        assert_eq!(cloud.tier, DeviceTier::Cloud);
    }

    #[test]
    fn test_feature_flags() {
        let mut flags = FeatureFlags::EMPTY;
        assert!(!flags.has(FeatureFlags::SYMBOLIC));

        flags.set(FeatureFlags::SYMBOLIC);
        flags.set(FeatureFlags::LANGUAGE);
        assert!(flags.has(FeatureFlags::SYMBOLIC));
        assert!(flags.has(FeatureFlags::LANGUAGE));
        assert!(!flags.has(FeatureFlags::CONSCIOUSNESS));

        let combined = FeatureFlags::SYMBOLIC | FeatureFlags::GPU;
        assert!(combined.has(FeatureFlags::SYMBOLIC));
        assert!(combined.has(FeatureFlags::GPU));
    }

    #[test]
    fn test_available_memory() {
        let profile = CapabilityProfile::for_edge();
        let available = profile.available_memory();
        // Should be 80% of max
        assert_eq!(available, profile.memory_max * 80 / 100);
    }

    #[test]
    fn test_processing_capacity() {
        let micro = CapabilityProfile::for_microcontroller();
        let cloud = CapabilityProfile::for_cloud();

        assert!(micro.processing_capacity() < cloud.processing_capacity());
        assert!(micro.processing_capacity() <= 1.0);
        assert!(cloud.processing_capacity() <= 1.0);
    }
}
