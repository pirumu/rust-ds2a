//! # Interval Problems
//!
//! Merge, insert, overlap detection, meeting rooms, and intersection of intervals.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

/// Merge overlapping intervals into a minimal set of non-overlapping intervals.
///
/// Sorts the input in-place by start time, then merges any two intervals
/// whose ranges overlap or touch.
///
/// **Time:** O(n log n).  **Space:** O(n).
pub fn merge_intervals(intervals: &mut [[i32; 2]]) -> Vec<[i32; 2]> {
    if intervals.is_empty() {
        return vec![];
    }
    intervals.sort_by_key(|iv| iv[0]);

    let mut merged: Vec<[i32; 2]> = vec![intervals[0]];

    for iv in intervals.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if iv[0] <= last[1] {
            last[1] = last[1].max(iv[1]);
        } else {
            merged.push(*iv);
        }
    }
    merged
}

/// Insert a new interval into a sorted, non-overlapping list and merge if needed.
///
/// Walks through the existing intervals, collecting those that come before,
/// merging those that overlap with `new`, and appending those that come after.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn insert_interval(intervals: &[[i32; 2]], new: [i32; 2]) -> Vec<[i32; 2]> {
    let mut result: Vec<[i32; 2]> = Vec::with_capacity(intervals.len() + 1);
    let mut new = new;
    let mut i = 0;
    let n = intervals.len();

    // Add all intervals that end before new starts.
    while i < n && intervals[i][1] < new[0] {
        result.push(intervals[i]);
        i += 1;
    }

    // Merge all overlapping intervals with new.
    while i < n && intervals[i][0] <= new[1] {
        new[0] = new[0].min(intervals[i][0]);
        new[1] = new[1].max(intervals[i][1]);
        i += 1;
    }
    result.push(new);

    // Add remaining intervals.
    while i < n {
        result.push(intervals[i]);
        i += 1;
    }
    result
}

/// Check whether any two intervals overlap.
///
/// Sorts by start time, then checks each consecutive pair.
///
/// **Time:** O(n log n).  **Space:** O(1) (sorts in place).
pub fn has_overlap(intervals: &mut [[i32; 2]]) -> bool {
    if intervals.len() < 2 {
        return false;
    }
    intervals.sort_by_key(|iv| iv[0]);

    for w in intervals.windows(2) {
        if w[0][1] > w[1][0] {
            return true;
        }
    }
    false
}

/// Minimum number of meeting rooms required to hold all meetings.
///
/// Uses a min-heap (sweep-line): for each meeting sorted by start time,
/// if the earliest ending meeting has already finished, reuse its room;
/// otherwise allocate a new room.
///
/// **Time:** O(n log n).  **Space:** O(n).
pub fn min_meeting_rooms(intervals: &[[i32; 2]]) -> usize {
    if intervals.is_empty() {
        return 0;
    }

    let mut sorted: Vec<[i32; 2]> = intervals.to_vec();
    sorted.sort_by_key(|iv| iv[0]);

    // Min-heap of end times (using Reverse for min behaviour).
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for iv in &sorted {
        // If the earliest-ending meeting is done before this one starts, reuse room.
        if let Some(&Reverse(earliest_end)) = heap.peek() {
            if earliest_end <= iv[0] {
                heap.pop();
            }
        }
        heap.push(Reverse(iv[1]));
    }

    heap.len()
}

