//! Tiered Memory Manager
//!
//! This module provides hierarchical memory management for Tariro AGI,
//! optimizing memory usage across different device tiers.
//!
//! # Memory Tiers
//!
//! | Tier | Technology | Latency | Size | Volatile |
//! |------|------------|---------|------|----------|
//! | L1   | Registers/Cache | < 1μs | KB | Yes |
//! | L2   | Fast SRAM | < 10μs | MB | Yes |
//! | L3   | DRAM/Heap | < 10ms | GB | Yes |
//! | L4   | Disk/Flash | < 100ms | TB | No |
//! | L5   | Network | Variable | PB | No |

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, RwLock};

/// Memory tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MemoryTier {
    /// Ultra-fast, tiny, locked in RAM (registers, L1 cache)
    L1Cache = 0,

    /// Fast, small, managed memory (L2 cache, fast heap)
    L2Heap = 1,

    /// Slow, large, standard heap (DRAM)
    L3Dram = 2,

    /// Persistent, disk-backed (SSD, flash)
    L4Disk = 3,

    /// Network-accessed, distributed
    L5Network = 4,
}

impl MemoryTier {
    /// Get maximum size for this tier
    pub fn max_size(&self) -> u64 {
        match self {
            MemoryTier::L1Cache => 256 * 1024,                    // 256KB
            MemoryTier::L2Heap => 64 * 1024 * 1024,                // 64MB
            MemoryTier::L3Dram => 4 * 1024 * 1024 * 1024,          // 4GB
            MemoryTier::L4Disk => 1 * 1024 * 1024 * 1024 * 1024,   // 1TB
            MemoryTier::L5Network => u64::MAX,                      // Unlimited
        }
    }

    /// Get typical latency
    pub fn typical_latency_us(&self) -> u64 {
        match self {
            MemoryTier::L1Cache => 1,
            MemoryTier::L2Heap => 10,
            MemoryTier::L3Dram => 100,
            MemoryTier::L4Disk => 100_000,
            MemoryTier::L5Network => 1_000_000,
        }
    }

    /// Check if this tier is volatile
    pub fn is_volatile(&self) -> bool {
        matches!(self, MemoryTier::L1Cache | MemoryTier::L2Heap | MemoryTier::L3Dram)
    }

    /// Check if this tier supports random access
    pub fn supports_random_access(&self) -> bool {
        true
    }

    /// Get tier from size hint
    pub fn from_size(size: u64) -> Self {
        if size <= 4096 {
            MemoryTier::L1Cache
        } else if size <= 1024 * 1024 {
            MemoryTier::L2Heap
        } else if size <= 64 * 1024 * 1024 {
            MemoryTier::L3Dram
        } else if size <= 1024 * 1024 * 1024 {
            MemoryTier::L4Disk
        } else {
            MemoryTier::L5Network
        }
    }
}

impl Default for MemoryTier {
    fn default() -> Self {
        MemoryTier::L2Heap
    }
}

/// Access priority for memory allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AccessPriority {
    /// Critical, never evicted
    Critical = 0,

    /// High priority
    High = 1,

    /// Normal priority
    Normal = 2,

    /// Low priority, can be evicted
    Low = 3,

    /// Background, evicted first
    Background = 4,
}

/// Memory allocation request
#[derive(Debug, Clone)]
pub struct AllocationRequest {
    pub size: u64,
    pub tier: Option<MemoryTier>,
    pub priority: AccessPriority,
    pub persistent: bool,
    pub align: Option<u64>,
}

/// Memory region descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    /// Virtual address
    pub addr: u64,

    /// Size in bytes
    pub size: u64,

    /// Memory tier
    pub tier: MemoryTier,

    /// Is allocated
    pub allocated: bool,

    /// Access priority
    pub priority: AccessPriority,

    /// Reference count
    pub ref_count: u32,

    /// Last access timestamp
    pub last_access: u64,
}

impl MemoryRegion {
    /// Check if address is in this region
    pub fn contains(&self, addr: u64) -> bool {
        addr >= self.addr && addr < self.addr + self.size
    }
}

