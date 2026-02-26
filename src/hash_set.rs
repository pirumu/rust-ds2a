//! # Hash Set
//!
//! A set backed by the custom [`HashMap`](crate::hash_map::HashMap).

use std::fmt::Debug;
use std::hash::Hash;

use crate::hash_map::HashMap;

/// A hash set implemented on top of our custom `HashMap`.
///
/// Internally, every element is stored as a key in the map with `()` as the
/// value. This mirrors how `std::collections::HashSet` is built on top of
/// `std::collections::HashMap`.
#[derive(Debug)]
pub struct HashSet<T: Hash + Eq + Debug> {
    map: HashMap<T, ()>,
}

impl<T: Hash + Eq + Debug> HashSet<T> {
    /// Creates an empty `HashSet`.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Adds a value to the set.
    ///
    /// Returns `true` if the value was newly inserted, `false` if it was
    /// already present.
    pub fn insert(&mut self, val: T) -> bool {
        self.map.insert(val, ()).is_none()
    }

    /// Removes a value from the set.
    ///
    /// Returns `true` if the value was present and removed.
    pub fn remove(&mut self, val: &T) -> bool {
        self.map.remove(val).is_some()
    }

    /// Returns `true` if the set contains the given value.
    pub fn contains(&self, val: &T) -> bool {
        self.map.contains_key(val)
    }
}

impl<T: Hash + Eq + Debug + Clone> HashSet<T> {
    /// Returns a new set containing all elements in `self` or `other` (or both).
    ///
    /// ```text
    ///   ┌─────────────────────────┐
    ///   │ A ∪ B  (shaded region)  │
    ///   │  ┌────┐    ┌────┐       │
    ///   │  │ A  │xxxx│  B │       │
    ///   │  └────┘    └────┘       │
    ///   └─────────────────────────┘
    /// ```
    pub fn union(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        for key in self.map.keys() {
            result.insert(key.clone());
        }
        for key in other.map.keys() {
            result.insert(key.clone());
        }
        result
    }

    /// Returns a new set containing elements present in both `self` and `other`.
    ///
    /// ```text
    ///       ┌────┐ ┌────┐
    ///       │ A  │x│  B │   x = A ∩ B
    ///       └────┘ └────┘
    /// ```
    pub fn intersection(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        for key in self.map.keys() {
            if other.contains(key) {
                result.insert(key.clone());
            }
        }
        result
    }

    /// Returns a new set containing elements in `self` but not in `other`.
    ///
    /// ```text
    ///       ┌────┐ ┌────┐
    ///       │xxA │ │  B │   x = A \ B
    ///       └────┘ └────┘
    /// ```
    pub fn difference(&self, other: &HashSet<T>) -> HashSet<T> {
        let mut result = HashSet::new();
        for key in self.map.keys() {
            if !other.contains(key) {
                result.insert(key.clone());
            }
        }
        result
    }
}

impl<T: Hash + Eq + Debug> Default for HashSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_set_is_empty() {
        let set: HashSet<i32> = HashSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn insert_and_contains() {
        let mut set = HashSet::new();
        assert!(set.insert(1));
        assert!(set.insert(2));
        assert!(!set.insert(1)); // duplicate
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(!set.contains(&3));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn remove() {
        let mut set = HashSet::new();
        set.insert("hello");
        set.insert("world");
        assert!(set.remove(&"hello"));
        assert!(!set.contains(&"hello"));
        assert!(!set.remove(&"hello")); // already gone
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn union_of_sets() {
        let mut a = HashSet::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);

        let mut b = HashSet::new();
        b.insert(3);
        b.insert(4);
        b.insert(5);

        let u = a.union(&b);
        assert_eq!(u.len(), 5);
        for i in 1..=5 {
            assert!(u.contains(&i));
        }
    }

    #[test]
    fn intersection_of_sets() {
        let mut a = HashSet::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);

        let mut b = HashSet::new();
        b.insert(2);
        b.insert(3);
        b.insert(4);

        let inter = a.intersection(&b);
        assert_eq!(inter.len(), 2);
        assert!(inter.contains(&2));
        assert!(inter.contains(&3));
        assert!(!inter.contains(&1));
        assert!(!inter.contains(&4));
    }

    #[test]
    fn difference_of_sets() {
        let mut a = HashSet::new();
        a.insert(1);
        a.insert(2);
        a.insert(3);

        let mut b = HashSet::new();
        b.insert(2);
        b.insert(4);

        let diff = a.difference(&b);
        assert_eq!(diff.len(), 2);
        assert!(diff.contains(&1));
        assert!(diff.contains(&3));
        assert!(!diff.contains(&2));
    }

    #[test]
    fn many_elements() {
        let mut set = HashSet::new();
        for i in 0..100 {
            assert!(set.insert(i));
        }
        assert_eq!(set.len(), 100);
        for i in 0..100 {
            assert!(set.contains(&i));
        }
    }

    #[test]
    fn insert_after_remove() {
        let mut set = HashSet::new();
        set.insert(5);
        set.remove(&5);
        assert!(set.insert(5));
        assert!(set.contains(&5));
        assert_eq!(set.len(), 1);
    }
}
