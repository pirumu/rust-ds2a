//! # Bit Manipulation
//!
//! Bitwise operations, classic bit tricks, and bitmask subset enumeration.

/// Check whether `n` is a power of two (1, 2, 4, 8, ...).
///
/// Zero is **not** a power of two.
/// Uses the trick: a power of two has exactly one set bit, so `n & (n - 1) == 0`.
///
/// **Time:** O(1).  **Space:** O(1).
pub fn is_power_of_two(n: u64) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

/// Count the number of set bits (1-bits) in `n` using Brian Kernighan's algorithm.
///
/// Each iteration clears the lowest set bit, so the loop runs exactly
/// as many times as there are set bits.
///
/// **Time:** O(k) where k = number of set bits.  **Space:** O(1).
pub fn count_ones(n: u64) -> u32 {
    let mut count = 0u32;
    let mut x = n;
    while x != 0 {
        x &= x - 1; // clear lowest set bit
        count += 1;
    }
    count
}

/// Find the element that appears exactly once while every other element
/// appears exactly twice.
///
/// XOR of all elements cancels out the pairs, leaving the single number.
///
/// **Time:** O(n).  **Space:** O(1).
pub fn single_number(nums: &[i32]) -> i32 {
    nums.iter().fold(0, |acc, &x| acc ^ x)
}

/// Enumerate all subsets of `nums` using bitmask iteration.
///
/// For `n` elements there are 2^n subsets. Each integer in `0..2^n`
/// represents a subset: bit `i` indicates whether `nums[i]` is included.
///
/// **Time:** O(n * 2^n).  **Space:** O(n * 2^n) for the output.
pub fn subsets_bitmask(nums: &[i32]) -> Vec<Vec<i32>> {
    let n = nums.len();
    let total = 1u64 << n;
    let mut result = Vec::with_capacity(total as usize);
    for mask in 0..total {
        let mut subset = Vec::new();
        for (i, &val) in nums.iter().enumerate() {
            if mask & (1u64 << i) != 0 {
                subset.push(val);
            }
        }
        result.push(subset);
    }
    result
}

/// Get the bit at position `pos` (0-indexed from the right).
///
/// **Time:** O(1).  **Space:** O(1).
pub fn get_bit(n: u64, pos: u32) -> bool {
    (n >> pos) & 1 == 1
}

/// Set the bit at position `pos` to 1.
///
/// **Time:** O(1).  **Space:** O(1).
pub fn set_bit(n: u64, pos: u32) -> u64 {
    n | (1u64 << pos)
}

/// Clear the bit at position `pos` (set to 0).
///
/// **Time:** O(1).  **Space:** O(1).
pub fn clear_bit(n: u64, pos: u32) -> u64 {
    n & !(1u64 << pos)
}

/// Toggle (flip) the bit at position `pos`.
///
/// **Time:** O(1).  **Space:** O(1).
pub fn toggle_bit(n: u64, pos: u32) -> u64 {
    n ^ (1u64 << pos)
}

/// Swap two values without a temporary variable using XOR.
///
/// **Note:** This works correctly even when `a` and `b` point to the same
/// memory location (both become 0, which is safe if unexpected).
///
/// **Time:** O(1).  **Space:** O(1).
pub fn swap_without_temp(a: &mut i32, b: &mut i32) {
    if std::ptr::eq(a, b) {
        return;
    }
    std::mem::swap(&mut (*a), &mut (*b));
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- is_power_of_two ----

    #[test]
    fn power_of_two_basic() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(1024));
        assert!(is_power_of_two(1 << 40));
    }

    #[test]
    fn not_power_of_two() {
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(5));
        assert!(!is_power_of_two(6));
        assert!(!is_power_of_two(100));
    }

    // ---- count_ones ----

    #[test]
    fn count_ones_basic() {
        assert_eq!(count_ones(0), 0);
        assert_eq!(count_ones(1), 1);
        assert_eq!(count_ones(0b1010_1010), 4);
        assert_eq!(count_ones(0b1111_1111), 8);
        assert_eq!(count_ones(u64::MAX), 64);
    }

    #[test]
    fn count_ones_powers() {
        for i in 0..64 {
            assert_eq!(count_ones(1u64 << i), 1);
        }
    }

    // ---- single_number ----

    #[test]
    fn single_number_basic() {
        assert_eq!(single_number(&[2, 2, 1]), 1);
        assert_eq!(single_number(&[4, 1, 2, 1, 2]), 4);
        assert_eq!(single_number(&[1]), 1);
    }

    #[test]
    fn single_number_negative() {
        assert_eq!(single_number(&[-1, -1, -2]), -2);
        assert_eq!(single_number(&[0, 0, 42]), 42);
    }

    // ---- subsets_bitmask ----

    #[test]
    fn subsets_empty() {
        let result = subsets_bitmask(&[]);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn subsets_single() {
        let result = subsets_bitmask(&[5]);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![5]));
    }

    #[test]
    fn subsets_three() {
        let result = subsets_bitmask(&[1, 2, 3]);
        assert_eq!(result.len(), 8);
        assert!(result.contains(&vec![]));
        assert!(result.contains(&vec![1]));
        assert!(result.contains(&vec![2]));
        assert!(result.contains(&vec![3]));
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![1, 3]));
        assert!(result.contains(&vec![2, 3]));
        assert!(result.contains(&vec![1, 2, 3]));
    }

    // ---- get_bit / set_bit / clear_bit / toggle_bit ----

    #[test]
    fn get_bit_basic() {
        let n = 0b1010u64;
        assert!(!get_bit(n, 0));
        assert!(get_bit(n, 1));
        assert!(!get_bit(n, 2));
        assert!(get_bit(n, 3));
    }

    #[test]
    fn set_bit_basic() {
        assert_eq!(set_bit(0b1000, 0), 0b1001);
        assert_eq!(set_bit(0b1001, 0), 0b1001); // already set
        assert_eq!(set_bit(0, 5), 32);
    }

    #[test]
    fn clear_bit_basic() {
        assert_eq!(clear_bit(0b1010, 1), 0b1000);
        assert_eq!(clear_bit(0b1010, 0), 0b1010); // already clear
        assert_eq!(clear_bit(0b1111, 2), 0b1011);
    }

    #[test]
    fn toggle_bit_basic() {
        assert_eq!(toggle_bit(0b1010, 0), 0b1011);
        assert_eq!(toggle_bit(0b1010, 1), 0b1000);
        // toggle twice returns original
        assert_eq!(toggle_bit(toggle_bit(42, 3), 3), 42);
    }

    // ---- swap_without_temp ----

    #[test]
    fn swap_basic() {
        let mut a = 10;
        let mut b = 20;
        swap_without_temp(&mut a, &mut b);
        assert_eq!(a, 20);
        assert_eq!(b, 10);
    }

    #[test]
    fn swap_negative() {
        let mut a = -5;
        let mut b = 7;
        swap_without_temp(&mut a, &mut b);
        assert_eq!(a, 7);
        assert_eq!(b, -5);
    }

    #[test]
    fn swap_zeros() {
        let mut a = 0;
        let mut b = 99;
        swap_without_temp(&mut a, &mut b);
        assert_eq!(a, 99);
        assert_eq!(b, 0);
    }

    #[test]
    fn swap_equal_values() {
        let mut a = 42;
        let mut b = 42;
        swap_without_temp(&mut a, &mut b);
        assert_eq!(a, 42);
        assert_eq!(b, 42);
    }
}
