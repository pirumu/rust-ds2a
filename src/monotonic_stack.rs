//! # Monotonic Stack
//!
//! Stack-based patterns where the stack maintains a monotonically
//! increasing or decreasing order.  Useful for "next greater / smaller
//! element" family of problems.

/// For each element in `nums`, finds the **next greater element** to
/// its right.  Returns `None` for positions that have no greater
/// element after them.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn next_greater_element(nums: &[i32]) -> Vec<Option<i32>> {
    let n = nums.len();
    let mut result = vec![None; n];
    let mut stack: Vec<usize> = Vec::new(); // indices

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if nums[top] < nums[i] {
                result[top] = Some(nums[i]);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}

/// For each element in `nums`, finds the **next smaller element** to
/// its right.  Returns `None` for positions that have no smaller
/// element after them.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn next_smaller_element(nums: &[i32]) -> Vec<Option<i32>> {
    let n = nums.len();
    let mut result = vec![None; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if nums[top] > nums[i] {
                result[top] = Some(nums[i]);
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}

/// Given a list of daily temperatures, returns a vector where each
/// entry tells how many days you have to wait until a warmer
/// temperature.  `0` means no warmer day exists after that position.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn daily_temperatures(temps: &[i32]) -> Vec<i32> {
    let n = temps.len();
    let mut result = vec![0i32; n];
    let mut stack: Vec<usize> = Vec::new();

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if temps[top] < temps[i] {
                result[top] = (i - top) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }

    result
}

/// Computes the area of the **largest rectangle** that fits entirely
/// under the histogram defined by `heights`.
///
/// Uses a monotonic-increasing stack of indices.  When a bar shorter
/// than the stack top is found, bars are popped and rectangles are
/// evaluated.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn largest_rectangle_histogram(heights: &[i32]) -> i64 {
    let n = heights.len();
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area: i64 = 0;

    for i in 0..=n {
        let cur_h = if i < n { heights[i] } else { 0 };

        while let Some(&top) = stack.last() {
            if heights[top] >= cur_h {
                stack.pop();
                let h = heights[top] as i64;
                let w = match stack.last() {
                    Some(&left) => (i - left - 1) as i64,
                    None => i as i64,
                };
                max_area = max_area.max(h * w);
            } else {
                break;
            }
        }
        stack.push(i);
    }

    max_area
}

