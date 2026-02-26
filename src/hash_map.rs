//! # Hash Map
//!
//! A hash table using separate chaining (linked-list buckets) for collision resolution.

use std::fmt::Debug;
use std::hash::{Hash, Hasher};

// ---------------------------------------------------------------------------
// Simple educational hash function (FNV-1a inspired)
// ---------------------------------------------------------------------------

/// A minimal hasher for educational purposes.
///
/// It implements FNV-1a, a fast non-cryptographic hash well-suited for hash
/// tables. We provide this instead of using `std::collections::hash_map::DefaultHasher`
/// so that readers can see how bytes get mixed into a hash value.
struct SimpleHasher {
    state: u64,
}

impl SimpleHasher {
    const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    fn new() -> Self {
        Self {
            state: Self::FNV_OFFSET_BASIS,
        }
    }
}

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(Self::FNV_PRIME);
        }
    }
}

/// Compute a hash using our simple educational hasher.
fn simple_hash<K: Hash>(key: &K) -> u64 {
    let mut hasher = SimpleHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

// ---------------------------------------------------------------------------
// Entry — a key-value pair stored in a bucket chain
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
    next: Option<Box<Entry<K, V>>>,
}

// ---------------------------------------------------------------------------
// HashMap
// ---------------------------------------------------------------------------

const INITIAL_CAPACITY: usize = 16;
const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

/// A hash map implemented with separate chaining.
///
/// Keys must implement `Hash + Eq` so we can compute bucket indices and
/// compare entries. The map automatically resizes when the load factor
/// (len / capacity) exceeds 0.75.
#[derive(Debug)]
pub struct HashMap<K, V> {
    buckets: Vec<Option<Box<Entry<K, V>>>>,
    len: usize,
}

impl<K: Hash + Eq + Debug, V: Debug> HashMap<K, V> {
    /// Creates an empty `HashMap` with the default initial capacity (16).
    pub fn new() -> Self {
        let mut buckets = Vec::with_capacity(INITIAL_CAPACITY);
        for _ in 0..INITIAL_CAPACITY {
            buckets.push(None);
        }
        Self { buckets, len: 0 }
    }

    /// Returns the number of key-value pairs in the map.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the map contains no entries.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the bucket index for the given key.
    fn bucket_index(&self, key: &K) -> usize {
        (simple_hash(key) as usize) % self.buckets.len()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the key already exists, the old value is replaced and returned.
    /// Returns `None` if the key was new.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Check whether the key already exists — if so, update in place.
        let idx = self.bucket_index(&key);
        {
            let mut current = &mut self.buckets[idx];
            while let Some(entry) = current {
                if entry.key == key {
                    let old = std::mem::replace(&mut entry.value, value);
                    return Some(old);
                }
                current = &mut entry.next;
            }
        }

        // Key not found — insert a new entry at the head of the chain.
        let head = self.buckets[idx].take();
        self.buckets[idx] = Some(Box::new(Entry {
            key,
            value,
            next: head,
        }));
        self.len += 1;

        // Resize if we exceed the load-factor threshold.
        if self.load_factor() > LOAD_FACTOR_THRESHOLD {
            self.resize();
        }

        None
    }

    /// Returns a reference to the value associated with the key, or `None`.
    pub fn get(&self, key: &K) -> Option<&V> {
        let idx = self.bucket_index(key);
        let mut current = &self.buckets[idx];
        while let Some(entry) = current {
            if entry.key == *key {
                return Some(&entry.value);
            }
            current = &entry.next;
        }
        None
    }

    /// Returns `true` if the map contains the given key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Removes a key from the map, returning its value if the key was present.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.bucket_index(key);

        // Special case: the key is in the head of the chain.
        if let Some(ref head) = self.buckets[idx] {
            if head.key == *key {
                let removed = self.buckets[idx].take().unwrap();
                self.buckets[idx] = removed.next;
                self.len -= 1;
                return Some(removed.value);
            }
        }

