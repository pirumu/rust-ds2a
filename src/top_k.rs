//! # Top-K Problems
//!
//! Quick Select, top-K frequent elements, K closest points, and merging
//! K sorted arrays — classic interview patterns built on heaps and partitioning.

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

// ---------------------------------------------------------------------------
// Quick Select
// ---------------------------------------------------------------------------

/// Find the k-th smallest element in an unsorted array (0-indexed).
///
/// Uses the Quick Select algorithm (Hoare's selection), which is essentially
/// Quick Sort but only recurses into the partition that contains index `k`.
///
/// **Time:** O(n) average, O(n^2) worst case.  **Space:** O(1).
pub fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    assert!(!arr.is_empty(), "array must not be empty");
    assert!(k < arr.len(), "k must be < array length");
    let n = arr.len();
    quick_select_helper(arr, 0, n - 1, k)
}

fn quick_select_helper(arr: &mut [i32], lo: usize, hi: usize, k: usize) -> i32 {
    if lo == hi {
        return arr[lo];
    }

    let pivot_idx = partition(arr, lo, hi);

    if k == pivot_idx {
        arr[k]
    } else if k < pivot_idx {
        // pivot_idx > 0 guaranteed because k < pivot_idx and k >= lo >= 0
        quick_select_helper(arr, lo, pivot_idx - 1, k)
    } else {
        quick_select_helper(arr, pivot_idx + 1, hi, k)
    }
}

/// Lomuto partition: pick last element as pivot, return its final position.
fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi];
    let mut i = lo;
    for j in lo..hi {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, hi);
    i
}

// ---------------------------------------------------------------------------
// Top K Frequent Elements
// ---------------------------------------------------------------------------

