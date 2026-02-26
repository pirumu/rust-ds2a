//! # Merkle Tree
//!
//! A hash tree where every leaf is the hash of a data block, and every
//! internal node is the hash of its two children.  The single root hash
//! acts as a fingerprint for the entire dataset — if even one byte
//! changes anywhere, the root hash changes too.
//!
//! Used in Git, blockchains, BitTorrent, and certificate transparency.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A Merkle Tree built from a list of data blocks.
///
/// Internally stored as a flat array (like a binary heap):
/// - `nodes[0]` is the root.
/// - Children of `nodes[i]` are at `2*i + 1` and `2*i + 2`.
/// - Leaves occupy the last `leaf_count` slots.
///
/// If the number of data blocks is not a power of two, the last block
/// is duplicated until we reach one.
pub struct MerkleTree {
    nodes: Vec<u64>,
    leaf_count: usize,
}

/// Hashes a single piece of data using `DefaultHasher`.
///
/// **Time:** O(n) where n = length of data.  **Space:** O(1).
fn hash_data(data: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}

/// Combines two hashes into one by hashing them together.
///
/// **Time:** O(1).  **Space:** O(1).
fn hash_pair(left: u64, right: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    left.hash(&mut hasher);
    right.hash(&mut hasher);
    hasher.finish()
}

impl MerkleTree {
    /// Builds a Merkle tree from the given data blocks.
    ///
    /// If `data` is empty, creates a tree with a single zero hash.
    /// If `data.len()` is not a power of two, the last element is
    /// duplicated to pad.
    ///
    /// **Time:** O(n) where n = number of data blocks.
    /// **Space:** O(n).
    pub fn build(data: &[&str]) -> Self {
        if data.is_empty() {
            return Self {
                nodes: vec![0],
                leaf_count: 0,
            };
        }

        // Pad to next power of two.
        let mut leaves: Vec<u64> = data.iter().map(|d| hash_data(d)).collect();
        let leaf_count = leaves.len();
        while leaves.len().count_ones() != 1 {
            leaves.push(*leaves.last().unwrap());
        }

        let n = leaves.len(); // power of two
        let total = 2 * n - 1;
        let mut nodes = vec![0u64; total];

        // Fill leaves (last n slots).
        for (i, &h) in leaves.iter().enumerate() {
            nodes[total - n + i] = h;
        }

        // Build bottom-up.
        for i in (0..total - n).rev() {
            let left = nodes[2 * i + 1];
            let right = nodes[2 * i + 2];
            nodes[i] = hash_pair(left, right);
        }

        Self { nodes, leaf_count }
    }

    /// Returns the root hash of the tree.
    ///
    /// **Time:** O(1).  **Space:** O(1).
    pub fn root_hash(&self) -> u64 {
        self.nodes[0]
    }

    /// Generates a Merkle proof for the leaf at `index`.
    ///
    /// Returns a list of `(sibling_hash, is_right)` pairs walking from
    /// the leaf up to the root.  `is_right` is `true` when the sibling
    /// is the right child (i.e., the node being proven is on the left).
    ///
    /// **Time:** O(log n).  **Space:** O(log n).
    pub fn generate_proof(&self, index: usize) -> Vec<(u64, bool)> {
        assert!(index < self.leaf_count, "index out of range");

        let n = self.nodes.len().div_ceil(2); // number of padded leaves
        let mut pos = self.nodes.len() - n + index; // position in flat array
        let mut proof = Vec::new();

        while pos > 0 {
            let parent = (pos - 1) / 2;
            let left_child = 2 * parent + 1;
            let right_child = 2 * parent + 2;

            if pos == left_child {
                // Current node is left child; sibling is right.
                proof.push((self.nodes[right_child], true));
            } else {
                // Current node is right child; sibling is left.
                proof.push((self.nodes[left_child], false));
            }

            pos = parent;
        }

        proof
    }

