//! # Dynamic Programming
//!
//! Classic DP problems solved with tabulation (bottom-up).

/// Compute the n-th Fibonacci number using tabulation.
///
/// F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2).
/// **Time:** O(n).  **Space:** O(1).
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut prev2: u64 = 0;
    let mut prev1: u64 = 1;
    for _ in 2..=n {
        let curr = prev1 + prev2;
        prev2 = prev1;
        prev1 = curr;
    }
    prev1
}

/// 0/1 Knapsack — maximize total value without exceeding capacity.
///
/// **Time:** O(n * capacity).  **Space:** O(n * capacity).
pub fn knapsack_01(weights: &[usize], values: &[usize], capacity: usize) -> usize {
    let n = weights.len();
    // dp[i][w] = max value using items 0..i with capacity w
    let mut dp = vec![vec![0usize; capacity + 1]; n + 1];

    for i in 1..=n {
        for w in 0..=capacity {
            dp[i][w] = dp[i - 1][w]; // skip item i-1
            if weights[i - 1] <= w {
                let take = dp[i - 1][w - weights[i - 1]] + values[i - 1];
                if take > dp[i][w] {
                    dp[i][w] = take;
                }
            }
        }
    }
    dp[n][capacity]
}

/// Longest Common Subsequence — returns one LCS string.
///
/// **Time:** O(m * n).  **Space:** O(m * n).
pub fn longest_common_subsequence(s1: &str, s2: &str) -> String {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let m = a.len();
    let n = b.len();

    // Build DP table.
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Back-track to recover the LCS.
    let mut result = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] >= dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    result.reverse();
    result.into_iter().collect()
}

/// Coin Change — minimum number of coins to make `amount`.
///
/// Returns `None` if the amount cannot be made.
/// **Time:** O(amount * |coins|).  **Space:** O(amount).
pub fn coin_change(coins: &[usize], amount: usize) -> Option<usize> {
    const INF: usize = usize::MAX;
    let mut dp = vec![INF; amount + 1];
    dp[0] = 0;

    for a in 1..=amount {
        for &c in coins {
            if c <= a && dp[a - c] != INF {
                let candidate = dp[a - c] + 1;
                if candidate < dp[a] {
                    dp[a] = candidate;
                }
            }
        }
    }

    if dp[amount] == INF {
        None
    } else {
        Some(dp[amount])
    }
}

/// Longest Increasing Subsequence — returns one LIS.
///
/// Uses patience-sorting style O(n log n) for length, then back-tracks for the actual sequence.
/// **Time:** O(n log n).  **Space:** O(n).
pub fn longest_increasing_subsequence(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let n = arr.len();
    // tails[i] = smallest tail value of all increasing subsequences of length i+1
    let mut tails: Vec<i32> = Vec::new();
    // parent[i] = index of previous element in the LIS ending at i
    let mut parent = vec![0usize; n];
    // indices[i] = index in arr of the element that gives tails[i]
    let mut indices: Vec<usize> = Vec::new();
    // pos[i] = position in tails when arr[i] was placed
    let mut pos = vec![0usize; n];

    for i in 0..n {
        // Binary search for the leftmost tail >= arr[i].
        let mut lo = 0usize;
        let mut hi = tails.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if tails[mid] < arr[i] {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        if lo == tails.len() {
            tails.push(arr[i]);
            indices.push(i);
        } else {
            tails[lo] = arr[i];
            indices[lo] = i;
        }
        pos[i] = lo;
        parent[i] = if lo > 0 { indices[lo - 1] } else { i };
    }

    // Reconstruct.
    let lis_len = tails.len();
    let mut result = vec![0i32; lis_len];
    let mut k = indices[lis_len - 1];
    for idx in (0..lis_len).rev() {
        result[idx] = arr[k];
        k = parent[k];
    }
    result
}

/// Edit distance (Levenshtein distance) between two strings.
///
/// **Time:** O(m * n).  **Space:** O(m * n).
pub fn edit_distance(s1: &str, s2: &str) -> usize {
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let m = a.len();
    let n = b.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1]
                    .min(dp[i - 1][j])
                    .min(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

// ---------------------------------------------------------------------------
// Matrix DP
// ---------------------------------------------------------------------------

/// Count unique paths from top-left to bottom-right in an m×n grid.
///
/// Only moves right or down are allowed.
/// **Time:** O(m*n).  **Space:** O(n).
pub fn unique_paths(m: usize, n: usize) -> u64 {
    if m == 0 || n == 0 {
        return 0;
    }
    let mut dp = vec![1u64; n];
    for _ in 1..m {
        for j in 1..n {
            dp[j] += dp[j - 1];
        }
    }
    dp[n - 1]
}

/// Minimum path sum in a grid, moving only right or down.
///
/// **Time:** O(m*n).  **Space:** O(n).
pub fn min_path_sum(grid: &[Vec<i32>]) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![0i32; n];
    dp[0] = grid[0][0];
    for j in 1..n {
        dp[j] = dp[j - 1] + grid[0][j];
    }
    for i in 1..m {
        dp[0] += grid[i][0];
        for j in 1..n {
            dp[j] = dp[j].min(dp[j - 1]) + grid[i][j];
        }
    }
    dp[n - 1]
}

// ---------------------------------------------------------------------------
// String DP
// ---------------------------------------------------------------------------

/// Length of the longest palindromic subsequence.
///
/// **Time:** O(n²).  **Space:** O(n²).
pub fn longest_palindromic_subsequence(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return 0;
    }
    // dp[i][j] = LPS length of chars[i..=j]
    let mut dp = vec![vec![0usize; n]; n];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if chars[i] == chars[j] {
                dp[i][j] = dp[i + 1][j - 1] + 2;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[0][n - 1]
}

/// Wildcard pattern matching with `?` (any single char) and `*` (any sequence including empty).
///
/// **Time:** O(m*n).  **Space:** O(m*n).
pub fn is_match_wildcard(s: &str, p: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    let m = s.len();
    let n = p.len();
    // dp[i][j] = s[..i] matches p[..j]
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;
    // p starts with '*' can match empty string
    for j in 1..=n {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 1];
        }
    }
    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                // '*' matches zero chars (dp[i][j-1]) or one more char (dp[i-1][j])
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            } else if p[j - 1] == '?' || p[j - 1] == s[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }
    dp[m][n]
}