/// Return the `k` most frequent elements from `nums`.
///
/// Uses a HashMap for counting and a min-heap (BinaryHeap<Reverse<…>>) of
/// size `k` to extract the top-K frequencies.
///
/// **Time:** O(n log k).  **Space:** O(n).
pub fn top_k_frequent(nums: &[i32], k: usize) -> Vec<i32> {
    assert!(k > 0, "k must be > 0");

    // Count frequencies.
    let mut freq: HashMap<i32, usize> = HashMap::new();
    for &num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    // Min-heap of size k: (frequency, value).
    let mut heap: BinaryHeap<Reverse<(usize, i32)>> = BinaryHeap::new();

    for (&val, &count) in &freq {
        heap.push(Reverse((count, val)));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_iter().map(|Reverse((_, val))| val).collect()
}

// ---------------------------------------------------------------------------
// K Closest Points to Origin
// ---------------------------------------------------------------------------

/// Return the `k` points closest to the origin (0, 0).
///
/// Uses a max-heap of size `k` keyed by squared distance so that the
/// farthest of the current top-K can be evicted cheaply.
///
/// **Time:** O(n log k).  **Space:** O(k).
pub fn k_closest_points(points: &[[i32; 2]], k: usize) -> Vec<[i32; 2]> {
    assert!(k > 0 && k <= points.len(), "k must be in 1..=points.len()");

    // Max-heap: (squared_distance, point).
    let mut heap: BinaryHeap<(i64, [i32; 2])> = BinaryHeap::new();

    for &p in points {
        let dist = (p[0] as i64) * (p[0] as i64) + (p[1] as i64) * (p[1] as i64);
        heap.push((dist, p));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_iter().map(|(_, p)| p).collect()
}

// ---------------------------------------------------------------------------
// Merge K Sorted Arrays
// ---------------------------------------------------------------------------

/// Merge `k` individually-sorted arrays into one sorted array.
///
/// Uses a min-heap that always holds one element per non-exhausted list.
///
/// **Time:** O(N log k) where N = total elements.  **Space:** O(N + k).
pub fn merge_k_sorted(lists: Vec<Vec<i32>>) -> Vec<i32> {
    // Min-heap entries: (value, list_index, element_index).
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    // Seed with the first element of each non-empty list.
    for (i, list) in lists.iter().enumerate() {
        if !list.is_empty() {
            heap.push(Reverse((list[0], i, 0)));
        }
    }

    let total: usize = lists.iter().map(|l| l.len()).sum();
    let mut result = Vec::with_capacity(total);

    while let Some(Reverse((val, list_idx, elem_idx))) = heap.pop() {
        result.push(val);
        let next = elem_idx + 1;
        if next < lists[list_idx].len() {
            heap.push(Reverse((lists[list_idx][next], list_idx, next)));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- quick_select ----

    #[test]
    fn quick_select_basic() {
        let mut arr = [3, 2, 1, 5, 4];
        assert_eq!(quick_select(&mut arr, 0), 1); // smallest
        let mut arr = [3, 2, 1, 5, 4];
        assert_eq!(quick_select(&mut arr, 4), 5); // largest
        let mut arr = [3, 2, 1, 5, 4];
        assert_eq!(quick_select(&mut arr, 2), 3); // median
    }

    #[test]
    fn quick_select_single() {
        let mut arr = [42];
        assert_eq!(quick_select(&mut arr, 0), 42);
    }

    #[test]
    fn quick_select_duplicates() {
        let mut arr = [3, 3, 3, 1, 1];
        assert_eq!(quick_select(&mut arr, 0), 1);
        let mut arr = [3, 3, 3, 1, 1];
        assert_eq!(quick_select(&mut arr, 2), 3);
    }

    #[test]
    fn quick_select_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(quick_select(&mut arr, 3), 4);
    }

    #[test]
    fn quick_select_reverse_sorted() {
        let mut arr = [5, 4, 3, 2, 1];
        assert_eq!(quick_select(&mut arr, 1), 2);
    }

    #[test]
    fn quick_select_negative() {
        let mut arr = [-5, -1, -3, 0, 2];
        assert_eq!(quick_select(&mut arr, 0), -5);
        let mut arr = [-5, -1, -3, 0, 2];
        assert_eq!(quick_select(&mut arr, 4), 2);
    }

    #[test]
    #[should_panic]
    fn quick_select_empty_panics() {
        quick_select(&mut [], 0);
    }

    #[test]
    #[should_panic]
    fn quick_select_k_oob_panics() {
        quick_select(&mut [1, 2], 5);
    }

    // ---- top_k_frequent ----

    #[test]
    fn top_k_frequent_basic() {
        let mut result = top_k_frequent(&[1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn top_k_frequent_single() {
        let result = top_k_frequent(&[1], 1);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn top_k_frequent_all_same() {
        let result = top_k_frequent(&[5, 5, 5, 5], 1);
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn top_k_frequent_all_unique() {
        let mut result = top_k_frequent(&[1, 2, 3, 4], 4);
        result.sort();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn top_k_frequent_negative() {
        let mut result = top_k_frequent(&[-1, -1, -2, -2, -2, 3], 1);
        result.sort();
        assert_eq!(result, vec![-2]);
    }

    #[test]
    #[should_panic]
    fn top_k_frequent_zero_k_panics() {
        top_k_frequent(&[1, 2, 3], 0);
    }

    // ---- k_closest_points ----

    #[test]
    fn k_closest_basic() {
        let points = [[1, 3], [-2, 2]];
        let result = k_closest_points(&points, 1);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], [-2, 2]); // distance^2 = 8 vs 10
    }

    #[test]
    fn k_closest_two() {
        let points = [[3, 3], [5, -1], [-2, 4]];
        let mut result = k_closest_points(&points, 2);
        result.sort();
        let mut expected = vec![[3, 3], [-2, 4]]; // dist^2 = 18, 20 (vs 26)
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn k_closest_all() {
        let points = [[1, 0], [0, 1]];
        let mut result = k_closest_points(&points, 2);
        result.sort();
        assert_eq!(result, vec![[0, 1], [1, 0]]);
    }

    #[test]
    fn k_closest_origin_point() {
        let points = [[0, 0], [1, 1], [2, 2]];
        let result = k_closest_points(&points, 1);
        assert_eq!(result[0], [0, 0]);
    }

    #[test]
    #[should_panic]
    fn k_closest_zero_k_panics() {
        k_closest_points(&[[1, 2]], 0);
    }

    // ---- merge_k_sorted ----

    #[test]
    fn merge_k_sorted_basic() {
        let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        assert_eq!(merge_k_sorted(lists), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn merge_k_sorted_empty_lists() {
        let lists: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
        assert_eq!(merge_k_sorted(lists), vec![]);
    }

    #[test]
    fn merge_k_sorted_no_lists() {
        let lists: Vec<Vec<i32>> = vec![];
        assert_eq!(merge_k_sorted(lists), vec![]);
    }

    #[test]
    fn merge_k_sorted_single_list() {
        let lists = vec![vec![1, 2, 3]];
        assert_eq!(merge_k_sorted(lists), vec![1, 2, 3]);
    }

    #[test]
    fn merge_k_sorted_mixed_empty() {
        let lists = vec![vec![], vec![1], vec![], vec![2, 3]];
        assert_eq!(merge_k_sorted(lists), vec![1, 2, 3]);
    }

    #[test]
    fn merge_k_sorted_negatives() {
        let lists = vec![vec![-5, -3, 0], vec![-4, -2, 1]];
        assert_eq!(merge_k_sorted(lists), vec![-5, -4, -3, -2, 0, 1]);
    }

    #[test]
    fn merge_k_sorted_single_elements() {
        let lists = vec![vec![5], vec![3], vec![1], vec![4], vec![2]];
        assert_eq!(merge_k_sorted(lists), vec![1, 2, 3, 4, 5]);
    }
}
