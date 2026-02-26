//! # Arrays & Slices
//!
//! Common array and slice operations including searching, reversing,
//! rotating, and subarray problems.

use std::collections::HashSet;

/// Linear search: scan every element until the target is found.
///
/// Returns `Some(index)` of the first match, or `None`.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

/// Binary search on a **sorted** slice.
///
/// Returns `Some(index)` of a matching element, or `None`.
///
/// **Time:** O(log n) — **Space:** O(1)
pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match arr[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
        }
    }
    None
}

/// Reverse a mutable slice in-place using the two-pointer technique.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn reverse<T>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let mut left = 0;
    let mut right = len - 1;
    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}

/// Rotate a slice to the left by `k` positions.
///
/// Uses the "three reverses" trick:
/// 1. Reverse the first `k` elements.
/// 2. Reverse the remaining elements.
/// 3. Reverse the whole slice.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn rotate_left<T>(arr: &mut [T], k: usize) {
    let len = arr.len();
    if len == 0 {
        return;
    }
    let k = k % len;
    if k == 0 {
        return;
    }
    arr[..k].reverse();
    arr[k..].reverse();
    arr.reverse();
}

/// Find all duplicate values in a slice of `i32`s.
///
/// Returns a `Vec` of values that appear more than once (each listed once).
///
/// **Time:** O(n) — **Space:** O(n)
pub fn find_duplicates(arr: &[i32]) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for &val in arr {
        if !seen.insert(val) {
            duplicates.insert(val);
        }
    }

    let mut result: Vec<i32> = duplicates.into_iter().collect();
    result.sort();
    result
}

/// Maximum subarray sum using Kadane's algorithm.
///
/// Finds the contiguous subarray with the largest sum.
/// If the slice is empty, returns 0.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut max_ending_here = arr[0];
    let mut max_so_far = arr[0];

    for &val in &arr[1..] {
        max_ending_here = val.max(max_ending_here + val);
        max_so_far = max_so_far.max(max_ending_here);
    }

    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- linear_search ---------------------------------------------------

    #[test]
    fn linear_search_found() {
        assert_eq!(linear_search(&[10, 20, 30, 40], &30), Some(2));
    }

    #[test]
    fn linear_search_not_found() {
        assert_eq!(linear_search(&[10, 20, 30], &99), None);
    }

    #[test]
    fn linear_search_empty() {
        let empty: &[i32] = &[];
        assert_eq!(linear_search(empty, &1), None);
    }

    // -- binary_search ---------------------------------------------------

    #[test]
    fn binary_search_found() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], &5), Some(2));
    }

    #[test]
    fn binary_search_not_found() {
        assert_eq!(binary_search(&[1, 3, 5, 7, 9], &4), None);
    }

    #[test]
    fn binary_search_single_element() {
        assert_eq!(binary_search(&[42], &42), Some(0));
        assert_eq!(binary_search(&[42], &7), None);
    }

    // -- reverse ---------------------------------------------------------

    #[test]
    fn reverse_even_length() {
        let mut v = vec![1, 2, 3, 4];
        reverse(&mut v);
        assert_eq!(v, vec![4, 3, 2, 1]);
    }

    #[test]
    fn reverse_odd_length() {
        let mut v = vec![1, 2, 3, 4, 5];
        reverse(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn reverse_empty_and_single() {
        let mut empty: Vec<i32> = vec![];
        reverse(&mut empty);
        assert_eq!(empty, vec![]);

        let mut one = vec![7];
        reverse(&mut one);
        assert_eq!(one, vec![7]);
    }

    // -- rotate_left -----------------------------------------------------

    #[test]
    fn rotate_left_basic() {
        let mut v = vec![1, 2, 3, 4, 5];
        rotate_left(&mut v, 2);
        assert_eq!(v, vec![3, 4, 5, 1, 2]);
    }

    #[test]
    fn rotate_left_full_rotation() {
        let mut v = vec![1, 2, 3];
        rotate_left(&mut v, 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn rotate_left_more_than_length() {
        let mut v = vec![1, 2, 3, 4];
        rotate_left(&mut v, 6); // 6 % 4 == 2
        assert_eq!(v, vec![3, 4, 1, 2]);
    }

    // -- find_duplicates -------------------------------------------------

    #[test]
    fn find_duplicates_basic() {
        assert_eq!(find_duplicates(&[1, 2, 3, 2, 4, 3]), vec![2, 3]);
    }

    #[test]
    fn find_duplicates_none() {
        assert_eq!(find_duplicates(&[1, 2, 3, 4]), Vec::<i32>::new());
    }

    #[test]
    fn find_duplicates_all_same() {
        assert_eq!(find_duplicates(&[5, 5, 5]), vec![5]);
    }

    // -- max_subarray_sum ------------------------------------------------

    #[test]
    fn max_subarray_sum_mixed() {
        assert_eq!(max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn max_subarray_sum_all_negative() {
        assert_eq!(max_subarray_sum(&[-3, -1, -4]), -1);
    }

    #[test]
    fn max_subarray_sum_empty() {
        assert_eq!(max_subarray_sum(&[]), 0);
    }
}
