//! # Deque (Double-Ended Queue)
//!
//! A double-ended queue backed by `VecDeque`, supporting efficient
//! insertion and removal at both ends.

use std::collections::VecDeque;

/// A generic double-ended queue backed by a `VecDeque<T>`.
#[derive(Debug, Clone)]
pub struct Deque<T> {
    data: VecDeque<T>,
}

impl<T> Deque<T> {
    /// Creates an empty deque.
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    /// Inserts a value at the front. O(1) amortized.
    pub fn push_front(&mut self, val: T) {
        self.data.push_front(val);
    }

    /// Inserts a value at the back. O(1) amortized.
    pub fn push_back(&mut self, val: T) {
        self.data.push_back(val);
    }

    /// Removes and returns the front value, or `None` if empty. O(1).
    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Removes and returns the back value, or `None` if empty. O(1).
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Returns a reference to the front element, or `None` if empty. O(1).
    pub fn peek_front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Returns a reference to the back element, or `None` if empty. O(1).
    pub fn peek_back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Returns `true` if the deque contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the deque.
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Finds the maximum value in each sliding window of size `k`.
///
/// Uses a monotonic deque to achieve O(n) time instead of brute-force O(nk).
/// The deque stores indices, keeping values in decreasing order from front to back.
pub fn sliding_window_max(nums: &[i32], k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return vec![];
    }
    let k = k.min(nums.len());
    let mut result = Vec::with_capacity(nums.len() - k + 1);
    // Deque stores indices; values at those indices are in decreasing order
    let mut dq: VecDeque<usize> = VecDeque::new();

    for i in 0..nums.len() {
        // Remove indices that have fallen out of the window
        if let Some(&front) = dq.front() {
            if front + k <= i {
                dq.pop_front();
            }
        }

        // Remove indices whose values are smaller than nums[i]
        // (they will never be the max while nums[i] is in the window)
        while let Some(&back) = dq.back() {
            if nums[back] <= nums[i] {
                dq.pop_back();
            } else {
                break;
            }
        }

        dq.push_back(i);

        // Once we've processed at least k elements, record the window max
        if i >= k - 1 {
            result.push(nums[dq[0]]);
        }
    }

    result
}

/// A bounded deque that automatically removes the oldest element (front)
/// when the capacity is exceeded. Useful for browser history, log buffers, etc.
#[derive(Debug, Clone)]
pub struct BoundedDeque<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> BoundedDeque<T> {
    /// Creates a bounded deque with the given maximum capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Adds a value to the back. If full, the front element is removed first.
    pub fn push_back(&mut self, val: T) {
        if self.data.len() == self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(val);
    }

    /// Removes and returns the back element.
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    /// Returns a reference to the front element.
    pub fn peek_front(&self) -> Option<&T> {
        self.data.front()
    }

    /// Returns a reference to the back element.
    pub fn peek_back(&self) -> Option<&T> {
        self.data.back()
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_deque_is_empty() {
        let d: Deque<i32> = Deque::new();
        assert!(d.is_empty());
        assert_eq!(d.size(), 0);
    }

    #[test]
    fn push_front_and_pop_front() {
        let mut d = Deque::new();
        d.push_front(1);
        d.push_front(2);
        d.push_front(3);
        assert_eq!(d.pop_front(), Some(3));
        assert_eq!(d.pop_front(), Some(2));
        assert_eq!(d.pop_front(), Some(1));
        assert_eq!(d.pop_front(), None);
    }

    #[test]
    fn push_back_and_pop_back() {
        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_back(3);
        assert_eq!(d.pop_back(), Some(3));
        assert_eq!(d.pop_back(), Some(2));
        assert_eq!(d.pop_back(), Some(1));
        assert_eq!(d.pop_back(), None);
    }

    #[test]
    fn mixed_push_pop() {
        let mut d = Deque::new();
        d.push_back(1);
        d.push_back(2);
        d.push_front(0);
        // [0, 1, 2]
        assert_eq!(d.pop_front(), Some(0));
        assert_eq!(d.pop_back(), Some(2));
        assert_eq!(d.pop_front(), Some(1));
        assert!(d.is_empty());
    }

    #[test]
    fn peek_front_and_back() {
        let mut d = Deque::new();
        d.push_back(10);
        d.push_back(20);
        d.push_back(30);
        assert_eq!(d.peek_front(), Some(&10));
        assert_eq!(d.peek_back(), Some(&30));
        assert_eq!(d.size(), 3); // peeks do not remove
    }

    #[test]
    fn peek_empty() {
        let d: Deque<i32> = Deque::new();
        assert_eq!(d.peek_front(), None);
        assert_eq!(d.peek_back(), None);
    }

    #[test]
    fn size_tracks_correctly() {
        let mut d = Deque::new();
        d.push_front(1);
        d.push_back(2);
        assert_eq!(d.size(), 2);
        d.pop_front();
        assert_eq!(d.size(), 1);
        d.pop_back();
        assert_eq!(d.size(), 0);
        assert!(d.is_empty());
    }

    #[test]
    fn sliding_window_max_basic() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(sliding_window_max(&nums, 3), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn sliding_window_max_k_equals_len() {
        let nums = vec![4, 2, 5, 1];
        assert_eq!(sliding_window_max(&nums, 4), vec![5]);
    }

    #[test]
    fn sliding_window_max_k_is_one() {
        let nums = vec![3, 1, 4, 1, 5];
        assert_eq!(sliding_window_max(&nums, 1), vec![3, 1, 4, 1, 5]);
    }

    #[test]
    fn sliding_window_max_empty() {
        assert_eq!(sliding_window_max(&[], 3), Vec::<i32>::new());
    }

    #[test]
    fn bounded_deque_evicts_oldest() {
        let mut bd = BoundedDeque::new(3);
        bd.push_back("a");
        bd.push_back("b");
        bd.push_back("c");
        // Full: ["a", "b", "c"]
        bd.push_back("d");
        // "a" evicted: ["b", "c", "d"]
        assert_eq!(bd.peek_front(), Some(&"b"));
        assert_eq!(bd.peek_back(), Some(&"d"));
        assert_eq!(bd.len(), 3);
    }

    #[test]
    fn bounded_deque_pop_back() {
        let mut bd = BoundedDeque::new(2);
        bd.push_back(1);
        bd.push_back(2);
        assert_eq!(bd.pop_back(), Some(2));
        assert_eq!(bd.len(), 1);
    }
}
