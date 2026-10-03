//! TUMC Garbage Collection
//!
//! Memory management and garbage collection for Tariro programs.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Garbage collector configuration
#[derive(Debug, Clone)]
pub struct GcConfig {
    /// Enable incremental collection
    pub incremental: bool,
    /// Enable concurrent collection
    pub concurrent: bool,
    /// Initial heap size
    pub initial_heap_size: usize,
    /// Maximum heap size
    pub max_heap_size: usize,
    /// GC threshold (percentage of heap used)
    pub gc_threshold: f64,
    /// Minor collection ratio
    pub minor_ratio: f64,
    /// Nursery size
    pub nursery_size: usize,
    /// Enable compaction
    pub enable_compaction: bool,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            incremental: true,
            concurrent: true,
            initial_heap_size: 1024 * 1024,
            max_heap_size: 64 * 1024 * 1024,
            gc_threshold: 0.8,
            minor_ratio: 0.1,
            nursery_size: 256 * 1024,
            enable_compaction: true,
        }
    }
}

/// Heap object
#[derive(Debug, Clone)]
pub struct HeapObject {
    /// Object address
    pub address: usize,
    /// Object size
    pub size: usize,
    /// Object kind
    pub kind: ObjectKind,
    /// Header
    pub header: ObjectHeader,
}

/// Object kind
#[derive(Debug, Clone)]
pub enum ObjectKind {
    String,
    Array,
    Struct,
    Closure,
    Tuple,
    Boxed,
}

/// Object header
#[derive(Debug, Clone)]
pub struct ObjectHeader {
    /// Size
    pub size: usize,
    /// Marked flag
    pub marked: bool,
    /// Age
    pub age: u8,
    /// Forwarding pointer
    pub forwarding: Option<usize>,
    /// Object type ID
    pub type_id: u32,
}

/// GC state
#[derive(Debug, Clone)]
pub enum GcState {
    Idle,
    Marking,
    Sweeping,
    Compacting,
}

/// Heap region
#[derive(Debug, Clone)]
pub struct HeapRegion {
    /// Start address
    pub start: usize,
    /// End address
    pub end: usize,
    /// Current allocation point
    pub alloc_ptr: usize,
    /// Free space
    pub free: usize,
    /// Objects
    pub objects: Vec<HeapObject>,
}

impl HeapRegion {
    /// Create new region
    pub fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            end: start + size,
            alloc_ptr: start,
            free: size,
            objects: Vec::new(),
        }
    }

    /// Allocate object
    pub fn allocate(&mut self, size: usize, kind: ObjectKind, type_id: u32) -> Option<usize> {
        if self.free < size + std::mem::size_of::<ObjectHeader>() {
            return None;
        }

        let address = self.alloc_ptr;
        let header = ObjectHeader {
            size,
            marked: false,
            age: 0,
            forwarding: None,
            type_id,
        };

        let header_size = std::mem::size_of::<ObjectHeader>();
        self.alloc_ptr += header_size + size;
        self.free -= header_size + size;

        self.objects.push(HeapObject {
            address,
            size,
            kind,
            header,
        });

        Some(address + header_size)
    }

    /// Check if address is in region
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start && addr < self.end
    }
}

/// Garbage collector
pub struct GarbageCollector {
    /// Configuration
    config: GcConfig,
    /// Heap regions
    nursery: HeapRegion,
    mature: HeapRegion,
    large: Vec<HeapRegion>,
    /// GC state
    state: GcState,
    /// Root set
    roots: HashSet<usize>,
    /// Allocation count
    alloc_count: u64,
    /// Collection count
    collection_count: u64,
    /// Last collection time
    last_collection: Option<Instant>,
    /// Tracing queue
    trace_queue: VecDeque<usize>,
    /// Freed objects
    freed_objects: Vec<usize>,
}

impl GarbageCollector {
    /// Create new GC
    pub fn new(config: GcConfig) -> Self {
        let nursery_size = config.nursery_size;
        let mature_size = config.initial_heap_size - nursery_size;

        Self {
            config: config.clone(),
            nursery: HeapRegion::new(0, nursery_size),
            mature: HeapRegion::new(nursery_size, mature_size),
            large: Vec::new(),
            state: GcState::Idle,
            roots: HashSet::new(),
            alloc_count: 0,
            collection_count: 0,
            last_collection: None,
            trace_queue: VecDeque::new(),
            freed_objects: Vec::new(),
        }
    }

