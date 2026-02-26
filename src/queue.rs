//! # Queue
//!
//! A first-in, first-out (FIFO) queue backed by `VecDeque`.

use std::collections::VecDeque;

/// A generic FIFO queue backed by a `VecDeque<T>`.
#[derive(Debug, Clone)]
pub struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    /// Adds a value to the back of the queue. O(1) amortized.
    pub fn enqueue(&mut self, val: T) {
        self.data.push_back(val);
    }

    /// Removes and returns the value at the front of the queue, or `None` if empty. O(1).
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Returns a reference to the front element without removing it, or `None` if empty. O(1).
    pub fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    /// Returns `true` if the queue contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the number of elements in the queue.
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue_is_empty() {
        let q: Queue<i32> = Queue::new();
        assert!(q.is_empty());
        assert_eq!(q.size(), 0);
    }

    #[test]
    fn enqueue_and_dequeue() {
        let mut q = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn peek_returns_front() {
        let mut q = Queue::new();
        q.enqueue(10);
        q.enqueue(20);
        assert_eq!(q.peek(), Some(&10));
        assert_eq!(q.size(), 2); // peek does not remove
    }

    #[test]
    fn peek_empty() {
        let q: Queue<i32> = Queue::new();
        assert_eq!(q.peek(), None);
    }

    #[test]
    fn fifo_order() {
        let mut q = Queue::new();
        for i in 0..5 {
            q.enqueue(i);
        }
        let mut result = Vec::new();
        while let Some(v) = q.dequeue() {
            result.push(v);
        }
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn size_tracks_correctly() {
        let mut q = Queue::new();
        q.enqueue('x');
        q.enqueue('y');
        assert_eq!(q.size(), 2);
        q.dequeue();
        assert_eq!(q.size(), 1);
        q.dequeue();
        assert_eq!(q.size(), 0);
        assert!(q.is_empty());
    }
}