/// Memory statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total memory across all tiers
    pub total_bytes: u64,

    /// Used memory across all tiers
    pub used_bytes: u64,

    /// Available memory across all tiers
    pub available_bytes: u64,

    /// Per-tier statistics
    pub tier_stats: HashMap<MemoryTier, TierStats>,
}

/// Per-tier memory statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TierStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub allocation_count: u64,
    pub eviction_count: u64,
    pub hit_count: u64,
    pub miss_count: u64,
}

impl TierStats {
    /// Get utilization percentage
    pub fn utilization_pct(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.used_bytes as f32 / self.total_bytes as f32) * 100.0
        }
    }

    /// Get hit rate
    pub fn hit_rate(&self) -> f32 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            (self.hit_count as f32 / total as f32) * 100.0
        }
    }
}

/// Memory allocation result
#[derive(Debug, Clone)]
pub struct Allocation {
    /// Virtual address
    pub addr: u64,

    /// Size in bytes
    pub size: u64,

    /// Tier where allocated
    pub tier: MemoryTier,

    /// Handle for accessing the allocation
    pub handle: AllocationHandle,
}

/// Handle for accessing memory
#[derive(Debug, Clone)]
pub struct AllocationHandle {
    pub id: u64,
    pub tier: MemoryTier,
}

/// Tiered memory manager
pub struct TieredMemory {
    /// L1: Ring buffer for ultra-fast access
    l1_regions: VecDeque<MemoryRegion>,

    /// L2: Slab allocator
    l2_regions: VecDeque<MemoryRegion>,

    /// L3: BTree for standard heap
    l3_regions: BTreeMap<u64, MemoryRegion>,

    /// L4: Disk-backed storage
    l4_regions: BTreeMap<u64, MemoryRegion>,

    /// L5: Network storage (handles)
    l5_regions: HashMap<u64, NetworkMemoryHandle>,

    /// Current base addresses for each tier
    tier_bases: HashMap<MemoryTier, u64>,

    /// Next allocation ID
    next_alloc_id: u64,

    /// Memory statistics
    stats: MemoryStats,

    /// Memory limits per tier
    tier_limits: HashMap<MemoryTier, u64>,

    /// Eviction queue (LRU)
    eviction_queue: VecDeque<u64>,

    /// Configuration
    config: TieredMemoryConfig,
}

/// Network memory handle (for distributed memory)
#[derive(Debug, Clone)]
pub struct NetworkMemoryHandle {
    pub node_id: u64,
    pub remote_addr: u64,
    pub size: u64,
    pub cached_locally: bool,
}

/// Configuration for tiered memory
#[derive(Debug, Clone)]
pub struct TieredMemoryConfig {
    /// L1 size limit (bytes)
    pub l1_limit: u64,

    /// L2 size limit (bytes)
    pub l2_limit: u64,

    /// L3 size limit (bytes)
    pub l3_limit: u64,

    /// L4 size limit (bytes)
    pub l4_limit: u64,

    /// Enable automatic tier migration
    pub auto_migrate: bool,

    /// Migration threshold (access count)
    pub migration_threshold: u64,

    /// Defragmentation interval (ms)
    pub defrag_interval_ms: u64,

    /// Enable tier statistics
    pub enable_stats: bool,
}

impl Default for TieredMemoryConfig {
    fn default() -> Self {
        Self {
            l1_limit: 256 * 1024,                 // 256KB
            l2_limit: 64 * 1024 * 1024,          // 64MB
            l3_limit: 4 * 1024 * 1024 * 1024,    // 4GB
            l4_limit: 1 * 1024 * 1024 * 1024,    // 1TB
            auto_migrate: true,
            migration_threshold: 100,
            defrag_interval_ms: 60_000,
            enable_stats: true,
        }
    }
}