/// Minimum cuts needed to partition `s` so every substring is a palindrome.
///
/// **Time:** O(n²).  **Space:** O(n²).
pub fn palindrome_min_cuts(s: &str) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n <= 1 {
        return 0;
    }
    // is_pal[i][j] = true if chars[i..=j] is a palindrome
    let mut is_pal = vec![vec![false; n]; n];
    for i in 0..n {
        is_pal[i][i] = true;
    }
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if chars[i] == chars[j] {
                is_pal[i][j] = len == 2 || is_pal[i + 1][j - 1];
            }
        }
    }
    // cuts[i] = min cuts for chars[0..=i]
    let mut cuts = vec![0usize; n];
    for i in 0..n {
        if is_pal[0][i] {
            cuts[i] = 0;
        } else {
            cuts[i] = i; // worst case: cut every char
            for j in 1..=i {
                if is_pal[j][i] {
                    cuts[i] = cuts[i].min(cuts[j - 1] + 1);
                }
            }
        }
    }
    cuts[n - 1]
}

// ---------------------------------------------------------------------------
// State Machine DP
// ---------------------------------------------------------------------------

/// Best profit from stock prices with a 1-day cooldown after selling.
///
/// States: hold, sold, rest.
/// **Time:** O(n).  **Space:** O(1).
pub fn max_profit_with_cooldown(prices: &[i32]) -> i32 {
    if prices.len() < 2 {
        return 0;
    }
    let mut hold = -prices[0]; // holding stock
    let mut sold = 0i32; // just sold
    let mut rest = 0i32; // cooldown / idle
    for &price in &prices[1..] {
        let prev_hold = hold;
        let prev_sold = sold;
        hold = hold.max(rest - price);
        sold = prev_hold + price;
        rest = rest.max(prev_sold);
    }
    sold.max(rest)
}

/// Maximum profit with at most `k` buy-sell transactions.
///
/// When k >= n/2, simplifies to unlimited transactions.
/// **Time:** O(n*k).  **Space:** O(k).
pub fn max_profit_k_transactions(k: usize, prices: &[i32]) -> i32 {
    let n = prices.len();
    if n < 2 || k == 0 {
        return 0;
    }
    // Unlimited transactions when k is large enough.
    if k >= n / 2 {
        let mut profit = 0i32;
        for i in 1..n {
            if prices[i] > prices[i - 1] {
                profit += prices[i] - prices[i - 1];
            }
        }
        return profit;
    }
    // buy[t]  = max profit after buying for the t-th transaction
    // sell[t] = max profit after selling for the t-th transaction
    let mut buy = vec![i32::MIN; k];
    let mut sell = vec![0i32; k];
    for &price in prices {
        for t in 0..k {
            let prev_sell = if t == 0 { 0 } else { sell[t - 1] };
            buy[t] = buy[t].max(prev_sell - price);
            sell[t] = sell[t].max(buy[t] + price);
        }
    }
    sell[k - 1]
}

