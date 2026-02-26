//! # Priority Queue
//!
//! A max-priority queue built on top of our binary heap implementation.

use crate::heap::BinaryHeap;

/// A priority queue that always dequeues the largest element first.
#[derive(Debug)]
pub struct PriorityQueue<T: Ord> {
    heap: BinaryHeap<T>,
}

impl<T: Ord> PriorityQueue<T> {
    /// Creates an empty priority queue.
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    /// Enqueues a value. Higher values have higher priority. O(log n).
    pub fn enqueue(&mut self, val: T) {
        self.heap.push(val);
    }

    /// Dequeues and returns the highest-priority element, or `None` if empty. O(log n).
    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop()
    }

    /// Returns a reference to the highest-priority element without removing it. O(1).
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek()
    }

    /// Returns `true` if the priority queue is empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns the number of elements in the priority queue.
    pub fn size(&self) -> usize {
        self.heap.size()
    }
}

impl<T: Ord> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pq_is_empty() {
        let pq: PriorityQueue<i32> = PriorityQueue::new();
        assert!(pq.is_empty());
        assert_eq!(pq.size(), 0);
    }

    #[test]
    fn enqueue_and_dequeue() {
        let mut pq = PriorityQueue::new();
        pq.enqueue(3);
        pq.enqueue(1);
        pq.enqueue(5);
        assert_eq!(pq.dequeue(), Some(5));
        assert_eq!(pq.dequeue(), Some(3));
        assert_eq!(pq.dequeue(), Some(1));
        assert_eq!(pq.dequeue(), None);
    }

    #[test]
    fn peek_returns_highest() {
        let mut pq = PriorityQueue::new();
        pq.enqueue(10);
        pq.enqueue(20);
        assert_eq!(pq.peek(), Some(&20));
        assert_eq!(pq.size(), 2); // peek doesn't remove
    }

    #[test]
    fn size_tracks() {
        let mut pq = PriorityQueue::new();
        pq.enqueue(1);
        pq.enqueue(2);
        assert_eq!(pq.size(), 2);
        pq.dequeue();
        assert_eq!(pq.size(), 1);
    }

    #[test]
    fn dequeue_empty() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        assert_eq!(pq.dequeue(), None);
    }

    #[test]
    fn priority_order_with_strings() {
        let mut pq = PriorityQueue::new();
        pq.enqueue("apple");
        pq.enqueue("cherry");
        pq.enqueue("banana");
        assert_eq!(pq.dequeue(), Some("cherry"));
        assert_eq!(pq.dequeue(), Some("banana"));
        assert_eq!(pq.dequeue(), Some("apple"));
    }
}