impl TieredMemoryConfig {
    /// Configuration for microcontrollers
    pub fn for_microcontroller() -> Self {
        Self {
            l1_limit: 8 * 1024,
            l2_limit: 32 * 1024,
            l3_limit: 0,
            l4_limit: 0,
            auto_migrate: false,
            migration_threshold: 50,
            defrag_interval_ms: 0,
            enable_stats: true,
        }
    }

    /// Configuration for embedded devices
    pub fn for_embedded() -> Self {
        Self {
            l1_limit: 64 * 1024,
            l2_limit: 8 * 1024 * 1024,
            l3_limit: 64 * 1024 * 1024,
            l4_limit: 0,
            auto_migrate: true,
            migration_threshold: 100,
            defrag_interval_ms: 300_000,
            enable_stats: true,
        }
    }

    /// Configuration for edge devices
    pub fn for_edge() -> Self {
        Self {
            l1_limit: 256 * 1024,
            l2_limit: 64 * 1024 * 1024,
            l3_limit: 4 * 1024 * 1024 * 1024,
            l4_limit: 512 * 1024 * 1024 * 1024,
            auto_migrate: true,
            migration_threshold: 100,
            defrag_interval_ms: 60_000,
            enable_stats: true,
        }
    }

    /// Configuration for cloud
    pub fn for_cloud() -> Self {
        Self {
            l1_limit: 256 * 1024 * 1024,
            l2_limit: 1 * 1024 * 1024 * 1024,
            l3_limit: 256 * 1024 * 1024 * 1024,
            l4_limit: 10 * 1024 * 1024 * 1024 * 1024,
            auto_migrate: true,
            migration_threshold: 100,
            defrag_interval_ms: 30_000,
            enable_stats: true,
        }
    }
}

impl Default for TieredMemory {
    fn default() -> Self {
        Self::new(TieredMemoryConfig::default())
    }
}

impl TieredMemory {
    /// Create a new tiered memory manager
    pub fn new(config: TieredMemoryConfig) -> Self {
        let mut tier_limits = HashMap::new();
        tier_limits.insert(MemoryTier::L1Cache, config.l1_limit);
        tier_limits.insert(MemoryTier::L2Heap, config.l2_limit);
        tier_limits.insert(MemoryTier::L3Dram, config.l3_limit);
        tier_limits.insert(MemoryTier::L4Disk, config.l4_limit);

        let mut tier_bases = HashMap::new();
        let mut base = 0x1000_0000_u64;
        for tier in [
            MemoryTier::L1Cache,
            MemoryTier::L2Heap,
            MemoryTier::L3Dram,
            MemoryTier::L4Disk,
        ] {
            tier_bases.insert(tier, base);
            base += tier_limits.get(&tier).copied().unwrap_or(0);
        }

        Self {
            l1_regions: VecDeque::new(),
            l2_regions: VecDeque::new(),
            l3_regions: BTreeMap::new(),
            l4_regions: BTreeMap::new(),
            l5_regions: HashMap::new(),
            tier_bases,
            next_alloc_id: 1,
            stats: MemoryStats::default(),
            tier_limits,
            eviction_queue: VecDeque::new(),
            config,
        }
    }

    /// Create for a specific device tier
    pub fn for_device_tier(tier: crate::scaling::capability::DeviceTier) -> Self {
        let config = match tier {
            crate::scaling::capability::DeviceTier::Microcontroller => {
                TieredMemoryConfig::for_microcontroller()
            }
            crate::scaling::capability::DeviceTier::Embedded => {
                TieredMemoryConfig::for_embedded()
            }
            crate::scaling::capability::DeviceTier::Edge => {
                TieredMemoryConfig::for_edge()
            }
            _ => TieredMemoryConfig::for_cloud(),
        };
        Self::new(config)
    }

    /// Allocate memory with automatic tier selection
    pub fn allocate(&mut self, request: AllocationRequest) -> Result<Allocation, MemoryError> {
        // Determine target tier
        let tier = request.tier.unwrap_or_else(|| MemoryTier::from_size(request.size));

        // Check if tier is available
        if self.tier_limits.get(&tier) == Some(&0) {
            // Try to use next available tier
            let fallback = self.find_available_tier(request.size);
            if let Some(fallback_tier) = fallback {
                return self.allocate_in_tier(request, fallback_tier);
            }
            return Err(MemoryError::TierUnavailable { tier });
        }

        self.allocate_in_tier(request, tier)
    }