        // Walk the chain looking for a node whose *next* matches.
        let mut current = &mut self.buckets[idx];
        while let Some(entry) = current {
            if let Some(ref next) = entry.next {
                if next.key == *key {
                    let removed = entry.next.take().unwrap();
                    entry.next = removed.next;
                    self.len -= 1;
                    return Some(removed.value);
                }
            }
            current = &mut entry.next;
        }

        None
    }

    /// Returns the current load factor (len / capacity).
    fn load_factor(&self) -> f64 {
        self.len as f64 / self.buckets.len() as f64
    }

    /// Doubles the capacity and re-hashes all entries.
    fn resize(&mut self) {
        let new_cap = self.buckets.len() * 2;
        let mut new_buckets: Vec<Option<Box<Entry<K, V>>>> = Vec::with_capacity(new_cap);
        for _ in 0..new_cap {
            new_buckets.push(None);
        }

        // Drain all entries from the old buckets.
        for bucket in &mut self.buckets {
            let mut current = bucket.take();
            while let Some(mut entry) = current {
                current = entry.next.take();
                let idx = (simple_hash(&entry.key) as usize) % new_cap;
                entry.next = new_buckets[idx].take();
                new_buckets[idx] = Some(entry);
            }
        }

        self.buckets = new_buckets;
    }
}

impl<K: Hash + Eq + Debug + Clone, V: Debug> HashMap<K, V> {
    /// Returns a `Vec` of references to all keys in the map.
    ///
    /// The order is arbitrary (depends on hashing). This is provided so that
    /// `HashSet` can implement set operations without a full `Iterator` impl.
    pub fn keys(&self) -> Vec<&K> {
        let mut result = Vec::with_capacity(self.len);
        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(entry) = current {
                result.push(&entry.key);
                current = &entry.next;
            }
        }
        result
    }
}

impl<K: Hash + Eq + Debug, V: Debug> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map_is_empty() {
        let map: HashMap<String, i32> = HashMap::new();
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn insert_and_get() {
        let mut map = HashMap::new();
        assert_eq!(map.insert("apple", 1), None);
        assert_eq!(map.insert("banana", 2), None);
        assert_eq!(map.get(&"apple"), Some(&1));
        assert_eq!(map.get(&"banana"), Some(&2));
        assert_eq!(map.get(&"cherry"), None);
    }

    #[test]
    fn insert_replaces_existing() {
        let mut map = HashMap::new();
        map.insert("key", 10);
        let old = map.insert("key", 20);
        assert_eq!(old, Some(10));
        assert_eq!(map.get(&"key"), Some(&20));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn remove_existing_key() {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        assert_eq!(map.remove(&"a"), Some(1));
        assert_eq!(map.get(&"a"), None);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn remove_nonexistent_key() {
        let mut map: HashMap<&str, i32> = HashMap::new();
        map.insert("a", 1);
        assert_eq!(map.remove(&"z"), None);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn contains_key() {
        let mut map = HashMap::new();
        map.insert(42, "answer");
        assert!(map.contains_key(&42));
        assert!(!map.contains_key(&0));
    }

    #[test]
    fn automatic_resizing() {
        let mut map = HashMap::new();
        // Insert enough entries to trigger at least one resize.
        for i in 0..100 {
            map.insert(i, i * 10);
        }
        assert_eq!(map.len(), 100);
        // Every entry should still be retrievable after resizing.
        for i in 0..100 {
            assert_eq!(map.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn collision_handling() {
        // Even though we can't easily force collisions with our hash,
        // inserting many keys exercises the chaining logic.
        let mut map = HashMap::new();
        for i in 0..50 {
            map.insert(format!("key-{i}"), i);
        }
        for i in 0..50 {
            assert_eq!(map.get(&format!("key-{i}")), Some(&i));
        }
    }

    #[test]
    fn remove_head_vs_middle_of_chain() {
        let mut map = HashMap::new();
        // Fill densely to increase probability of chains with >1 entry.
        for i in 0..200 {
            map.insert(i, i);
        }
        // Remove every other key.
        for i in (0..200).step_by(2) {
            assert_eq!(map.remove(&i), Some(i));
        }
        assert_eq!(map.len(), 100);
        for i in (1..200).step_by(2) {
            assert_eq!(map.get(&i), Some(&i));
        }
    }
}
