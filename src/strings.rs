//! # Strings
//!
//! Common string operations: reversing, palindrome detection, anagram checking,
//! finding non-repeating characters, and run-length encoding.

use std::collections::HashMap;

/// Reverse a string, respecting Unicode grapheme boundaries.
///
/// **Time:** O(n) — **Space:** O(n)
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Check whether a string is a palindrome (case-insensitive, alphanumeric only).
///
/// **Time:** O(n) — **Space:** O(n) for the filtered collection
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

/// Check whether two strings are anagrams of each other (case-insensitive).
///
/// Two strings are anagrams if they contain the exact same characters
/// with the same frequencies.
///
/// **Time:** O(n) — **Space:** O(n)
pub fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in s1.chars().filter(|c| c.is_alphanumeric()) {
        *counts.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }

    for c in s2.chars().filter(|c| c.is_alphanumeric()) {
        *counts.entry(c.to_ascii_lowercase()).or_insert(0) -= 1;
    }

    counts.values().all(|&v| v == 0)
}

/// Return the first character that does not repeat in the string.
///
/// **Time:** O(n) — **Space:** O(n)
pub fn first_non_repeating_char(s: &str) -> Option<char> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    let chars: Vec<char> = s.chars().collect();

    for &c in &chars {
        *counts.entry(c).or_insert(0) += 1;
    }

    chars.into_iter().find(|c| counts[c] == 1)
}

/// Run-length encoding: compress consecutive identical characters.
///
/// Example: `"aaabbc"` becomes `"a3b2c1"`.
///
/// **Time:** O(n) — **Space:** O(n)
pub fn compress(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut chars = s.chars();
    let mut current = match chars.next() {
        Some(c) => c,
        None => return result,
    };
    let mut count: usize = 1;

    for c in chars {
        if c == current {
            count += 1;
        } else {
            result.push(current);
            result.push_str(&count.to_string());
            current = c;
            count = 1;
        }
    }
    // Flush the last run.
    result.push(current);
    result.push_str(&count.to_string());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- reverse_string --------------------------------------------------

    #[test]
    fn reverse_string_basic() {
        assert_eq!(reverse_string("hello"), "olleh");
    }

    #[test]
    fn reverse_string_empty() {
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn reverse_string_unicode() {
        assert_eq!(reverse_string("abc"), "cba");
    }

    // -- is_palindrome ---------------------------------------------------

    #[test]
    fn is_palindrome_true() {
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn is_palindrome_mixed_case_and_spaces() {
        assert!(is_palindrome("A man a plan a canal Panama"));
    }

    #[test]
    fn is_palindrome_false() {
        assert!(!is_palindrome("hello"));
    }

    // -- are_anagrams ----------------------------------------------------

    #[test]
    fn are_anagrams_true() {
        assert!(are_anagrams("listen", "silent"));
    }

    #[test]
    fn are_anagrams_false() {
        assert!(!are_anagrams("hello", "world"));
    }

    #[test]
    fn are_anagrams_case_insensitive() {
        assert!(are_anagrams("Astronomer", "Moon starer"));
    }

    // -- first_non_repeating_char ----------------------------------------

    #[test]
    fn first_non_repeating_basic() {
        assert_eq!(first_non_repeating_char("aabcc"), Some('b'));
    }

    #[test]
    fn first_non_repeating_none() {
        assert_eq!(first_non_repeating_char("aabb"), None);
    }

    #[test]
    fn first_non_repeating_first_char() {
        assert_eq!(first_non_repeating_char("abcabc"), None);
        assert_eq!(first_non_repeating_char("abcab"), Some('c'));
    }

    // -- compress --------------------------------------------------------

    #[test]
    fn compress_basic() {
        assert_eq!(compress("aaabbc"), "a3b2c1");
    }

    #[test]
    fn compress_single_chars() {
        assert_eq!(compress("abcd"), "a1b1c1d1");
    }

    #[test]
    fn compress_empty() {
        assert_eq!(compress(""), "");
    }
}