    /// Allocate in a specific tier
    fn allocate_in_tier(
        &mut self,
        request: AllocationRequest,
        tier: MemoryTier,
    ) -> Result<Allocation, MemoryError> {
        // Check limit for tier
        if let Some(limit) = self.tier_limits.get(&tier) {
            let used = self.tier_used(tier);
            if used + request.size > *limit {
                // Try to evict
                if self.config.auto_migrate {
                    self.evict_for_tier(tier, request.size)?;
                } else {
                    return Err(MemoryError::OutOfMemory {
                        tier,
                        requested: request.size,
                        available: limit.saturating_sub(used),
                    });
                }
            }
        }

        let addr = self.alloc_in_tier(tier, request.size, request.align)?;
        let id = self.next_alloc_id;
        self.next_alloc_id += 1;

        let region = MemoryRegion {
            addr,
            size: request.size,
            tier,
            allocated: true,
            priority: request.priority,
            ref_count: 1,
            last_access: 0,
        };

        self.add_region(region);
        self.update_stats(tier, request.size as i64);

        Ok(Allocation {
            addr,
            size: request.size,
            tier,
            handle: AllocationHandle { id, tier },
        })
    }

    /// Allocate in a specific tier internal
    fn alloc_in_tier(
        &mut self,
        tier: MemoryTier,
        size: u64,
        _align: Option<u64>,
    ) -> Result<u64, MemoryError> {
        let base = self.tier_bases.get(&tier).copied().unwrap_or(0);
        let offset = self.tier_used(tier);
        Ok(base + offset)
    }

    /// Find an available tier for the given size
    fn find_available_tier(&self, size: u64) -> Option<MemoryTier> {
        for tier in [
            MemoryTier::L1Cache,
            MemoryTier::L2Heap,
            MemoryTier::L3Dram,
            MemoryTier::L4Disk,
        ] {
            if let Some(limit) = self.tier_limits.get(&tier) {
                if *limit > 0 && self.tier_used(tier) + size <= *limit {
                    return Some(tier);
                }
            }
        }
        None
    }

    /// Evict memory to make room in a tier
    fn evict_for_tier(&mut self, tier: MemoryTier, needed: u64) -> Result<(), MemoryError> {
        let mut freed = 0u64;

        while freed < needed {
            if let Some(addr) = self.eviction_queue.pop_front() {
                freed += self.free_region(addr)?;
            } else {
                return Err(MemoryError::OutOfMemory {
                    tier,
                    requested: needed,
                    available: freed,
                });
            }
        }

        Ok(())
    }

    /// Free a memory region
    pub fn free(&mut self, addr: u64) -> Result<u64, MemoryError> {
        self.free_region(addr)
    }

    /// Free a region by address
    fn free_region(&mut self, addr: u64) -> Result<u64, MemoryError> {
        // Find the region
        if let Some(region) = self.find_region(addr) {
            let size = region.size;
            let tier = region.tier;

            // Remove from tier
            match tier {
                MemoryTier::L1Cache => {
                    self.l1_regions.retain(|r| r.addr != addr);
                }
                MemoryTier::L2Heap => {
                    self.l2_regions.retain(|r| r.addr != addr);
                }
                MemoryTier::L3Dram => {
                    self.l3_regions.remove(&addr);
                }
                MemoryTier::L4Disk => {
                    self.l4_regions.remove(&addr);
                }
                MemoryTier::L5Network => {
                    self.l5_regions.remove(&addr);
                }
            }

            self.update_stats(tier, -(size as i64));
            Ok(size)
        } else {
            Err(MemoryError::InvalidAddress { addr })
        }
    }

