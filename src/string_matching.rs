//! # String Matching Algorithms
//!
//! Naive search, KMP (Knuth-Morris-Pratt), and Rabin-Karp algorithms
//! for finding all occurrences of a pattern within a text.

/// Brute-force string matching. Checks every position in `text` for a match with `pattern`.
///
/// Returns a `Vec<usize>` of starting indices where `pattern` occurs in `text`.
///
/// **Time:** O(n * m) where n = text length, m = pattern length.  **Space:** O(1) (excluding output).
pub fn naive_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();

    if p.is_empty() || p.len() > t.len() {
        return result;
    }

    for i in 0..=t.len() - p.len() {
        let mut matched = true;
        for j in 0..p.len() {
            if t[i + j] != p[j] {
                matched = false;
                break;
            }
        }
        if matched {
            result.push(i);
        }
    }
    result
}

/// Build the LPS (Longest Proper Prefix which is also Suffix) table for KMP.
///
/// `lps[i]` = length of the longest proper prefix of `pattern[0..=i]` that is also a suffix.
///
/// **Time:** O(m).  **Space:** O(m).
pub fn build_lps(pattern: &str) -> Vec<usize> {
    let p = pattern.as_bytes();
    let m = p.len();
    let mut lps = vec![0usize; m];

    if m == 0 {
        return lps;
    }

    let mut len: usize = 0; // length of previous longest prefix suffix
    let mut i: usize = 1;

    while i < m {
        if p[i] == p[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}

/// KMP (Knuth-Morris-Pratt) string matching using the LPS table.
///
/// Returns a `Vec<usize>` of starting indices where `pattern` occurs in `text`.
///
/// **Time:** O(n + m).  **Space:** O(m).
pub fn kmp_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m == 0 || m > n {
        return result;
    }

    let lps = build_lps(pattern);
    let mut i: usize = 0; // index in text
    let mut j: usize = 0; // index in pattern

    while i < n {
        if t[i] == p[j] {
            i += 1;
            j += 1;
        }

        if j == m {
            result.push(i - m);
            j = lps[j - 1];
        } else if i < n && t[i] != p[j] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    result
}

/// Rabin-Karp string matching using a rolling polynomial hash.
///
/// Returns a `Vec<usize>` of starting indices where `pattern` occurs in `text`.
///
/// **Time:** O(n + m) average, O(n * m) worst case (hash collisions).  **Space:** O(1) (excluding output).
pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let t = text.as_bytes();
    let p = pattern.as_bytes();
    let n = t.len();
    let m = p.len();

    if m == 0 || m > n {
        return result;
    }

    let base: u64 = 256;
    let modulus: u64 = 1_000_000_007;

    // Compute base^(m-1) % modulus
    let mut h: u64 = 1;
    for _ in 0..m - 1 {
        h = (h * base) % modulus;
    }

    // Compute hash of pattern and first window of text
    let mut p_hash: u64 = 0;
    let mut t_hash: u64 = 0;
    for i in 0..m {
        p_hash = (p_hash * base + p[i] as u64) % modulus;
        t_hash = (t_hash * base + t[i] as u64) % modulus;
    }

    for i in 0..=n - m {
        if p_hash == t_hash {
            // Verify character by character to handle hash collisions
            if t[i..i + m] == p[..] {
                result.push(i);
            }
        }
        // Compute hash for next window
        if i < n - m {
            // Remove leading char, add trailing char
            t_hash = (t_hash + modulus - (t[i] as u64 * h) % modulus) % modulus;
            t_hash = (t_hash * base + t[i + m] as u64) % modulus;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- naive_search ----
    #[test]
    fn naive_found() {
        assert_eq!(naive_search("hello world", "world"), vec![6]);
    }

    #[test]
    fn naive_not_found() {
        assert_eq!(naive_search("hello world", "xyz"), vec![]);
    }

    #[test]
    fn naive_multiple_matches() {
        assert_eq!(naive_search("ababab", "ab"), vec![0, 2, 4]);
    }

    #[test]
    fn naive_overlapping() {
        assert_eq!(naive_search("aaaa", "aa"), vec![0, 1, 2]);
    }

    #[test]
    fn naive_empty_pattern() {
        assert_eq!(naive_search("hello", ""), vec![]);
    }

    #[test]
    fn naive_pattern_longer_than_text() {
        assert_eq!(naive_search("hi", "hello"), vec![]);
    }

    #[test]
    fn naive_single_char() {
        assert_eq!(naive_search("abcabc", "a"), vec![0, 3]);
    }

    #[test]
    fn naive_full_match() {
        assert_eq!(naive_search("abc", "abc"), vec![0]);
    }

    // ---- build_lps ----
    #[test]
    fn lps_empty() {
        assert_eq!(build_lps(""), vec![]);
    }

    #[test]
    fn lps_no_prefix_suffix() {
        assert_eq!(build_lps("abcd"), vec![0, 0, 0, 0]);
    }

    #[test]
    fn lps_with_overlap() {
        // "abab" -> lps = [0, 0, 1, 2]
        assert_eq!(build_lps("abab"), vec![0, 0, 1, 2]);
    }

    #[test]
    fn lps_aaaa() {
        assert_eq!(build_lps("aaaa"), vec![0, 1, 2, 3]);
    }

    #[test]
    fn lps_ababaca() {
        // Classic example: "ababaca" -> [0, 0, 1, 2, 3, 0, 1]
        assert_eq!(build_lps("ababaca"), vec![0, 0, 1, 2, 3, 0, 1]);
    }

    // ---- kmp_search ----
    #[test]
    fn kmp_found() {
        assert_eq!(kmp_search("hello world", "world"), vec![6]);
    }

    #[test]
    fn kmp_not_found() {
        assert_eq!(kmp_search("hello world", "xyz"), vec![]);
    }

    #[test]
    fn kmp_multiple_matches() {
        assert_eq!(kmp_search("ababab", "ab"), vec![0, 2, 4]);
    }

    #[test]
    fn kmp_overlapping() {
        assert_eq!(kmp_search("aaaa", "aa"), vec![0, 1, 2]);
    }

    #[test]
    fn kmp_empty_pattern() {
        assert_eq!(kmp_search("hello", ""), vec![]);
    }

    #[test]
    fn kmp_pattern_longer_than_text() {
        assert_eq!(kmp_search("hi", "hello"), vec![]);
    }

    #[test]
    fn kmp_single_char() {
        assert_eq!(kmp_search("abcabc", "c"), vec![2, 5]);
    }

    #[test]
    fn kmp_full_match() {
        assert_eq!(kmp_search("abc", "abc"), vec![0]);
    }

    #[test]
    fn kmp_complex_pattern() {
        assert_eq!(kmp_search("ababababca", "ababc"), vec![4]);
    }

    // ---- rabin_karp ----
    #[test]
    fn rk_found() {
        assert_eq!(rabin_karp("hello world", "world"), vec![6]);
    }

    #[test]
    fn rk_not_found() {
        assert_eq!(rabin_karp("hello world", "xyz"), vec![]);
    }

    #[test]
    fn rk_multiple_matches() {
        assert_eq!(rabin_karp("ababab", "ab"), vec![0, 2, 4]);
    }

    #[test]
    fn rk_overlapping() {
        assert_eq!(rabin_karp("aaaa", "aa"), vec![0, 1, 2]);
    }

    #[test]
    fn rk_empty_pattern() {
        assert_eq!(rabin_karp("hello", ""), vec![]);
    }

    #[test]
    fn rk_pattern_longer_than_text() {
        assert_eq!(rabin_karp("hi", "hello"), vec![]);
    }

    #[test]
    fn rk_single_char() {
        assert_eq!(rabin_karp("abcabc", "b"), vec![1, 4]);
    }

    #[test]
    fn rk_full_match() {
        assert_eq!(rabin_karp("abc", "abc"), vec![0]);
    }

    // ---- cross-algorithm consistency ----
    #[test]
    fn all_algorithms_agree() {
        let cases = vec![
            ("abcabcabc", "abc"),
            ("aaaaaaa", "aaa"),
            ("hello", "ll"),
            ("mississippi", "issi"),
            ("abcdef", "xyz"),
        ];
        for (text, pattern) in cases {
            let naive = naive_search(text, pattern);
            let kmp = kmp_search(text, pattern);
            let rk = rabin_karp(text, pattern);
            assert_eq!(naive, kmp, "naive vs kmp on ({}, {})", text, pattern);
            assert_eq!(naive, rk, "naive vs rk on ({}, {})", text, pattern);
        }
    }
}
