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

/// Check whether a byte slice is a palindrome using two pointers.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn is_palindrome(s: &[u8]) -> bool {
    if s.is_empty() {
        return true;
    }
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

/// Remove duplicates from a sorted `Vec` in-place (slow/fast pointer).
///
/// Returns the new length after deduplication.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn remove_duplicates_sorted(arr: &mut Vec<i32>) -> usize {
    if arr.is_empty() {
        return 0;
    }

    let mut slow = 0;
    for fast in 1..arr.len() {
        if arr[fast] != arr[slow] {
            slow += 1;
            arr[slow] = arr[fast];
        }
    }

    let new_len = slow + 1;
    arr.truncate(new_len);
    new_len
}

/// Find duplicates in a **sorted** slice using O(1) extra space.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn find_duplicates_sorted(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 1..arr.len() {
        if arr[i] == arr[i - 1] {
            if result.last() != Some(&arr[i]) {
                result.push(arr[i]);
            }
        }
    }
    result
}

/// Maximum sum of a contiguous subarray of length `k` (fixed-size sliding window).
///
/// Returns `None` if `k == 0` or `k > arr.len()`.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn max_sum_subarray_k(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None;
    }

    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;

    for i in k..arr.len() {
        window_sum += arr[i] - arr[i - k];
        max_sum = max_sum.max(window_sum);
    }

    Some(max_sum)
}

/// Length of the shortest subarray whose sum is ≥ `target` (variable-size sliding window).
///
/// Returns `None` if no such subarray exists.
///
/// **Time:** O(n) — **Space:** O(1)
pub fn min_subarray_len(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut sum = 0;
    let mut min_len = usize::MAX;

    for right in 0..arr.len() {
        sum += arr[right];

        while sum >= target {
            min_len = min_len.min(right - left + 1);
            sum -= arr[left];
            left += 1;
        }
    }

    if min_len == usize::MAX {
        None
    } else {
        Some(min_len)
    }
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

    // -- is_palindrome ---------------------------------------------------

    #[test]
    fn palindrome_odd() {
        assert!(is_palindrome(b"racecar"));
    }

    #[test]
    fn palindrome_even() {
        assert!(is_palindrome(b"abba"));
    }

    #[test]
    fn palindrome_not() {
        assert!(!is_palindrome(b"hello"));
    }

    #[test]
    fn palindrome_empty_and_single() {
        assert!(is_palindrome(b""));
        assert!(is_palindrome(b"x"));
    }

    // -- remove_duplicates_sorted ----------------------------------------

    #[test]
    fn remove_dups_sorted_basic() {
        let mut v = vec![1, 1, 2, 2, 3];
        assert_eq!(remove_duplicates_sorted(&mut v), 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn remove_dups_sorted_no_dups() {
        let mut v = vec![1, 2, 3];
        assert_eq!(remove_duplicates_sorted(&mut v), 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn remove_dups_sorted_all_same() {
        let mut v = vec![5, 5, 5, 5];
        assert_eq!(remove_duplicates_sorted(&mut v), 1);
        assert_eq!(v, vec![5]);
    }

    #[test]
    fn remove_dups_sorted_empty() {
        let mut v: Vec<i32> = vec![];
        assert_eq!(remove_duplicates_sorted(&mut v), 0);
    }

    // -- find_duplicates_sorted ------------------------------------------

    #[test]
    fn find_dups_sorted_basic() {
        assert_eq!(find_duplicates_sorted(&[1, 1, 2, 3, 3, 4]), vec![1, 3]);
    }

    #[test]
    fn find_dups_sorted_none() {
        assert_eq!(find_duplicates_sorted(&[1, 2, 3]), Vec::<i32>::new());
    }

    #[test]
    fn find_dups_sorted_all_same() {
        assert_eq!(find_duplicates_sorted(&[7, 7, 7]), vec![7]);
    }

    // -- max_sum_subarray_k ----------------------------------------------

    #[test]
    fn sliding_window_fixed_basic() {
        assert_eq!(max_sum_subarray_k(&[2, 1, 5, 1, 3, 2], 3), Some(9));
    }

    #[test]
    fn sliding_window_fixed_k_equals_len() {
        assert_eq!(max_sum_subarray_k(&[1, 2, 3], 3), Some(6));
    }

    #[test]
    fn sliding_window_fixed_k_too_large() {
        assert_eq!(max_sum_subarray_k(&[1, 2], 5), None);
    }

    #[test]
    fn sliding_window_fixed_k_zero() {
        assert_eq!(max_sum_subarray_k(&[1, 2, 3], 0), None);
    }

    // -- min_subarray_len ------------------------------------------------

    #[test]
    fn sliding_window_var_basic() {
        assert_eq!(min_subarray_len(&[2, 3, 1, 2, 4, 3], 7), Some(2));
    }

    #[test]
    fn sliding_window_var_whole_array() {
        assert_eq!(min_subarray_len(&[1, 1, 1, 1], 4), Some(4));
    }

    #[test]
    fn sliding_window_var_no_solution() {
        assert_eq!(min_subarray_len(&[1, 2, 3], 100), None);
    }

    #[test]
    fn sliding_window_var_single_element() {
        assert_eq!(min_subarray_len(&[10, 1, 2], 7), Some(1));
    }
}