    /// Find a region by address
    fn find_region(&self, addr: u64) -> Option<MemoryRegion> {
        // Check L1
        for region in &self.l1_regions {
            if region.contains(addr) {
                return Some(region.clone());
            }
        }

        // Check L2
        for region in &self.l2_regions {
            if region.contains(addr) {
                return Some(region.clone());
            }
        }

        // Check L3
        if let Some((_, region)) = self.l3_regions.range(..=addr).last() {
            if region.contains(addr) {
                return Some(region.clone());
            }
        }

        // Check L4
        if let Some((_, region)) = self.l4_regions.range(..=addr).last() {
            if region.contains(addr) {
                return Some(region.clone());
            }
        }

        // Check L5
        if let Some(handle) = self.l5_regions.get(&addr) {
            return Some(MemoryRegion {
                addr,
                size: handle.size,
                tier: MemoryTier::L5Network,
                allocated: true,
                priority: AccessPriority::Normal,
                ref_count: 1,
                last_access: 0,
            });
        }

        None
    }

    /// Add a region to the appropriate tier
    fn add_region(&mut self, region: MemoryRegion) {
        match region.tier {
            MemoryTier::L1Cache => self.l1_regions.push_back(region),
            MemoryTier::L2Heap => self.l2_regions.push_back(region),
            MemoryTier::L3Dram => {
                self.l3_regions.insert(region.addr, region);
            }
            MemoryTier::L4Disk => {
                self.l4_regions.insert(region.addr, region);
            }
            MemoryTier::L5Network => {
                self.l5_regions.insert(
                    region.addr,
                    NetworkMemoryHandle {
                        node_id: 0,
                        remote_addr: region.addr,
                        size: region.size,
                        cached_locally: false,
                    },
                );
            }
        }

        // Add to eviction queue
        self.eviction_queue.push_back(region.addr);
    }

    /// Get used memory for a tier
    fn tier_used(&self, tier: MemoryTier) -> u64 {
        match tier {
            MemoryTier::L1Cache => self.l1_regions.iter().map(|r| r.size).sum(),
            MemoryTier::L2Heap => self.l2_regions.iter().map(|r| r.size).sum(),
            MemoryTier::L3Dram => self.l3_regions.values().map(|r| r.size).sum(),
            MemoryTier::L4Disk => self.l4_regions.values().map(|r| r.size).sum(),
            MemoryTier::L5Network => self.l5_regions.values().map(|h| h.size).sum(),
        }
    }

    /// Update statistics
    fn update_stats(&mut self, tier: MemoryTier, delta: i64) {
        if !self.config.enable_stats {
            return;
        }

        let tier_stats = self.stats.tier_stats.entry(tier).or_default();

        if delta > 0 {
            tier_stats.used_bytes = tier_stats.used_bytes.saturating_add(delta as u64);
            tier_stats.allocation_count += 1;
        } else {
            tier_stats.used_bytes = tier_stats.used_bytes.saturating_sub((-delta) as u64);
            tier_stats.eviction_count += 1;
        }
    }

    /// Get memory statistics
    pub fn stats(&self) -> &MemoryStats {
        &self.stats
    }

    /// Get total memory usage
    pub fn total_used(&self) -> u64 {
        self.stats.tier_stats.values().map(|s| s.used_bytes).sum()
    }

    /// Get total available memory
    pub fn total_available(&self) -> u64 {
        self.tier_limits.values().sum()
    }

