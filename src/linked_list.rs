//! # Linked Lists
//!
//! Singly and doubly linked list implementations in safe Rust.

use std::cell::RefCell;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Singly Linked List
// ---------------------------------------------------------------------------

/// A node in a singly linked list.
#[derive(Debug)]
struct SinglyNode<T> {
    val: T,
    next: Option<Box<SinglyNode<T>>>,
}

/// A singly linked list that owns its elements on the heap.
///
/// Each node is stored in a `Box`, and the list is connected through
/// `Option<Box<SinglyNode<T>>>` pointers.
#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Option<Box<SinglyNode<T>>>,
    len: usize,
}

impl<T> SinglyLinkedList<T> {
    /// Creates an empty singly linked list.
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Inserts a value at the front of the list. O(1).
    pub fn push_front(&mut self, val: T) {
        let new_node = Box::new(SinglyNode {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    /// Removes and returns the value at the front of the list. O(1).
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.val
        })
    }

    /// Inserts a value at the back of the list. O(n).
    pub fn push_back(&mut self, val: T) {
        let new_node = Box::new(SinglyNode { val, next: None });
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
        self.len += 1;
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Reverses the list in place. O(n).
    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }

    /// Returns an iterator over references to the list's elements.
    pub fn iter(&self) -> SinglyIter<'_, T> {
        SinglyIter {
            current: self.head.as_deref(),
        }
    }

    /// Returns a reference to the nth element (0-indexed). O(n).
    pub fn nth(&self, n: usize) -> Option<&T> {
        self.iter().nth(n)
    }

    /// Finds the middle element using the slow/fast pointer technique. O(n).
    ///
    /// For even-length lists, returns the first of the two middle nodes.
    pub fn find_middle(&self) -> Option<&T> {
        let mut slow = self.head.as_deref()?;
        let mut fast = self.head.as_deref()?;

        while let Some(next) = fast.next.as_deref() {
            if let Some(next_next) = next.next.as_deref() {
                fast = next_next;
                slow = slow.next.as_deref().unwrap();
            } else {
                break;
            }
        }
        Some(&slow.val)
    }
}

impl<T: PartialEq> SinglyLinkedList<T> {
    /// Returns `true` if the list contains an element equal to `target`. O(n).
    pub fn find(&self, target: &T) -> bool {
        self.iter().any(|val| val == target)
    }
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator over `&T` values in a `SinglyLinkedList`.
pub struct SinglyIter<'a, T> {
    current: Option<&'a SinglyNode<T>>,
}

impl<'a, T> Iterator for SinglyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.val
        })
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    /// Iterative drop to avoid stack overflow on very long lists.
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

/// Merges two sorted singly linked lists into one sorted list. O(n+m).
///
/// Pops from the front of each list in order, pushes to a result list in
/// reverse, then reverses the result at the end.
pub fn merge_sorted_lists<T: Ord>(
    mut list1: SinglyLinkedList<T>,
    mut list2: SinglyLinkedList<T>,
) -> SinglyLinkedList<T> {
    let mut result = SinglyLinkedList::new();
    loop {
        let take_from_1 = match (list1.head.as_ref(), list2.head.as_ref()) {
            (None, None) => break,
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (Some(a), Some(b)) => a.val <= b.val,
        };
        if take_from_1 {
            if let Some(val) = list1.pop_front() {
                result.push_front(val);
            }
        } else {
            if let Some(val) = list2.pop_front() {
                result.push_front(val);
            }
        }
    }
    result.reverse();
    result
}

// ---------------------------------------------------------------------------
// Doubly Linked List  (Rc<RefCell<..>> approach)
// ---------------------------------------------------------------------------

type Link<T> = Option<Rc<RefCell<DoublyNode<T>>>>;

/// A node in a doubly linked list.
#[derive(Debug)]
struct DoublyNode<T> {
    val: T,
    prev: Link<T>,
    next: Link<T>,
}

