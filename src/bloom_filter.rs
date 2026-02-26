//! # Bloom Filter
//!
//! A space-efficient probabilistic data structure for set membership queries.
//! It can tell you *definitely not in set* or *possibly in set* — but never
//! gives a false negative.

/// A Bloom filter backed by a bit array and multiple hash functions.
///
/// # How it works
///
/// On insertion, the item is hashed by `num_hashes` independent hash functions,
/// and the corresponding bits in the bit array are set to 1. On lookup, we
/// check all the same bit positions — if *any* is 0 the item is definitely not
/// in the set; if all are 1 the item is *probably* in the set (with a small
/// false-positive probability).
pub struct BloomFilter {
    bits: Vec<bool>,
    size: usize,
    num_hashes: usize,
    count: usize,
}

impl BloomFilter {
    /// Creates a new Bloom filter with the given bit-array `size` and number
    /// of hash functions.
    ///
    /// Larger `size` and more hash functions (up to a point) reduce the
    /// false-positive rate but use more memory and CPU.
    pub fn new(size: usize, num_hashes: usize) -> Self {
        Self {
            bits: vec![false; size],
            size,
            num_hashes,
            count: 0,
        }
    }

    /// Inserts an item into the Bloom filter.
    pub fn insert(&mut self, item: &str) {
        for i in 0..self.num_hashes {
            let idx = self.hash(item, i);
            self.bits[idx] = true;
        }
        self.count += 1;
    }

    /// Returns `true` if the item *might* be in the set.
    ///
    /// A return value of `false` guarantees the item was never inserted.
    /// A return value of `true` means the item *probably* was inserted, but
    /// there is a small chance of a false positive.
    pub fn might_contain(&self, item: &str) -> bool {
        for i in 0..self.num_hashes {
            let idx = self.hash(item, i);
            if !self.bits[idx] {
                return false;
            }
        }
        true
    }

    /// Returns an estimate of the current false-positive rate.
    ///
    /// Uses the formula:
    ///
    /// ```text
    /// p ≈ (1 - e^(-k·n / m))^k
    /// ```
    ///
    /// where `k` = number of hash functions, `n` = items inserted, `m` = bit
    /// array size.
    pub fn false_positive_rate(&self) -> f64 {
        let k = self.num_hashes as f64;
        let n = self.count as f64;
        let m = self.size as f64;
        let exp = (-k * n / m).exp();
        (1.0 - exp).powf(k)
    }

    /// Computes the `i`-th hash of `item`, returning an index into the bit array.
    ///
    /// We use a double-hashing scheme: two base hashes `h1` and `h2` are
    /// combined as `h(i) = h1 + i * h2` to produce `num_hashes` independent
    /// hash values from just two underlying hashes.
    fn hash(&self, item: &str, i: usize) -> usize {
        let h1 = self.fnv1a(item);
        let h2 = self.djb2(item);
        let combined = h1.wrapping_add(i.wrapping_mul(h2));
        combined % self.size
    }

    /// FNV-1a hash — same algorithm as our `SimpleHasher` in `hash_map.rs`.
    fn fnv1a(&self, s: &str) -> usize {
        let mut hash: usize = 0xcbf29ce484222325_u64 as usize;
        for byte in s.bytes() {
            hash ^= byte as usize;
            hash = hash.wrapping_mul(0x100000001b3_u64 as usize);
        }
        hash
    }

    /// DJB2 hash — a classic, simple hash function by Daniel J. Bernstein.
    fn djb2(&self, s: &str) -> usize {
        let mut hash: usize = 5381;
        for byte in s.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as usize);
        }
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_filter_contains_nothing() {
        let bf = BloomFilter::new(1000, 3);
        assert!(!bf.might_contain("hello"));
        assert!(!bf.might_contain("world"));
    }

    #[test]
    fn inserted_items_are_found() {
        let mut bf = BloomFilter::new(1000, 3);
        bf.insert("apple");
        bf.insert("banana");
        bf.insert("cherry");
        assert!(bf.might_contain("apple"));
        assert!(bf.might_contain("banana"));
        assert!(bf.might_contain("cherry"));
    }

    #[test]
    fn non_inserted_items_usually_not_found() {
        let mut bf = BloomFilter::new(10_000, 5);
        for i in 0..100 {
            bf.insert(&format!("item-{i}"));
        }
        // Check 100 items that were *not* inserted. With a 10000-bit filter
        // and only 100 items, false positives should be very rare.
        let mut false_positives = 0;
        for i in 1000..1100 {
            if bf.might_contain(&format!("item-{i}")) {
                false_positives += 1;
            }
        }
        // Allow up to 5 false positives out of 100 — extremely generous.
        assert!(
            false_positives <= 5,
            "Too many false positives: {false_positives}/100"
        );
    }

    #[test]
    fn false_positive_rate_increases_with_inserts() {
        let mut bf = BloomFilter::new(100, 3);
        let rate_empty = bf.false_positive_rate();
        assert!((rate_empty - 0.0).abs() < f64::EPSILON);

        for i in 0..50 {
            bf.insert(&format!("item-{i}"));
        }
        let rate_half = bf.false_positive_rate();
        assert!(rate_half > 0.0);
        assert!(rate_half < 1.0);
    }

    #[test]
    fn large_filter_low_false_positive_rate() {
        let mut bf = BloomFilter::new(100_000, 7);
        for i in 0..1000 {
            bf.insert(&format!("key-{i}"));
        }
        let rate = bf.false_positive_rate();
        // With m=100000, k=7, n=1000 the theoretical rate is tiny.
        assert!(rate < 0.001, "Rate too high: {rate}");
    }
}
