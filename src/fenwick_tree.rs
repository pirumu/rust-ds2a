//! # Fenwick Tree (Binary Indexed Tree)
//!
//! A data structure that supports efficient prefix sum queries and point updates.
//! Combines the idea of prefix sums with bit manipulation to achieve O(log n)
//! for both operations.

/// A Fenwick Tree (Binary Indexed Tree) for prefix sum queries with point updates.
///
/// Internally uses 1-based indexing. The public API accepts 0-based indices
/// and converts them automatically.
///
/// **Time:** O(log n) for `update`, `prefix_sum`, and `range_sum`.
/// **Space:** O(n).
#[derive(Debug, Clone)]
pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    /// Creates a new Fenwick Tree of size `n`, initialized to all zeros.
    ///
    /// **Time:** O(n). **Space:** O(n).
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1], // 1-indexed
            n,
        }
    }

    /// Builds a Fenwick Tree from an existing slice.
    ///
    /// This is more efficient than creating an empty tree and calling `update`
    /// for each element — it runs in O(n) instead of O(n log n).
    ///
    /// **Time:** O(n). **Space:** O(n).
    pub fn from_vec(data: &[i64]) -> Self {
        let n = data.len();
        let mut tree = vec![0i64; n + 1];

        // Copy data into 1-indexed positions.
        tree[1..=n].copy_from_slice(data);

        // Build the tree in O(n) by propagating values to parents.
        for i in 1..=n {
            let parent = i + lowbit(i);
            if parent <= n {
                tree[parent] += tree[i];
            }
        }

        Self { tree, n }
    }

    /// Returns the number of elements the tree was built for.
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns `true` if the tree has no elements.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Adds `delta` to the element at position `idx` (0-based).
    ///
    /// **Time:** O(log n).
    pub fn update(&mut self, idx: usize, delta: i64) {
        assert!(idx < self.n, "index {idx} out of bounds (size {})", self.n);
        let mut i = idx + 1; // convert to 1-indexed
        while i <= self.n {
            self.tree[i] += delta;
            i += lowbit(i);
        }
    }

    /// Returns the prefix sum of elements in `[0, idx]` (0-based, inclusive).
    ///
    /// **Time:** O(log n).
    pub fn prefix_sum(&self, idx: usize) -> i64 {
        assert!(idx < self.n, "index {idx} out of bounds (size {})", self.n);
        let mut sum = 0i64;
        let mut i = idx + 1; // convert to 1-indexed
        while i > 0 {
            sum += self.tree[i];
            i -= lowbit(i);
        }
        sum
    }

    /// Returns the sum of elements in `[l, r]` (0-based, inclusive).
    ///
    /// **Time:** O(log n).
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r, "left {l} must be <= right {r}");
        assert!(r < self.n, "right index {r} out of bounds (size {})", self.n);
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }
}

