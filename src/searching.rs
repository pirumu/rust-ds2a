//! # Searching Algorithms
//!
//! Binary search variants and two-pointer techniques.

/// Classic binary search. Returns `Some(index)` if `target` is found, `None` otherwise.
///
/// **Requires:** `arr` is sorted in ascending order.
/// **Time:** O(log n).  **Space:** O(1).
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

/// Returns the index of the first element >= `target` (lower bound).
///
/// If all elements are less than `target`, returns `arr.len()`.
/// **Time:** O(log n).  **Space:** O(1).
pub fn lower_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] < *target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Returns the index of the first element > `target` (upper bound).
///
/// If no element is greater, returns `arr.len()`.
/// **Time:** O(log n).  **Space:** O(1).
pub fn upper_bound<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if arr[mid] <= *target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Two-pointer approach on a sorted array: find two indices whose values sum to `target`.
///
/// Returns `Some((i, j))` with `i < j`, or `None` if no pair exists.
/// **Time:** O(n).  **Space:** O(1).
pub fn two_sum_sorted(arr: &[i32], target: i32) -> Option<(usize, usize)> {
    if arr.len() < 2 {
        return None;
    }
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;
    while left < right {
        let sum = arr[left] + arr[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- binary_search ----
    #[test]
    fn binary_search_found() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &5), Some(2));
    }

    #[test]
    fn binary_search_not_found() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &4), None);
    }

    #[test]
    fn binary_search_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &1), None);
    }

    // ---- lower_bound ----
    #[test]
    fn lower_bound_exact() {
        let arr = [1, 2, 4, 4, 6];
        assert_eq!(lower_bound(&arr, &4), 2);
    }

    #[test]
    fn lower_bound_between() {
        let arr = [1, 3, 5, 7];
        assert_eq!(lower_bound(&arr, &4), 2);
    }

    // ---- upper_bound ----
    #[test]
    fn upper_bound_exact() {
        let arr = [1, 2, 4, 4, 6];
        assert_eq!(upper_bound(&arr, &4), 4);
    }

    #[test]
    fn upper_bound_all_less() {
        let arr = [1, 2, 3];
        assert_eq!(upper_bound(&arr, &5), 3);
    }

    // ---- two_sum_sorted ----
    #[test]
    fn two_sum_found() {
        let arr = [1, 2, 3, 4, 6];
        assert_eq!(two_sum_sorted(&arr, 6), Some((1, 3)));
    }

    #[test]
    fn two_sum_not_found() {
        let arr = [1, 2, 3];
        assert_eq!(two_sum_sorted(&arr, 100), None);
    }
}
