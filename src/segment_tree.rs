//! # Segment Tree
//!
//! Array-backed segment trees for efficient range queries and point updates.
//! Supports range sum and range minimum queries, plus lazy propagation.

// ---------------------------------------------------------------------------
// SegmentTree (Range Sum)
// ---------------------------------------------------------------------------

/// A segment tree for **range sum** queries with point updates.
///
/// Internally stored as a flat array of size `4 * n`.
#[derive(Debug)]
pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    /// Build a segment tree from a slice.
    ///
    /// **Time:** O(n).  **Space:** O(n).
    pub fn new(data: &[i64]) -> Self {
        let n = data.len();
        let mut st = Self {
            n,
            tree: vec![0i64; 4 * n.max(1)],
        };
        if n > 0 {
            st.build(data, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, data: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = data[start];
        } else {
            let mid = start + (end - start) / 2;
            self.build(data, 2 * node, start, mid);
            self.build(data, 2 * node + 1, mid + 1, end);
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    /// Return the sum of elements in `[l, r]` (inclusive).
    ///
    /// **Time:** O(log n).  **Space:** O(log n) stack.
    pub fn query(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r < self.n, "query range out of bounds");
        self.query_inner(1, 0, self.n - 1, l, r)
    }

    fn query_inner(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if r < start || end < l {
            return 0; // identity for sum
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = start + (end - start) / 2;
        self.query_inner(2 * node, start, mid, l, r)
            + self.query_inner(2 * node + 1, mid + 1, end, l, r)
    }

    /// Set the value at index `idx` to `val`.
    ///
    /// **Time:** O(log n).  **Space:** O(log n) stack.
    pub fn update(&mut self, idx: usize, val: i64) {
        assert!(idx < self.n, "update index out of bounds");
        self.update_inner(1, 0, self.n - 1, idx, val);
    }

    fn update_inner(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64) {
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = start + (end - start) / 2;
            if idx <= mid {
                self.update_inner(2 * node, start, mid, idx, val);
            } else {
                self.update_inner(2 * node + 1, mid + 1, end, idx, val);
            }
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }
}

// ---------------------------------------------------------------------------
// SegmentTreeMin (Range Minimum)
// ---------------------------------------------------------------------------

/// A segment tree for **range minimum** queries with point updates.
#[derive(Debug)]
pub struct SegmentTreeMin {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTreeMin {
    /// Build a range-min segment tree from a slice.
    ///
    /// **Time:** O(n).  **Space:** O(n).
    pub fn new(data: &[i64]) -> Self {
        let n = data.len();
        let mut st = Self {
            n,
            tree: vec![i64::MAX; 4 * n.max(1)],
        };
        if n > 0 {
            st.build(data, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, data: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = data[start];
        } else {
            let mid = start + (end - start) / 2;
            self.build(data, 2 * node, start, mid);
            self.build(data, 2 * node + 1, mid + 1, end);
            self.tree[node] = self.tree[2 * node].min(self.tree[2 * node + 1]);
        }
    }

    /// Return the minimum of elements in `[l, r]` (inclusive).
    ///
    /// **Time:** O(log n).  **Space:** O(log n) stack.
    pub fn query(&self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r < self.n, "query range out of bounds");
        self.query_inner(1, 0, self.n - 1, l, r)
    }

    fn query_inner(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if r < start || end < l {
            return i64::MAX; // identity for min
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        let mid = start + (end - start) / 2;
        self.query_inner(2 * node, start, mid, l, r)
            .min(self.query_inner(2 * node + 1, mid + 1, end, l, r))
    }

    /// Set the value at index `idx` to `val`.
    ///
    /// **Time:** O(log n).  **Space:** O(log n) stack.
    pub fn update(&mut self, idx: usize, val: i64) {
        assert!(idx < self.n, "update index out of bounds");
        self.update_inner(1, 0, self.n - 1, idx, val);
    }

    fn update_inner(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i64) {
        if start == end {
            self.tree[node] = val;
        } else {
            let mid = start + (end - start) / 2;
            if idx <= mid {
                self.update_inner(2 * node, start, mid, idx, val);
            } else {
                self.update_inner(2 * node + 1, mid + 1, end, idx, val);
            }
            self.tree[node] = self.tree[2 * node].min(self.tree[2 * node + 1]);
        }
    }
}

// ---------------------------------------------------------------------------
// LazySegmentTree (Range Sum with Lazy Range-Add)
// ---------------------------------------------------------------------------

/// A segment tree for **range sum** queries with **lazy range-add** updates.
///
/// Supports adding a value to every element in a range in O(log n).
#[derive(Debug)]
pub struct LazySegmentTree {
    n: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    /// Build a lazy segment tree from a slice.
    ///
    /// **Time:** O(n).  **Space:** O(n).
    pub fn new(data: &[i64]) -> Self {
        let n = data.len();
        let cap = 4 * n.max(1);
        let mut st = Self {
            n,
            tree: vec![0i64; cap],
            lazy: vec![0i64; cap],
        };
        if n > 0 {
            st.build(data, 1, 0, n - 1);
        }
        st
    }

    fn build(&mut self, data: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = data[start];
        } else {
            let mid = start + (end - start) / 2;
            self.build(data, 2 * node, start, mid);
            self.build(data, 2 * node + 1, mid + 1, end);
            self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
        }
    }

    fn push_down(&mut self, node: usize, start: usize, end: usize) {
        if self.lazy[node] != 0 {
            let mid = start + (end - start) / 2;
            self.apply(2 * node, start, mid, self.lazy[node]);
            self.apply(2 * node + 1, mid + 1, end, self.lazy[node]);
            self.lazy[node] = 0;
        }
    }

    fn apply(&mut self, node: usize, start: usize, end: usize, val: i64) {
        self.tree[node] += val * (end - start + 1) as i64;
        self.lazy[node] += val;
    }

    /// Add `val` to every element in `[l, r]` (inclusive).
    ///
    /// **Time:** O(log n).  **Space:** O(log n) stack.
    pub fn range_update(&mut self, l: usize, r: usize, val: i64) {
        assert!(l <= r && r < self.n, "range_update range out of bounds");
        self.range_update_inner(1, 0, self.n - 1, l, r, val);
    }

    fn range_update_inner(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        val: i64,
    ) {
        if r < start || end < l {
            return;
        }
        if l <= start && end <= r {
            self.apply(node, start, end, val);
            return;
        }
        self.push_down(node, start, end);
        let mid = start + (end - start) / 2;
        self.range_update_inner(2 * node, start, mid, l, r, val);
        self.range_update_inner(2 * node + 1, mid + 1, end, l, r, val);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    /// Return the sum of elements in `[l, r]` (inclusive).
    ///
    /// Pushes pending lazy values down as needed.
    ///
    /// **Time:** O(log n).  **Space:** O(log n) stack.
    pub fn query(&mut self, l: usize, r: usize) -> i64 {
        assert!(l <= r && r < self.n, "query range out of bounds");
        self.query_inner(1, 0, self.n - 1, l, r)
    }

    fn query_inner(
        &mut self,
        node: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
    ) -> i64 {
        if r < start || end < l {
            return 0;
        }
        if l <= start && end <= r {
            return self.tree[node];
        }
        self.push_down(node, start, end);
        let mid = start + (end - start) / 2;
        self.query_inner(2 * node, start, mid, l, r)
            + self.query_inner(2 * node + 1, mid + 1, end, l, r)
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- SegmentTree (sum) ----

    #[test]
    fn sum_single_element() {
        let st = SegmentTree::new(&[42]);
        assert_eq!(st.query(0, 0), 42);
    }

    #[test]
    fn sum_full_range() {
        let data = vec![1, 3, 5, 7, 9, 11];
        let st = SegmentTree::new(&data);
        assert_eq!(st.query(0, 5), 36);
    }

    #[test]
    fn sum_partial_range() {
        let data = vec![1, 3, 5, 7, 9, 11];
        let st = SegmentTree::new(&data);
        assert_eq!(st.query(1, 3), 15); // 3 + 5 + 7
        assert_eq!(st.query(2, 4), 21); // 5 + 7 + 9
        assert_eq!(st.query(0, 0), 1);
        assert_eq!(st.query(5, 5), 11);
    }

    #[test]
    fn sum_after_update() {
        let data = vec![1, 3, 5, 7, 9, 11];
        let mut st = SegmentTree::new(&data);
        st.update(2, 10); // change 5 -> 10
        assert_eq!(st.query(0, 5), 41); // 1+3+10+7+9+11
        assert_eq!(st.query(1, 3), 20); // 3+10+7
        assert_eq!(st.query(2, 2), 10);
    }

    #[test]
    fn sum_multiple_updates() {
        let data = vec![2, 4, 6, 8];
        let mut st = SegmentTree::new(&data);
        st.update(0, 0);
        st.update(3, 0);
        assert_eq!(st.query(0, 3), 10); // 0+4+6+0
        assert_eq!(st.query(1, 2), 10); // 4+6
    }

    #[test]
    fn sum_power_of_two_length() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let st = SegmentTree::new(&data);
        assert_eq!(st.query(0, 7), 36);
        assert_eq!(st.query(0, 3), 10);
        assert_eq!(st.query(4, 7), 26);
    }

    #[test]
    fn sum_negative_values() {
        let data = vec![-5, 3, -2, 7, -1];
        let st = SegmentTree::new(&data);
        assert_eq!(st.query(0, 4), 2);
        assert_eq!(st.query(0, 2), -4);
    }

    // ---- SegmentTreeMin ----

    #[test]
    fn min_single_element() {
        let st = SegmentTreeMin::new(&[42]);
        assert_eq!(st.query(0, 0), 42);
    }

    #[test]
    fn min_full_range() {
        let data = vec![5, 1, 8, 3, 9, 2];
        let st = SegmentTreeMin::new(&data);
        assert_eq!(st.query(0, 5), 1);
    }

    #[test]
    fn min_partial_range() {
        let data = vec![5, 1, 8, 3, 9, 2];
        let st = SegmentTreeMin::new(&data);
        assert_eq!(st.query(0, 1), 1);
        assert_eq!(st.query(2, 4), 3);
        assert_eq!(st.query(3, 5), 2);
        assert_eq!(st.query(4, 4), 9);
    }

    #[test]
    fn min_after_update() {
        let data = vec![5, 1, 8, 3, 9, 2];
        let mut st = SegmentTreeMin::new(&data);
        st.update(1, 10); // change 1 -> 10
        assert_eq!(st.query(0, 5), 2);
        assert_eq!(st.query(0, 1), 5); // was 1, now min(5,10)=5
        st.update(5, 100);
        assert_eq!(st.query(0, 5), 3);
    }

    #[test]
    fn min_all_same() {
        let data = vec![7, 7, 7, 7];
        let st = SegmentTreeMin::new(&data);
        assert_eq!(st.query(0, 3), 7);
        assert_eq!(st.query(1, 2), 7);
    }

    #[test]
    fn min_negative_values() {
        let data = vec![3, -1, 4, -5, 2];
        let st = SegmentTreeMin::new(&data);
        assert_eq!(st.query(0, 4), -5);
        assert_eq!(st.query(0, 2), -1);
        assert_eq!(st.query(3, 4), -5);
    }

    // ---- LazySegmentTree ----

    #[test]
    fn lazy_single_element() {
        let mut st = LazySegmentTree::new(&[10]);
        assert_eq!(st.query(0, 0), 10);
        st.range_update(0, 0, 5);
        assert_eq!(st.query(0, 0), 15);
    }

    #[test]
    fn lazy_range_update_full() {
        let data = vec![1, 2, 3, 4, 5];
        let mut st = LazySegmentTree::new(&data);
        st.range_update(0, 4, 10); // add 10 to all
        assert_eq!(st.query(0, 4), 65); // 11+12+13+14+15
    }

    #[test]
    fn lazy_range_update_partial() {
        let data = vec![1, 2, 3, 4, 5];
        let mut st = LazySegmentTree::new(&data);
        st.range_update(1, 3, 10); // add 10 to indices 1,2,3
        assert_eq!(st.query(0, 4), 45); // 1+12+13+14+5
        assert_eq!(st.query(1, 3), 39); // 12+13+14
        assert_eq!(st.query(0, 0), 1);
        assert_eq!(st.query(4, 4), 5);
    }

    #[test]
    fn lazy_multiple_range_updates() {
        let data = vec![0, 0, 0, 0, 0];
        let mut st = LazySegmentTree::new(&data);
        st.range_update(0, 2, 3); // [3, 3, 3, 0, 0]
        st.range_update(2, 4, 5); // [3, 3, 8, 5, 5]
        assert_eq!(st.query(0, 4), 24);
        assert_eq!(st.query(0, 2), 14); // 3+3+8
        assert_eq!(st.query(2, 4), 18); // 8+5+5
    }

    #[test]
    fn lazy_query_without_update() {
        let data = vec![10, 20, 30];
        let mut st = LazySegmentTree::new(&data);
        assert_eq!(st.query(0, 2), 60);
        assert_eq!(st.query(0, 0), 10);
        assert_eq!(st.query(1, 1), 20);
    }

    #[test]
    fn lazy_negative_update() {
        let data = vec![10, 20, 30, 40];
        let mut st = LazySegmentTree::new(&data);
        st.range_update(0, 3, -5);
        assert_eq!(st.query(0, 3), 80); // 5+15+25+35
    }
}