/// Intersection of two lists of closed intervals.
///
/// Both input lists must be sorted and internally non-overlapping.
/// Uses a two-pointer approach.
///
/// **Time:** O(m + n).  **Space:** O(m + n) for the result.
pub fn interval_intersection(a: &[[i32; 2]], b: &[[i32; 2]]) -> Vec<[i32; 2]> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < a.len() && j < b.len() {
        let lo = a[i][0].max(b[j][0]);
        let hi = a[i][1].min(b[j][1]);

        if lo <= hi {
            result.push([lo, hi]);
        }

        // Advance the pointer whose interval ends first.
        if a[i][1] < b[j][1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- merge_intervals ----

    #[test]
    fn merge_overlapping() {
        let mut ivs = vec![[1, 3], [2, 6], [8, 10], [15, 18]];
        assert_eq!(merge_intervals(&mut ivs), vec![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn merge_all_overlapping() {
        let mut ivs = vec![[1, 4], [2, 5], [3, 6]];
        assert_eq!(merge_intervals(&mut ivs), vec![[1, 6]]);
    }

    #[test]
    fn merge_no_overlap() {
        let mut ivs = vec![[1, 2], [5, 6], [9, 10]];
        assert_eq!(merge_intervals(&mut ivs), vec![[1, 2], [5, 6], [9, 10]]);
    }

    #[test]
    fn merge_empty() {
        let mut ivs: Vec<[i32; 2]> = vec![];
        assert_eq!(merge_intervals(&mut ivs), Vec::<[i32; 2]>::new());
    }

    #[test]
    fn merge_single() {
        let mut ivs = vec![[1, 5]];
        assert_eq!(merge_intervals(&mut ivs), vec![[1, 5]]);
    }

    #[test]
    fn merge_touching() {
        let mut ivs = vec![[1, 3], [3, 5]];
        assert_eq!(merge_intervals(&mut ivs), vec![[1, 5]]);
    }

    #[test]
    fn merge_unsorted() {
        let mut ivs = vec![[8, 10], [1, 3], [2, 6], [15, 18]];
        assert_eq!(merge_intervals(&mut ivs), vec![[1, 6], [8, 10], [15, 18]]);
    }

    #[test]
    fn merge_nested() {
        let mut ivs = vec![[1, 10], [2, 5], [3, 7]];
        assert_eq!(merge_intervals(&mut ivs), vec![[1, 10]]);
    }

    // ---- insert_interval ----

    #[test]
    fn insert_middle() {
        let ivs = vec![[1, 3], [6, 9]];
        assert_eq!(insert_interval(&ivs, [2, 5]), vec![[1, 5], [6, 9]]);
    }

    #[test]
    fn insert_merge_multiple() {
        let ivs = vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]];
        assert_eq!(
            insert_interval(&ivs, [4, 8]),
            vec![[1, 2], [3, 10], [12, 16]]
        );
    }

    #[test]
    fn insert_no_overlap() {
        let ivs = vec![[1, 2], [5, 6]];
        assert_eq!(insert_interval(&ivs, [3, 4]), vec![[1, 2], [3, 4], [5, 6]]);
    }

    #[test]
    fn insert_into_empty() {
        let ivs: Vec<[i32; 2]> = vec![];
        assert_eq!(insert_interval(&ivs, [1, 5]), vec![[1, 5]]);
    }

    #[test]
    fn insert_at_beginning() {
        let ivs = vec![[3, 5], [7, 9]];
        assert_eq!(insert_interval(&ivs, [1, 2]), vec![[1, 2], [3, 5], [7, 9]]);
    }

    #[test]
    fn insert_at_end() {
        let ivs = vec![[1, 2], [3, 5]];
        assert_eq!(insert_interval(&ivs, [7, 9]), vec![[1, 2], [3, 5], [7, 9]]);
    }

    #[test]
    fn insert_merge_all() {
        let ivs = vec![[1, 3], [5, 7], [9, 11]];
        assert_eq!(insert_interval(&ivs, [2, 10]), vec![[1, 11]]);
    }

    // ---- has_overlap ----

    #[test]
    fn overlap_true() {
        let mut ivs = vec![[1, 5], [3, 7]];
        assert!(has_overlap(&mut ivs));
    }

    #[test]
    fn overlap_false() {
        let mut ivs = vec![[1, 2], [3, 4], [5, 6]];
        assert!(!has_overlap(&mut ivs));
    }

    #[test]
    fn overlap_touching_is_not_overlap() {
        let mut ivs = vec![[1, 3], [3, 5]];
        assert!(!has_overlap(&mut ivs));
    }

    #[test]
    fn overlap_empty() {
        let mut ivs: Vec<[i32; 2]> = vec![];
        assert!(!has_overlap(&mut ivs));
    }

    #[test]
    fn overlap_single() {
        let mut ivs = vec![[1, 5]];
        assert!(!has_overlap(&mut ivs));
    }

    #[test]
    fn overlap_nested() {
        let mut ivs = vec![[1, 10], [2, 5]];
        assert!(has_overlap(&mut ivs));
    }

    // ---- min_meeting_rooms ----

    #[test]
    fn rooms_basic() {
        assert_eq!(min_meeting_rooms(&[[0, 30], [5, 10], [15, 20]]), 2);
    }

    #[test]
    fn rooms_no_overlap() {
        assert_eq!(min_meeting_rooms(&[[1, 5], [6, 10], [11, 15]]), 1);
    }

    #[test]
    fn rooms_all_overlap() {
        assert_eq!(min_meeting_rooms(&[[1, 10], [2, 10], [3, 10]]), 3);
    }

    #[test]
    fn rooms_empty() {
        assert_eq!(min_meeting_rooms(&[]), 0);
    }

    #[test]
    fn rooms_single() {
        assert_eq!(min_meeting_rooms(&[[1, 5]]), 1);
    }

    #[test]
    fn rooms_back_to_back() {
        assert_eq!(min_meeting_rooms(&[[1, 5], [5, 10], [10, 15]]), 1);
    }

    #[test]
    fn rooms_complex() {
        assert_eq!(
            min_meeting_rooms(&[[1, 4], [2, 5], [7, 9], [3, 6], [8, 10]]),
            3
        );
    }

    // ---- interval_intersection ----

    #[test]
    fn intersection_basic() {
        let a = vec![[0, 2], [5, 10], [13, 23], [24, 25]];
        let b = vec![[1, 5], [8, 12], [15, 24], [25, 26]];
        assert_eq!(
            interval_intersection(&a, &b),
            vec![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
        );
    }

    #[test]
    fn intersection_no_overlap() {
        let a = vec![[1, 2], [5, 6]];
        let b = vec![[3, 4], [7, 8]];
        assert_eq!(interval_intersection(&a, &b), Vec::<[i32; 2]>::new());
    }

    #[test]
    fn intersection_one_empty() {
        let a: Vec<[i32; 2]> = vec![];
        let b = vec![[1, 5]];
        assert_eq!(interval_intersection(&a, &b), Vec::<[i32; 2]>::new());
    }

    #[test]
    fn intersection_identical() {
        let a = vec![[1, 5], [8, 10]];
        let b = vec![[1, 5], [8, 10]];
        assert_eq!(interval_intersection(&a, &b), vec![[1, 5], [8, 10]]);
    }

    #[test]
    fn intersection_contained() {
        let a = vec![[1, 10]];
        let b = vec![[3, 5], [7, 8]];
        assert_eq!(interval_intersection(&a, &b), vec![[3, 5], [7, 8]]);
    }
}
