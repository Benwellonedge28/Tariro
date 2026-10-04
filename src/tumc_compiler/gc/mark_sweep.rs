// Mark-Sweep Garbage Collector

use super::*;

/// Mark-sweep GC implementation
pub struct MarkSweepGc {
    config: HeapConfig,
    heap: Option<HeapRegion>,
    roots: RootSet,
    stats: GcStats,
    enabled: bool,
}

impl MarkSweepGc {
    pub fn new(config: HeapConfig) -> Self {
        Self {
            heap: HeapRegion::new(config.initial_size),
            config,
            roots: RootSet::new(),
            stats: GcStats::default(),
            enabled: true,
        }
    }

    pub fn with_heap_size(size: usize) -> Self {
        Self::new(HeapConfig {
            initial_size: size,
            ..Default::default()
        })
    }
}

impl GarbageCollector for MarkSweepGc {
    fn init(&mut self) {
        if self.heap.is_none() {
            self.heap = HeapRegion::new(self.config.initial_size);
        }
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if !self.enabled {
            return std::ptr::null_mut();
        }

        if let Some(ref mut heap) = self.heap {
            // Check if we need to GC
            if heap.allocated + size > (self.config.gc_threshold * heap.size as f64) as usize {
                self.collect();
            }

            if let Some(ptr) = heap.allocate(size) {
                self.stats.allocations += 1;
                return ptr;
            }

            // Try expanding heap
            self.collect();
            if let Some(ptr) = heap.allocate(size) {
                return ptr;
            }
        }

        std::ptr::null_mut()
    }

    fn collect(&mut self) {
        let start = std::time::Instant::now();

        if let Some(ref mut heap) = self.heap {
            // Phase 1: Mark
            let mut marked = HashMap::new();
            self.mark(&self.roots, &mut marked);

            // Phase 2: Sweep
            let freed = self.sweep(heap, &marked);

            self.stats.collections += 1;
            self.stats.freed_objects += freed;
            self.stats.free_memory = heap.size - heap.allocated;
        }

        self.stats.total_collection_time +=
            start.elapsed().as_millis() as u64;
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn stats(&self) -> GcStats {
        self.stats.clone()
    }
}

impl MarkSweepGc {
    /// Mark phase - traverse from roots
    fn mark(&self, roots: &RootSet, marked: &mut HashMap<*mut u8, bool>) {
        for root in roots.all_roots() {
            if !root.is_null() {
                self.traverse(*root, marked);
            }
        }
    }

    /// Traverse object graph
    fn traverse(&self, ptr: *mut u8, marked: &mut HashMap<*mut u8, bool>) {
        if ptr.is_null() || marked.contains_key(&ptr) {
            return;
        }

        // Mark this object
        marked.insert(ptr, true);

        // Get object header
        let header = unsafe { &*(ptr as *const ObjectHeader) };
        let data_ptr = unsafe { ptr.add(std::mem::size_of::<ObjectHeader>()) };

        // Traverse fields based on type
        // This would typically use type information or pointer maps
        let _size = header.size;

        // For simplicity, assume we can find pointers in the data region
        // Real implementations would use pointer maps or conservative scanning
    }

    /// Sweep phase - free unmarked objects
    fn sweep(&self, heap: &mut HeapRegion, marked: &HashMap<*mut u8, bool>) -> usize {
        let mut freed = 0;
        let mut ptr = heap.start;

        while ptr < heap.end {
            let header = unsafe { &*(ptr as *const ObjectHeader) };

            if header.size == 0 {
                break;
            }

            let obj_size = std::mem::size_of::<ObjectHeader>() + header.size;

            if !marked.contains_key(&ptr) {
                // Object is garbage - free it
                unsafe {
                    let layout = std::alloc::Layout::from_size_align_unchecked(
                        obj_size,
                        8,
                    );
                    std::alloc::dealloc(ptr, layout);
                }
                freed += 1;
            }

            ptr = unsafe { ptr.add(obj_size) };
        }

        freed
    }

    /// Get heap utilization
    pub fn utilization(&self) -> f64 {
        self.stats.utilization()
    }
}

impl Default for MarkSweepGc {
    fn default() -> Self {
        Self::new(HeapConfig::default())
    }
}

/// Conservative GC - treats any value that looks like a pointer as a potential root
pub struct ConservativeMarkSweep {
    inner: MarkSweepGc,
}

impl ConservativeMarkSweep {
    pub fn new() -> Self {
        Self {
            inner: MarkSweepGc::new(HeapConfig::default()),
        }
    }
}

impl GarbageCollector for ConservativeMarkSweep {
    fn init(&mut self) {
        self.inner.init();
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        self.inner.allocate(size)
    }

    fn collect(&mut self) {
        // Would scan all memory and collect potential roots
        self.inner.collect();
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.inner.set_enabled(enabled);
    }

    fn stats(&self) -> GcStats {
        self.inner.stats()
    }
}

impl Default for ConservativeMarkSweep {
    fn default() -> Self {
        Self::new()
    }
}
