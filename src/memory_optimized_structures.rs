/// Module for Memory-Optimized Data Structures in Rust for RAG/SIMD workflows.
///
/// This module explores conceptual data structures designed for cache-locality,
/// minimal overhead, and efficient access patterns crucial for high-performance
/// RAG context management and SIMD processing.

use std::ops::{Index, IndexMut};

/// A simple packed vector for numerical data, ensuring contiguous memory.
/// This is conceptually similar to how `Vec<T>` works but emphasizes
/// the intention for SIMD-friendly layout.
#[derive(Debug, Clone, PartialEq)]
pub struct PackedVector<T> {
    data: Vec<T>,
}

impl<T> PackedVector<T> {
    /// Creates a new `PackedVector` with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        PackedVector { data: Vec::with_capacity(capacity) }
    }

    /// Appends an element to the end of the vector.
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a slice over the elements of the vector.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable slice over the elements of the vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Index<usize> for PackedVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for PackedVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// A fixed-size circular buffer for managing RAG contexts efficiently.
/// This helps in keeping the most recent N contexts in memory without reallocations.
#[derive(Debug, Clone)]
pub struct ContextBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
    head: usize,
    len: usize,
}

impl<T: Clone> ContextBuffer<T> {
    /// Creates a new `ContextBuffer` with a specified capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be greater than 0.");
        ContextBuffer {
            buffer: Vec::with_capacity(capacity),
            capacity,
            head: 0,
            len: 0,
        }
    }

    /// Pushes an item into the buffer.
    /// If the buffer is full, the oldest item is overwritten.
    pub fn push(&mut self, item: T) {
        if self.len < self.capacity {
            self.buffer.push(item);
            self.len += 1;
        } else {
            // Overwrite the oldest element (at head position)
            self.buffer[self.head] = item;
            self.head = (self.head + 1) % self.capacity;
        }
    }

    /// Returns the number of elements currently in the buffer.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns an iterator over the elements in the buffer, from oldest to newest.
    pub fn iter(&self) -> ContextBufferIterator<T> {
        ContextBufferIterator {
            buffer: self,
            index: 0,
        }
    }
}

pub struct ContextBufferIterator<'a, T> {
    buffer: &'a ContextBuffer<T>,
    index: usize,
}

impl<'a, T: Clone> Iterator for ContextBufferIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.buffer.len {
            let actual_idx = (self.buffer.head + self.index) % self.buffer.capacity;
            self.index += 1;
            Some(self.buffer.buffer[actual_idx].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_vector() {
        let mut pv = PackedVector::with_capacity(4);
        pv.push(1.0f32);
        pv.push(2.0f32);
        assert_eq!(pv.len(), 2);
        assert_eq!(pv[0], 1.0f32);

        pv[0] = 3.0f32;
        assert_eq!(pv[0], 3.0f32);

        let slice = pv.as_slice();
        assert_eq!(slice.len(), 2);
    }

    #[test]
    fn test_context_buffer_push_and_len() {
        let mut cb = ContextBuffer::new(3);
        cb.push("ctx1".to_string());
        cb.push("ctx2".to_string());
        assert_eq!(cb.len(), 2);
        cb.push("ctx3".to_string());
        assert_eq!(cb.len(), 3);
        cb.push("ctx4".to_string()); // Should overwrite ctx1
        assert_eq!(cb.len(), 3);

        let items: Vec<String> = cb.iter().collect();
        assert_eq!(items, vec!["ctx2".to_string(), "ctx3".to_string(), "ctx4".to_string()]);
    }

    #[test]
    fn test_context_buffer_iterator() {
        let mut cb = ContextBuffer::new(2);
        cb.push("A".to_string());
        cb.push("B".to_string());
        cb.push("C".to_string());

        let mut iter = cb.iter();
        assert_eq!(iter.next(), Some("B".to_string()));
        assert_eq!(iter.next(), Some("C".to_string()));
        assert_eq!(iter.next(), None);
    }
}

fn main() {
    println!("--- PackedVector Demonstration ---");
    let mut pv_f32 = PackedVector::with_capacity(8);
    for i in 0..8 { pv_f32.push(i as f32 * 1.1); }
    println!("PackedVector (f32): {:?}", pv_f32.as_slice());
    println!("Length: {}", pv_f32.len());
    pv_f32[3] = 99.9f32;
    println!("Modified PackedVector: {:?}", pv_f32.as_slice());

    let mut pv_u64 = PackedVector::with_capacity(5);
    for i in 0..5 { pv_u64.push(i as u64 * 100); }
    println!("PackedVector (u64): {:?}", pv_u64.as_slice());


    println!("\n--- ContextBuffer Demonstration ---");
    let mut ctx_buffer = ContextBuffer::new(3);
    println!("Buffer empty: {}", ctx_buffer.is_empty());

    ctx_buffer.push("Initial RAG context on Tariro AGI.".to_string());
    ctx_buffer.push("Another context about SIMD processing.".to_string());
    ctx_buffer.push("Third context, about Rust memory safety.".to_string());
    println!("Buffer after 3 pushes (len={}):", ctx_buffer.len());
    for (i, ctx) in ctx_buffer.iter().enumerate() { println!("  {}: '{}'", i + 1, ctx); }

    ctx_buffer.push("New context: Advanced RAG models.".to_string()); // Overwrites oldest
    println!("\nBuffer after 4th push (len={}):", ctx_buffer.len());
    for (i, ctx) in ctx_buffer.iter().enumerate() { println!("  {}: '{}'", i + 1, ctx); }

    ctx_buffer.push("Final context: AI ethics and alignment.".to_string()); // Overwrites next oldest
    println!("\nBuffer after 5th push (len={}):", ctx_buffer.len());
    for (i, ctx) in ctx_buffer.iter().enumerate() { println!("  {}: '{}'", i + 1, ctx); }
}
