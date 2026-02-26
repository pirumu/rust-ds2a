//! # Design-Oriented Data Structures
//!
//! Interview-classic data structures that combine multiple primitives
//! to achieve specific performance guarantees: MinStack, MedianFinder,
//! RandomizedSet, and NestedIterator.

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;

// ---------------------------------------------------------------------------
// 1. MinStack — O(1) push, pop, top, get_min
// ---------------------------------------------------------------------------

/// A stack that supports retrieving the minimum element in O(1).
///
/// Uses an auxiliary stack that mirrors the main stack, where each entry
/// records the current minimum at that depth.
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        let current_min = match self.min_stack.last() {
            Some(&m) => val.min(m),
            None => val,
        };
        self.min_stack.push(current_min);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.min_stack.pop();
        self.stack.pop()
    }

    pub fn top(&self) -> Option<i32> {
        self.stack.last().copied()
    }

    pub fn get_min(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }
}

// ---------------------------------------------------------------------------
// 2. MedianFinder — two-heap approach
// ---------------------------------------------------------------------------

/// Finds the running median of a stream of integers using two heaps.
///
/// `lo` is a max-heap holding the smaller half of the numbers.
/// `hi` is a min-heap (via `Reverse`) holding the larger half.
///
/// Invariant: `lo.len() >= hi.len()` and `lo.len() - hi.len() <= 1`.
pub struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        // Always push to lo first (via hi to maintain ordering).
        self.lo.push(num);

        // Move the largest of lo to hi to keep ordering.
        if let Some(lo_max) = self.lo.pop() {
            self.hi.push(Reverse(lo_max));
        }

        // Rebalance: lo should have >= hi elements.
        if self.hi.len() > self.lo.len() {
            if let Some(Reverse(hi_min)) = self.hi.pop() {
                self.lo.push(hi_min);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.lo.len() > self.hi.len() {
            *self.lo.peek().unwrap() as f64
        } else {
            let lo_max = *self.lo.peek().unwrap() as f64;
            let hi_min = self.hi.peek().unwrap().0 as f64;
            (lo_max + hi_min) / 2.0
        }
    }
}

// ---------------------------------------------------------------------------
// 3. RandomizedSet — O(1) insert / remove / get_random
// ---------------------------------------------------------------------------

/// A set supporting O(1) average-time insert, remove, and random access.
///
/// Uses a `Vec` for storage (enabling O(1) random index) and a `HashMap`
/// for O(1) lookups. Removal swaps the target with the last element.
pub struct RandomizedSet {
    vals: Vec<i32>,
    index_map: HashMap<i32, usize>,
    rng_state: u64,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            vals: Vec::new(),
            index_map: HashMap::new(),
            rng_state: 0x12345678_9ABCDEF0,
        }
    }

    /// Inserts `val` if not already present. Returns `true` if inserted.
    pub fn insert(&mut self, val: i32) -> bool {
        if self.index_map.contains_key(&val) {
            return false;
        }
        let idx = self.vals.len();
        self.vals.push(val);
        self.index_map.insert(val, idx);
        true
    }

    /// Removes `val` if present. Returns `true` if removed.
    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.index_map.get(&val) {
            let last_idx = self.vals.len() - 1;
            let last_val = self.vals[last_idx];

            // Swap target with last element.
            self.vals.swap(idx, last_idx);
            self.index_map.insert(last_val, idx);

            self.vals.pop();
            self.index_map.remove(&val);
            true
        } else {
            false
        }
    }

    /// Returns a random element. Panics if the set is empty.
    pub fn get_random(&mut self) -> i32 {
        assert!(!self.vals.is_empty(), "get_random called on empty set");
        let idx = self.xorshift() as usize % self.vals.len();
        self.vals[idx]
    }

    pub fn len(&self) -> usize {
        self.vals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vals.is_empty()
    }

    /// Simple xorshift64 PRNG — no external crate needed.
    fn xorshift(&mut self) -> u64 {
        let mut x = self.rng_state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng_state = x;
        x
    }
}

// ---------------------------------------------------------------------------
// 4. NestedIterator — flatten nested integer lists
// ---------------------------------------------------------------------------

/// Represents either a single integer or a nested list of `NestedInteger`s.
#[derive(Debug, Clone)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

/// An iterator that flattens a `Vec<NestedInteger>` into a sequence of `i32`.
///
/// Pre-flattens in the constructor; stores results in a `Vec` used as a
/// queue via an index pointer.
pub struct NestedIterator {
    stack: Vec<i32>,
    index: usize,
}

impl NestedIterator {
    pub fn new(list: Vec<NestedInteger>) -> Self {
        let mut flat = Vec::new();
        Self::flatten(&list, &mut flat);
        Self {
            stack: flat,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Option<i32> {
        if self.index < self.stack.len() {
            let val = self.stack[self.index];
            self.index += 1;
            Some(val)
        } else {
            None
        }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.stack.len()
    }

    fn flatten(list: &[NestedInteger], out: &mut Vec<i32>) {
        for item in list {
            match item {
                NestedInteger::Int(v) => out.push(*v),
                NestedInteger::List(inner) => Self::flatten(inner, out),
            }
        }
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_stack_basic() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), Some(-3));
        s.pop();
        assert_eq!(s.top(), Some(0));
        assert_eq!(s.get_min(), Some(-2));
    }

    #[test]
    fn median_finder_basic() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        assert_eq!(mf.find_median(), 1.0);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }

    #[test]
    fn median_finder_negative() {
        let mut mf = MedianFinder::new();
        mf.add_num(-1);
        mf.add_num(-2);
        assert_eq!(mf.find_median(), -1.5);
    }

    #[test]
    fn randomized_set_basic() {
        let mut rs = RandomizedSet::new();
        assert!(rs.insert(1));
        assert!(!rs.insert(1));
        assert!(rs.insert(2));
        assert_eq!(rs.len(), 2);
        assert!(rs.remove(1));
        assert!(!rs.remove(1));
        assert_eq!(rs.len(), 1);
        let val = rs.get_random();
        assert_eq!(val, 2);
    }

    #[test]
    fn nested_iterator_basic() {
        let list = vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ];
        let mut iter = NestedIterator::new(list);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next().unwrap());
        }
        assert_eq!(result, vec![1, 1, 2, 1, 1]);
    }

    #[test]
    fn nested_iterator_deeply_nested() {
        let list = vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ];
        let mut iter = NestedIterator::new(list);
        let mut result = Vec::new();
        while iter.has_next() {
            result.push(iter.next().unwrap());
        }
        assert_eq!(result, vec![1, 4, 6]);
    }
}
