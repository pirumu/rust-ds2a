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

/// A min-heap: the smallest element is always at the top.
///
/// This is a convenience wrapper around `BinaryHeap` with `HeapType::Min`.
/// API matches the chapter's teaching examples (`insert`, `delete_min`, `from_vec`).
#[derive(Debug)]
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    /// Creates an empty min-heap.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Creates a min-heap from an existing Vec in O(n) using heapify.
    pub fn from_vec(data: Vec<T>) -> Self {
        let mut heap = Self { data };
        let last_parent = heap.data.len() / 2;
        for i in (0..last_parent).rev() {
            heap.sift_down(i);
        }
        heap
    }

    /// Inserts a value into the heap. O(log n).
    pub fn insert(&mut self, val: T) {
        self.data.push(val);
        self.sift_up(self.data.len() - 1);
    }

    /// Removes and returns the smallest element. O(log n).
    pub fn delete_min(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }

    /// Returns a reference to the smallest element without removing it. O(1).
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    /// Returns `true` if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the heap.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }
    fn left(i: usize) -> usize {
        2 * i + 1
    }
    fn right(i: usize) -> usize {
        2 * i + 2
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let p = Self::parent(idx);
            if self.data[idx] >= self.data[p] {
                break;
            }
            self.data.swap(idx, p);
            idx = p;
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.data.len();
        loop {
            let left = Self::left(idx);
            let right = Self::right(idx);
            let mut smallest = idx;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }

            if smallest == idx {
                break;
            }
            self.data.swap(idx, smallest);
            idx = smallest;
        }
    }
}

impl<T: Ord> Default for MinHeap<T> {
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

    // --- MinHeap tests ---

    #[test]
    fn min_heap_basic() {
        let mut heap = MinHeap::new();
        heap.insert(5);
        heap.insert(3);
        heap.insert(7);
        heap.insert(1);

        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.size(), 4);

        assert_eq!(heap.delete_min(), Some(1));
        assert_eq!(heap.delete_min(), Some(3));
        assert_eq!(heap.delete_min(), Some(5));
        assert_eq!(heap.delete_min(), Some(7));
        assert_eq!(heap.delete_min(), None);
    }

    #[test]
    fn min_heap_from_vec() {
        let data = vec![5, 3, 8, 1, 2, 7, 4];
        let mut heap = MinHeap::from_vec(data);

        assert_eq!(heap.peek(), Some(&1));
        assert_eq!(heap.size(), 7);

        // Should come out sorted
        let mut sorted = Vec::new();
        while let Some(v) = heap.delete_min() {
            sorted.push(v);
        }
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn min_heap_top_k() {
        let scores = vec![85, 92, 67, 45, 78, 93, 12, 56, 88, 34];
        let mut heap = MinHeap::from_vec(scores);

        let top3: Vec<i32> = (0..3).filter_map(|_| heap.delete_min()).collect();
        assert_eq!(top3, vec![12, 34, 45]);
    }

    #[test]
    fn min_heap_heapsort() {
        fn heapsort<T: Ord>(data: Vec<T>) -> Vec<T> {
            let mut heap = MinHeap::from_vec(data);
            let mut sorted = Vec::with_capacity(heap.size());
            while let Some(val) = heap.delete_min() {
                sorted.push(val);
            }
            sorted
        }

        let data = vec![5, 3, 8, 1, 2, 7, 4];
        assert_eq!(heapsort(data), vec![1, 2, 3, 4, 5, 7, 8]);
    }

    #[test]
    fn min_heap_empty() {
        let mut heap: MinHeap<i32> = MinHeap::new();
        assert!(heap.is_empty());
        assert_eq!(heap.peek(), None);
        assert_eq!(heap.delete_min(), None);
    }

    #[test]
    fn min_heap_single() {
        let mut heap = MinHeap::new();
        heap.insert(42);
        assert_eq!(heap.peek(), Some(&42));
        assert_eq!(heap.delete_min(), Some(42));
        assert!(heap.is_empty());
    }

    #[test]
    fn min_heap_sorted_input() {
        let mut heap = MinHeap::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(heap.delete_min(), Some(1));
        assert_eq!(heap.delete_min(), Some(2));
        assert_eq!(heap.delete_min(), Some(3));
    }

    #[test]
    fn min_heap_reverse_input() {
        let mut heap = MinHeap::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(heap.peek(), Some(&1));
        let mut sorted = Vec::new();
        while let Some(v) = heap.delete_min() {
            sorted.push(v);
        }
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn min_heap_duplicates() {
        let mut heap = MinHeap::new();
        heap.insert(3);
        heap.insert(3);
        heap.insert(1);
        heap.insert(1);
        assert_eq!(heap.delete_min(), Some(1));
        assert_eq!(heap.delete_min(), Some(1));
        assert_eq!(heap.delete_min(), Some(3));
        assert_eq!(heap.delete_min(), Some(3));
    }
}