    /// Allocate object
    pub fn allocate(&mut self, size: usize, kind: ObjectKind, type_id: u32) -> Option<usize> {
        self.alloc_count += 1;

        // Check if GC needed
        if self.should_collect() {
            self.collect();
        }

        // Try nursery first (young allocation)
        if size < self.nursery.free {
            return self.nursery.allocate(size, kind, type_id);
        }

        // Try mature space
        if size < self.mature.free {
            return self.mature.allocate(size, kind, type_id);
        }

        // Large allocation
        let region = HeapRegion::new(0, size + 100);
        if let Some(addr) = region.allocate(size, kind, type_id) {
            self.large.push(region);
            return Some(addr);
        }

        // Need GC
        self.collect();
        self.allocate(size, kind, type_id)
    }

    /// Check if collection needed
    fn should_collect(&self) -> bool {
        let nursery_used = self.nursery.alloc_ptr - self.nursery.start;
        let nursery_ratio = nursery_used as f64 / self.nursery.free as f64;
        nursery_ratio > self.config.gc_threshold
    }

    /// Add root
    pub fn add_root(&mut self, addr: usize) {
        self.roots.insert(addr);
    }

    /// Remove root
    pub fn remove_root(&mut self, addr: usize) {
        self.roots.remove(&addr);
    }

    /// Run GC collection
    pub fn collect(&mut self) {
        let start = Instant::now();

        // Phase 1: Mark
        self.state = GcState::Marking;
        self.mark_roots();

        // Phase 2: Sweep
        self.state = GcState::Sweeping;
        self.sweep_unreachable();

        // Phase 3: Compact (if enabled)
        if self.config.enable_compaction {
            self.state = GcState::Compacting;
            self.compact();
        }

        self.state = GcState::Idle;
        self.collection_count += 1;
        self.last_collection = Some(start);
    }

    /// Mark root objects
    fn mark_roots(&mut self) {
        for &root in &self.roots {
            self.mark(root);
        }
    }

    /// Mark object and its references
    fn mark(&mut self, addr: usize) {
        if self.nursery.contains(addr) {
            self.mark_object(&mut self.nursery, addr);
        } else if self.mature.contains(addr) {
            self.mark_object(&mut self.mature, addr);
        }
    }

    /// Mark object in region
    fn mark_object(&mut self, region: &mut HeapRegion, addr: usize) {
        if let Some(obj) = region.objects.iter_mut().find(|o| o.address == addr) {
            if !obj.header.marked {
                obj.header.marked = true;
                // Trace references
                self.trace_references(obj);
            }
        }
    }

    /// Trace object references
    fn trace_references(&mut self, obj: &HeapObject) {
        // Would trace references based on object type
        // For simplicity, we just add to queue
        self.trace_queue.push_back(obj.address);
    }

    /// Sweep unreachable objects
    fn sweep_unreachable(&mut self) {
        // Sweep nursery
        self.sweep_region(&mut self.nursery, true);
        // Sweep mature
        self.sweep_region(&mut self.mature, false);
        // Clean large objects
        self.sweep_large();
    }

    /// Sweep region
    fn sweep_region(&mut self, region: &mut HeapRegion, promote: bool) {
        region.objects.retain(|obj| {
            if obj.header.marked {
                // Keep and potentially promote
                if promote {
                    obj.header.age += 1;
                }
                true
            } else {
                // Free
                self.freed_objects.push(obj.address);
                false
            }
        });

        // Reset marked flag
        for obj in &mut region.objects {
            obj.header.marked = false;
        }
    }

    /// Sweep large objects
    fn sweep_large(&mut self) {
        self.large.retain(|region| {
            if let Some(obj) = region.objects.first() {
                if !obj.header.marked {
                    self.freed_objects.push(obj.address);
                    return false;
                }
            }
            true
        });
    }

    /// Compact heap
    fn compact(&mut self) {
        // Simple compaction - move objects to fill gaps
        let mut new_alloc_ptr = self.nursery.start;
        let objects: Vec<_> = self.nursery.objects.drain(..).collect();

        for mut obj in objects {
            let header_size = std::mem::size_of::<ObjectHeader>();
            let new_address = new_alloc_ptr + header_size;

            if new_address != obj.address {
                // Update forwarding pointer
                obj.header.forwarding = Some(new_address);
            }

            new_alloc_ptr = new_address + obj.size;
            self.nursery.objects.push(obj);
        }

        self.nursery.alloc_ptr = new_alloc_ptr;
    }

    /// Get GC statistics
    pub fn stats(&self) -> GcStats {
        GcStats {
            alloc_count: self.alloc_count,
            collection_count: self.collection_count,
            nursery_size: self.nursery.free,
            mature_size: self.mature.free,
            large_objects: self.large.len(),
            last_collection_ms: self.last_collection
                .map(|t| t.elapsed().as_millis() as u64),
        }
    }

    /// Reset GC
    pub fn reset(&mut self) {
        *self = Self::new(self.config.clone());
    }
}

