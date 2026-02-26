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
}
