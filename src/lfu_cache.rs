//! # LFU Cache
//!
//! A Least Frequently Used cache that evicts the item with the lowest access
//! frequency.  When multiple items share the same frequency, the least recently
//! used among them is evicted (LRU tiebreaker).

use std::collections::HashMap;

// ---------------------------------------------------------------------------
// FreqNode — one entry inside a frequency bucket's doubly-linked list
// ---------------------------------------------------------------------------

/// Internal node stored inside each frequency bucket.
///
/// The doubly-linked list is implemented with indices into a `Vec<Node>`
/// (arena allocation) to keep things simple and avoid `unsafe`.
struct Node {
    key: i32,
    value: i32,
    freq: usize,
    /// Index of the previous node in the same frequency bucket (or `usize::MAX`).
    prev: usize,
    /// Index of the next node in the same frequency bucket (or `usize::MAX`).
    next: usize,
}

const NONE: usize = usize::MAX;

// ---------------------------------------------------------------------------
// FreqBucket — a doubly-linked list of nodes that share the same frequency
// ---------------------------------------------------------------------------

/// A bucket holding all nodes that share a particular access frequency.
///
/// Nodes are ordered by recency: `head` is the *least* recently used,
/// `tail` is the *most* recently used.  This makes eviction O(1) — just
/// remove from the head.
struct FreqBucket {
    head: usize,
    tail: usize,
}

impl FreqBucket {
    fn new() -> Self {
        Self {
            head: NONE,
            tail: NONE,
        }
    }

    fn is_empty(&self) -> bool {
        self.head == NONE
    }
}

// ---------------------------------------------------------------------------
// LFUCache
// ---------------------------------------------------------------------------

/// A Least Frequently Used (LFU) cache with O(1) `get` and `put`.
///
/// **How it works:**
///
/// * Every key has an access frequency (starts at 1 on first `put`).
/// * On each `get` or `put` (update), the frequency is incremented.
/// * When the cache is full and a new key arrives, the key with the
///   *lowest* frequency is evicted.  If there is a tie, the *least
///   recently used* key among those with the lowest frequency is evicted.
///
/// **Time:** O(1) for both `get` and `put`.
/// **Space:** O(capacity).
pub struct LFUCache {
    capacity: usize,
    /// Maps key → index into `nodes` arena.
    key_to_idx: HashMap<i32, usize>,
    /// Arena holding all nodes.
    nodes: Vec<Node>,
    /// Maps frequency → doubly-linked list of nodes at that frequency.
    freq_buckets: HashMap<usize, FreqBucket>,
    /// The current minimum frequency across all keys (used for O(1) eviction).
    min_freq: usize,
}

