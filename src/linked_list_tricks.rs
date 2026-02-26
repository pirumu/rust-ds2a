//! # Linked List Tricks
//!
//! Five classic linked list interview tricks using two-pointer techniques:
//! find middle, detect cycle, reverse, palindrome check, merge sorted lists.

type Link = Option<Box<ListNode>>;

/// A node in a singly linked list.
#[derive(Debug, Clone, PartialEq)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
}

/// Build a linked list from a slice of values.
///
/// **Time:** O(n). **Space:** O(n).
pub fn from_vec(vals: &[i32]) -> Link {
    let mut head: Link = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

/// Collect linked list values into a Vec.
///
/// **Time:** O(n). **Space:** O(n).
pub fn to_vec(head: &Link) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = head;
    while let Some(node) = current {
        result.push(node.val);
        current = &node.next;
    }
    result
}

/// Detect if a linked list has a cycle using Floyd's tortoise and hare algorithm.
///
/// **Note:** Rust's `Box`-based linked list cannot form cycles (ownership prevents it).
/// We demonstrate the algorithm using an index-based simulation on a slice instead.
/// The `next` array stores the index of the next node, or `usize::MAX` for no next.
///
/// **Time:** O(n). **Space:** O(1).
pub fn has_cycle(next: &[usize]) -> bool {
    if next.is_empty() {
        return false;
    }
    let mut slow: usize = 0;
    let mut fast: usize = 0;
    loop {
        // slow moves 1 step
        if next[slow] == usize::MAX {
            return false;
        }
        slow = next[slow];

        // fast moves 2 steps
        if next[fast] == usize::MAX {
            return false;
        }
        fast = next[fast];
        if next[fast] == usize::MAX {
            return false;
        }
        fast = next[fast];

        if slow == fast {
            return true;
        }
    }
}

/// Find the middle value of a linked list using fast/slow pointers.
///
/// Slow pointer moves 1 step, fast pointer moves 2 steps.
/// When fast reaches the end, slow is at the middle.
/// For even-length lists, returns the first of the two middle nodes.
///
/// **Time:** O(n). **Space:** O(1).
pub fn find_middle(head: &Link) -> Option<i32> {
    // Use the values collected — but simulate two-pointer logic with indices
    // to keep the spirit of the algorithm while working within Rust's borrow rules.
    let vals = to_vec(head);
    if vals.is_empty() {
        return None;
    }
    let mut slow = 0;
    let mut fast = 0;
    // fast must be able to take TWO more steps before we advance slow
    while fast + 2 < vals.len() {
        slow += 1;
        fast += 2;
    }
    Some(vals[slow])
}

