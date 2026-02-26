//! # Prefix Sum
//!
//! Prefix sum (cumulative sum) pre-processes an array so that any range sum
//! query can be answered in O(1). Extends naturally to 2D matrices.

use std::collections::HashMap;

/// Build a 1D prefix sum array where `result[i] = arr[0] + arr[1] + ... + arr[i-1]`.
///
/// `result` has length `arr.len() + 1` with `result[0] = 0` so that
/// `range_sum(prefix, l, r) = prefix[r+1] - prefix[l]`.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn prefix_sum(arr: &[i64]) -> Vec<i64> {
    let mut prefix = Vec::with_capacity(arr.len() + 1);
    prefix.push(0);
    for &val in arr {
        let last = *prefix.last().unwrap();
        prefix.push(last + val);
    }
    prefix
}

/// O(1) range sum query on a pre-computed prefix sum array.
///
/// Returns the sum `arr[left] + arr[left+1] + ... + arr[right]` (inclusive).
///
/// # Panics
///
/// Panics if `right + 1 >= prefix.len()` or `left > right`.
///
/// **Time:** O(1).  **Space:** O(1).
pub fn range_sum(prefix: &[i64], left: usize, right: usize) -> i64 {
    prefix[right + 1] - prefix[left]
}

/// Count the number of contiguous subarrays whose elements sum to `target`.
///
/// Uses prefix sums + a hash map to achieve O(n) time.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn subarray_sum_count(arr: &[i64], target: i64) -> usize {
    let mut count: usize = 0;
    let mut current_sum: i64 = 0;
    let mut map: HashMap<i64, usize> = HashMap::new();
    map.insert(0, 1); // empty prefix

    for &val in arr {
        current_sum += val;
        if let Some(&c) = map.get(&(current_sum - target)) {
            count += c;
        }
        *map.entry(current_sum).or_insert(0) += 1;
    }
    count
}

