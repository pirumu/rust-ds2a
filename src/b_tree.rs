//! # B-Tree
//!
//! A self-balancing search tree with configurable order. Each node can hold
//! multiple keys and have multiple children, keeping the tree shallow for
//! efficient disk-based access patterns.

/// A B-Tree node.
#[derive(Debug)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    leaf: bool,
}

impl<T: Ord> Node<T> {
    fn new(leaf: bool) -> Self {
        Node {
            keys: Vec::new(),
            children: Vec::new(),
            leaf,
        }
    }

    fn search(&self, val: &T) -> bool {
        let mut i = 0;
        while i < self.keys.len() && *val > self.keys[i] {
            i += 1;
        }
        if i < self.keys.len() && self.keys[i] == *val {
            return true;
        }
        if self.leaf {
            return false;
        }
        self.children[i].search(val)
    }
}

/// A B-Tree with minimum degree `t`.
///
/// Each non-root node has at least `t-1` keys and at most `2t-1` keys.
/// The root may have as few as 1 key.
#[derive(Debug)]
pub struct BTree<T: Ord> {
    root: Node<T>,
    t: usize, // minimum degree
}

impl<T: Ord + Clone> BTree<T> {
    /// Creates an empty B-Tree with the given minimum degree.
    ///
    /// # Panics
    ///
    /// Panics if `t < 2`.
    pub fn new(t: usize) -> Self {
        assert!(t >= 2, "Minimum degree must be at least 2");
        BTree {
            root: Node::new(true),
            t,
        }
    }

    /// Returns `true` if the tree contains the given value.
    pub fn search(&self, val: &T) -> bool {
        self.root.search(val)
    }

    /// Inserts a value into the B-Tree.
    pub fn insert(&mut self, val: T) {
        let max_keys = 2 * self.t - 1;

        if self.root.keys.len() == max_keys {
            // Root is full -- split it.
            let old_root = std::mem::replace(&mut self.root, Node::new(false));
            self.root.children.push(old_root);
            self.split_child(0);
            self.insert_non_full(&mut InsertPath::Root, val);
        } else {
            self.insert_non_full(&mut InsertPath::Root, val);
        }
    }

    /// Splits the i-th child of the root or a node reached during insertion.
    fn split_child_of(node: &mut Node<T>, i: usize, t: usize) {
        let mid = t - 1;
        let child = &mut node.children[i];

        let mut new_node = Node::new(child.leaf);
        // Move the upper half of keys to the new node.
        new_node.keys = child.keys.split_off(mid + 1);
        let median = child.keys.pop().unwrap();

        if !child.leaf {
            new_node.children = child.children.split_off(mid + 1);
        }

        node.keys.insert(i, median);
        node.children.insert(i + 1, new_node);
    }

    fn split_child(&mut self, i: usize) {
        Self::split_child_of(&mut self.root, i, self.t);
    }

    fn insert_non_full(&mut self, _path: &mut InsertPath, val: T) {
        Self::insert_into_node(&mut self.root, val, self.t);
    }
}

/// Internal path tracker (not exposed).
enum InsertPath {
    Root,
}

impl<T: Ord + Clone> BTree<T> {
    fn insert_into_node(node: &mut Node<T>, val: T, t: usize) {
        let mut i = node.keys.len();

        if node.leaf {
            // Find position and insert.
            while i > 0 && val < node.keys[i - 1] {
                i -= 1;
            }
            // Skip duplicates.
            if i > 0 && node.keys[i - 1] == val {
                return;
            }
            node.keys.insert(i, val);
        } else {
            // Find the child to descend into.
            while i > 0 && val < node.keys[i - 1] {
                i -= 1;
            }
            // Check for duplicate.
            if i > 0 && node.keys[i - 1] == val {
                return;
            }

            let max_keys = 2 * t - 1;
            if node.children[i].keys.len() == max_keys {
                Self::split_child_of(node, i, t);
                if val > node.keys[i] {
                    i += 1;
                } else if val == node.keys[i] {
                    return; // duplicate
                }
            }
            Self::insert_into_node(&mut node.children[i], val, t);
        }
    }

    /// Returns all keys in sorted order.
    pub fn inorder(&self) -> Vec<T> {
        let mut result = Vec::new();
        fn walk<T: Clone>(node: &Node<T>, out: &mut Vec<T>) {
            for i in 0..node.keys.len() {
                if !node.leaf {
                    walk(&node.children[i], out);
                }
                out.push(node.keys[i].clone());
            }
            if !node.leaf {
                walk(node.children.last().unwrap(), out);
            }
        }
        walk(&self.root, &mut result);
        result
    }
}

impl<T: Ord + Clone> Default for BTree<T> {
    fn default() -> Self {
        Self::new(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_btree_is_empty() {
        let tree: BTree<i32> = BTree::new(2);
        assert!(!tree.search(&1));
        assert!(tree.inorder().is_empty());
    }

    #[test]
    fn insert_and_search() {
        let mut tree = BTree::new(2);
        tree.insert(10);
        tree.insert(20);
        tree.insert(5);
        assert!(tree.search(&10));
        assert!(tree.search(&20));
        assert!(tree.search(&5));
        assert!(!tree.search(&15));
    }

    #[test]
    fn inorder_is_sorted() {
        let mut tree = BTree::new(2);
        for v in [10, 20, 5, 15, 25, 1, 8] {
            tree.insert(v);
        }
        assert_eq!(tree.inorder(), vec![1, 5, 8, 10, 15, 20, 25]);
    }

    #[test]
    fn many_insertions() {
        let mut tree = BTree::new(3);
        for i in (1..=50).rev() {
            tree.insert(i);
        }
        let sorted = tree.inorder();
        let expected: Vec<i32> = (1..=50).collect();
        assert_eq!(sorted, expected);
    }

    #[test]
    fn higher_degree() {
        let mut tree = BTree::new(5);
        for i in 1..=100 {
            tree.insert(i);
        }
        for i in 1..=100 {
            assert!(tree.search(&i));
        }
        assert!(!tree.search(&101));
    }

    #[test]
    fn duplicate_insert_ignored() {
        let mut tree = BTree::new(2);
        tree.insert(5);
        tree.insert(5);
        tree.insert(5);
        assert_eq!(tree.inorder().len(), 1);
    }

    #[test]
    #[should_panic]
    fn min_degree_too_small() {
        let _tree: BTree<i32> = BTree::new(1);
    }
}
