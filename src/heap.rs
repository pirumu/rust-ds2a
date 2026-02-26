//! # Binary Heap
//!
//! An array-backed binary heap supporting both max-heap and min-heap variants.

use std::cmp::Ordering;

/// Ordering mode for the heap.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeapType {
    Max,
    Min,
}

/// An array-backed binary heap.
#[derive(Debug)]
pub struct BinaryHeap<T: Ord> {
    data: Vec<T>,
    heap_type: HeapType,
}

impl<T: Ord> BinaryHeap<T> {
    /// Creates an empty max-heap.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            heap_type: HeapType::Max,
        }
    }

    /// Creates an empty heap of the specified type.
    pub fn with_type(heap_type: HeapType) -> Self {
        Self {
            data: Vec::new(),
            heap_type,
        }
    }

    /// Creates a heap from a vector using the heapify algorithm. O(n).
    pub fn heapify(vec: Vec<T>) -> Self {
        let mut heap = Self {
            data: vec,
            heap_type: HeapType::Max,
        };
        let len = heap.data.len();
        if len > 1 {
            // Start from the last non-leaf node and sift down.
            for i in (0..len / 2).rev() {
                heap.sift_down(i);
            }
        }
        heap
    }

    /// Pushes a value onto the heap. O(log n).
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        let last = self.data.len() - 1;
        self.sift_up(last);
    }

    /// Removes and returns the top element (max for max-heap, min for min-heap).
    /// Returns `None` if the heap is empty. O(log n).
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let top = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        top
    }

    /// Returns a reference to the top element without removing it. O(1).
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    /// Returns the number of elements in the heap.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns whether `a` should be above `b` in the heap.
    fn should_be_above(&self, a: &T, b: &T) -> bool {
        match self.heap_type {
            HeapType::Max => a > b,
            HeapType::Min => a < b,
        }
    }

    /// Compare two elements according to heap ordering.
    fn heap_cmp(&self, a: &T, b: &T) -> Ordering {
        match self.heap_type {
            HeapType::Max => a.cmp(b),
            HeapType::Min => b.cmp(a),
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.should_be_above(&self.data[idx], &self.data[parent]) {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.data.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut best = idx;

            if left < len && self.heap_cmp(&self.data[left], &self.data[best]) == Ordering::Greater
            {
                best = left;
            }
            if right < len
                && self.heap_cmp(&self.data[right], &self.data[best]) == Ordering::Greater
            {
                best = right;
            }

            if best != idx {
                self.data.swap(idx, best);
                idx = best;
            } else {
                break;
            }
        }
    }
}

impl<T: Ord> Default for BinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_heap_is_empty() {
        let heap: BinaryHeap<i32> = BinaryHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.size(), 0);
        assert_eq!(heap.peek(), None);
    }

    #[test]
    fn push_and_peek_max() {
        let mut heap = BinaryHeap::new();
        heap.push(3);
        heap.push(1);
        heap.push(5);
        assert_eq!(heap.peek(), Some(&5));
    }

    #[test]
    fn pop_returns_max() {
        let mut heap = BinaryHeap::new();
        for v in [3, 1, 5, 2, 4] {
            heap.push(v);
        }
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn min_heap() {
        let mut heap = BinaryHeap::with_type(HeapType::Min);
        for v in [3, 1, 5, 2, 4] {
            heap.push(v);
        }
        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
    }

    #[test]
    fn heapify() {
        let heap = BinaryHeap::heapify(vec![4, 1, 7, 3, 8, 2, 5]);
        assert_eq!(heap.peek(), Some(&8));
        assert_eq!(heap.size(), 7);
    }

    #[test]
    fn size_tracks_correctly() {
        let mut heap = BinaryHeap::new();
        heap.push(1);
        heap.push(2);
        assert_eq!(heap.size(), 2);
        heap.pop();
        assert_eq!(heap.size(), 1);
    }

    #[test]
    fn pop_empty() {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn heapify_sorted_input() {
        let mut heap = BinaryHeap::heapify(vec![1, 2, 3, 4, 5]);
        let mut result = Vec::new();
        while let Some(v) = heap.pop() {
            result.push(v);
        }
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn single_element() {
        let mut heap = BinaryHeap::new();
        heap.push(42);
        assert_eq!(heap.peek(), Some(&42));
        assert_eq!(heap.pop(), Some(42));
        assert!(heap.is_empty());
    }

    #[test]
    fn duplicate_values() {
        let mut heap = BinaryHeap::new();
        heap.push(5);
        heap.push(5);
        heap.push(5);
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), Some(5));
        assert_eq!(heap.pop(), None);
    }
}