/// Build a 2D prefix sum matrix.
///
/// `result[r][c]` = sum of all elements in `matrix[0..r][0..c]`.
/// The result has dimensions `(rows+1) x (cols+1)` with a zero-padded border.
///
/// **Time:** O(rows * cols).  **Space:** O(rows * cols).
pub fn prefix_sum_2d(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    if matrix.is_empty() {
        return vec![vec![0]];
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut prefix = vec![vec![0i64; cols + 1]; rows + 1];

    for r in 1..=rows {
        for c in 1..=cols {
            prefix[r][c] = matrix[r - 1][c - 1]
                + prefix[r - 1][c]
                + prefix[r][c - 1]
                - prefix[r - 1][c - 1];
        }
    }
    prefix
}

/// O(1) range sum query on a 2D prefix sum matrix.
///
/// Returns the sum of elements in the sub-rectangle from `(r1, c1)` to `(r2, c2)` inclusive.
///
/// # Panics
///
/// Panics if indices are out of bounds.
///
/// **Time:** O(1).  **Space:** O(1).
pub fn range_sum_2d(
    prefix: &[Vec<i64>],
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
) -> i64 {
    prefix[r2 + 1][c2 + 1]
        - prefix[r1][c2 + 1]
        - prefix[r2 + 1][c1]
        + prefix[r1][c1]
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- prefix_sum ----
    #[test]
    fn prefix_sum_basic() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(prefix_sum(&arr), vec![0, 1, 3, 6, 10, 15]);
    }

    #[test]
    fn prefix_sum_empty() {
        let arr: [i64; 0] = [];
        assert_eq!(prefix_sum(&arr), vec![0]);
    }

    #[test]
    fn prefix_sum_single() {
        assert_eq!(prefix_sum(&[42]), vec![0, 42]);
    }

    #[test]
    fn prefix_sum_negative() {
        let arr = [-3, 5, -2, 7];
        assert_eq!(prefix_sum(&arr), vec![0, -3, 2, 0, 7]);
    }

    // ---- range_sum ----
    #[test]
    fn range_sum_full_array() {
        let prefix = prefix_sum(&[1, 2, 3, 4, 5]);
        assert_eq!(range_sum(&prefix, 0, 4), 15);
    }

    #[test]
    fn range_sum_single_element() {
        let prefix = prefix_sum(&[10, 20, 30]);
        assert_eq!(range_sum(&prefix, 1, 1), 20);
    }

    #[test]
    fn range_sum_middle_range() {
        let prefix = prefix_sum(&[1, 2, 3, 4, 5]);
        assert_eq!(range_sum(&prefix, 1, 3), 9); // 2+3+4
    }

    #[test]
    fn range_sum_with_negatives() {
        let prefix = prefix_sum(&[-1, 4, -3, 2]);
        assert_eq!(range_sum(&prefix, 0, 3), 2);
        assert_eq!(range_sum(&prefix, 1, 2), 1); // 4 + (-3)
    }

    // ---- subarray_sum_count ----
    #[test]
    fn subarray_sum_count_basic() {
        let arr = [1, 1, 1];
        assert_eq!(subarray_sum_count(&arr, 2), 2); // [1,1] at pos 0..1 and 1..2
    }

    #[test]
    fn subarray_sum_count_empty() {
        let arr: [i64; 0] = [];
        assert_eq!(subarray_sum_count(&arr, 0), 0);
    }

    #[test]
    fn subarray_sum_count_with_negatives() {
        let arr = [1, -1, 1, -1, 1];
        // subarrays summing to 0: [1,-1] x3, [1,-1,1,-1] x1, [-1,1] x2, [-1,1,-1,1] x1
        assert_eq!(subarray_sum_count(&arr, 0), 6);
    }

    #[test]
    fn subarray_sum_count_single_match() {
        let arr = [5];
        assert_eq!(subarray_sum_count(&arr, 5), 1);
    }

    #[test]
    fn subarray_sum_count_no_match() {
        let arr = [1, 2, 3];
        assert_eq!(subarray_sum_count(&arr, 100), 0);
    }

    #[test]
    fn subarray_sum_count_zero_target() {
        let arr = [0, 0, 0];
        // [0] x3, [0,0] x2, [0,0,0] x1 = 6
        assert_eq!(subarray_sum_count(&arr, 0), 6);
    }

    // ---- prefix_sum_2d ----
    #[test]
    fn prefix_sum_2d_basic() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let prefix = prefix_sum_2d(&matrix);
        // prefix[3][3] = sum of entire matrix = 45
        assert_eq!(prefix[3][3], 45);
        // prefix[1][1] = matrix[0][0] = 1
        assert_eq!(prefix[1][1], 1);
        // prefix[2][2] = 1+2+4+5 = 12
        assert_eq!(prefix[2][2], 12);
    }

    #[test]
    fn prefix_sum_2d_empty() {
        let matrix: Vec<Vec<i64>> = vec![];
        let prefix = prefix_sum_2d(&matrix);
        assert_eq!(prefix, vec![vec![0]]);
    }

    #[test]
    fn prefix_sum_2d_single_cell() {
        let matrix = vec![vec![42]];
        let prefix = prefix_sum_2d(&matrix);
        assert_eq!(prefix[1][1], 42);
    }

    #[test]
    fn prefix_sum_2d_single_row() {
        let matrix = vec![vec![1, 2, 3]];
        let prefix = prefix_sum_2d(&matrix);
        assert_eq!(prefix[1][3], 6);
    }

    #[test]
    fn prefix_sum_2d_negative() {
        let matrix = vec![
            vec![-1, 2],
            vec![3, -4],
        ];
        let prefix = prefix_sum_2d(&matrix);
        assert_eq!(prefix[2][2], 0); // -1+2+3+(-4) = 0
    }

    // ---- range_sum_2d ----
    #[test]
    fn range_sum_2d_full() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let prefix = prefix_sum_2d(&matrix);
        assert_eq!(range_sum_2d(&prefix, 0, 0, 2, 2), 45);
    }

    #[test]
    fn range_sum_2d_sub_rectangle() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let prefix = prefix_sum_2d(&matrix);
        // sub-rectangle (1,1) to (2,2) = 5+6+8+9 = 28
        assert_eq!(range_sum_2d(&prefix, 1, 1, 2, 2), 28);
    }

    #[test]
    fn range_sum_2d_single_cell() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        let prefix = prefix_sum_2d(&matrix);
        assert_eq!(range_sum_2d(&prefix, 0, 0, 0, 0), 1);
        assert_eq!(range_sum_2d(&prefix, 1, 1, 1, 1), 4);
    }

    #[test]
    fn range_sum_2d_single_row() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ];
        let prefix = prefix_sum_2d(&matrix);
        // row 1, cols 0..2 = 4+5+6 = 15
        assert_eq!(range_sum_2d(&prefix, 1, 0, 1, 2), 15);
    }

    #[test]
    fn range_sum_2d_single_column() {
        let matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        let prefix = prefix_sum_2d(&matrix);
        // col 0, rows 0..2 = 1+3+5 = 9
        assert_eq!(range_sum_2d(&prefix, 0, 0, 2, 0), 9);
    }
}