// ---------------------------------------------------------------------------
// Bitmask DP / Subset DP
// ---------------------------------------------------------------------------

/// Can the array be partitioned into two subsets with equal sum?
///
/// Uses bitset DP on subset sums.
/// **Time:** O(n * sum).  **Space:** O(sum).
pub fn can_partition_equal_subset(nums: &[i32]) -> bool {
    let total: i32 = nums.iter().sum();
    if total % 2 != 0 || total < 0 {
        return false;
    }
    let target = total as usize / 2;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &num in nums {
        let num = num as usize;
        for s in (num..=target).rev() {
            dp[s] = dp[s] || dp[s - num];
        }
    }
    dp[target]
}

/// Shortest path that visits every node exactly once (Hamiltonian path).
///
/// Tries every starting node. Bitmask DP.
/// **Time:** O(2^n * n²).  **Space:** O(2^n * n).
pub fn shortest_hamiltonian_path(dist: &[Vec<i32>]) -> i32 {
    let n = dist.len();
    if n <= 1 {
        return 0;
    }
    let full = (1usize << n) - 1;
    let inf = i32::MAX / 2;
    // dp[mask][i] = shortest path visiting nodes in mask, ending at i
    let mut dp = vec![vec![inf; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for mask in 1..=full {
        for last in 0..n {
            if mask & (1 << last) == 0 || dp[mask][last] >= inf {
                continue;
            }
            for next in 0..n {
                if mask & (1 << next) != 0 {
                    continue;
                }
                let new_mask = mask | (1 << next);
                let cost = dp[mask][last] + dist[last][next];
                if cost < dp[new_mask][next] {
                    dp[new_mask][next] = cost;
                }
            }
        }
    }
    *dp[full].iter().min().unwrap()
}

// ---------------------------------------------------------------------------
// Interval DP
// ---------------------------------------------------------------------------

/// Maximize coins collected by bursting all balloons.
///
/// Balloons have implicit 1s on both boundaries.
/// **Time:** O(n³).  **Space:** O(n²).
pub fn burst_balloons(nums: &[i32]) -> i32 {
    let n = nums.len();
    // Extend with boundary 1s.
    let mut vals = Vec::with_capacity(n + 2);
    vals.push(1);
    vals.extend_from_slice(nums);
    vals.push(1);
    let m = vals.len(); // n + 2
    // dp[i][j] = max coins from bursting all balloons between i and j (exclusive)
    let mut dp = vec![vec![0i32; m]; m];
    for len in 2..m {
        for i in 0..m - len {
            let j = i + len;
            for k in i + 1..j {
                let coins = vals[i] * vals[k] * vals[j] + dp[i][k] + dp[k][j];
                if coins > dp[i][j] {
                    dp[i][j] = coins;
                }
            }
        }
    }
    dp[0][m - 1]
}

/// Minimum number of scalar multiplications to multiply a chain of matrices.
///
/// `dims` has length n+1 for n matrices; matrix i has dimensions dims[i] × dims[i+1].
/// **Time:** O(n³).  **Space:** O(n²).
pub fn matrix_chain_order(dims: &[usize]) -> usize {
    let n = dims.len();
    if n < 2 {
        return 0;
    }
    let num_matrices = n - 1;
    // dp[i][j] = min cost to multiply matrices i..=j (0-indexed)
    let mut dp = vec![vec![0usize; num_matrices]; num_matrices];
    for len in 2..=num_matrices {
        for i in 0..=num_matrices - len {
            let j = i + len - 1;
            dp[i][j] = usize::MAX;
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1];
                if cost < dp[i][j] {
                    dp[i][j] = cost;
                }
            }
        }
    }
    dp[0][num_matrices - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- fibonacci ----
    #[test]
    fn fib_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn fib_larger() {
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(20), 6765);
    }

    // ---- knapsack ----
    #[test]
    fn knapsack_basic() {
        let weights = [1, 3, 4, 5];
        let values = [1, 4, 5, 7];
        assert_eq!(knapsack_01(&weights, &values, 7), 9); // items 1+3 (4+5)
    }

    #[test]
    fn knapsack_zero_capacity() {
        assert_eq!(knapsack_01(&[1, 2], &[10, 20], 0), 0);
    }

    // ---- LCS ----
    #[test]
    fn lcs_basic() {
        let result = longest_common_subsequence("abcde", "ace");
        assert_eq!(result, "ace");
    }

    #[test]
    fn lcs_no_common() {
        let result = longest_common_subsequence("abc", "xyz");
        assert_eq!(result, "");
    }

    // ---- coin change ----
    #[test]
    fn coin_change_basic() {
        assert_eq!(coin_change(&[1, 5, 10, 25], 30), Some(2)); // 25+5
    }

    #[test]
    fn coin_change_impossible() {
        assert_eq!(coin_change(&[2], 3), None);
    }

    // ---- LIS ----
    #[test]
    fn lis_basic() {
        let result = longest_increasing_subsequence(&[10, 9, 2, 5, 3, 7, 101, 18]);
        assert_eq!(result.len(), 4);
        // Verify it is actually increasing.
        assert!(result.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn lis_empty() {
        assert_eq!(longest_increasing_subsequence(&[]), vec![]);
    }

    // ---- edit distance ----
    #[test]
    fn edit_distance_basic() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn edit_distance_empty() {
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", ""), 3);
    }

    // ---- unique_paths ----
    #[test]
    fn unique_paths_basic() {
        assert_eq!(unique_paths(3, 7), 28);
        assert_eq!(unique_paths(3, 3), 6);
    }

    #[test]
    fn unique_paths_edge() {
        assert_eq!(unique_paths(1, 1), 1);
        assert_eq!(unique_paths(1, 5), 1);
    }

    // ---- min_path_sum ----
    #[test]
    fn min_path_sum_basic() {
        assert_eq!(
            min_path_sum(&[vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn min_path_sum_single() {
        assert_eq!(min_path_sum(&[vec![5]]), 5);
    }

    // ---- longest_palindromic_subsequence ----
    #[test]
    fn lps_basic() {
        assert_eq!(longest_palindromic_subsequence("bbbab"), 4);
    }

    #[test]
    fn lps_single() {
        assert_eq!(longest_palindromic_subsequence("a"), 1);
    }

    // ---- is_match_wildcard ----
    #[test]
    fn wildcard_match() {
        assert!(is_match_wildcard("adceb", "*a*b"));
        assert!(is_match_wildcard("", "*"));
    }

    #[test]
    fn wildcard_no_match() {
        assert!(!is_match_wildcard("cb", "?a"));
        assert!(!is_match_wildcard("acdcb", "a*c?b"));
    }

    // ---- palindrome_min_cuts ----
    #[test]
    fn palindrome_cuts_basic() {
        assert_eq!(palindrome_min_cuts("aab"), 1);
    }

    #[test]
    fn palindrome_cuts_already_palindrome() {
        assert_eq!(palindrome_min_cuts("aba"), 0);
    }

    // ---- max_profit_with_cooldown ----
    #[test]
    fn cooldown_basic() {
        assert_eq!(max_profit_with_cooldown(&[1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn cooldown_decreasing() {
        assert_eq!(max_profit_with_cooldown(&[5, 4, 3, 2, 1]), 0);
    }

    // ---- max_profit_k_transactions ----
    #[test]
    fn k_transactions_two() {
        assert_eq!(max_profit_k_transactions(2, &[3, 2, 6, 5, 0, 3]), 7);
    }

    #[test]
    fn k_transactions_one() {
        assert_eq!(max_profit_k_transactions(1, &[2, 4, 1]), 2);
    }

    // ---- can_partition_equal_subset ----
    #[test]
    fn partition_yes() {
        assert!(can_partition_equal_subset(&[1, 5, 11, 5]));
    }

    #[test]
    fn partition_no() {
        assert!(!can_partition_equal_subset(&[1, 2, 3, 5]));
    }

    // ---- shortest_hamiltonian_path ----
    #[test]
    fn hamiltonian_basic() {
        assert_eq!(
            shortest_hamiltonian_path(&[vec![0, 1, 10], vec![1, 0, 1], vec![10, 1, 0]]),
            2
        );
    }

    #[test]
    fn hamiltonian_single() {
        assert_eq!(shortest_hamiltonian_path(&[vec![0]]), 0);
    }

    // ---- burst_balloons ----
    #[test]
    fn burst_basic() {
        assert_eq!(burst_balloons(&[3, 1, 5, 8]), 167);
    }

    #[test]
    fn burst_single() {
        assert_eq!(burst_balloons(&[5]), 5);
    }

    // ---- matrix_chain_order ----
    #[test]
    fn matrix_chain_basic() {
        assert_eq!(matrix_chain_order(&[10, 30, 5, 60]), 4500);
    }

    #[test]
    fn matrix_chain_two() {
        assert_eq!(matrix_chain_order(&[10, 20, 30]), 6000);
    }
}
