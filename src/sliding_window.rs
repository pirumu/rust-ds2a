//! # Sliding Window
//!
//! Fixed-size and variable-size sliding window algorithms for subarray/substring problems.

use std::collections::HashMap;
use std::collections::VecDeque;

/// Fixed-size sliding window: maximum sum of any contiguous sub-array of size `k`.
///
/// Returns `None` if `arr.len() < k` or `k == 0`.
/// **Time:** O(n).  **Space:** O(1).
pub fn max_sum_subarray_of_size_k(arr: &[i32], k: usize) -> Option<i32> {
    if k == 0 || arr.len() < k {
        return None;
    }
    let mut window_sum: i32 = arr[..k].iter().sum();
    let mut max_sum = window_sum;
    for i in k..arr.len() {
        window_sum += arr[i] - arr[i - k];
        if window_sum > max_sum {
            max_sum = window_sum;
        }
    }
    Some(max_sum)
}

/// Variable-size sliding window: length of the longest substring with no repeating characters.
///
/// **Time:** O(n).  **Space:** O(min(n, alphabet_size)).
pub fn longest_substring_no_repeat(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut last_seen: HashMap<u8, usize> = HashMap::new();
    let mut max_len: usize = 0;
    let mut start: usize = 0;

    for (i, &b) in bytes.iter().enumerate() {
        if let Some(&prev) = last_seen.get(&b) {
            if prev >= start {
                start = prev + 1;
            }
        }
        last_seen.insert(b, i);
        let current_len = i - start + 1;
        if current_len > max_len {
            max_len = current_len;
        }
    }
    max_len
}

/// Variable-size sliding window: length of the smallest contiguous sub-array
/// whose sum is >= `target`.
///
/// Returns `None` if no such sub-array exists.
/// **Time:** O(n).  **Space:** O(1).
pub fn smallest_subarray_with_sum(arr: &[i32], target: i32) -> Option<usize> {
    let mut window_sum: i32 = 0;
    let mut min_len: usize = usize::MAX;
    let mut start: usize = 0;

    for end in 0..arr.len() {
        window_sum += arr[end];
        while window_sum >= target {
            let current_len = end - start + 1;
            if current_len < min_len {
                min_len = current_len;
            }
            window_sum -= arr[start];
            start += 1;
        }
    }

    if min_len == usize::MAX {
        None
    } else {
        Some(min_len)
    }
}

/// Fixed-size sliding window: maximum element in each window of size `k`.
///
/// Uses a monotonic deque for O(n) total time.
/// Returns an empty vec if `k == 0` or `arr.len() < k`.
/// **Time:** O(n).  **Space:** O(k).
pub fn max_of_subarrays(arr: &[i32], k: usize) -> Vec<i32> {
    if k == 0 || arr.len() < k {
        return vec![];
    }
    let mut result = Vec::with_capacity(arr.len() - k + 1);
    // Deque stores indices; front is always the index of the current window max
    let mut deque: VecDeque<usize> = VecDeque::new();

    for i in 0..arr.len() {
        // Remove indices outside the window
        if let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            }
        }
        // Remove smaller elements from the back
        while let Some(&back) = deque.back() {
            if arr[back] <= arr[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        deque.push_back(i);

        // Window is fully formed starting at index k-1
        if i >= k - 1 {
            result.push(arr[deque[0]]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- max_sum_subarray_of_size_k ----
    #[test]
    fn fixed_window_sum() {
        let arr = [2, 1, 5, 1, 3, 2];
        assert_eq!(max_sum_subarray_of_size_k(&arr, 3), Some(9));
    }

    #[test]
    fn fixed_window_k_too_large() {
        let arr = [1, 2];
        assert_eq!(max_sum_subarray_of_size_k(&arr, 5), None);
    }

    #[test]
    fn fixed_window_k_zero() {
        assert_eq!(max_sum_subarray_of_size_k(&[1, 2, 3], 0), None);
    }

    #[test]
    fn fixed_window_single() {
        assert_eq!(max_sum_subarray_of_size_k(&[4, 2, 7], 1), Some(7));
    }

    // ---- longest_substring_no_repeat ----
    #[test]
    fn variable_window_basic() {
        assert_eq!(longest_substring_no_repeat("abcabcbb"), 3);
    }

    #[test]
    fn variable_window_all_same() {
        assert_eq!(longest_substring_no_repeat("bbbbb"), 1);
    }

    #[test]
    fn variable_window_empty() {
        assert_eq!(longest_substring_no_repeat(""), 0);
    }

    #[test]
    fn variable_window_unique() {
        assert_eq!(longest_substring_no_repeat("abcdef"), 6);
    }

    // ---- smallest_subarray_with_sum ----
    #[test]
    fn smallest_sub_basic() {
        assert_eq!(smallest_subarray_with_sum(&[2, 1, 5, 2, 3, 2], 7), Some(2));
    }

    #[test]
    fn smallest_sub_exact() {
        assert_eq!(smallest_subarray_with_sum(&[3, 4, 1, 1, 6], 8), Some(3));
    }

    #[test]
    fn smallest_sub_impossible() {
        assert_eq!(smallest_subarray_with_sum(&[1, 1, 1], 100), None);
    }

    #[test]
    fn smallest_sub_single_element() {
        assert_eq!(smallest_subarray_with_sum(&[10, 1, 2], 5), Some(1));
    }

    // ---- max_of_subarrays ----
    #[test]
    fn max_subarrays_basic() {
        assert_eq!(max_of_subarrays(&[1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn max_subarrays_k_equals_len() {
        assert_eq!(max_of_subarrays(&[4, 2, 7], 3), vec![7]);
    }

    #[test]
    fn max_subarrays_k_one() {
        assert_eq!(max_of_subarrays(&[5, 3, 8], 1), vec![5, 3, 8]);
    }

    #[test]
    fn max_subarrays_empty() {
        assert_eq!(max_of_subarrays(&[1, 2], 3), vec![]);
    }
}
