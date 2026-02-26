//! # Skip List
//!
//! A probabilistic data structure that provides O(log n) expected search, insert,
//! and delete — without needing tree balancing. Uses multiple layers of linked
//! lists where higher layers "skip" over elements, like express elevators.

use std::fmt;

/// Maximum number of levels in the skip list.
const MAX_LEVEL: usize = 16;

/// A single node in the skip list.
///
/// Each node stores a value and a vector of forward links — one per level
/// the node participates in. A forward link is `Some(index)` pointing to
/// the next node at that level, or `None` if there is no successor.
#[derive(Debug, Clone)]
struct Node {
    value: i32,
    /// `forward[i]` = index of the next node at level `i`, or `None`.
    forward: Vec<Option<usize>>,
}

/// A skip list of `i32` values, backed entirely by `Vec` (no raw pointers).
///
/// Internally, index `0` is a sentinel head node. All real values live at
/// indices `1..`. Each node has a random height; higher levels act as
/// "express lanes" that let searches skip over many elements.
///
/// # Examples
///
/// ```
/// use rust_ds2a::skip_list::SkipList;
///
/// let mut sl = SkipList::new();
/// sl.insert(10);
/// sl.insert(20);
/// sl.insert(5);
///
/// assert!(sl.search(10));
/// assert!(!sl.search(15));
/// assert_eq!(sl.len(), 3);
///
/// assert!(sl.delete(10));
/// assert!(!sl.search(10));
/// ```
pub struct SkipList {
    /// Arena of nodes. Index 0 is the sentinel head.
    nodes: Vec<Node>,
    /// Current maximum level in use (0-indexed).
    level: usize,
    /// Number of real elements (excludes the sentinel).
    length: usize,
    /// Simple RNG state for level generation.
    rng_state: u64,
}

impl SkipList {
    /// Creates a new, empty skip list.
    ///
    /// **Time:** O(1). **Space:** O(1).
    pub fn new() -> Self {
        let head = Node {
            value: i32::MIN,
            forward: vec![None; MAX_LEVEL],
        };
        Self {
            nodes: vec![head],
            level: 0,
            length: 0,
            rng_state: 0xDEAD_BEEF_CAFE_BABE,
        }
    }

    /// Returns the number of elements in the skip list.
    ///
    /// **Time:** O(1). **Space:** O(1).
    pub fn len(&self) -> usize {
        self.length
    }

    /// Returns `true` if the skip list contains no elements.
    ///
    /// **Time:** O(1). **Space:** O(1).
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Searches for `val` in the skip list.
    ///
    /// **Time:** O(log n) expected. **Space:** O(1).
    pub fn search(&self, val: i32) -> bool {
        let mut current = 0; // start at head sentinel
        for lvl in (0..=self.level).rev() {
            while let Some(next_idx) = self.nodes[current].forward[lvl] {
                if self.nodes[next_idx].value < val {
                    current = next_idx;
                } else {
                    break;
                }
            }
        }
        // `current` is the last node whose value < val.
        // The node right after it at level 0 should be val (if it exists).
        if let Some(next_idx) = self.nodes[current].forward[0] {
            self.nodes[next_idx].value == val
        } else {
            false
        }
    }

    /// Inserts `val` into the skip list. Duplicates are allowed.
    ///
    /// **Time:** O(log n) expected. **Space:** O(log n) expected for the new node.
    pub fn insert(&mut self, val: i32) {
        // Find the update path — the last node at each level before the
        // insertion point.
        let mut update = [0usize; MAX_LEVEL];
        let mut current = 0;
        for lvl in (0..=self.level).rev() {
            while let Some(next_idx) = self.nodes[current].forward[lvl] {
                if self.nodes[next_idx].value < val {
                    current = next_idx;
                } else {
                    break;
                }
            }
            update[lvl] = current;
        }

        let new_level = self.random_level();
        if new_level > self.level {
            // New levels point back to head (index 0).
            for lvl in (self.level + 1)..=new_level {
                update[lvl] = 0;
            }
            self.level = new_level;
        }

        let new_node = Node {
            value: val,
            forward: vec![None; new_level + 1],
        };
        let new_idx = self.nodes.len();
        self.nodes.push(new_node);

        // Splice the new node into each level.
        for lvl in 0..=new_level {
            self.nodes[new_idx].forward[lvl] = self.nodes[update[lvl]].forward[lvl];
            self.nodes[update[lvl]].forward[lvl] = Some(new_idx);
        }

        self.length += 1;
    }