    /// Verifies that `data` belongs to a Merkle tree with the given
    /// `root` hash, using the provided proof.
    ///
    /// **Time:** O(log n) where n = number of leaves.
    /// **Space:** O(1).
    pub fn verify(root: u64, data: &str, proof: &[(u64, bool)]) -> bool {
        let mut current = hash_data(data);

        for &(sibling, is_right) in proof {
            if is_right {
                current = hash_pair(current, sibling);
            } else {
                current = hash_pair(sibling, current);
            }
        }

        current == root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- build ----

    #[test]
    fn build_empty() {
        let tree = MerkleTree::build(&[]);
        assert_eq!(tree.root_hash(), 0);
        assert_eq!(tree.leaf_count, 0);
    }

    #[test]
    fn build_single_element() {
        let tree = MerkleTree::build(&["hello"]);
        // Single leaf — root equals that leaf's hash directly (no pairing needed).
        assert_eq!(tree.root_hash(), hash_data("hello"));
    }

    #[test]
    fn build_power_of_two() {
        let tree = MerkleTree::build(&["a", "b", "c", "d"]);
        // 4 leaves, 7 nodes total.
        assert_eq!(tree.nodes.len(), 7);
    }

    #[test]
    fn build_non_power_of_two() {
        let tree = MerkleTree::build(&["a", "b", "c"]);
        // Padded to 4 leaves → 7 nodes.
        assert_eq!(tree.nodes.len(), 7);
    }

    // ---- root_hash ----

    #[test]
    fn root_hash_deterministic() {
        let t1 = MerkleTree::build(&["a", "b", "c", "d"]);
        let t2 = MerkleTree::build(&["a", "b", "c", "d"]);
        assert_eq!(t1.root_hash(), t2.root_hash());
    }

    #[test]
    fn root_hash_changes_with_different_data() {
        let t1 = MerkleTree::build(&["a", "b", "c", "d"]);
        let t2 = MerkleTree::build(&["a", "b", "c", "e"]);
        assert_ne!(t1.root_hash(), t2.root_hash());
    }

    #[test]
    fn root_hash_order_matters() {
        let t1 = MerkleTree::build(&["a", "b"]);
        let t2 = MerkleTree::build(&["b", "a"]);
        assert_ne!(t1.root_hash(), t2.root_hash());
    }

    // ---- generate_proof ----

    #[test]
    fn proof_length_is_log_n() {
        let tree = MerkleTree::build(&["a", "b", "c", "d"]);
        let proof = tree.generate_proof(0);
        // 4 leaves → depth = 2.
        assert_eq!(proof.len(), 2);
    }

    #[test]
    fn proof_length_for_eight_leaves() {
        let data: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h"];
        let tree = MerkleTree::build(&data);
        let proof = tree.generate_proof(3);
        // 8 leaves → depth = 3.
        assert_eq!(proof.len(), 3);
    }

    #[test]
    #[should_panic(expected = "index out of range")]
    fn proof_out_of_range() {
        let tree = MerkleTree::build(&["a", "b"]);
        tree.generate_proof(5);
    }

    // ---- verify ----

    #[test]
    fn verify_valid_proof() {
        let data = &["alpha", "beta", "gamma", "delta"];
        let tree = MerkleTree::build(data);
        let root = tree.root_hash();

        for (i, &item) in data.iter().enumerate() {
            let proof = tree.generate_proof(i);
            assert!(
                MerkleTree::verify(root, item, &proof),
                "verification failed for index {i}"
            );
        }
    }

    #[test]
    fn verify_all_leaves_in_larger_tree() {
        let data: Vec<&str> = (0..8).map(|i| match i {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            _ => "seven",
        }).collect();
        let tree = MerkleTree::build(&data);
        let root = tree.root_hash();

        for (i, &item) in data.iter().enumerate() {
            let proof = tree.generate_proof(i);
            assert!(MerkleTree::verify(root, item, &proof));
        }
    }

    // ---- tamper detection ----

    #[test]
    fn tampered_data_fails_verification() {
        let tree = MerkleTree::build(&["a", "b", "c", "d"]);
        let root = tree.root_hash();
        let proof = tree.generate_proof(0);

        // "a" should verify, but "x" should not.
        assert!(MerkleTree::verify(root, "a", &proof));
        assert!(!MerkleTree::verify(root, "x", &proof));
    }

    #[test]
    fn tampered_proof_fails_verification() {
        let tree = MerkleTree::build(&["a", "b", "c", "d"]);
        let root = tree.root_hash();
        let mut proof = tree.generate_proof(0);

        // Flip the first sibling hash.
        proof[0].0 = proof[0].0.wrapping_add(1);
        assert!(!MerkleTree::verify(root, "a", &proof));
    }

    #[test]
    fn wrong_root_fails_verification() {
        let tree = MerkleTree::build(&["a", "b", "c", "d"]);
        let proof = tree.generate_proof(0);
        let wrong_root = tree.root_hash().wrapping_add(1);

        assert!(!MerkleTree::verify(wrong_root, "a", &proof));
    }

    #[test]
    fn non_power_of_two_verify() {
        let data = &["one", "two", "three"];
        let tree = MerkleTree::build(data);
        let root = tree.root_hash();

        for (i, &item) in data.iter().enumerate() {
            let proof = tree.generate_proof(i);
            assert!(MerkleTree::verify(root, item, &proof));
        }
    }
}