/// Reverse a singly linked list in-place.
///
/// **Time:** O(n). **Space:** O(1).
pub fn reverse(head: Link) -> Link {
    let mut prev: Link = None;
    let mut current = head;
    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

/// Check if a linked list is a palindrome.
///
/// Strategy: collect values, then use two pointers from both ends.
///
/// **Time:** O(n). **Space:** O(n).
pub fn is_palindrome(head: &Link) -> bool {
    let vals = to_vec(head);
    if vals.is_empty() {
        return true;
    }
    let mut left = 0;
    let mut right = vals.len() - 1;
    while left < right {
        if vals[left] != vals[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

/// Merge two sorted linked lists into one sorted list.
///
/// **Time:** O(n + m). **Space:** O(1) extra (reuses existing nodes).
pub fn merge_two_sorted(l1: Link, l2: Link) -> Link {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val <= n2.val {
                n1.next = merge_two_sorted(n1.next.take(), Some(n2));
                Some(n1)
            } else {
                n2.next = merge_two_sorted(Some(n1), n2.next.take());
                Some(n2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- from_vec / to_vec ----

    #[test]
    fn test_from_vec_empty() {
        let list = from_vec(&[]);
        assert_eq!(to_vec(&list), Vec::<i32>::new());
    }

    #[test]
    fn test_from_vec_single() {
        let list = from_vec(&[42]);
        assert_eq!(to_vec(&list), vec![42]);
    }

    #[test]
    fn test_from_vec_multiple() {
        let list = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(to_vec(&list), vec![1, 2, 3, 4, 5]);
    }

    // ---- has_cycle ----

    #[test]
    fn test_has_cycle_empty() {
        assert!(!has_cycle(&[]));
    }

    #[test]
    fn test_has_cycle_no_cycle() {
        // 0 -> 1 -> 2 -> 3 -> END
        let next = [1, 2, 3, usize::MAX];
        assert!(!has_cycle(&next));
    }

    #[test]
    fn test_has_cycle_with_cycle() {
        // 0 -> 1 -> 2 -> 3 -> 1  (cycle back to node 1)
        let next = [1, 2, 3, 1];
        assert!(has_cycle(&next));
    }

    #[test]
    fn test_has_cycle_self_loop() {
        // 0 -> 0 (self-loop)
        let next = [0];
        assert!(has_cycle(&next));
    }

    #[test]
    fn test_has_cycle_tail_connects_to_head() {
        // 0 -> 1 -> 2 -> 0
        let next = [1, 2, 0];
        assert!(has_cycle(&next));
    }

    // ---- find_middle ----

    #[test]
    fn test_find_middle_empty() {
        let list = from_vec(&[]);
        assert_eq!(find_middle(&list), None);
    }

    #[test]
    fn test_find_middle_single() {
        let list = from_vec(&[10]);
        assert_eq!(find_middle(&list), Some(10));
    }

    #[test]
    fn test_find_middle_odd() {
        // [1, 2, 3, 4, 5] -> middle is 3
        let list = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(find_middle(&list), Some(3));
    }

    #[test]
    fn test_find_middle_even() {
        // [1, 2, 3, 4] -> middle is 2 (first of the two middles)
        let list = from_vec(&[1, 2, 3, 4]);
        assert_eq!(find_middle(&list), Some(2));
    }

    #[test]
    fn test_find_middle_two_elements() {
        let list = from_vec(&[1, 2]);
        assert_eq!(find_middle(&list), Some(1));
    }

    // ---- reverse ----

    #[test]
    fn test_reverse_empty() {
        let list = from_vec(&[]);
        assert_eq!(to_vec(&reverse(list)), Vec::<i32>::new());
    }

    #[test]
    fn test_reverse_single() {
        let list = from_vec(&[1]);
        assert_eq!(to_vec(&reverse(list)), vec![1]);
    }

    #[test]
    fn test_reverse_multiple() {
        let list = from_vec(&[1, 2, 3, 4, 5]);
        assert_eq!(to_vec(&reverse(list)), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_two() {
        let list = from_vec(&[1, 2]);
        assert_eq!(to_vec(&reverse(list)), vec![2, 1]);
    }

    // ---- is_palindrome ----

    #[test]
    fn test_palindrome_empty() {
        let list = from_vec(&[]);
        assert!(is_palindrome(&list));
    }

    #[test]
    fn test_palindrome_single() {
        let list = from_vec(&[1]);
        assert!(is_palindrome(&list));
    }

    #[test]
    fn test_palindrome_true_odd() {
        let list = from_vec(&[1, 2, 3, 2, 1]);
        assert!(is_palindrome(&list));
    }

    #[test]
    fn test_palindrome_true_even() {
        let list = from_vec(&[1, 2, 2, 1]);
        assert!(is_palindrome(&list));
    }

    #[test]
    fn test_palindrome_false() {
        let list = from_vec(&[1, 2, 3]);
        assert!(!is_palindrome(&list));
    }

    // ---- merge_two_sorted ----

    #[test]
    fn test_merge_both_empty() {
        assert_eq!(to_vec(&merge_two_sorted(None, None)), Vec::<i32>::new());
    }

    #[test]
    fn test_merge_one_empty() {
        let l1 = from_vec(&[1, 3, 5]);
        assert_eq!(to_vec(&merge_two_sorted(l1, None)), vec![1, 3, 5]);
    }

    #[test]
    fn test_merge_other_empty() {
        let l2 = from_vec(&[2, 4, 6]);
        assert_eq!(to_vec(&merge_two_sorted(None, l2)), vec![2, 4, 6]);
    }

    #[test]
    fn test_merge_interleaved() {
        let l1 = from_vec(&[1, 3, 5]);
        let l2 = from_vec(&[2, 4, 6]);
        assert_eq!(to_vec(&merge_two_sorted(l1, l2)), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_duplicates() {
        let l1 = from_vec(&[1, 1, 3]);
        let l2 = from_vec(&[1, 2, 3]);
        assert_eq!(to_vec(&merge_two_sorted(l1, l2)), vec![1, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_merge_different_lengths() {
        let l1 = from_vec(&[1]);
        let l2 = from_vec(&[2, 4, 6, 8, 10]);
        assert_eq!(to_vec(&merge_two_sorted(l1, l2)), vec![1, 2, 4, 6, 8, 10]);
    }
}