    /// Removes one occurrence of `val` from the skip list.
    ///
    /// Returns `true` if the value was found and removed, `false` otherwise.
    ///
    /// **Time:** O(log n) expected. **Space:** O(log n) for the update array.
    pub fn delete(&mut self, val: i32) -> bool {
        let mut update = [0usize; MAX_LEVEL];
        let mut current = 0;
        for lvl in (0..=self.level).rev() {
            while let Some(next_idx) = self.nodes[current].forward[lvl] {
                if self.nodes[next_idx].value < val {
                    current = next_idx;
                } else {
                    break;
                }
            }
            update[lvl] = current;
        }

        // Check if the candidate node exists and matches.
        let target = match self.nodes[current].forward[0] {
            Some(idx) if self.nodes[idx].value == val => idx,
            _ => return false,
        };

        // Unlink the target node from every level it appears in.
        for lvl in 0..=self.level {
            if self.nodes[update[lvl]].forward[lvl] == Some(target) {
                self.nodes[update[lvl]].forward[lvl] = self.nodes[target].forward[lvl];
            }
        }

        // Shrink the level if highest levels are now empty.
        while self.level > 0 && self.nodes[0].forward[self.level].is_none() {
            self.level -= 1;
        }

        self.length -= 1;
        true
    }

    /// Generates a random level for a new node (0-indexed).
    ///
    /// Each level has a 50% chance of being promoted to the next level,
    /// capped at `MAX_LEVEL - 1`.
    fn random_level(&mut self) -> usize {
        let mut lvl = 0;
        while lvl < MAX_LEVEL - 1 && self.next_random_bit() {
            lvl += 1;
        }
        lvl
    }

    /// Returns a pseudo-random boolean using xorshift64.
    fn next_random_bit(&mut self) -> bool {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        self.rng_state & 1 == 0
    }
}