/// A doubly linked list using `Rc<RefCell<Node>>` for shared ownership.
///
/// Each node is reference-counted so that both `prev` and `next` pointers
/// can coexist. The `prev` pointer is a strong `Rc` as well; the list is
/// kept acyclic by explicitly clearing pointers on removal.
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    /// Creates an empty doubly linked list.
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    /// Inserts a value at the front of the list. O(1).
    pub fn push_front(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(DoublyNode {
            val,
            prev: None,
            next: self.head.clone(),
        }));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
        self.len += 1;
    }

    /// Inserts a value at the back of the list. O(1).
    pub fn push_back(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(DoublyNode {
            val,
            prev: self.tail.clone(),
            next: None,
        }));
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.len += 1;
    }

    /// Removes and returns the value at the front. O(1).
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    /// Removes and returns the value at the back. O(1).
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head = None;
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns an iterator that traverses from head to tail (forward).
    pub fn iter(&self) -> DoublyIter<T> {
        DoublyIter {
            current: self.head.clone(),
        }
    }

    /// Returns an iterator that traverses from tail to head (backward).
    pub fn iter_back(&self) -> DoublyIterBack<T> {
        DoublyIterBack {
            current: self.tail.clone(),
        }
    }
}

/// Forward iterator over a `DoublyLinkedList`, yielding cloned values.
pub struct DoublyIter<T> {
    current: Link<T>,
}

impl<T: Clone> Iterator for DoublyIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let borrowed = node.borrow();
            self.current = borrowed.next.clone();
            borrowed.val.clone()
        })
    }
}

/// Backward iterator over a `DoublyLinkedList`, yielding cloned values.
pub struct DoublyIterBack<T> {
    current: Link<T>,
}

