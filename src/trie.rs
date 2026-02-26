//! # Trie
//!
//! A prefix tree for efficient string storage and prefix lookups.

use std::collections::HashMap;

/// A node in the trie.
#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

/// An alphabet trie for storing and searching strings.
#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    /// Creates an empty trie.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Inserts a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_default();
        }
        current.is_end = true;
    }

    /// Returns `true` if the trie contains the exact word.
    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end
    }

    /// Returns `true` if any word in the trie starts with the given prefix.
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        for ch in prefix.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return false,
            }
        }
        true
    }

    /// Deletes a word from the trie. Returns `true` if the word was found and deleted.
    pub fn delete(&mut self, word: &str) -> bool {
        fn remove(node: &mut TrieNode, word: &[char], depth: usize) -> (bool, bool) {
            if depth == word.len() {
                if !node.is_end {
                    return (false, false); // word not found
                }
                node.is_end = false;
                // Can delete this node if it has no children.
                return (true, node.children.is_empty());
            }

            let ch = word[depth];
            let Some(child) = node.children.get_mut(&ch) else {
                return (false, false);
            };

            let (found, should_delete_child) = remove(child, word, depth + 1);
            if should_delete_child {
                node.children.remove(&ch);
            }

            (found, !node.is_end && node.children.is_empty())
        }

        let chars: Vec<char> = word.chars().collect();
        let (found, _) = remove(&mut self.root, &chars, 0);
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_trie_is_empty() {
        let trie = Trie::new();
        assert!(!trie.search("hello"));
    }

    #[test]
    fn insert_and_search() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");
        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        assert!(!trie.search("hell"));
        assert!(!trie.search("worlds"));
    }

    #[test]
    fn starts_with() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        assert!(trie.starts_with("hel"));
        assert!(trie.starts_with("hello"));
        assert!(!trie.starts_with("hex"));
    }

    #[test]
    fn delete_word() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");
        assert!(trie.delete("hello"));
        assert!(!trie.search("hello"));
        assert!(trie.search("help"));
        // "hel" prefix should still work because "help" remains.
        assert!(trie.starts_with("hel"));
    }

    #[test]
    fn delete_nonexistent() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(!trie.delete("world"));
        assert!(trie.search("hello"));
    }

    #[test]
    fn delete_prefix_does_not_break_longer_word() {
        let mut trie = Trie::new();
        trie.insert("app");
        trie.insert("apple");
        assert!(trie.delete("app"));
        assert!(!trie.search("app"));
        assert!(trie.search("apple"));
    }

    #[test]
    fn empty_string() {
        let mut trie = Trie::new();
        trie.insert("");
        assert!(trie.search(""));
        assert!(trie.starts_with(""));
    }

    #[test]
    fn unicode_support() {
        let mut trie = Trie::new();
        trie.insert("cafe");
        assert!(trie.search("cafe"));
        assert!(trie.starts_with("caf"));
    }
}