impl Default for SkipList {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for SkipList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print each level as a list of values.
        writeln!(f, "SkipList (len={}, levels={}):", self.length, self.level + 1)?;
        for lvl in (0..=self.level).rev() {
            write!(f, "  L{lvl}: HEAD")?;
            let mut idx = 0;
            while let Some(next) = self.nodes[idx].forward[lvl] {
                write!(f, " -> {}", self.nodes[next].value)?;
                idx = next;
            }
            writeln!(f, " -> END")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- new / is_empty / len ----

    #[test]
    fn new_skip_list_is_empty() {
        let sl = SkipList::new();
        assert!(sl.is_empty());
        assert_eq!(sl.len(), 0);
    }

    #[test]
    fn default_is_empty() {
        let sl = SkipList::default();
        assert!(sl.is_empty());
    }

    // ---- insert ----

    #[test]
    fn insert_single() {
        let mut sl = SkipList::new();
        sl.insert(42);
        assert_eq!(sl.len(), 1);
        assert!(!sl.is_empty());
    }

    #[test]
    fn insert_multiple_sorted() {
        let mut sl = SkipList::new();
        for v in [10, 20, 30, 40, 50] {
            sl.insert(v);
        }
        assert_eq!(sl.len(), 5);
    }

    #[test]
    fn insert_multiple_unsorted() {
        let mut sl = SkipList::new();
        for v in [30, 10, 50, 20, 40] {
            sl.insert(v);
        }
        assert_eq!(sl.len(), 5);
        for v in [10, 20, 30, 40, 50] {
            assert!(sl.search(v), "should find {v}");
        }
    }

    #[test]
    fn insert_duplicates() {
        let mut sl = SkipList::new();
        sl.insert(5);
        sl.insert(5);
        sl.insert(5);
        assert_eq!(sl.len(), 3);
        assert!(sl.search(5));
    }

    #[test]
    fn insert_negative_values() {
        let mut sl = SkipList::new();
        sl.insert(-10);
        sl.insert(-5);
        sl.insert(0);
        sl.insert(5);
        assert_eq!(sl.len(), 4);
        assert!(sl.search(-10));
        assert!(sl.search(0));
    }

    // ---- search ----

    #[test]
    fn search_empty() {
        let sl = SkipList::new();
        assert!(!sl.search(1));
    }

    #[test]
    fn search_existing() {
        let mut sl = SkipList::new();
        for v in [1, 3, 5, 7, 9] {
            sl.insert(v);
        }
        for v in [1, 3, 5, 7, 9] {
            assert!(sl.search(v), "should find {v}");
        }
    }

    #[test]
    fn search_missing() {
        let mut sl = SkipList::new();
        for v in [1, 3, 5, 7, 9] {
            sl.insert(v);
        }
        for v in [0, 2, 4, 6, 8, 10] {
            assert!(!sl.search(v), "should NOT find {v}");
        }
    }

    // ---- delete ----

    #[test]
    fn delete_from_empty() {
        let mut sl = SkipList::new();
        assert!(!sl.delete(1));
    }

    #[test]
    fn delete_existing() {
        let mut sl = SkipList::new();
        sl.insert(10);
        sl.insert(20);
        sl.insert(30);

        assert!(sl.delete(20));
        assert!(!sl.search(20));
        assert_eq!(sl.len(), 2);
        assert!(sl.search(10));
        assert!(sl.search(30));
    }

    #[test]
    fn delete_missing() {
        let mut sl = SkipList::new();
        sl.insert(10);
        assert!(!sl.delete(99));
        assert_eq!(sl.len(), 1);
    }

    #[test]
    fn delete_one_duplicate() {
        let mut sl = SkipList::new();
        sl.insert(5);
        sl.insert(5);
        sl.insert(5);
        assert_eq!(sl.len(), 3);

        assert!(sl.delete(5));
        assert_eq!(sl.len(), 2);
        assert!(sl.search(5)); // still has two 5s

        assert!(sl.delete(5));
        assert!(sl.delete(5));
        assert_eq!(sl.len(), 0);
        assert!(!sl.search(5));
    }

    #[test]
    fn delete_all_elements() {
        let mut sl = SkipList::new();
        for v in [3, 1, 4, 1, 5] {
            sl.insert(v);
        }
        assert!(sl.delete(1));
        assert!(sl.delete(1));
        assert!(sl.delete(3));
        assert!(sl.delete(4));
        assert!(sl.delete(5));
        assert!(sl.is_empty());
    }

    // ---- stress / larger inputs ----

    #[test]
    fn insert_and_search_many() {
        let mut sl = SkipList::new();
        for v in 0..200 {
            sl.insert(v);
        }
        assert_eq!(sl.len(), 200);
        for v in 0..200 {
            assert!(sl.search(v), "should find {v}");
        }
        for v in 200..300 {
            assert!(!sl.search(v), "should NOT find {v}");
        }
    }

    #[test]
    fn insert_delete_interleaved() {
        let mut sl = SkipList::new();
        for v in [10, 20, 30, 40, 50] {
            sl.insert(v);
        }
        assert!(sl.delete(10));
        assert!(sl.delete(30));
        assert!(sl.delete(50));
        assert_eq!(sl.len(), 2);
        assert!(sl.search(20));
        assert!(sl.search(40));

        sl.insert(35);
        assert_eq!(sl.len(), 3);
        assert!(sl.search(35));
    }

    // ---- debug ----

    #[test]
    fn debug_format_does_not_panic() {
        let mut sl = SkipList::new();
        sl.insert(1);
        sl.insert(2);
        let output = format!("{:?}", sl);
        assert!(output.contains("SkipList"));
    }
}
