//! # LRU Cache
//!
//! A Least Recently Used cache combining a HashMap with a Vec-based doubly
//! linked list.  Every `get` and `put` runs in O(1) amortised time.

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Node — an entry in the doubly linked list stored inside a Vec
// ---------------------------------------------------------------------------

/// A single node that lives inside `LRUCache::nodes`.
///
/// `prev` and `next` are indices into the same `Vec`, forming a doubly linked
/// list without any `unsafe` code.
#[derive(Debug)]
struct Node {
    key: i32,
    value: i32,
    prev: usize,
    next: usize,
}

// ---------------------------------------------------------------------------
// LRUCache
// ---------------------------------------------------------------------------

/// A fixed-capacity cache that evicts the **least recently used** entry when
/// full.
///
/// Internally it keeps:
/// - a `Vec<Node>` acting as a doubly linked list (indices 0 and 1 are
///   sentinel head/tail nodes),
/// - a `HashMap<i32, usize>` mapping keys → node indices.
///
/// **Time:** O(1) amortised for `get` and `put`.
/// **Space:** O(capacity).
#[derive(Debug)]
pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    // Index 0 = dummy head, Index 1 = dummy tail.
    // Most recent is right after head; least recent is right before tail.
}

impl LRUCache {
    /// Creates a new LRU cache that holds at most `capacity` entries.
    ///
    /// **Time:** O(1). **Space:** O(capacity).
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "LRU cache capacity must be > 0");

        let mut nodes = Vec::with_capacity(capacity + 2);

        // Sentinel head (index 0)
        nodes.push(Node {
            key: 0,
            value: 0,
            prev: 0,
            next: 1,
        });
        // Sentinel tail (index 1)
        nodes.push(Node {
            key: 0,
            value: 0,
            prev: 0,
            next: 1,
        });

        Self {
            capacity,
            map: HashMap::new(),
            nodes,
        }
    }

    /// Returns the value for `key` and marks it as most recently used.
    ///
    /// **Time:** O(1). **Space:** O(1).
    pub fn get(&mut self, key: i32) -> Option<i32> {
        if let Some(&idx) = self.map.get(&key) {
            let value = self.nodes[idx].value;
            self.detach(idx);
            self.attach_after_head(idx);
            Some(value)
        } else {
            None
        }
    }

    /// Inserts or updates `key` with `value`.
    ///
    /// If the cache is already at capacity and `key` is new, the least
    /// recently used entry is evicted first.
    ///
    /// **Time:** O(1) amortised. **Space:** O(1).
    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.map.get(&key) {
            // Key exists — update value and move to front.
            self.nodes[idx].value = value;
            self.detach(idx);
            self.attach_after_head(idx);
        } else {
            // Evict if at capacity.
            if self.map.len() == self.capacity {
                self.evict_lru();
            }
            // Create a new node.
            let idx = self.nodes.len();
            self.nodes.push(Node {
                key,
                value,
                prev: 0,
                next: 0,
            });
            self.map.insert(key, idx);
            self.attach_after_head(idx);
        }
    }

    /// Returns the number of entries currently in the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the cache contains no entries.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    // ---- internal helpers ----

    /// Remove node `idx` from the linked list (does NOT remove from map).
    fn detach(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }

    /// Insert node `idx` right after the head sentinel (position = most recent).
    fn attach_after_head(&mut self, idx: usize) {
        let head = 0;
        let old_first = self.nodes[head].next;

        self.nodes[idx].prev = head;
        self.nodes[idx].next = old_first;

        self.nodes[head].next = idx;
        self.nodes[old_first].prev = idx;
    }

    /// Evict the least recently used entry (the node right before tail).
    fn evict_lru(&mut self) {
        let tail = 1;
        let lru_idx = self.nodes[tail].prev;
        debug_assert!(lru_idx != 0, "cannot evict from empty cache");

        self.detach(lru_idx);
        let evicted_key = self.nodes[lru_idx].key;
        self.map.remove(&evicted_key);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- new ----

    #[test]
    fn new_cache_is_empty() {
        let cache = LRUCache::new(5);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    #[should_panic(expected = "capacity must be > 0")]
    fn new_zero_capacity_panics() {
        LRUCache::new(0);
    }

    // ---- put ----

    #[test]
    fn put_single_entry() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(1), Some(10));
    }

    #[test]
    fn put_updates_existing_key() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(1, 20);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(1), Some(20));
    }

    #[test]
    fn put_multiple_entries() {
        let mut cache = LRUCache::new(3);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    // ---- get ----

    #[test]
    fn get_missing_key_returns_none() {
        let mut cache = LRUCache::new(2);
        assert_eq!(cache.get(42), None);
    }

    #[test]
    fn get_moves_entry_to_most_recent() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        // Access key 1 so it becomes most recent.
        assert_eq!(cache.get(1), Some(10));
        // Adding key 3 should evict key 2 (the LRU), not key 1.
        cache.put(3, 30);
        assert_eq!(cache.get(2), None); // evicted
        assert_eq!(cache.get(1), Some(10)); // still here
        assert_eq!(cache.get(3), Some(30)); // just added
    }

    // ---- eviction ----

    #[test]
    fn evicts_least_recently_used() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30); // evicts key 1
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn eviction_chain() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(3, 3); // evicts 1
        cache.put(4, 4); // evicts 2
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(3));
        assert_eq!(cache.get(4), Some(4));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn update_existing_prevents_wrong_eviction() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(1, 15); // update key 1 — it becomes most recent
        cache.put(3, 30); // should evict key 2 (LRU), not key 1
        assert_eq!(cache.get(1), Some(15));
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(30));
    }

    // ---- capacity 1 ----

    #[test]
    fn capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 10);
        assert_eq!(cache.get(1), Some(10));

        cache.put(2, 20); // evicts key 1
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn capacity_one_update() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 10);
        cache.put(1, 20);
        assert_eq!(cache.get(1), Some(20));
        assert_eq!(cache.len(), 1);
    }

    // ---- leetcode-style sequence (problem 146) ----

    #[test]
    fn leetcode_146_example() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), Some(1));
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), None);
        cache.put(4, 4); // evicts key 1
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(3), Some(3));
        assert_eq!(cache.get(4), Some(4));
    }
}