/// For each day, computes the **stock span** — the number of
/// consecutive days (including today) where the price was less than or
/// equal to today's price.
///
/// **Time:** O(n).  **Space:** O(n).
pub fn stock_span(prices: &[i32]) -> Vec<i32> {
    let n = prices.len();
    let mut result = vec![0i32; n];
    let mut stack: Vec<usize> = Vec::new(); // indices of "previous greater" candidates

    for i in 0..n {
        while let Some(&top) = stack.last() {
            if prices[top] <= prices[i] {
                stack.pop();
            } else {
                break;
            }
        }
        result[i] = match stack.last() {
            Some(&prev) => (i - prev) as i32,
            None => (i + 1) as i32,
        };
        stack.push(i);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- next_greater_element ----
    #[test]
    fn nge_basic() {
        assert_eq!(
            next_greater_element(&[2, 1, 2, 4, 3]),
            vec![Some(4), Some(2), Some(4), None, None]
        );
    }

    #[test]
    fn nge_descending() {
        assert_eq!(
            next_greater_element(&[5, 4, 3, 2, 1]),
            vec![None, None, None, None, None]
        );
    }

    #[test]
    fn nge_ascending() {
        assert_eq!(
            next_greater_element(&[1, 2, 3, 4, 5]),
            vec![Some(2), Some(3), Some(4), Some(5), None]
        );
    }

    #[test]
    fn nge_single() {
        assert_eq!(next_greater_element(&[42]), vec![None]);
    }

    #[test]
    fn nge_empty() {
        assert_eq!(next_greater_element(&[]), Vec::<Option<i32>>::new());
    }

    #[test]
    fn nge_duplicates() {
        assert_eq!(
            next_greater_element(&[3, 3, 3]),
            vec![None, None, None]
        );
    }

    // ---- next_smaller_element ----
    #[test]
    fn nse_basic() {
        assert_eq!(
            next_smaller_element(&[4, 8, 5, 2, 25]),
            vec![Some(2), Some(5), Some(2), None, None]
        );
    }

    #[test]
    fn nse_ascending() {
        assert_eq!(
            next_smaller_element(&[1, 2, 3, 4, 5]),
            vec![None, None, None, None, None]
        );
    }

    #[test]
    fn nse_descending() {
        assert_eq!(
            next_smaller_element(&[5, 4, 3, 2, 1]),
            vec![Some(4), Some(3), Some(2), Some(1), None]
        );
    }

    #[test]
    fn nse_empty() {
        assert_eq!(next_smaller_element(&[]), Vec::<Option<i32>>::new());
    }

    // ---- daily_temperatures ----
    #[test]
    fn dt_basic() {
        assert_eq!(
            daily_temperatures(&[73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn dt_all_same() {
        assert_eq!(daily_temperatures(&[30, 30, 30]), vec![0, 0, 0]);
    }

    #[test]
    fn dt_decreasing() {
        assert_eq!(daily_temperatures(&[80, 70, 60]), vec![0, 0, 0]);
    }

    #[test]
    fn dt_increasing() {
        assert_eq!(daily_temperatures(&[60, 70, 80]), vec![1, 1, 0]);
    }

    #[test]
    fn dt_single() {
        assert_eq!(daily_temperatures(&[50]), vec![0]);
    }

    #[test]
    fn dt_empty() {
        assert_eq!(daily_temperatures(&[]), Vec::<i32>::new());
    }

    // ---- largest_rectangle_histogram ----
    #[test]
    fn lrh_classic() {
        assert_eq!(largest_rectangle_histogram(&[2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn lrh_single() {
        assert_eq!(largest_rectangle_histogram(&[5]), 5);
    }

    #[test]
    fn lrh_equal() {
        assert_eq!(largest_rectangle_histogram(&[3, 3, 3, 3]), 12);
    }

    #[test]
    fn lrh_ascending() {
        // 1,2,3,4 -> best is 3*2=6 or 2*3=6 or 1*4=4 or 4*1=4 => max rectangle is at heights 2,3,4 -> nope
        // Actually: heights 3,4 give min 3 * width 2 = 6; heights 2,3,4 give 2*3=6; 1,2,3,4 give 1*4=4
        // So max = 6
        assert_eq!(largest_rectangle_histogram(&[1, 2, 3, 4]), 6);
    }

    #[test]
    fn lrh_descending() {
        assert_eq!(largest_rectangle_histogram(&[4, 3, 2, 1]), 6);
    }

    #[test]
    fn lrh_empty() {
        assert_eq!(largest_rectangle_histogram(&[]), 0);
    }

    #[test]
    fn lrh_valley() {
        assert_eq!(largest_rectangle_histogram(&[6, 2, 5, 4, 5, 1, 6]), 12);
    }

    // ---- stock_span ----
    #[test]
    fn span_basic() {
        assert_eq!(
            stock_span(&[100, 80, 60, 70, 60, 75, 85]),
            vec![1, 1, 1, 2, 1, 4, 6]
        );
    }

    #[test]
    fn span_increasing() {
        assert_eq!(
            stock_span(&[10, 20, 30, 40]),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn span_decreasing() {
        assert_eq!(
            stock_span(&[40, 30, 20, 10]),
            vec![1, 1, 1, 1]
        );
    }

    #[test]
    fn span_single() {
        assert_eq!(stock_span(&[50]), vec![1]);
    }

    #[test]
    fn span_empty() {
        assert_eq!(stock_span(&[]), Vec::<i32>::new());
    }

    #[test]
    fn span_all_equal() {
        assert_eq!(stock_span(&[5, 5, 5, 5]), vec![1, 2, 3, 4]);
    }
}