impl LFUCache {
    /// Creates a new LFU cache that holds at most `capacity` key-value pairs.
    ///
    /// **Time:** O(1). **Space:** O(capacity).
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            key_to_idx: HashMap::new(),
            nodes: Vec::new(),
            freq_buckets: HashMap::new(),
            min_freq: 0,
        }
    }

    /// Returns the value for `key` if present, incrementing its frequency.
    ///
    /// **Time:** O(1). **Space:** O(1).
    pub fn get(&mut self, key: i32) -> Option<i32> {
        if self.capacity == 0 {
            return None;
        }
        let &idx = self.key_to_idx.get(&key)?;
        let value = self.nodes[idx].value;
        self.touch(idx);
        Some(value)
    }

    /// Inserts or updates `key` with `value`.
    ///
    /// * If `key` already exists its value is updated and frequency incremented.
    /// * If the cache is full the least frequently used key is evicted first
    ///   (LRU tiebreaker among equal frequencies).
    ///
    /// **Time:** O(1). **Space:** O(1) amortized.
    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        // --- Update existing key ---
        if let Some(&idx) = self.key_to_idx.get(&key) {
            self.nodes[idx].value = value;
            self.touch(idx);
            return;
        }

        // --- Evict if full ---
        if self.key_to_idx.len() == self.capacity {
            self.evict();
        }

        // --- Insert new node ---
        let idx = self.nodes.len();
        self.nodes.push(Node {
            key,
            value,
            freq: 1,
            prev: NONE,
            next: NONE,
        });
        self.key_to_idx.insert(key, idx);
        self.add_to_bucket(1, idx);
        self.min_freq = 1;
    }

    /// Returns the number of key-value pairs currently in the cache.
    pub fn len(&self) -> usize {
        self.key_to_idx.len()
    }

    /// Returns `true` if the cache contains no entries.
    pub fn is_empty(&self) -> bool {
        self.key_to_idx.is_empty()
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    /// Increment the frequency of the node at `idx` and move it to the new
    /// frequency bucket (at the tail = most recently used).
    fn touch(&mut self, idx: usize) {
        let old_freq = self.nodes[idx].freq;
        let new_freq = old_freq + 1;
        self.nodes[idx].freq = new_freq;

        self.remove_from_bucket(old_freq, idx);

        // If the old bucket was the min-frequency bucket and is now empty,
        // bump min_freq.
        if old_freq == self.min_freq {
            if let Some(bucket) = self.freq_buckets.get(&old_freq) {
                if bucket.is_empty() {
                    self.min_freq = new_freq;
                }
            }
        }

        self.add_to_bucket(new_freq, idx);
    }

    /// Evict the least-frequently-used node (head of the min-frequency bucket).
    fn evict(&mut self) {
        let bucket = self
            .freq_buckets
            .get(&self.min_freq)
            .expect("min_freq bucket must exist");
        let victim_idx = bucket.head;
        assert_ne!(victim_idx, NONE, "cannot evict from empty bucket");

        let victim_key = self.nodes[victim_idx].key;
        self.remove_from_bucket(self.min_freq, victim_idx);
        self.key_to_idx.remove(&victim_key);
    }

    /// Append `idx` to the tail (most-recently-used end) of the bucket for
    /// `freq`.
    fn add_to_bucket(&mut self, freq: usize, idx: usize) {
        let bucket = self
            .freq_buckets
            .entry(freq)
            .or_insert_with(FreqBucket::new);

        if bucket.tail == NONE {
            // Bucket was empty.
            bucket.head = idx;
            bucket.tail = idx;
            self.nodes[idx].prev = NONE;
            self.nodes[idx].next = NONE;
        } else {
            let old_tail = bucket.tail;
            self.nodes[old_tail].next = idx;
            self.nodes[idx].prev = old_tail;
            self.nodes[idx].next = NONE;
            bucket.tail = idx;
        }
    }

    /// Remove `idx` from the bucket for `freq`.
    fn remove_from_bucket(&mut self, freq: usize, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        let bucket = self
            .freq_buckets
            .get_mut(&freq)
            .expect("bucket must exist for this freq");

        if prev != NONE {
            self.nodes[prev].next = next;
        } else {
            bucket.head = next;
        }

        if next != NONE {
            self.nodes[next].prev = prev;
        } else {
            bucket.tail = prev;
        }

        // Clear the node's links so it can be re-inserted cleanly.
        self.nodes[idx].prev = NONE;
        self.nodes[idx].next = NONE;
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
        let cache = LFUCache::new(3);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    // ---- get / put basics ----

    #[test]
    fn put_and_get_single() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 10);
        assert_eq!(cache.get(1), Some(10));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 10);
        assert_eq!(cache.get(99), None);
    }

    #[test]
    fn put_updates_existing_value() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 10);
        cache.put(1, 20);
        assert_eq!(cache.get(1), Some(20));
        assert_eq!(cache.len(), 1);
    }

    // ---- eviction by frequency ----

    #[test]
    fn evicts_least_frequently_used() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        // Access key 1 so its freq becomes 2; key 2 stays at freq 1.
        cache.get(1);
        // Insert key 3 → must evict key 2 (lowest freq).
        cache.put(3, 30);
        assert_eq!(cache.get(2), None); // evicted
        assert_eq!(cache.get(1), Some(10));
        assert_eq!(cache.get(3), Some(30));
    }

    // ---- LRU tiebreaker ----

    #[test]
    fn evicts_lru_among_equal_frequency() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 10); // freq 1
        cache.put(2, 20); // freq 1
        // Both have freq 1; key 1 was inserted first (least recently used).
        cache.put(3, 30);
        assert_eq!(cache.get(1), None); // evicted (LRU tiebreaker)
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
    }

    #[test]
    fn lru_tiebreaker_considers_access_order() {
        let mut cache = LFUCache::new(3);
        cache.put(1, 10); // freq 1
        cache.put(2, 20); // freq 1
        cache.put(3, 30); // freq 1
        // Access 1 and 3 — their freq becomes 2. Key 2 stays at freq 1.
        cache.get(1);
        cache.get(3);
        // Now access 2 so its freq becomes 2 as well. All at freq 2.
        cache.get(2);
        // Among freq-2 keys, order of last touch: 1, 3, 2.
        // Eviction should remove key 1 (touched earliest among freq-2).
        cache.put(4, 40); // freq 1 → new min_freq = 1
        assert_eq!(cache.get(1), None); // evicted
        assert_eq!(cache.get(2), Some(20));
        assert_eq!(cache.get(3), Some(30));
        assert_eq!(cache.get(4), Some(40));
    }

    // ---- capacity edge cases ----

    #[test]
    fn zero_capacity_always_returns_none() {
        let mut cache = LFUCache::new(0);
        cache.put(1, 10);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LFUCache::new(1);
        cache.put(1, 10);
        assert_eq!(cache.get(1), Some(10));
        cache.put(2, 20); // evicts key 1
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(2), Some(20));
    }

    // ---- update does not double-count ----

    #[test]
    fn update_existing_does_not_increase_len() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(1, 100); // update, not insert
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(1), Some(100));
    }

    #[test]
    fn update_increases_frequency() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 10); // freq 1
        cache.put(2, 20); // freq 1
        cache.put(1, 100); // update → freq 2
        // Evict: key 2 has lower freq.
        cache.put(3, 30);
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(1), Some(100));
        assert_eq!(cache.get(3), Some(30));
    }

    // ---- many operations ----

    #[test]
    fn stress_sequential_inserts() {
        let mut cache = LFUCache::new(3);
        for i in 0..100 {
            cache.put(i, i * 10);
        }
        // Only the last 3 should remain (each inserted at freq 1, LRU eviction).
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(97), Some(970));
        assert_eq!(cache.get(98), Some(980));
        assert_eq!(cache.get(99), Some(990));
    }

    // ---- LeetCode 460 example ----

    #[test]
    fn leetcode_460_example() {
        let mut cache = LFUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), Some(1)); // freq(1)=2
        cache.put(3, 3); // evicts key 2 (freq 1)
        assert_eq!(cache.get(2), None);
        assert_eq!(cache.get(3), Some(3)); // freq(3)=2
        cache.put(4, 4); // evicts key 1 or 3? Both freq 2. Key 1 was touched earlier → evict 1.
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(3), Some(3));
        assert_eq!(cache.get(4), Some(4));
    }
}