/// Returns the lowest set bit of `i` (i.e., `i & (-i)`).
///
/// This is the fundamental bit trick behind Fenwick Trees.
#[inline]
fn lowbit(i: usize) -> usize {
    // For unsigned: i & i.wrapping_neg()
    i & i.wrapping_neg()
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- lowbit ----

    #[test]
    fn lowbit_examples() {
        assert_eq!(lowbit(1), 1);  // 0b0001 -> 1
        assert_eq!(lowbit(2), 2);  // 0b0010 -> 2
        assert_eq!(lowbit(3), 1);  // 0b0011 -> 1
        assert_eq!(lowbit(4), 4);  // 0b0100 -> 4
        assert_eq!(lowbit(6), 2);  // 0b0110 -> 2
        assert_eq!(lowbit(12), 4); // 0b1100 -> 4
    }

    // ---- new ----

    #[test]
    fn new_creates_zero_tree() {
        let ft = FenwickTree::new(5);
        assert_eq!(ft.len(), 5);
        assert!(!ft.is_empty());
        for i in 0..5 {
            assert_eq!(ft.prefix_sum(i), 0);
        }
    }

    #[test]
    fn new_empty_tree() {
        let ft = FenwickTree::new(0);
        assert!(ft.is_empty());
        assert_eq!(ft.len(), 0);
    }

    // ---- from_vec ----

    #[test]
    fn from_vec_basic() {
        let data = vec![1, 2, 3, 4, 5];
        let ft = FenwickTree::from_vec(&data);
        assert_eq!(ft.len(), 5);
        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(1), 3);
        assert_eq!(ft.prefix_sum(2), 6);
        assert_eq!(ft.prefix_sum(3), 10);
        assert_eq!(ft.prefix_sum(4), 15);
    }

    #[test]
    fn from_vec_empty() {
        let ft = FenwickTree::from_vec(&[]);
        assert!(ft.is_empty());
    }

    #[test]
    fn from_vec_single_element() {
        let ft = FenwickTree::from_vec(&[42]);
        assert_eq!(ft.prefix_sum(0), 42);
    }

    #[test]
    fn from_vec_negative_values() {
        let data = vec![-3, 5, -2, 7];
        let ft = FenwickTree::from_vec(&data);
        assert_eq!(ft.prefix_sum(0), -3);
        assert_eq!(ft.prefix_sum(1), 2);
        assert_eq!(ft.prefix_sum(2), 0);
        assert_eq!(ft.prefix_sum(3), 7);
    }

    // ---- update ----

    #[test]
    fn update_single_element() {
        let mut ft = FenwickTree::new(5);
        ft.update(2, 10);
        assert_eq!(ft.prefix_sum(1), 0);
        assert_eq!(ft.prefix_sum(2), 10);
        assert_eq!(ft.prefix_sum(3), 10);
        assert_eq!(ft.prefix_sum(4), 10);
    }

    #[test]
    fn update_multiple_elements() {
        let mut ft = FenwickTree::new(4);
        ft.update(0, 1);
        ft.update(1, 2);
        ft.update(2, 3);
        ft.update(3, 4);
        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(1), 3);
        assert_eq!(ft.prefix_sum(2), 6);
        assert_eq!(ft.prefix_sum(3), 10);
    }

    #[test]
    fn update_accumulates() {
        let mut ft = FenwickTree::new(3);
        ft.update(1, 5);
        ft.update(1, 3);
        ft.update(1, -2);
        assert_eq!(ft.prefix_sum(1), 6); // 5 + 3 - 2
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn update_out_of_bounds() {
        let mut ft = FenwickTree::new(3);
        ft.update(3, 1);
    }

    // ---- prefix_sum ----

    #[test]
    fn prefix_sum_after_from_vec_and_update() {
        let mut ft = FenwickTree::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(ft.prefix_sum(4), 15);
        ft.update(2, 10); // [1, 2, 13, 4, 5]
        assert_eq!(ft.prefix_sum(4), 25);
        assert_eq!(ft.prefix_sum(2), 16);
        assert_eq!(ft.prefix_sum(1), 3);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn prefix_sum_out_of_bounds() {
        let ft = FenwickTree::new(3);
        ft.prefix_sum(3);
    }

    // ---- range_sum ----

    #[test]
    fn range_sum_basic() {
        let ft = FenwickTree::from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(ft.range_sum(0, 4), 15);
        assert_eq!(ft.range_sum(1, 3), 9);  // 2 + 3 + 4
        assert_eq!(ft.range_sum(2, 2), 3);  // single element
        assert_eq!(ft.range_sum(0, 0), 1);  // first element
        assert_eq!(ft.range_sum(4, 4), 5);  // last element
    }

    #[test]
    fn range_sum_after_updates() {
        let mut ft = FenwickTree::from_vec(&[10, 20, 30, 40, 50]);
        ft.update(1, -5);  // [10, 15, 30, 40, 50]
        ft.update(3, 10);  // [10, 15, 30, 50, 50]
        assert_eq!(ft.range_sum(1, 3), 95); // 15 + 30 + 50
        assert_eq!(ft.range_sum(0, 4), 155);
    }

    #[test]
    #[should_panic(expected = "left")]
    fn range_sum_invalid_range() {
        let ft = FenwickTree::from_vec(&[1, 2, 3]);
        ft.range_sum(2, 1);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn range_sum_out_of_bounds() {
        let ft = FenwickTree::from_vec(&[1, 2, 3]);
        ft.range_sum(0, 5);
    }

    // ---- large input ----

    #[test]
    fn large_input() {
        let n = 10_000;
        let data: Vec<i64> = (1..=n as i64).collect();
        let ft = FenwickTree::from_vec(&data);

        // Sum of 1..=n = n*(n+1)/2
        let expected = (n as i64) * (n as i64 + 1) / 2;
        assert_eq!(ft.prefix_sum(n - 1), expected);

        // Range sum of [500, 999] = sum(501..=1000)
        let range_expected: i64 = (501..=1000).sum();
        assert_eq!(ft.range_sum(500, 999), range_expected);
    }

    // ---- consistency with brute force ----

    #[test]
    fn consistency_with_naive() {
        let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
        let mut ft = FenwickTree::from_vec(&data);
        let mut naive = data.clone();

        // Apply some updates
        let updates = vec![(0, 7), (5, -3), (9, 10), (3, -1)];
        for &(idx, delta) in &updates {
            ft.update(idx, delta);
            naive[idx] += delta;
        }

        // Verify every prefix sum matches
        let mut running = 0i64;
        for i in 0..naive.len() {
            running += naive[i];
            assert_eq!(ft.prefix_sum(i), running, "mismatch at prefix_sum({i})");
        }

        // Verify several range sums
        for l in 0..naive.len() {
            for r in l..naive.len() {
                let expected: i64 = naive[l..=r].iter().sum();
                assert_eq!(
                    ft.range_sum(l, r),
                    expected,
                    "mismatch at range_sum({l}, {r})"
                );
            }
        }
    }
}
