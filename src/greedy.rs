//! # Greedy Algorithms
//!
//! Activity selection, fractional knapsack, and Huffman encoding.

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Activity Selection — choose the maximum number of non-overlapping activities.
///
/// Given parallel arrays of start and end times, returns the indices of selected
/// activities sorted by their finish time.
///
/// **Time:** O(n log n).  **Space:** O(n).
pub fn activity_selection(start: &[usize], end: &[usize]) -> Vec<usize> {
    assert_eq!(start.len(), end.len(), "start and end must have the same length");
    let n = start.len();
    if n == 0 {
        return vec![];
    }

    // Sort activities by end time.
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by_key(|&i| end[i]);

    let mut selected = vec![indices[0]];
    let mut last_end = end[indices[0]];

    for &i in &indices[1..] {
        if start[i] >= last_end {
            selected.push(i);
            last_end = end[i];
        }
    }
    selected
}

/// Fractional Knapsack — maximize value allowing fractional items.
///
/// **Time:** O(n log n).  **Space:** O(n).
pub fn fractional_knapsack(weights: &[f64], values: &[f64], capacity: f64) -> f64 {
    assert_eq!(weights.len(), values.len());
    let n = weights.len();
    if n == 0 || capacity <= 0.0 {
        return 0.0;
    }

    // Sort by value-to-weight ratio descending.
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_by(|&a, &b| {
        let ratio_a = values[a] / weights[a];
        let ratio_b = values[b] / weights[b];
        ratio_b.partial_cmp(&ratio_a).unwrap_or(Ordering::Equal)
    });

    let mut remaining = capacity;
    let mut total_value = 0.0;

    for &i in &indices {
        if remaining <= 0.0 {
            break;
        }
        if weights[i] <= remaining {
            total_value += values[i];
            remaining -= weights[i];
        } else {
            total_value += values[i] * (remaining / weights[i]);
            remaining = 0.0;
        }
    }
    total_value
}

// ---- Huffman encoding helpers ----

#[derive(Debug)]
enum HuffmanNode {
    Leaf { ch: char, freq: usize },
    Internal { freq: usize, left: Box<HuffmanNode>, right: Box<HuffmanNode> },
}

impl HuffmanNode {
    fn freq(&self) -> usize {
        match self {
            HuffmanNode::Leaf { freq, .. } => *freq,
            HuffmanNode::Internal { freq, .. } => *freq,
        }
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq() == other.freq()
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap behavior with BinaryHeap (which is a max-heap).
        other.freq().cmp(&self.freq())
    }
}

fn build_code_table(node: &HuffmanNode, prefix: &str, table: &mut HashMap<char, String>) {
    match node {
        HuffmanNode::Leaf { ch, .. } => {
            // If the tree has a single leaf, assign code "0".
            let code = if prefix.is_empty() { "0".to_string() } else { prefix.to_string() };
            table.insert(*ch, code);
        }
        HuffmanNode::Internal { left, right, .. } => {
            build_code_table(left, &format!("{prefix}0"), table);
            build_code_table(right, &format!("{prefix}1"), table);
        }
    }
}

/// Huffman Encoding — build a prefix-free code and encode the input text.
///
/// Returns `(encoded_string, code_table)`.
/// **Time:** O(n log k) where k = number of distinct characters.
pub fn huffman_encoding(text: &str) -> (String, HashMap<char, String>) {
    if text.is_empty() {
        return (String::new(), HashMap::new());
    }

    // Count frequencies.
    let mut freq_map: HashMap<char, usize> = HashMap::new();
    for ch in text.chars() {
        *freq_map.entry(ch).or_insert(0) += 1;
    }

    // Build priority queue.
    let mut heap = BinaryHeap::new();
    for (&ch, &freq) in &freq_map {
        heap.push(HuffmanNode::Leaf { ch, freq });
    }

    // Build tree.
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let merged = HuffmanNode::Internal {
            freq: left.freq() + right.freq(),
            left: Box::new(left),
            right: Box::new(right),
        };
        heap.push(merged);
    }

    let root = heap.pop().unwrap();
    let mut code_table = HashMap::new();
    build_code_table(&root, "", &mut code_table);

    // Encode.
    let encoded: String = text.chars().map(|ch| code_table[&ch].as_str()).collect();

    (encoded, code_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- activity selection ----
    #[test]
    fn activity_selection_basic() {
        let start = [1, 3, 0, 5, 8, 5];
        let end = [2, 4, 6, 7, 9, 9];
        let selected = activity_selection(&start, &end);
        assert!(selected.len() >= 3); // optimal is 4 non-overlapping: (1,2),(3,4),(5,7),(8,9)
    }

    #[test]
    fn activity_selection_empty() {
        assert_eq!(activity_selection(&[], &[]), vec![]);
    }

    #[test]
    fn activity_selection_single() {
        assert_eq!(activity_selection(&[0], &[1]), vec![0]);
    }

    // ---- fractional knapsack ----
    #[test]
    fn fractional_knapsack_basic() {
        let weights = [10.0, 20.0, 30.0];
        let values = [60.0, 100.0, 120.0];
        let result = fractional_knapsack(&weights, &values, 50.0);
        assert!((result - 240.0).abs() < 1e-9);
    }

    #[test]
    fn fractional_knapsack_zero_capacity() {
        let result = fractional_knapsack(&[10.0], &[100.0], 0.0);
        assert!((result - 0.0).abs() < 1e-9);
    }

    // ---- huffman encoding ----
    #[test]
    fn huffman_roundtrip_length() {
        let text = "aabbc";
        let (encoded, table) = huffman_encoding(text);
        // The encoded string length should equal sum of code lengths * frequencies.
        let expected_len: usize = text.chars().map(|ch| table[&ch].len()).sum();
        assert_eq!(encoded.len(), expected_len);
    }

    #[test]
    fn huffman_single_char() {
        let (encoded, table) = huffman_encoding("aaaa");
        assert_eq!(table.len(), 1);
        assert_eq!(table[&'a'], "0");
        assert_eq!(encoded, "0000");
    }

    #[test]
    fn huffman_empty() {
        let (encoded, table) = huffman_encoding("");
        assert!(encoded.is_empty());
        assert!(table.is_empty());
    }
}