impl<T: Clone> Iterator for DoublyIterBack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let borrowed = node.borrow();
            self.current = borrowed.prev.clone();
            borrowed.val.clone()
        })
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    /// Break all `Rc` cycles so memory is freed.
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Singly Linked List tests (10+) ---

    #[test]
    fn singly_new_is_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn singly_push_front() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        let vals: Vec<_> = list.iter().copied().collect();
        assert_eq!(vals, vec![3, 2, 1]);
    }

    #[test]
    fn singly_pop_front() {
        let mut list = SinglyLinkedList::new();
        list.push_front(10);
        list.push_front(20);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn singly_push_back() {
        let mut list = SinglyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let vals: Vec<_> = list.iter().copied().collect();
        assert_eq!(vals, vec![1, 2, 3]);
    }

    #[test]
    fn singly_push_front_and_back_mixed() {
        let mut list = SinglyLinkedList::new();
        list.push_back(2);
        list.push_front(1);
        list.push_back(3);
        let vals: Vec<_> = list.iter().copied().collect();
        assert_eq!(vals, vec![1, 2, 3]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn singly_reverse_empty() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        list.reverse();
        assert!(list.is_empty());
    }

    #[test]
    fn singly_reverse() {
        let mut list = SinglyLinkedList::new();
        for i in 1..=5 {
            list.push_back(i);
        }
        list.reverse();
        let vals: Vec<_> = list.iter().copied().collect();
        assert_eq!(vals, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn singly_reverse_single() {
        let mut list = SinglyLinkedList::new();
        list.push_front(42);
        list.reverse();
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![42]);
    }

    #[test]
    fn singly_iter_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!(list.iter().count(), 0);
    }

    #[test]
    fn singly_len_tracks_correctly() {
        let mut list = SinglyLinkedList::new();
        assert_eq!(list.len(), 0);
        list.push_front(1);
        assert_eq!(list.len(), 1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
        list.pop_front();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn singly_with_strings() {
        let mut list = SinglyLinkedList::new();
        list.push_back(String::from("hello"));
        list.push_back(String::from("world"));
        let vals: Vec<_> = list.iter().map(|s| s.as_str()).collect();
        assert_eq!(vals, vec!["hello", "world"]);
    }

    // --- Doubly Linked List tests (8+) ---

    #[test]
    fn doubly_new_is_empty() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn doubly_push_front() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn doubly_push_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn doubly_pop_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn doubly_mixed_operations() {
        let mut list = DoublyLinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        list.push_back(4);
        assert_eq!(list.len(), 4);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn doubly_pop_front_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn doubly_pop_back_empty() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn doubly_single_element() {
        let mut list = DoublyLinkedList::new();
        list.push_front(42);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(42));
        assert!(list.is_empty());
    }

    #[test]
    fn doubly_iter_forward() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let vals: Vec<_> = list.iter().collect();
        assert_eq!(vals, vec![1, 2, 3]);
    }

    #[test]
    fn doubly_iter_back() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let vals: Vec<_> = list.iter_back().collect();
        assert_eq!(vals, vec![3, 2, 1]);
    }

    #[test]
    fn doubly_iter_empty() {
        let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(list.iter().count(), 0);
        assert_eq!(list.iter_back().count(), 0);
    }

    #[test]
    fn doubly_iter_single() {
        let mut list = DoublyLinkedList::new();
        list.push_front(42);
        assert_eq!(list.iter().collect::<Vec<_>>(), vec![42]);
        assert_eq!(list.iter_back().collect::<Vec<_>>(), vec![42]);
    }

    #[test]
    fn doubly_len_tracks() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        list.pop_front();
        assert_eq!(list.len(), 1);
        list.pop_back();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    // --- nth ---

    #[test]
    fn singly_nth() {
        let mut list = SinglyLinkedList::new();
        for i in 0..5 {
            list.push_back(i);
        }
        assert_eq!(list.nth(0), Some(&0));
        assert_eq!(list.nth(2), Some(&2));
        assert_eq!(list.nth(4), Some(&4));
        assert_eq!(list.nth(5), None);
    }

    #[test]
    fn singly_nth_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!(list.nth(0), None);
    }

    // --- find ---

    #[test]
    fn singly_find() {
        let mut list = SinglyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert!(list.find(&2));
        assert!(!list.find(&99));
    }

    #[test]
    fn singly_find_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert!(!list.find(&1));
    }

    // --- find_middle ---

    #[test]
    fn singly_find_middle_odd() {
        let mut list = SinglyLinkedList::new();
        for i in 1..=5 {
            list.push_back(i);
        }
        assert_eq!(list.find_middle(), Some(&3));
    }

    #[test]
    fn singly_find_middle_even() {
        let mut list = SinglyLinkedList::new();
        for i in 1..=4 {
            list.push_back(i);
        }
        // For even length, returns the first of two middle nodes
        assert_eq!(list.find_middle(), Some(&2));
    }

    #[test]
    fn singly_find_middle_single() {
        let mut list = SinglyLinkedList::new();
        list.push_front(42);
        assert_eq!(list.find_middle(), Some(&42));
    }

    #[test]
    fn singly_find_middle_empty() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::new();
        assert_eq!(list.find_middle(), None);
    }

    // --- merge_sorted_lists ---

    #[test]
    fn singly_merge_sorted() {
        let mut l1 = SinglyLinkedList::new();
        let mut l2 = SinglyLinkedList::new();
        for &v in &[1, 3, 5] {
            l1.push_back(v);
        }
        for &v in &[2, 4, 6] {
            l2.push_back(v);
        }
        let merged = merge_sorted_lists(l1, l2);
        let vals: Vec<_> = merged.iter().copied().collect();
        assert_eq!(vals, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn singly_merge_sorted_empty() {
        let l1: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let mut l2 = SinglyLinkedList::new();
        l2.push_back(1);
        l2.push_back(2);
        let merged = merge_sorted_lists(l1, l2);
        let vals: Vec<_> = merged.iter().copied().collect();
        assert_eq!(vals, vec![1, 2]);
    }

    #[test]
    fn singly_merge_sorted_both_empty() {
        let l1: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let l2: SinglyLinkedList<i32> = SinglyLinkedList::new();
        let merged = merge_sorted_lists(l1, l2);
        assert!(merged.is_empty());
    }
}
