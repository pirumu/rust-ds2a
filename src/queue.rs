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

/// A sliding-window rate limiter backed by a queue of timestamps.
///
/// Allows at most `max_requests` within a rolling `window_secs` window.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    timestamps: VecDeque<f64>,
    max_requests: usize,
    window_secs: f64,
}

impl RateLimiter {
    /// Creates a new rate limiter.
    ///
    /// # Example
    /// ```
    /// use rust_ds2a::queue::RateLimiter;
    /// let mut limiter = RateLimiter::new(5, 10.0); // 5 requests per 10 seconds
    /// assert!(limiter.allow_request(0.0));
    /// ```
    pub fn new(max_requests: usize, window_secs: f64) -> Self {
        Self {
            timestamps: VecDeque::new(),
            max_requests,
            window_secs,
        }
    }

    /// Checks whether a request at time `now` is allowed.
    ///
    /// Returns `true` if the request is within the rate limit, `false` otherwise.
    pub fn allow_request(&mut self, now: f64) -> bool {
        // Remove expired timestamps from the front
        while let Some(&oldest) = self.timestamps.front() {
            if now - oldest > self.window_secs {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        if self.timestamps.len() < self.max_requests {
            self.timestamps.push_back(now);
            true
        } else {
            false
        }
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

    #[test]
    fn rate_limiter_allows_within_limit() {
        let mut rl = RateLimiter::new(3, 10.0);
        assert!(rl.allow_request(0.0));
        assert!(rl.allow_request(1.0));
        assert!(rl.allow_request(2.0));
    }

    #[test]
    fn rate_limiter_rejects_over_limit() {
        let mut rl = RateLimiter::new(3, 10.0);
        assert!(rl.allow_request(0.0));
        assert!(rl.allow_request(1.0));
        assert!(rl.allow_request(2.0));
        assert!(!rl.allow_request(3.0)); // 4th request within window → reject
    }

    #[test]
    fn rate_limiter_allows_after_window_expires() {
        let mut rl = RateLimiter::new(3, 10.0);
        assert!(rl.allow_request(0.0));
        assert!(rl.allow_request(1.0));
        assert!(rl.allow_request(2.0));
        assert!(!rl.allow_request(5.0)); // still within window
        // After 10s, first request expires
        assert!(rl.allow_request(11.0)); // t=0.0 expired → slot freed
    }

    #[test]
    fn rate_limiter_sliding_window() {
        let mut rl = RateLimiter::new(2, 5.0);
        assert!(rl.allow_request(0.0));
        assert!(rl.allow_request(3.0));
        assert!(!rl.allow_request(4.0)); // 2 within [0,5] → reject
        assert!(rl.allow_request(6.0));  // t=0.0 expired → allow
    }
}
