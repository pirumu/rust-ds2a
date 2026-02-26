//! # Sorting Algorithms
//!
//! Classic comparison-based and non-comparison-based sorting algorithms.

/// Bubble sort — repeatedly swap adjacent out-of-order elements.
///
/// **Time:** O(n^2) average/worst, O(n) best (already sorted).
/// **Space:** O(1).
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n.saturating_sub(i + 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

/// Selection sort — find the minimum and place it at the front.
///
/// **Time:** O(n^2).  **Space:** O(1).
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

/// Insertion sort — insert each element into its correct position in the sorted prefix.
///
/// **Time:** O(n^2) average/worst, O(n) best.  **Space:** O(1).
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Merge sort — divide the array in half, sort each half, then merge.
///
/// **Time:** O(n log n).  **Space:** O(n).
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(&left, &right, arr);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], out: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out[k] = left[i].clone();
            i += 1;
        } else {
            out[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        out[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        out[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

/// Quick sort (Lomuto partition scheme).
///
/// **Time:** O(n log n) average, O(n^2) worst.  **Space:** O(log n) stack.
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    quick_sort_helper(arr, 0, n - 1);
}

fn quick_sort_helper<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let pivot = lomuto_partition(arr, lo, hi);
    if pivot > 0 {
        quick_sort_helper(arr, lo, pivot - 1);
    }
    quick_sort_helper(arr, pivot + 1, hi);
}

fn lomuto_partition<T: Ord>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    // Use last element as pivot.
    let mut i = lo;
    for j in lo..hi {
        if arr[j] <= arr[hi] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, hi);
    i
}

/// Heap sort — build a max-heap then repeatedly extract the maximum.
///
/// **Time:** O(n log n).  **Space:** O(1).
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    // Build max-heap.
    for i in (0..n / 2).rev() {
        sift_down(arr, i, n);
    }
    // Extract elements one by one.
    for end in (1..n).rev() {
        arr.swap(0, end);
        sift_down(arr, 0, end);
    }
}

fn sift_down<T: Ord>(arr: &mut [T], mut idx: usize, len: usize) {
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut largest = idx;
        if left < len && arr[left] > arr[largest] {
            largest = left;
        }
        if right < len && arr[right] > arr[largest] {
            largest = right;
        }
        if largest == idx {
            break;
        }
        arr.swap(idx, largest);
        idx = largest;
    }
}

/// LSD Radix sort for unsigned 32-bit integers.
///
/// **Time:** O(d * (n + b)) where d = number of digits, b = base (256).
/// **Space:** O(n + b).
pub fn radix_sort(arr: &mut [u32]) {
    if arr.len() <= 1 {
        return;
    }
    let max_val = match arr.iter().max() {
        Some(&v) => v,
        None => return,
    };

    let mut exp: u32 = 1;
    let mut output = vec![0u32; arr.len()];

    while max_val / exp > 0 {
        let mut count = [0usize; 256];

        // Count occurrences of each digit.
        for &val in arr.iter() {
            let digit = ((val / exp) % 256) as usize;
            count[digit] += 1;
        }

        // Prefix sum.
        for i in 1..256 {
            count[i] += count[i - 1];
        }

        // Build output (iterate in reverse for stability).
        for &val in arr.iter().rev() {
            let digit = ((val / exp) % 256) as usize;
            count[digit] -= 1;
            output[count[digit]] = val;
        }

        arr.copy_from_slice(&output);

        // Prevent overflow on the multiplication.
        if exp > u32::MAX / 256 {
            break;
        }
        exp *= 256;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- helpers ----
    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    // ---- bubble sort ----
    #[test]
    fn bubble_sort_empty() {
        let mut v: Vec<i32> = vec![];
        bubble_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn bubble_sort_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn bubble_sort_reverse() {
        let mut v = vec![5, 4, 3, 2, 1];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    // ---- selection sort ----
    #[test]
    fn selection_sort_empty() {
        let mut v: Vec<i32> = vec![];
        selection_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn selection_sort_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn selection_sort_reverse() {
        let mut v = vec![5, 4, 3, 2, 1];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    // ---- insertion sort ----
    #[test]
    fn insertion_sort_empty() {
        let mut v: Vec<i32> = vec![];
        insertion_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn insertion_sort_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_sort_reverse() {
        let mut v = vec![5, 4, 3, 2, 1];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    // ---- merge sort ----
    #[test]
    fn merge_sort_empty() {
        let mut v: Vec<i32> = vec![];
        merge_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn merge_sort_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn merge_sort_random() {
        let mut v = vec![38, 27, 43, 3, 9, 82, 10];
        merge_sort(&mut v);
        assert!(is_sorted(&v));
    }

    // ---- quick sort ----
    #[test]
    fn quick_sort_empty() {
        let mut v: Vec<i32> = vec![];
        quick_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn quick_sort_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn quick_sort_reverse() {
        let mut v = vec![5, 4, 3, 2, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    // ---- heap sort ----
    #[test]
    fn heap_sort_empty() {
        let mut v: Vec<i32> = vec![];
        heap_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn heap_sort_already_sorted() {
        let mut v = vec![1, 2, 3, 4, 5];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn heap_sort_reverse() {
        let mut v = vec![5, 4, 3, 2, 1];
        heap_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    // ---- radix sort ----
    #[test]
    fn radix_sort_empty() {
        let mut v: Vec<u32> = vec![];
        radix_sort(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn radix_sort_already_sorted() {
        let mut v: Vec<u32> = vec![1, 2, 3, 4, 5];
        radix_sort(&mut v);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn radix_sort_random() {
        let mut v: Vec<u32> = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut v);
        assert!(is_sorted(&v));
    }
}