/// GC statistics
#[derive(Debug)]
pub struct GcStats {
    pub alloc_count: u64,
    pub collection_count: u64,
    pub nursery_size: usize,
    pub mature_size: usize,
    pub large_objects: usize,
    pub last_collection_ms: Option<u64>,
}

/// Allocation budget
pub struct AllocationBudget {
    /// Budget remaining
    remaining: usize,
    /// Initial budget
    initial: usize,
    /// Time quantum
    quantum: Duration,
    /// Start time
    start: Instant,
}

impl AllocationBudget {
    /// Create new budget
    pub fn new(bytes: usize) -> Self {
        Self {
            remaining: bytes,
            initial: bytes,
            quantum: Duration::from_millis(10),
            start: Instant::now(),
        }
    }

    /// Check if allocation allowed
    pub fn can_allocate(&self, bytes: usize) -> bool {
        self.remaining >= bytes
    }

    /// Allocate
    pub fn allocate(&mut self, bytes: usize) -> Result<(), BudgetExceeded> {
        if self.remaining >= bytes {
            self.remaining -= bytes;
            Ok(())
        } else {
            Err(BudgetExceeded {
                requested: bytes,
                remaining: self.remaining,
            })
        }
    }

    /// Refill budget
    pub fn refill(&mut self) {
        let elapsed = self.start.elapsed();
        if elapsed >= self.quantum {
            self.remaining = self.initial;
            self.start = Instant::now();
        }
    }
}

/// Budget exceeded error
#[derive(Debug)]
pub struct BudgetExceeded {
    pub requested: usize,
    pub remaining: usize,
}

/// Memory tracker
pub struct MemoryTracker {
    /// Tracked allocations
    allocations: HashMap<usize, AllocationInfo>,
    /// Total allocated
    total_allocated: usize,
    /// Peak usage
    peak_usage: usize,
    /// Threshold
    threshold: usize,
}

#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub size: usize,
    pub address: usize,
    pub timestamp: Instant,
    pub stack_trace: Vec<String>,
}

impl MemoryTracker {
    /// Create new tracker
    pub fn new(threshold: usize) -> Self {
        Self {
            allocations: HashMap::new(),
            total_allocated: 0,
            peak_usage: 0,
            threshold,
        }
    }

    /// Track allocation
    pub fn track(&mut self, addr: usize, size: usize) {
        self.allocations.insert(addr, AllocationInfo {
            size,
            address: addr,
            timestamp: Instant::now(),
            stack_trace: Vec::new(),
        });
        self.total_allocated += size;
        if self.total_allocated > self.peak_usage {
            self.peak_usage = self.total_allocated;
        }
    }

    /// Untrack deallocation
    pub fn untrack(&mut self, addr: usize) {
        if let Some(info) = self.allocations.remove(&addr) {
            self.total_allocated -= info.size;
        }
    }

    /// Check threshold
    pub fn check_threshold(&self) -> bool {
        self.total_allocated > self.threshold
    }

    /// Get report
    pub fn report(&self) -> MemoryReport {
        MemoryReport {
            total_allocated: self.total_allocated,
            peak_usage: self.peak_usage,
            allocation_count: self.allocations.len(),
            threshold: self.threshold,
        }
    }
}

/// Memory report
#[derive(Debug)]
pub struct MemoryReport {
    pub total_allocated: usize,
    pub peak_usage: usize,
    pub allocation_count: usize,
    pub threshold: usize,
}

/// GC-safe wrapper
pub struct Gc<T: 'static + Clone> {
    /// Object address
    address: usize,
    /// Phantom data
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone> Gc<T> {
    /// Create new GC object
    pub fn new(value: T, gc: &mut GarbageCollector, type_id: u32) -> Option<Self> {
        let size = std::mem::size_of::<T>();
        let kind = ObjectKind::Boxed;
        let addr = gc.allocate(size, kind, type_id)?;

        Some(Self {
            address: addr,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Get address
    pub fn address(&self) -> usize {
        self.address
    }
}

impl<T: Clone> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            address: self.address,
            _phantom: self._phantom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_creation() {
        let config = GcConfig::default();
        let gc = GarbageCollector::new(config);
        let stats = gc.stats();
        assert_eq!(stats.collection_count, 0);
    }

    #[test]
    fn test_allocation() {
        let mut gc = GarbageCollector::new(GcConfig::default());
        let addr = gc.allocate(100, ObjectKind::String, 0);
        assert!(addr.is_some());
    }

    #[test]
    fn test_budget() {
        let mut budget = AllocationBudget::new(1000);
        assert!(budget.can_allocate(500));
        budget.allocate(500).unwrap();
        assert!(!budget.can_allocate(600));
    }

    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new(1000);
        tracker.track(0x1000, 100);
        assert!(!tracker.check_threshold());
        tracker.track(0x2000, 1000);
        assert!(tracker.check_threshold());
    }
}