    /// Migrate a region to a different tier
    #[allow(dead_code)]
    pub fn migrate(&mut self, addr: u64, target_tier: MemoryTier) -> Result<(), MemoryError> {
        let region = self.find_region(addr).ok_or(MemoryError::InvalidAddress { addr })?;

        if region.tier == target_tier {
            return Ok(());
        }

        // Read data
        let _data = vec![0u8; region.size as usize];

        // Remove from source tier
        match region.tier {
            MemoryTier::L1Cache => {
                self.l1_regions.retain(|r| r.addr != addr);
            }
            MemoryTier::L2Heap => {
                self.l2_regions.retain(|r| r.addr != addr);
            }
            MemoryTier::L3Dram => {
                self.l3_regions.remove(&addr);
            }
            MemoryTier::L4Disk => {
                self.l4_regions.remove(&addr);
            }
            MemoryTier::L5Network => {
                self.l5_regions.remove(&addr);
            }
        }

        // Allocate in target tier
        let new_region = MemoryRegion {
            addr: 0, // Will be set by allocator
            size: region.size,
            tier: target_tier,
            allocated: true,
            priority: region.priority,
            ref_count: region.ref_count,
            last_access: region.last_access,
        };

        // For now, use same address
        let mut new_region = new_region;
        new_region.addr = self.alloc_in_tier(target_tier, new_region.size, None)?;

        self.add_region(new_region);
        self.update_stats(region.tier, -(region.size as i64));
        self.update_stats(target_tier, region.size as i64);

        Ok(())
    }
}

/// Memory errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// Out of memory in tier
    OutOfMemory {
        tier: MemoryTier,
        requested: u64,
        available: u64,
    },

    /// Tier not available
    TierUnavailable {
        tier: MemoryTier,
    },

    /// Invalid address
    InvalidAddress {
        addr: u64,
    },

    /// Allocation failed
    AllocationFailed {
        reason: String,
    },
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::OutOfMemory {
                tier,
                requested,
                available,
            } => write!(
                f,
                "Out of memory in {:?}: requested {} bytes, {} available",
                tier, requested, available
            ),
            MemoryError::TierUnavailable { tier } => {
                write!(f, "Memory tier {:?} is not available", tier)
            }
            MemoryError::InvalidAddress { addr } => {
                write!(f, "Invalid memory address: 0x{:x}", addr)
            }
            MemoryError::AllocationFailed { reason } => {
                write!(f, "Allocation failed: {}", reason)
            }
        }
    }
}

impl std::error::Error for MemoryError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_from_size() {
        assert_eq!(MemoryTier::from_size(1024), MemoryTier::L1Cache);
        assert_eq!(MemoryTier::from_size(1024 * 1024), MemoryTier::L2Heap);
        assert_eq!(MemoryTier::from_size(64 * 1024 * 1024), MemoryTier::L3Dram);
    }

    #[test]
    fn test_tiered_memory_creation() {
        let memory = TieredMemory::new(TieredMemoryConfig::for_embedded());
        assert_eq!(memory.total_used(), 0);
    }

    #[test]
    fn test_allocation() {
        let mut memory = TieredMemory::new(TieredMemoryConfig::for_edge());

        let alloc = memory
            .allocate(AllocationRequest {
                size: 1024,
                tier: None,
                priority: AccessPriority::Normal,
                persistent: false,
                align: None,
            })
            .unwrap();

        assert_eq!(alloc.size, 1024);
        assert!(alloc.addr > 0);
        assert_eq!(memory.total_used(), 1024);
    }

    #[test]
    fn test_free() {
        let mut memory = TieredMemory::new(TieredMemoryConfig::for_edge());

        let alloc = memory
            .allocate(AllocationRequest {
                size: 1024,
                tier: None,
                priority: AccessPriority::Normal,
                persistent: false,
                align: None,
            })
            .unwrap();

        let freed = memory.free(alloc.addr).unwrap();
        assert_eq!(freed, 1024);
        assert_eq!(memory.total_used(), 0);
    }

    #[test]
    fn test_tier_stats() {
        let mut memory = TieredMemory::new(TieredMemoryConfig::for_edge());

        memory
            .allocate(AllocationRequest {
                size: 1024,
                tier: Some(MemoryTier::L2Heap),
                priority: AccessPriority::Normal,
                persistent: false,
                align: None,
            })
            .unwrap();

        let stats = memory.stats();
        let l2_stats = stats.tier_stats.get(&MemoryTier::L2Heap).unwrap();
        assert_eq!(l2_stats.used_bytes, 1024);
        assert_eq!(l2_stats.allocation_count, 1);
    }
}
